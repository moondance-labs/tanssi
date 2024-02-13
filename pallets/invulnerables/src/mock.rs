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
    super::*,
    crate as invulnerables,
    frame_support::{
        ord_parameter_types, parameter_types,
        traits::{ConstU32, ValidatorRegistration},
    },
    frame_system::{self as system, EnsureSignedBy},
    pallet_balances::AccountData,
    sp_core::H256,
    sp_runtime::{
        testing::UintAuthorityId,
        traits::{BlakeTwo256, IdentityLookup, OpaqueKeys},
        BuildStorage, RuntimeAppPublic,
    },
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Invulnerables: invulnerables,
        Session: pallet_session,
        Balances: pallet_balances,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type Nonce = u64;
    type Block = Block;
    type RuntimeTask = ();
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 5;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Balance = u64;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type ReserveIdentifier = [u8; 8];
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type FreezeIdentifier = ();
    type MaxLocks = ();
    type MaxReserves = MaxReserves;
    type MaxHolds = ConstU32<0>;
    type MaxFreezes = ConstU32<0>;
}

ord_parameter_types! {
    pub const RootAccount: u64 = 777;
}

pub struct IsRegistered;
impl ValidatorRegistration<u64> for IsRegistered {
    fn is_registered(id: &u64) -> bool {
        *id != 42u64
    }
}

impl Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type UpdateOrigin = EnsureSignedBy<RootAccount, u64>;
    type MaxInvulnerables = ConstU32<20>;
    type CollatorId = <Self as frame_system::Config>::AccountId;
    type CollatorIdOf = IdentityCollator;
    type CollatorRegistration = IsRegistered;
    type WeightInfo = ();
    #[cfg(feature = "runtime-benchmarks")]
    type Currency = Balances;
}

sp_runtime::impl_opaque_keys! {
    pub struct MockSessionKeys {
        // a key for aura authoring
        pub aura: UintAuthorityId,
    }
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
    const KEY_TYPE_IDS: &'static [sp_runtime::KeyTypeId] = &[UintAuthorityId::ID];
    fn on_genesis_session<Ks: OpaqueKeys>(keys: &[(u64, Ks)]) {
        SessionHandlerCollators::set(keys.iter().map(|(a, _)| *a).collect::<Vec<_>>())
    }
    fn on_new_session<Ks: OpaqueKeys>(_: bool, keys: &[(u64, Ks)], _: &[(u64, Ks)]) {
        SessionChangeBlock::set(System::block_number());
        SessionHandlerCollators::set(keys.iter().map(|(a, _)| *a).collect::<Vec<_>>())
    }
    fn on_before_session_ending() {}
    fn on_disabled(_: u32) {}
}

parameter_types! {
    pub const Offset: u64 = 0;
    pub const Period: u64 = 10;
}

impl pallet_session::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    // we don't have stash and controller, thus we don't need the convert as well.
    type ValidatorIdOf = IdentityCollator;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionManager = Invulnerables;
    type SessionHandler = TestSessionHandler;
    type Keys = MockSessionKeys;
    type WeightInfo = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    let invulnerables = vec![1, 2];

    let balances = vec![(1, 100), (2, 100), (3, 100), (4, 100), (5, 100)];
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
    let session = pallet_session::GenesisConfig::<Test> { keys };
    pallet_balances::GenesisConfig::<Test> { balances }
        .assimilate_storage(&mut t)
        .unwrap();
    invulnerables::GenesisConfig::<Test> { invulnerables }
        .assimilate_storage(&mut t)
        .unwrap();
    session.assimilate_storage(&mut t).unwrap();

    t.into()
}

pub fn initialize_to_block(n: u64) {
    for i in System::block_number() + 1..=n {
        System::set_block_number(i);
        <AllPalletsWithSystem as frame_support::traits::OnInitialize<u64>>::on_initialize(i);
    }
}
