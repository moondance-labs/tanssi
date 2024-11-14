// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

//! Development Polkadot service. Adapted from `polkadot_service` crate
//! and removed un-necessary components which are not required in dev node.

use codec::{Decode, Encode};
use jsonrpsee::{
    core::RpcResult,
    proc_macros::rpc,
    types::{
        error::{INTERNAL_ERROR_CODE, INTERNAL_ERROR_MSG},
        ErrorObjectOwned,
    },
};

/// This RPC interface is used to provide methods in dev mode only
#[rpc(server)]
#[jsonrpsee::core::async_trait]
pub trait DevApi {
    /// Indicate the mock parachain candidate insertion to be active
    #[method(name = "mock_enableParaInherentCandidate")]
    async fn enable_para_inherent_candidate(&self) -> RpcResult<()>;

    /// Indicate the mock parachain candidate insertion to be disabled
    #[method(name = "mock_disableParaInherentCandidate")]
    async fn disable_para_inherent_candidate(&self) -> RpcResult<()>;
}

pub struct DevRpc {
    pub mock_para_inherent_channel: flume::Sender<Vec<u8>>,
}

#[jsonrpsee::core::async_trait]
impl DevApiServer for DevRpc {
    async fn enable_para_inherent_candidate(&self) -> RpcResult<()> {
        log::info!("entering here");
        let mock_para_inherent_channel = self.mock_para_inherent_channel.clone();
        // Push the message to the shared channel where it will be queued up
        // to be injected in to an upcoming block.
        mock_para_inherent_channel
            .send_async(true.encode())
            .await
            .map_err(|err| internal_err(err.to_string()))?;

        log::info!("SENEDING ENABLE");
        Ok(())
    }

    async fn disable_para_inherent_candidate(&self) -> RpcResult<()> {
        let mock_para_inherent_channel = self.mock_para_inherent_channel.clone();
        // Push the message to the shared channel where it will be queued up
        // to be injected in to an upcoming block.
        mock_para_inherent_channel
            .send_async(false.encode())
            .await
            .map_err(|err| internal_err(err.to_string()))?;

        Ok(())
    }
}

// This bit cribbed from frontier.
pub fn internal_err<T: ToString>(message: T) -> ErrorObjectOwned {
    ErrorObjectOwned::owned(
        INTERNAL_ERROR_CODE,
        INTERNAL_ERROR_MSG,
        Some(message.to_string()),
    )
}
