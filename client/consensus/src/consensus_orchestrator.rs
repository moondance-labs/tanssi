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

//! The Tanssi AuRa consensus algorithm for orchestrator chain and container chain collators.    
//!
//! It calculates based on the orchestrator-state dictated authorities
//! It is identical to AuraWorker and AuraConsensus, except for the fact that we re-implement
//! the ParachainConsensus trait to access the orchestrator-dicated authorities, and further
//! it implements the TanssiWorker to TanssiOnSlot trait. This trait is
use {
    crate::{AuthorityId, Pair, Slot},
    sp_runtime::traits::Block as BlockT,
};

#[async_trait::async_trait]
pub trait RetrieveAuthoritiesFromOrchestrator<Block: BlockT, ExtraArgs, A>: Send + Sync {
    /// Create the inherent data providers at the given `parent` block using the given `extra_args`.
    async fn retrieve_authorities_from_orchestrator(
        &self,
        parent: Block::Hash,
        extra_args: ExtraArgs,
    ) -> Result<A, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait::async_trait]
impl<F, Block, ExtraArgs, Fut, A> RetrieveAuthoritiesFromOrchestrator<Block, ExtraArgs, A> for F
where
    Block: BlockT,
    F: Fn(Block::Hash, ExtraArgs) -> Fut + Sync + Send,
    Fut: std::future::Future<Output = Result<A, Box<dyn std::error::Error + Send + Sync>>>
        + Send
        + 'static,
    ExtraArgs: Send + 'static,
{
    async fn retrieve_authorities_from_orchestrator(
        &self,
        parent: Block::Hash,
        extra_args: ExtraArgs,
    ) -> Result<A, Box<dyn std::error::Error + Send + Sync>> {
        (*self)(parent, extra_args).await
    }
}

pub struct OrchestratorAuraWorkerAuxData<P>
where
    P: Pair + Send + Sync + 'static,
{
    pub authorities: Vec<AuthorityId<P>>,
    pub min_slot_freq: Option<Slot>,
}
