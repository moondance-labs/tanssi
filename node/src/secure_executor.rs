use self::candidate_validation::execute_candidate_exhaustive;
use crate::service::container_log_str;
use futures::channel::oneshot;
use futures::channel::oneshot::Sender;
use futures::FutureExt;
use jsonrpsee::tracing;
use node_common::service::TanssiExecutorExt;
use parity_scale_codec::{Decode, Encode};
use polkadot_node_core_candidate_validation::Config as CvConfig;
use polkadot_node_subsystem_types::messages::ValidationFailed;
use polkadot_parachain_primitives::primitives::ValidationCode;
use polkadot_primitives::{ExecutorParams, PersistedValidationData};
use sc_cli::RuntimeVersion;
use sc_executor::sp_wasm_interface::HostFunctions;
use sc_executor::{Externalities, RuntimeVersionOf, WasmExecutor};
use sc_executor_common::error::Error;
use sc_executor_common::runtime_blob::RuntimeBlob;
use sc_executor_common::wasm_runtime::{HeapAllocStrategy, DEFAULT_HEAP_ALLOC_STRATEGY};
use sc_service::SpawnTaskHandle;
use sp_core::traits::{
    CallContext, CodeExecutor, FetchRuntimeCode, ReadRuntimeVersion, RuntimeCode,
};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tanssi_worker::ValidationHost;
use tanssi_worker_common::executor_interface::IpcExtRequest;
use tanssi_worker_common::executor_interface::{handle_ipc_ext_req, IpcExtResponse};
use tc_consensus::ParaId;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

#[derive(Clone)]
pub struct MaybeSecureWasmExecutor {
    pub wasm_executor: WasmExecutor<sp_io::SubstrateHostFunctions>,
    pub secure_executor: SecureWasmExecutor<sp_io::SubstrateHostFunctions>,
    pub use_secure: bool,
}

impl CodeExecutor for MaybeSecureWasmExecutor {
    type Error = sc_executor_common::error::Error;

    fn call(
        &self,
        ext: &mut dyn Externalities,
        runtime_code: &RuntimeCode,
        method: &str,
        data: &[u8],
        context: CallContext,
    ) -> (Result<Vec<u8>, Self::Error>, bool) {
        if self.use_secure {
            self.secure_executor
                .call(ext, runtime_code, method, data, context)
        } else {
            self.wasm_executor
                .call(ext, runtime_code, method, data, context)
        }
    }
}

impl RuntimeVersionOf for MaybeSecureWasmExecutor {
    fn runtime_version(
        &self,
        ext: &mut dyn Externalities,
        runtime_code: &RuntimeCode,
    ) -> sc_executor_common::error::Result<RuntimeVersion> {
        if self.use_secure {
            self.secure_executor.runtime_version(ext, runtime_code)
        } else {
            self.wasm_executor.runtime_version(ext, runtime_code)
        }
    }
}

impl ReadRuntimeVersion for MaybeSecureWasmExecutor {
    fn read_runtime_version(
        &self,
        wasm_code: &[u8],
        ext: &mut dyn Externalities,
    ) -> Result<Vec<u8>, String> {
        if self.use_secure {
            self.secure_executor.read_runtime_version(wasm_code, ext)
        } else {
            self.wasm_executor.read_runtime_version(wasm_code, ext)
        }
    }
}

impl TanssiExecutorExt for MaybeSecureWasmExecutor {
    type HostFun = sp_io::SubstrateHostFunctions;

    fn new_with_wasm_executor(wasm_executor: WasmExecutor<Self::HostFun>) -> Self {
        Self {
            wasm_executor,
            secure_executor: SecureWasmExecutor::new(),
            use_secure: true,
        }
    }
}

pub struct SecureWasmExecutor<H> {
    /// The heap allocation strategy for onchain Wasm calls.
    default_onchain_heap_alloc_strategy: HeapAllocStrategy,
    /// The heap allocation strategy for offchain Wasm calls.
    default_offchain_heap_alloc_strategy: HeapAllocStrategy,
    /// Ignore onchain heap pages value.
    ignore_onchain_heap_pages: bool,
    // Need to use interior mutability because constructing this type is only easy if we use
    // new_with_wasm_executor
    pvf_tx: Arc<Mutex<Option<UnboundedSender<PvfMsg>>>>,
    phantom: PhantomData<H>,
}

