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

//! A staking pallet based on pools of shares.
//!
//! This pallet works with pools inspired by AMM liquidity pools to easily distribute
//! rewards with support for both non-compounding and compounding rewards.
//!
//! Each candidate internally have 3 pools:
//! - a pool for all delegators willing to auto compound.
//! - a pool for all delegators not willing to auto compound.
//! - a pool for all delegators that are in the process of removing stake.
//!
//! When delegating the funds of the delegator are reserved, and shares allow to easily
//! distribute auto compounding rewards (by simply increasing the total shared amount)
//! and easily slash (each share loose part of its value). Rewards are distributed to an account
//! id dedicated to the staking pallet, and delegators can call an extrinsic to transfer their rewards
//! to their own account (but as reserved). Keeping funds reserved in user accounts allow them to
//! participate in other processes such as gouvernance.

#![cfg_attr(not(feature = "std"), no_std)]

mod calls;
mod candidate;
mod pools;
pub mod traits;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(any(feature = "migrations", feature = "try-runtime"))]
pub mod migrations;

pub mod weights;
use frame_support::pallet;
pub use weights::WeightInfo;

pub use {
    candidate::EligibleCandidate,
    pallet::*,
    pools::{ActivePoolKind, CandidateSummary, DelegatorCandidateSummary, PoolKind},
};

#[pallet]
pub mod pallet {
    use {
        super::*,
        crate::{
            traits::{IsCandidateEligible, Timer},
            weights::WeightInfo,
        },
        calls::Calls,
        core::marker::PhantomData,
        frame_support::{
            pallet_prelude::*,
            storage::types::{StorageDoubleMap, StorageValue, ValueQuery},
            traits::{fungible, fungible::Mutate, tokens::Balance, IsType , },
            Blake2_128Concat,
        },
        frame_system::pallet_prelude::*,
        parity_scale_codec::{Decode, Encode, FullCodec},
        scale_info::TypeInfo,
        serde::{Deserialize, Serialize},
        sp_core::Get,
        sp_runtime::{BoundedVec, Perbill},
        sp_std::vec::Vec,
        tp_maths::MulDiv,
        tp_traits::NodeActivityTrackingHelper,
    };

    /// A reason for this pallet placing a hold on funds.
    #[pallet::composite_enum]
    pub enum HoldReason {
        PooledStake,
    }

    // Type aliases for better readability.
    pub type Candidate<T> = <T as frame_system::Config>::AccountId;
    pub type CreditOf<T> =
        fungible::Credit<<T as frame_system::Config>::AccountId, <T as Config>::Currency>;
    pub type Delegator<T> = <T as frame_system::Config>::AccountId;

    pub type SessionIndex = u32;

