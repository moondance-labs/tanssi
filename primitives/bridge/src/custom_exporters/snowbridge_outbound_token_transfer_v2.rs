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

// TODO:
// Rewrite of the following code which cause issues as Tanssi is not a parachain
// https://github.com/moondance-labs/polkadot-sdk/blob/tanssi-polkadot-stable2412/bridges/snowbridge/primitives/router/src/outbound/mod.rs#L98

use crate::XcmConverterError;
use alloc::vec::Vec;
use core::iter::Peekable;
use core::marker::PhantomData;
use core::ops::ControlFlow;
use core::slice::Iter;
use frame_support::{
    ensure,
    traits::{Get, ProcessMessageError},
    BoundedVec,
};
use parity_scale_codec::{Decode, Encode};
use snowbridge_core::TokenId;
use snowbridge_outbound_queue_primitives::v2::message::{Command, Message, SendMessage};
use sp_core::H160;
use sp_runtime::traits::MaybeEquivalence;
use xcm::prelude::*;
use xcm_builder::{CreateMatcher, MatchXcm};
use xcm_executor::traits::{ConvertLocation, ExportXcm};

pub struct EthereumBlobExporterV2<
    UniversalLocation,
    EthereumNetwork,
    OutboundQueue,
    ConvertAssetId,
    MinReward,
>(
    PhantomData<(
        UniversalLocation,
        EthereumNetwork,
        OutboundQueue,
        ConvertAssetId,
        MinReward,
    )>,
);

impl<UniversalLocation, EthereumNetwork, OutboundQueue, ConvertAssetId, MinReward> ExportXcm
    for EthereumBlobExporterV2<
        UniversalLocation,
        EthereumNetwork,
        OutboundQueue,
        ConvertAssetId,
        MinReward,
    >
where
    UniversalLocation: Get<InteriorLocation>,
    EthereumNetwork: Get<NetworkId>,
    OutboundQueue: SendMessage,
    ConvertAssetId: MaybeEquivalence<TokenId, Location>,
    // Evaluated in XCM context!
    MinReward: Get<Asset>,
{
    type Ticket = (Vec<u8>, XcmHash);

    fn validate(
        network: NetworkId,
        _channel: u32,
        universal_source: &mut Option<InteriorLocation>,
        destination: &mut Option<InteriorLocation>,
        message: &mut Option<Xcm<()>>,
    ) -> SendResult<Self::Ticket> {
        let expected_network = EthereumNetwork::get();
        let universal_location = UniversalLocation::get();

        if network != expected_network {
            log::trace!(target: "xcm::ethereum_blob_exporterv2", "skipped due to unmatched bridge network {network:?}.");
            return Err(SendError::NotApplicable);
        }

        // Cloning destination to avoid modifying the value so subsequent exporters can use it.
        let dest = destination
            .clone()
            .take()
            .ok_or(SendError::MissingArgument)?;
        if dest != Here {
            log::trace!(target: "xcm::ethereum_blob_exporterv2", "skipped due to unmatched remote destination {dest:?}.");
            return Err(SendError::NotApplicable);
        }

        // Cloning universal_source to avoid modifying the value so subsequent exporters can use it.
        let (local_net, local_sub) = universal_source.clone()
            .take()
            .ok_or_else(|| {
                log::error!(target: "xcm::ethereum_blob_exporterv2", "universal source not provided.");
                SendError::MissingArgument
            })?
            .split_global()
            .map_err(|()| {
                log::error!(target: "xcm::ethereum_blob_exporterv2", "could not get global consensus from universal source '{universal_source:?}'.");
                SendError::NotApplicable
            })?;

        if Ok(local_net) != universal_location.global_consensus() {
            log::trace!(target: "xcm::ethereum_blob_exporterv2", "skipped due to unmatched relay network {local_net:?}.");
            return Err(SendError::NotApplicable);
        }

        // TODO: Support source being a parachain.
        if !matches!(local_sub, Junctions::Here) {
            log::trace!(target: "xcm::ethereum_blob_exporterv2", "skipped due to unmatched sub network {local_sub:?}.");
            return Err(SendError::NotApplicable);
        }

        let message = message.take().ok_or_else(|| {
            log::error!(target: "xcm::ethereum_blob_exporterv2", "xcm message not provided.");
            SendError::MissingArgument
        })?;

        // Inspect `AliasOrigin` as V2 message. This exporter should only process Snowbridge V2
        // messages. We use the presence of an `AliasOrigin` instruction to distinguish between
        // Snowbridge V2 and Snowbridge V1 messages, since XCM V5 came after Snowbridge V1 and
        // so it's not supported in Snowbridge V1. Snowbridge V1 messages are processed by the
        // snowbridge-outbound-queue-primitives v1 exporter.
        let mut instructions = message.clone().0;
        let result = instructions.matcher().match_next_inst_while(
            |_| true,
            |inst| {
                return match inst {
                    AliasOrigin(..) => Err(ProcessMessageError::Yield),
                    _ => Ok(ControlFlow::Continue(())),
                };
            },
        );
        ensure!(result.is_err(), SendError::NotApplicable);

        let dest_junction: Junction = expected_network.into();
        let dest_location = dest_junction.into_exterior(1);
        let min_reward_destination_view = MinReward::get().reanchored(&dest_location, &UniversalLocation::get()).map_err(|err| {
            log::error!(target: "xcm::ethereum_blob_exporter", "OutboundQueue validation of message failed. {err:?}");
            SendError::Unroutable
        })?;

        let mut converter = XcmConverterV2::<ConvertAssetId, ()>::new(
            &message,
            expected_network,
            min_reward_destination_view,
        );

        let outbound_message = converter.convert().map_err(|err|{
            log::error!(target: "xcm::ethereum_blob_exporter", "unroutable due to pattern matching error '{err:?}'.");
            SendError::Unroutable
        })?;

        // validate the message
        let ticket = OutboundQueue::validate(&outbound_message).map_err(|err| {
            log::error!(target: "xcm::ethereum_blob_exporter", "OutboundQueue validation of message failed. {err:?}");
            SendError::Unroutable
        })?;

        // convert fee to Asset
        let fee = Asset::from((MinReward::get().id, outbound_message.fee)).into();

        Ok(((ticket.encode(), XcmHash::from(outbound_message.id)), fee))
    }

    fn deliver(blob: (Vec<u8>, XcmHash)) -> Result<XcmHash, SendError> {
        let ticket: OutboundQueue::Ticket = OutboundQueue::Ticket::decode(&mut blob.0.as_ref())
            .map_err(|_| {
                log::trace!(target: "xcm::ethereum_blob_exporterv2", "undeliverable due to decoding error");
                SendError::NotApplicable
            })?;

        let message_id = OutboundQueue::deliver(ticket).map_err(|_| {
            log::error!(target: "xcm::ethereum_blob_exporterv2", "OutboundQueue submit of message failed");
            SendError::Transport("other transport error")
        })?;

        log::info!(target: "xcm::ethereum_blob_exporterv2", "message delivered {message_id:#?}.");
        Ok(message_id.into())
    }
}

