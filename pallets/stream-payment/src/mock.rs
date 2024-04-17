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
    crate as pallet_stream_payment,
    frame_support::{
        parameter_types,
        traits::{
            tokens::{
                fungible::{InspectHold, Mutate, MutateHold},
                Precision, Preservation,
            },
            Everything, OnFinalize, OnInitialize,
        },
    },
    parity_scale_codec::{Decode, Encode, MaxEncodedLen},
    scale_info::TypeInfo,
    sp_core::{ConstU32, ConstU64, RuntimeDebug, H256},
    sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    },
};

type Block = frame_system::mocking::MockBlock<Runtime>;
pub type AccountId = u64;
pub type Balance = u128;

pub const ALICE: u64 = 0;
pub const BOB: u64 = 1;
pub const CHARLIE: u64 = 2;

pub const KILO: u128 = 1000;
pub const MEGA: u128 = 1000 * KILO;
pub const GIGA: u128 = 1000 * MEGA;
pub const TERA: u128 = 1000 * GIGA;
pub const PETA: u128 = 1000 * TERA;
pub const DEFAULT_BALANCE: u128 = PETA;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Runtime
    {
        System: frame_system,
        Balances: pallet_balances,
        StreamPayment: pallet_stream_payment,
    }
);

impl frame_system::Config for Runtime {
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
    pub ExistentialDeposit: u128 = 1;
}

impl pallet_balances::Config for Runtime {
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 4];
    type MaxLocks = ();
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type FreezeIdentifier = RuntimeFreezeReason;
    type MaxFreezes = ConstU32<1>;
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type MaxHolds = ConstU32<5>;
    type WeightInfo = ();
}

#[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Copy, Clone, TypeInfo, MaxEncodedLen)]
pub enum StreamPaymentAssetId {
    Native,
    Dummy,
}

pub struct StreamPaymentAssets;
impl pallet_stream_payment::Assets<AccountId, StreamPaymentAssetId, Balance>
    for StreamPaymentAssets
{
    fn transfer_deposit(
        asset_id: &StreamPaymentAssetId,
        from: &AccountId,
        to: &AccountId,
        amount: Balance,
    ) -> frame_support::pallet_prelude::DispatchResult {
        Self::decrease_deposit(asset_id, from, amount)?;
        match asset_id {
            StreamPaymentAssetId::Native => {
                Balances::transfer(from, to, amount, Preservation::Preserve).map(|_| ())
            }
            StreamPaymentAssetId::Dummy => Ok(()),
        }
    }

    fn increase_deposit(
        asset_id: &StreamPaymentAssetId,
        account: &AccountId,
        amount: Balance,
    ) -> frame_support::pallet_prelude::DispatchResult {
        match asset_id {
            StreamPaymentAssetId::Native => Balances::hold(
                &pallet_stream_payment::HoldReason::StreamPayment.into(),
                account,
                amount,
            ),
            StreamPaymentAssetId::Dummy => Ok(()),
        }
    }

    fn decrease_deposit(
        asset_id: &StreamPaymentAssetId,
        account: &AccountId,
        amount: Balance,
    ) -> frame_support::pallet_prelude::DispatchResult {
        match asset_id {
            StreamPaymentAssetId::Native => Balances::release(
                &pallet_stream_payment::HoldReason::StreamPayment.into(),
                account,
                amount,
                Precision::Exact,
            )
            .map(|_| ()),
            StreamPaymentAssetId::Dummy => Ok(()),
        }
    }

    fn get_deposit(asset_id: &StreamPaymentAssetId, account: &AccountId) -> Balance {
        match asset_id {
            StreamPaymentAssetId::Native => Balances::balance_on_hold(
                &pallet_stream_payment::HoldReason::StreamPayment.into(),
                account,
            ),
            StreamPaymentAssetId::Dummy => 0,
        }
    }

    /// Benchmarks: should return the asset id which has the worst performance when interacting
    /// with it.
    #[cfg(feature = "runtime-benchmarks")]
    fn bench_worst_case_asset_id() -> StreamPaymentAssetId {
        StreamPaymentAssetId::Native
    }

    /// Benchmarks: should return the another asset id which has the worst performance when interacting
    /// with it afther `bench_worst_case_asset_id`. This is to benchmark the worst case when changing config
    /// from one asset to another.
    #[cfg(feature = "runtime-benchmarks")]
    fn bench_worst_case_asset_id2() -> StreamPaymentAssetId {
        StreamPaymentAssetId::Native
    }

    /// Benchmarks: should set the balance for the asset id returned by `bench_worst_case_asset_id`.
    #[cfg(feature = "runtime-benchmarks")]
    fn bench_set_balance(asset_id: &StreamPaymentAssetId, account: &AccountId, amount: Balance) {
        match asset_id {
            StreamPaymentAssetId::Native => {
                Balances::set_balance(account, amount);
            }
            StreamPaymentAssetId::Dummy => {}
        }
    }
}

