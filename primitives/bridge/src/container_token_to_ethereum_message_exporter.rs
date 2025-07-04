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

use {
    core::{marker::PhantomData, slice::Iter},
    frame_support::{ensure, traits::Get},
    parity_scale_codec::{Decode, Encode},
    snowbridge_core::{
        outbound::{Command, Message, SendMessage},
        ChannelId, TokenId, TokenIdOf,
    },
    sp_core::{H160, H256},
    sp_runtime::traits::MaybeEquivalence,
    sp_std::{iter::Peekable, prelude::*},
    xcm::latest::SendError::{MissingArgument, NotApplicable, Unroutable},
    xcm::prelude::*,
    xcm_executor::traits::{ConvertLocation, ExportXcm},
};

pub struct EthereumBlobExporter<
    UniversalLocation,
    EthereumNetwork,
    OutboundQueue,
    AgentHashedDescription,
    ConvertAssetId,
    BridgeChannelId,
>(
    PhantomData<(
        UniversalLocation,
        EthereumNetwork,
        OutboundQueue,
        AgentHashedDescription,
        ConvertAssetId,
        BridgeChannelId,
    )>,
);

impl<
        UniversalLocation,
        EthereumNetwork,
        OutboundQueue,
        AgentHashedDescription,
        ConvertAssetId,
        BridgeChannelId,
    > ExportXcm
    for EthereumBlobExporter<
        UniversalLocation,
        EthereumNetwork,
        OutboundQueue,
        AgentHashedDescription,
        ConvertAssetId,
        BridgeChannelId,
    >
where
    UniversalLocation: Get<InteriorLocation>,
    EthereumNetwork: Get<NetworkId>,
    OutboundQueue: SendMessage<Balance = u128>,
    AgentHashedDescription: ConvertLocation<H256>,
    ConvertAssetId: MaybeEquivalence<TokenId, Location>,
    BridgeChannelId: Get<Option<ChannelId>>,
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

        log::info!("validate params: network={network:?}, _channel={_channel:?}, universal_source={universal_source:?}, destination={destination:?}, message={message:?}, ");

        if network != expected_network {
            log::trace!(target: "xcm::ethereum_blob_exporter", "skipped due to unmatched bridge network {network:?}.");
            return Err(NotApplicable);
        }

        // Cloning destination to avoid modifying the value so subsequent exporters can use it.
        let dest = destination.clone().take().ok_or(MissingArgument)?;
        if dest != Here {
            log::trace!(target: "xcm::ethereum_blob_exporter", "skipped due to unmatched remote destination {dest:?}.");
            return Err(NotApplicable);
        }

        // Cloning universal_source to avoid modifying the value so subsequent exporters can use it.
        let (local_net, local_sub) = universal_source.clone()
            .take()
            .ok_or_else(|| {
                log::error!(target: "xcm::ethereum_blob_exporter", "universal source not provided.");
                MissingArgument
            })?
            .split_global()
            .map_err(|()| {
                log::error!(target: "xcm::ethereum_blob_exporter", "could not get global consensus from universal source '{universal_source:?}'.");
                NotApplicable
            })?;

        if Ok(local_net) != universal_location.global_consensus() {
            log::trace!(target: "xcm::ethereum_blob_exporter", "skipped due to unmatched relay network {local_net:?}.");
            return Err(NotApplicable);
        }

        let _ = match local_sub.as_slice() {
            [Parachain(para_id)] => *para_id,
            _ => {
                log::error!(target: "xcm::ethereum_blob_exporter", "could not get parachain id from universal source '{local_sub:?}'.");
                return Err(NotApplicable);
            }
        };

        let message = message.take().ok_or_else(|| {
            log::error!(target: "xcm::ethereum_blob_exporter", "xcm message not provided.");
            MissingArgument
        })?;

        let mut converter = XcmConverter::<ConvertAssetId, ()>::new(&message, expected_network);
        let (command, message_id) = converter.convert().map_err(|err|{
            log::error!(target: "xcm::ethereum_blob_exporter", "unroutable due to pattern matching error '{err:?}'.");
            Unroutable
        })?;

        let channel_id = BridgeChannelId::get().ok_or_else(|| {
            log::error!(target: "xcm::ethereum_blob_exporter", "channel id cannot be fetched");
            Unroutable
        })?;

        let outbound_message = Message {
            id: Some(message_id.into()),
            channel_id,
            command,
        };

        // validate the message
        let (ticket, fee) = OutboundQueue::validate(&outbound_message).map_err(|err| {
            log::error!(target: "xcm::ethereum_blob_exporter", "OutboundQueue validation of message failed. {err:?}");
            Unroutable
        })?;

        // convert fee to Asset (specify native tokens)
        let fee = Asset::from((Location::new(0, []), fee.total())).into();

        Ok(((ticket.encode(), message_id), fee))
    }

    fn deliver(blob: (Vec<u8>, XcmHash)) -> Result<XcmHash, SendError> {
        let ticket: OutboundQueue::Ticket = OutboundQueue::Ticket::decode(&mut blob.0.as_ref())
            .map_err(|_| {
                log::trace!(target: "xcm::ethereum_blob_exporter", "undeliverable due to decoding error");
                NotApplicable
            })?;

        let message_id = OutboundQueue::deliver(ticket).map_err(|_| {
            log::error!(target: "xcm::ethereum_blob_exporter", "OutboundQueue submit of message failed");
            SendError::Transport("other transport error")
        })?;

        log::info!(target: "xcm::ethereum_blob_exporter", "message delivered {message_id:#?}.");
        Ok(message_id.into())
    }
}

