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
        self as pallet_pooled_staking,
        candidate::Candidates,
        pools::Pool,
        traits::{BlockNumberTimer, Timer},
        Candidate, Delegator, PendingOperationKey, PendingOperationKeyOf, TargetPool,
    },
    frame_support::{
        parameter_types,
        traits::{
            tokens::fungible::{Inspect, InspectHold},
            Everything, OnFinalize, OnInitialize,
        },
    },
    frame_system::pallet_prelude::BlockNumberFor,
    num_traits::Num,
    parity_scale_codec::{Decode, Encode, MaxEncodedLen},
    scale_info::TypeInfo,
    sp_core::{ConstU32, ConstU64, RuntimeDebug, H256},
    sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage, Perbill,
    },
};

#[derive(
    RuntimeDebug,
    PartialEq,
    Eq,
    Encode,
    Decode,
    Copy,
    Clone,
    TypeInfo,
    PartialOrd,
    Ord,
    MaxEncodedLen,
)]
pub enum HoldIdentifier {
    Staking,
}

type Block = frame_system::mocking::MockBlock<Runtime>;
pub type AccountId = u64;
pub type Balance = u128;

pub const ACCOUNT_STAKING: u64 = 0;
pub const ACCOUNT_CANDIDATE_1: u64 = 1;
pub const ACCOUNT_CANDIDATE_2: u64 = 2;
pub const ACCOUNT_DELEGATOR_1: u64 = 3;
pub const ACCOUNT_DELEGATOR_2: u64 = 4;

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
        Staking: pallet_pooled_staking,
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

/// Allows to change ED mid-test.
pub struct MockExistentialDeposit;
impl MockExistentialDeposit {
    pub fn get() -> Balance {
        frame_support::storage::unhashed::get(b":mock_ed").unwrap_or(1)
    }

    pub fn set(amount: Balance) {
        frame_support::storage::unhashed::put(b":mock_ed", &amount);
    }
}

parameter_types! {
    pub ExistentialDeposit: u128 = MockExistentialDeposit::get();
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
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type MaxHolds = ConstU32<5>;
    type WeightInfo = ();
}

pub const SHARE_INIT: u128 = MEGA;
pub const BLOCKS_TO_WAIT: u64 = 2;

parameter_types! {
    pub const StakingAccount: u64 = ACCOUNT_STAKING;
    pub const InitialManualClaimShareValue: u128 = SHARE_INIT;
    pub const InitialAutoCompoundingShareValue: u128 = SHARE_INIT;
    pub const MinimumSelfDelegation: u128 = 10 * MEGA;
    pub const RewardsCollatorCommission: Perbill = Perbill::from_percent(20);
    pub const BlocksToWait: u64 = BLOCKS_TO_WAIT;
}

impl pallet_pooled_staking::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Balance = Balance;
    type StakingAccount = StakingAccount;
    type InitialManualClaimShareValue = InitialManualClaimShareValue;
    type InitialAutoCompoundingShareValue = InitialAutoCompoundingShareValue;
    type MinimumSelfDelegation = MinimumSelfDelegation;
    type RewardsCollatorCommission = RewardsCollatorCommission;
    type JoiningRequestTimer = BlockNumberTimer<Self, BlocksToWait>;
    type LeavingRequestTimer = BlockNumberTimer<Self, BlocksToWait>;
    // low value so we can test vec bounding, in practice it should be bigger
    type EligibleCandidatesBufferSize = ConstU32<3>;
    type EligibleCandidatesFilter = ();
    type WeightInfo = ();
    type RuntimeHoldReason = RuntimeHoldReason;
}

pub trait PoolExt<T: crate::Config>: Pool<T> {
    type OppositePool: PoolExt<T>;

    fn target_pool() -> TargetPool;
    fn event_staked(
        candidate: Candidate<T>,
        delegator: Delegator<T>,
        shares: T::Balance,
        stake: T::Balance,
    ) -> crate::Event<T>;
    fn joining_operation_key(
        candidate: Candidate<T>,
        at: <T::JoiningRequestTimer as Timer>::Instant,
    ) -> PendingOperationKeyOf<T>;
}

impl<T: crate::Config> PoolExt<T> for crate::pools::ManualRewards<T> {
    type OppositePool = crate::pools::AutoCompounding<T>;

    fn target_pool() -> TargetPool {
        TargetPool::ManualRewards
    }

    fn event_staked(
        candidate: Candidate<T>,
        delegator: Delegator<T>,
        shares: T::Balance,
        stake: T::Balance,
    ) -> crate::Event<T> {
        crate::Event::StakedManualRewards {
            candidate,
            delegator,
            shares,
            stake,
        }
    }

