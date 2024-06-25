#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

use frame_support::sp_runtime::traits::Zero;

use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement::KeepAlive, Get, ReservableCurrency},
};

use frame_support::traits::UnixTime;

use frame_support::sp_runtime::Saturating;

use sp_std::prelude::*;

use frame_support::sp_runtime::SaturatedConversion;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReasons};

	pub type StakingIndex = u64;
	pub type QueueIndex = u64;

	use frame_system::{ensure_signed, pallet_prelude::*};

	const EXAMPLE_ID: LockIdentifier = *b"stkxcavc";

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct LedgerAccount<Balance, T: Config> {
		/// Staker AccountId
		pub staker: AccountIdOf<T>,
		/// Balance locked
		pub locked: Balance,
		/// Timestamp locked
		pub timestamp: u64,
		/// Earned Rewards
		pub earned_rewards: Balance,
	}

	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct QueueLedgerAccount<Balance, T: Config> {
		/// Staker AccountId
		pub staker: AccountIdOf<T>,
		/// Balance locked
		pub locked: Balance,
		/// Timestamp locked
		pub timestamp: u64,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_community_loan_pool::Config + pallet_xcavate_whitelist::Config
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet.
		type WeightInfo: WeightInfo;
		/// The lockable currency type.
		type Currency: Currency<Self::AccountId>
			+ LockableCurrency<Self::AccountId, Moment = BlockNumberFor<Self>>
			+ ReservableCurrency<Self::AccountId>;
		/// Minimum amount that should be left on staker account after staking.
		/// Serves as a safeguard to prevent users from locking their entire free balance.
		#[pallet::constant]
		type MinimumRemainingAmount: Get<BalanceOf<Self>>;
		/// The maximum amount of loans that can run at the same time.
		#[pallet::constant]
		type MaxStakers: Get<u32>;
		/// Lose coupling of pallet timestamp.
		type TimeProvider: UnixTime;
		/// Frequence in which the staking rewards are distributed
		type RewardsDistributingTime: Get<BlockNumberFor<Self>>;
	}

	/// Number of stakings that has been made.
	#[pallet::storage]
	#[pallet::getter(fn staking_count)]
	pub(super) type StakingCount<T> = StorageValue<_, StakingIndex, ValueQuery>;

	/// Number of queues.
	#[pallet::storage]
	#[pallet::getter(fn queue_count)]
	pub(super) type QueueCount<T> = StorageValue<_, QueueIndex, ValueQuery>;

	/// Mapping of the account to the staking info.
	#[pallet::storage]
	#[pallet::getter(fn ledger)]
	pub type Ledger<T: Config> =
		StorageMap<_, Blake2_128Concat, StakingIndex, LedgerAccount<BalanceOf<T>, T>, OptionQuery>;

	/// Mapping of the account to the queue info.
	#[pallet::storage]
	#[pallet::getter(fn queue_ledger)]
	pub type QueueLedger<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		QueueIndex,
		QueueLedgerAccount<BalanceOf<T>, T>,
		OptionQuery,
	>;

	/// All current stakers.
	#[pallet::storage]
	#[pallet::getter(fn active_stakings)]
	pub type ActiveStakings<T: Config> =
		StorageValue<_, BoundedVec<StakingIndex, T::MaxStakers>, ValueQuery>;

	/// All current users waiting in the queue.
	#[pallet::storage]
	#[pallet::getter(fn queue_staking)]
	pub type QueueStaking<T: Config> =
		StorageValue<_, BoundedVec<QueueIndex, T::MaxStakers>, ValueQuery>;

	/// The total staked amount.
	#[pallet::storage]
	#[pallet::getter(fn total_stake)]
	pub(super) type TotalStake<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// Mapping of account id to the amount locked
	#[pallet::storage]
	#[pallet::getter(fn amount_locked)]
	pub type AmountLocked<T: Config> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, BalanceOf<T>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Balance was locked successfully.
		Locked { staker: <T as frame_system::Config>::AccountId, amount: BalanceOf<T> },
		/// Balance was unlocked successfully.
		Unlocked { staker: <T as frame_system::Config>::AccountId, amount: BalanceOf<T> },
		/// Rewards were claimed successfully.
		RewardsClaimed { amount: BalanceOf<T>, apy: u128 },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Can not stake with zero value.
		StakingWithNoValue,
		/// Unstaking a contract with zero value
		UnstakingWithNoValue,
		/// The locked period didn't end yet
		UnlockPeriodNotReached,
		/// No staked amount
		NoStakedAmount,
		/// Too many stakers
		TooManyStakers,
		/// The staker does not exist.
		NoStaker,
		/// The Ledger does not exist.
		LedgerNotFound,
		/// Error by convertion to balance type.
		ConversionError,
		/// The loan does not exist.
		NoLoanFound,
		/// Index is already used.
		IndexInUse,
		/// The index has not been found in the queue.
		NotInQueue,
		/// Caller is not the staker.
		CallerNotStaker,
		/// The Staker has nothing locked.
		StakerNothingLocked,
		/// User has not passed the kyc.
		UserNotWhitelisted,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: frame_system::pallet_prelude::BlockNumberFor<T>) -> Weight {
			let weight = T::DbWeight::get().reads_writes(1, 1);
			let block = n.saturated_into::<u64>();
			let time_frame = T::RewardsDistributingTime::get().saturated_into::<u64>();
			if block % time_frame == 0 {
				let _ = Self::claim_rewards();
				weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
				let _ = Self::check_relation_to_loan();
				weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
			}

			weight
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::stake())]

		/// Lets the user stake.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `value`: The amount of token that the user wants to stake
		///
		/// Emits `Locked` event when succesfful
		pub fn stake(
			origin: OriginFor<T>,
			#[pallet::compact] value: BalanceOf<T>,
		) -> DispatchResult {
			let staker = ensure_signed(origin)?;

			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(staker.clone()),
				Error::<T>::UserNotWhitelisted
			);

			ensure!(!value.is_zero(), Error::<T>::StakingWithNoValue);

			let total_amount_loan =
				pallet_community_loan_pool::Pallet::<T>::total_loan_amount() as u128;
			let total_stake = Self::balance_to_u128(Self::total_stake())?;
			if total_stake >= total_amount_loan {
				let available_balance = <T as pallet::Config>::Currency::free_balance(&staker)
					.saturating_sub(T::MinimumRemainingAmount::get());
				let value_to_queue = value.min(available_balance);
				Self::queue_helper(staker.clone(), value_to_queue)?;
			} else if total_stake + Self::balance_to_u128(value)? >= total_amount_loan {
				let available_balance = <T as pallet::Config>::Currency::free_balance(&staker)
					.saturating_sub(T::MinimumRemainingAmount::get());
				let value_available = value.min(available_balance);
				let staking_value = total_amount_loan - total_stake;
				let queue_value = Self::balance_to_u128(value_available)? - staking_value;
				Self::queue_helper(staker.clone(), Self::u128_to_balance_option(queue_value)?)?;
				Self::stake_helper(staker.clone(), Self::u128_to_balance_option(staking_value)?)?;
			} else {
				let available_balance = <T as pallet::Config>::Currency::free_balance(&staker)
					.saturating_sub(T::MinimumRemainingAmount::get());

				let value_to_stake = value.min(available_balance);

				Self::stake_helper(staker.clone(), value_to_stake)?;
			}
			Self::deposit_event(Event::Locked { staker, amount: value });
			Ok(())
		}

		/// Lets the user unstake.
		///
		/// The origin must be a staker, signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `value`: The amount of token that the user wants to unstake
		///
		/// Emits `Unlocked` event when succesfful
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::unstake())]
		pub fn unstake(
			origin: OriginFor<T>,
			staking_index: StakingIndex,
			#[pallet::compact] value: BalanceOf<T>,
		) -> DispatchResult {
			let staker = ensure_signed(origin)?;

			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(staker.clone()),
				Error::<T>::UserNotWhitelisted
			);

			ensure!(Self::ledger(staking_index).is_some(), Error::<T>::NoStaker);

			let mut ledger = Self::ledger(staking_index).ok_or(Error::<T>::LedgerNotFound)?;
			ensure!(ledger.staker == staker, Error::<T>::CallerNotStaker);

			ledger.locked = ledger.locked.saturating_sub(value);

			let locked_amount = Self::amount_locked(ledger.staker.clone())
				.ok_or(Error::<T>::StakerNothingLocked)?;
			let new_locked_amount = locked_amount.saturating_sub(value);
			if new_locked_amount.is_zero() {
				<T as pallet::Config>::Currency::remove_lock(EXAMPLE_ID, &ledger.staker);
			} else {
				let locking_amount = Self::balance_to_u128(new_locked_amount)? * 1000000000000;
				if Self::u128_to_balance_option(locking_amount).is_ok() {
					<T as pallet::Config>::Currency::set_lock(
						EXAMPLE_ID,
						&ledger.staker,
						Self::u128_to_balance_option(locking_amount)?,
						WithdrawReasons::all(),
					);
				}
			}

			if ledger.locked.is_zero() {
				let mut active_stakings = Self::active_stakings();
				let index = active_stakings
					.iter()
					.position(|x| *x == staking_index)
					.ok_or(Error::<T>::NoStaker)?;
				active_stakings.remove(index);
				Ledger::<T>::remove(staking_index);
				ActiveStakings::<T>::put(active_stakings);
			} else {
				Ledger::<T>::insert(staking_index, ledger.clone());
			}
			AmountLocked::<T>::insert(ledger.staker, new_locked_amount);
			let total_stake = Self::total_stake();
			TotalStake::<T>::put(total_stake - value);

			Self::deposit_event(Event::Unlocked { staker, amount: value });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::withdraw_from_queue())]
		pub fn withdraw_from_queue(
			origin: OriginFor<T>,
			queue_index: QueueIndex,
			#[pallet::compact] value: BalanceOf<T>,
		) -> DispatchResult {
			let staker = ensure_signed(origin)?;

			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(staker.clone()),
				Error::<T>::UserNotWhitelisted
			);

			ensure!(Self::queue_ledger(queue_index).is_some(), Error::<T>::NotInQueue);

			let mut queue_ledger =
				Self::queue_ledger(queue_index).ok_or(Error::<T>::LedgerNotFound)?;
			ensure!(queue_ledger.staker == staker, Error::<T>::CallerNotStaker);

			queue_ledger.locked = queue_ledger.locked.saturating_sub(value);

			let locked_amount = Self::amount_locked(queue_ledger.staker.clone())
				.ok_or(Error::<T>::StakerNothingLocked)?;
			let new_locked_amount = locked_amount.saturating_sub(value);
			if new_locked_amount.is_zero() {
				QueueLedger::<T>::remove(queue_index);
				<T as pallet::Config>::Currency::remove_lock(EXAMPLE_ID, &queue_ledger.staker);
			} else {
				let locking_amount = Self::balance_to_u128(new_locked_amount)? * 1000000000000;
				if Self::u128_to_balance_option(locking_amount).is_ok() {
					<T as pallet::Config>::Currency::set_lock(
						EXAMPLE_ID,
						&queue_ledger.staker,
						Self::u128_to_balance_option(locking_amount)?,
						WithdrawReasons::all(),
					);
				}
			}

			if queue_ledger.locked.is_zero() {
				let mut queue_staking = Self::queue_staking();
				let index = queue_staking
					.iter()
					.position(|x| *x == queue_index)
					.ok_or(Error::<T>::NoStaker)?;
				queue_staking.remove(index);
				QueueLedger::<T>::remove(queue_index);
				QueueStaking::<T>::put(queue_staking);
			} else {
				QueueLedger::<T>::insert(queue_index, queue_ledger.clone());
			}
			AmountLocked::<T>::insert(queue_ledger.staker.clone(), new_locked_amount);

			Self::deposit_event(Event::Unlocked { staker, amount: value });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn stake_helper(staker: T::AccountId, value: BalanceOf<T>) -> DispatchResult {
			let staking_index = Self::staking_count() + 1;
			ensure!(Self::ledger(staking_index).is_none(), Error::<T>::IndexInUse);

			let timestamp = <T as pallet::Config>::TimeProvider::now().as_secs();

			let ledger = LedgerAccount {
				staker: staker.clone(),
				locked: value,
				timestamp,
				earned_rewards: Default::default(),
			};

			let locked_amount = if Self::amount_locked(staker.clone()).is_none() {
				Default::default()
			} else {
				Self::amount_locked(staker).ok_or(Error::<T>::StakerNothingLocked)?
			};
			let new_locked_amount = locked_amount.saturating_add(value);
			if new_locked_amount.is_zero() {
				Ledger::<T>::remove(staking_index);
				<T as pallet::Config>::Currency::remove_lock(EXAMPLE_ID, &ledger.staker);
			} else {
				let locking_amount = Self::balance_to_u128(new_locked_amount)? * 1000000000000;
				if Self::u128_to_balance_option(locking_amount).is_ok() {
					<T as pallet::Config>::Currency::set_lock(
						EXAMPLE_ID,
						&ledger.staker,
						Self::u128_to_balance_option(locking_amount)?,
						WithdrawReasons::all(),
					);
					Ledger::<T>::insert(staking_index, ledger.clone());
				} else {
					Ledger::<T>::insert(staking_index, ledger.clone());
				}
			}
			AmountLocked::<T>::insert(ledger.staker, new_locked_amount);

			ActiveStakings::<T>::try_append(staking_index)
				.map_err(|_| Error::<T>::TooManyStakers)?;

			let total_stake = Self::total_stake();
			StakingCount::<T>::put(staking_index);
			TotalStake::<T>::put(total_stake + value);
			Ok(())
		}

		fn queue_helper(staker: T::AccountId, value: BalanceOf<T>) -> DispatchResult {
			let queue_index = Self::queue_count() + 1;
			ensure!(Self::queue_ledger(queue_index).is_none(), Error::<T>::IndexInUse);
			let timestamp = <T as pallet::Config>::TimeProvider::now().as_secs();
			let queue_ledger =
				QueueLedgerAccount { staker: staker.clone(), locked: value, timestamp };

			let locked_amount = if Self::amount_locked(staker.clone()).is_none() {
				Default::default()
			} else {
				Self::amount_locked(staker).ok_or(Error::<T>::StakerNothingLocked)?
			};
			let new_locked_amount = locked_amount.saturating_add(value);
			if new_locked_amount.is_zero() {
				QueueLedger::<T>::remove(queue_index);
				<T as pallet::Config>::Currency::remove_lock(EXAMPLE_ID, &queue_ledger.staker);
			} else {
				let locking_amount = Self::balance_to_u128(new_locked_amount)? * 1000000000000;
				if Self::u128_to_balance_option(locking_amount).is_ok() {
					<T as pallet::Config>::Currency::set_lock(
						EXAMPLE_ID,
						&queue_ledger.staker,
						Self::u128_to_balance_option(locking_amount)?,
						WithdrawReasons::all(),
					);
					QueueLedger::<T>::insert(queue_index, queue_ledger.clone());
				} else {
					QueueLedger::<T>::insert(queue_index, queue_ledger.clone());
				}
			}
			AmountLocked::<T>::insert(queue_ledger.staker, new_locked_amount);
			QueueCount::<T>::put(queue_index);
			QueueStaking::<T>::try_append(queue_index).map_err(|_| Error::<T>::TooManyStakers)?;
			Ok(())
		}

		/// Calculates the current staking apy.
		fn calculate_current_apy() -> Result<u128, Error<T>> {
			let ongoing_loans = pallet_community_loan_pool::Pallet::<T>::ongoing_loans();
			let mut loan_apys = 0;
			if ongoing_loans.len() == 0 {
				return Ok(0);
			}
			let total_amount_loan =
				pallet_community_loan_pool::Pallet::<T>::total_loan_amount() as u128;
			if total_amount_loan == 0 {
				return Ok(0);
			}
			for i in ongoing_loans {
				let loan_index = i;
				let loan = pallet_community_loan_pool::Pallet::<T>::loans(loan_index)
					.ok_or(Error::<T>::NoLoanFound)?;
				loan_apys += loan.loan_apy as u128
					* TryInto::<u128>::try_into(loan.current_loan_balance)
						.map_err(|_| Error::<T>::ConversionError)?
					* 10000 / total_amount_loan;
			}
			let average_loan_apy = loan_apys / 10000;
			Ok(average_loan_apy.saturating_sub(200))
		}

		/// Claims the rewards for the stakers
		pub fn claim_rewards() -> DispatchResult {
			let active_stakings = Self::active_stakings();
			for i in active_stakings {
				let staking = i;
				let mut ledger = Self::ledger(staking).ok_or(Error::<T>::LedgerNotFound)?;
				//ensure!(ledger.locked > 0, Error::<T>::NoStakedAmount);
				let apy = Self::calculate_current_apy()?;
				let current_timestamp = <T as pallet::Config>::TimeProvider::now().as_secs();
				let locked_amount = Self::balance_to_u128(ledger.locked)?;
				let rewards = locked_amount * apy * (current_timestamp - ledger.timestamp) as u128
					/ 365 / 60 / 60 / 24 / 100
					/ 100;
				let new_earned_rewards =
					ledger.earned_rewards + Self::u128_to_balance_option(rewards)?;
				ledger.earned_rewards = new_earned_rewards;
				ledger.timestamp = current_timestamp;
				Ledger::<T>::insert(staking, ledger.clone());
				let loan_pool_account = pallet_community_loan_pool::Pallet::<T>::account_id();
				<T as pallet::Config>::Currency::transfer(
					&loan_pool_account,
					&ledger.staker,
					Self::u128_to_balance_option(rewards * 1000000000000)?,
					KeepAlive,
				)?;
				Self::deposit_event(Event::<T>::RewardsClaimed {
					amount: Self::u128_to_balance_option(rewards)?,
					apy,
				});
			}
			Ok(())
		}

		/// If the total loan amount is lower than the total stake, this function
		/// unstakes the stake so that the total amount of the stake equals the total amount of the loan
		fn check_relation_to_loan() -> DispatchResult {
			let mut total_amount_loan =
				pallet_community_loan_pool::Pallet::<T>::total_loan_amount() as u128;
			let mut total_stake = Self::balance_to_u128(Self::total_stake())?;
			while total_stake > total_amount_loan {
				let first_staking = &Self::active_stakings()[0];
				let ledger = Self::ledger(first_staking).ok_or(Error::<T>::LedgerNotFound)?;
				if Self::balance_to_u128(ledger.locked)?
					< total_stake.saturating_sub(total_amount_loan)
				{
					Self::unstake_staker(*first_staking, ledger.locked)?;
				} else {
					let value = total_stake.saturating_sub(total_amount_loan);
					Self::unstake_staker(*first_staking, Self::u128_to_balance_option(value)?)?;
				};
				total_amount_loan =
					pallet_community_loan_pool::Pallet::<T>::total_loan_amount() as u128;
				total_stake = Self::balance_to_u128(Self::total_stake())?;
			}
			while total_stake < total_amount_loan {
				let queue = Self::queue_staking();
				if queue.len() == 0 {
					break;
				}
				let first_queue = queue[0];
				let queue_ledger =
					Self::queue_ledger(first_queue).ok_or(Error::<T>::LedgerNotFound)?;
				if Self::balance_to_u128(queue_ledger.locked)?
					< total_amount_loan.saturating_sub(total_stake)
				{
				} else {
					let value = total_amount_loan.saturating_sub(total_stake);
					Self::stake_from_queue(first_queue, Self::u128_to_balance_option(value)?)?;
				}
				total_amount_loan =
					pallet_community_loan_pool::Pallet::<T>::total_loan_amount() as u128;
				total_stake = Self::balance_to_u128(Self::total_stake())?;
			}
			Ok(())
		}

		/// Unstakes stakers
		fn unstake_staker(staking_index: StakingIndex, value: BalanceOf<T>) -> DispatchResult {
			let mut ledger = Self::ledger(staking_index).ok_or(Error::<T>::LedgerNotFound)?;

			ledger.locked = ledger.locked.saturating_sub(value);

			let locked_amount = if Self::amount_locked(ledger.staker.clone()).is_none() {
				Default::default()
			} else {
				Self::amount_locked(ledger.staker.clone()).ok_or(Error::<T>::StakerNothingLocked)?
			};
			let new_locked_amount = locked_amount.saturating_sub(value);
			if new_locked_amount.is_zero() {
				Ledger::<T>::remove(staking_index);
				<T as pallet::Config>::Currency::remove_lock(EXAMPLE_ID, &ledger.staker);
			} else {
				let locking_amount = Self::balance_to_u128(new_locked_amount)? * 1000000000000;
				if Self::u128_to_balance_option(locking_amount).is_ok() {
					<T as pallet::Config>::Currency::set_lock(
						EXAMPLE_ID,
						&ledger.staker,
						Self::u128_to_balance_option(locking_amount)?,
						WithdrawReasons::all(),
					);
				}
			}

			if ledger.locked.is_zero() {
				let mut active_stakings = Self::active_stakings();
				let index = active_stakings
					.iter()
					.position(|x| *x == staking_index)
					.ok_or(Error::<T>::NoStaker)?;
				active_stakings.remove(index);
				Ledger::<T>::remove(staking_index);
				ActiveStakings::<T>::put(active_stakings);
			} else {
				Ledger::<T>::insert(staking_index, ledger.clone());
			}
			AmountLocked::<T>::insert(ledger.staker.clone(), new_locked_amount);
			Self::queue_helper(ledger.staker.clone(), value)?;
			let total_stake = Self::total_stake();
			TotalStake::<T>::put(total_stake.saturating_sub(value));

			Self::deposit_event(Event::Unlocked { staker: ledger.staker, amount: value });
			Ok(())
		}

		/// Staker from the queue
		fn stake_from_queue(queue_index: QueueIndex, value: BalanceOf<T>) -> DispatchResult {
			let mut queue_ledger =
				Self::queue_ledger(queue_index).ok_or(Error::<T>::LedgerNotFound)?;

			queue_ledger.locked = queue_ledger.locked.saturating_sub(value);

			let locked_amount = if Self::amount_locked(queue_ledger.staker.clone()).is_none() {
				Default::default()
			} else {
				Self::amount_locked(queue_ledger.staker.clone())
					.ok_or(Error::<T>::StakerNothingLocked)?
			};
			let new_locked_amount = locked_amount.saturating_sub(value);
			if new_locked_amount.is_zero() {
				QueueLedger::<T>::remove(queue_index);
				<T as pallet::Config>::Currency::remove_lock(EXAMPLE_ID, &queue_ledger.staker);
			} else {
				let locking_amount = Self::balance_to_u128(new_locked_amount)? * 1000000000000;
				if Self::u128_to_balance_option(locking_amount).is_ok() {
					<T as pallet::Config>::Currency::set_lock(
						EXAMPLE_ID,
						&queue_ledger.staker,
						Self::u128_to_balance_option(locking_amount)?,
						WithdrawReasons::all(),
					);
				}
			}

			if queue_ledger.locked.is_zero() {
				let mut queue_stakers = Self::queue_staking();
				let index = queue_stakers
					.iter()
					.position(|x| *x == queue_index)
					.ok_or(Error::<T>::NoStaker)?;
				queue_stakers.remove(index);
				QueueLedger::<T>::remove(queue_index);
				QueueStaking::<T>::put(queue_stakers);
			} else {
				QueueLedger::<T>::insert(queue_index, queue_ledger.clone());
			}
			AmountLocked::<T>::insert(queue_ledger.staker.clone(), new_locked_amount);
			Self::stake_helper(queue_ledger.staker.clone(), value)?;
			Self::deposit_event(Event::Locked { staker: queue_ledger.staker, amount: value });
			Ok(())
		}

		pub fn balance_to_u128(input: BalanceOf<T>) -> Result<u128, Error<T>> {
			TryInto::<u128>::try_into(input).map_err(|_| Error::<T>::ConversionError)
		}

		pub fn u128_to_balance_option(input: u128) -> Result<BalanceOf<T>, Error<T>> {
			input.try_into().map_err(|_| Error::<T>::ConversionError)
		}
	}
}