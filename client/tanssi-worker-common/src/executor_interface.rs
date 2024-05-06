// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Interface to the Substrate Executor

use crate::{framed_recv_blocking, framed_send_blocking};
use parity_scale_codec::{Decode, Encode};
use polkadot_primitives::{
    executor_params::{DEFAULT_LOGICAL_STACK_MAX, DEFAULT_NATIVE_STACK_MAX},
    ExecutorParam, ExecutorParams,
};
use sc_executor_common::{
    error::WasmError,
    runtime_blob::RuntimeBlob,
    wasm_runtime::{HeapAllocStrategy, InvokeMethod, WasmModule as _},
};
use sc_executor_wasmtime::{Config, DeterministicStackLimit, Semantics, WasmtimeRuntime};
use sp_core::storage::{ChildInfo, TrackedStorageKey};
use sp_externalities::{Extensions, Externalities, MultiRemovalResults};
use std::any::{Any, TypeId};
use std::io;
use std::io::{Read, Write};
use std::sync::Mutex;

// Memory configuration
//
// When Substrate Runtime is instantiated, a number of WASM pages are allocated for the Substrate
// Runtime instance's linear memory. The exact number of pages is a sum of whatever the WASM blob
// itself requests (by default at least enough to hold the data section as well as have some space
// left for the stack; this is, of course, overridable at link time when compiling the runtime)
// plus the number of pages specified in the `extra_heap_pages` passed to the executor.
//
// By default, rustc (or `lld` specifically) should allocate 1 MiB for the shadow stack, or 16
// pages. The data section for runtimes are typically rather small and can fit in a single digit
// number of WASM pages, so let's say an extra 16 pages. Thus let's assume that 32 pages or 2 MiB
// are used for these needs by default.
const DEFAULT_HEAP_PAGES_ESTIMATE: u32 = 32;
const EXTRA_HEAP_PAGES: u32 = 2048;

// VALUES OF THE DEFAULT CONFIGURATION SHOULD NEVER BE CHANGED
// They are used as base values for the execution environment parametrization.
// To overwrite them, add new ones to `EXECUTOR_PARAMS` in the `session_info` pallet and perform
// a runtime upgrade to make them active.
pub const DEFAULT_CONFIG: Config = Config {
    allow_missing_func_imports: true,
    cache_path: None,
    semantics: Semantics {
        heap_alloc_strategy: sc_executor_common::wasm_runtime::HeapAllocStrategy::Dynamic {
            maximum_pages: Some(DEFAULT_HEAP_PAGES_ESTIMATE + EXTRA_HEAP_PAGES),
        },

        instantiation_strategy:
            sc_executor_wasmtime::InstantiationStrategy::RecreateInstanceCopyOnWrite,

        // Enable deterministic stack limit to pin down the exact number of items the wasmtime stack
        // can contain before it traps with stack overflow.
        //
        // Here is how the values below were chosen.
        //
        // At the moment of writing, the default native stack size limit is 1 MiB. Assuming a
        // logical item (see the docs about the field and the instrumentation algorithm) is 8 bytes,
        // 1 MiB can fit 2x 65536 logical items.
        //
        // Since reaching the native stack limit is undesirable, we halve the logical item limit and
        // also increase the native 256x. This hopefully should preclude wasm code from reaching
        // the stack limit set by the wasmtime.
        deterministic_stack_limit: Some(DeterministicStackLimit {
            logical_max: DEFAULT_LOGICAL_STACK_MAX,
            native_stack_max: DEFAULT_NATIVE_STACK_MAX,
        }),
        canonicalize_nans: true,
        // Rationale for turning the multi-threaded compilation off is to make the preparation time
        // easily reproducible and as deterministic as possible.
        //
        // Currently the prepare queue doesn't distinguish between precheck and prepare requests.
        // On the one hand, it simplifies the code, on the other, however, slows down compile times
        // for execute requests. This behavior may change in future.
        parallel_compilation: false,

        // WASM extensions. Only those that are meaningful to us may be controlled here. By default,
        // we're using WASM MVP, which means all the extensions are disabled. Nevertheless, some
        // extensions (e.g., sign extension ops) are enabled by Wasmtime and cannot be disabled.
        wasm_reference_types: false,
        wasm_simd: false,
        wasm_bulk_memory: false,
        wasm_multi_value: false,
    },
};

