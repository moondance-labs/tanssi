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

pub use tp_container_chain_genesis_data::ContainerChainGenesisData;
use {frame_support::traits::Get, scale_info::prelude::vec::Vec};

sp_api::decl_runtime_apis! {
    pub trait RegistrarApi<ParaId, MaxLengthTokenSymbol> where
        ParaId: parity_scale_codec::Codec,
        MaxLengthTokenSymbol: Get<u32>,
    {
        /// Return the registered para ids
        fn registered_paras() -> Vec<ParaId>;

        /// Fetch genesis data for this para id
        fn genesis_data(para_id: ParaId) -> Option<ContainerChainGenesisData<MaxLengthTokenSymbol>>;

        /// Fetch boot_nodes for this para id
        fn boot_nodes(para_id: ParaId) -> Vec<Vec<u8>>;
    }
}

sp_api::decl_runtime_apis! {
    pub trait OnDemandBlockProductionApi<ParaId, Slot> where
        ParaId: parity_scale_codec::Codec,
        Slot: parity_scale_codec::Codec,
    {
        /// Return the minimum number of slots that must pass between to blocks before parathread collators can propose
        /// the next block.
        ///
        /// # Returns
        ///
        /// * `Some(min)`, where the condition for the slot to be valid is `(slot - parent_slot) >= min`.
        /// * `None` if the `para_id` is not a parathread.
        fn min_slot_freq(para_id: ParaId) -> Option<Slot>;
    }
}
