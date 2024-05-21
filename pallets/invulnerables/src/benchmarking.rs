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

//! Benchmarking setup for pallet-invulnerables

#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::Pallet as InvulnerablesPallet;
use {
    frame_benchmarking::{account, impl_benchmark_test_suite, v2::*, BenchmarkError},
    frame_support::{
        pallet_prelude::*,
        traits::{tokens::fungible::Balanced, Currency, EnsureOrigin, Get},
    },
    frame_system::{EventRecord, RawOrigin},
    pallet_session::{self as session, SessionManager},
    sp_runtime::traits::AtLeast32BitUnsigned,
    sp_std::prelude::*,
    tp_traits::DistributeRewards,
};
const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

fn create_funded_user<T: Config + pallet_balances::Config>(
    string: &'static str,
    n: u32,
    balance_factor: u32,
) -> T::AccountId {
    let user = account(string, n, SEED);
    let balance = <pallet_balances::Pallet<T> as Currency<T::AccountId>>::minimum_balance()
        * balance_factor.into();
    let _ = <pallet_balances::Pallet<T> as Currency<T::AccountId>>::make_free_balance_be(
        &user, balance,
    );
    user
}

fn keys<T: Config + session::Config>(c: u32) -> <T as session::Config>::Keys {
    use rand::{RngCore, SeedableRng};

    let keys = {
        let mut keys = [0u8; 128];

        if c > 0 {
            let mut rng = rand::rngs::StdRng::seed_from_u64(c as u64);
            rng.fill_bytes(&mut keys);
        }

        keys
    };

    Decode::decode(&mut &keys[..]).unwrap()
}

fn invulnerable<T: Config + session::Config + pallet_balances::Config>(
    c: u32,
) -> (T::AccountId, T::CollatorId, <T as session::Config>::Keys) {
    let funded_user = create_funded_user::<T>("candidate", c, 100);
    let collator_id = T::CollatorIdOf::convert(funded_user.clone())
        .expect("Converstion of account id of collator id failed.");
    (funded_user, collator_id, keys::<T>(c))
}

fn invulnerables<
    T: Config + frame_system::Config + pallet_session::Config + pallet_balances::Config,
>(
    count: u32,
) -> Vec<(T::AccountId, T::CollatorId)> {
    let invulnerables = (0..count).map(|c| invulnerable::<T>(c)).collect::<Vec<_>>();

    for (who, _collator_id, keys) in invulnerables.clone() {
        <session::Pallet<T>>::set_keys(RawOrigin::Signed(who).into(), keys, Vec::new()).unwrap();
    }

    invulnerables
        .into_iter()
        .map(|(who, collator_id, _)| (who, collator_id))
        .collect()
}

pub type BalanceOf<T> =
    <<T as crate::Config>::Currency as frame_support::traits::fungible::Inspect<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

pub(crate) fn currency_issue<T: Config + frame_system::Config>(
    amount: BalanceOf<T>,
) -> crate::CreditOf<T, T::Currency> {
    <<T as crate::Config>::Currency as Balanced<T::AccountId>>::issue(amount)
}

#[allow(clippy::multiple_bound_locations)]
#[benchmarks(where T: session::Config + pallet_balances::Config, BalanceOf<T>: AtLeast32BitUnsigned)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn add_invulnerable(
        b: Linear<1, { T::MaxInvulnerables::get() - 1 }>,
    ) -> Result<(), BenchmarkError> {
        let origin =
            T::UpdateOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;

        // now we need to fill up invulnerables
        let mut invulnerables = invulnerables::<T>(b);
        invulnerables.sort();

        let (_account_ids, collator_ids): (Vec<T::AccountId>, Vec<T::CollatorId>) =
            invulnerables.into_iter().unzip();

        let invulnerables: frame_support::BoundedVec<_, T::MaxInvulnerables> =
            frame_support::BoundedVec::try_from(collator_ids).unwrap();
        <Invulnerables<T>>::put(invulnerables);

        let (new_invulnerable, _collator_id, keys) = invulnerable::<T>(b + 1);
        <session::Pallet<T>>::set_keys(
            RawOrigin::Signed(new_invulnerable.clone()).into(),
            keys,
            Vec::new(),
        )
        .unwrap();

        #[extrinsic_call]
        _(origin as T::RuntimeOrigin, new_invulnerable.clone());

        assert_last_event::<T>(
            Event::InvulnerableAdded {
                account_id: new_invulnerable,
            }
            .into(),
        );
        Ok(())
    }

    #[benchmark]
    fn remove_invulnerable(
        b: Linear<{ 1 }, { T::MaxInvulnerables::get() }>,
    ) -> Result<(), BenchmarkError> {
        let origin =
            T::UpdateOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
        let mut invulnerables = invulnerables::<T>(b);
        invulnerables.sort();

        let (account_ids, collator_ids): (Vec<T::AccountId>, Vec<T::CollatorId>) =
            invulnerables.into_iter().unzip();

        let invulnerables: frame_support::BoundedVec<_, T::MaxInvulnerables> =
            frame_support::BoundedVec::try_from(collator_ids).unwrap();
        <Invulnerables<T>>::put(invulnerables);

        let to_remove = account_ids.last().unwrap().clone();

        #[extrinsic_call]
        _(origin as T::RuntimeOrigin, to_remove.clone());

        assert_last_event::<T>(
            Event::InvulnerableRemoved {
                account_id: to_remove,
            }
            .into(),
        );
        Ok(())
    }

    // worst case for new session.
    #[benchmark]
    fn new_session(r: Linear<1, { T::MaxInvulnerables::get() }>) -> Result<(), BenchmarkError> {
        let origin =
            T::UpdateOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;

        frame_system::Pallet::<T>::set_block_number(0u32.into());
        // now we need to fill up invulnerables
        let mut invulnerables = invulnerables::<T>(r);
        invulnerables.sort();

        let (account_ids, _collator_ids): (Vec<T::AccountId>, Vec<T::CollatorId>) =
            invulnerables.into_iter().unzip();

        for account in account_ids {
            <InvulnerablesPallet<T>>::add_invulnerable(origin.clone(), account)
                .expect("add invulnerable failed");
        }

        #[block]
        {
            <InvulnerablesPallet<T> as SessionManager<_>>::new_session(0);
        }

        Ok(())
    }

    #[benchmark]
    fn reward_invulnerable(
        b: Linear<{ 1 }, { T::MaxInvulnerables::get() }>,
    ) -> Result<(), BenchmarkError> where {
        let mut invulnerables = invulnerables::<T>(b);
        invulnerables.sort();

        let (account_ids, collator_ids): (Vec<T::AccountId>, Vec<T::CollatorId>) =
            invulnerables.into_iter().unzip();

        let invulnerables: frame_support::BoundedVec<_, T::MaxInvulnerables> =
            frame_support::BoundedVec::try_from(collator_ids).unwrap();
        <Invulnerables<T>>::put(invulnerables);
        let to_reward = account_ids.first().unwrap().clone();
        // Create new supply for rewards
        let new_supply = currency_issue::<T>(1000u32.into());
        #[block]
        {
            let _ = InvulnerableRewardDistribution::<T, T::Currency, ()>::distribute_rewards(
                to_reward, new_supply,
            );
        }

        Ok(())
    }
    impl_benchmark_test_suite!(
        InvulnerablesPallet,
        crate::mock::new_test_ext(),
        crate::mock::Test,
    );
}
