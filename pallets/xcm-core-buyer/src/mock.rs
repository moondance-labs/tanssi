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

use pallet_xcm::Origin;
use staging_xcm::prelude::{GlobalConsensus, Parachain, X2};
use staging_xcm::prelude::{InteriorMultiLocation, NetworkId};
use {
    crate::{
        self as pallet_xcm_core_buyer, GetParathreadCollators, GetPurchaseCoreCall,
        ParaIdIntoAccountTruncating, RelayXcmWeightConfigInner,
    },
    dp_core::ParaId,
    frame_support::{
        assert_ok,
        pallet_prelude::*,
        parameter_types,
        traits::{ConstU64, Everything},
    },
    sp_core::H256,
    sp_io::TestExternalities,
    sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    },
    sp_std::collections::btree_map::BTreeMap,
    staging_xcm::latest::{
        MultiAssets, MultiLocation, SendError, SendResult, SendXcm, Xcm, XcmHash,
    },
    tp_traits::{ParathreadParams, SlotFrequency},
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

/// Only needed for benchmark test suite
impl From<pallet_xcm::Origin> for RuntimeOrigin {
    fn from(_value: Origin) -> Self {
        RuntimeOrigin::root()
    }
}

/// Only needed for benchmark test suite
impl From<RuntimeOrigin> for Result<pallet_xcm::Origin, RuntimeOrigin> {
    fn from(_value: RuntimeOrigin) -> Self {
        Ok(Origin::Response(MultiLocation::parent()))
    }
}

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
    pub container_chain_collators: BTreeMap<ParaId, Vec<AccountId>>,
    pub parathread_params: BTreeMap<ParaId, ParathreadParams>,
}

impl Default for Mocks {
    fn default() -> Self {
        Self {
            container_chain_collators: BTreeMap::from_iter([(ParaId::from(3333), vec![BOB])]),
            parathread_params: BTreeMap::from_iter([(
                ParaId::from(3333),
                ParathreadParams {
                    slot_frequency: SlotFrequency { min: 10, max: 10 },
                },
            )]),
        }
    }
}

parameter_types! {
    pub const ParachainId: ParaId = ParaId::new(1000);
}

parameter_types! {
    pub const PendingBlocksTtl: u32 = 5;
    pub const CoreBuyingXCMQueryTtl: u32 = 100;
    pub const AdditionalTtlForInflightOrders: u32 = 5;
    pub UniversalLocation: InteriorMultiLocation = X2(GlobalConsensus(NetworkId::Westend), Parachain(1000));
}

impl pallet_xcm_core_buyer::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type XcmSender = DevNull;
    type GetPurchaseCoreCall = EncodedCallToBuyCore;
    type GetBlockNumber = ();
    type GetParathreadAccountId = ParaIdIntoAccountTruncating;
    type GetParathreadMaxCorePrice = ();
    type SelfParaId = ParachainId;
    type RelayChain = ();
    type GetParathreadParams = GetParathreadParamsImpl;
    type GetAssignedCollators = GetAssignedCollatorsImpl;
    type UnsignedPriority = ();
    type PendingBlocksTtl = PendingBlocksTtl;
    type CoreBuyingXCMQueryTtl = CoreBuyingXCMQueryTtl;
    type AdditionalTtlForInflightOrders = AdditionalTtlForInflightOrders;
    type UniversalLocation = UniversalLocation;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type XCMNotifier = ();

    type WeightInfo = ();
}

pub struct DevNull;
impl SendXcm for DevNull {
    type Ticket = ();
    fn validate(_: &mut Option<MultiLocation>, _: &mut Option<Xcm<()>>) -> SendResult<()> {
        Ok(((), MultiAssets::new()))
    }
    fn deliver(_: ()) -> Result<XcmHash, SendError> {
        Ok([0; 32])
    }
}

pub struct GetParathreadParamsImpl;

impl crate::GetParathreadParams for GetParathreadParamsImpl {
    fn get_parathread_params(para_id: ParaId) -> Option<ParathreadParams> {
        MockData::mock().parathread_params.get(&para_id).cloned()
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn set_parathread_params(para_id: ParaId, parathread_params: Option<ParathreadParams>) {
        MockData::mutate(|m| {
            if let Some(parathread_params) = parathread_params {
                m.parathread_params.insert(para_id, parathread_params);
            } else {
                m.parathread_params.remove(&para_id);
            }
        });
    }
}

pub struct GetAssignedCollatorsImpl;

impl GetParathreadCollators<AccountId> for GetAssignedCollatorsImpl {
    fn get_parathread_collators(para_id: ParaId) -> Vec<AccountId> {
        MockData::mock()
            .container_chain_collators
            .get(&para_id)
            .cloned()
            .unwrap_or_default()
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn set_parathread_collators(para_id: ParaId, collators: Vec<AccountId>) {
        MockData::mutate(|m| {
            m.container_chain_collators.insert(para_id, collators);
        })
    }
}

pub struct EncodedCallToBuyCore;

impl GetPurchaseCoreCall<()> for EncodedCallToBuyCore {
    fn get_encoded(_relay_chain: (), _max_amount: u128, _para_id: ParaId) -> Vec<u8> {
        vec![]
    }
}

pub const BUY_EXECUTION_COST: u128 = 50_000_000;
pub const PLACE_ORDER_WEIGHT_AT_MOST: Weight = Weight::from_parts(1_000_000_000, 100_000);

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

        let mut ext: TestExternalities = t.into();

        ext.execute_with(|| {
            assert_ok!(XcmCoreBuyer::set_relay_xcm_weight_config(
                RuntimeOrigin::root(),
                Some(RelayXcmWeightConfigInner {
                    buy_execution_cost: BUY_EXECUTION_COST,
                    weight_at_most: PLACE_ORDER_WEIGHT_AT_MOST,
                    _phantom: PhantomData,
                }),
            ));
        });

        ext
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

#[cfg(feature = "runtime-benchmarks")]
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut ext: TestExternalities = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into();

    ext.execute_with(|| {
        assert_ok!(XcmCoreBuyer::set_relay_xcm_weight_config(
            RuntimeOrigin::root(),
            Some(RelayXcmWeightConfigInner {
                buy_execution_cost: BUY_EXECUTION_COST,
                weight_at_most: PLACE_ORDER_WEIGHT_AT_MOST,
                _phantom: PhantomData,
            }),
        ));
    });

    ext
}

pub fn run_to_block(n: u64) {
    let old_block_number = System::block_number();

    for x in (old_block_number + 1)..=n {
        if x > 0 {
            XcmCoreBuyer::on_finalize(x - 1);
            System::on_finalize(x - 1);
        }
        System::reset_events();
        System::set_block_number(x);
        System::on_initialize(x);
        XcmCoreBuyer::on_initialize(x);
    }
}

pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
