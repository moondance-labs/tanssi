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

#[macro_export]
macro_rules! impl_consensus_pallets {
	{} => {
        use nimbus_primitives::NimbusId;
        // NOTE: Currently it is not possible to change the slot duration after the chain has started.
        // Attempting to do so will brick block production.
        pub const SLOT_DURATION: u64 = 12000;

        impl pallet_cc_authorities_noting::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type SelfParaId = parachain_info::Pallet<Runtime>;
            type RelayChainStateProvider = cumulus_pallet_parachain_system::RelaychainDataProvider<Self>;
            type AuthorityId = NimbusId;
            type WeightInfo = pallet_cc_authorities_noting::weights::SubstrateWeight<Runtime>;
        }

        impl pallet_timestamp::Config for Runtime {
            /// A timestamp: milliseconds since the unix epoch.
            type Moment = u64;
            type OnTimestampSet = tp_consensus::OnTimestampSet<
                <Self as pallet_author_inherent::Config>::SlotBeacon,
                ConstU64<{ SLOT_DURATION }>,
            >;
            type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
            type WeightInfo = pallet_timestamp::weights::SubstrateWeight<Runtime>;
        }

        impl pallet_author_inherent::Config for Runtime {
            type AuthorId = NimbusId;
            type AccountLookup = tp_consensus::NimbusLookUp;
            type CanAuthor = pallet_cc_authorities_noting::CanAuthor<Runtime>;
            type SlotBeacon = tp_consensus::AuraDigestSlotBeacon<Runtime>;
            type WeightInfo = pallet_author_inherent::weights::SubstrateWeight<Runtime>;
        }
	}
}