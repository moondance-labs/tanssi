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
    crate::{self as pallet_inflation_rewards},
    bounded_collections::bounded_vec,
    dp_core::ParaId,
    frame_support::{
        pallet_prelude::*,
        parameter_types,
        traits::{
            fungible::{Balanced, Credit},
            ConstU64, Everything,
        },
    },
    sp_core::H256,
    sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage, Perbill,
    },
};

type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u64;
pub type Balance = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances,
        InflationRewards: pallet_inflation_rewards,
        MockData: mock_data,
    }
);

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type Block = Block;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
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
    type MaxHolds = ConstU32<5>;
    type WeightInfo = ();
}

// Pallet to provide some mock data, used to test
#[frame_support::pallet]
pub mod mock_data {
    use super::*;

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

impl mock_data::Config for Test {}

#[derive(Clone, Encode, Decode, PartialEq, sp_core::RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Mocks {
    pub container_chains: BoundedVec<ParaId, ConstU32<5>>,
    pub orchestrator_author: AccountId,
}

impl Default for Mocks {
    fn default() -> Self {
        Self {
            container_chains: bounded_vec![1001.into()],
            orchestrator_author: 1,
        }
    }
}

pub struct MockContainerChainGetter;

impl tp_traits::GetCurrentContainerChains for MockContainerChainGetter {
    type MaxContainerChains = ConstU32<5>;

    fn current_container_chains() -> BoundedVec<ParaId, Self::MaxContainerChains> {
        MockData::mock().container_chains
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn set_current_container_chains(container_chains: &[ParaId]) {
        MockData::mutate(|m| {
            m.container_chains = container_chains.to_vec().try_into().unwrap();
        });
    }
}

pub struct MockGetSelfChainBlockAuthor;

impl Get<AccountId> for MockGetSelfChainBlockAuthor {
    fn get() -> AccountId {
        MockData::mock().orchestrator_author
    }
}

pub struct OnUnbalancedInflation;
impl frame_support::traits::OnUnbalanced<Credit<AccountId, Balances>> for OnUnbalancedInflation {
    fn on_nonzero_unbalanced(credit: Credit<AccountId, Balances>) {
        let _ = <Balances as Balanced<_>>::resolve(&OnUnbalancedInflationAccount::get(), credit);
    }
}

pub struct MockRewardsDistributor;
impl tp_traits::DistributeRewards<AccountId, Credit<AccountId, Balances>>
    for MockRewardsDistributor
{
    fn distribute_rewards(
        rewarded: AccountId,
        amount: Credit<AccountId, Balances>,
    ) -> DispatchResultWithPostInfo {
        <<Test as pallet_inflation_rewards::Config>::Currency as Balanced<AccountId>>::resolve(
            &rewarded, amount,
        )
        .map_err(|_| DispatchError::NoProviders)?;
        Ok(().into())
    }
}

parameter_types! {
    pub OnUnbalancedInflationAccount: AccountId = 0;
    pub PendingRewardsAccount: AccountId = 99;
    pub const RewardsPortion: Perbill = Perbill::from_percent(70);
    pub const InflationRate: Perbill = Perbill::from_percent(1);
}

impl pallet_inflation_rewards::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type ContainerChains = MockContainerChainGetter;
    type GetSelfChainBlockAuthor = MockGetSelfChainBlockAuthor;
    type InflationRate = InflationRate;
    type OnUnbalanced = OnUnbalancedInflation;
    type PendingRewardsAccount = PendingRewardsAccount;
    type StakingRewardsDistributor = MockRewardsDistributor;
    type RewardsPortion = RewardsPortion;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    let balances = vec![(0, 10_000)];

    pallet_balances::GenesisConfig::<Test> { balances }
        .assimilate_storage(&mut t)
        .unwrap();

    t.into()
}