// TODO: I removed execute_artifact

/// Executes the given PVF in the form of a compiled artifact and returns the result of
/// execution upon success.
///
/// # Safety
///
/// The caller must ensure that the compiled artifact passed here was:
///   1) produced by `prepare`,
///   2) was not modified,
///
/// Failure to adhere to these requirements might lead to crashes and arbitrary code execution.
pub unsafe fn execute_artifact_export(
    compiled_artifact_blob: &[u8],
    executor_params: &ExecutorParams,
    export_name: &str,
    params: &[u8],
    ipc_pipe_reader: impl Read + Unpin,
    ipc_pipe_writer: impl Write + Unpin,
) -> Result<Vec<u8>, String> {
    let mut extensions = sp_externalities::Extensions::new();

    extensions.register(sp_core::traits::ReadRuntimeVersionExt::new(
        ReadRuntimeVersion,
    ));
    // TODO: LocalCallExecutor handles PoV, maybe use that here

    // TODO: here we need to add the database somehow
    // Some options:
    // * Send serialized db to worker as Vec<u8>, similar to params parameter
    // * Make a copy of the paritydb folder inside the worker chroot dir, give the worker access to it
    // * Use IPC to ask parent process on every read
    // And we also need a way to return the PoV if we are creating a block, and also how do we handle
    // db writes? For now every export call starts with a fresh state and gets killed after completion,
    // so executing a batch of runtime calls that modify state is impossible.
    //let mut ext = ValidationExternalities(extensions);
    let mut ext = IpcExternalities {
        extensions,
        //ipc_stream: Mutex::new(ipc_stream.unwrap()),
        ipc_stream: Mutex::new(PipePair(ipc_pipe_reader, ipc_pipe_writer)),
    };

    match sc_executor::with_externalities_safe(&mut ext, || {
        let runtime = create_runtime_from_artifact_bytes(compiled_artifact_blob, executor_params)?;
        runtime
            .new_instance()?
            .call(InvokeMethod::Export(export_name), params)
    }) {
        Ok(Ok(ok)) => Ok(ok),
        Ok(Err(err)) | Err(err) => Err(err),
    }
    .map_err(|err| format!("execute error: {:?}", err))
}

/// Constructs the runtime for the given PVF, given the artifact bytes.
///
/// # Safety
///
/// The caller must ensure that the compiled artifact passed here was:
///   1) produced by `prepare`,
///   2) was not modified,
///
/// Failure to adhere to these requirements might lead to crashes and arbitrary code execution.
pub unsafe fn create_runtime_from_artifact_bytes(
    compiled_artifact_blob: &[u8],
    executor_params: &ExecutorParams,
) -> Result<WasmtimeRuntime, WasmError> {
    let mut config = DEFAULT_CONFIG.clone();
    config.semantics = params_to_wasmtime_semantics(executor_params);

    sc_executor_wasmtime::create_runtime_from_artifact_bytes::<HostFunctions>(
        compiled_artifact_blob,
        config,
    )
}

pub fn params_to_wasmtime_semantics(par: &ExecutorParams) -> Semantics {
    let mut sem = DEFAULT_CONFIG.semantics.clone();
    let mut stack_limit = sem
		.deterministic_stack_limit
		.expect("There is a comment to not change the default stack limit; it should always be available; qed")
		.clone();

    for p in par.iter() {
        match p {
            ExecutorParam::MaxMemoryPages(max_pages) => {
                sem.heap_alloc_strategy = HeapAllocStrategy::Dynamic {
                    maximum_pages: Some((*max_pages).saturating_add(DEFAULT_HEAP_PAGES_ESTIMATE)),
                }
            }
            ExecutorParam::StackLogicalMax(slm) => stack_limit.logical_max = *slm,
            ExecutorParam::StackNativeMax(snm) => stack_limit.native_stack_max = *snm,
            ExecutorParam::WasmExtBulkMemory => sem.wasm_bulk_memory = true,
            ExecutorParam::PrecheckingMaxMemory(_)
            | ExecutorParam::PvfPrepTimeout(_, _)
            | ExecutorParam::PvfExecTimeout(_, _) => (), /* Not used here */
        }
    }
    sem.deterministic_stack_limit = Some(stack_limit);
    sem
}

