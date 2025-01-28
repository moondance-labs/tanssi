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

use super::*;

#[allow(unused)]
use crate::Pallet as EthereumTokenTransfers;
use {
    frame_benchmarking::{account, v2::*, BenchmarkError},
    frame_support::traits::Currency,
    frame_system::{EventRecord, RawOrigin},
    snowbridge_core::{AgentId, ChannelId, ParaId},
    sp_core::H160,
    sp_std::prelude::*,
};

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

const SEED: u32 = 0;

fn create_funded_user<T: Config + pallet_balances::Config>(
    string: &'static str,
    n: u32,
    balance_factor: u32,
) -> (T::AccountId, <T as pallet_balances::Config>::Balance) {
    let user = account(string, n, SEED);
    let balance = <pallet_balances::Pallet<T> as Currency<T::AccountId>>::minimum_balance()
        * balance_factor.into();
    let _ = <pallet_balances::Pallet<T> as Currency<T::AccountId>>::make_free_balance_be(
        &user, balance,
    );
    (user, balance)
}

#[allow(clippy::multiple_bound_locations)]
#[benchmarks(where T: pallet_balances::Config<Balance = u128>)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_token_transfer_channel() -> Result<(), BenchmarkError> {
        let channel_id = ChannelId::new([4u8; 32]);
        let agent_id = AgentId::from([5u8; 32]);
        let para_id: ParaId = 2000u32.into();

        #[extrinsic_call]
        _(RawOrigin::Root, channel_id, agent_id, para_id);

        assert_eq!(CurrentChannelId::<T>::get().unwrap(), channel_id);
        assert_eq!(CurrentAgentId::<T>::get().unwrap(), agent_id);
        assert_eq!(CurrentParaId::<T>::get().unwrap(), para_id);
        Ok(())
    }

    #[benchmark]
    fn transfer_native_token() -> Result<(), BenchmarkError> {
        let channel_id = ChannelId::new([4u8; 32]);
        let agent_id = AgentId::from([5u8; 32]);
        let para_id: ParaId = 2000u32.into();

        CurrentChannelId::<T>::put(channel_id);

        T::BenchmarkHelper::set_up_channel(
            channel_id,
            para_id,
            agent_id
        );

        T::BenchmarkHelper::set_up_token(
            T::TokenLocationReanchored::get(),
            H256::repeat_byte(0x01),
        );

        let (caller, initial_amount) = create_funded_user::<T>("account", 1, 1000000000);

        let amount_transferred = 10_000_000_000_000u128;
        let recipient = H160::from([1u8; 20]);

        let eth_sovereign_balance_before =
            <pallet_balances::Pallet<T> as Currency<T::AccountId>>::free_balance(
                &T::EthereumSovereignAccount::get(),
            );
        let fees_account_balance_before =
            <pallet_balances::Pallet<T> as Currency<T::AccountId>>::free_balance(
                &T::FeesAccount::get(),
            );

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            amount_transferred,
            recipient,
        );

        let caller_balance_after =
            <pallet_balances::Pallet<T> as Currency<T::AccountId>>::free_balance(&caller.clone());
        let eth_sovereign_balance_after =
            <pallet_balances::Pallet<T> as Currency<T::AccountId>>::free_balance(
                &T::EthereumSovereignAccount::get(),
            );
        let fees_account_balance_after =
            <pallet_balances::Pallet<T> as Currency<T::AccountId>>::free_balance(
                &T::FeesAccount::get(),
            );

        // We hardcode this as the fee is calculated inside pallet_outbound_queue, using
        // some internal functions which we don't have access to.
        let expected_fee = 2_680_020_281_600u128;

        assert_eq!(
            caller_balance_after,
            initial_amount - amount_transferred - expected_fee
        );
        assert_eq!(
            eth_sovereign_balance_after,
            eth_sovereign_balance_before + amount_transferred
        );
        assert_eq!(
            fees_account_balance_after,
            fees_account_balance_before + expected_fee
        );

        let expected_token_id =
            T::TokenIdFromLocation::convert_back(&T::TokenLocationReanchored::get());

        assert_last_event::<T>(
            Event::NativeTokenTransferred {
                channel_id,
                source: caller.clone(),
                recipient,
                token_id: expected_token_id.unwrap(),
                amount: amount_transferred,
                fee: expected_fee.into(),
            }
            .into(),
        );
        Ok(())
    }

    impl_benchmark_test_suite!(
        EthereumTokenTransfers,
        crate::mock::new_test_ext(),
        crate::mock::Test,
    );
}
