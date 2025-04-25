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
use {
    cumulus_primitives_core::ParaId,
    jsonrpsee::{core::RpcResult, proc_macros::rpc},
};

#[rpc(server)]
#[jsonrpsee::core::async_trait]
pub trait ManualContainerChainsExclusionApi {
    /// Preventing container chains from producing blocks
    #[method(name = "mock_excludeContainerChains")]
    async fn exclude_container_chains(&self, para_ids: Vec<ParaId>) -> RpcResult<()>;
}

pub struct ManualContainerChainsExclusion {
    pub container_chain_exclusion_message_channel: flume::Sender<Vec<ParaId>>,
}

#[jsonrpsee::core::async_trait]
impl ManualContainerChainsExclusionApiServer for ManualContainerChainsExclusion {
    async fn exclude_container_chains(&self, para_ids: Vec<ParaId>) -> RpcResult<()> {
        let container_chain_exclusion_message_channel =
            self.container_chain_exclusion_message_channel.clone();

        // Push the message to the shared channel where it will be queued up
        // to be injected in to an upcoming block.
        container_chain_exclusion_message_channel
            .send_async(para_ids)
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
