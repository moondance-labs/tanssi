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
    frame_support::{
        parameter_types,
        traits::{ConstU32, ConstU64, Everything, OnFinalize, OnInitialize},
    },
    sp_core::H256,
    sp_runtime::{
        testing::UintAuthorityId,
        traits::{BlakeTwo256, ConvertInto, IdentityLookup, OpaqueKeys},
        BuildStorage, RuntimeAppPublic,
    },
    sp_staking::SessionIndex,
    sp_std::{collections::btree_set::BTreeSet, marker::PhantomData},
    tp_traits::{ForSession, ParaId},
};

type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u64;

pub const COLLATOR_1: AccountId = 1;
pub const COLLATOR_2: AccountId = 2;
pub const COLLATOR_3: AccountId = 3;
pub const CONTAINER_CHAIN_ID_1: ParaId = ParaId::new(3000);
pub const CONTAINER_CHAIN_ID_2: ParaId = ParaId::new(3001);
pub const CONTAINER_CHAIN_ID_3: ParaId = ParaId::new(3002);
pub const SESSION_BLOCK_LENGTH: u64 = 5;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Session: pallet_session,
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

sp_runtime::impl_opaque_keys! {
    pub struct MockSessionKeys {
        // a key for aura authoring
        pub aura: UintAuthorityId,
    }
}
parameter_types! {
    pub static Validators: Option<Vec<u64>> = Some(vec![
        1,
        2,
    ]);
}

pub struct TestSessionManager;
impl pallet_session::SessionManager<u64> for TestSessionManager {
    fn new_session(_new_index: SessionIndex) -> Option<Vec<u64>> {
        Validators::get()
    }
    fn end_session(_: SessionIndex) {}
    fn start_session(_: SessionIndex) {}
}

impl From<UintAuthorityId> for MockSessionKeys {
    fn from(aura: sp_runtime::testing::UintAuthorityId) -> Self {
        Self { aura }
    }
}

parameter_types! {
    pub static SessionHandlerCollators: Vec<u64> = Vec::new();
    pub static SessionChangeBlock: u64 = 0;
}
pub struct TestSessionHandler;
impl pallet_session::SessionHandler<u64> for TestSessionHandler {
    const KEY_TYPE_IDS: &'static [sp_runtime::KeyTypeId] =
        &[sp_runtime::testing::UintAuthorityId::ID];
    fn on_genesis_session<Ks: OpaqueKeys>(keys: &[(u64, Ks)]) {
        SessionHandlerCollators::set(keys.iter().map(|(a, _)| *a).collect::<Vec<_>>())
    }
    fn on_new_session<Ks: OpaqueKeys>(_: bool, keys: &[(u64, Ks)], _: &[(u64, Ks)]) {
        SessionChangeBlock::set(System::block_number());
        SessionHandlerCollators::set(keys.iter().map(|(a, _)| *a).collect::<Vec<_>>());
        InactivityTracking::process_ended_session()
    }
    fn on_before_session_ending() {
        InactivityTracking::on_before_session_ending()
    }
    fn on_disabled(_: u32) {}
}

parameter_types! {
    pub const Offset: u64 = 0;
    pub const Period: u64 = SESSION_BLOCK_LENGTH;
}
impl pallet_session::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    // we don't have stash and controller, thus we don't need the convert as well.
    type ValidatorIdOf = ConvertInto;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionManager = TestSessionManager;
    type SessionHandler = TestSessionHandler;
    type Keys = MockSessionKeys;
    type WeightInfo = ();
}

pub struct CurrentSessionIndexGetter;

impl tp_traits::GetSessionIndex<u32> for CurrentSessionIndexGetter {
    /// Returns current session index.
    fn session_index() -> u32 {
        Session::current_index()
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn skip_to_session(_session_index: u32) {}
}

pub struct MockContainerChainsInfoFetcher;
impl tp_traits::GetContainerChainsWithCollators<AccountId> for MockContainerChainsInfoFetcher {
    fn container_chains_with_collators(_for_session: ForSession) -> Vec<(ParaId, Vec<AccountId>)> {
        vec![
            (CONTAINER_CHAIN_ID_1, vec![COLLATOR_1, COLLATOR_2]),
            (CONTAINER_CHAIN_ID_2, vec![]),
            (CONTAINER_CHAIN_ID_3, vec![COLLATOR_3]),
        ]
    }

    fn get_all_collators_assigned_to_chains(_for_session: ForSession) -> BTreeSet<AccountId> {
        let mut collators = BTreeSet::new();
        collators.insert(COLLATOR_1);
        collators.insert(COLLATOR_2);
        collators.insert(COLLATOR_3);
        collators
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn set_container_chains_with_collators(
        for_session: ForSession,
        container_chains: &[(ParaId, Vec<AccountId>)],
    ) {
    }
}

pub struct MockParathreadHelper;
impl tp_traits::ParathreadHelper for MockParathreadHelper {
    fn get_parathreads_for_session() -> BTreeSet<ParaId> {
        let mut paras_for_session = BTreeSet::new();
        paras_for_session.insert(CONTAINER_CHAIN_ID_3);
        paras_for_session
    }
}
pub struct MockInvulnerableCheckHandler<AccountId>(PhantomData<AccountId>);

impl tp_traits::InvulnerablesHelper<AccountId> for MockInvulnerableCheckHandler<AccountId> {
    fn is_invulnerable(account: &AccountId) -> bool {
        *account == COLLATOR_2
    }
}

impl pallet_inactivity_tracking::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type CollatorId = AccountId;
    type MaxInactiveSessions = ConstU32<2>;
    type MaxCollatorsPerSession = ConstU32<5>;
    type MaxContainerChains = ConstU32<3>;
    type CurrentSessionIndex = CurrentSessionIndexGetter;
    type CurrentCollatorsFetcher = MockContainerChainsInfoFetcher;
    type GetSelfChainBlockAuthor = ();
    type ParaFilter = MockParathreadHelper;
    type InvulnerablesFilter = MockInvulnerableCheckHandler<AccountId>;
    type WeightInfo = ();
}

#[derive(Default)]
pub(crate) struct ExtBuilder;

impl ExtBuilder {
    pub(crate) fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .expect("Frame system builds valid default genesis config");
        let balances = vec![(1, 100), (2, 100)];
        let keys = balances
            .iter()
            .map(|&(i, _)| {
                (
                    i,
                    i,
                    MockSessionKeys {
                        aura: UintAuthorityId(i),
                    },
                )
            })
            .collect::<Vec<_>>();
        let session = pallet_session::GenesisConfig::<Test> {
            keys,
            ..Default::default()
        };
        session.assimilate_storage(&mut t).unwrap();
        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

/// Rolls forward one block. Returns the new block number.
#[allow(dead_code)]
pub(crate) fn roll_one_block() -> u64 {
    InactivityTracking::on_finalize(System::block_number());
    Session::on_finalize(System::block_number());
    System::on_finalize(System::block_number());

    System::set_block_number(System::block_number() + 1);

    System::on_initialize(System::block_number());
    Session::on_initialize(System::block_number());
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
