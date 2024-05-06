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

//! Host interface to the execute worker.

use crate::{
    artifacts::ArtifactPathId,
    worker_interface::{
        clear_worker_dir_path, framed_recv, framed_send, spawn_with_program_path, IdleWorker,
        SpawnErr, WorkerDir, WorkerHandle, JOB_TIMEOUT_WALL_CLOCK_FACTOR,
    },
    LOG_TARGET,
};
use futures::FutureExt;
use futures_timer::Delay;
use os_pipe::{self, PipeReader, PipeWriter};
use parity_scale_codec::{Decode, Encode};
use polkadot_primitives::ExecutorParams;
use std::io::IoSliceMut;
use std::os::fd::FromRawFd;
use std::os::fd::{OwnedFd, RawFd};
use std::{path::Path, time::Duration};
use tanssi_worker_ancillary::{recv_vectored_with_ancillary_from, AncillaryData, SocketAncillary};
use tanssi_worker_common::execute::ExecuteRequest;
use tanssi_worker_common::executor_interface::{IpcExtRequest, IpcExtResponse};
use tanssi_worker_common::{
    error::InternalValidationError,
    execute::{Handshake, WorkerResponse},
    framed_recv_blocking, framed_send_blocking, worker_dir, SecurityStatus,
};
use tokio::{io, net::UnixStream};
use tracing_gum as gum;

/// Spawns a new worker with the given program path that acts as the worker and the spawn timeout.
///
/// Sends a handshake message to the worker as soon as it is spawned.
pub async fn spawn(
    program_path: &Path,
    cache_path: &Path,
    executor_params: ExecutorParams,
    spawn_timeout: Duration,
    node_version: Option<&str>,
    security_status: SecurityStatus,
) -> Result<(IdleWorker, WorkerHandle), SpawnErr> {
    let mut extra_args = vec!["tanssi-execute-worker"];
    if let Some(node_version) = node_version {
        extra_args.extend_from_slice(&["--node-impl-version", node_version]);
    }

    let (mut idle_worker, worker_handle) = spawn_with_program_path(
        "execute",
        program_path,
        cache_path,
        &extra_args,
        spawn_timeout,
        security_status,
    )
    .await?;
    send_execute_handshake(&mut idle_worker.stream, Handshake { executor_params })
        .await
        .map_err(|error| {
            let err = SpawnErr::Handshake {
                err: error.to_string(),
            };
            gum::warn!(
                target: LOG_TARGET,
                worker_pid = %idle_worker.pid,
                %err
            );
            err
        })?;
    Ok((idle_worker, worker_handle))
}

/// Outcome of PVF execution.
///
/// If the idle worker token is not returned, it means the worker must be terminated.
pub enum Outcome {
    /// PVF execution completed successfully and the result is returned. The worker is ready for
    /// another job.
    Ok {
        encoded_result: Vec<u8>,
        duration: Duration,
        idle_worker: IdleWorker,
    },
    /// The candidate validation failed. It may be for example because the wasm execution triggered
    /// a trap. Errors related to the preparation process are not expected to be encountered by the
    /// execution workers.
    InvalidCandidate {
        err: String,
        idle_worker: IdleWorker,
    },
    /// The execution time exceeded the hard limit. The worker is terminated.
    HardTimeout,
    /// An I/O error happened during communication with the worker. This may mean that the worker
    /// process already died. The token is not returned in any case.
    WorkerIntfErr,
    /// The job process has died. We must kill the worker just in case.
    ///
    /// We cannot treat this as an internal error because malicious code may have caused this.
    JobDied { err: String },
    /// An unexpected error occurred in the job process.
    ///
    /// Because malicious code can cause a job error, we must not treat it as an internal error.
    JobError { err: String },

    /// An internal error happened during the validation. Such an error is most likely related to
    /// some transient glitch.
    ///
    /// Should only ever be used for errors independent of the candidate and PVF. Therefore it may
    /// be a problem with the worker, so we terminate it.
    InternalError { err: InternalValidationError },
}