/// Errors that can be thrown to the pattern matching step.
#[derive(PartialEq, Debug)]
enum XcmConverterError {
    UnexpectedEndOfXcm,
    EndOfXcmMessageExpected,
    DepositAssetExpected,
    NoReserveAssets,
    FilterDoesNotConsumeAllAssets,
    TooManyAssets,
    ZeroAssetTransfer,
    BeneficiaryResolutionFailed,
    AssetResolutionFailed,
    InvalidFeeAsset,
    SetTopicExpected,
    ReserveAssetDepositedExpected,
    InvalidAsset,
    UnexpectedInstruction,
}

macro_rules! match_expression {
	($expression:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $value:expr $(,)?) => {
		match $expression {
			$( $pattern )|+ $( if $guard )? => Some($value),
			_ => None,
		}
	};
}

struct XcmConverter<'a, ConvertAssetId, Call> {
    iter: Peekable<Iter<'a, Instruction<Call>>>,
    ethereum_network: NetworkId,
    _marker: PhantomData<ConvertAssetId>,
}
impl<'a, ConvertAssetId, Call> XcmConverter<'a, ConvertAssetId, Call>
where
    ConvertAssetId: MaybeEquivalence<TokenId, Location>,
{
    fn new(message: &'a Xcm<Call>, ethereum_network: NetworkId) -> Self {
        Self {
            iter: message.inner().iter().peekable(),
            ethereum_network,
            _marker: Default::default(),
        }
    }

    fn convert(&mut self) -> Result<(Command, [u8; 32]), XcmConverterError> {
        let result = match self.peek() {
            // Prepare the command to mint the foreign token.
            Ok(ReserveAssetDeposited { .. }) => self.make_mint_foreign_token_command(),
            Err(e) => {
                log::trace!(target: "xcm::convert", "peak error: {:?}", e);
                Err(e)
            }
            _ => return Err(XcmConverterError::UnexpectedInstruction),
        }?;

        // All xcm instructions must be consumed before exit.
        if self.next().is_ok() {
            return Err(XcmConverterError::EndOfXcmMessageExpected);
        }

        Ok(result)
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

    /// Convert the xcm for Polkadot-native token from AH into the Command
    /// To match transfers of Polkadot-native tokens, we expect an input of the form:
    /// # ReserveAssetDeposited
    /// # ClearOrigin
    /// # BuyExecution
    /// # DepositAsset
    /// # SetTopic
    fn make_mint_foreign_token_command(
        &mut self,
    ) -> Result<(Command, [u8; 32]), XcmConverterError> {
        use XcmConverterError::*;

        // Get the reserve assets.
        let reserve_assets = match_expression!(
            self.next()?,
            ReserveAssetDeposited(reserve_assets),
            reserve_assets
        )
        .ok_or(ReserveAssetDepositedExpected)?;

        // Check if clear origin exists and skip over it.
        if match_expression!(self.peek(), Ok(ClearOrigin), ()).is_some() {
            let _ = self.next();
        }

        // Get the fee asset item from BuyExecution or continue parsing.
        let fee_asset = match_expression!(self.peek(), Ok(BuyExecution { fees, .. }), fees);
        if fee_asset.is_some() {
            let _ = self.next();
        }

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

        // Fees are collected on AH, up front and directly from the user, to cover the
        // complete cost of the transfer. Any additional fees provided in the XCM program are
        // refunded to the beneficiary. We only validate the fee here if its provided to make sure
        // the XCM program is well formed. Another way to think about this from an XCM perspective
        // would be that the user offered to pay X amount in fees, but we charge 0 of that X amount
        // (no fee) and refund X to the user.
        if let Some(fee_asset) = fee_asset {
            // The fee asset must be the same as the reserve asset.
            if fee_asset.id != reserve_asset.id || fee_asset.fun > reserve_asset.fun {
                return Err(InvalidFeeAsset);
            }
        }

        let (asset_id, amount) = match reserve_asset {
            Asset {
                id: AssetId(inner_location),
                fun: Fungible(amount),
            } => Some((inner_location.clone(), *amount)),
            _ => None,
        }
        .ok_or(AssetResolutionFailed)?;

        // transfer amount must be greater than 0.
        ensure!(amount > 0, ZeroAssetTransfer);

        let token_id = TokenIdOf::convert_location(&asset_id).ok_or(InvalidAsset)?;

        // let expected_asset_id = ConvertAssetId::convert(&token_id).ok_or(InvalidAsset)?;

        // ensure!(asset_id == expected_asset_id, InvalidAsset);

        // Check if there is a SetTopic and skip over it if found.
        let topic_id = match_expression!(self.next()?, SetTopic(id), id).ok_or(SetTopicExpected)?;

        Ok((
            Command::MintForeignToken {
                token_id,
                recipient,
                amount,
            },
            *topic_id,
        ))
    }
}