impl<H> Clone for SecureWasmExecutor<H> {
    fn clone(&self) -> Self {
        Self {
            default_onchain_heap_alloc_strategy: self.default_onchain_heap_alloc_strategy,
            default_offchain_heap_alloc_strategy: self.default_offchain_heap_alloc_strategy,
            ignore_onchain_heap_pages: self.ignore_onchain_heap_pages,
            pvf_tx: self.pvf_tx.clone(),
            phantom: self.phantom,
        }
    }
}

fn unwrap_heap_pages(pages: Option<HeapAllocStrategy>) -> HeapAllocStrategy {
    pages.unwrap_or_else(|| DEFAULT_HEAP_ALLOC_STRATEGY)
}

impl<H> SecureWasmExecutor<H> {
    pub fn new() -> Self {
        let default_heap_pages: Option<u64> = None;

        Self {
            default_onchain_heap_alloc_strategy: unwrap_heap_pages(default_heap_pages.map(|h| {
                HeapAllocStrategy::Static {
                    extra_pages: h as _,
                }
            })),
            default_offchain_heap_alloc_strategy: unwrap_heap_pages(default_heap_pages.map(|h| {
                HeapAllocStrategy::Static {
                    extra_pages: h as _,
                }
            })),
            ignore_onchain_heap_pages: false,
            pvf_tx: Arc::new(Mutex::new(None)),
            phantom: PhantomData,
        }
    }

    pub fn initialize_pvf_tx(&self, new_pvf_tx: UnboundedSender<PvfMsg>) {
        let mut pvf_tx = self.pvf_tx.lock().unwrap();
        assert!(
            pvf_tx.is_none(),
            "Attempted to double initialize SecureWasmExecutor"
        );
        *pvf_tx = Some(new_pvf_tx);
    }

    /// Perform a call into the given runtime.
    ///
    /// The runtime is passed as a [`RuntimeBlob`]. The runtime will be instantiated with the
    /// parameters this `WasmExecutor` was initialized with.
    ///
    /// In case of problems with during creation of the runtime or instantiation, a `Err` is
    /// returned. that describes the message.
    pub fn uncached_call(
        &self,
        runtime_blob: RuntimeBlob,
        ext: &mut dyn Externalities,
        allow_missing_host_functions: bool,
        export_name: &str,
        call_data: &[u8],
    ) -> std::result::Result<Vec<u8>, Error> {
        // TODO: so far this function looks not needed
        let start_instant = Instant::now();
        log::info!("uncached_call START {}", export_name);
        let pvf_tx_lock = self.pvf_tx.lock().unwrap();
        let pvf_tx = pvf_tx_lock.clone().unwrap();
        drop(pvf_tx_lock);
        let (tx, rx) = std::sync::mpsc::channel();

        // TODO: real executor params
        let executor_params = ExecutorParams::default();
        let persisted_validation_data = Default::default();
        let validation_code = ValidationCode(runtime_blob.serialize());
        pvf_tx
            .send(PvfMsg::ValidateFromExhaustive {
                persisted_validation_data,
                validation_code,
                executor_params,
                response_sender: tx,
                export_name: export_name.to_string(),
                call_data: call_data.to_vec(),
            })
            .unwrap();
        // TODO: ^ this unwrap can happen when the container is stopping

        // PERF: at this point we have only spent 0.15ms, good enough
        // The first received message arrives at 25ms, then each message takes less than 0.1ms to
        // be handled on average. The last message (Done) is received 3ms after the last response.
        let res = loop {
            log::error!(
                "SECURE_EXECUTOR DURATION at {}: {}us",
                line!(),
                start_instant.elapsed().as_micros()
            );

            let exec_timeout = Duration::from_millis(2_000);
            match rx.recv_timeout(exec_timeout) {
                Ok(msg) => match msg {
                    PvfMsgResponse::Done(x) => break x,
                    PvfMsgResponse::IpcExtReq(ipc_ext_req, ipc_ext_tx) => {
                        let ipc_ext_response = handle_ipc_ext_req(ext, ipc_ext_req);
                        let _ = ipc_ext_tx.send(ipc_ext_response);
                    }
                },
                Err(e) => {
                    break Err(ValidationFailed(format!(
                        "failed to recv from channel: {:?}",
                        e
                    )))
                }
            }
        };

        log::info!("with_instance_call_export DONE  {}", export_name);
        log::info!("res: {:?}", res);

        log::error!(
            "SECURE_EXECUTOR DURATION at {}: {}us",
            line!(),
            start_instant.elapsed().as_micros()
        );

        match res {
            Ok(x) => Ok(x),
            Err(e) => Err(Error::Other(format!("{:?}", e))),
        }
    }

