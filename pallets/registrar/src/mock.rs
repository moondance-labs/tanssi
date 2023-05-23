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

use tp_container_chain_genesis_data::ContainerChainGenesisData;

use {
    crate::{self as pallet_registrar},
    frame_support::traits::{ConstU16, ConstU64},
    frame_system as system,
    sp_core::{ConstU32, H256},
    sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    },
    tp_traits::ParaId,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        ParaRegistrar: pallet_registrar,
    }
);

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub struct CurrentSessionIndexGetter;

impl tp_traits::GetSessionIndex<u32> for CurrentSessionIndexGetter {
    /// Returns current session index.
    fn session_index() -> u32 {
        // For tests, let 1 session be 5 blocks
        (System::block_number() / 5) as u32
    }
}

impl pallet_registrar::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RegistrarOrigin = frame_system::EnsureRoot<u64>;
    type MaxLengthParaIds = ConstU32<1000>;
    type MaxGenesisDataSize = ConstU32<5_000_000>;
    type SessionDelay = ConstU32<2>;
    type SessionIndex = u32;
    type CurrentSessionIndex = CurrentSessionIndexGetter;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext_with_genesis(
    para_ids: Vec<(ParaId, ContainerChainGenesisData)>,
) -> sp_io::TestExternalities {
    GenesisConfig {
        system: Default::default(),
        para_registrar: pallet_registrar::GenesisConfig { para_ids },
    }
    .build_storage()
    .unwrap()
    .into()
}

pub fn empty_genesis_data() -> ContainerChainGenesisData {
    ContainerChainGenesisData {
        storage: Default::default(),
        name: Default::default(),
        id: Default::default(),
        fork_id: Default::default(),
        extensions: Default::default(),
        properties: Default::default(),
    }
}