#[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Copy, Clone, TypeInfo, MaxEncodedLen)]
pub enum TimeUnit {
    BlockNumber,
    Timestamp,
    Never,
    Decreasing,
}

pub struct TimeProvider;
impl pallet_stream_payment::TimeProvider<TimeUnit, Balance> for TimeProvider {
    fn now(unit: &TimeUnit) -> Option<Balance> {
        match *unit {
            TimeUnit::BlockNumber => Some(System::block_number().into()),
            TimeUnit::Timestamp => Some((System::block_number() * 12).into()),
            TimeUnit::Never => None,
            TimeUnit::Decreasing => Some((u64::MAX - System::block_number()).into()),
        }
    }

    /// Benchmarks: should return the time unit which has the worst performance calling
    /// `TimeProvider::now(unit)` with.
    #[cfg(feature = "runtime-benchmarks")]
    fn bench_worst_case_time_unit() -> TimeUnit {
        TimeUnit::BlockNumber
    }

    /// Benchmarks: sets the "now" time for time unit returned by `worst_case_time_unit`.
    #[cfg(feature = "runtime-benchmarks")]
    fn bench_set_now(instant: u128) {
        System::set_block_number(instant as u64)
    }
}

parameter_types! {
    pub const OpenStreamHoldAmount: Balance = 100;
}

impl pallet_stream_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type StreamId = u64;
    type TimeUnit = TimeUnit;
    type Balance = Balance;
    type AssetId = StreamPaymentAssetId;
    type Assets = StreamPaymentAssets;
    type Currency = Balances;
    type OpenStreamHoldAmount = OpenStreamHoldAmount;
    type RuntimeHoldReason = RuntimeHoldReason;
    type TimeProvider = TimeProvider;
    type WeightInfo = ();
}

pub(crate) struct ExtBuilder {
    // endowed accounts with balances
    balances: Vec<(AccountId, Balance)>,
}

impl Default for ExtBuilder {
    fn default() -> ExtBuilder {
        ExtBuilder {
            balances: vec![
                (ALICE, 1 * DEFAULT_BALANCE),
                (BOB, 1 * DEFAULT_BALANCE),
                (CHARLIE, 1 * DEFAULT_BALANCE),
            ],
        }
    }
}

impl ExtBuilder {
    #[allow(dead_code)]
    pub(crate) fn with_balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
        self.balances = balances;
        self
    }

    pub(crate) fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::<Runtime>::default()
            .build_storage()
            .expect("Frame system builds valid default genesis config");

        pallet_balances::GenesisConfig::<Runtime> {
            balances: self.balances,
        }
        .assimilate_storage(&mut t)
        .expect("Pallet balances storage can be assimilated");

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

/// Rolls forward one block. Returns the new block number.
#[allow(dead_code)]
pub(crate) fn roll_one_block() -> u64 {
    Balances::on_finalize(System::block_number());
    System::on_finalize(System::block_number());
    System::set_block_number(System::block_number() + 1);
    System::on_initialize(System::block_number());
    Balances::on_initialize(System::block_number());
    System::block_number()
}

/// Rolls to the desired block. Returns the number of blocks played.
#[allow(dead_code)]
pub(crate) fn roll_to(n: u64) -> u64 {
    let mut num_blocks = 0;
    let mut block = System::block_number();
    while block < n {
        block = roll_one_block();
        num_blocks += 1;
    }
    num_blocks
}

#[allow(dead_code)]
pub(crate) fn last_event() -> RuntimeEvent {
    System::events().pop().expect("Event expected").event
}

