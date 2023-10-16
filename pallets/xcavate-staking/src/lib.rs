#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
/// <https://docs.substrate.io/reference/frame-pallets/>
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::sp_runtime::traits::Zero;

use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement::KeepAlive, Get, ReservableCurrency},
};

use frame_support::traits::UnixTime;

use frame_support::sp_runtime::Saturating;

use sp_std::prelude::*;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReasons},
	};

	use frame_system::{ensure_signed, pallet_prelude::*};

	const EXAMPLE_ID: LockIdentifier = *b"stkxcavc";

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_community_loan_pool::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
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
		/// lose coupling of pallet timestamp
		type TimeProvider: UnixTime;
	}

	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	pub struct LedgerAccount<Balance> {
		/// Balance locked
		pub locked: Balance,
		/// Timestamp locked
		pub timestamp: u64,
	}

	#[pallet::storage]
	#[pallet::getter(fn ledger)]
	pub type Ledger<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, LedgerAccount<BalanceOf<T>>, OptionQuery>;

	/// All current stakers
	#[pallet::storage]
	#[pallet::getter(fn active_stakers)]
	pub type ActiveStakers<T: Config> =
		StorageValue<_, BoundedVec<T::AccountId, T::MaxStakers>, ValueQuery>;

	/// Number of proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn total_stake)]
	pub(super) type TotalStake<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Balance was locked successfully.
		Locked { staker: <T as frame_system::Config>::AccountId, amount: BalanceOf<T> },
		/// Lock was extended successfully.
		ExtendedLock { staker: <T as frame_system::Config>::AccountId, amount: BalanceOf<T> },
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
		///
		NoStaker,
	}

	// Work in progress, to be included in the future
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: frame_system::pallet_prelude::BlockNumberFor<T>) -> Weight {
			T::DbWeight::get().writes(1)
		}

		fn on_finalize(_n: frame_system::pallet_prelude::BlockNumberFor<T>) {
			//let block = n.saturated_into::<u64>();
			/* if block % 10 == 0 { */
			Self::claim_rewards().unwrap_or_default();
			Self::check_relation_to_loan().unwrap_or_default();
			/* } */
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn stake(
			origin: OriginFor<T>,
			#[pallet::compact] value: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let staker = ensure_signed(origin)?;

			ensure!(!value.is_zero(), Error::<T>::StakingWithNoValue);

			if Self::ledger(&staker).is_none() {
				let available_balance = <T as pallet::Config>::Currency::free_balance(&staker)
					.saturating_sub(T::MinimumRemainingAmount::get());

				let value_to_stake = value.min(available_balance);

				let timestamp = <T as pallet::Config>::TimeProvider::now().as_secs();

				let ledger = LedgerAccount { locked: value_to_stake, timestamp };

				Self::update_ledger(&staker, ledger);

				ActiveStakers::<T>::try_append(staker.clone())
					.map_err(|_| Error::<T>::TooManyStakers)?;

				let total_stake = Self::total_stake();
				TotalStake::<T>::put(total_stake + value_to_stake);
			} else {
				let mut ledger = Self::ledger(&staker).unwrap();

				let available_balance = Self::available_staking_balance(&staker, &ledger);
				let value_to_stake = value.min(available_balance);

				let timestamp = <T as pallet::Config>::TimeProvider::now().as_secs();

				//ensure!(value_to_stake > 0, Error::<T>::StakingWithNoValue);

				ledger.locked += value;
				ledger.timestamp = timestamp;

				Self::update_ledger(&staker, ledger);

				let total_stake = Self::total_stake();
				TotalStake::<T>::put(total_stake + value_to_stake);
			}

			Self::deposit_event(Event::Locked { staker, amount: value });
			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(1_000)]
		pub fn unstake(
			origin: OriginFor<T>,
			#[pallet::compact] value: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let staker = ensure_signed(origin)?;

			ensure!(Self::ledger(&staker).is_some(), Error::<T>::NoStaker);

			let mut ledger = Self::ledger(&staker).unwrap();

			ledger.locked = ledger.locked.saturating_sub(value);

			if ledger.locked.is_zero() {
				let mut active_staker = Self::active_stakers();
				let index = active_staker.iter().position(|x| *x == staker.clone()).unwrap();
				active_staker.remove(index);
				ActiveStakers::<T>::put(active_staker);
			}

			Self::update_ledger(&staker, ledger);

			let total_stake = Self::total_stake();
			TotalStake::<T>::put(total_stake - value);

			Self::deposit_event(Event::Unlocked { staker, amount: value });
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn available_staking_balance(
			staker: &T::AccountId,
			ledger: &LedgerAccount<BalanceOf<T>>,
		) -> BalanceOf<T> {
			let free_balance = <T as pallet::Config>::Currency::free_balance(staker)
				.saturating_sub(T::MinimumRemainingAmount::get());
			free_balance.saturating_sub(ledger.locked)
		}

		fn update_ledger(staker: &T::AccountId, ledger: LedgerAccount<BalanceOf<T>>) {
			if ledger.locked.is_zero() {
				Ledger::<T>::remove(staker);
				<T as pallet::Config>::Currency::remove_lock(EXAMPLE_ID, staker);
			} else {
				let locking_amount = Self::balance_to_u128(ledger.locked).unwrap() * 1000000000000;
				if Self::u128_to_balance_option(locking_amount).is_none() {
					Ledger::<T>::insert(staker, ledger);
				} else {
					<T as pallet::Config>::Currency::set_lock(
						EXAMPLE_ID,
						staker,
						Self::u128_to_balance_option(locking_amount).unwrap(),
						WithdrawReasons::all(),
					);
					Ledger::<T>::insert(staker, ledger);
				}
			}
		}

		fn calculate_current_apy() -> u128 {
			let ongoing_loans = pallet_community_loan_pool::Pallet::<T>::ongoing_loans();
			let mut loan_apys = 0;
			if ongoing_loans.len() == 0 {
				return 0;
			}
			let total_amount_loan =
				pallet_community_loan_pool::Pallet::<T>::total_loan_amount() as u128;
			if total_amount_loan == 0 {
				return 0;
			}
			for i in ongoing_loans {
				let loan_index = i;
				let loan = pallet_community_loan_pool::Pallet::<T>::loans(loan_index).unwrap();
				loan_apys += loan.loan_apy as u128
					* TryInto::<u128>::try_into(loan.borrowed_amount + loan.available_amount)
						.ok()
						.unwrap() * 10000 / total_amount_loan;
			}
			let average_loan_apy = loan_apys / 10000;
			average_loan_apy - 200
		}

		pub fn claim_rewards() -> DispatchResult {
			let active_stakers = Self::active_stakers();
			for i in active_stakers {
				let staker = i;
				let mut ledger = Self::ledger(staker.clone()).unwrap();
				//ensure!(ledger.locked > 0, Error::<T>::NoStakedAmount);
				let apy = Self::calculate_current_apy();
				let current_timestamp = <T as pallet::Config>::TimeProvider::now().as_secs();
				let locked_amount = Self::balance_to_u128(ledger.locked).unwrap();
				let rewards = locked_amount * apy * (current_timestamp - ledger.timestamp) as u128
					/ 365 / 60 / 60 / 24 / 100
					/ 100;
				let new_locked_amount = locked_amount + rewards;
				ledger.locked = Self::u128_to_balance_option(new_locked_amount).unwrap();
				ledger.timestamp = current_timestamp;
				Ledger::<T>::insert(staker.clone(), ledger.clone());
				let loan_pool_account = pallet_community_loan_pool::Pallet::<T>::account_id();
				<T as pallet::Config>::Currency::transfer(
					&loan_pool_account,
					&staker,
					(rewards * 1000000000000).try_into().ok().unwrap(),
					KeepAlive,
				)
				.unwrap_or_default();
				let locking_amount =
					Self::u128_to_balance_option(new_locked_amount * 1000000000000).unwrap();
				<T as pallet::Config>::Currency::set_lock(
					EXAMPLE_ID,
					&staker,
					locking_amount,
					WithdrawReasons::all(),
				);
				let total_stake = Self::total_stake();
				let new_total_stake = total_stake + Self::u128_to_balance_option(rewards).unwrap();
				TotalStake::<T>::put(new_total_stake);
				Self::deposit_event(Event::<T>::RewardsClaimed {
					amount: rewards.try_into().ok().unwrap(),
					apy,
				});
			}
			Ok(())
		}

		fn check_relation_to_loan() -> DispatchResult {
			let mut total_amount_loan =
				pallet_community_loan_pool::Pallet::<T>::total_loan_amount() as u128;
			let mut total_stake = Self::balance_to_u128(Self::total_stake()).unwrap();
			while total_stake > total_amount_loan {
				let stakers = Self::active_stakers();
				let last_staker = &Self::active_stakers()[stakers.len() - 1];
				let mut ledger = Self::ledger(&last_staker).unwrap();
				if Self::balance_to_u128(ledger.locked).unwrap()
					< total_stake.saturating_sub(total_amount_loan)
				{
					Self::unstake_staker(last_staker.clone(), ledger.locked);
				} else {
					let value = total_stake.saturating_sub(total_amount_loan);
					Self::unstake_staker(
						last_staker.clone(),
						Self::u128_to_balance_option(value).unwrap(),
					);
				};
				total_amount_loan =
					pallet_community_loan_pool::Pallet::<T>::total_loan_amount() as u128;
				total_stake = Self::balance_to_u128(Self::total_stake()).unwrap();
			}
			Ok(())
		}

		fn unstake_staker(staker: T::AccountId, value: BalanceOf<T>) -> DispatchResult {
			let mut ledger = Self::ledger(&staker).unwrap();

			ledger.locked = ledger.locked.saturating_sub(value);

			if ledger.locked.is_zero() {
				let mut active_staker = Self::active_stakers();
				let index = active_staker.iter().position(|x| *x == staker.clone()).unwrap();
				active_staker.remove(index);
				ActiveStakers::<T>::put(active_staker);
			}

			Self::update_ledger(&staker, ledger);

			let total_stake = Self::total_stake();
			TotalStake::<T>::put(total_stake - value);

			Self::deposit_event(Event::Unlocked { staker, amount: value });
			Ok(())
		}

		pub fn balance_to_u128(input: BalanceOf<T>) -> Option<u128> {
			TryInto::<u128>::try_into(input).ok()
		}

		pub fn u128_to_balance_option(input: u128) -> Option<BalanceOf<T>> {
			input.try_into().ok()
		}
	}
}
