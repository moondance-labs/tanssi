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
use alloc::{vec, vec::Vec};
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
use sp_core::{Get, H256};
use sp_runtime::{
    traits::{Hash as _, MaybeEquivalence},
    DispatchError, DispatchResult,
};
use xcm::latest::{prelude::*, Assets as XcmAssets};
use xcm_builder::{deposit_or_burn_fee, HandleFee};
use xcm_executor::traits::{FeeReason, TransactAsset};
use {
    snowbridge_inbound_queue_primitives::v1::{
        Command, Destination, Envelope, MessageProcessor, MessageV1, VersionedXcmMessage,
    },
    snowbridge_inbound_queue_primitives::EventProof as Message,
};

#[cfg(feature = "relay")]
pub mod v1 {
    pub use crate::processors::v1::{
        EthTokensLocalProcessor, InboundTokenTransferValidator, NativeContainerTokensProcessor,
        NativeTokenTransferData, NativeTokenTransferMessageProcessor,
    };

    pub use super::RewardThroughFeesAccount;
}

#[cfg(feature = "relay")]
pub mod v2 {
    pub use crate::processors::v2::{RawMessageProcessor, SymbioticMessageProcessor};
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

/// Handler for depositing fees to the exporter fees account or a default account based on the reason.
pub struct ExporterFeeHandler<AssetTransactor, ExporterFeesAccount, DefaultAccount>(
    PhantomData<(AssetTransactor, ExporterFeesAccount, DefaultAccount)>,
);
impl<AssetTransactor, ExporterFeesAccount, DefaultAccount> HandleFee
    for ExporterFeeHandler<AssetTransactor, ExporterFeesAccount, DefaultAccount>
where
    AssetTransactor: TransactAsset,
    ExporterFeesAccount: Get<Location>,
    DefaultAccount: Get<Location>,
{
    fn handle_fee(fee: XcmAssets, context: Option<&XcmContext>, reason: FeeReason) -> XcmAssets {
        match reason {
            FeeReason::Export {
                network: _,
                destination: _,
            } => {
                deposit_or_burn_fee::<AssetTransactor>(fee, context, ExporterFeesAccount::get());
            }
            _ => {
                deposit_or_burn_fee::<AssetTransactor>(fee, context, DefaultAccount::get());
            }
        }

        XcmAssets::new()
    }
}