#[allow(dead_code)]
pub(crate) fn events() -> Vec<crate::Event<Runtime>> {
    System::events()
        .into_iter()
        .map(|r| r.event)
        .filter_map(|e| {
            if let RuntimeEvent::StreamPayment(inner) = e {
                Some(inner)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

/// Assert input equal to the last event emitted
#[macro_export]
macro_rules! assert_last_event {
    ($event:expr) => {
        match &$event {
            e => assert_eq!(*e, $crate::mock::last_event()),
        }
    };
}

/// Compares the system events with passed in events
/// Prints highlighted diff iff assert_eq fails
#[macro_export]
macro_rules! assert_eq_events {
    ($events:expr) => {
        match &$events {
            e => similar_asserts::assert_eq!(*e, $crate::mock::events()),
        }
    };
}

/// Compares the last N system events with passed in events, where N is the length of events passed
/// in.
///
/// Prints highlighted diff iff assert_eq fails.
/// The last events from frame_system will be taken in order to match the number passed to this
/// macro. If there are insufficient events from frame_system, they will still be compared; the
/// output may or may not be helpful.
///
/// Examples:
/// If frame_system has events [A, B, C, D, E] and events [C, D, E] are passed in, the result would
/// be a successful match ([C, D, E] == [C, D, E]).
///
/// If frame_system has events [A, B, C, D] and events [B, C] are passed in, the result would be an
/// error and a hopefully-useful diff will be printed between [C, D] and [B, C].
///
/// Note that events are filtered to only match parachain-staking (see events()).
#[macro_export]
macro_rules! assert_eq_last_events {
    ($events:expr) => {
        $crate::assert_tail_eq!($events, $crate::mock::events())
    };
}

/// Assert that one array is equal to the tail of the other. A more generic and testable version of
/// assert_eq_last_events.
#[macro_export]
macro_rules! assert_tail_eq {
    ($tail:expr, $arr:expr) => {
        if $tail.len() != 0 {
            // 0-length always passes

            if $tail.len() > $arr.len() {
                similar_asserts::assert_eq!($tail, $arr); // will fail
            }

            let len_diff = $arr.len() - $tail.len();
            similar_asserts::assert_eq!($tail, $arr[len_diff..]);
        }
    };
}

/// Panics if an event is not found in the system log of events
#[macro_export]
macro_rules! assert_event_emitted {
    ($event:expr) => {
        match &$event.into() {
            e => {
                assert!(
                    $crate::mock::events().iter().find(|x| *x == e).is_some(),
                    "Event {:#?} was not found in events: \n {:#?}",
                    e,
                    $crate::mock::events()
                );
            }
        }
    };
}

/// Panics if an event is found in the system log of events
#[macro_export]
macro_rules! assert_event_not_emitted {
    ($event:expr) => {
        match &$event.into() {
            e => {
                assert!(
                    $crate::mock::events().iter().find(|x| *x == e).is_none(),
                    "Event {:#?} was found in events: \n {:#?}",
                    e,
                    $crate::mock::events()
                );
            }
        }
    };
}

#[macro_export]
macro_rules! assert_fields_eq {
    ($left:expr, $right:expr, $field:ident) => {
        assert_eq!($left.$field, $right.$field);
    };
    ($left:expr, $right:expr, [$( $field:ident ),+ $(,)?] ) => {
        $(
            assert_eq!($left.$field, $right.$field);
        )+
    };
}

#[test]
fn assert_tail_eq_works() {
    assert_tail_eq!(vec![1, 2], vec![0, 1, 2]);

    assert_tail_eq!(vec![1], vec![1]);

    assert_tail_eq!(
        vec![0u32; 0], // 0 length array
        vec![0u32; 1]  // 1-length array
    );

    assert_tail_eq!(vec![0u32, 0], vec![0u32, 0]);
}

#[test]
#[should_panic]
fn assert_tail_eq_panics_on_non_equal_tail() {
    assert_tail_eq!(vec![2, 2], vec![0, 1, 2]);
}

#[test]
#[should_panic]
fn assert_tail_eq_panics_on_empty_arr() {
    assert_tail_eq!(vec![2, 2], vec![0u32; 0]);
}

#[test]
#[should_panic]
fn assert_tail_eq_panics_on_longer_tail() {
    assert_tail_eq!(vec![1, 2, 3], vec![1, 2]);
}

#[test]
#[should_panic]
fn assert_tail_eq_panics_on_unequal_elements_same_length_array() {
    assert_tail_eq!(vec![1, 2, 3], vec![0, 1, 2]);
}
