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
    crate::{self as pallet_services_payment_price_oracle},
    frame_support::traits::{ConstU16, ConstU64},
    frame_system as system,
    sp_core::H256,
    sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    },
};

pub const ALICE: u64 = 1;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        ServicesPaymentPriceOracle: pallet_services_payment_price_oracle,
    }
);

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Block = Block;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
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
    type RuntimeTask = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
    type ExtensionsWeightInfo = ();
}

frame_support::parameter_types! {
    pub const FixedMonthlyServicesCostUsd: u128 = 2_000_000_000; // with 6 decimal
    pub const BlockTimeMs: u64 = 6000;
    pub const SessionDurationBlocks: u32 = 600; // 1 hour
    pub const TokenDecimals: u32 = 12;
    pub const ReferenceBlockCost: u128 = 30_000_000_000; // 0.03 STAR
    pub const ReferenceSessionCost: u128 = 50_000_000_000_000; // 50 STAR
}

impl pallet_services_payment_price_oracle::Config for Test {
    type SetPriceOrigin = frame_system::EnsureRoot<u64>;
    type FixedMonthlyServicesCostUsd = FixedMonthlyServicesCostUsd;
    type BlockTimeMs = BlockTimeMs;
    type SessionDurationBlocks = SessionDurationBlocks;
    type TokenDecimals = TokenDecimals;
    type ReferenceBlockCost = ReferenceBlockCost;
    type ReferenceSessionCost = ReferenceSessionCost;
    type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}

// Build genesis storage with initial price
pub fn new_test_ext_with_price(price: u128) -> sp_io::TestExternalities {
    let mut storage = system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    pallet_services_payment_price_oracle::GenesisConfig::<Test> {
        initial_price: Some(price),
        _config: Default::default(),
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    storage.into()
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        System::set_block_number(System::block_number() + 1);
    }
}
