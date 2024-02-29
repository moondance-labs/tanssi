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

use crate::GetPurchaseCoretimeCall;
use bounded_collections::ConstU128;
use sp_runtime::traits::Convert;
use sp_runtime::BuildStorage;
use {
    crate::{self as pallet_xcm_core_buyer},
    dp_core::ParaId,
    frame_support::{
        pallet_prelude::*,
        parameter_types,
        traits::{ConstU64, EitherOfDiverse, EnsureOriginWithArg, Everything},
    },
    frame_system::{EnsureSigned, RawOrigin},
    sp_core::H256,
    sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        Either,
    },
    sp_std::collections::btree_map::BTreeMap,
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
        XcmCoreBuyer: pallet_xcm_core_buyer,
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
    /// List of container chains, with the corresponding "manager" account.
    /// In dancebox, the manager is the one who put the deposit in pallet_registrar.
    /// The manager can be None if the chain was registered by root, or in genesis.
    pub container_chain_managers: BTreeMap<ParaId, Option<AccountId>>,
}

impl Default for Mocks {
    fn default() -> Self {
        Self {
            container_chain_managers: BTreeMap::from_iter([(ParaId::from(1001), None)]),
        }
    }
}

pub struct MockContainerChainManagerOrRootOrigin<T, RootOrigin> {
    // Configurable root origin
    container_chain_manager_origin: PhantomData<RootOrigin>,
    _phantom: PhantomData<T>,
}

impl<O, T, RootOrigin> EnsureOriginWithArg<O, ParaId>
    for MockContainerChainManagerOrRootOrigin<T, RootOrigin>
where
    T: crate::Config,
    RootOrigin: EnsureOriginWithArg<O, ParaId>,
    O: From<RawOrigin<T::AccountId>>,
    Result<RawOrigin<T::AccountId>, O>: From<O>,
    u64: From<T::AccountId>,
    T::AccountId: From<u64>,
    O: Clone,
{
    type Success = Either<T::AccountId, <RootOrigin as EnsureOriginWithArg<O, ParaId>>::Success>;

    fn try_origin(o: O, para_id: &ParaId) -> Result<Self::Success, O> {
        let origin = EitherOfDiverse::<EnsureSigned<T::AccountId>, RootOrigin>::try_origin(
            o.clone(),
            para_id,
        )?;

        if let Either::Left(signed_account) = &origin {
            // This check will only pass if both are true:
            // * The para_id has a deposit in pallet_registrar
            // * The deposit creator is the signed_account
            MockData::get()
                .container_chain_managers
                .get(para_id)
                .and_then(|inner| *inner)
                .and_then(|manager| {
                    if manager != u64::from(signed_account.clone()) {
                        None
                    } else {
                        Some(())
                    }
                })
                .ok_or(o)?;
        }

        Ok(origin)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin(para_id: &ParaId) -> Result<O, ()> {
        // Return container chain manager, or register container chain as ALICE if it does not exist
        MockData::mutate(|m| {
            m.container_chain_managers
                .entry(*para_id)
                .or_insert_with(move || {
                    const ALICE: u64 = 1;

                    Some(ALICE)
                });
        });

        // This panics if the container chain was registered by root (None)
        let o = MockData::get()
            .container_chain_managers
            .get(para_id)
            .unwrap()
            .unwrap();

        Ok(O::from(RawOrigin::Signed(o.into())))
    }
}

parameter_types! {
    pub const ParachainId: ParaId = ParaId::new(1000);
}

impl pallet_xcm_core_buyer::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type XcmBuyExecutionDot = ConstU128<1_000>;
    type XcmSender = ();
    type GetPurchaseCoretimeCall = EncodedCallToBuyCoretime;
    type GetBlockNumber = ();
    type AccountIdToArray32 = AccountIdToArray32;
    type SelfParaId = ParachainId;
    type UnsignedPriority = ();

    type WeightInfo = ();
}

pub struct GetBlockNumber;

impl Get<u32> for GetBlockNumber {
    fn get() -> u32 {
        System::block_number() as u32
    }
}

pub struct AccountIdToArray32;

impl Convert<u64, [u8; 32]> for AccountIdToArray32 {
    fn convert(a: u64) -> [u8; 32] {
        let mut res = [0; 32];

        res[..8].copy_from_slice(&a.to_le_bytes());

        res
    }
}

pub struct EncodedCallToBuyCoretime;

impl GetPurchaseCoretimeCall for EncodedCallToBuyCoretime {
    fn get_encoded(_max_amount: u128, _para_id: ParaId) -> (Vec<u8>, Weight) {
        let weight = Weight::from_parts(1_000_000_000, 100_000);

        let encoded_call = vec![];

        (encoded_call, weight)
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

pub(crate) fn events() -> Vec<pallet_xcm_core_buyer::Event<Test>> {
    System::events()
        .into_iter()
        .map(|r| r.event)
        .filter_map(|e| {
            if let RuntimeEvent::XcmCoreBuyer(inner) = e {
                Some(inner)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