    pub fn with_instance_returns_version(
        &self,
        runtime_code: &RuntimeCode,
        ext: &mut dyn Externalities,
        heap_alloc_strategy: HeapAllocStrategy,
    ) -> sc_executor_common::error::Result<RuntimeVersion> {
        // TODO: how does with_instance get the runtime version?
        // Exactly the same as read_runtime_version, including a call to Core_version if the
        // version is not found in constants.
        let version: Option<&RuntimeVersion> = None;

        let runtime_blob =
            RuntimeBlob::uncompress_if_needed(&*runtime_code.fetch_runtime_code().unwrap())
                .map_err(|e| format!("Failed to create runtime blob: {:?}", e))?;

        if let Some(version) = sc_executor::read_embedded_version(&runtime_blob)
            .map_err(|e| format!("Failed to read the static section: {:?}", e))
            .map(|v| v.map(|v| v.encode()))?
        {
            return RuntimeVersion::decode(&mut version.as_slice())
                .map_err(|e| format!("Failed to decode static version: {:?}", e).into());
        }

        // TODO: this should call Core_version

        version
            .cloned()
            .ok_or_else(|| Error::ApiError("Unknown version".into()))
    }

    pub fn with_instance_call_export(
        &self,
        runtime_code: &RuntimeCode,
        ext: &mut dyn Externalities,
        heap_alloc_strategy: HeapAllocStrategy,
        export_name: &str,
        call_data: &[u8],
    ) -> sc_executor_common::error::Result<Vec<u8>> {
        let start_instant = Instant::now();
        log::info!("with_instance_call_export START {}", export_name);
        // TODO: similar to uncached_call
        // See also create_versioned_wasm_runtime
        let pvf_tx_lock = self.pvf_tx.lock().unwrap();
        let pvf_tx = pvf_tx_lock.clone().unwrap();
        drop(pvf_tx_lock);
        let (tx, rx) = std::sync::mpsc::channel();

        // TODO: real executor params
        let executor_params = ExecutorParams::default();
        let persisted_validation_data = Default::default();
        let validation_code = ValidationCode(
            runtime_code
                .code_fetcher
                .fetch_runtime_code()
                .unwrap()
                .to_vec(),
        );

        pvf_tx
            .send(PvfMsg::ValidateFromExhaustive {
                persisted_validation_data,
                validation_code,
                executor_params,
                response_sender: tx,
                export_name: export_name.to_string(),
                call_data: call_data.to_vec(),
            })
            .unwrap();
        // TODO: ^ this unwrap can happen when the container is stopping

        // PERF: at this point we have only spent 0.15ms, good enough
        // The first received message arrives at 25ms, then each message takes less than 0.1ms to
        // be handled on average. The last message (Done) is received 3ms after the last response.
        let res = loop {
            log::error!(
                "SECURE_EXECUTOR DURATION at {}: {}us",
                line!(),
                start_instant.elapsed().as_micros()
            );

            let exec_timeout = Duration::from_millis(2_000);
            match rx.recv_timeout(exec_timeout) {
                Ok(msg) => match msg {
                    PvfMsgResponse::Done(x) => break x,
                    PvfMsgResponse::IpcExtReq(ipc_ext_req, ipc_ext_tx) => {
                        let ipc_ext_response = handle_ipc_ext_req(ext, ipc_ext_req);
                        let _ = ipc_ext_tx.send(ipc_ext_response);
                    }
                },
                Err(e) => {
                    break Err(ValidationFailed(format!(
                        "failed to recv from channel: {:?}",
                        e
                    )))
                }
            }
        };

        log::info!("with_instance_call_export DONE  {}", export_name);
        log::info!("res: {:?}", res);

        log::error!(
            "SECURE_EXECUTOR DURATION at {}: {}us",
            line!(),
            start_instant.elapsed().as_micros()
        );

        match res {
            Ok(x) => Ok(x),
            Err(e) => Err(Error::Other(format!("{:?}", e))),
        }
    }
}