/// Runs the prevalidation on the given code. Returns a [`RuntimeBlob`] if it succeeds.
pub fn prevalidate(code: &[u8]) -> Result<RuntimeBlob, sc_executor_common::error::WasmError> {
    // Construct the runtime blob and do some basic checks for consistency.
    let blob = RuntimeBlob::new(code)?;
    // In the future this function should take care of any further prevalidation logic.
    Ok(blob)
}

/// Runs preparation on the given runtime blob. If successful, it returns a serialized compiled
/// artifact which can then be used to pass into `Executor::execute` after writing it to the disk.
pub fn prepare(
    blob: RuntimeBlob,
    executor_params: &ExecutorParams,
) -> Result<Vec<u8>, sc_executor_common::error::WasmError> {
    let semantics = params_to_wasmtime_semantics(executor_params);
    sc_executor_wasmtime::prepare_runtime_artifact(blob, Default::default(), &semantics)
}

/// Available host functions. We leave out:
///
/// 1. storage related stuff (PVF doesn't have a notion of a persistent storage/trie)
/// 2. tracing
/// 3. off chain workers (PVFs do not have such a notion)
/// 4. runtime tasks
/// 5. sandbox
/*
type HostFunctions = (
    sp_io::misc::HostFunctions,
    sp_io::crypto::HostFunctions,
    sp_io::hashing::HostFunctions,
    sp_io::allocator::HostFunctions,
    sp_io::logging::HostFunctions,
    sp_io::trie::HostFunctions,
);
 */
// Equivalent to sp_io::SubstrateHostFunctions
pub type HostFunctions = (
    sp_io::storage::HostFunctions,
    sp_io::default_child_storage::HostFunctions,
    sp_io::misc::HostFunctions,
    sp_io::wasm_tracing::HostFunctions,
    sp_io::offchain::HostFunctions,
    sp_io::crypto::HostFunctions,
    sp_io::hashing::HostFunctions,
    sp_io::allocator::HostFunctions,
    sp_io::panic_handler::HostFunctions,
    sp_io::logging::HostFunctions,
    sp_io::trie::HostFunctions,
    sp_io::offchain_index::HostFunctions,
    sp_io::transaction_index::HostFunctions,
);

struct ReadRuntimeVersion;

impl sp_core::traits::ReadRuntimeVersion for ReadRuntimeVersion {
    fn read_runtime_version(
        &self,
        wasm_code: &[u8],
        _ext: &mut dyn sp_externalities::Externalities,
    ) -> Result<Vec<u8>, String> {
        let blob = RuntimeBlob::uncompress_if_needed(wasm_code)
            .map_err(|e| format!("Failed to read the PVF runtime blob: {:?}", e))?;

        match sc_executor::read_embedded_version(&blob).map_err(|e| {
            format!(
                "Failed to read the static section from the PVF blob: {:?}",
                e
            )
        })? {
            Some(version) => {
                use parity_scale_codec::Encode;
                Ok(version.encode())
            }
            None => Err("runtime version section is not found".to_string()),
        }
    }
}

pub struct IpcExternalities<S: Read + Write + Unpin> {
    extensions: Extensions,
    ipc_stream: Mutex<S>,
}

impl<S> IpcExternalities<S>
where
    S: Read + Write + Unpin,
{
    fn r<T>(&self, req: IpcExtRequest) -> T
    where
        T: TryFrom<IpcExtResponse>,
    {
        // Using try_lock instead of lock because we always have a `&mut ext`, and this worker is
        // single-threaded, so it should be impossible to try to lock twice.
        let mut stream = self.ipc_stream.try_lock().expect("deadlock?");
        r_inner(&mut *stream, req)
            .try_into()
            .unwrap_or_else(|e| panic!("wrong response type for request"))
    }
}