/// Given the idle token of a worker and parameters of work, communicates with the worker and
/// returns the outcome.
///
/// NOTE: Not returning the idle worker token in `Outcome` will trigger the child process being
/// killed, if it's still alive.
pub async fn start_work<T>(
    worker: IdleWorker,
    artifact: ArtifactPathId,
    execution_timeout: Duration,
    export_name: String,
    call_data: Vec<u8>,
    response_sender: std::sync::mpsc::Sender<T>,
) -> Outcome
where
    T: From<(
            IpcExtRequest,
            futures::channel::oneshot::Sender<IpcExtResponse>,
        )> + Send
        + 'static,
{
    let IdleWorker {
        mut stream,
        pid,
        worker_dir,
    } = worker;

    gum::debug!(
        target: LOG_TARGET,
        worker_pid = %pid,
        ?worker_dir,
        validation_code_hash = ?artifact.id.code_hash,
        "starting execute for {}",
        artifact.path.display(),
    );

    with_worker_dir_setup(worker_dir, pid, &artifact.path, |worker_dir| async move {
        // Need to use blocking IO to recv the pipes, shouldn't matter because the pipes should have
        // been sent a long time ago already.
        let raw_fds = {
            let mut blocking_stream = stream.into_std().unwrap();
            blocking_stream.set_nonblocking(false).unwrap();
            let mut some_stream = Some(blocking_stream);
            let raw_fds = recv_pipes_using_ancillary_magic(&mut some_stream).unwrap();
            blocking_stream = some_stream.unwrap();
            blocking_stream.set_nonblocking(true).unwrap();
            stream = UnixStream::from_std(blocking_stream).unwrap();
            raw_fds
        };

        gum::debug!(
            target: LOG_TARGET,
            worker_pid = %pid,
            ?worker_dir,
            validation_code_hash = ?artifact.id.code_hash,
            "received some nice raw fds: {}",
            raw_fds.len(),
        );
        assert_eq!(raw_fds.len(), 2);

        #[allow(unsafe_code)]
        let mut pipe_reader = unsafe { PipeReader::from_raw_fd(raw_fds[0]) };
        #[allow(unsafe_code)]
        let mut pipe_writer = unsafe { PipeWriter::from_raw_fd(raw_fds[1]) };

		if let Err(error) = send_exec_request(&mut stream, ExecuteRequest {
            export_name,
            params: call_data,
            execution_timeout,
        }).await {
			gum::warn!(
				target: LOG_TARGET,
				worker_pid = %pid,
				validation_code_hash = ?artifact.id.code_hash,
				?error,
				"failed to send an execute request",
			);
			return Outcome::WorkerIntfErr
		}

        gum::debug!(
            target: LOG_TARGET,
            worker_pid = %pid,
            ?worker_dir,
            validation_code_hash = ?artifact.id.code_hash,
            "execute request sent successfully",
        );

        let pipe_worker = move || {
            while let Ok(msg_enc) = framed_recv_blocking(&mut pipe_reader) {
                let msg = IpcExtRequest::decode(&mut msg_enc.as_slice()).unwrap();
                let (tx, rx) = futures::channel::oneshot::channel::<IpcExtResponse>();
                let _ = response_sender.send((msg, tx).into());
                // TODO: this unwrap can also panic
                let res = futures::executor::block_on(rx).unwrap();
                framed_send_blocking(&mut pipe_writer, &res.encode()).unwrap();
            }
        };

        let pipe_worker_thread = std::thread::spawn(pipe_worker);

		// We use a generous timeout here. This is in addition to the one in the child process, in
		// case the child stalls. We have a wall clock timeout here in the host, but a CPU timeout
		// in the child. We want to use CPU time because it varies less than wall clock time under
		// load, but the CPU resources of the child can only be measured from the parent after the
		// child process terminates.
		let timeout = execution_timeout * JOB_TIMEOUT_WALL_CLOCK_FACTOR;
		let response = futures::select! {
			response = recv_response(&mut stream).fuse() => {
				match response {
					Ok(response) =>
						handle_response(
							response,
							pid,
							execution_timeout,
						)
							.await,
					Err(error) => {
						gum::warn!(
							target: LOG_TARGET,
							worker_pid = %pid,
							validation_code_hash = ?artifact.id.code_hash,
							?error,
							"failed to recv an execute response",
						);

						return Outcome::WorkerIntfErr
					},
				}
			},
			_ = Delay::new(timeout).fuse() => {
				gum::warn!(
					target: LOG_TARGET,
					worker_pid = %pid,
					validation_code_hash = ?artifact.id.code_hash,
					"execution worker exceeded lenient timeout for execution, child worker likely stalled",
				);
				WorkerResponse::JobTimedOut
			},
		};

        let _ = pipe_worker_thread.join();

		match response {
			WorkerResponse::Ok { encoded_result, duration } => Outcome::Ok {
				encoded_result,
				duration,
				idle_worker: IdleWorker { stream, pid, worker_dir },
			},
			WorkerResponse::InvalidCandidate(err) => Outcome::InvalidCandidate {
				err,
				idle_worker: IdleWorker { stream, pid, worker_dir },
			},
			WorkerResponse::JobTimedOut => Outcome::HardTimeout,
			WorkerResponse::JobDied { err, job_pid: _ } => Outcome::JobDied { err },
			WorkerResponse::JobError(err) => Outcome::JobError { err },

			WorkerResponse::InternalError(err) => Outcome::InternalError { err },
		}
	})
	.await
}