impl<H> CodeExecutor for SecureWasmExecutor<H>
where
    H: HostFunctions,
{
    type Error = sc_executor_common::error::Error;

    fn call(
        &self,
        ext: &mut dyn Externalities,
        runtime_code: &RuntimeCode,
        method: &str,
        data: &[u8],
        context: CallContext,
    ) -> (Result<Vec<u8>, Self::Error>, bool) {
        tracing::trace!(
            target: "executor",
            %method,
            "Executing function",
        );

        let on_chain_heap_alloc_strategy = if self.ignore_onchain_heap_pages {
            self.default_onchain_heap_alloc_strategy
        } else {
            runtime_code
                .heap_pages
                .map(|h| HeapAllocStrategy::Static {
                    extra_pages: h as _,
                })
                .unwrap_or_else(|| self.default_onchain_heap_alloc_strategy)
        };

        let heap_alloc_strategy = match context {
            CallContext::Offchain => self.default_offchain_heap_alloc_strategy,
            CallContext::Onchain => on_chain_heap_alloc_strategy,
        };

        let result =
            self.with_instance_call_export(runtime_code, ext, heap_alloc_strategy, method, data);

        (result, false)
    }
}

impl<H> RuntimeVersionOf for SecureWasmExecutor<H>
where
    H: HostFunctions,
{
    fn runtime_version(
        &self,
        ext: &mut dyn Externalities,
        runtime_code: &RuntimeCode,
    ) -> sc_executor_common::error::Result<RuntimeVersion> {
        let on_chain_heap_pages = if self.ignore_onchain_heap_pages {
            self.default_onchain_heap_alloc_strategy
        } else {
            runtime_code
                .heap_pages
                .map(|h| HeapAllocStrategy::Static {
                    extra_pages: h as _,
                })
                .unwrap_or_else(|| self.default_onchain_heap_alloc_strategy)
        };

        self.with_instance_returns_version(runtime_code, ext, on_chain_heap_pages)
    }
}

impl<H> ReadRuntimeVersion for SecureWasmExecutor<H>
where
    H: HostFunctions,
{
    fn read_runtime_version(
        &self,
        wasm_code: &[u8],
        ext: &mut dyn Externalities,
    ) -> Result<Vec<u8>, String> {
        let runtime_blob = RuntimeBlob::uncompress_if_needed(wasm_code)
            .map_err(|e| format!("Failed to create runtime blob: {:?}", e))?;

        if let Some(version) = sc_executor::read_embedded_version(&runtime_blob)
            .map_err(|e| format!("Failed to read the static section: {:?}", e))
            .map(|v| v.map(|v| v.encode()))?
        {
            return Ok(version);
        }

        // If the blob didn't have embedded runtime version section, we fallback to the legacy
        // way of fetching the version: i.e. instantiating the given instance and calling
        // `Core_version` on it.

        self.uncached_call(
            runtime_blob,
            ext,
            // If a runtime upgrade introduces new host functions that are not provided by
            // the node, we should not fail at instantiation. Otherwise nodes that are
            // updated could run this successfully and it could lead to a storage root
            // mismatch when importing this block.
            true,
            "Core_version",
            &[],
        )
        .map_err(|e| e.to_string())
    }
}

