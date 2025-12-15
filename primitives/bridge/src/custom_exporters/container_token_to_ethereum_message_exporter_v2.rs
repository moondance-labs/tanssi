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
    alloc::vec::Vec,
    core::{iter::Peekable, marker::PhantomData, ops::ControlFlow, slice::Iter},
    frame_support::{
        ensure,
        traits::{Get, ProcessMessageError},
        BoundedVec,
    },
    parity_scale_codec::{Decode, Encode},
    snowbridge_core::TokenId,
    snowbridge_outbound_queue_primitives::v2::message::{Message, SendMessage},
    sp_runtime::traits::MaybeEquivalence,
    xcm::latest::SendError::NotApplicable,
    xcm::prelude::*,
    xcm_builder::{CreateMatcher, MatchXcm},
    xcm_executor::traits::ExportXcm,
};

#[cfg(not(feature = "runtime-benchmarks"))]
use {
    crate::{match_expression, XcmConverterError},
    snowbridge_outbound_queue_primitives::v2::message::Command,
    sp_core::H160,
    xcm_executor::traits::ConvertLocation,
};

// In case of error, we will always return NotApplicable.
// If we return other kind of errors, the tuple implementation for ExportXcm will not
// allow us to go to the next exporter available.
pub struct ContainerEthereumBlobExporterV2<
    UniversalLocation,
    EthereumNetwork,
    EthereumLocation,
    OutboundQueue,
    ConvertAssetId,
    MinReward,
>(
    PhantomData<(
        UniversalLocation,
        EthereumNetwork,
        EthereumLocation,
        OutboundQueue,
        ConvertAssetId,
        MinReward,
    )>,
);

impl<
        UniversalLocation,
        EthereumNetwork,
        EthereumLocation,
        OutboundQueue,
        ConvertAssetId,
        MinReward,
    > ExportXcm
    for ContainerEthereumBlobExporterV2<
        UniversalLocation,
        EthereumNetwork,
        EthereumLocation,
        OutboundQueue,
        ConvertAssetId,
        MinReward,
    >
