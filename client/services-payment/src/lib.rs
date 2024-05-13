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

//! RPC client for Services Payment pallet

pub use pallet_services_payment_runtime_api::ServicesPaymentApi as ServicesPaymentRuntimeApi;
use {
    core::marker::PhantomData,
    jsonrpsee::{
        core::{async_trait, RpcResult},
        proc_macros::rpc,
    },
    sc_client_api::UsageProvider,
    sp_api::ProvideRuntimeApi,
    sp_runtime::traits::Block as BlockT,
    std::sync::Arc,
};

#[rpc(server)]
pub trait ServicesPaymentApi<Balance, ParaId> {
    #[method(name = "tanssi_servicesPaymentBlockCost")]
    async fn block_cost(&self, para_id: ParaId) -> RpcResult<Balance>;

    #[method(name = "tanssi_servicesPaymentCollatorAssignmentCost")]
    async fn collator_assignment_cost(&self, para_id: ParaId) -> RpcResult<Balance>;
}

pub struct ServicesPayment<Client, Block> {
    client: Arc<Client>,
    _phantom: PhantomData<Block>,
}

impl<Client, Block> ServicesPayment<Client, Block> {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<Client, Hash, Block, Balance, ParaId> ServicesPaymentApiServer<Balance, ParaId>
    for ServicesPayment<Client, Block>
where
    Hash: Send + 'static,
    Block: BlockT<Hash = Hash>,
    Client: ProvideRuntimeApi<Block> + Sync + Send + UsageProvider<Block> + 'static,
    Client::Api: ServicesPaymentRuntimeApi<Block, Balance, ParaId>,
    Balance: parity_scale_codec::Codec + Send + 'static,
    ParaId: parity_scale_codec::Codec + Send + 'static,
{
    async fn block_cost(&self, para_id: ParaId) -> RpcResult<Balance> {
        let cost = self
            .client
            .runtime_api()
            .block_cost(self.client.usage_info().chain.best_hash, para_id)
            .map_err(|e| internal_err(e))?;
        Ok(cost)
    }

    async fn collator_assignment_cost(&self, para_id: ParaId) -> RpcResult<Balance> {
        let cost = self
            .client
            .runtime_api()
            .collator_assignment_cost(self.client.usage_info().chain.best_hash, para_id)
            .map_err(|e| internal_err(e))?;
        Ok(cost)
    }
}

pub fn internal_err<T: ToString>(error: T) -> jsonrpsee::core::Error {
    jsonrpsee::core::Error::Call(jsonrpsee::types::error::CallError::Custom(
        jsonrpsee::types::error::ErrorObject::borrowed(
            jsonrpsee::types::error::INTERNAL_ERROR_CODE,
            &error.to_string(),
            None,
        )
        .into_owned(),
    ))
}
