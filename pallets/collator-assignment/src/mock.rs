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
        self as pallet_collator_assignment, GetRandomnessForNextBlock,
        RotateCollatorsEveryNSessions,
    },
    frame_support::{
        parameter_types,
        traits::{ConstU16, ConstU64, Hooks},
    },
    frame_system as system,
    parity_scale_codec::{Decode, Encode},
    sp_core::H256,
    sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    },
    tp_traits::{ParaId, RemoveInvulnerables, RemoveParaIdsWithNoCredits},
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        MockData: mock_data,
        CollatorAssignment: pallet_collator_assignment,
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

#[derive(
    Default, Clone, Encode, Decode, PartialEq, sp_core::RuntimeDebug, scale_info::TypeInfo,
)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Mocks {
    pub min_orchestrator_chain_collators: u32,
    pub max_orchestrator_chain_collators: u32,
    pub collators_per_container: u32,
    pub collators: Vec<u64>,
    pub container_chains: Vec<u32>,
    pub random_seed: [u8; 32],
}

impl mock_data::Config for Test {}

// In tests, we ignore the session_index param, so changes to the configuration are instant

pub struct HostConfigurationGetter;

parameter_types! {
    pub const ParachainId: ParaId = ParaId::new(200);
}

impl pallet_collator_assignment::GetHostConfiguration<u32> for HostConfigurationGetter {
    fn min_collators_for_orchestrator(_session_index: u32) -> u32 {
        MockData::mock().min_orchestrator_chain_collators
    }

    fn max_collators_for_orchestrator(_session_index: u32) -> u32 {
        MockData::mock().max_orchestrator_chain_collators
    }

    fn collators_per_container(_session_index: u32) -> u32 {
        MockData::mock().collators_per_container
    }
}

pub struct CollatorsGetter;

impl GetCollators<u64, u32> for CollatorsGetter {
    fn collators(_session_index: u32) -> Vec<u64> {
        MockData::mock().collators
    }
}

pub struct ContainerChainsGetter;

impl tp_traits::GetSessionContainerChains<u32> for ContainerChainsGetter {
    fn session_container_chains(_session_index: u32) -> Vec<ParaId> {
        MockData::mock()
            .container_chains
            .iter()
            .cloned()
            .map(ParaId::from)
            .collect()
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn set_session_container_chains(_session_index: u32, para_ids: &[ParaId]) {
        MockData::mutate(|mocks| {
            mocks.container_chains = para_ids.iter().cloned().map(|x| x.into()).collect();
        })
    }
}

pub struct MockGetRandomnessForNextBlock;

impl GetRandomnessForNextBlock<u64> for MockGetRandomnessForNextBlock {
    fn should_end_session(n: u64) -> bool {
        n % 5 == 0
    }

    fn get_randomness() -> [u8; 32] {
        MockData::mock().random_seed
    }
}

parameter_types! {
    pub const CollatorRotationSessionPeriod: u32 = 5;
}

impl pallet_collator_assignment::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type SessionIndex = u32;
    type HostConfiguration = HostConfigurationGetter;
    type ContainerChains = ContainerChainsGetter;
    type SelfParaId = ParachainId;
    type ShouldRotateAllCollators = RotateCollatorsEveryNSessions<CollatorRotationSessionPeriod>;
    type GetRandomnessForNextBlock = MockGetRandomnessForNextBlock;
    type RemoveInvulnerables = RemoveAccountIdsAbove100;
    type RemoveParaIdsWithNoCredits = RemoveParaIdsAbove5000;
    type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}

pub trait GetCollators<AccountId, SessionIndex> {
    fn collators(session_index: SessionIndex) -> Vec<AccountId>;
}

pub fn run_to_block(n: u64) {
    let old_block_number = System::block_number();
    let session_len = 5;

    for x in (old_block_number + 1)..=n {
        System::reset_events();
        System::set_block_number(x);
        CollatorAssignment::on_initialize(x);

        if x % session_len == 1 {
            let session_index = (x / session_len) as u32;
            CollatorAssignment::initializer_on_new_session(
                &session_index,
                CollatorsGetter::collators(session_index),
            );
        }

        CollatorAssignment::on_finalize(x);
    }
}

/// Any AccountId >= 100 will be considered an invulnerable
pub struct RemoveAccountIdsAbove100;

impl RemoveInvulnerables<u64> for RemoveAccountIdsAbove100 {
    fn remove_invulnerables(collators: &mut Vec<u64>, num_invulnerables: usize) -> Vec<u64> {
        let mut invulnerables = vec![];
        collators.retain(|x| {
            if invulnerables.len() < num_invulnerables && *x >= 100 {
                invulnerables.push(*x);
                false
            } else {
                true
            }
        });

        invulnerables
    }
}

/// Any ParaId >= 5000 will be considered to not have enough credits
pub struct RemoveParaIdsAbove5000;

impl RemoveParaIdsWithNoCredits for RemoveParaIdsAbove5000 {
    fn remove_para_ids_with_no_credits(para_ids: &mut Vec<ParaId>) {
        para_ids.retain(|para_id| *para_id <= ParaId::from(5000));
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn make_valid_para_ids(para_ids: &[ParaId]) {
        for para_id in para_ids {
            assert!(*para_id > ParaId::from(5000), "{}", para_id);
        }
    }
}