where
    UniversalLocation: Get<InteriorLocation>,
    EthereumNetwork: Get<NetworkId>,
    EthereumLocation: Get<Location>,
    OutboundQueue: SendMessage,
    ConvertAssetId: MaybeEquivalence<TokenId, Location>,
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

        log::trace!("validate params: network={network:?}, universal_source={universal_source:?}, destination={destination:?}, message={message:?}");

        if network != expected_network {
            log::trace!(target: "xcm::ethereum_blob_exporter", "skipped due to unmatched bridge network {network:?}.");
            return Err(NotApplicable);
        }

        // Cloning destination to avoid modifying the value so subsequent exporters can use it.
        let dest = destination.clone().ok_or(NotApplicable)?;
        if dest != Here {
            log::trace!(target: "xcm::ethereum_blob_exporter", "skipped due to unmatched remote destination {dest:?}.");
            return Err(NotApplicable);
        }

        // Cloning universal_source to avoid modifying the value so subsequent exporters can use it.
        let (local_net, local_sub) = universal_source.clone()
            .ok_or_else(|| {
                log::error!(target: "xcm::ethereum_blob_exporter", "universal source not provided.");
                NotApplicable
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

        let para_id = match local_sub.as_slice() {
            [Parachain(para_id)] => *para_id,
            _ => {
                log::error!(target: "xcm::ethereum_blob_exporter", "could not get parachain id from universal source '{local_sub:?}'.");
                return Err(NotApplicable);
            }
        };

        let message = message.clone().ok_or_else(|| {
            log::error!(target: "xcm::ethereum_blob_exporter", "xcm message not provided.");
            NotApplicable
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
            log::error!(target: "xcm::ethereum_blob_exporter", "failed to reanchor MinReward to destination view. {err:?}");
            NotApplicable
        })?;

        let mut converter =
            XcmConverterV2::<ConvertAssetId, UniversalLocation, EthereumLocation, ()>::new(
                &message,
                expected_network,
                para_id,
                min_reward_destination_view,
            );

        let outbound_message = converter.convert().map_err(|err|{
            log::error!(target: "xcm::ethereum_blob_exporter", "NotApplicable due to pattern matching error '{err:?}'.");
            NotApplicable
        })?;

        // validate the message
        let ticket = OutboundQueue::validate(&outbound_message).map_err(|err| {
            log::error!(target: "xcm::ethereum_blob_exporter", "OutboundQueue validation of message failed. {err:?}");
            NotApplicable
        })?;

        // convert fee to Asset
        let fee = Asset::from((MinReward::get().id, outbound_message.fee)).into();

        Ok(((ticket.encode(), XcmHash::from(outbound_message.id)), fee))
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

#[cfg(feature = "runtime-benchmarks")]
struct XcmConverterV2<'a, ConvertAssetId, UniversalLocation, EthereumLocation, Call> {
    iter: Peekable<Iter<'a, Instruction<Call>>>,
    _ethereum_network: NetworkId,
    _para_id: u32,
    _min_reward: Asset,
    _marker: PhantomData<(ConvertAssetId, UniversalLocation, EthereumLocation)>,
}
#[cfg(feature = "runtime-benchmarks")]
impl<'a, ConvertAssetId, UniversalLocation, EthereumLocation, Call>
    XcmConverterV2<'a, ConvertAssetId, UniversalLocation, EthereumLocation, Call>
where
    ConvertAssetId: MaybeEquivalence<TokenId, Location>,
    UniversalLocation: Get<InteriorLocation>,
    EthereumLocation: Get<Location>,
{
    fn new(
        message: &'a Xcm<Call>,
        _ethereum_network: NetworkId,
        _para_id: u32,
        _min_reward: Asset,
    ) -> Self {
        Self {
            iter: message.inner().iter().peekable(),
            _ethereum_network,
            _para_id,
            _min_reward,
            _marker: Default::default(),
        }
    }

    fn convert(&mut self) -> Result<Message, crate::XcmConverterError> {
        use sp_core::H256;

        if self.iter.len() == 0 {
            return Err(crate::XcmConverterError::UnexpectedEndOfXcm);
        }

        return Ok(Message {
            id: H256::zero(),
            origin: H256::zero(),
            fee: 0,
            commands: BoundedVec::new(),
        });
    }
}

#[cfg(not(feature = "runtime-benchmarks"))]
struct XcmConverterV2<'a, ConvertAssetId, UniversalLocation, EthereumLocation, Call> {
    iter: Peekable<Iter<'a, Instruction<Call>>>,
    ethereum_network: NetworkId,
    para_id: u32,
    min_reward: Asset,
    _marker: PhantomData<(ConvertAssetId, UniversalLocation, EthereumLocation)>,
}
#[cfg(not(feature = "runtime-benchmarks"))]
impl<'a, ConvertAssetId, UniversalLocation, EthereumLocation, Call>
    XcmConverterV2<'a, ConvertAssetId, UniversalLocation, EthereumLocation, Call>
