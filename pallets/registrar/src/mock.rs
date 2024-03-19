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
    crate::{self as pallet_registrar, RegistrarHooks},
    frame_support::{
        traits::{ConstU16, ConstU64},
        weights::Weight,
    },
    parity_scale_codec::{Decode, Encode},
    sp_core::{parameter_types, ConstU32, H256},
    sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    },
    std::collections::BTreeMap,
    tp_traits::ParaId,
};

type Block = frame_system::mocking::MockBlock<Test>;
pub type Balance = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        ParaRegistrar: pallet_registrar,
        Mock: mock_data,
    }
);

impl frame_system::Config for Test {
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
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
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

pub struct CurrentSessionIndexGetter;

impl tp_traits::GetSessionIndex<u32> for CurrentSessionIndexGetter {
    /// Returns current session index.
    fn session_index() -> u32 {
        // For tests, let 1 session be 5 blocks
        (System::block_number() / 5) as u32
    }
}

parameter_types! {
    pub const DepositAmount: Balance = 100;
    pub const MaxLengthTokenSymbol: u32 = 255;
}
impl pallet_registrar::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RegistrarOrigin = frame_system::EnsureRoot<u64>;
    type MaxLengthParaIds = ConstU32<1000>;
    type MaxGenesisDataSize = ConstU32<5_000_000>;
    type MaxLengthTokenSymbol = MaxLengthTokenSymbol;
    type SessionDelay = ConstU32<2>;
    type SessionIndex = u32;
    type CurrentSessionIndex = CurrentSessionIndexGetter;
    type Currency = Balances;
    type DepositAmount = DepositAmount;
    type RegistrarHooks = Mock;
    type WeightInfo = ();
}

// Pallet to provide some mock data, used to test
#[frame_support::pallet]
pub mod mock_data {
    use {super::*, frame_support::pallet_prelude::*};

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn mock)]
    pub(super) type Mock<T: Config> = StorageValue<_, Mocks, ValueQuery>;

    impl<T: Config> Pallet<T> {
        pub fn get() -> Mocks {
            Mock::<T>::get()
        }
        pub fn mutate<F, R>(f: F) -> R
        where
            F: FnOnce(&mut Mocks) -> R,
        {
            Mock::<T>::mutate(f)
        }
    }
}

#[derive(Clone, Encode, Decode, PartialEq, sp_core::RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum HookCall {
    MarkedValid(ParaId),
    Deregistered(ParaId),
}

pub enum HookCallType {
    MarkedValid,
    Deregistered,
}

// We use the mock_data pallet to test registrar hooks: we store a list of all the calls, and then check that there
// are no consecutive calls. Because there used to be a bug where the deregister hook was called twice.
impl<T> RegistrarHooks for mock_data::Pallet<T> {
    fn para_deregistered(para_id: ParaId) -> Weight {
        Mock::mutate(|m| {
            m.called_hooks.push(HookCall::Deregistered(para_id));

            Weight::default()
        })
    }

    fn para_marked_valid_for_collating(para_id: ParaId) -> Weight {
        Mock::mutate(|m| {
            m.called_hooks.push(HookCall::MarkedValid(para_id));

            Weight::default()
        })
    }

    fn check_valid_for_collating(_para_id: ParaId) -> crate::DispatchResult {
        // Ignored, we already test this in integration tests
        Ok(())
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn benchmarks_ensure_valid_for_collating(_para_id: ParaId) {}
}

impl mock_data::Config for Test {}

#[derive(
    Clone, Default, Encode, Decode, PartialEq, sp_core::RuntimeDebug, scale_info::TypeInfo,
)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Mocks {
    pub called_hooks: Vec<HookCall>,
}

impl Drop for Mocks {
    fn drop(&mut self) {
        self.check_consistency();
    }
}

impl Mocks {
    pub fn check_consistency(&self) {
        /// Asserts that the calls for each ParaId alternate between MarkedValid and Deregister,
        /// we never see two calls with the same type.
        pub fn assert_alternating(hook_calls: &[HookCall]) {
            let mut last_call_type: BTreeMap<ParaId, HookCallType> = BTreeMap::new();

            for call in hook_calls {
                match call {
                    HookCall::MarkedValid(para_id) => {
                        if let Some(HookCallType::MarkedValid) = last_call_type.get(para_id) {
                            panic!(
                                "Two consecutive MarkedValid calls for ParaId: {:?}",
                                para_id
                            );
                        }
                        last_call_type.insert(*para_id, HookCallType::MarkedValid);
                    }
                    HookCall::Deregistered(para_id) => {
                        if let Some(HookCallType::Deregistered) = last_call_type.get(para_id) {
                            panic!(
                                "Two consecutive Deregistered calls for ParaId: {:?}",
                                para_id
                            );
                        }
                        last_call_type.insert(*para_id, HookCallType::Deregistered);
                    }
                }
            }
        }

        // For each para id, the calls must alterante between MarkedValid and Deregister
        assert_alternating(&self.called_hooks);
        // Since para ids can already be registered in genesis, we cannot assert that the first call is MarkedValid
    }
}

const ALICE: u64 = 1;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(ALICE, 1_000)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext_with_genesis(
    para_ids: Vec<(ParaId, ContainerChainGenesisData<MaxLengthTokenSymbol>)>,
) -> sp_io::TestExternalities {
    RuntimeGenesisConfig {
        system: Default::default(),
        balances: Default::default(),
        para_registrar: pallet_registrar::GenesisConfig { para_ids },
    }
    .build_storage()
    .unwrap()
    .into()
}

pub fn empty_genesis_data() -> ContainerChainGenesisData<MaxLengthTokenSymbol> {
    ContainerChainGenesisData {
        storage: Default::default(),
        name: Default::default(),
        id: Default::default(),
        fork_id: Default::default(),
        extensions: Default::default(),
        properties: Default::default(),
    }
}

pub const SESSION_LEN: u64 = 5;

pub fn run_to_session(n: u32) {
    let block_number = SESSION_LEN * u64::from(n);
    run_to_block(block_number + 1);
}

pub fn run_to_block(n: u64) {
    let old_block_number = System::block_number();

    for x in (old_block_number + 1)..=n {
        System::reset_events();
        System::set_block_number(x);

        if x % SESSION_LEN == 1 {
            let session_index = (x / SESSION_LEN) as u32;
            ParaRegistrar::initializer_on_new_session(&session_index);
        }
    }
}