    /// Key used by the `Pools` StorageDoubleMap, avoiding lots of maps.
    /// StorageDoubleMap first key is the account id of the candidate.
    #[derive(
        RuntimeDebug,
        PartialEq,
        Eq,
        Encode,
        Decode,
        Clone,
        TypeInfo,
        Serialize,
        Deserialize,
        MaxEncodedLen,
    )]
    pub enum PoolsKey<A: FullCodec> {
        /// Total amount of currency backing this candidate across all pools.
        CandidateTotalStake,

        /// Amount of joining shares a delegator have for that candidate.
        JoiningShares { delegator: A },
        /// Total amount of joining shares existing for that candidate.
        JoiningSharesSupply,
        /// Amount of currency backing all the joining shares of that candidate.
        JoiningSharesTotalStaked,
        /// Amount of currency held in the delegator account.
        JoiningSharesHeldStake { delegator: A },

        /// Amount of auto compounding shares a delegator have for that candidate.
        AutoCompoundingShares { delegator: A },
        /// Total amount of auto compounding shares existing for that candidate.
        AutoCompoundingSharesSupply,
        /// Amount of currency backing all the auto compounding shares of that candidate.
        AutoCompoundingSharesTotalStaked,
        /// Amount of currency held in the delegator account.
        AutoCompoundingSharesHeldStake { delegator: A },

        /// Amount of manual rewards shares a delegator have for that candidate.
        ManualRewardsShares { delegator: A },
        /// Total amount of manual rewards shares existing for that candidate.
        ManualRewardsSharesSupply,
        /// Amount of currency backing all the manual rewards shares of that candidate.
        ManualRewardsSharesTotalStaked,
        /// Amount of currency held in the delegator account.
        ManualRewardsSharesHeldStake { delegator: A },
        /// Counter of the cumulated rewards per share generated by that candidate since genesis.
        /// Is safe to wrap around the maximum value of the balance type.
        ManualRewardsCounter,
        /// Value of the counter at the last time the delegator claimed its rewards or changed its amount of shares
        /// (changing the amount of shares automatically claims pending rewards).
        /// The difference between the checkpoint and the counter is the amount of claimable reward per share for
        /// that delegator.
        ManualRewardsCheckpoint { delegator: A },

        /// Amount of shares of that delegator in the leaving pool of that candidate.
        /// When leaving delegating funds are placed in the leaving pool until the leaving period is elapsed.
        /// While in the leaving pool the funds are still slashable.
        LeavingShares { delegator: A },
        /// Total amount of leaving shares existing for that candidate.
        LeavingSharesSupply,
        /// Amount of currency backing all the leaving shares of that candidate.
        LeavingSharesTotalStaked,
        /// Amount of currency held in the delegator account.
        LeavingSharesHeldStake { delegator: A },
    }

    /// Key used by the "PendingOperations" StorageDoubleMap.
    /// StorageDoubleMap first key is the account id of the delegator who made the request.
    /// Value is the amount of shares in the joining/leaving pool.

    #[derive(
        RuntimeDebug,
        PartialEq,
        Eq,
        Encode,
        Decode,
        Clone,
        TypeInfo,
        Serialize,
        Deserialize,
        MaxEncodedLen,
    )]
    pub enum PendingOperationKey<A: FullCodec, J: FullCodec, L: FullCodec> {
        /// Candidate requested to join the auto compounding pool of a candidate.
        JoiningAutoCompounding { candidate: A, at: J },
        /// Candidate requested to join the manual rewards pool of a candidate.
        JoiningManualRewards { candidate: A, at: J },
        /// Candidate requested to to leave a pool of a candidate.
        Leaving { candidate: A, at: L },
    }

    pub type PendingOperationKeyOf<T> = PendingOperationKey<
        <T as frame_system::Config>::AccountId,
        <<T as Config>::JoiningRequestTimer as Timer>::Instant,
        <<T as Config>::LeavingRequestTimer as Timer>::Instant,
    >;

    #[derive(
        RuntimeDebug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo, Serialize, Deserialize,
    )]
    pub struct PendingOperationQuery<A: FullCodec, J: FullCodec, L: FullCodec> {
        pub delegator: A,
        pub operation: PendingOperationKey<A, J, L>,
    }

    pub type PendingOperationQueryOf<T> = PendingOperationQuery<
        <T as frame_system::Config>::AccountId,
        <<T as Config>::JoiningRequestTimer as Timer>::Instant,
        <<T as Config>::LeavingRequestTimer as Timer>::Instant,
    >;

    /// Allow calls to be performed using either share amounts or stake.
    /// When providing stake, calls will convert them into share amounts that are
    /// worth up to the provided stake. The amount of stake thus will be at most the provided
    /// amount.
    #[derive(
        RuntimeDebug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo, Serialize, Deserialize,
    )]
    pub enum SharesOrStake<T> {
        Shares(T),
        Stake(T),
    }

    /// Wrapper type for an amount of shares.
    #[derive(
        RuntimeDebug,
        Default,
        PartialEq,
        Eq,
        Encode,
        Decode,
        Copy,
        Clone,
        TypeInfo,
        Serialize,
        Deserialize,
    )]
    pub struct Shares<T>(pub T);

    /// Wrapper type for an amount of staked currency.
    #[derive(
        RuntimeDebug,
        Default,
        PartialEq,
        Eq,
        Encode,
        Decode,
        Copy,
        Clone,
        TypeInfo,
        Serialize,
        Deserialize,
    )]
    pub struct Stake<T>(pub T);

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    /// Pooled Staking pallet.
    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// The currency type.
        /// Shares will use the same Balance type.
        type Currency: fungible::Inspect<Self::AccountId, Balance = Self::Balance>
            + fungible::Mutate<Self::AccountId>
            + fungible::Balanced<Self::AccountId>
            + fungible::MutateHold<Self::AccountId, Reason = Self::RuntimeHoldReason>;

        /// Same as Currency::Balance. Must impl `MulDiv` which perform
        /// multiplication followed by division using a bigger type to avoid
        /// overflows.
        type Balance: Balance + MulDiv;

        /// Account holding Currency of all delegators.
        #[pallet::constant]
        type StakingAccount: Get<Self::AccountId>;

        /// When creating the first Shares for a candidate the supply can be arbitrary.
        /// Picking a value too low will make an higher supply, which means each share will get
        /// less rewards, and rewards calculations will have more impactful rounding errors.
        /// Picking a value too high is a barrier of entry for staking.
        #[pallet::constant]
        type InitialManualClaimShareValue: Get<Self::Balance>;
        /// When creating the first Shares for a candidate the supply can arbitrary.
        /// Picking a value too high is a barrier of entry for staking, which will increase overtime
        /// as the value of each share will increase due to auto compounding.
        #[pallet::constant]
        type InitialAutoCompoundingShareValue: Get<Self::Balance>;

        /// Minimum amount of stake a Candidate must delegate (stake) towards itself. Not reaching
        /// this minimum prevents from being elected.
        #[pallet::constant]
        type MinimumSelfDelegation: Get<Self::Balance>;
        /// Part of the rewards that will be sent exclusively to the collator.
        #[pallet::constant]
        type RewardsCollatorCommission: Get<Perbill>;

        /// The overarching runtime hold reason.
        type RuntimeHoldReason: From<HoldReason>;

        /// Condition for when a joining request can be executed.
        type JoiningRequestTimer: Timer;
        /// Condition for when a leaving request can be executed.
        type LeavingRequestTimer: Timer;
        /// All eligible candidates are stored in a sorted list that is modified each time
        /// delegations changes. It is safer to bound this list, in which case eligible candidate
        /// could fall out of this list if they have less stake than the top `EligibleCandidatesBufferSize`
        /// eligible candidates. One of this top candidates leaving will then not bring the dropped candidate
        /// in the list. An extrinsic is available to manually bring back such dropped candidate.
        #[pallet::constant]
        type EligibleCandidatesBufferSize: Get<u32>;
        /// Additional filter for candidates to be eligible.
        type EligibleCandidatesFilter: IsCandidateEligible<Self::AccountId>;

        /// Helper for collator activity tracking
        type ActivityTrackingHelper: NodeActivityTrackingHelper<Self::AccountId>;

        type WeightInfo: WeightInfo;
    }

    /// Keeps a list of all eligible candidates, sorted by the amount of stake backing them.
    /// This can be quickly updated using a binary search, and allow to easily take the top
    /// `MaxCollatorSetSize`.
    #[pallet::storage]
    pub type SortedEligibleCandidates<T: Config> = StorageValue<
        _,
        BoundedVec<
            candidate::EligibleCandidate<Candidate<T>, T::Balance>,
            T::EligibleCandidatesBufferSize,
        >,
        ValueQuery,
    >;

    /// Pools balances.
    #[pallet::storage]
    pub type Pools<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        Candidate<T>,
        Blake2_128Concat,
        PoolsKey<T::AccountId>,
        T::Balance,
        ValueQuery,
    >;

    /// Pending operations balances.
    /// Balances are expressed in joining/leaving shares amounts.
    #[pallet::storage]
    pub type PendingOperations<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        Delegator<T>,
        Blake2_128Concat,
        PendingOperationKeyOf<T>,
        T::Balance,
        ValueQuery,
    >;

    /// Summary of a delegator's delegation.
    /// Used to quickly fetch all delegations of a delegator.
    #[pallet::storage]
    pub type DelegatorCandidateSummaries<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        Delegator<T>,
        Blake2_128Concat,
        Candidate<T>,
        DelegatorCandidateSummary,
        ValueQuery,
    >;

    /// Summary of a candidate state.
    #[pallet::storage]
    pub type CandidateSummaries<T: Config> =
        StorageMap<_, Blake2_128Concat, Candidate<T>, CandidateSummary, ValueQuery>;

    /// Pauses the ability to modify pools through extrinsics.
    ///
    /// Currently added only to run the multi-block migration to compute
    /// `DelegatorCandidateSummaries` and `CandidateSummaries`. It will NOT
    /// prevent to distribute rewards, which is fine as the reward distribution
    /// process doesn't alter the pools in a way that will mess with the migration.
    #[pallet::storage]
    pub type PausePoolsExtrinsics<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Stake of the candidate has changed, which may have modified its
        /// position in the eligible candidates list.
        UpdatedCandidatePosition {
            candidate: Candidate<T>,
            stake: T::Balance,
            self_delegation: T::Balance,
            before: Option<u32>,
            after: Option<u32>,
        },

        /// User requested to delegate towards a candidate.
        RequestedDelegate {
            candidate: Candidate<T>,
            delegator: Delegator<T>,
            pool: ActivePoolKind,
            pending: T::Balance,
        },
        /// Delegation request was executed. `staked` has been properly staked
        /// in `pool`, while the rounding when converting to shares has been
        /// `released`.
        ExecutedDelegate {
            candidate: Candidate<T>,
            delegator: Delegator<T>,
            pool: ActivePoolKind,
            staked: T::Balance,
            released: T::Balance,
        },
        /// User requested to undelegate from a candidate.
        /// Stake was removed from a `pool` and is `pending` for the request
        /// to be executed. The rounding when converting to leaving shares has
        /// been `released` immediately.
        RequestedUndelegate {
            candidate: Candidate<T>,
            delegator: Delegator<T>,
            from: ActivePoolKind,
            pending: T::Balance,
            released: T::Balance,
        },
        /// Undelegation request was executed.
        ExecutedUndelegate {
            candidate: Candidate<T>,
            delegator: Delegator<T>,
            released: T::Balance,
        },

        /// Stake of that Candidate increased.
        IncreasedStake {
            candidate: Candidate<T>,
            stake_diff: T::Balance,
        },
        /// Stake of that Candidate decreased.
        DecreasedStake {
            candidate: Candidate<T>,
            stake_diff: T::Balance,
        },
        /// Delegator staked towards a Candidate for AutoCompounding Shares.
        StakedAutoCompounding {
            candidate: Candidate<T>,
            delegator: Delegator<T>,
            shares: T::Balance,
            stake: T::Balance,
        },
        /// Delegator unstaked towards a candidate with AutoCompounding Shares.
        UnstakedAutoCompounding {
            candidate: Candidate<T>,
            delegator: Delegator<T>,
            shares: T::Balance,
            stake: T::Balance,
        },
        /// Delegator staked towards a candidate for ManualRewards Shares.
        StakedManualRewards {
            candidate: Candidate<T>,
            delegator: Delegator<T>,
            shares: T::Balance,
            stake: T::Balance,
        },
        /// Delegator unstaked towards a candidate with ManualRewards Shares.
        UnstakedManualRewards {
            candidate: Candidate<T>,
            delegator: Delegator<T>,
            shares: T::Balance,
            stake: T::Balance,
        },
        /// Collator has been rewarded.
        RewardedCollator {
            collator: Candidate<T>,
            auto_compounding_rewards: T::Balance,
            manual_claim_rewards: T::Balance,
        },
        /// Delegators have been rewarded.
        RewardedDelegators {
            collator: Candidate<T>,
            auto_compounding_rewards: T::Balance,
            manual_claim_rewards: T::Balance,
        },
        /// Rewards manually claimed.
        ClaimedManualRewards {
            candidate: Candidate<T>,
            delegator: Delegator<T>,
            rewards: T::Balance,
        },
        /// Swapped between AutoCompounding and ManualReward shares
        SwappedPool {
            candidate: Candidate<T>,
            delegator: Delegator<T>,
            source_pool: ActivePoolKind,
            source_shares: T::Balance,
            source_stake: T::Balance,
            target_shares: T::Balance,
            target_stake: T::Balance,
            pending_leaving: T::Balance,
            released: T::Balance,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidPalletSetting,
        DisabledFeature,
        NoOneIsStaking,
        StakeMustBeNonZero,
        RewardsMustBeNonZero,
        MathUnderflow,
        MathOverflow,
        NotEnoughShares,
        TryingToLeaveTooSoon,
        InconsistentState,
        UnsufficientSharesForTransfer,
        CandidateTransferingOwnSharesForbidden,
        RequestCannotBeExecuted(u16),
        SwapResultsInZeroShares,
        CollatorDoesNotExist,
        CollatorCannotBeNotifiedAsInactive,
        PoolsExtrinsicsArePaused,
    }

    impl<T: Config> From<tp_maths::OverflowError> for Error<T> {
        fn from(_: tp_maths::OverflowError) -> Self {
            Error::MathOverflow
        }
    }

    impl<T: Config> From<tp_maths::UnderflowError> for Error<T> {
        fn from(_: tp_maths::UnderflowError) -> Self {
            Error::MathUnderflow
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        #[cfg(feature = "try-runtime")]
        fn try_state(_n: BlockNumberFor<T>) -> Result<(), sp_runtime::TryRuntimeError> {
            use frame_support::storage_alias;
            use sp_std::collections::btree_set::BTreeSet;

            let mut all_candidates = BTreeSet::new();
            for (candidate, _k2) in Pools::<T>::iter_keys() {
                all_candidates.insert(candidate);
            }

            for candidate in all_candidates {
                pools::check_candidate_consistency::<T>(&candidate)?;
            }

            // Sorted storage items are sorted
            fn assert_is_sorted_and_unique<T: Ord>(x: &[T], name: &str) {
                assert!(
                    x.windows(2).all(|w| w[0] < w[1]),
                    "sorted list not sorted or not unique: {}",
                    name,
                );
            }
            assert_is_sorted_and_unique(
                &SortedEligibleCandidates::<T>::get(),
                "SortedEligibleCandidates",
            );

            if Pallet::<T>::on_chain_storage_version() < 1 {
                return Ok(());
            }

            // Summaries updated dynamically matches summaries generated entirely from shares.
            #[storage_alias(pallet_name)]
            pub type TryStateDelegatorSummaries<T: Config> = StorageDoubleMap<
                Pallet<T>,
                Blake2_128Concat,
                Delegator<T>,
                Blake2_128Concat,
                Candidate<T>,
                DelegatorCandidateSummary,
                ValueQuery,
            >;

            #[storage_alias(pallet_name)]
            pub type TryStateCandidateSummaries<T: Config> =
                StorageMap<Pallet<T>, Blake2_128Concat, Candidate<T>, CandidateSummary, ValueQuery>;

            assert!(
                migrations::stepped_generate_summaries::<
                    T,
                    TryStateDelegatorSummaries<T>,
                    TryStateCandidateSummaries<T>,
                >(
                    None,
                    &mut frame_support::weights::WeightMeter::new(), // call with no limit on weight
                )
                .expect("to generate summaries without errors")
                .is_none(),
                "failed to generate all summaries"
            );

            let mut candidate_summaries_count = 0;
            for (candidate, summary) in TryStateCandidateSummaries::<T>::iter() {
                candidate_summaries_count += 1;
                assert_eq!(
                    CandidateSummaries::<T>::get(&candidate),
                    summary,
                    "candidate summary for {candidate:?} didn't match"
                );
            }
            assert_eq!(
                candidate_summaries_count,
                CandidateSummaries::<T>::iter().count(),
                "count of candidate summaries didn't match"
            );

            let mut delegator_summaries_count = 0;
            for (delegator, candidate, summary) in TryStateDelegatorSummaries::<T>::iter() {
                delegator_summaries_count += 1;
                assert_eq!(
                    DelegatorCandidateSummaries::<T>::get(&delegator, &candidate),
                    summary,
                    "delegator summary for {delegator:?} to {candidate:?} didn't match"
                );
            }
            assert_eq!(
                delegator_summaries_count,
                DelegatorCandidateSummaries::<T>::iter().count(),
                "count of delegator summaries didn't match"
            );

            Ok(())
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::rebalance_hold())]
        pub fn rebalance_hold(
            origin: OriginFor<T>,
            candidate: Candidate<T>,
            delegator: Delegator<T>,
            pool: PoolKind,
        ) -> DispatchResultWithPostInfo {
            // We don't care about the sender.
            let _ = ensure_signed(origin)?;

            Calls::<T>::rebalance_hold(candidate, delegator, pool)
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::request_delegate())]
        pub fn request_delegate(
            origin: OriginFor<T>,
            candidate: Candidate<T>,
            pool: ActivePoolKind,
            stake: T::Balance,
        ) -> DispatchResultWithPostInfo {
            let delegator = ensure_signed(origin)?;

            Calls::<T>::request_delegate(candidate, delegator, pool, stake)
        }

        /// Execute pending operations can incur in claim manual rewards per operation, we simply add the worst case
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::execute_pending_operations(operations.len() as u32).saturating_add(T::WeightInfo::claim_manual_rewards(operations.len() as u32)))]
        pub fn execute_pending_operations(
            origin: OriginFor<T>,
            operations: Vec<PendingOperationQueryOf<T>>,
        ) -> DispatchResultWithPostInfo {
            // We don't care about the sender.
            let _ = ensure_signed(origin)?;

            Calls::<T>::execute_pending_operations(operations)
        }

        /// Request undelegate can incur in either claim manual rewards or hold rebalances, we simply add the worst case
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::request_undelegate().saturating_add(T::WeightInfo::claim_manual_rewards(1).max(T::WeightInfo::rebalance_hold())))]
        pub fn request_undelegate(
            origin: OriginFor<T>,
            candidate: Candidate<T>,
            pool: ActivePoolKind,
            amount: SharesOrStake<T::Balance>,
        ) -> DispatchResultWithPostInfo {
            let delegator = ensure_signed(origin)?;

            Calls::<T>::request_undelegate(candidate, delegator, pool, amount)
        }

        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::claim_manual_rewards(pairs.len() as u32))]
        pub fn claim_manual_rewards(
            origin: OriginFor<T>,
            pairs: Vec<(Candidate<T>, Delegator<T>)>,
        ) -> DispatchResultWithPostInfo {
            // We don't care about the sender.
            let _ = ensure_signed(origin)?;

            Calls::<T>::claim_manual_rewards(&pairs)
        }

        #[pallet::call_index(5)]
        #[pallet::weight(T::WeightInfo::update_candidate_position(candidates.len() as u32))]
        pub fn update_candidate_position(
            origin: OriginFor<T>,
            candidates: Vec<Candidate<T>>,
        ) -> DispatchResultWithPostInfo {
            // We don't care about the sender.
            let _ = ensure_signed(origin)?;

            Calls::<T>::update_candidate_position(&candidates)
        }

        #[pallet::call_index(6)]
        #[pallet::weight(T::WeightInfo::swap_pool())]
        pub fn swap_pool(
            origin: OriginFor<T>,
            candidate: Candidate<T>,
            source_pool: ActivePoolKind,
            amount: SharesOrStake<T::Balance>,
        ) -> DispatchResultWithPostInfo {
            let delegator = ensure_signed(origin)?;

            Calls::<T>::swap_pool(candidate, delegator, source_pool, amount)
        }

        #[pallet::call_index(7)]
        #[pallet::weight(T::WeightInfo::swap_pool())]
        pub fn notify_inactive_collator(
            origin: OriginFor<T>,
            collator: Candidate<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;
            ensure!(
                T::ActivityTrackingHelper::is_node_inactive(&collator),
                Error::<T>::CollatorCannotBeNotifiedAsInactive
            );
            T::ActivityTrackingHelper::set_offline(&collator)
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn computed_stake(
            candidate: Candidate<T>,
            delegator: Delegator<T>,
            pool: PoolKind,
        ) -> Option<T::Balance> {
            use pools::Pool;
            match pool {
                PoolKind::Joining => pools::Joining::<T>::computed_stake(&candidate, &delegator),
                PoolKind::AutoCompounding => {
                    pools::AutoCompounding::<T>::computed_stake(&candidate, &delegator)
                }
                PoolKind::ManualRewards => {
                    pools::ManualRewards::<T>::computed_stake(&candidate, &delegator)
                }
                PoolKind::Leaving => pools::Leaving::<T>::computed_stake(&candidate, &delegator),
            }
            .ok()
            .map(|x| x.0)
        }
    }

    impl<T: Config> tp_traits::DistributeRewards<Candidate<T>, CreditOf<T>> for Pallet<T> {
        fn distribute_rewards(
            candidate: Candidate<T>,
            rewards: CreditOf<T>,
        ) -> DispatchResultWithPostInfo {
            pools::distribute_rewards::<T>(&candidate, rewards)
        }
    }
    impl<T: Config> tp_traits::NotifyCollatorOnlineStatusChange<Candidate<T>> for Pallet<T> {
        fn is_collator_in_sorted_eligible_candidates(collator: &Candidate<T>) -> bool {
            <SortedEligibleCandidates<T>>::get()
                .into_iter()
                .any(|c| c.candidate == collator.clone())
        }
        fn update_staking_on_online_status_change(
            collator: &Candidate<T>,
        ) -> DispatchResultWithPostInfo {
            Calls::<T>::update_candidate_position(&[collator.clone()])
        }

        #[cfg(feature = "runtime-benchmarks")]
        fn make_collator_eligible_candidate(collator: &Candidate<T>) {
            let minimum_stake = T::MinimumSelfDelegation::get();
            T::Currency::set_balance(collator, minimum_stake + minimum_stake);
            T::EligibleCandidatesFilter::make_candidate_eligible(collator, true);
            Calls::<T>::request_delegate(
                collator.clone(),
                collator.clone(),
                ActivePoolKind::AutoCompounding,
                minimum_stake,
            );
            T::JoiningRequestTimer::skip_to_elapsed();
        }
    }
}
