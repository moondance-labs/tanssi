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

use {
    codec::Encode,
    jsonrpsee::{
        core::RpcResult,
        proc_macros::rpc,
        types::{
            error::{INTERNAL_ERROR_CODE, INTERNAL_ERROR_MSG},
            ErrorObjectOwned,
        },
    },
    xcm::latest::prelude::*,
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

    #[method(name = "xcm_injectUpwardMessage")]
    async fn inject_upward_message(&self, message: Vec<u8>) -> RpcResult<()>;
}

#[derive(Clone)]
pub struct DevRpc {
    pub mock_para_inherent_channel: flume::Sender<Vec<u8>>,
    pub upward_message_channel: flume::Sender<Vec<u8>>,
}

#[jsonrpsee::core::async_trait]
impl DevApiServer for DevRpc {
    async fn enable_para_inherent_candidate(&self) -> RpcResult<()> {
        let mock_para_inherent_channel = self.mock_para_inherent_channel.clone();
        // Push the message to the shared channel where it will be queued up
        // to be injected in to an upcoming block.
        mock_para_inherent_channel
            .send_async(true.encode())
            .await
            .map_err(|err| internal_err(err.to_string()))?;

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

    async fn inject_upward_message(&self, msg: Vec<u8>) -> RpcResult<()> {
        let upward_message_channel = self.upward_message_channel.clone();
        // If no message is supplied, inject a default one.
        let msg = if msg.is_empty() {
            // Note: Sovereign account of the origin parachain must be funded before injecting the message.
            xcm::VersionedXcm::<()>::V4(Xcm(vec![
                WithdrawAsset((Here, 10000000000000u128).into()),
                BuyExecution {
                    fees: (Here, 10000000000000u128).into(),
                    weight_limit: Unlimited,
                },
                DepositAsset {
                    assets: AllCounted(1).into(),
                    beneficiary: Location::new(
                        0,
                        [AccountKey20 {
                            network: None,
                            key: hex_literal::hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac"),
                        }],
                    ),
                },
            ]))
            .encode()
        } else {
            msg
        };

        // Push the message to the shared channel where it will be queued up
        // to be injected in to an upcoming block.
        upward_message_channel
            .send_async(msg)
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
