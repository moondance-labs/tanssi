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

//! RPC client for Author Noting pallet

pub use pallet_stream_payment_runtime_api::StreamPaymentApi as StreamPaymentRuntimeApi;
use {
    core::marker::PhantomData,
    jsonrpsee::{
        core::{async_trait, RpcResult},
        proc_macros::rpc,
    },
    pallet_stream_payment_runtime_api::{StreamPaymentApiError, StreamPaymentApiStatus},
    sp_api::ProvideRuntimeApi,
    sp_runtime::traits::Block as BlockT,
    std::sync::Arc,
};

/// Top-level error type for the RPC handler.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to fetch API
    #[error("Failed to fetch API: {0}")]
    ApiError(sp_api::ApiError),

    /// Failed to fetch the current best header.
    #[error("Failed to fetch stream payment status: {0}")]
    StreamPaymentApiError(StreamPaymentApiError),
}

#[rpc(client, server)]
pub trait StreamPaymentApi<Hash, StreamId, Instant, Balance> {
    #[method(name = "tanssi_streamPaymentStatus")]
    async fn stream_payment_status(
        &self,
        block: Hash,
        stream_id: StreamId,
        now: Option<Instant>,
    ) -> RpcResult<StreamPaymentApiStatus<Balance>>;
}

pub struct StreamPayment<Client, Block> {
    client: Arc<Client>,
    _phantom: PhantomData<Block>,
}

impl<Client, Block> StreamPayment<Client, Block> {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<Block, Hash, Client, StreamId, Instant, Balance>
    StreamPaymentApiServer<Hash, StreamId, Instant, Balance> for StreamPayment<Client, Block>
where
    Hash: Send + 'static,
    Block: BlockT<Hash = Hash>,
    Client: ProvideRuntimeApi<Block> + Sync + Send + 'static,
    Client::Api: StreamPaymentRuntimeApi<Block, StreamId, Instant, Balance>,
    StreamId: parity_scale_codec::Codec + Send + 'static,
    Instant: parity_scale_codec::Codec + Send + 'static,
    Balance: parity_scale_codec::Codec + Send + 'static,
{
    async fn stream_payment_status(
        &self,
        block: Hash,
        stream_id: StreamId,
        now: Option<Instant>,
    ) -> RpcResult<StreamPaymentApiStatus<Balance>> {
        let status = self
            .client
            .runtime_api()
            .stream_payment_status(block, stream_id, now)
            .map_err(|e| internal_err(Error::ApiError(e)))?
            .map_err(|e| internal_err(Error::StreamPaymentApiError(e)))?;

        Ok(status)
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
