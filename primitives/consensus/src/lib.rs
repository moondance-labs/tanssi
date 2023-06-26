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

#![cfg_attr(not(feature = "std"), no_std)]
use {cumulus_primitives_core::ParaId, parity_scale_codec::Codec, sp_std::vec::Vec};
use {frame_support::traits::Get, sp_runtime::traits::Zero, sp_std::marker::PhantomData};

sp_api::decl_runtime_apis! {
    /// API necessary for block authorship with Tanssi.
    pub trait TanssiAuthorityAssignmentApi<AuthorityId: Codec> {
        /// Returns the authorities for a given paraId
        fn para_id_authorities(para_id: ParaId) -> Option<Vec<AuthorityId>>;

        /// Returns the paraId for which an authority is assigned (if any)
        fn check_para_id_assignment(authority: AuthorityId) -> Option<ParaId>;
    }
}

pub struct OnTimestampSet<SlotBeacon, SlotDuration>(PhantomData<(SlotBeacon, SlotDuration)>);
impl<SlotBeacon, SlotDuration> frame_support::traits::OnTimestampSet<u64>
    for OnTimestampSet<SlotBeacon, SlotDuration>
where
    SlotBeacon: nimbus_primitives::SlotBeacon,
    SlotDuration: Get<u64>,
{
    fn on_timestamp_set(moment: u64) {
        assert!(
            !SlotDuration::get().is_zero(),
            "Slot duration cannot be zero."
        );

        let timestamp_slot = moment / SlotDuration::get();

        assert!(
            SlotBeacon::slot() as u64 == timestamp_slot,
            "Timestamp slot must match SlotBeacon slot"
        );
    }
}

pub struct ContainerNimbusLookUp;
impl nimbus_primitives::AccountLookup<nimbus_primitives::NimbusId> for ContainerNimbusLookUp {
    fn lookup_account(author: &nimbus_primitives::NimbusId) -> Option<nimbus_primitives::NimbusId> {
        Some(author.clone())
    }
}

pub struct AuraDigestSlotBeacon<ContainerRuntime>(PhantomData<ContainerRuntime>);
impl<ContainerRuntime> nimbus_primitives::SlotBeacon for AuraDigestSlotBeacon<ContainerRuntime>
where
    ContainerRuntime: frame_system::Config,
{
    fn slot() -> u32 {
        use sp_consensus_aura::{Slot, AURA_ENGINE_ID};

        let digests = frame_system::Pallet::<ContainerRuntime>::digest();

        let slot = digests
            .convert_first(|item| item.pre_runtime_try_to::<Slot>(&AURA_ENGINE_ID))
            .expect("slot digest should exist");

        let slot: u64 = slot.into();
        slot as u32
    }
}