fn send_ipc_request<S>(stream: &mut S, req: IpcExtRequest) -> Result<IpcExtResponse, String>
where
    S: Read + Write + Unpin,
{
    //crate::enable_framed_transcript(true, 0);
    framed_send_blocking(stream, &req.encode())
        .map_err(|e| format!("failed to send request: {:?}", e))?;
    // TODO: if the return type is `()`, we do not need to wait for the response
    // Could be a nice optimization
    let response_enc =
        framed_recv_blocking(stream).map_err(|e| format!("failed to recv response: {:?}", e))?;
    let response = IpcExtResponse::decode(&mut response_enc.as_slice())
        .map_err(|e| format!("invalid response: {:?}", response_enc))?;

    Ok(response)
}

fn r_inner<S>(stream: &mut S, req: IpcExtRequest) -> IpcExtResponse
where
    S: Read + Write + Unpin,
{
    match req {
        IpcExtRequest::storage { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::storage(..)));
            res
        }
        IpcExtRequest::storage_hash { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::storage_hash(..)));
            res
        }
        IpcExtRequest::child_storage_hash { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::child_storage_hash(..)));
            res
        }
        IpcExtRequest::child_storage { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::child_storage(..)));
            res
        }
        IpcExtRequest::kill_child_storage { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::kill_child_storage(..)));
            res
        }
        IpcExtRequest::clear_prefix { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::clear_prefix(..)));
            res
        }
        IpcExtRequest::clear_child_prefix { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::clear_child_prefix(..)));
            res
        }
        IpcExtRequest::place_storage { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::place_storage(..)));
            res
        }
        IpcExtRequest::place_child_storage { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::place_child_storage(..)));
            res
        }
        IpcExtRequest::storage_root { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::storage_root(..)));
            res
        }
        IpcExtRequest::child_storage_root { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::child_storage_root(..)));
            res
        }
        IpcExtRequest::next_child_storage_key { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::next_child_storage_key(..)));
            res
        }
        IpcExtRequest::next_storage_key { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::next_storage_key(..)));
            res
        }
        IpcExtRequest::storage_append { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::storage_append(..)));
            res
        }
        IpcExtRequest::storage_start_transaction { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::storage_start_transaction(..)));
            res
        }
        IpcExtRequest::storage_rollback_transaction { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(
                res,
                IpcExtResponse::storage_rollback_transaction(..)
            ));
            res
        }
        IpcExtRequest::storage_commit_transaction { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(
                res,
                IpcExtResponse::storage_commit_transaction(..)
            ));
            res
        }
        IpcExtRequest::wipe { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::wipe(..)));
            res
        }
        IpcExtRequest::commit { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::commit(..)));
            res
        }
        IpcExtRequest::read_write_count { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::read_write_count(..)));
            res
        }
        IpcExtRequest::reset_read_write_count { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::reset_read_write_count(..)));
            res
        }
        IpcExtRequest::get_whitelist { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::get_whitelist(..)));
            res
        }
        IpcExtRequest::set_whitelist { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::set_whitelist(..)));
            res
        }
        IpcExtRequest::set_offchain_storage { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::set_offchain_storage(..)));
            res
        }
        IpcExtRequest::get_read_and_written_keys { .. } => {
            let res = send_ipc_request(stream, req).unwrap();
            assert!(matches!(res, IpcExtResponse::get_read_and_written_keys(..)));
            res
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Encode, Decode, Debug)]
pub enum IpcExtRequest {
    storage {
        key: Vec<u8>,
    },
    storage_hash {
        key: Vec<u8>,
    },
    child_storage_hash {
        child_info: ChildInfo,
        key: Vec<u8>,
    },
    child_storage {
        child_info: ChildInfo,
        key: Vec<u8>,
    },
    kill_child_storage {
        child_info: ChildInfo,
        maybe_limit: Option<u32>,
        maybe_cursor: Option<Vec<u8>>,
    },
    clear_prefix {
        prefix: Vec<u8>,
        maybe_limit: Option<u32>,
        maybe_cursor: Option<Vec<u8>>,
    },
    clear_child_prefix {
        child_info: ChildInfo,
        prefix: Vec<u8>,
        maybe_limit: Option<u32>,
        maybe_cursor: Option<Vec<u8>>,
    },
    place_storage {
        key: Vec<u8>,
        value: Option<Vec<u8>>,
    },
    place_child_storage {
        child_info: ChildInfo,
        key: Vec<u8>,
        value: Option<Vec<u8>>,
    },
    storage_root {
        state_version: sp_core::storage::StateVersion,
    },
    child_storage_root {
        child_info: ChildInfo,
        state_version: sp_core::storage::StateVersion,
    },
    next_child_storage_key {
        child_info: ChildInfo,
        key: Vec<u8>,
    },
    next_storage_key {
        key: Vec<u8>,
    },
    storage_append {
        key: Vec<u8>,
        value: Vec<u8>,
    },
    storage_start_transaction {},
    storage_rollback_transaction {},
    storage_commit_transaction {},
    wipe {},
    commit {},
    read_write_count {},
    reset_read_write_count {},
    get_whitelist {},
    set_whitelist {
        new: Vec<TrackedStorageKey>,
    },
    set_offchain_storage {
        key: Vec<u8>,
        value: Option<Vec<u8>>,
    },
    get_read_and_written_keys {},
}

