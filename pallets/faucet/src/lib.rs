//! # Faucet Pallet
//!
//! - [`faucet::Config`](./pallet/trait.Config.html)
//! - [`Call`](./pallet/enum.Call.html)
//!
//! ## Overview
//!
//! The Faucet pallet allows an account to claim a pre-configured number of tokens
//! up to a certain number of times, and not faster than a configured interval.
//!
//! _This is a very simple pallet to be used on dev / test PoA chains only._
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `claim_tokens` - Claims the pre-defined number of tokens for the requestor account.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
pub mod weights;

use frame_support::traits::{Currency, Get, ReservableCurrency};
use frame_support::transactional;
pub use pallet::*;
use sp_runtime::traits::Saturating;
pub use weights::WeightInfo;
// pub use frame_system::Config;
use sp_std::convert::TryInto;
use frame_system::pallet_prelude::{BlockNumberFor};

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_runtime::{traits::CheckedAdd, ArithmeticError};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The currency mechanism.
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		/// The amount of token that "drips" from the faucet for every claim.
		#[pallet::constant]
		type DripAmount: Get<BalanceOf<Self>>;

		/// The minimum period, as a number of blocks, between consecutive claims of a given account.
		#[pallet::constant]
		type MinBlocksBetweenClaims: Get<BlockNumberFor<Self>>;

		/// The maximum number of times an account can claim tokens from the faucet.
		#[pallet::constant]
		type MaxClaimsPerAccount: Get<u32>;

		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	/// The total amount of token that dripped from the faucet.
	#[pallet::storage]
	#[pallet::getter(fn total_amount_dripped)]
	pub(super) type TotalAmountDripped<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// A map of the last claim per account with the claim count
	/// and the block number at which the claim was made.
	#[pallet::storage]
	#[pallet::getter(fn last_claim_of)]
	pub(super) type LastClaimOf<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, Option<(u32, BlockNumberFor<T>)>, ValueQuery>;

	/// Events type.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Faucet has dripped an amount of tokens to account [balance, who]
		FaucetDripped(BalanceOf<T>, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Last claim for account was made too recently
		LastClaimTooRecent,
		/// Maximum number of claims for account was exceeded
		MaxClaimsExceeded,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Allows an account to claim a pre-configured number of tokens up to a certain
		/// number of times, and not faster than a configured interval.
		///
		/// This extrinsic must be called by a signed origin.
		///
		/// Note: the dispatchable function is configured for feeless extrinsics, so the
		/// the requestor account will not pay any fee. This allows new accounts to claim
		/// tokens right away, without needing a initial transfer from another account.
		/// On the other hand, this renders the chain vulnerable to DOS attacks, so this
		/// pallet should not be used on production / incentivized networks (so, only on
		/// non-incentivized testnets / in non-adversarial environments).
		#[transactional]
        #[pallet::weight((T::WeightInfo::claim_tokens(), DispatchClass::Normal, Pays::No))]
		pub fn claim_tokens(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let current_block = <frame_system::Pallet<T>>::block_number();

			// Update existing claim or create a first one
			<LastClaimOf<T>>::try_mutate(&who, |last_claim| -> DispatchResult {
				let current_claim = match last_claim {
					Some((claim_count, last_claim_block)) => {
						// Increment claim count
						let new_claim_count =
							claim_count.checked_add(1).ok_or(ArithmeticError::Overflow)?;

						// Verify account's total claims does not exceed MaxClaimsPerAccount
						ensure!(
							new_claim_count <= T::MaxClaimsPerAccount::get(),
							Error::<T>::MaxClaimsExceeded
						);

						// Verify account's last claim was not less MinBlockBetweenClaims ago
						ensure!(
							last_claim_block.saturating_add(T::MinBlocksBetweenClaims::get())
								< current_block,
							Error::<T>::LastClaimTooRecent
						);

						(new_claim_count, current_block)
					}
					None => (1, current_block),
				};
				// Update last claim info
				*last_claim = Some(current_claim);
				Ok(())
			})?;

			// Get configured drip amount
			let amount = T::DripAmount::get();

			// Increment total amount dripped
			<TotalAmountDripped<T>>::try_mutate(|total| -> DispatchResult {
				*total = total.checked_add(&amount).ok_or(ArithmeticError::Overflow)?;
				Ok(())
			})?;

			// Drop positive imbalance, which will increase the total issuance of the chain's native token
			let imbalance = T::Currency::deposit_creating(&who, amount);
			drop(imbalance);

			Self::deposit_event(Event::FaucetDripped(amount, who));

			Ok(())
		}
	}
}