#[sc_tracing::logging::prefix_logs_with(container_log_str(para_id))]
pub async fn spawn_candidate_validation(
    handle: SpawnTaskHandle,
    //metrics: polkadot_node_core_candidate_validation::metrics::Metrics,
    pvf_metrics: tanssi_worker::Metrics,
    CvConfig {
        artifacts_cache_path,
        node_version,
        secure_validator_mode,
        prep_worker_path,
        exec_worker_path,
    }: CvConfig,
    para_id: ParaId,
) -> sc_service::error::Result<(ValidationHost<PvfMsgResponse>, UnboundedSender<PvfMsg>)> {
    let (validation_host, task) = tanssi_worker::start(
        tanssi_worker::Config::new(
            artifacts_cache_path,
            node_version,
            secure_validator_mode,
            prep_worker_path,
            exec_worker_path,
        ),
        pvf_metrics,
    )
    .await
    .map_err(|e| sc_service::Error::Application(Box::new(e)))?;
    // TODO: spawn_essential_blocking?
    handle.spawn_blocking("pvf-validation-host", None, task.boxed());
    let handle2 = handle.clone();
    let validation_host2 = (&validation_host).clone();
    let (tx, rx) = unbounded_channel();
    // TODO: spawn_essential_blocking?
    // Using spawn_blocking to have this run in a separate thread, because we block inside
    // with_instance_call_export
    handle.spawn_blocking("pvf-validation-rx-loop", None, async move {
        pvf_rx_loop(rx, handle2, validation_host2, para_id)
            .await
            .map_err(|e| {
                // Panic is rx loop fails
                panic!("{:?}", e)
            })
            .unwrap()
    });

    Ok((validation_host, tx))
}

#[derive(Debug)]
pub enum PvfMsg {
    Conclude,
    ValidateFromChainState,
    ValidateFromExhaustive {
        persisted_validation_data: PersistedValidationData,
        validation_code: ValidationCode,
        executor_params: ExecutorParams,
        response_sender: std::sync::mpsc::Sender<PvfMsgResponse>,
        export_name: String,
        call_data: Vec<u8>,
    },
    PreCheck,
}

pub enum PvfMsgResponse {
    Done(Result<Vec<u8>, ValidationFailed>),
    IpcExtReq(IpcExtRequest, oneshot::Sender<IpcExtResponse>),
}

impl From<(IpcExtRequest, oneshot::Sender<IpcExtResponse>)> for PvfMsgResponse {
    fn from(x: (IpcExtRequest, Sender<IpcExtResponse>)) -> Self {
        Self::IpcExtReq(x.0, x.1)
    }
}