#[allow(non_camel_case_types)]
#[derive(Encode, Decode)]
pub enum IpcExtResponse {
    storage(Option<Vec<u8>>),
    storage_hash(Option<Vec<u8>>),
    child_storage_hash(Option<Vec<u8>>),
    child_storage(Option<Vec<u8>>),
    kill_child_storage(MultiRemovalResults),
    clear_prefix(MultiRemovalResults),
    clear_child_prefix(MultiRemovalResults),
    place_storage(()),
    place_child_storage(()),
    storage_root(Vec<u8>),
    child_storage_root(Vec<u8>),
    next_child_storage_key(Option<Vec<u8>>),
    next_storage_key(Option<Vec<u8>>),
    storage_append(()),
    storage_start_transaction(()),
    storage_rollback_transaction(Result<(), ()>),
    storage_commit_transaction(Result<(), ()>),
    wipe(()),
    commit(()),
    read_write_count((u32, u32, u32, u32)),
    reset_read_write_count(()),
    get_whitelist(Vec<TrackedStorageKey>),
    set_whitelist(()),
    set_offchain_storage(()),
    get_read_and_written_keys(Vec<(Vec<u8>, u32, u32, bool)>),
}

impl TryFrom<IpcExtResponse> for () {
    type Error = ();

    fn try_from(x: IpcExtResponse) -> Result<Self, Self::Error> {
        Ok(match x {
            IpcExtResponse::place_storage(r) => r,
            IpcExtResponse::place_child_storage(r) => r,
            IpcExtResponse::storage_append(r) => r,
            IpcExtResponse::storage_start_transaction(r) => r,
            IpcExtResponse::wipe(r) => r,
            IpcExtResponse::commit(r) => r,
            IpcExtResponse::reset_read_write_count(r) => r,
            _ => return Err(()),
        })
    }
}
impl TryFrom<IpcExtResponse> for Result<(), ()> {
    type Error = ();

    fn try_from(x: IpcExtResponse) -> Result<Self, Self::Error> {
        Ok(match x {
            IpcExtResponse::storage_rollback_transaction(r) => r,
            IpcExtResponse::storage_commit_transaction(r) => r,
            _ => return Err(()),
        })
    }
}

impl TryFrom<IpcExtResponse> for Vec<u8> {
    type Error = ();

    fn try_from(x: IpcExtResponse) -> Result<Self, Self::Error> {
        Ok(match x {
            IpcExtResponse::storage_root(r) => r,
            IpcExtResponse::child_storage_root(r) => r,
            _ => return Err(()),
        })
    }
}

impl TryFrom<IpcExtResponse> for Option<Vec<u8>> {
    type Error = ();

    fn try_from(x: IpcExtResponse) -> Result<Self, Self::Error> {
        Ok(match x {
            IpcExtResponse::storage(r) => r,
            IpcExtResponse::storage_hash(r) => r,
            IpcExtResponse::child_storage_hash(r) => r,
            IpcExtResponse::child_storage(r) => r,
            IpcExtResponse::next_child_storage_key(r) => r,
            IpcExtResponse::next_storage_key(r) => r,
            _ => return Err(()),
        })
    }
}

