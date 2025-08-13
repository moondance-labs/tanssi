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
use core::marker::PhantomData;
use frame_support::{
    pallet_prelude::Zero,
    traits::{
        fungible::{Inspect, Mutate},
        tokens::{Fortitude, Preservation},
    },
};
use frame_system::pallet_prelude::BlockNumberFor;
use parity_scale_codec::{DecodeAll, Encode};
use snowbridge_core::Channel;
use snowbridge_pallet_inbound_queue::RewardProcessor;
use sp_core::Get;
use sp_runtime::Vec;
use sp_runtime::{
    traits::{Hash as _, MaybeEquivalence},
    DispatchError, DispatchResult,
};
use {
    snowbridge_inbound_queue_primitives::v1::{
        Command, Destination, Envelope, MessageProcessor, MessageV1, VersionedXcmMessage,
    },
    snowbridge_inbound_queue_primitives::EventProof as Message,
};

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
        // Ensure that the message is intended for the current channel, para_id and agent_id
        if let Some(channel_info) = pallet_ethereum_token_transfers::CurrentChannelInfo::<T>::get()
        {
            if envelope.channel_id != channel_info.channel_id
                || channel.para_id != channel_info.para_id
                || channel.agent_id != channel_info.agent_id
            {
                log::debug!(
                    "Unexpected channel id: {:?} != {:?}",
                    (envelope.channel_id, channel.para_id, channel.agent_id),
                    (
                        channel_info.channel_id,
                        channel_info.para_id,
                        channel_info.agent_id
                    )
                );
                return false;
            }
        } else {
            log::warn!("CurrentChannelInfo not set in storage");
            return false;
        }

        // Check it is from the right gateway
        if envelope.gateway != T::GatewayAddress::get() {
            log::warn!("Wrong gateway address: {:?}", envelope.gateway);
            return false;
        }

        // Try decode the message and check the token id is the expected one
        match VersionedXcmMessage::decode_all(&mut envelope.payload.as_slice()) {
            Ok(VersionedXcmMessage::V1(MessageV1 {
                command: Command::SendNativeToken { token_id, .. },
                ..
            })) => {
                let token_location = T::TokenLocationReanchored::get();

                if let Some(expected_token_id) =
                    snowbridge_pallet_system::Pallet::<T>::convert_back(&token_location)
                {
                    if token_id == expected_token_id {
                        true
                    } else {
                        // TODO: ensure this does not warn on container token transfers or other message types, if yes change to debug
                        log::warn!(
                            "NativeTokenTransferMessageProcessor: unexpected token_id: {:?}",
                            token_id
                        );
                        false
                    }
                } else {
                    log::warn!("NativeTokenTransferMessageProcessor: token id not found for location: {:?}", token_location);

                    false
                }
            }
            Ok(msg) => {
                log::trace!(
                    "NativeTokenTransferMessageProcessor: unexpected message: {:?}",
                    msg
                );
                false
            }
            Err(e) => {
                log::trace!("NativeTokenTransferMessageProcessor: failed to decode message. This is expected if the message is not for this processor. Error: {:?}", e);
                false
            }
        }
    }

    fn process_message(_channel: Channel, envelope: Envelope) -> DispatchResult {
        // - Decode payload as SendNativeToken
        let message = VersionedXcmMessage::decode_all(&mut envelope.payload.as_slice())
        .map_err(|e| {
            log::trace!("NativeTokenTransferMessageProcessor: failed to decode message. This is expected if the message is not for this processor. Error: {:?}", e);

            DispatchError::Other("unable to parse the envelope payload")
        })?;

        log::trace!("NativeTokenTransferMessageProcessor: {:?}", message);

        match message {
            VersionedXcmMessage::V1(MessageV1 {
                chain_id: _,
                command:
                    Command::SendNativeToken {
                        destination:
                            Destination::AccountId32 {
                                id: destination_account,
                            },
                        amount,
                        ..
                    },
            }) => {
                // - Transfer the amounts of tokens from Ethereum sov account to the destination
                let sovereign_account = T::EthereumSovereignAccount::get();

                if let Err(e) = T::Currency::transfer(
                    &sovereign_account,
                    &destination_account.into(),
                    amount.into(),
                    Preservation::Preserve,
                ) {
                    log::warn!(
                        "NativeTokenTransferMessageProcessor: Error transferring tokens: {:?}",
                        e
                    );
                }

                Ok(())
            }
            msg => {
                log::warn!(
                    "NativeTokenTransferMessageProcessor: unexpected message: {:?}",
                    msg
                );
                Ok(())
            }
        }
    }
}

