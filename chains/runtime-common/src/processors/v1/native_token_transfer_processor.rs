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

//! Shared code between relay runtimes.

extern crate alloc;

use crate::processors::v1::GatewayAndChannelValidator;
use crate::processors::v1::NativeTokenTransferData;
use core::marker::PhantomData;
use frame_support::traits::{fungible::Mutate, tokens::Preservation};
use snowbridge_core::Channel;
use snowbridge_inbound_queue_primitives::v1::{Destination, Envelope, MessageProcessor};
use sp_core::Get;
use sp_runtime::{traits::MaybeEquivalence, DispatchError, DispatchResult};

/// `NativeTokenTransferMessageProcessor` is responsible for receiving and processing the Tanssi
/// native token sent from Ethereum. If the message is valid, it performs the token transfer
/// from the Ethereum sovereign account to the specified destination account.
pub struct NativeTokenTransferMessageProcessor<T>(PhantomData<T>);
impl<T> MessageProcessor for NativeTokenTransferMessageProcessor<T>
where
    T: snowbridge_pallet_inbound_queue::Config
        + pallet_ethereum_token_transfers::Config
        + snowbridge_pallet_system::Config,
    T::AccountId: From<[u8; 32]>,
{
    fn can_process_message(channel: &Channel, envelope: &Envelope) -> bool {
        if !GatewayAndChannelValidator::<T>::validate_gateway_and_channel(channel, envelope) {
            log::warn!("NativeTokenTransferMessageProcessor: invalid gateway or channel");
            return false;
        }

        // Try decode the message and check the token id is the expected one
        if let Some(token_data) =
            NativeTokenTransferData::decode_native_token_message(&envelope.payload)
        {
            let token_location = T::TokenLocationReanchored::get();

            if let Some(expected_token_id) =
                snowbridge_pallet_system::Pallet::<T>::convert_back(&token_location)
            {
                if token_data.token_id == expected_token_id {
                    true
                } else {
                    // TODO: ensure this does not warn on container token transfers or other message types, if yes change to debug
                    log::warn!(
                        "NativeTokenTransferMessageProcessor: unexpected token_id: {:?}",
                        token_data.token_id
                    );
                    false
                }
            } else {
                log::warn!(
                    "NativeTokenTransferMessageProcessor: token id not found for location: {:?}",
                    token_location
                );
                false
            }
        } else {
            false
        }
    }

    fn process_message(_channel: Channel, envelope: Envelope) -> DispatchResult {
        // Decode payload as SendNativeToken using the helper function
        if let Some(token_data) =
            NativeTokenTransferData::decode_native_token_message(&envelope.payload)
        {
            log::trace!("NativeTokenTransferMessageProcessor: processing token transfer: token_id={:?}, amount={}, destination={:?}",
                token_data.token_id, token_data.amount, token_data.destination);

            match token_data.destination {
                Destination::AccountId32 {
                    id: destination_account,
                } => {
                    // Transfer the amounts of tokens from Ethereum sov account to the destination
                    let sovereign_account = T::EthereumSovereignAccount::get();

                    if let Err(e) = T::Currency::transfer(
                        &sovereign_account,
                        &destination_account.into(),
                        token_data.amount.into(),
                        Preservation::Preserve,
                    ) {
                        log::warn!(
                            "NativeTokenTransferMessageProcessor: Error transferring tokens: {:?}",
                            e
                        );
                    }

                    Ok(())
                }
                _ => {
                    log::warn!(
                        "NativeTokenTransferMessageProcessor: unsupported destination type: {:?}",
                        token_data.destination
                    );
                    Ok(())
                }
            }
        } else {
            log::trace!("NativeTokenTransferMessageProcessor: failed to decode message. This is expected if the message is not for this processor.");
            Err(DispatchError::Other("unable to parse the envelope payload"))
        }
    }
}