/// Handles the case where we successfully received response bytes on the host from the child.
///
/// Here we know the artifact exists, but is still located in a temporary file which will be cleared
/// by [`with_worker_dir_setup`].
async fn handle_response(
    response: WorkerResponse,
    worker_pid: u32,
    execution_timeout: Duration,
) -> WorkerResponse {
    if let WorkerResponse::Ok { duration, .. } = response {
        if duration > execution_timeout {
            // The job didn't complete within the timeout.
            gum::warn!(
                target: LOG_TARGET,
                worker_pid,
                "execute job took {}ms cpu time, exceeded execution timeout {}ms.",
                duration.as_millis(),
                execution_timeout.as_millis(),
            );

            // Return a timeout error.
            return WorkerResponse::JobTimedOut;
        }
    }

    response
}

/// Create a temporary file for an artifact in the worker cache, execute the given future/closure
/// passing the file path in, and clean up the worker cache.
///
/// Failure to clean up the worker cache results in an error - leaving any files here could be a
/// security issue, and we should shut down the worker. This should be very rare.
async fn with_worker_dir_setup<F, Fut>(
    worker_dir: WorkerDir,
    pid: u32,
    artifact_path: &Path,
    f: F,
) -> Outcome
where
    Fut: futures::Future<Output = Outcome>,
    F: FnOnce(WorkerDir) -> Fut,
{
    // Cheaply create a hard link to the artifact. The artifact is always at a known location in the
    // worker cache, and the child can't access any other artifacts or gain any information from the
    // original filename.
    let link_path = worker_dir::execute_artifact(worker_dir.path());
    if let Err(err) = tokio::fs::hard_link(artifact_path, link_path).await {
        gum::warn!(
            target: LOG_TARGET,
            worker_pid = %pid,
            ?worker_dir,
            "failed to clear worker cache after the job: {:?}",
            err,
        );
        return Outcome::InternalError {
            err: InternalValidationError::CouldNotCreateLink(format!("{:?}", err)),
        };
    }

    let worker_dir_path = worker_dir.path().to_owned();
    let outcome = f(worker_dir).await;

    // Try to clear the worker dir.
    if let Err(err) = clear_worker_dir_path(&worker_dir_path) {
        gum::warn!(
            target: LOG_TARGET,
            worker_pid = %pid,
            ?worker_dir_path,
            "failed to clear worker cache after the job: {:?}",
            err,
        );
        return Outcome::InternalError {
            err: InternalValidationError::CouldNotClearWorkerDir {
                err: format!("{:?}", err),
                path: worker_dir_path.to_str().map(String::from),
            },
        };
    }

    outcome
}

/// Sends a handshake with information specific to the execute worker.
async fn send_execute_handshake(stream: &mut UnixStream, handshake: Handshake) -> io::Result<()> {
    framed_send(stream, &handshake.encode()).await
}

async fn send_exec_request(stream: &mut UnixStream, req: ExecuteRequest) -> io::Result<()> {
    framed_send(stream, &req.encode()).await
}

async fn recv_response(stream: &mut UnixStream) -> io::Result<WorkerResponse> {
    let response_bytes = framed_recv(stream).await?;
    WorkerResponse::decode(&mut response_bytes.as_slice()).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("execute pvf recv_response: decode error: {:?}", e),
        )
    })
}

pub fn recv_pipes_using_ancillary_magic(
    stream: &mut Option<std::os::unix::net::UnixStream>,
) -> std::io::Result<Vec<RawFd>> {
    // We are not transmitting any additional data, only 1 magic byte
    let mut buf = [0; 1];
    let mut bufs = vec![IoSliceMut::new(&mut buf)];
    let owned_fd_stream = OwnedFd::from(stream.take().unwrap());
    let socket_stream = tanssi_worker_ancillary::Socket(owned_fd_stream);

    let mut ancillary_buffer = [0; 128];
    let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);

    let (_count, _, _) =
        recv_vectored_with_ancillary_from(&socket_stream, &mut bufs, &mut ancillary)?;

    *stream = Some(std::os::unix::net::UnixStream::from(socket_stream.0));

    assert_eq!(bufs[0][0], 0xAA);

    let mut fds = vec![];

    for ancillary_result in ancillary.messages() {
        if let AncillaryData::ScmRights(scm_rights) = ancillary_result.unwrap() {
            for fd in scm_rights {
                fds.push(fd);
            }
        }
    }

    Ok(fds)
}
