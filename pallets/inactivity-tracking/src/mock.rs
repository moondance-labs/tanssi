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
use {
    crate as pallet_inactivity_tracking,
    frame_support::traits::{ConstU32, ConstU64, Everything, OnFinalize, OnInitialize},
    sp_core::H256,
    sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    },
    sp_std::convert::Into,
    tp_traits::{ForSession, ParaId},
};

type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u64;

pub const COLLATOR_1: AccountId = 1;
pub const COLLATOR_2: AccountId = 2;
pub const CONTAINER_CHAIN_ID_1: ParaId = ParaId::new(3000);
pub const CONTAINER_CHAIN_ID_2: ParaId = ParaId::new(3001);
pub const SESSION_BLOCK_LENGTH: u64 = 5;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        InactivityTracking: pallet_inactivity_tracking,
    }
);

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Block = Block;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type RuntimeTask = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
    type ExtensionsWeightInfo = ();
}

pub struct CurrentSessionIndexGetter;

impl tp_traits::GetSessionIndex<u32> for CurrentSessionIndexGetter {
    /// Returns current session index.
    fn session_index() -> u32 {
        // For tests, let 1 session be 5 blocks
        (System::block_number() / SESSION_BLOCK_LENGTH) as u32
    }
}

pub struct MockContainerChainsInfoFetcher;
impl tp_traits::GetContainerChainsWithCollators<AccountId> for MockContainerChainsInfoFetcher {
    fn container_chains_with_collators(_for_session: ForSession) -> Vec<(ParaId, Vec<AccountId>)> {
        vec![
            (CONTAINER_CHAIN_ID_1, vec![COLLATOR_1, COLLATOR_2]),
            (CONTAINER_CHAIN_ID_2, vec![]),
        ]
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn set_container_chains_with_collators(
        for_session: ForSession,
        container_chains: &[(ParaId, Vec<AccountId>)],
    ) {
    }
}

impl pallet_inactivity_tracking::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type CollatorId = AccountId;
    type MaxInactiveSessions = ConstU32<2>;
    type MaxCollatorsPerSession = ConstU32<5>;
    type MaxContainerChains = ConstU32<3>;
    type CurrentSessionIndex = CurrentSessionIndexGetter;
    type GetSelfChainBlockAuthor = ();
    type ContainerChainsFetcher = MockContainerChainsInfoFetcher;
    type WeightInfo = ();
}

#[derive(Default)]
pub(crate) struct ExtBuilder;

impl ExtBuilder {
    pub(crate) fn build(self) -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .expect("Frame system builds valid default genesis config");

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

/// Rolls forward one block. Returns the new block number.
#[allow(dead_code)]
pub(crate) fn roll_one_block() -> u64 {
    InactivityTracking::on_finalize(System::block_number());
    System::on_finalize(System::block_number());
    System::set_block_number(System::block_number() + 1);
    System::on_initialize(System::block_number());
    InactivityTracking::on_initialize(System::block_number());
    System::block_number()
}

/// Rolls to the desired block. Returns the number of blocks played.
#[allow(dead_code)]
pub(crate) fn roll_to(n: u64) -> u64 {
    let mut num_blocks = 0;
    let mut block = System::block_number();
    while block < n {
        block = roll_one_block();
        num_blocks += 1;
    }
    num_blocks
}

#[allow(dead_code)]
pub(crate) fn last_event() -> RuntimeEvent {
    System::events().pop().expect("Event expected").event
}
