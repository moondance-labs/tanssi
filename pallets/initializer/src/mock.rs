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
    crate as pallet_initializer,
    frame_support::traits::{ConstU16, ConstU64},
    frame_system as system,
    sp_core::H256,
    sp_runtime::{
        testing::UintAuthorityId,
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    },
    sp_std::cell::RefCell,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Initializer: pallet_initializer,
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
}

thread_local! {
    pub static SESSION_CHANGE_VALIDATORS: RefCell<Option<(u32, Vec<u64>)>> = const { RefCell::new(None) };
}

pub fn session_change_validators() -> Option<(u32, Vec<u64>)> {
    SESSION_CHANGE_VALIDATORS.with(|q| (*q.borrow()).clone())
}

pub struct OwnApplySession;
impl pallet_initializer::ApplyNewSession<Test> for OwnApplySession {
    fn apply_new_session(
        _changed: bool,
        session_index: u32,
        all_validators: Vec<(u64, UintAuthorityId)>,
        _queued: Vec<(u64, UintAuthorityId)>,
    ) {
        let validators: Vec<_> = all_validators.iter().map(|(k, _)| *k).collect();
        SESSION_CHANGE_VALIDATORS.with(|r| *r.borrow_mut() = Some((session_index, validators)));
    }
}

impl pallet_initializer::Config for Test {
    type SessionIndex = u32;

    /// The identifier type for an authority.
    type AuthorityId = UintAuthorityId;

    type SessionHandler = OwnApplySession;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    // Start with None in the global var
    SESSION_CHANGE_VALIDATORS.with(|r| *r.borrow_mut() = None);

    system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