#[sc_tracing::logging::prefix_logs_with(container_log_str(para_id))]
pub async fn pvf_rx_loop(
    mut rx: mpsc::UnboundedReceiver<PvfMsg>,
    handle: SpawnTaskHandle,
    validation_host: ValidationHost<PvfMsgResponse>,
    para_id: ParaId,
) -> sc_service::error::Result<()> {
    loop {
        match rx.recv().await.ok_or_else(|| {
            sc_service::Error::Other("pvf_rx_loop: rx.recv() returned None".to_string())
        })? {
            PvfMsg::Conclude => return Ok(()),
            PvfMsg::PreCheck => {
                /*
                let bg = {
                    let mut sender = ctx.sender().clone();
                    let validation_host = validation_host.clone();

                    async move {
                        let precheck_result = precheck_pvf(
                            &mut sender,
                            validation_host,
                            relay_parent,
                            validation_code_hash,
                        )
                            .await;

                        let _ = response_sender.send(precheck_result);
                    }
                };

                ctx.spawn("candidate-validation-pre-check", bg.boxed())?;
                 */
                // No-op? This should take the runtime code as input and pre-compile it, so that
                // the execute job can start immediately.
                // But the handle_execute_pvf function will send a request to prepare it anyway if
                // it has not been prepared previously.
            }
            PvfMsg::ValidateFromExhaustive {
                persisted_validation_data,
                validation_code,
                executor_params,
                response_sender,
                export_name,
                call_data,
            } => {
                let bg = {
                    //let metrics = metrics.clone();
                    let validation_host = validation_host.clone();

                    async move {
                        //let _timer = metrics.time_validate_from_exhaustive();
                        let res = execute_candidate_exhaustive(
                            validation_host,
                            persisted_validation_data,
                            validation_code,
                            //candidate_receipt,
                            //pov,
                            executor_params,
                            //&metrics,
                            para_id,
                            export_name,
                            call_data,
                            response_sender.clone(),
                        )
                        .await;

                        //metrics.on_validation_event(&res);
                        let _ = response_sender.send(PvfMsgResponse::Done(res));
                    }
                };

                handle.spawn("validate-from-exhaustive", None, bg.boxed());
            }
            msg => unimplemented!("{:?}", msg),
            /*
            CandidateValidationMessage::ValidateFromChainState {
                candidate_receipt,
                pov,
                executor_params,
                exec_kind,
                response_sender,
                ..
            } => {
                let bg = {
                    let mut sender = ctx.sender().clone();
                    let metrics = metrics.clone();
                    let validation_host = validation_host.clone();

                    async move {
                        let _timer = metrics.time_validate_from_chain_state();
                        let res = validate_from_chain_state(
                            &mut sender,
                            validation_host,
                            candidate_receipt,
                            pov,
                            executor_params,
                            exec_kind,
                            &metrics,
                        )
                            .await;

                        metrics.on_validation_event(&res);
                        let _ = response_sender.send(res);
                    }
                };

                ctx.spawn("validate-from-chain-state", bg.boxed())?;
            },
            CandidateValidationMessage::ValidateFromExhaustive {
                validation_data,
                validation_code,
                candidate_receipt,
                pov,
                executor_params,
                exec_kind,
                response_sender,
                ..
            } => {
                let bg = {
                    let metrics = metrics.clone();
                    let validation_host = validation_host.clone();

                    async move {
                        let _timer = metrics.time_validate_from_exhaustive();
                        let res = validate_candidate_exhaustive(
                            validation_host,
                            validation_data,
                            validation_code,
                            candidate_receipt,
                            pov,
                            executor_params,
                            exec_kind,
                            &metrics,
                        )
                            .await;

                        metrics.on_validation_event(&res);
                        let _ = response_sender.send(res);
                    }
                };

                ctx.spawn("validate-from-exhaustive", bg.boxed())?;
            },
            CandidateValidationMessage::PreCheck {
                relay_parent,
                validation_code_hash,
                response_sender,
                ..
            } => {
                let bg = {
                    let mut sender = ctx.sender().clone();
                    let validation_host = validation_host.clone();

                    async move {
                        let precheck_result = precheck_pvf(
                            &mut sender,
                            validation_host,
                            relay_parent,
                            validation_code_hash,
                        )
                            .await;

                        let _ = response_sender.send(precheck_result);
                    }
                };

                ctx.spawn("candidate-validation-pre-check", bg.boxed())?;
            },
             */
        }
    }
}

mod candidate_validation {
    use crate::secure_executor::PvfMsgResponse;
    use async_trait::async_trait;
    use cumulus_primitives_core::ParaId;
    use futures::channel::oneshot;
    use polkadot_node_subsystem_types::messages::ValidationFailed;
    use polkadot_parachain_primitives::primitives::ValidationCode;
    use polkadot_primitives::executor_params::{
        DEFAULT_APPROVAL_EXECUTION_TIMEOUT, DEFAULT_BACKING_EXECUTION_TIMEOUT,
        DEFAULT_LENIENT_PREPARATION_TIMEOUT, DEFAULT_PRECHECK_PREPARATION_TIMEOUT,
    };
    use polkadot_primitives::{ExecutorParams, PersistedValidationData, PvfExecKind, PvfPrepKind};
    use std::time::Duration;
    use tanssi_worker::InternalValidationError;
    use tanssi_worker::{ValidationError, ValidationHost};
    use tanssi_worker_common::prepare::PrepareJobKind;
    use tanssi_worker_common::pvf::PvfPrepData;
    use tracing_gum as gum;

    const LOG_TARGET: &str = "candidate-validation";

