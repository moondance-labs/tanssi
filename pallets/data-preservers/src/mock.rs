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
    crate::{self as pallet_data_preservers},
    dp_core::ParaId,
    frame_support::{
        dispatch::DispatchErrorWithPostInfo,
        pallet_prelude::*,
        parameter_types,
        traits::{ConstU128, ConstU64, EitherOfDiverse, EnsureOriginWithArg, Everything},
    },
    frame_system::{EnsureRoot, EnsureSigned, RawOrigin},
    serde::{Deserialize, Serialize},
    sp_core::H256,
    sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage, Either,
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
        DataPreservers: pallet_data_preservers,
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
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
    type ExtensionsWeightInfo = ();
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
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
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
    pub(super) type Mock<T: Config> = StorageValue<_, Mocks, ValueQuery>;

    impl<T: Config> Pallet<T> {
        pub fn mock() -> Mocks {
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

#[derive(
    Clone,
    Encode,
    Decode,
    PartialEq,
    sp_core::RuntimeDebug,
    scale_info::TypeInfo,
    serde::Serialize,
    serde::Deserialize,
)]
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
            MockData::mock()
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
        let o = MockData::mock()
            .container_chain_managers
            .get(para_id)
            .unwrap()
            .unwrap();

        Ok(O::from(RawOrigin::Signed(o.into())))
    }
}

pub struct MockContainerChainManager<T> {
    _phantom: PhantomData<T>,
}

impl<O, T> EnsureOriginWithArg<O, ParaId> for MockContainerChainManager<T>
where
    T: crate::Config,
    O: From<RawOrigin<T::AccountId>>,
    Result<RawOrigin<T::AccountId>, O>: From<O>,
    u64: From<T::AccountId>,
    T::AccountId: From<u64>,
    O: Clone,
{
    type Success = T::AccountId;

    fn try_origin(o: O, para_id: &ParaId) -> Result<Self::Success, O> {
        let origin = <EnsureSigned<T::AccountId> as EnsureOriginWithArg<O, ParaId>>::try_origin(
            o.clone(),
            para_id,
        )?;

        // This check will only pass if both are true:
        // * The para_id has a deposit in pallet_registrar
        // * The deposit creator is the signed_account
        MockData::mock()
            .container_chain_managers
            .get(para_id)
            .and_then(|inner| *inner)
            .and_then(|manager| {
                if manager != u64::from(origin.clone()) {
                    None
                } else {
                    Some(())
                }
            })
            .ok_or(o)?;

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
        let o = MockData::mock()
            .container_chain_managers
            .get(para_id)
            .unwrap()
            .unwrap();

        Ok(O::from(RawOrigin::Signed(o.into())))
    }
}

#[derive(
    Serialize,
    Deserialize,
    RuntimeDebug,
    PartialEq,
    Eq,
    Encode,
    Decode,
    Copy,
    Clone,
    TypeInfo,
    MaxEncodedLen,
)]
pub enum ProviderRequest {
    Free,
    SomeKindOfPayment {
        // in this mock the provider requests a fixed amount
        amount: Balance,
    },
}

#[derive(
    Serialize, Deserialize, RuntimeDebug, PartialEq, Eq, Encode, Decode, Copy, Clone, TypeInfo,
)]
pub enum AssignerParameter {
    Free,
    SomeKindOfPayment {
        // in this mock the assigner can add funds to the requested amount
        extra: Balance,
    },
}

#[derive(
    Serialize,
    Deserialize,
    RuntimeDebug,
    PartialEq,
    Eq,
    Encode,
    Decode,
    Copy,
    Clone,
    TypeInfo,
    MaxEncodedLen,
)]
pub enum AssignmentWitness {
    Free,
    SomeKindOfPayment {
        // in this mock we store requested + extra to ensure `AssignmentProcessor` is called
        // properly. We also store the payer to perform a second payment while stopping the
        // assignement.
        payed: Balance,
        payer: AccountId,
    },
}

pub struct AssignmentProcessor;

impl pallet_data_preservers::AssignmentProcessor<AccountId> for AssignmentProcessor {
    /// Providers requests which kind of payment it accepts.
    type ProviderRequest = ProviderRequest;
    /// Extra parameter the assigner provides.
    type AssignerParameter = AssignerParameter;
    /// Represents the succesful outcome of the assignment.
    type AssignmentWitness = AssignmentWitness;

    fn try_start_assignment(
        assigner: AccountId,
        provider: AccountId,
        request: &Self::ProviderRequest,
        extra: Self::AssignerParameter,
    ) -> Result<Self::AssignmentWitness, DispatchErrorWithPostInfo> {
        let witness = match (request, extra) {
            (ProviderRequest::Free, AssignerParameter::Free) => AssignmentWitness::Free,
            (
                ProviderRequest::SomeKindOfPayment { amount },
                AssignerParameter::SomeKindOfPayment { extra },
            ) => {
                let total_amount = amount + extra;
                Balances::transfer_allow_death(
                    RuntimeOrigin::signed(assigner),
                    provider,
                    total_amount,
                )?;
                AssignmentWitness::SomeKindOfPayment {
                    payed: total_amount,
                    payer: assigner,
                }
            }
            _ => Err(crate::Error::<Test>::AssignmentPaymentRequestParameterMismatch)?,
        };

        Ok(witness)
    }

    fn try_stop_assignment(
        provider: AccountId,
        witness: Self::AssignmentWitness,
    ) -> Result<(), DispatchErrorWithPostInfo> {
        // for testing purposes there is also a payment at the end of the assignment
        match witness {
            AssignmentWitness::Free => (),
            AssignmentWitness::SomeKindOfPayment { payed, payer } => {
                Balances::transfer_allow_death(RuntimeOrigin::signed(payer), provider, payed)?;
            }
        };

        Ok(())
    }

    /// Return the values for a free assignment if it is supported.
    /// This is required to perform automatic migration from old Bootnodes storage.
    fn free_variant_values() -> Option<(
        Self::ProviderRequest,
        Self::AssignerParameter,
        Self::AssignmentWitness,
    )> {
        Some((
            Self::ProviderRequest::Free,
            Self::AssignerParameter::Free,
            Self::AssignmentWitness::Free,
        ))
    }

    // The values returned by the following functions should match with each other.
    #[cfg(feature = "runtime-benchmarks")]
    fn benchmark_provider_request() -> Self::ProviderRequest {
        ProviderRequest::Free
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn benchmark_assigner_parameter() -> Self::AssignerParameter {
        AssignerParameter::Free
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn benchmark_assignment_witness() -> Self::AssignmentWitness {
        AssignmentWitness::Free
    }
}

impl pallet_data_preservers::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeHoldReason = RuntimeHoldReason;
    type Currency = Balances;
    type ProfileId = u64;
    type AssignmentProcessor = AssignmentProcessor;
    type AssignmentOrigin = MockContainerChainManager<Test>;
    type ForceSetProfileOrigin = EnsureRoot<AccountId>;
    type MaxAssignmentsPerParaId = ConstU32<10>;
    type MaxNodeUrlLen = ConstU32<200>;
    type MaxParaIdsVecLen = ConstU32<20>;
    type ProfileDeposit = tp_traits::BytesDeposit<ConstU128<1000>, ConstU128<51>>;
    type WeightInfo = ();
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

        let mut ext: sp_io::TestExternalities = t.into();
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

pub(crate) fn events() -> Vec<crate::Event<Test>> {
    System::events()
        .into_iter()
        .map(|r| r.event)
        .filter_map(|e| {
            if let RuntimeEvent::DataPreservers(inner) = e {
                Some(inner)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
