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

pub(crate) fn ethereum_token_transfers_events<T: Config>() -> Vec<crate::Event<T>> {
    frame_system::Pallet::<T>::events()
        .into_iter()
        .map(|r| r.event)
        .filter_map(|e| <T as Config>::RuntimeEvent::from(e).try_into().ok())
        .collect::<Vec<_>>()
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

        let curren_channel_info = CurrentChannelInfo::<T>::get().unwrap();

        assert_eq!(curren_channel_info.channel_id, channel_id);
        assert_eq!(curren_channel_info.agent_id, agent_id);
        assert_eq!(curren_channel_info.para_id, para_id);
        Ok(())
    }

    #[benchmark]
    fn transfer_native_token() -> Result<(), BenchmarkError> {
        let channel_id = ChannelId::new([4u8; 32]);
        let agent_id = AgentId::from([5u8; 32]);
        let para_id: ParaId = 2000u32.into();

        let channel_info = ChannelInfo {
            channel_id,
            para_id,
            agent_id,
        };

        CurrentChannelInfo::<T>::put(channel_info);

        T::BenchmarkHelper::set_up_channel(channel_id, para_id, agent_id);

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

        let expected_token_id =
            T::TokenIdFromLocation::convert_back(&T::TokenLocationReanchored::get());

        if let Some(Event::NativeTokenTransferred {
            message_id,
            channel_id: channel_id_found,
            source,
            recipient: recipient_found,
            token_id,
            amount,
            fee,
        }) = ethereum_token_transfers_events::<T>().last()
        {
            assert_eq!(*message_id, H256::default());
            assert_eq!(*channel_id_found, channel_id);
            assert_eq!(*source, caller.clone());
            assert_eq!(*recipient_found, recipient);
            assert_eq!(*token_id, expected_token_id.unwrap());
            assert_eq!(*amount, amount_transferred);

            // Check balances after the transfer
            assert_eq!(
                eth_sovereign_balance_after,
                eth_sovereign_balance_before + amount_transferred
            );

            // Convert to the proper types to operate with the fee found
            let initial_amount: <<T as Config>::Currency as Inspect<
                <T as frame_system::Config>::AccountId,
            >>::Balance = initial_amount.into();

            let amount_transferred: <<T as Config>::Currency as Inspect<
                <T as frame_system::Config>::AccountId,
            >>::Balance = amount_transferred.into();

            let caller_balance_after: <<T as Config>::Currency as Inspect<
                <T as frame_system::Config>::AccountId,
            >>::Balance = caller_balance_after.into();

            let fees_account_balance_after: <<T as Config>::Currency as Inspect<
                <T as frame_system::Config>::AccountId,
            >>::Balance = fees_account_balance_after.into();

            let fees_account_balance_before: <<T as Config>::Currency as Inspect<
                <T as frame_system::Config>::AccountId,
            >>::Balance = fees_account_balance_before.into();

            assert_eq!(
                caller_balance_after,
                initial_amount - amount_transferred - *fee
            );

            assert_eq!(
                fees_account_balance_after,
                fees_account_balance_before + *fee
            );
        } else {
            panic!("NativeTokenTransferred event not found!");
        }

        Ok(())
    }

    impl_benchmark_test_suite!(
        EthereumTokenTransfers,
        crate::mock::new_test_ext(),
        crate::mock::Test,
    );
}
