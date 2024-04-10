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
    crate::{
        self as pallet_services_payment, ProvideBlockProductionCost, ProvideCollatorAssignmentCost,
    },
    cumulus_primitives_core::ParaId,
    frame_support::{
        pallet_prelude::*,
        parameter_types,
        traits::{ConstU32, ConstU64, Everything},
    },
    frame_system::EnsureRoot,
    sp_core::H256,
    sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    },
};

type Block = frame_system::mocking::MockBlock<Test>;
type AccountId = u64;
type Balance = u128;

frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        PaymentServices: pallet_services_payment::{Pallet, Call, Config<T>, Storage, Event<T>}
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
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type RuntimeTask = ();
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 1;
}

impl pallet_balances::Config for Test {
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 4];
    type MaxLocks = ();
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type MaxHolds = ();
    type WeightInfo = ();
}

parameter_types! {
    pub const FreeBlockProductionCredits: u64 = 5;
    pub const FreeCollatorAssignmentCredits: u32 = 5;
}

impl pallet_services_payment::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeForBlock = ();
    type OnChargeForCollatorAssignment = ();
    type OnChargeForCollatorAssignmentTip = ();
    type Currency = Balances;
    type ProvideBlockProductionCost = BlockProductionCost<Test>;
    type ProvideCollatorAssignmentCost = CollatorAssignmentProductionCost<Test>;
    type FreeBlockProductionCredits = FreeBlockProductionCredits;
    type FreeCollatorAssignmentCredits = FreeCollatorAssignmentCredits;
    type ManagerOrigin = EnsureRoot<AccountId>;
    type WeightInfo = ();
}

pub(crate) const FIXED_BLOCK_PRODUCTION_COST: u128 = 100;
pub(crate) const FIXED_COLLATOR_ASSIGNMENT_COST: u128 = 200;

pub struct BlockProductionCost<Test>(PhantomData<Test>);
impl ProvideBlockProductionCost<Test> for BlockProductionCost<Test> {
    fn block_cost(_para_id: &ParaId) -> (u128, Weight) {
        (FIXED_BLOCK_PRODUCTION_COST, Weight::zero())
    }
}

pub struct CollatorAssignmentProductionCost<Test>(PhantomData<Test>);
impl ProvideCollatorAssignmentCost<Test> for CollatorAssignmentProductionCost<Test> {
    fn collator_assignment_cost(_para_id: &ParaId) -> (u128, Weight) {
        (FIXED_COLLATOR_ASSIGNMENT_COST, Weight::zero())
    }
}

#[derive(Default)]
pub struct ExtBuilder {
    balances: Vec<(AccountId, Balance)>,
}

impl ExtBuilder {
    pub fn with_balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
        self.balances = balances;
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: self.balances,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }
}

pub(crate) fn events() -> Vec<pallet_services_payment::Event<Test>> {
    System::events()
        .into_iter()
        .map(|r| r.event)
        .filter_map(|e| {
            if let RuntimeEvent::PaymentServices(inner) = e {
                Some(inner)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
