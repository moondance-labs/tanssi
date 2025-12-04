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

#[cfg(not(feature = "runtime-benchmarks"))]
use crate::{match_expression, XcmConverterError};
use {
    alloc::vec::Vec,
    core::{iter::Peekable, marker::PhantomData, slice::Iter},
    frame_support::{ensure, traits::Get},
    parity_scale_codec::{Decode, Encode},
    snowbridge_core::{AgentId, ChannelId, TokenId},
    snowbridge_outbound_queue_primitives::v1::message::{Command, Message, SendMessage},
    sp_core::H160,
    sp_runtime::traits::MaybeEquivalence,
    xcm::latest::SendError::{MissingArgument, NotApplicable, Unroutable},
    xcm::prelude::*,
    xcm_executor::traits::ExportXcm,
};

pub struct ContainerEthereumBlobExporter<
    UniversalLocation,
    EthereumNetwork,
    EthereumLocation,
    OutboundQueue,
    ConvertAssetId,
    BridgeChannelInfo,
>(
    PhantomData<(
        UniversalLocation,
        EthereumNetwork,
        EthereumLocation,
        OutboundQueue,
        ConvertAssetId,
        BridgeChannelInfo,
    )>,
);

impl<
        UniversalLocation,
        EthereumNetwork,
        EthereumLocation,
        OutboundQueue,
        ConvertAssetId,
        BridgeChannelInfo,
    > ExportXcm
    for ContainerEthereumBlobExporter<
        UniversalLocation,
        EthereumNetwork,
        EthereumLocation,
        OutboundQueue,
        ConvertAssetId,
        BridgeChannelInfo,
    >
where
    UniversalLocation: Get<InteriorLocation>,
    EthereumNetwork: Get<NetworkId>,
    EthereumLocation: Get<Location>,
    OutboundQueue: SendMessage<Balance = u128>,
    ConvertAssetId: MaybeEquivalence<TokenId, Location>,
    BridgeChannelInfo: Get<Option<(ChannelId, AgentId)>>,
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

        log::trace!(target: "xcm::container_ethereum_blob_exporter", "validate params: network={network:?}, _channel={_channel:?}, universal_source={universal_source:?}, destination={destination:?}, message={message:?}");

        if network != expected_network {
            log::trace!(target: "xcm::container_ethereum_blob_exporter", "skipped due to unmatched bridge network {network:?}.");
            return Err(NotApplicable);
        }

        // Cloning destination to avoid modifying the value so subsequent exporters can use it.
        let dest = destination.clone().ok_or(MissingArgument)?;
        if dest != Here {
            log::trace!(target: "xcm::container_ethereum_blob_exporter", "skipped due to unmatched remote destination {dest:?}.");
            return Err(NotApplicable);
        }

        // Cloning universal_source to avoid modifying the value so subsequent exporters can use it.
        let (local_net, local_sub) = universal_source.clone()
            .ok_or_else(|| {
                log::error!(target: "xcm::container_ethereum_blob_exporter", "universal source not provided.");
                MissingArgument
            })?
            .split_global()
            .map_err(|()| {
                log::error!(target: "xcm::container_ethereum_blob_exporter", "could not get global consensus from universal source '{universal_source:?}'.");
                NotApplicable
            })?;

        if Ok(local_net) != universal_location.global_consensus() {
            log::trace!(target: "xcm::container_ethereum_blob_exporter", "skipped due to unmatched relay network {local_net:?}.");
            return Err(NotApplicable);
        }

        let para_id = match local_sub.as_slice() {
            [Parachain(para_id)] => *para_id,
            _ => {
                log::error!(target: "xcm::container_ethereum_blob_exporter", "could not get parachain id from universal source '{local_sub:?}'.");
                return Err(NotApplicable);
            }
        };

        let message = message.take().ok_or_else(|| {
            log::error!(target: "xcm::container_ethereum_blob_exporter", "xcm message not provided.");
            MissingArgument
        })?;

        let mut converter =
            XcmConverter::<ConvertAssetId, UniversalLocation, EthereumLocation, ()>::new(
                &message,
                expected_network,
                para_id,
            );
        let (command, message_id) = converter.convert().map_err(|err|{
            log::error!(target: "xcm::container_ethereum_blob_exporter", "unroutable due to pattern matching error '{err:?}'.");
            Unroutable
        })?;

        let (channel_id, _) = BridgeChannelInfo::get().ok_or_else(|| {
            log::error!(target: "xcm::container_ethereum_blob_exporter", "channel id and agent id cannot be fetched");
            Unroutable
        })?;

        let outbound_message = Message {
            id: Some(message_id.into()),
            channel_id,
            command,
        };

        // validate the message
        let (ticket, fee) = OutboundQueue::validate(&outbound_message).map_err(|err| {
            log::error!(target: "xcm::container_ethereum_blob_exporter", "OutboundQueue validation of message failed. {err:?}");
            Unroutable
        })?;

        // convert fee to Asset (specify native tokens)
        let fee = Asset::from((Location::new(0, []), fee.total())).into();

        Ok(((ticket.encode(), message_id), fee))
    }

    fn deliver(blob: (Vec<u8>, XcmHash)) -> Result<XcmHash, SendError> {
        let ticket: OutboundQueue::Ticket = OutboundQueue::Ticket::decode(&mut blob.0.as_ref())
            .map_err(|_| {
                log::trace!(target: "xcm::container_ethereum_blob_exporter", "undeliverable due to decoding error");
                NotApplicable
            })?;

        let message_id = OutboundQueue::deliver(ticket).map_err(|_| {
            log::error!(target: "xcm::container_ethereum_blob_exporter", "OutboundQueue submit of message failed");
            SendError::Transport("other transport error")
        })?;

        log::info!(target: "xcm::container_ethereum_blob_exporter", "message delivered {message_id:#?}.");
        Ok(message_id.into())
    }
}

