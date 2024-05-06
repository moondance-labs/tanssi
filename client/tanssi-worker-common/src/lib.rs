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

//! Contains functionality related to PVFs that is shared by the PVF host and the PVF workers.

// TODO: maybe this crate doesn't need to exist, we can re-use 90% of polkadot-node-core-pvf-common

#![allow(unsafe_code)]

pub mod error;
pub mod execute;
pub mod executor_interface;
pub mod prepare;
pub mod pvf;
pub mod worker;
pub mod worker_dir;

pub use cpu_time::ProcessTime;

// Used by `decl_worker_main!`.
pub use sp_tracing;

const LOG_TARGET: &str = "tanssi-worker-common";

use parity_scale_codec::{Decode, Encode};
use std::{
    io::{self, Read, Write},
    mem,
};

#[cfg(feature = "test-utils")]
pub mod tests {
    use std::time::Duration;

    pub const TEST_EXECUTION_TIMEOUT: Duration = Duration::from_secs(3);
    pub const TEST_PREPARATION_TIMEOUT: Duration = Duration::from_secs(30);
}

/// Status of security features on the current system.
#[derive(Debug, Clone, Default, PartialEq, Eq, Encode, Decode)]
pub struct SecurityStatus {
    /// Whether Secure Validator Mode is enabled. This mode enforces that all required security
    /// features are present. All features are enabled on a best-effort basis regardless.
    pub secure_validator_mode: bool,
    /// Whether the landlock features we use are fully available on this system.
    pub can_enable_landlock: bool,
    /// Whether the seccomp features we use are fully available on this system.
    pub can_enable_seccomp: bool,
    /// Whether we are able to unshare the user namespace and change the filesystem root.
    pub can_unshare_user_namespace_and_change_root: bool,
}

/// A handshake with information for the worker.
#[derive(Debug, Encode, Decode)]
pub struct WorkerHandshake {
    pub security_status: SecurityStatus,
}

// This is for debugging only
/*
static FRAMED_TRANSCRIPT: Mutex<FramedTranscript> = Mutex::new(FramedTranscript::new());
pub fn enable_framed_transcript(enabled: bool, socket_ptr_recv: usize) {
    let mut f = FRAMED_TRANSCRIPT.try_lock().expect("framed_recv deadlock");
    f.enabled = enabled;
    f.socket_ptr_recv = socket_ptr_recv;
}

struct FramedTranscript {
    enabled: bool,
    socket_ptr_recv: usize,
    transcript: Vec<SendRecvFrame>,
}

impl FramedTranscript {
    const fn new() -> Self {
        Self {
            enabled: false,
            socket_ptr_recv: 0,
            transcript: vec![],
        }
    }

    fn log_recv_len(&mut self, len: usize) {
        if !self.enabled {
            return;
        }
        self.transcript.push(SendRecvFrame::RecvLen(len));
    }

    fn log_recv_data(&mut self, data: &[u8]) {
        if !self.enabled {
            return;
        }
        self.transcript.push(SendRecvFrame::RecvData(data.to_vec()));
    }

    fn log_send_len(&mut self, len: usize) {
        if !self.enabled {
            return;
        }
        self.transcript.push(SendRecvFrame::SendLen(len));
    }

    fn log_send_data(&mut self, data: &[u8]) {
        if !self.enabled {
            return;
        }
        self.transcript.push(SendRecvFrame::SendData(data.to_vec()));
    }

    fn debug_format(&self) -> String {
        format!("{:?}", self.transcript)
    }
}

#[derive(Debug)]
enum SendRecvFrame {
    RecvLen(usize),
    RecvData(Vec<u8>),
    SendLen(usize),
    SendData(Vec<u8>),
}
*/

/// Write some data prefixed by its length into `w`. Sync version of `framed_send` to avoid
/// dependency on tokio.
pub fn framed_send_blocking(w: &mut (impl Write + Unpin), buf: &[u8]) -> io::Result<()> {
    let len_buf = buf.len().to_le_bytes();
    w.write_all(&len_buf)?;
    /*
    FRAMED_TRANSCRIPT
        .try_lock()
        .expect("framed_send deadlock")
        .log_send_len(buf.len());
     */
    w.write_all(buf)?;
    /*
    FRAMED_TRANSCRIPT
        .try_lock()
        .expect("framed_send deadlock")
        .log_send_data(&buf);
     */
    Ok(())
}

/// Read some data prefixed by its length from `r`. Sync version of `framed_recv` to avoid
/// dependency on tokio.
pub fn framed_recv_blocking(r: &mut (impl Read + Unpin)) -> Result<Vec<u8>, String> {
    let mut len_buf = [0u8; mem::size_of::<usize>()];
    r.read_exact(&mut len_buf).map_err(|e| {
        format!(
            "Failed read_exact for frame length: {:?}\nLEFTOVER BUF: {:?}TRANSCRIPT: {}",
            e,
            len_buf,
            /*
            FRAMED_TRANSCRIPT
                .try_lock()
                .expect("framed_recv deadlock on error")
                .debug_format()
             */
            ""
        )
    })?;
    let len = usize::from_le_bytes(len_buf);
    /*
    FRAMED_TRANSCRIPT
        .try_lock()
        .expect("framed_recv deadlock")
        .log_recv_len(len);
     */
    let mut buf = vec![0; len];
    r.read_exact(&mut buf).map_err(|e| {
        format!(
            "Failed read_exact of {} bytes: {:?}\nLEFTOVER BUF: {:?}\nTRANSCRIPT: {}",
            len,
            e,
            buf,
            /*
            FRAMED_TRANSCRIPT
                .try_lock()
                .expect("framed_recv deadlock on error")
                .debug_format()
             */
            ""
        )
    })?;
    /*
    FRAMED_TRANSCRIPT
        .try_lock()
        .expect("framed_recv deadlock")
        .log_recv_data(&buf);
     */
    Ok(buf)
}

// Desperate idea
pub fn framed_recv_blocking_one_byte_at_a_time(r: &mut (impl Read + Unpin)) -> io::Result<Vec<u8>> {
    let mut len_buf = [0u8; mem::size_of::<usize>()];
    r.read_exact(&mut len_buf)?;
    let len = usize::from_le_bytes(len_buf);

    let mut buf = vec![0; len];
    for i in 0..len {
        r.read(&mut buf[i..i + 1])?;
    }
    Ok(buf)
}
