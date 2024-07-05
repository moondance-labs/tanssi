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

//! Runtime API for Registrar pallet

#![cfg_attr(not(feature = "std"), no_std)]

pub use dp_container_chain_genesis_data::ContainerChainGenesisData;
use {scale_info::prelude::vec::Vec, tp_traits::SlotFrequency};

sp_api::decl_runtime_apis! {
    pub trait RegistrarApi<ParaId> where
        ParaId: parity_scale_codec::Codec,
    {
        /// Return the registered para ids
        fn registered_paras() -> Vec<ParaId>;

        /// Fetch genesis data for this para id
        fn genesis_data(para_id: ParaId) -> Option<ContainerChainGenesisData>;

        /// Fetch boot_nodes for this para id
        fn boot_nodes(para_id: ParaId) -> Vec<Vec<u8>>;
    }
}

sp_api::decl_runtime_apis! {
    pub trait OnDemandBlockProductionApi<ParaId, Slot> where
        ParaId: parity_scale_codec::Codec,
        Slot: parity_scale_codec::Codec,
    {
        /// Returns slot frequency for particular para thread. Slot frequency specifies amount of slot
        /// need to be passed between two parathread blocks. It is expressed as `(min, max)` pair where `min`
        /// indicates amount of slot must pass before we produce another block and `max` indicates amount of
        /// blocks before this parathread must produce the block.
        ///
        /// Simply put, parathread must produce a block after `min`  but before `(min+max)` slots.
        ///
        /// # Returns
        ///
        /// * `Some(slot_frequency)`.
        /// * `None` if the `para_id` is not a parathread.
        fn parathread_slot_frequency(para_id: ParaId) -> Option<SlotFrequency>;
    }
}
