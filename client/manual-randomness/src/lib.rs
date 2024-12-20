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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_core::H256;

#[rpc(server)]
#[jsonrpsee::core::async_trait]
pub trait ManualRandomnessApi {
    /// Inject randomness
    #[method(name = "mock_activateRandomness")]
    async fn activate_randomness(&self, seed: Option<H256>) -> RpcResult<()>;
    #[method(name = "mock_deactivateRandomness")]
    async fn deactivate_randomness(&self) -> RpcResult<()>;
}

pub struct ManualRandomness {
    pub randomness_message_channel: flume::Sender<(bool, Option<[u8; 32]>)>,
}

#[jsonrpsee::core::async_trait]
impl ManualRandomnessApiServer for ManualRandomness {
    async fn activate_randomness(&self, seed: Option<H256>) -> RpcResult<()> {
        let randomness_message_channel = self.randomness_message_channel.clone();

        // Push the message to the shared channel where it will be queued up
        // to be injected in to an upcoming block.
        randomness_message_channel
            .send_async((true, seed.map(|x| x.into())))
            .await
            .map_err(|err| internal_err(err.to_string()))?;

        Ok(())
    }

    async fn deactivate_randomness(&self) -> RpcResult<()> {
        let randomness_message_channel = self.randomness_message_channel.clone();

        // Push the message to the shared channel where it will be queued up
        // to be injected in to an upcoming block.
        randomness_message_channel
            .send_async((false, None))
            .await
            .map_err(|err| internal_err(err.to_string()))?;

        Ok(())
    }
}

// This bit cribbed from frontier.
pub fn internal_err<T: AsRef<str>>(message: T) -> jsonrpsee::types::ErrorObjectOwned {
    jsonrpsee::types::error::ErrorObject::borrowed(
        jsonrpsee::types::error::INTERNAL_ERROR_CODE,
        message.as_ref(),
        None,
    )
    .into_owned()
}