/// Rewards the relayer that processed a native token transfer message
/// using the FeesAccount configured in pallet_ethereum_token_transfers
pub struct RewardThroughFeesAccount<T>(PhantomData<T>);

impl<T> RewardProcessor<T> for RewardThroughFeesAccount<T>
where
    T: snowbridge_pallet_inbound_queue::Config + pallet_ethereum_token_transfers::Config,
    T::AccountId: From<sp_runtime::AccountId32>,
    <T::Token as Inspect<T::AccountId>>::Balance: core::fmt::Debug,
{
    fn process_reward(who: T::AccountId, _channel: Channel, message: Message) -> DispatchResult {
        let reward_amount = snowbridge_pallet_inbound_queue::Pallet::<T>::calculate_delivery_cost(
            message.encode().len() as u32,
        );

        let fees_account: T::AccountId = T::FeesAccount::get();

        let amount =
            T::Token::reducible_balance(&fees_account, Preservation::Preserve, Fortitude::Polite)
                .min(reward_amount);

        if amount != reward_amount {
            log::warn!(
                "RewardThroughFeesAccount: fees account running low on funds {:?}: {:?}",
                fees_account,
                amount
            );
        }

        if !amount.is_zero() {
            T::Token::transfer(&fees_account, &who, amount, Preservation::Preserve)?;
        }

        Ok(())
    }
}

pub struct BabeSlotBeacon<T>(PhantomData<T>);
impl<T: pallet_babe::Config> sp_runtime::traits::BlockNumberProvider for BabeSlotBeacon<T> {
    type BlockNumber = u32;

    fn current_block_number() -> Self::BlockNumber {
        // TODO: nimbus_primitives::SlotBeacon requires u32, but this is a u64 in pallet_babe, and
        // also it gets converted to u64 in pallet_author_noting, so let's do something to remove
        // this intermediate u32 conversion, such as using a different trait
        u64::from(pallet_babe::CurrentSlot::<T>::get()) as u32
    }
}

/// Combines the vrf output of the previous block with the provided subject.
/// This ensures that the randomness will be different on different pallets, as long as the subject is different.
pub fn mix_randomness<T: frame_system::Config>(vrf_output: [u8; 32], subject: &[u8]) -> T::Hash {
    let mut digest = Vec::new();
    digest.extend_from_slice(vrf_output.as_ref());
    digest.extend_from_slice(subject);

    T::Hashing::hash(digest.as_slice())
}

pub struct BabeAuthorVrfBlockRandomness<T>(PhantomData<T>);
impl<T: pallet_babe::Config + frame_system::Config> BabeAuthorVrfBlockRandomness<T> {
    pub fn get_block_randomness() -> Option<[u8; 32]> {
        // In a relay context we get block randomness from Babe's AuthorVrfRandomness
        pallet_babe::Pallet::<T>::author_vrf_randomness()
    }

    pub fn get_block_randomness_mixed(subject: &[u8]) -> Option<T::Hash> {
        Self::get_block_randomness().map(|random_hash| mix_randomness::<T>(random_hash, subject))
    }
}

impl<T: pallet_babe::Config + frame_system::Config>
    frame_support::traits::Randomness<T::Hash, BlockNumberFor<T>>
    for BabeAuthorVrfBlockRandomness<T>
{
    fn random(subject: &[u8]) -> (T::Hash, BlockNumberFor<T>) {
        let block_number = frame_system::Pallet::<T>::block_number();
        let randomness = Self::get_block_randomness_mixed(subject).unwrap_or_default();

        (randomness, block_number)
    }
}

pub struct BabeGetCollatorAssignmentRandomness<T>(PhantomData<T>);
impl<T: pallet_babe::Config + frame_system::Config> Get<[u8; 32]>
    for BabeGetCollatorAssignmentRandomness<T>
{
    fn get() -> [u8; 32] {
        let block_number = frame_system::Pallet::<T>::block_number();
        let random_seed = if !block_number.is_zero() {
            if let Some(random_hash) = {
                BabeAuthorVrfBlockRandomness::<T>::get_block_randomness_mixed(b"CollatorAssignment")
            } {
                // Return random_hash as a [u8; 32] instead of a Hash
                let mut buf = [0u8; 32];
                let len = core::cmp::min(32, random_hash.as_ref().len());
                buf[..len].copy_from_slice(&random_hash.as_ref()[..len]);

                buf
            } else {
                // If there is no randomness return [0; 32]
                [0; 32]
            }
        } else {
            // In block 0 (genesis) there is no randomness
            [0; 32]
        };

        random_seed
    }
}
