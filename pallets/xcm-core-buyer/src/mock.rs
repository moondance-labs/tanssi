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

use crate::GetPurchaseCoreCall;
use bounded_collections::ConstU128;
use sp_runtime::traits::Convert;
use sp_runtime::BuildStorage;
use staging_xcm::latest::{
    MultiAssets, MultiLocation, SendError, SendResult, SendXcm, Xcm, XcmHash,
};
use tp_traits::{ParathreadParams, SlotFrequency};
use {
    crate::{self as pallet_xcm_core_buyer},
    dp_core::ParaId,
    frame_support::{
        pallet_prelude::*,
        parameter_types,
        traits::{ConstU64, Everything},
    },
    sp_core::H256,
    sp_runtime::traits::{BlakeTwo256, IdentityLookup},
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
    pub container_chain_collators: BTreeMap<ParaId, Vec<AccountId>>,
}

impl Default for Mocks {
    fn default() -> Self {
        Self {
            container_chain_collators: BTreeMap::from_iter([(ParaId::from(3333), vec![BOB])]),
        }
    }
}

parameter_types! {
    pub const ParachainId: ParaId = ParaId::new(1000);
}

impl pallet_xcm_core_buyer::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type XcmBuyExecutionDot = ConstU128<1_000>;
    type XcmSender = DevNull;
    type GetPurchaseCoreCall = EncodedCallToBuyCore;
    type GetBlockNumber = ();
    type AccountIdToArray32 = AccountIdToArray32;
    type SelfParaId = ParachainId;
    type MaxParathreads = ConstU32<100>;
    type GetParathreadParams = GetParathreadParams;
    type GetAssignedCollators = GetAssignedCollators;
    type UnsignedPriority = ();

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

pub struct GetParathreadParams;

impl Convert<ParaId, Option<ParathreadParams>> for GetParathreadParams {
    fn convert(para_id: ParaId) -> Option<ParathreadParams> {
        if para_id == 3333.into() {
            Some(ParathreadParams {
                slot_frequency: SlotFrequency { min: 10, max: 10 },
            })
        } else {
            None
        }
    }
}

pub struct GetAssignedCollators;

impl Convert<ParaId, Vec<AccountId>> for GetAssignedCollators {
    fn convert(para_id: ParaId) -> Vec<AccountId> {
        MockData::mock()
            .container_chain_collators
            .get(&para_id)
            .cloned()
            .unwrap_or_default()
    }
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

pub struct EncodedCallToBuyCore;

impl GetPurchaseCoreCall for EncodedCallToBuyCore {
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

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
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