    fn joining_operation_key(
        candidate: Candidate<T>,
        at: <T::JoiningRequestTimer as Timer>::Instant,
    ) -> PendingOperationKeyOf<T> {
        PendingOperationKey::JoiningManualRewards { candidate, at }
    }
}

impl<T: crate::Config> PoolExt<T> for crate::pools::AutoCompounding<T> {
    type OppositePool = crate::pools::ManualRewards<T>;

    fn target_pool() -> TargetPool {
        TargetPool::AutoCompounding
    }
    fn event_staked(
        candidate: Candidate<T>,
        delegator: Delegator<T>,
        shares: T::Balance,
        stake: T::Balance,
    ) -> crate::Event<T> {
        crate::Event::StakedAutoCompounding {
            candidate,
            delegator,
            shares,
            stake,
        }
    }

    fn joining_operation_key(
        candidate: Candidate<T>,
        at: <T::JoiningRequestTimer as Timer>::Instant,
    ) -> PendingOperationKeyOf<T> {
        PendingOperationKey::JoiningAutoCompounding { candidate, at }
    }
}

#[macro_export]
macro_rules! pool_test {
    (fn $name:ident<$pool:ident>() { $body:expr }) => {
        mod $name {
            use super::*;
            fn generic<$pool: PoolExt<Runtime>>() {
                $body
            }

            #[test]
            fn manual() {
                generic::<pools::ManualRewards<Runtime>>();
            }

            #[test]
            fn auto() {
                generic::<pools::AutoCompounding<Runtime>>();
            }
        }
    };
}

pub fn total_balance(who: &AccountId) -> Balance {
    Balances::total_balance(who)
}

pub fn balance_hold(who: &AccountId) -> Balance {
    Balances::balance_on_hold(&crate::HoldReason::PooledStake.into(), who)
}

pub fn block_number() -> BlockNumberFor<Runtime> {
    System::block_number()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct State {
    pub delegator_balance: Balance,
    pub delegator_hold: Balance,
    pub staking_balance: Balance,
    pub candidate_total_stake: Balance,
}

impl State {
    pub fn extract(candidate: AccountId, delegator: AccountId) -> Self {
        Self {
            delegator_balance: total_balance(&delegator),
            delegator_hold: balance_hold(&delegator),
            staking_balance: total_balance(&ACCOUNT_STAKING),
            candidate_total_stake: Candidates::<Runtime>::total_stake(&candidate).0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PoolState {
    pub hold: Balance,
    pub stake: Balance,
}

impl PoolState {
    pub fn extract<P: Pool<Runtime>>(candidate: AccountId, delegator: AccountId) -> Self {
        Self {
            hold: P::hold(&candidate, &delegator).0,
            stake: P::computed_stake(&candidate, &delegator)
                .expect("invalid state")
                .0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SignedBalance {
    Positive(Balance),
    Negative(Balance),
}

#[allow(dead_code)]
pub fn round_down<T: Num + Copy>(value: T, increment: T) -> T {
    if (value % increment).is_zero() {
        value
    } else {
        (value / increment) * increment
    }
}

pub(crate) struct ExtBuilder {
    // endowed accounts with balances
    balances: Vec<(AccountId, Balance)>,
}

impl Default for ExtBuilder {
    fn default() -> ExtBuilder {
        ExtBuilder {
            balances: vec![
                (ACCOUNT_STAKING, 1 * DEFAULT_BALANCE),
                (ACCOUNT_CANDIDATE_1, 1 * DEFAULT_BALANCE),
                (ACCOUNT_CANDIDATE_2, 1 * DEFAULT_BALANCE),
                (ACCOUNT_DELEGATOR_1, 1 * DEFAULT_BALANCE),
                (ACCOUNT_DELEGATOR_2, 1 * DEFAULT_BALANCE),
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
    // Staking::on_finalize(System::block_number());
    Balances::on_finalize(System::block_number());
    System::on_finalize(System::block_number());
    System::set_block_number(System::block_number() + 1);
    System::on_initialize(System::block_number());
    Balances::on_initialize(System::block_number());
    // Staking::on_initialize(System::block_number());
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
            if let RuntimeEvent::Staking(inner) = e {
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
        match &$event {
            e => {
                assert!(
                    $crate::mock::events().iter().find(|x| *x == e).is_some(),
                    "Event {:?} was not found in events: \n {:?}",
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
        match &$event {
            e => {
                assert!(
                    $crate::mock::events().iter().find(|x| *x == e).is_none(),
                    "Event {:?} was found in events: \n {:?}",
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