#[cfg(feature = "runtime-benchmarks")]
struct XcmConverter<'a, ConvertAssetId, UniversalLocation, EthereumLocation, Call> {
    iter: Peekable<Iter<'a, Instruction<Call>>>,
    _ethereum_network: NetworkId,
    _para_id: u32,
    _marker: PhantomData<(ConvertAssetId, UniversalLocation, EthereumLocation)>,
}
#[cfg(feature = "runtime-benchmarks")]
impl<'a, ConvertAssetId, UniversalLocation, EthereumLocation, Call>
    XcmConverter<'a, ConvertAssetId, UniversalLocation, EthereumLocation, Call>
where
    ConvertAssetId: MaybeEquivalence<TokenId, Location>,
    UniversalLocation: Get<InteriorLocation>,
    EthereumLocation: Get<Location>,
{
    fn new(message: &'a Xcm<Call>, _ethereum_network: NetworkId, _para_id: u32) -> Self {
        Self {
            iter: message.inner().iter().peekable(),
            _ethereum_network,
            _para_id,
            _marker: Default::default(),
        }
    }

    fn convert(&mut self) -> Result<(Command, [u8; 32]), sp_runtime::DispatchError> {
        ensure!(self.iter.len() > 0, "Should have at least one instruction");

        return Ok((
            Command::MintForeignToken {
                token_id: Default::default(),
                recipient: H160::repeat_byte(0),
                amount: 0,
            },
            [0u8; 32],
        ));
    }
}

#[cfg(not(feature = "runtime-benchmarks"))]
struct XcmConverter<'a, ConvertAssetId, UniversalLocation, EthereumLocation, Call> {
    iter: Peekable<Iter<'a, Instruction<Call>>>,
    ethereum_network: NetworkId,
    para_id: u32,
    _marker: PhantomData<(ConvertAssetId, UniversalLocation, EthereumLocation)>,
}
#[cfg(not(feature = "runtime-benchmarks"))]
impl<'a, ConvertAssetId, UniversalLocation, EthereumLocation, Call>
    XcmConverter<'a, ConvertAssetId, UniversalLocation, EthereumLocation, Call>
