#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet.
		type WeightInfo: WeightInfo;
		/// Origin who can add and remove users to the whitelist.
		type WhitelistOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Max users allowed in the whitelist.
		type MaxUsersInWhitelist: Get<u32>;
	}

	/// Mapping of an account to a bool.
	#[pallet::storage]
	#[pallet::getter(fn whitelisted_accounts)]
	pub type WhitelistedAccounts<T: Config> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new user has been successfully whitelisted.
		NewUserWhitelisted { user: T::AccountId },
		/// A new user has been successfully removed.
		UserRemoved { user: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The user is already registered in the whitelist.
		AccountAlreadyWhitelisted,
		/// The user has not been registered in the whitelist.
		UserNotInWhitelist,
		/// Too many users are already in the whitelist.
		TooManyUsers,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Adds a user to the whitelist.
		///
		/// The origin must be the sudo.
		///
		/// Parameters:
		/// - `user`: The address of the new account added to the whitelist.
		///
		/// Emits `NewUserWhitelisted` event when succesfful
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_to_whitelist())]
		pub fn add_to_whitelist(origin: OriginFor<T>, user: AccountIdOf<T>) -> DispatchResult {
			T::WhitelistOrigin::ensure_origin(origin)?;
			ensure!(
				!Self::whitelisted_accounts(user.clone()),
				Error::<T>::AccountAlreadyWhitelisted
			);
			WhitelistedAccounts::<T>::insert(user.clone(), true);
			Self::deposit_event(Event::<T>::NewUserWhitelisted { user });
			Ok(())
		}

		/// Removes a user from the whitelist.
		///
		/// The origin must be the sudo.
		///
		/// Parameters:
		/// - `user`: The address of the new account removed from the whitelist.
		///
		/// Emits `UserRemoved` event when succesfful
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::remove_from_whitelist())]
		pub fn remove_from_whitelist(origin: OriginFor<T>, user: AccountIdOf<T>) -> DispatchResult {
			T::WhitelistOrigin::ensure_origin(origin)?;
			ensure!(Self::whitelisted_accounts(user.clone()), Error::<T>::UserNotInWhitelist);
			WhitelistedAccounts::<T>::take(user.clone());
			Self::deposit_event(Event::<T>::UserRemoved { user });
			Ok(())
		}
	}
}