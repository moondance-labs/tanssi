//! Faucet pallet benchmarking.

use super::*;

#[allow(unused)]
use crate::Pallet as Faucet;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::{Pallet as System, RawOrigin};

benchmarks! {
	claim_tokens {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller.clone()))
	verify {
		assert_eq!(
			Faucet::<T>::last_claim_of(&caller),
			Some((1, System::<T>::block_number()))
		);
	}
}

impl_benchmark_test_suite!(
	Faucet,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);