impl TryFrom<IpcExtResponse> for MultiRemovalResults {
    type Error = ();

    fn try_from(x: IpcExtResponse) -> Result<Self, Self::Error> {
        Ok(match x {
            IpcExtResponse::kill_child_storage(r) => r,
            IpcExtResponse::clear_prefix(r) => r,
            IpcExtResponse::clear_child_prefix(r) => r,
            _ => return Err(()),
        })
    }
}

impl TryFrom<IpcExtResponse> for (u32, u32, u32, u32) {
    type Error = ();

    fn try_from(x: IpcExtResponse) -> Result<Self, Self::Error> {
        Ok(match x {
            IpcExtResponse::read_write_count(r) => r,
            _ => return Err(()),
        })
    }
}

impl TryFrom<IpcExtResponse> for Vec<TrackedStorageKey> {
    type Error = ();

    fn try_from(x: IpcExtResponse) -> Result<Self, Self::Error> {
        Ok(match x {
            IpcExtResponse::get_whitelist(r) => r,
            _ => return Err(()),
        })
    }
}

impl TryFrom<IpcExtResponse> for Vec<(Vec<u8>, u32, u32, bool)> {
    type Error = ();

    fn try_from(x: IpcExtResponse) -> Result<Self, Self::Error> {
        Ok(match x {
            IpcExtResponse::get_read_and_written_keys(r) => r,
            _ => return Err(()),
        })
    }
}

impl<S> Externalities for IpcExternalities<S>
where
    S: Read + Write + Unpin,
{
    fn storage(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.r(IpcExtRequest::storage { key: key.to_vec() })
    }

    fn storage_hash(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.r(IpcExtRequest::storage_hash { key: key.to_vec() })
    }

    fn child_storage_hash(&self, child_info: &ChildInfo, key: &[u8]) -> Option<Vec<u8>> {
        self.r(IpcExtRequest::child_storage_hash {
            child_info: child_info.clone(),
            key: key.to_vec(),
        })
    }

    fn child_storage(&self, child_info: &ChildInfo, key: &[u8]) -> Option<Vec<u8>> {
        self.r(IpcExtRequest::child_storage {
            child_info: child_info.clone(),
            key: key.to_vec(),
        })
    }

    fn kill_child_storage(
        &mut self,
        child_info: &ChildInfo,
        maybe_limit: Option<u32>,
        maybe_cursor: Option<&[u8]>,
    ) -> MultiRemovalResults {
        self.r(IpcExtRequest::kill_child_storage {
            child_info: child_info.clone(),
            maybe_limit,
            maybe_cursor: maybe_cursor.map(|x| x.to_vec()),
        })
    }

    fn clear_prefix(
        &mut self,
        prefix: &[u8],
        maybe_limit: Option<u32>,
        maybe_cursor: Option<&[u8]>,
    ) -> MultiRemovalResults {
        self.r(IpcExtRequest::clear_prefix {
            prefix: prefix.to_vec(),
            maybe_limit,
            maybe_cursor: maybe_cursor.map(|x| x.to_vec()),
        })
    }

    fn clear_child_prefix(
        &mut self,
        child_info: &ChildInfo,
        prefix: &[u8],
        maybe_limit: Option<u32>,
        maybe_cursor: Option<&[u8]>,
    ) -> MultiRemovalResults {
        self.r(IpcExtRequest::clear_child_prefix {
            child_info: child_info.clone(),
            prefix: prefix.to_vec(),
            maybe_limit,
            maybe_cursor: maybe_cursor.map(|x| x.to_vec()),
        })
    }

    fn place_storage(&mut self, key: Vec<u8>, value: Option<Vec<u8>>) {
        self.r(IpcExtRequest::place_storage { key, value })
    }

    fn place_child_storage(
        &mut self,
        child_info: &ChildInfo,
        key: Vec<u8>,
        value: Option<Vec<u8>>,
    ) {
        self.r(IpcExtRequest::place_child_storage {
            child_info: child_info.clone(),
            key,
            value,
        })
    }

    fn storage_root(&mut self, state_version: sp_core::storage::StateVersion) -> Vec<u8> {
        self.r(IpcExtRequest::storage_root { state_version })
    }

    fn child_storage_root(
        &mut self,
        child_info: &ChildInfo,
        state_version: sp_core::storage::StateVersion,
    ) -> Vec<u8> {
        self.r(IpcExtRequest::child_storage_root {
            child_info: child_info.clone(),
            state_version,
        })
    }

    fn next_child_storage_key(&self, child_info: &ChildInfo, key: &[u8]) -> Option<Vec<u8>> {
        self.r(IpcExtRequest::next_child_storage_key {
            child_info: child_info.clone(),
            key: key.to_vec(),
        })
    }

    fn next_storage_key(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.r(IpcExtRequest::next_storage_key { key: key.to_vec() })
    }

    fn storage_append(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.r(IpcExtRequest::storage_append { key, value })
    }

    fn storage_start_transaction(&mut self) {
        self.r(IpcExtRequest::storage_start_transaction {})
    }

    fn storage_rollback_transaction(&mut self) -> Result<(), ()> {
        self.r(IpcExtRequest::storage_rollback_transaction {})
    }

    fn storage_commit_transaction(&mut self) -> Result<(), ()> {
        self.r(IpcExtRequest::storage_commit_transaction {})
    }

    fn wipe(&mut self) {
        self.r(IpcExtRequest::wipe {})
    }

    fn commit(&mut self) {
        self.r(IpcExtRequest::commit {})
    }

    fn read_write_count(&self) -> (u32, u32, u32, u32) {
        self.r(IpcExtRequest::read_write_count {})
    }

    fn reset_read_write_count(&mut self) {
        self.r(IpcExtRequest::reset_read_write_count {})
    }

    fn get_whitelist(&self) -> Vec<TrackedStorageKey> {
        self.r(IpcExtRequest::get_whitelist {})
    }

    fn set_whitelist(&mut self, new: Vec<TrackedStorageKey>) {
        self.r(IpcExtRequest::set_whitelist { new })
    }

    fn set_offchain_storage(&mut self, key: &[u8], value: std::option::Option<&[u8]>) {
        self.r(IpcExtRequest::set_offchain_storage {
            key: key.to_vec(),
            value: value.map(|x| x.to_vec()),
        })
    }

    fn get_read_and_written_keys(&self) -> Vec<(Vec<u8>, u32, u32, bool)> {
        self.r(IpcExtRequest::get_read_and_written_keys {})
    }
}