    pub async fn execute_candidate_exhaustive(
        mut validation_backend: impl SecureExecutionBackend + Send,
        persisted_validation_data: PersistedValidationData,
        validation_code: ValidationCode,
        executor_params: ExecutorParams,
        //metrics: &Metrics,
        para_id: ParaId,
        export_name: String,
        call_data: Vec<u8>,
        response_sender: std::sync::mpsc::Sender<PvfMsgResponse>,
    ) -> Result<Vec<u8>, ValidationFailed> {
        //let _timer = metrics.time_validate_candidate_exhaustive();

        let validation_code_hash = validation_code.hash();
        gum::debug!(
            target: LOG_TARGET,
            ?validation_code_hash,
            ?para_id,
            "About to validate a candidate.",
        );

        // polkadot/node/primitives/src/lib.rs
        pub const VALIDATION_CODE_BOMB_LIMIT: usize =
            (polkadot_primitives::MAX_CODE_SIZE * 4u32) as usize;

        let raw_validation_code = match sp_maybe_compressed_blob::decompress(
            &validation_code.0,
            VALIDATION_CODE_BOMB_LIMIT,
        ) {
            Ok(code) => code,
            Err(e) => {
                gum::info!(target: LOG_TARGET, ?para_id, err=?e, "Invalid candidate (validation code)");

                // Code already passed pre-checking, if decompression fails now this most likely means
                // some local corruption happened.
                return Err(ValidationFailed("Code decompression failed".to_string()));
            }
        };
        //metrics.observe_code_size(raw_validation_code.len());
        //metrics.observe_pov_size(pov.block_data.0.len(), true);
        /*
        let raw_block_data =
            match sp_maybe_compressed_blob::decompress(&pov.block_data.0, POV_BOMB_LIMIT) {
                Ok(block_data) => BlockData(block_data.to_vec()),
                Err(e) => {
                    gum::info!(target: LOG_TARGET, ?para_id, err=?e, "Invalid candidate (PoV code)");

                    // If the PoV is invalid, the candidate certainly is.
                    return Ok(ValidationResult::Invalid(InvalidCandidate::PoVDecompressionFailure))
                },
            };
         */
        //metrics.observe_pov_size(raw_block_data.0.len(), false);

        let result = {
            // Retry is disabled to reduce the chance of nondeterministic blocks getting backed and
            // honest backers getting slashed.
            let prep_timeout = pvf_prep_timeout(&executor_params, PvfPrepKind::Prepare);
            let exec_timeout = pvf_exec_timeout(&executor_params, PvfExecKind::Backing);
            let pvf = PvfPrepData::from_code(
                raw_validation_code.to_vec(),
                executor_params,
                prep_timeout,
                PrepareJobKind::Compilation,
            );

            validation_backend
                .execute_candidate(pvf, exec_timeout, export_name, call_data, response_sender)
                .await
        };

        if let Err(ref error) = result {
            gum::info!(target: LOG_TARGET, ?para_id, ?error, "Failed to validate candidate");
        }

        match result {
            Err(ValidationError::Internal(e)) => {
                gum::warn!(
                    target: LOG_TARGET,
                    ?para_id,
                    ?e,
                    "An internal error occurred during validation, will abstain from voting",
                );
                Err(ValidationFailed(e.to_string()))
            }
            /*
            Err(ValidationError::Invalid(WasmInvalidCandidate::HardTimeout)) => {
                Ok(ValidationResult::Invalid(InvalidCandidate::Timeout))
            }
            Err(ValidationError::Invalid(WasmInvalidCandidate::WorkerReportedInvalid(e))) => Ok(
                ValidationResult::Invalid(InvalidCandidate::ExecutionError(e)),
            ),
            Err(ValidationError::PossiblyInvalid(PossiblyInvalidError::AmbiguousWorkerDeath)) => {
                Ok(ValidationResult::Invalid(InvalidCandidate::ExecutionError(
                    "ambiguous worker death".to_string(),
                )))
            }
            Err(ValidationError::PossiblyInvalid(PossiblyInvalidError::JobError(err))) => Ok(
                ValidationResult::Invalid(InvalidCandidate::ExecutionError(err)),
            ),
            Err(ValidationError::PossiblyInvalid(PossiblyInvalidError::AmbiguousJobDeath(err))) => {
                Ok(ValidationResult::Invalid(InvalidCandidate::ExecutionError(
                    format!("ambiguous job death: {err}"),
                )))
            }
             */
            Err(ValidationError::Preparation(e)) => {
                gum::warn!(
                    target: LOG_TARGET,
                    ?para_id,
                    ?e,
                    "Deterministic error occurred during preparation (should have been ruled out by pre-checking phase)",
                );
                Err(ValidationFailed(e.to_string()))
            }
            Err(e) => {
                // TODO
                Err(ValidationFailed(format!("{:?}", e)))
            }
            Ok(res) => Ok(res),
        }
    }