where
    ConvertAssetId: MaybeEquivalence<TokenId, Location>,
    UniversalLocation: Get<InteriorLocation>,
    EthereumLocation: Get<Location>,
{
    fn new(
        message: &'a Xcm<Call>,
        ethereum_network: NetworkId,
        para_id: u32,
        min_reward: Asset,
    ) -> Self {
        Self {
            iter: message.inner().iter().peekable(),
            ethereum_network,
            para_id,
            min_reward,
            _marker: Default::default(),
        }
    }

    fn convert(&mut self) -> Result<Message, XcmConverterError> {
        let fee = self.try_extract_fee()?;
        let result = match self.peek() {
            // Prepare the command to mint the foreign token.
            Ok(ReserveAssetDeposited { .. }) => self.make_mint_foreign_token_command(fee),
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

    /// Extract the fee asset item from PayFees(V5)
    fn try_extract_fee(&mut self) -> Result<u128, XcmConverterError> {
        use XcmConverterError::*;
        let reserved_fee_assets = match_expression!(self.next()?, WithdrawAsset(fee), fee)
            .ok_or(WithdrawAssetExpected)?;
        ensure!(reserved_fee_assets.len() == 1, AssetResolutionFailed);
        let reserved_fee_asset = reserved_fee_assets
            .inner()
            .first()
            .cloned()
            .ok_or(AssetResolutionFailed)?;
        let (reserved_fee_asset_id, reserved_fee_amount) = match reserved_fee_asset {
            Asset {
                id: asset_id,
                fun: Fungible(amount),
            } => Ok((asset_id, amount)),
            _ => Err(AssetResolutionFailed),
        }?;
        let fee_asset =
            match_expression!(self.next()?, PayFees { asset: fee }, fee).ok_or(InvalidFeeAsset)?;
        let (fee_asset_id, fee_amount) = match fee_asset {
            Asset {
                id: asset_id,
                fun: Fungible(amount),
            } => Ok((asset_id, *amount)),
            _ => Err(AssetResolutionFailed),
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

    /// Convert the xcm for container-native token into the Message
    /// We expect an input of the form:
    /// # ReserveAssetDeposited
    /// # AliasOrigin
    /// # DepositAsset
    /// TODO: support Transact
    /// # SetTopic
    fn make_mint_foreign_token_command(&mut self, fee: u128) -> Result<Message, XcmConverterError> {
        use XcmConverterError::*;

        // Get the reserve assets.
        let reserve_assets = match_expression!(
            self.next()?,
            ReserveAssetDeposited(reserve_assets),
            reserve_assets
        )
        .ok_or(ReserveAssetDepositedExpected)?;

        // Make sure there are reserved assets.
        if reserve_assets.len() == 0 {
            return Err(NoReserveAssets);
        }

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

        // Check the deposit asset filter matches what was reserved.
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

        log::trace!(target: "xcm::snowbridge_v2::make_mint_foreign_token_command", "asset_id={asset_id:?}");

        // NOTE: For now we have hardcoded RelayNetwork to the DANCELIGHT_GENESIS_HASH,
        // so asset_id won't work with Starlight runtime, but after we add pallet parameters and make the
        // RelayNetwork parameter dynamic, it will work with both
        let token_id = ConvertAssetId::convert_back(&asset_id).ok_or(InvalidAsset)?;

        // Check if there is a SetTopic and skip over it if found.
        let topic_id = match_expression!(self.next()?, SetTopic(id), id).ok_or(SetTopicExpected)?;

        let mut commands: Vec<Command> = Vec::new();
        commands.push(Command::MintForeignToken {
            token_id,
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
        snowbridge_outbound_queue_primitives::{SendError, SendMessageFeeProvider},
        sp_core::H256,
    };

    parameter_types! {
        pub EthereumLocation: Location = Location::new(1, EthereumNetwork::get());
        const EthereumNetwork: NetworkId = Ethereum { chain_id: 11155111 };
        UniversalLocation: InteriorLocation = [GlobalConsensus(ByGenesis([ 152, 58, 26, 114, 80, 61, 108, 195, 99, 103, 118, 116, 126, 198, 39, 23, 43, 81, 39, 43, 244, 94, 80, 163, 85, 52, 143, 172, 182, 122, 130, 10 ]))].into();
        pub const MinV2Reward: u128 = 1u128;
        pub MinReward: Asset = (Location::here(), MinV2Reward::get()).into();
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

        fn validate(_: &Message) -> Result<Self::Ticket, SendError> {
            Ok(())
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

        fn validate(_: &Message) -> Result<Self::Ticket, SendError> {
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
        >::validate(
            network,
            channel,
            &mut universal_source,
            &mut destination,
            &mut message,
        );
        assert_eq!(result, Err(NotApplicable));

        let mut universal_source: Option<InteriorLocation> = Here.into();
        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
    fn exporter_pay_fees_with_more_than_reserved_yields_not_applicable_v2() {
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
        let fee_asset_location =
            Location::new(1, universal_source.clone().unwrap().take_first().unwrap());
        let fee_asset: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            fun: Fungible(500000000),
        }]
        .into();

        let fee_asset2: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            fun: Fungible(1000000000),
        }]
        .into();

        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let container_random_account_origin = Location::new(
            0,
            [
                Parachain(2001),
                AccountKey20 {
                    network: None,
                    key: [0; 20],
                },
            ],
        );

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                WithdrawAsset(fee_asset.clone()),
                PayFees {
                    // This will fail as the amount is more than the reserved one.
                    asset: fee_asset2.get(0).unwrap().clone(),
                },
                ReserveAssetDeposited(assets.clone()),
                AliasOrigin(container_random_account_origin.clone()),
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
    fn exporter_pay_fees_mismatch_yields_not_applicable_v2() {
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
        let fee_asset_location =
            Location::new(1, universal_source.clone().unwrap().take_first().unwrap());
        let fee_asset: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            fun: Fungible(500000000),
        }]
        .into();

        let fee_asset_location2 = Location::new(0, Here);
        let fee_asset2: Assets = vec![Asset {
            id: AssetId(fee_asset_location2.clone()),
            fun: Fungible(500000000),
        }]
        .into();

        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let container_random_account_origin = Location::new(
            0,
            [
                Parachain(2001),
                AccountKey20 {
                    network: None,
                    key: [0; 20],
                },
            ],
        );

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                WithdrawAsset(fee_asset.clone()),
                PayFees {
                    // This will fail as this asset won't match the reserved one.
                    asset: fee_asset2.get(0).unwrap().clone(),
                },
                ReserveAssetDeposited(assets.clone()),
                AliasOrigin(container_random_account_origin.clone()),
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
    fn exporter_missing_deposit_asset_yields_not_applicable_v2() {
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
        let fee_asset_location =
            Location::new(1, universal_source.clone().unwrap().take_first().unwrap());
        let fee_asset: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            fun: Fungible(500000000),
        }]
        .into();

        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let container_random_account_origin = Location::new(
            0,
            [
                Parachain(2001),
                AccountKey20 {
                    network: None,
                    key: [0; 20],
                },
            ],
        );

        let mut message: Option<Xcm<()>> = Some(
            vec![
                WithdrawAsset(fee_asset.clone()),
                PayFees {
                    asset: fee_asset.get(0).unwrap().clone(),
                },
                ReserveAssetDeposited(assets.clone()),
                AliasOrigin(container_random_account_origin.clone()),
                // DepositAsset {
                //     assets: filter,
                //     beneficiary: AccountKey20 {
                //         network: Some(Ethereum { chain_id: 11155111 }),
                //         key: beneficiary_address,
                //     }
                //     .into(),
                // },
                SetTopic([
                    57, 238, 159, 80, 83, 113, 184, 105, 108, 164, 73, 6, 134, 160, 7, 234, 121,
                    88, 234, 173, 250, 136, 18, 29, 1, 204, 109, 70, 45, 3, 160, 251,
                ]),
            ]
            .into(),
        );

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
    fn exporter_incorrect_beneficiary_yields_not_applicable_v2() {
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
        let fee_asset_location =
            Location::new(1, universal_source.clone().unwrap().take_first().unwrap());
        let fee_asset: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            fun: Fungible(500000000),
        }]
        .into();

        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let container_random_account_origin = Location::new(
            0,
            [
                Parachain(2001),
                AccountKey20 {
                    network: None,
                    key: [0; 20],
                },
            ],
        );

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                WithdrawAsset(fee_asset.clone()),
                PayFees {
                    asset: fee_asset.get(0).unwrap().clone(),
                },
                ReserveAssetDeposited(assets.clone()),
                AliasOrigin(container_random_account_origin.clone()),
                DepositAsset {
                    assets: filter,
                    beneficiary: AccountId32 {
                        // This will fail as the expected type is AccountKey20.
                        network: None,
                        id: [0; 32],
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
    fn exporter_missing_alias_origin_fails_v2() {
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
        let fee_asset_location =
            Location::new(1, universal_source.clone().unwrap().take_first().unwrap());
        let fee_asset: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            fun: Fungible(500000000),
        }]
        .into();

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
                WithdrawAsset(fee_asset.clone()),
                PayFees {
                    asset: fee_asset.get(0).unwrap().clone(),
                },
                ReserveAssetDeposited(assets.clone()),
                //AliasOrigin(container_random_account_origin.clone()),
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
    fn exporter_empty_reserve_assets_yields_not_applicable_v2() {
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
        let fee_asset_location =
            Location::new(1, universal_source.clone().unwrap().take_first().unwrap());
        let fee_asset: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            fun: Fungible(500000000),
        }]
        .into();

        let container_random_account_origin = Location::new(
            0,
            [
                Parachain(2001),
                AccountKey20 {
                    network: None,
                    key: [0; 20],
                },
            ],
        );

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                WithdrawAsset(fee_asset.clone()),
                PayFees {
                    asset: fee_asset.get(0).unwrap().clone(),
                },
                ReserveAssetDeposited(vec![].into()),
                AliasOrigin(container_random_account_origin.clone()),
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
    fn exporter_reserve_assets_more_than_one_yields_not_applicable_v2() {
        let asset_para_location: InteriorLocation = [
            GlobalConsensus(Polkadot),
            Parachain(2001),
            PalletInstance(10),
        ]
        .into();
        let asset_para_location2: InteriorLocation = [
            GlobalConsensus(Polkadot),
            Parachain(2000),
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
        let fee_asset_location =
            Location::new(1, universal_source.clone().unwrap().take_first().unwrap());
        let fee_asset: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            fun: Fungible(500000000),
        }]
        .into();

        let asset_location = Location::new(1, asset_para_location);
        let asset_location2 = Location::new(1, asset_para_location2);
        let assets: Assets = vec![
            Asset {
                id: AssetId(asset_location.clone()),
                fun: Fungible(123321000000000000),
            },
            Asset {
                id: AssetId(asset_location2.clone()),
                fun: Fungible(123321000000000000),
            },
        ]
        .into();

        let container_random_account_origin = Location::new(
            0,
            [
                Parachain(2001),
                AccountKey20 {
                    network: None,
                    key: [0; 20],
                },
            ],
        );

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                WithdrawAsset(fee_asset.clone()),
                PayFees {
                    asset: fee_asset.get(0).unwrap().clone(),
                },
                ReserveAssetDeposited(assets.clone()),
                AliasOrigin(container_random_account_origin.clone()),
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
    fn exporter_reserve_assets_incorrect_resolution_yields_not_applicable_v2() {
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
        let fee_asset_location =
            Location::new(1, universal_source.clone().unwrap().take_first().unwrap());
        let fee_asset: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            fun: Fungible(500000000),
        }]
        .into();

        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            // Non-fungible assets are not supported.
            fun: NonFungible([42u8; 32].into()),
        }]
        .into();

        let container_random_account_origin = Location::new(
            0,
            [
                Parachain(2001),
                AccountKey20 {
                    network: None,
                    key: [0; 20],
                },
            ],
        );

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                WithdrawAsset(fee_asset.clone()),
                PayFees {
                    asset: fee_asset.get(0).unwrap().clone(),
                },
                ReserveAssetDeposited(assets.clone()),
                AliasOrigin(container_random_account_origin.clone()),
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
    fn exporter_assets_mismatch_yields_not_applicable_v2() {
        sp_tracing::try_init_simple();
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
        let fee_asset_location =
            Location::new(1, universal_source.clone().unwrap().take_first().unwrap());
        let fee_asset: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            fun: Fungible(500000000),
        }]
        .into();

        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let asset_location1 = Location::new(1, asset_para_location1);
        let assets1: Assets = vec![Asset {
            id: AssetId(asset_location1.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let container_random_account_origin = Location::new(
            0,
            [
                Parachain(2001),
                AccountKey20 {
                    network: None,
                    key: [0; 20],
                },
            ],
        );

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        // Should fail as we are trying to deposit an asset that doesn't match what was reserved.
        let filter: AssetFilter = Definite(assets.clone());

        let mut message: Option<Xcm<()>> = Some(
            vec![
                WithdrawAsset(fee_asset.clone()),
                PayFees {
                    asset: fee_asset.get(0).unwrap().clone(),
                },
                ReserveAssetDeposited(assets1.clone()),
                AliasOrigin(container_random_account_origin.clone()),
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
    fn exporter_incorrect_asset_amount_yields_not_applicable_v2() {
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
        let fee_asset_location =
            Location::new(1, universal_source.clone().unwrap().take_first().unwrap());
        let fee_asset: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            fun: Fungible(500000000),
        }]
        .into();

        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(0),
        }]
        .into();

        let container_random_account_origin = Location::new(
            0,
            [
                Parachain(2001),
                AccountKey20 {
                    network: None,
                    key: [0; 20],
                },
            ],
        );

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                WithdrawAsset(fee_asset.clone()),
                PayFees {
                    asset: fee_asset.get(0).unwrap().clone(),
                },
                ReserveAssetDeposited(assets.clone()),
                AliasOrigin(container_random_account_origin.clone()),
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
    fn exporter_incorrect_fee_amount_yields_not_applicable_v2() {
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
        let fee_asset_location =
            Location::new(1, universal_source.clone().unwrap().take_first().unwrap());
        let fee_asset: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            // Minimum export fee is 1, so we put 0 to test the failure.
            fun: Fungible(0),
        }]
        .into();

        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let container_random_account_origin = Location::new(
            0,
            [
                Parachain(2001),
                AccountKey20 {
                    network: None,
                    key: [0; 20],
                },
            ],
        );

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                WithdrawAsset(fee_asset.clone()),
                PayFees {
                    asset: fee_asset.get(0).unwrap().clone(),
                },
                ReserveAssetDeposited(assets.clone()),
                AliasOrigin(container_random_account_origin.clone()),
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
    fn exporter_incorrect_fee_location_yields_not_applicable_v2() {
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
        // This will fail as fee_asset_location is not reanchored to Ethereum's point of view.
        let fee_asset_location = Location::new(0, Here);
        let fee_asset: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            fun: Fungible(500000000),
        }]
        .into();

        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let container_random_account_origin = Location::new(
            0,
            [
                Parachain(2001),
                AccountKey20 {
                    network: None,
                    key: [0; 20],
                },
            ],
        );

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                WithdrawAsset(fee_asset.clone()),
                PayFees {
                    asset: fee_asset.get(0).unwrap().clone(),
                },
                ReserveAssetDeposited(assets.clone()),
                AliasOrigin(container_random_account_origin.clone()),
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
    fn exporter_no_set_topic_yields_not_applicable_v2() {
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
        let fee_asset_location =
            Location::new(1, universal_source.clone().unwrap().take_first().unwrap());
        let fee_asset: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            fun: Fungible(500000000),
        }]
        .into();

        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let container_random_account_origin = Location::new(
            0,
            [
                Parachain(2001),
                AccountKey20 {
                    network: None,
                    key: [0; 20],
                },
            ],
        );

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                WithdrawAsset(fee_asset.clone()),
                PayFees {
                    asset: fee_asset.get(0).unwrap().clone(),
                },
                ReserveAssetDeposited(assets.clone()),
                AliasOrigin(container_random_account_origin.clone()),
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
    fn exporter_extra_instruction_yields_not_applicable_v2() {
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
        let fee_asset_location =
            Location::new(1, universal_source.clone().unwrap().take_first().unwrap());
        let fee_asset: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            fun: Fungible(500000000),
        }]
        .into();

        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let container_random_account_origin = Location::new(
            0,
            [
                Parachain(2001),
                AccountKey20 {
                    network: None,
                    key: [0; 20],
                },
            ],
        );

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                WithdrawAsset(fee_asset.clone()),
                PayFees {
                    asset: fee_asset.get(0).unwrap().clone(),
                },
                ReserveAssetDeposited(assets.clone()),
                AliasOrigin(container_random_account_origin.clone()),
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
    fn exporter_bridge_success_v2() {
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
        let fee_asset_location =
            Location::new(1, universal_source.clone().unwrap().take_first().unwrap());
        let fee_asset: Assets = vec![Asset {
            id: AssetId(fee_asset_location.clone()),
            fun: Fungible(500000000),
        }]
        .into();

        let asset_location = Location::new(1, asset_para_location);
        let assets: Assets = vec![Asset {
            id: AssetId(asset_location.clone()),
            fun: Fungible(123321000000000000),
        }]
        .into();

        let container_random_account_origin = Location::new(
            0,
            [
                Parachain(2001),
                AccountKey20 {
                    network: None,
                    key: [0; 20],
                },
            ],
        );

        let beneficiary_address: [u8; 20] = hex!("2000000000000000000000000000000000000000");

        let filter: AssetFilter = Wild(WildAsset::AllCounted(1));

        let mut message: Option<Xcm<()>> = Some(
            vec![
                WithdrawAsset(fee_asset.clone()),
                PayFees {
                    asset: fee_asset.get(0).unwrap().clone(),
                },
                ReserveAssetDeposited(assets.clone()),
                AliasOrigin(container_random_account_origin.clone()),
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

        let result = ContainerEthereumBlobExporterV2::<
            UniversalLocation,
            EthereumNetwork,
            EthereumLocation,
            MockOkOutboundQueue,
            MockTokenIdConvert,
            MinReward,
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