where
    ConvertAssetId: MaybeEquivalence<TokenId, Location>,
    UniversalLocation: Get<InteriorLocation>,
    EthereumLocation: Get<Location>,
{
    fn new(message: &'a Xcm<Call>, ethereum_network: NetworkId, para_id: u32) -> Self {
        Self {
            iter: message.inner().iter().peekable(),
            ethereum_network,
            para_id,
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

    fn check_reserve_asset_para_id(&self, location: &Location) -> Result<(), ()> {
        if location.parents != 1 {
            return Err(());
        }

        match &location.interior {
            Junctions::X3(arc) => {
                let [j1, j2, _] = arc.as_ref();
                if let GlobalConsensus(_) = j1 {
                    if let Parachain(id) = j2 {
                        if *id == self.para_id {
                            return Ok(());
                        }
                    }
                }
                Err(())
            }
            _ => Err(()),
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

        // Check if clear origin exists
        match_expression!(self.next(), Ok(ClearOrigin), ()).ok_or(ClearOriginExpected)?;

        // Get the fee asset item from BuyExecution
        let fee_asset = match_expression!(self.next(), Ok(BuyExecution { fees, .. }), fees)
            .ok_or(BuyExecutionExpected)?;

        let (deposit_assets, beneficiary) = match self.next()? {
            Instruction::DepositAsset {
                assets,
                beneficiary,
            } => (assets, beneficiary),
            _ => return Err(DepositAssetExpected),
        };

        // assert that the beneficiary is AccountKey20.
        let (parents, interior) = beneficiary.unpack();

        let recipient = match (parents, interior) {
            (0, [AccountKey20 { network, key }]) if self.network_matches(network) => H160(*key),
            _ => return Err(BeneficiaryResolutionFailed),
        };

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
        if fee_asset.id != reserve_asset.id || fee_asset.fun > reserve_asset.fun {
            return Err(InvalidFeeAsset);
        }

        let (asset_id, amount) = match reserve_asset {
            Asset {
                id: AssetId(inner_location),
                fun: Fungible(amount),
            } => Some((inner_location.clone(), *amount)),
            _ => None,
        }
        .ok_or(AssetResolutionFailed)?;

        self.check_reserve_asset_para_id(&asset_id)
            .map_err(|_| ParaIdMismatch)?;

        // transfer amount must be greater than 0.
        ensure!(amount > 0, ZeroAssetTransfer);

        log::trace!(target: "xcm::make_mint_foreign_token_command", "asset_id={asset_id:?}");

        // NOTE: For now we have hardcoded RelayNetwork to the DANCELIGHT_GENESIS_HASH,
        // so asset_id won't work with Starlight runtime, but after we add pallet parameters and make the
        // RelayNetwork parameter dynamic, it will work with both
        let token_id = ConvertAssetId::convert_back(&asset_id).ok_or(InvalidAsset)?;

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

#[cfg(feature = "runtime-benchmarks")]
pub struct ToEthDeliveryHelper<XcmConfig, ExistentialDeposit, PriceForDelivery>(
    core::marker::PhantomData<(XcmConfig, ExistentialDeposit, PriceForDelivery)>,
);

#[cfg(feature = "runtime-benchmarks")]
impl<
        XcmConfig: xcm_executor::Config,
        ExistentialDeposit: Get<Option<Asset>>,
        PriceForDelivery: Get<u128>,
    > xcm_builder::EnsureDelivery
    for ToEthDeliveryHelper<XcmConfig, ExistentialDeposit, PriceForDelivery>
{
    fn ensure_successful_delivery(
        origin_ref: &Location,
        dest: &Location,
        fee_reason: xcm_executor::traits::FeeReason,
    ) -> (Option<xcm_executor::FeesMode>, Option<Assets>) {
        log::trace!(target: "xcm::delivery_helper",
            "ensure_successful_delivery params: {origin_ref:?} {dest:?} {fee_reason:?} "
        );

        use xcm_executor::{
            traits::{FeeManager, TransactAsset},
            FeesMode,
        };

        if !dest.is_here() {
            log::trace!(target: "xcm::delivery_helper",
                "skipped due to unmatched remote destination {dest:?}."
            );
            return (None, None);
        }

        let mut fees_mode = None;
        if !XcmConfig::FeeManager::is_waived(Some(origin_ref), fee_reason) {
            // if not waived, we need to set up accounts for paying and receiving fees

            // overestimate delivery fee
            let overestimated_fees = PriceForDelivery::get();
            log::debug!(target: "xcm::delivery_helper", "fees to deposit {overestimated_fees:?} for origin: {origin_ref:?}");

            // mint overestimated fee to origin
            XcmConfig::AssetTransactor::deposit_asset(
                &Asset {
                    id: AssetId(Location::new(0, Here)),
                    fun: Fungible(overestimated_fees),
                },
                &origin_ref,
                None,
            )
            .unwrap();

            // expected worst case - direct withdraw
            fees_mode = Some(FeesMode { jit_withdraw: true });
        }
        (fees_mode, None)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        frame_support::parameter_types,
        hex_literal::hex,
        snowbridge_outbound_queue_primitives::{v1::Fee, SendError, SendMessageFeeProvider},
        sp_core::H256,
    };

    parameter_types! {
        pub EthereumLocation: Location = Location::new(1, EthereumNetwork::get());
        const EthereumNetwork: NetworkId = Ethereum { chain_id: 11155111 };
        UniversalLocation: InteriorLocation = [GlobalConsensus(ByGenesis([ 152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10 ]))].into();
        const BridgeChannelInfo: Option<(ChannelId, AgentId)> = Some((ChannelId::new([1u8; 32]), H256([2u8; 32])));
    }

    pub struct MockTokenIdConvert;
    impl MaybeEquivalence<TokenId, Location> for MockTokenIdConvert {
        fn convert(_id: &TokenId) -> Option<Location> {
            Some(Location::parent())
        }
        fn convert_back(_loc: &Location) -> Option<TokenId> {
            Some(H256::from_low_u64_be(123))
        }
    }

    struct MockOkOutboundQueue;
    impl SendMessage for MockOkOutboundQueue {
        type Ticket = ();

        fn validate(_: &Message) -> Result<(Self::Ticket, Fee<Self::Balance>), SendError> {
            Ok((
                (),
                Fee {
                    local: 1,
                    remote: 1,
                },
            ))
        }

        fn deliver(_: Self::Ticket) -> Result<H256, SendError> {
            Ok(H256::zero())
        }
    }

    impl SendMessageFeeProvider for MockOkOutboundQueue {
        type Balance = u128;

        fn local_fee() -> Self::Balance {
            1
        }
    }
    struct MockErrOutboundQueue;
    impl SendMessage for MockErrOutboundQueue {
        type Ticket = ();

        fn validate(_: &Message) -> Result<(Self::Ticket, Fee<Self::Balance>), SendError> {
            Err(SendError::MessageTooLarge)
        }

        fn deliver(_: Self::Ticket) -> Result<H256, SendError> {
            Err(SendError::MessageTooLarge)
        }
    }

    impl SendMessageFeeProvider for MockErrOutboundQueue {
        type Balance = u128;

        fn local_fee() -> Self::Balance {
            1
        }
    }

    #[test]
    fn exporter_validate_with_unknown_network_yields_not_applicable() {
        let network = Ethereum { chain_id: 12345 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = None;
        let mut destination: Option<InteriorLocation> = None;
        let mut message: Option<Xcm<()>> = None;

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(NotApplicable));
    }

    #[test]
    fn exporter_validate_with_empty_destination_yields_missing_argument() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = None;
        let mut destination: Option<InteriorLocation> = None;
        let mut message: Option<Xcm<()>> = None;

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(MissingArgument));
    }

    #[test]
    fn exporter_validate_with_incorrect_destination_yields_not_applicable() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = None;
        let mut destination: Option<InteriorLocation> = Some(
            [
                OnlyChild, OnlyChild, OnlyChild, OnlyChild, OnlyChild, OnlyChild, OnlyChild,
                OnlyChild,
            ]
            .into(),
        );
        let mut message: Option<Xcm<()>> = None;

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(NotApplicable));
    }

    #[test]
    fn exporter_validate_with_incorrect_universal_source_yields_validation_error() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = None;
        let mut destination: Option<InteriorLocation> = Here.into();
        let mut message: Option<Xcm<()>> = None;

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(MissingArgument));

        let mut universal_source: Option<InteriorLocation> = Here.into();
        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(NotApplicable));
    }

    #[test]
    fn exporter_validate_with_missing_para_id_universal_source_yields_validation_error() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let mut message: Option<Xcm<()>> = None;

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(NotApplicable));
    }

    #[test]
    fn exporter_validate_with_empty_message_yields_missing_argument() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let mut message: Option<Xcm<()>> = None;

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(MissingArgument));
    }

    #[test]
    fn exporter_incorrect_message_yields_incorrect_instruction() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let mut message: Option<Xcm<()>> = Some(vec![SetTopic([0; 32])].into());

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(Unroutable));
    }

    #[test]
    fn exporter_incorrect_clear_origin_yields_incorrect_instruction() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let asset_location = Location::new(
            1,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let mut message: Option<Xcm<()>> = Some(
            vec![
                ReserveAssetDeposited(assets.clone()),
                BuyExecution {
                    fees: assets.get(0).unwrap().clone(),
                    weight_limit: Unlimited,
                },
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(Unroutable));
    }

    #[test]
    fn exporter_incorrect_buy_execution_yields_unroutable() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let asset_location = Location::new(
            1,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let mut message: Option<Xcm<()>> = Some(
            vec![
                ReserveAssetDeposited(assets.clone()),
                ClearOrigin,
                ClearOrigin,
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(Unroutable));
    }

    #[test]
    fn exporter_lack_of_deposit_asset_yields_unroutable() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let asset_location = Location::new(
            1,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let mut message: Option<Xcm<()>> = Some(
            vec![
                ReserveAssetDeposited(assets.clone()),
                ClearOrigin,
                BuyExecution {
                    fees: assets.get(0).unwrap().clone(),
                    weight_limit: Unlimited,
                },
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(Unroutable));
    }

    #[test]
    fn exporter_incorrect_beneficiary_yields_unroutable() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let asset_location = Location::new(
            1,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");
        let filter: AssetFilter = assets.clone().into();
        let mut message: Option<Xcm<()>> = Some(
            vec![
                ReserveAssetDeposited(assets.clone()),
                ClearOrigin,
                BuyExecution {
                    fees: assets.get(0).unwrap().clone(),
                    weight_limit: Unlimited,
                },
                DepositAsset {
                    assets: filter,
                    beneficiary: AccountKey20 {
                        network: None,
                        key: beneficiary_address,
                    }
                    .into(),
                },
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(Unroutable));
    }

    #[test]
    fn exporter_empty_reserve_assets_yields_unroutable() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let asset_location = Location::new(
            1,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");
        let filter: AssetFilter = assets.clone().into();
        let mut message: Option<Xcm<()>> = Some(
            vec![
                ReserveAssetDeposited(vec![].into()),
                ClearOrigin,
                BuyExecution {
                    fees: assets.get(0).unwrap().clone(),
                    weight_limit: Unlimited,
                },
                DepositAsset {
                    assets: filter,
                    beneficiary: AccountKey20 {
                        network: Some(Ethereum { chain_id: 11155111 }),
                        key: beneficiary_address,
                    }
                    .into(),
                },
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(Unroutable));
    }

    #[test]
    fn exporter_reserve_not_match_deposit_yields_unroutable() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let asset_location = Location::new(
            1,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");
        let assets1: Assets = vec![Asset {
            id: AssetId(asset_location),
            fun: Fungible(999),
        }]
        .into();
        let filter: AssetFilter = assets1.clone().into();
        let mut message: Option<Xcm<()>> = Some(
            vec![
                ReserveAssetDeposited(assets.clone()),
                ClearOrigin,
                BuyExecution {
                    fees: assets.get(0).unwrap().clone(),
                    weight_limit: Unlimited,
                },
                DepositAsset {
                    assets: filter,
                    beneficiary: AccountKey20 {
                        network: Some(Ethereum { chain_id: 11155111 }),
                        key: beneficiary_address,
                    }
                    .into(),
                },
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(Unroutable));
    }

    #[test]
    fn exporter_reserve_assets_more_then_one_yields_unroutable() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let asset_location = Location::new(
            1,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets: Assets = vec![
            Asset {
                id: AssetId(asset_location.clone()),
                fun: Fungible(123321000000000000),
            },
            Asset {
                id: AssetId(asset_location.clone()),
                fun: Fungible(123321000000000000),
            },
        ]
        .into();

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                ReserveAssetDeposited(assets.clone()),
                ClearOrigin,
                BuyExecution {
                    fees: assets.get(0).unwrap().clone(),
                    weight_limit: Unlimited,
                },
                DepositAsset {
                    assets: filter,
                    beneficiary: AccountKey20 {
                        network: Some(Ethereum { chain_id: 11155111 }),
                        key: beneficiary_address,
                    }
                    .into(),
                },
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(Unroutable));
    }

    #[test]
    fn exporter_fee_id_does_not_equal_to_reserve_id_yields_unroutable() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let asset_location = Location::new(
            1,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let asset_location1 = Location::new(
            2,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets1: Assets = vec![Asset {
            id: AssetId(asset_location1.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                ReserveAssetDeposited(assets.clone()),
                ClearOrigin,
                BuyExecution {
                    fees: assets1.get(0).unwrap().clone(),
                    weight_limit: Unlimited,
                },
                DepositAsset {
                    assets: filter,
                    beneficiary: AccountKey20 {
                        network: Some(Ethereum { chain_id: 11155111 }),
                        key: beneficiary_address,
                    }
                    .into(),
                },
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(Unroutable));
    }

    #[test]
    fn exporter_fee_amount_greater_then_reserve_amount_yields_unroutable() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let asset_location = Location::new(
            1,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let asset_location1 = Location::new(
            1,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets1: Assets = vec![Asset {
            id: AssetId(asset_location1.clone()),
            fun: Fungible(123321000000000001),
        }]
        .into();

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                ReserveAssetDeposited(assets.clone()),
                ClearOrigin,
                BuyExecution {
                    fees: assets1.get(0).unwrap().clone(),
                    weight_limit: Unlimited,
                },
                DepositAsset {
                    assets: filter,
                    beneficiary: AccountKey20 {
                        network: Some(Ethereum { chain_id: 11155111 }),
                        key: beneficiary_address,
                    }
                    .into(),
                },
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(Unroutable));
    }

    #[test]
    fn exporter_reserve_assets_incorrect_resolution_yields_unroutable() {
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let asset_location = Location::new(
            1,
            [GlobalConsensus(ByGenesis([
                152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81,
                39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
            ]))],
        );
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: NonFungible([42u8; 32].into()),
        }]
        .into();

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                ReserveAssetDeposited(assets.clone()),
                ClearOrigin,
                BuyExecution {
                    fees: assets.get(0).unwrap().clone(),
                    weight_limit: Unlimited,
                },
                DepositAsset {
                    assets: filter,
                    beneficiary: AccountKey20 {
                        network: Some(Ethereum { chain_id: 11155111 }),
                        key: beneficiary_address,
                    }
                    .into(),
                },
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(Unroutable));
    }

    #[test]
    fn exporter_para_mismatch_yields_unroutable() {
        let asset_para_location: InteriorLocation = [
            GlobalConsensus(Polkadot),
            Parachain(2001),
            PalletInstance(10),
        ]
        .into();
        let asset_para_location1: InteriorLocation = [
            GlobalConsensus(Polkadot),
            Parachain(2002),
            PalletInstance(10),
        ]
        .into();
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let asset_location = Location::new(1, asset_para_location);
        let asset_location1 = Location::new(1, asset_para_location1);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let assets1: Assets = vec![Asset {
            id: AssetId(asset_location1.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                ReserveAssetDeposited(assets1.clone()),
                ClearOrigin,
                BuyExecution {
                    fees: assets.get(0).unwrap().clone(),
                    weight_limit: Unlimited,
                },
                DepositAsset {
                    assets: filter,
                    beneficiary: AccountKey20 {
                        network: Some(Ethereum { chain_id: 11155111 }),
                        key: beneficiary_address,
                    }
                    .into(),
                },
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(Unroutable));
    }

    #[test]
    fn exporter_incorrect_amount_yields_unroutable() {
        let asset_para_location: InteriorLocation = [
            GlobalConsensus(Polkadot),
            Parachain(2001),
            PalletInstance(10),
        ]
        .into();
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(0),
        }]
        .into();

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                ReserveAssetDeposited(assets.clone()),
                ClearOrigin,
                BuyExecution {
                    fees: assets.get(0).unwrap().clone(),
                    weight_limit: Unlimited,
                },
                DepositAsset {
                    assets: filter,
                    beneficiary: AccountKey20 {
                        network: Some(Ethereum { chain_id: 11155111 }),
                        key: beneficiary_address,
                    }
                    .into(),
                },
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(Unroutable));
    }

    #[test]
    fn exporter_no_set_topic_yields_unroutable() {
        let asset_para_location: InteriorLocation = [
            GlobalConsensus(Polkadot),
            Parachain(2001),
            PalletInstance(10),
        ]
        .into();
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                ReserveAssetDeposited(assets.clone()),
                ClearOrigin,
                BuyExecution {
                    fees: assets.get(0).unwrap().clone(),
                    weight_limit: Unlimited,
                },
                DepositAsset {
                    assets: filter,
                    beneficiary: AccountKey20 {
                        network: Some(Ethereum { chain_id: 11155111 }),
                        key: beneficiary_address,
                    }
                    .into(),
                },
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(Unroutable));
    }

    #[test]
    fn exporter_extra_instruction_yields_unroutable() {
        let asset_para_location: InteriorLocation = [
            GlobalConsensus(Polkadot),
            Parachain(2001),
            PalletInstance(10),
        ]
        .into();
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                ReserveAssetDeposited(assets.clone()),
                ClearOrigin,
                BuyExecution {
                    fees: assets.get(0).unwrap().clone(),
                    weight_limit: Unlimited,
                },
                DepositAsset {
                    assets: filter,
                    beneficiary: AccountKey20 {
                        network: Some(Ethereum { chain_id: 11155111 }),
                        key: beneficiary_address,
                    }
                    .into(),
                },
                SetTopic([
                    57, 238, 159, 80, 83, 113, 184, 105, 108, 164, 73, 6, 134, 160, 7, 234, 121,
                    88, 234, 173, 250, 136, 18, 29, 1, 204, 109, 70, 45, 3, 160, 251,
                ]),
                ClearOrigin,
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(Unroutable));
    }

    #[test]
    fn exporter_bridge_success() {
        let asset_para_location: InteriorLocation = [
            GlobalConsensus(Polkadot),
            Parachain(2001),
            PalletInstance(10),
        ]
        .into();
        let network = Ethereum { chain_id: 11155111 };
        let channel: u32 = 0;
        let mut universal_source: Option<InteriorLocation> = Some(
            [
                GlobalConsensus(ByGenesis([
                    152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43,
                    81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10,
                ])),
                Parachain(2001),
            ]
            .into(),
        );
        let mut destination: Option<InteriorLocation> = Here.into();
        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                ReserveAssetDeposited(assets.clone()),
                ClearOrigin,
                BuyExecution {
                    fees: assets.get(0).unwrap().clone(),
                    weight_limit: Unlimited,
                },
                DepositAsset {
                    assets: filter,
                    beneficiary: AccountKey20 {
                        network: Some(Ethereum { chain_id: 11155111 }),
                        key: beneficiary_address,
                    }
                    .into(),
                },
                SetTopic([
                    57, 238, 159, 80, 83, 113, 184, 105, 108, 164, 73, 6, 134, 160, 7, 234, 121,
                    88, 234, 173, 250, 136, 18, 29, 1, 204, 109, 70, 45, 3, 160, 251,
                ]),
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporter::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            BridgeChannelInfo,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert!(result.is_ok());
    }
}
