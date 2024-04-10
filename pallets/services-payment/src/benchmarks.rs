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

#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking
use {
    crate::{
        BalanceOf, BlockNumberFor, Call, Config, Pallet, ProvideBlockProductionCost,
        ProvideCollatorAssignmentCost,
    },
    frame_benchmarking::{account, v2::*},
    frame_support::{
        assert_ok,
        traits::{Currency, EnsureOriginWithArg, Get},
    },
    frame_system::RawOrigin,
    sp_runtime::Saturating,
    sp_std::prelude::*,
    tp_traits::{AuthorNotingHook, CollatorAssignmentHook},
};

// Build genesis storage according to the mock runtime.
#[cfg(test)]
pub fn new_test_ext() -> sp_io::TestExternalities {
    const ALICE: u64 = 1;

    crate::mock::ExtBuilder::default()
        .with_balances(vec![(ALICE, 1_000)])
        .build()
}

const SEED: u32 = 0;

fn create_funded_user<T: Config>(
    string: &'static str,
    n: u32,
    balance_factor: u32,
) -> T::AccountId {
    let user = account(string, n, SEED);
    let balance = <T::Currency>::minimum_balance() * balance_factor.into();
    let _ = <T::Currency>::make_free_balance_be(&user, balance);
    user
}

#[benchmarks(where BalanceOf<T>: From<BlockNumberFor<T>>)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn purchase_credits() {
        let para_id = 1001u32.into();
        let payment: BalanceOf<T> = T::ProvideBlockProductionCost::block_cost(&para_id)
            .0
            .saturating_mul(1000u32.into());
        let caller = create_funded_user::<T>("caller", 1, 1_000_000_000u32);

        // Before call: 0 credits
        assert_eq!(
            crate::BlockProductionCredits::<T>::get(para_id).unwrap_or_default(),
            0u32.into()
        );

        #[extrinsic_call]
        Pallet::<T>::purchase_credits(RawOrigin::Signed(caller), para_id, payment);

        // verification code
        assert_eq!(
            <T::Currency>::total_balance(&crate::Pallet::<T>::parachain_tank(para_id)),
            payment
        );
    }

    #[benchmark]
    fn set_block_production_credits() {
        let para_id = 1001u32.into();
        let credits = T::FreeBlockProductionCredits::get();

        assert_ok!(Pallet::<T>::set_block_production_credits(
            RawOrigin::Root.into(),
            para_id,
            credits,
        ));

        // Before call: 1000 credits
        assert_eq!(
            crate::BlockProductionCredits::<T>::get(para_id).unwrap_or_default(),
            T::FreeBlockProductionCredits::get()
        );

        #[extrinsic_call]
        Pallet::<T>::set_block_production_credits(RawOrigin::Root, para_id, 1u32.into());

        // After call: 1 credit
        assert_eq!(
            crate::BlockProductionCredits::<T>::get(para_id).unwrap_or_default(),
            1u32.into()
        );
    }

    #[benchmark]
    fn set_given_free_credits() {
        let para_id = 1001u32.into();

        // Before call: no given free credits
        assert!(crate::GivenFreeCredits::<T>::get(para_id).is_none());

        #[extrinsic_call]
        Pallet::<T>::set_given_free_credits(RawOrigin::Root, para_id, true);

        // After call: given free credits
        assert!(crate::GivenFreeCredits::<T>::get(para_id).is_some());
    }

    #[benchmark]
    fn set_refund_address() {
        let para_id = 1001u32.into();

        let origin = T::ManagerOrigin::try_successful_origin(&para_id)
            .expect("failed to create ManagerOrigin");

        let refund_address = account("sufficient", 0, 1000);

        // Before call: no given free credits
        assert!(crate::RefundAddress::<T>::get(para_id).is_none());

        #[extrinsic_call]
        Pallet::<T>::set_refund_address(origin as T::RuntimeOrigin, para_id, Some(refund_address));

        // After call: given free credits
        assert!(crate::RefundAddress::<T>::get(para_id).is_some());
    }

    #[benchmark]
    fn set_max_core_price() {
        let para_id = 1001u32.into();

        let origin = T::ManagerOrigin::try_successful_origin(&para_id)
            .expect("failed to create ManagerOrigin");

        let max_price = 100_000_000;

        // Before call: none
        assert_eq!(crate::MaxCorePrice::<T>::get(para_id), None);

        #[extrinsic_call]
        Pallet::<T>::set_max_core_price(origin as T::RuntimeOrigin, para_id, Some(max_price));

        // After call: some
        assert_eq!(crate::MaxCorePrice::<T>::get(para_id), Some(max_price));
    }

    #[benchmark]
    fn on_container_author_noted() {
        let para_id = 1001u32;
        let block_cost = T::ProvideBlockProductionCost::block_cost(&para_id.into()).0;
        let credits: BalanceOf<T> = 1000u32.into();
        let balance_to_purchase = block_cost.saturating_mul(credits);
        let caller = create_funded_user::<T>("caller", 1, 1_000_000_000u32);
        let existential_deposit = <T::Currency>::minimum_balance();
        assert_ok!(Pallet::<T>::purchase_credits(
            RawOrigin::Signed(caller.clone()).into(),
            para_id.into(),
            balance_to_purchase + existential_deposit
        ));
        #[block]
        {
            <Pallet<T> as AuthorNotingHook<T::AccountId>>::on_container_author_noted(
                &caller,
                0,
                para_id.into(),
            );
        }
    }

    #[benchmark]
    fn on_collators_assigned() {
        let para_id = 1001u32;
        let collator_assignment_cost =
            T::ProvideCollatorAssignmentCost::collator_assignment_cost(&para_id.into()).0;
        let max_credit_stored = T::FreeCollatorAssignmentCredits::get();
        let balance_to_purchase = collator_assignment_cost.saturating_mul(max_credit_stored.into());
        let caller = create_funded_user::<T>("caller", 1, 1_000_000_000u32);
        let existential_deposit = <T::Currency>::minimum_balance();
        let tip = 1_000_000u32;
        assert_ok!(Pallet::<T>::purchase_credits(
            RawOrigin::Signed(caller.clone()).into(),
            para_id.into(),
            balance_to_purchase + existential_deposit + tip.into()
        ));
        assert_ok!(Pallet::<T>::set_max_tip(
            RawOrigin::Root.into(),
            para_id.into(),
            Some(tip.into())
        ));
        #[block]
        {
            <Pallet<T> as CollatorAssignmentHook<BalanceOf<T>>>::on_collators_assigned(
                para_id.into(),
                Some(&tip.into()),
                false,
            )
            .expect("failed on_collators_assigned");
        }
    }

    #[benchmark]
    fn set_max_tip() {
        let para_id = 1001u32.into();

        assert!(crate::MaxTip::<T>::get(para_id).is_none());

        #[extrinsic_call]
        Pallet::<T>::set_max_tip(RawOrigin::Root, para_id, Some(1_000_000u32.into()));

        assert!(crate::MaxTip::<T>::get(para_id).is_some());
    }

    impl_benchmark_test_suite!(Pallet, crate::benchmarks::new_test_ext(), crate::mock::Test);
}