macro_rules! match_expression {
	($expression:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $value:expr $(,)?) => {
		match $expression {
			$( $pattern )|+ $( if $guard )? => Some($value),
			_ => None,
		}
	};
}

pub struct XcmConverterV2<'a, ConvertAssetId, Call> {
    iter: Peekable<Iter<'a, Instruction<Call>>>,
    ethereum_network: NetworkId,
    min_reward: Asset,
    _marker: PhantomData<ConvertAssetId>,
}
impl<'a, ConvertAssetId, Call> XcmConverterV2<'a, ConvertAssetId, Call>
where
    ConvertAssetId: MaybeEquivalence<TokenId, Location>,
{
    pub fn new(message: &'a Xcm<Call>, ethereum_network: NetworkId, min_reward: Asset) -> Self {
        Self {
            iter: message.inner().iter().peekable(),
            ethereum_network,
            min_reward,
            _marker: Default::default(),
        }
    }

    pub fn convert(&mut self) -> Result<Message, XcmConverterError> {
        // Step 1: Try to extract optional remote fee
        // Can THIS be sent without the fee being extracted? this is a key question
        // If the answer to this is yes then we are screwed
        // We need to remember this is part of EXPORT
        // this instruction cannot be reached unless export is used
        // or is it also used in send?

        let fee = self.try_extract_fee()?;
        let result = match self.peek() {
            Ok(ReserveAssetDeposited { .. }) => self.make_mint_foreign_token_command(fee),
            // Get withdraw/deposit and make native tokens create message.
            Ok(WithdrawAsset { .. }) => self.make_unlock_native_token_command(fee),
            Err(e) => Err(e),
            _ => return Err(XcmConverterError::UnexpectedInstruction),
        }?;

        // All xcm instructions must be consumed before exit.
        if self.next().is_ok() {
            return Err(XcmConverterError::EndOfXcmMessageExpected);
        }

        Ok(result)
    }

    fn try_extract_fee(&mut self) -> Result<u128, XcmConverterError> {
        use XcmConverterError::*;

        let reserved_fee_assets = match_expression!(self.next()?, ReserveAssetDeposited(fee), fee)
            .ok_or(ReserveAssetDepositedExpected)?;
        ensure!(
            reserved_fee_assets.len() == 1,
            XcmConverterError::AssetResolutionFailed
        );

        let reserved_fee_asset = reserved_fee_assets
            .inner()
            .first()
            .cloned()
            .ok_or(XcmConverterError::AssetResolutionFailed)?;

        let (reserved_fee_asset_id, reserved_fee_amount) = match reserved_fee_asset {
            Asset {
                id: asset_id,
                fun: Fungible(amount),
            } => Ok((asset_id, amount)),
            _ => Err(XcmConverterError::AssetResolutionFailed),
        }?;

        let fee_asset =
            match_expression!(self.next()?, PayFees { asset: fee }, fee).ok_or(InvalidFeeAsset)?;

        let (fee_asset_id, fee_amount) = match fee_asset {
            Asset {
                id: asset_id,
                fun: Fungible(amount),
            } => Ok((asset_id, *amount)),
            _ => Err(XcmConverterError::AssetResolutionFailed),
        }?;

        let (min_reward_asset_id, min_reward_amount) = match &self.min_reward {
            Asset {
                id: asset_id,
                fun: Fungible(amount),
            } => Ok((asset_id, amount)),
            _ => Err(XcmConverterError::AssetResolutionFailed),
        }?;

        ensure!(fee_asset_id.0 == min_reward_asset_id.0, InvalidFeeAsset);

        ensure!(
            reserved_fee_asset_id.0 == min_reward_asset_id.0,
            InvalidFeeAsset
        );

        ensure!(reserved_fee_amount >= fee_amount, InvalidFeeAsset);

        ensure!(fee_amount >= *min_reward_amount, InvalidFeeAsset);

        Ok(fee_amount)
    }

    pub fn make_unlock_native_token_command(
        &mut self,
        fee: u128,
    ) -> Result<Message, XcmConverterError> {
        use XcmConverterError::*;

        // Get the fee asset from ReserveAssetDeposited, if any.
        // Get the reserve assets from WithdrawAsset.
        let reserve_assets =
            match_expression!(self.next()?, WithdrawAsset(reserve_assets), reserve_assets)
                .ok_or(WithdrawAssetExpected)?;

        // Check if clear origin exists and skip over it.
        if match_expression!(self.peek(), Ok(ClearOrigin), ()).is_some() {
            let _ = self.next();
        }

        // Check AliasOrigin.
        let origin_location = match_expression!(self.next()?, AliasOrigin(origin), origin)
            .ok_or(AliasOriginExpected)?;

        let origin =
            crate::TanssiAgentIdOf::convert_location(origin_location).ok_or(InvalidOrigin)?;

        let (deposit_assets, beneficiary) = match_expression!(
            self.next()?,
            DepositAsset {
                assets,
                beneficiary
            },
            (assets, beneficiary)
        )
        .ok_or(DepositAssetExpected)?;

        // assert that the beneficiary is AccountKey20.
        let recipient = match_expression!(
            beneficiary.unpack(),
            (0, [AccountKey20 { network, key }])
                if self.network_matches(network),
            H160(*key)
        )
        .ok_or(BeneficiaryResolutionFailed)?;

        // Make sure there are reserved assets.
        if reserve_assets.len() == 0 {
            return Err(NoReserveAssets);
        }

        // Check the the deposit asset filter matches what was reserved.
        if reserve_assets
            .inner()
            .iter()
            .any(|asset| !deposit_assets.matches(asset))
        {
            return Err(FilterDoesNotConsumeAllAssets);
        }

        // We only support a single asset at a time.
        ensure!(reserve_assets.len() == 1, TooManyAssets);
        let reserve_asset = reserve_assets.get(0).ok_or(AssetResolutionFailed)?;

        let (token, amount) = match reserve_asset {
            Asset {
                id: AssetId(inner_location),
                fun: Fungible(amount),
            } => match inner_location.unpack() {
                (0, [AccountKey20 { network, key }]) if self.network_matches(network) => {
                    Some((H160(*key), *amount))
                }
                // Native ETH token
                (0, []) => Some((H160::zero(), *amount)),
                _ => None,
            },
            _ => None,
        }
        .ok_or(AssetResolutionFailed)?;

        // transfer amount must be greater than 0.
        ensure!(amount > 0, ZeroAssetTransfer);

        // Check if there is a SetTopic and skip over it if found.
        let topic_id = match_expression!(self.next()?, SetTopic(id), id).ok_or(SetTopicExpected)?;

        let mut commands: Vec<Command> = Vec::new();
        commands.push(Command::UnlockNativeToken {
            token,
            recipient,
            amount,
        });

        let message = Message {
            id: (*topic_id).into(),
            origin,
            fee,
            commands: BoundedVec::try_from(commands).map_err(|_| TooManyCommands)?,
        };

        Ok(message)
    }

    fn next(&mut self) -> Result<&'a Instruction<Call>, XcmConverterError> {
        self.iter
            .next()
            .ok_or(XcmConverterError::UnexpectedEndOfXcm)
    }

    fn peek(&mut self) -> Result<&&'a Instruction<Call>, XcmConverterError> {
        self.iter
            .peek()
            .ok_or(XcmConverterError::UnexpectedEndOfXcm)
    }

    fn network_matches(&self, network: &Option<NetworkId>) -> bool {
        if let Some(network) = network {
            *network == self.ethereum_network
        } else {
            true
        }
    }

    /// Convert the xcm for Polkadot-native token from the origin chain (container chain) into the Command
    /// To match transfers of Polkadot-native tokens, we expect an input of the form:
    /// # ReserveAssetDeposited
    /// # ClearOrigin
    /// # BuyExecution
    /// # DepositAsset
    /// # SetTopic
    fn make_mint_foreign_token_command(
        &mut self,
        _fee: u128,
    ) -> Result<Message, XcmConverterError> {
        // TODO: This function will be used only when we start receiving tokens from containers.
        // The whole struct is copied from Snowbridge and modified for our needs, and thus function
        // will be modified in a latter PR.
        todo!("make_mint_foreign_token_command");

        // use XcmConverterError::*;

        // // Get the reserve assets.
        // let reserve_assets = match_expression!(
        //     self.next()?,
        //     ReserveAssetDeposited(reserve_assets),
        //     reserve_assets
        // )
        // .ok_or(ReserveAssetDepositedExpected)?;

        // // Check if clear origin exists and skip over it.
        // if match_expression!(self.peek(), Ok(ClearOrigin), ()).is_some() {
        //     let _ = self.next();
        // }

        // // Get the fee asset item from BuyExecution or continue parsing.
        // let fee_asset = match_expression!(self.peek(), Ok(BuyExecution { fees, .. }), fees);
        // if fee_asset.is_some() {
        //     let _ = self.next();
        // }

        // let (deposit_assets, beneficiary) = match_expression!(
        //     self.next()?,
        //     DepositAsset {
        //         assets,
        //         beneficiary
        //     },
        //     (assets, beneficiary)
        // )
        // .ok_or(DepositAssetExpected)?;

        // // assert that the beneficiary is AccountKey20.
        // let recipient = match_expression!(
        //     beneficiary.unpack(),
        //     (0, [AccountKey20 { network, key }])
        //         if self.network_matches(network),
        //     H160(*key)
        // )
        // .ok_or(BeneficiaryResolutionFailed)?;

        // // Make sure there are reserved assets.
        // if reserve_assets.len() == 0 {
        //     return Err(NoReserveAssets);
        // }

        // // Check the the deposit asset filter matches what was reserved.
        // if reserve_assets
        //     .inner()
        //     .iter()
        //     .any(|asset| !deposit_assets.matches(asset))
        // {
        //     return Err(FilterDoesNotConsumeAllAssets);
        // }

        // // We only support a single asset at a time.
        // ensure!(reserve_assets.len() == 1, TooManyAssets);
        // let reserve_asset = reserve_assets.get(0).ok_or(AssetResolutionFailed)?;

        // // Fees are collected on the origin chain (container chain), up front and directly from the
        // // user, to cover the complete cost of the transfer. Any additional fees provided in the XCM
        // // program are refunded to the beneficiary. We only validate the fee here if its provided to
        // // make sure the XCM program is well formed. Another way to think about this from an XCM
        // // perspective would be that the user offered to pay X amount in fees, but we charge 0 of
        // // that X amount (no fee) and refund X to the user.
        // if let Some(fee_asset) = fee_asset {
        //     // The fee asset must be the same as the reserve asset.
        //     if fee_asset.id != reserve_asset.id || fee_asset.fun > reserve_asset.fun {
        //         return Err(InvalidFeeAsset);
        //     }
        // }

        // let (asset_id, amount) = match reserve_asset {
        //     Asset {
        //         id: AssetId(inner_location),
        //         fun: Fungible(amount),
        //     } => Some((inner_location.clone(), *amount)),
        //     _ => None,
        // }
        // .ok_or(AssetResolutionFailed)?;

        // // transfer amount must be greater than 0.
        // ensure!(amount > 0, ZeroAssetTransfer);

        // let token_id = TokenIdOf::convert_location(&asset_id).ok_or(InvalidAsset)?;

        // let expected_asset_id = ConvertAssetId::convert(&token_id).ok_or(InvalidAsset)?;

        // ensure!(asset_id == expected_asset_id, InvalidAsset);

        // // Check if there is a SetTopic and skip over it if found.
        // let topic_id = match_expression!(self.next()?, SetTopic(id), id).ok_or(SetTopicExpected)?;

        // Ok((
        //     Command::MintForeignToken {
        //         token_id,
        //         recipient,
        //         amount,
        //     },
        //     *topic_id,
        // ))
    }
}