impl<S> sp_externalities::ExtensionStore for IpcExternalities<S>
where
    S: Read + Write + Unpin,
{
    fn extension_by_type_id(&mut self, type_id: TypeId) -> Option<&mut dyn Any> {
        self.extensions.get_mut(type_id)
    }

    fn register_extension_with_type_id(
        &mut self,
        type_id: TypeId,
        extension: Box<dyn sp_externalities::Extension>,
    ) -> Result<(), sp_externalities::Error> {
        self.extensions.register_with_type_id(type_id, extension)
    }

    fn deregister_extension_by_type_id(
        &mut self,
        type_id: TypeId,
    ) -> Result<(), sp_externalities::Error> {
        if self.extensions.deregister(type_id) {
            Ok(())
        } else {
            Err(sp_externalities::Error::ExtensionIsNotRegistered(type_id))
        }
    }
}

pub fn handle_ipc_ext_req(
    ext: &mut dyn Externalities,
    ipc_ext_request: IpcExtRequest,
) -> IpcExtResponse {
    match ipc_ext_request {
        IpcExtRequest::storage { key } => {
            let res = ext.storage(&key);
            IpcExtResponse::storage(res)
        }
        IpcExtRequest::storage_hash { key } => {
            let res = ext.storage_hash(&key);
            IpcExtResponse::storage_hash(res)
        }
        IpcExtRequest::child_storage_hash { child_info, key } => {
            let res = ext.child_storage_hash(&child_info, &key);
            IpcExtResponse::child_storage_hash(res)
        }
        IpcExtRequest::child_storage { child_info, key } => {
            let res = ext.child_storage(&child_info, &key);
            IpcExtResponse::child_storage(res)
        }
        IpcExtRequest::kill_child_storage {
            child_info,
            maybe_limit,
            maybe_cursor,
        } => {
            let res = ext.kill_child_storage(&child_info, maybe_limit, maybe_cursor.as_deref());
            IpcExtResponse::kill_child_storage(res)
        }
        IpcExtRequest::clear_prefix {
            prefix,
            maybe_limit,
            maybe_cursor,
        } => {
            let res = ext.clear_prefix(&prefix, maybe_limit, maybe_cursor.as_deref());
            IpcExtResponse::clear_prefix(res)
        }
        IpcExtRequest::clear_child_prefix {
            child_info,
            prefix,
            maybe_limit,
            maybe_cursor,
        } => {
            let res =
                ext.clear_child_prefix(&child_info, &prefix, maybe_limit, maybe_cursor.as_deref());
            IpcExtResponse::clear_child_prefix(res)
        }
        IpcExtRequest::place_storage { key, value } => {
            let res = ext.place_storage(key, value);
            IpcExtResponse::place_storage(res)
        }
        IpcExtRequest::place_child_storage {
            child_info,
            key,
            value,
        } => {
            let res = ext.place_child_storage(&child_info, key, value);
            IpcExtResponse::place_child_storage(res)
        }
        IpcExtRequest::storage_root { state_version } => {
            let res = ext.storage_root(state_version);
            IpcExtResponse::storage_root(res)
        }
        IpcExtRequest::child_storage_root {
            child_info,
            state_version,
        } => {
            let res = ext.child_storage_root(&child_info, state_version);
            IpcExtResponse::child_storage_root(res)
        }
        IpcExtRequest::next_child_storage_key { child_info, key } => {
            let res = ext.next_child_storage_key(&child_info, &key);
            IpcExtResponse::next_child_storage_key(res)
        }
        IpcExtRequest::next_storage_key { key } => {
            let res = ext.next_storage_key(&key);
            IpcExtResponse::next_storage_key(res)
        }
        IpcExtRequest::storage_append { key, value } => {
            let res = ext.storage_append(key, value);
            IpcExtResponse::storage_append(res)
        }
        IpcExtRequest::storage_start_transaction {} => {
            let res = ext.storage_start_transaction();
            IpcExtResponse::storage_start_transaction(res)
        }
        IpcExtRequest::storage_rollback_transaction {} => {
            let res = ext.storage_rollback_transaction();
            IpcExtResponse::storage_rollback_transaction(res)
        }
        IpcExtRequest::storage_commit_transaction {} => {
            let res = ext.storage_commit_transaction();
            IpcExtResponse::storage_commit_transaction(res)
        }
        IpcExtRequest::wipe {} => {
            let res = ext.wipe();
            IpcExtResponse::wipe(res)
        }
        IpcExtRequest::commit {} => {
            let res = ext.commit();
            IpcExtResponse::commit(res)
        }
        IpcExtRequest::read_write_count {} => {
            let res = ext.read_write_count();
            IpcExtResponse::read_write_count(res)
        }
        IpcExtRequest::reset_read_write_count {} => {
            let res = ext.reset_read_write_count();
            IpcExtResponse::reset_read_write_count(res)
        }
        IpcExtRequest::get_whitelist {} => {
            let res = ext.get_whitelist();
            IpcExtResponse::get_whitelist(res)
        }
        IpcExtRequest::set_whitelist { new } => {
            let res = ext.set_whitelist(new);
            IpcExtResponse::set_whitelist(res)
        }
        IpcExtRequest::set_offchain_storage { key, value } => {
            let res = ext.set_offchain_storage(&key, value.as_deref());
            IpcExtResponse::set_offchain_storage(res)
        }
        IpcExtRequest::get_read_and_written_keys {} => {
            let res = ext.get_read_and_written_keys();
            IpcExtResponse::get_read_and_written_keys(res)
        }
    }
}

pub struct PipePair<R: Read, W: Write>(R, W);

impl<R, W> Read for PipePair<R, W>
where
    R: Read,
    W: Write,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<R, W> Write for PipePair<R, W>
where
    R: Read,
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.1.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.1.flush()
    }
}
