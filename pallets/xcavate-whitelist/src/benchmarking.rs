//! Benchmarking setup for pallet-whitelist
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Whitelist;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::assert_ok;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn add_to_whitelist() {
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		add_to_whitelist(RawOrigin::Root, caller.clone());

		assert_eq!(Whitelist::<T>::whitelisted_accounts(caller), true);
	}

	#[benchmark]
	fn remove_from_whitelist() {
		let caller: T::AccountId = whitelisted_caller();
		assert_ok!(Whitelist::<T>::add_to_whitelist(RawOrigin::Root.into(), caller.clone()));
		assert_eq!(Whitelist::<T>::whitelisted_accounts(caller.clone()), true);
		#[extrinsic_call]
		remove_from_whitelist(RawOrigin::Root, caller.clone());

		assert_eq!(Whitelist::<T>::whitelisted_accounts(caller), false);
	}

	impl_benchmark_test_suite!(Whitelist, crate::mock::new_test_ext(), crate::mock::Test);
}