    #[async_trait]
    pub trait SecureExecutionBackend {
        /// Tries executing a PVF a single time (no retries).
        async fn execute_candidate(
            &mut self,
            pvf: PvfPrepData,
            exec_timeout: Duration,
            export_name: String,
            call_data: Vec<u8>,
            response_sender: std::sync::mpsc::Sender<PvfMsgResponse>,
        ) -> Result<Vec<u8>, ValidationError>;
    }

    #[async_trait]
    impl SecureExecutionBackend for ValidationHost<PvfMsgResponse> {
        /// Tries executing a PVF a single time (no retries).
        async fn execute_candidate(
            &mut self,
            pvf: PvfPrepData,
            exec_timeout: Duration,
            export_name: String,
            call_data: Vec<u8>,
            response_sender: std::sync::mpsc::Sender<PvfMsgResponse>,
        ) -> Result<Vec<u8>, ValidationError> {
            // TODO: LocalCallExecutor handles PoV, maybe use that here
            let priority = tanssi_worker::Priority::Normal;

            let (tx, rx) = oneshot::channel();
            if let Err(err) = self
                .execute_export(
                    pvf,
                    exec_timeout,
                    export_name,
                    call_data,
                    priority,
                    tx,
                    response_sender,
                )
                .await
            {
                return Err(InternalValidationError::HostCommunication(format!(
                    "cannot send pvf to the validation host, it might have shut down: {:?}",
                    err
                ))
                .into());
            }

            rx.await.map_err(|_| {
                ValidationError::from(InternalValidationError::HostCommunication(
                    "validation was cancelled".into(),
                ))
            })?
        }
    }

    /// To determine the amount of timeout time for the pvf execution.
    ///
    /// Precheck
    /// The time period after which the preparation worker is considered
    /// unresponsive and will be killed.
    ///
    /// Prepare
    ///The time period after which the preparation worker is considered
    /// unresponsive and will be killed.
    fn pvf_prep_timeout(executor_params: &ExecutorParams, kind: PvfPrepKind) -> Duration {
        if let Some(timeout) = executor_params.pvf_prep_timeout(kind) {
            return timeout;
        }
        match kind {
            PvfPrepKind::Precheck => DEFAULT_PRECHECK_PREPARATION_TIMEOUT,
            PvfPrepKind::Prepare => DEFAULT_LENIENT_PREPARATION_TIMEOUT,
        }
    }

    /// To determine the amount of timeout time for the pvf execution.
    ///
    /// Backing subsystem
    /// The amount of time to spend on execution during backing.
    ///
    /// Approval subsystem
    /// The amount of time to spend on execution during approval or disputes.
    /// This should be much longer than the backing execution timeout to ensure that in the
    /// absence of extremely large disparities between hardware, blocks that pass backing are
    /// considered executable by approval checkers or dispute participants.
    fn pvf_exec_timeout(executor_params: &ExecutorParams, kind: PvfExecKind) -> Duration {
        if let Some(timeout) = executor_params.pvf_exec_timeout(kind) {
            return timeout;
        }
        match kind {
            PvfExecKind::Backing => DEFAULT_BACKING_EXECUTION_TIMEOUT,
            PvfExecKind::Approval => DEFAULT_APPROVAL_EXECUTION_TIMEOUT,
        }
    }
}
