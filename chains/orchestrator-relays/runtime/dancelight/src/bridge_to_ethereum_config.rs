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

//! The bridge to ethereum config

#[cfg(all(not(test), not(feature = "testing-helpers")))]
use crate::EthereumBeaconClient;

#[cfg(not(feature = "runtime-benchmarks"))]
use {
    tanssi_runtime_common::relay::NativeTokenTransferMessageProcessor,
    tp_bridge::{
        generic_token_message_processor::GenericTokenMessageProcessor,
        symbiotic_message_processor::SymbioticMessageProcessor,
    },
};

use {
    crate::{
        parameter_types, weights, xcm_config, AggregateMessageOrigin, Balance, Balances, Encode,
        EthereumInboundQueue, EthereumOutboundQueue, EthereumSovereignAccount, EthereumSystem,
        FixedU128, GetAggregateMessageOrigin, Keccak256, MessageQueue,
        OutboundMessageCommitmentRecorder, Runtime, RuntimeEvent, SnowbridgeFeesAccount,
        TokenLocationReanchored, TransactionByteFee, TreasuryAccount, WeightToFee, UNITS,
    },
    alloc::vec,
    core::marker::PhantomData,
    frame_support::{traits::PalletInfoAccess, weights::ConstantMultiplier},
    pallet_xcm::EnsureXcm,
    parity_scale_codec::DecodeAll,
    snowbridge_beacon_primitives::ForkVersions,
    snowbridge_core::{gwei, meth, Channel, PricingParameters, Rewards},
    snowbridge_inbound_queue_primitives::v1::{
        Command, Destination, Envelope, MessageProcessor, MessageV1, VersionedXcmMessage,
    },
    snowbridge_pallet_outbound_queue::OnNewCommitment,
    sp_core::{ConstU32, ConstU8, Get, H160, H256},
    sp_runtime::{traits::MaybeEquivalence, DispatchError, DispatchResult},
    tanssi_runtime_common::relay::{
        GatewayAndChannelValidator, NativeTokenTransferData, RewardThroughFeesAccount,
    },
    tp_bridge::{DoNothingConvertMessage, DoNothingRouter, EthereumSystemHandler},
    xcm::latest::{
        prelude::*, Asset as XcmAsset, AssetId as XcmAssetId, Assets as XcmAssets, ExecuteXcm,
        Fungibility, Junctions::*, Xcm,
    },
    xcm_executor::traits::WeightBounds,
};

pub const SLOTS_PER_EPOCH: u32 = snowbridge_pallet_ethereum_client::config::SLOTS_PER_EPOCH as u32;

// Ethereum Bridge
parameter_types! {
    pub storage EthereumGatewayAddress: H160 = H160(hex_literal::hex!("EDa338E4dC46038493b885327842fD3E301CaB39"));
}

parameter_types! {
    pub Parameters: PricingParameters<u128> = PricingParameters {
        exchange_rate: FixedU128::from_rational(1, 400),
        fee_per_gas: gwei(20),
        rewards: Rewards { local: 1 * UNITS, remote: meth(1) },
        multiplier: FixedU128::from_rational(1, 1),
    };
}

pub struct CommitmentRecorder;

impl OnNewCommitment for CommitmentRecorder {
    fn on_new_commitment(commitment: H256) {
        OutboundMessageCommitmentRecorder::record_commitment_root(commitment);
    }
}

impl pallet_outbound_message_commitment_recorder::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

// https://github.com/paritytech/polkadot-sdk/blob/2ae79be8e028a995b850621ee55f46c041eceefe/cumulus/parachains/runtimes/bridge-hubs/bridge-hub-westend/src/bridge_to_ethereum_config.rs#L105
impl snowbridge_pallet_outbound_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Hashing = Keccak256;
    type AggregateMessageOrigin = AggregateMessageOrigin;
    type GetAggregateMessageOrigin = GetAggregateMessageOrigin;
    type MessageQueue = MessageQueue;
    type Decimals = ConstU8<12>;
    type MaxMessagePayloadSize = ConstU32<2048>;
    type MaxMessagesPerBlock = ConstU32<32>;
    type GasMeter = snowbridge_outbound_queue_primitives::v1::ConstantGasMeter;
    type Balance = Balance;
    type WeightToFee = WeightToFee;
    type WeightInfo = crate::weights::snowbridge_pallet_outbound_queue::SubstrateWeight<Runtime>;
    type PricingParameters = EthereumSystem;
    type Channels = EthereumSystem;
    type OnNewCommitment = CommitmentRecorder;
}

parameter_types! {
    pub const ChainForkVersions: ForkVersions = crate::eth_chain_config::fork_versions();
}

impl snowbridge_pallet_ethereum_client::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ForkVersions = ChainForkVersions;

    type FreeHeadersInterval = ();
    type WeightInfo = weights::snowbridge_pallet_ethereum_client::SubstrateWeight<Runtime>;
}

impl snowbridge_pallet_system::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OutboundQueue = EthereumOutboundQueue;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type SiblingOrigin = EnsureXcm<frame_support::traits::Nothing>;
    #[cfg(feature = "runtime-benchmarks")]
    type SiblingOrigin = EnsureXcm<snowbridge_core::AllowSiblingsOnly>;
    type AgentIdOf = snowbridge_core::AgentIdOf;
    type TreasuryAccount = TreasuryAccount;
    type Token = Balances;
    #[cfg(feature = "runtime-benchmarks")]
    type Helper = benchmark_helper::EthSystemBenchHelper;
    type DefaultPricingParameters = Parameters;
    type InboundDeliveryCost = EthereumInboundQueue;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type UniversalLocation = xcm_config::UniversalLocation;
    #[cfg(feature = "runtime-benchmarks")]
    type UniversalLocation = xcm_config::UniversalLocationForParaIdBenchmarks;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type EthereumLocation = dancelight_runtime_constants::snowbridge::EthereumLocation;
    #[cfg(feature = "runtime-benchmarks")]
    type EthereumLocation =
        dancelight_runtime_constants::snowbridge::EthereumLocationForParaIdBenchmarks;
    type WeightInfo = crate::weights::snowbridge_pallet_system::SubstrateWeight<Runtime>;
}

impl pallet_ethereum_token_transfers::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type OutboundQueue = EthereumOutboundQueue;
    type EthereumSystemHandler = EthereumSystemHandler<Runtime>;
    type EthereumSovereignAccount = EthereumSovereignAccount;
    type FeesAccount = SnowbridgeFeesAccount;
    type TokenLocationReanchored = TokenLocationReanchored;
    type TokenIdFromLocation = EthereumSystem;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = tp_bridge::EthereumTokenTransfersBenchHelper<Runtime>;
    type WeightInfo = crate::weights::pallet_ethereum_token_transfers::SubstrateWeight<Runtime>;
}

/// `EthTokensLocalProcessor` is responsible for receiving and processing the ETH native
/// token and ERC20s coming from Ethereum with Tanssi chain or container-chains as final destinations.
/// TODO: add support for container transfers
pub struct EthTokensLocalProcessor<T, XcmProcessor, XcmWeigher, EthereumLocation, EthereumNetwork>(
    PhantomData<(
        T,
        XcmProcessor,
        XcmWeigher,
        EthereumLocation,
        EthereumNetwork,
    )>,
);
impl<T, XcmProcessor, XcmWeigher, EthereumLocation, EthereumNetwork> MessageProcessor
    for EthTokensLocalProcessor<T, XcmProcessor, XcmWeigher, EthereumLocation, EthereumNetwork>
where
    T: snowbridge_pallet_inbound_queue::Config
        + pallet_ethereum_token_transfers::Config
        + pallet_foreign_asset_creator::Config,
    XcmProcessor: ExecuteXcm<T::RuntimeCall>,
    XcmWeigher: WeightBounds<T::RuntimeCall>,
    EthereumLocation: Get<Location>,
    EthereumNetwork: Get<NetworkId>,
{
    fn can_process_message(channel: &Channel, envelope: &Envelope) -> bool {
        if !GatewayAndChannelValidator::<T>::validate_gateway_and_channel(channel, envelope) {
            log::warn!("EthTokensLocalProcessor: invalid gateway or channel");
            return false;
        }

        if let Some(eth_transfer_data) =
            Self::decode_message_for_eth_transfer(envelope.payload.as_slice())
        {
            // Check if the token location is a foreign asset included in ForeignAssetCreator
            return pallet_foreign_asset_creator::ForeignAssetToAssetId::<Runtime>::get(
                eth_transfer_data.token_location,
            )
            .is_some();
        }

        false
    }

    fn process_message(_channel: Channel, envelope: Envelope) -> DispatchResult {
        let eth_transfer_data = Self::decode_message_for_eth_transfer(envelope.payload.as_slice())
            .ok_or(DispatchError::Other("unexpected message"))?;

        match eth_transfer_data.destination {
            Destination::AccountId32 { id: _ } => {
                Self::process_xcm_local_native_eth_transfer(eth_transfer_data)
            }
            // TODO: Add support for container transfers here
            _ => {
                log::error!("EthTokensLocalProcessor: container transfers not supported yet");
                return Ok(());
            }
        }
    }
}

/// Information needed to process an eth transfer message or check its validity.
pub struct EthTransferData {
    token_location: Location,
    destination: Destination,
    amount: u128,
}

impl<T, XcmProcessor, XcmWeigher, EthereumLocation, EthereumNetwork>
    EthTokensLocalProcessor<T, XcmProcessor, XcmWeigher, EthereumLocation, EthereumNetwork>
where
    T: frame_system::Config,
    XcmProcessor: ExecuteXcm<T::RuntimeCall>,
    XcmWeigher: WeightBounds<T::RuntimeCall>,
    EthereumLocation: Get<Location>,
    EthereumNetwork: Get<NetworkId>,
{
    /// Retrieve the eth transfer data from the message payload.
    fn decode_message_for_eth_transfer(mut payload: &[u8]) -> Option<EthTransferData> {
        match VersionedXcmMessage::decode_all(&mut payload) {
            Ok(VersionedXcmMessage::V1(MessageV1 {
                command:
                    Command::SendToken {
                        token: token_address,
                        destination,
                        amount,
                        fee: _,
                    },
                ..
            })) => {
                let token_location = if token_address == H160::zero() {
                    Location {
                        parents: 1,
                        interior: X1([GlobalConsensus(EthereumNetwork::get())].into()),
                    }
                } else {
                    Location {
                        parents: 1,
                        interior: X2([
                            GlobalConsensus(EthereumNetwork::get()),
                            AccountKey20 {
                                network: Some(EthereumNetwork::get()),
                                key: token_address.into(),
                            },
                        ]
                        .into()),
                    }
                };

                Some(EthTransferData {
                    token_location,
                    destination,
                    amount,
                })
            }
            _ => None,
        }
    }

    /// Process a native ETH transfer message to a local account in Tanssi chain.
    fn process_xcm_local_native_eth_transfer(eth_transfer_data: EthTransferData) -> DispatchResult {
        let assets_to_holding: XcmAssets = vec![XcmAsset {
            id: XcmAssetId::from(eth_transfer_data.token_location),
            fun: Fungibility::Fungible(eth_transfer_data.amount),
        }]
        .into();

        let destination_account = match eth_transfer_data.destination {
            Destination::AccountId32 { id } => id,
            _ => {
                log::error!("EthTokensLocalProcessor: invalid destination");
                return Ok(());
            }
        };

        let mut xcm = Xcm::<T::RuntimeCall>(vec![
            ReserveAssetDeposited(assets_to_holding),
            DepositAsset {
                assets: AllCounted(1).into(),
                beneficiary: Location::new(
                    0,
                    [AccountId32 {
                        network: None,
                        id: destination_account,
                    }],
                ),
            },
        ]);

        let ethereum_location = EthereumLocation::get();

        if let Ok(weight) = XcmWeigher::weight(&mut xcm) {
            let mut message_id = xcm.using_encoded(sp_io::hashing::blake2_256);

            let outcome = XcmProcessor::prepare_and_execute(
                ethereum_location,
                xcm,
                &mut message_id,
                weight,
                weight,
            );

            if let Err(error) = outcome.ensure_complete() {
                log::error!(
                    "EthTokensLocalProcessor: XCM execution failed with error {:?}",
                    error
                );
            }
        } else {
            log::error!("EthTokensLocalProcessor: unweighable message");
        }

        Ok(())
    }
}

/// `NativeContainerTokensProcessor` is responsible for receiving and processing native container
/// chain tokens coming from Ethereum and forwarding them to the container chain via Tanssi through XCM.
pub struct NativeContainerTokensProcessor<
    T,
    EthereumLocation,
    EthereumNetwork,
    InboundQueuePalletInstance,
>(
    PhantomData<(
        T,
        EthereumLocation,
        EthereumNetwork,
        InboundQueuePalletInstance,
    )>,
);

impl<T, EthereumLocation, EthereumNetwork, InboundQueuePalletInstance> MessageProcessor
    for NativeContainerTokensProcessor<
        T,
        EthereumLocation,
        EthereumNetwork,
        InboundQueuePalletInstance,
    >
where
    T: snowbridge_pallet_inbound_queue::Config
        + pallet_ethereum_token_transfers::Config
        + snowbridge_pallet_system::Config
        + pallet_xcm::Config,
    <T as frame_system::Config>::RuntimeEvent: From<RuntimeEvent>,
    EthereumLocation: Get<Location>,
    EthereumNetwork: Get<NetworkId>,
    InboundQueuePalletInstance: Get<u8>,
{
    fn can_process_message(channel: &Channel, envelope: &Envelope) -> bool {
        // Validate channel and gateway
        if !GatewayAndChannelValidator::<T>::validate_gateway_and_channel(channel, envelope) {
            log::warn!(
                "NativeContainerTokensProcessor::can_process_message: invalid gateway or channel"
            );
            return false;
        }

        // Try decoding the message and check if the destination owns the token being transferred
        match Self::get_token_data_and_location(&envelope.payload) {
            TokenDataResult::Success(token_data, token_location) => {
                Self::validate_destination_owns_token(&token_location, &token_data.destination)
            }
            TokenDataResult::DecodeFailure => {
                log::error!(
                    "NativeContainerTokensProcessor::can_process_message: failed to decode token data"
                );
                false
            }
            TokenDataResult::LocationNotFound(token_data) => {
                log::error!(
                    "NativeContainerTokensProcessor::can_process_message: token location not found for token_id: {:?}",
                    token_data.token_id
                );
                false
            }
        }
    }

    fn process_message(_channel: Channel, envelope: Envelope) -> DispatchResult {
        match Self::get_token_data_and_location(&envelope.payload) {
            TokenDataResult::Success(token_data, token_location) => {
                Self::process_native_token_transfer(token_data, token_location);
                Ok(())
            }
            TokenDataResult::DecodeFailure => Err(DispatchError::Other(
                "NativeContainerTokensProcessor: unexpected message",
            )),
            TokenDataResult::LocationNotFound(token_data) => {
                log::error!(
                    "NativeContainerTokensProcessor::process_message: token location not found for token_id: {:?}",
                    token_data.token_id
                );
                Ok(())
            }
        }
    }
}

/// Result of token data and location extraction
enum TokenDataResult {
    /// Successfully extracted both token data and location
    Success(NativeTokenTransferData, Location),
    /// Failed to decode token data from payload
    DecodeFailure,
    /// Token data decoded but location not found
    LocationNotFound(NativeTokenTransferData),
}

impl<T, EthereumLocation, EthereumNetwork, InboundQueuePalletInstance>
    NativeContainerTokensProcessor<T, EthereumLocation, EthereumNetwork, InboundQueuePalletInstance>
where
    T: snowbridge_pallet_inbound_queue::Config
        + pallet_ethereum_token_transfers::Config
        + snowbridge_pallet_system::Config
        + pallet_xcm::Config,
    <T as frame_system::Config>::RuntimeEvent: From<RuntimeEvent>,
    EthereumLocation: Get<Location>,
    EthereumNetwork: Get<NetworkId>,
    InboundQueuePalletInstance: Get<u8>,
{
    /// Decodes token data from payload and gets the corresponding token location.
    /// Returns different outcomes based on what succeeded or failed.
    fn get_token_data_and_location(payload: &[u8]) -> TokenDataResult {
        if let Some(token_data) = NativeTokenTransferData::decode_native_token_message(payload) {
            if let Some(token_location) =
                snowbridge_pallet_system::Pallet::<T>::convert(&token_data.token_id)
            {
                TokenDataResult::Success(token_data, token_location)
            } else {
                TokenDataResult::LocationNotFound(token_data)
            }
        } else {
            TokenDataResult::DecodeFailure
        }
    }

    /// Validates that the destination para_id owns the token being transferred.
    fn validate_destination_owns_token(
        token_location: &Location,
        destination: &Destination,
    ) -> bool {
        let chain_part = token_location.interior().clone().split_global().ok();

        let expected_para_id = match destination {
            Destination::ForeignAccountId32 { para_id, .. } => *para_id,
            Destination::ForeignAccountId20 { para_id, .. } => *para_id,
            _ => 0u32,
        };

        match chain_part {
            Some((_, interior)) => {
                if let Some(Parachain(id)) = interior.first() {
                    expected_para_id == *id
                } else {
                    log::error!(
                        "NativeContainerTokensProcessor: destination doesn't own the token!"
                    );
                    false
                }
            }
            _ => {
                log::error!("NativeContainerTokensProcessor: invalid chain part");
                false
            }
        }
    }

    /// Process a native token transfer by creating and sending an XCM message to the destination parachain.
    fn process_native_token_transfer(
        token_data: NativeTokenTransferData,
        token_location: Location,
    ) {
        let destination_info = match token_data.destination {
            Destination::ForeignAccountId32 { para_id, id, fee } => {
                let beneficiary = Location::new(0, [AccountId32 { network: None, id }]);
                Some((beneficiary, fee, para_id))
            }
            Destination::ForeignAccountId20 { para_id, id, fee } => {
                let beneficiary = Location::new(
                    0,
                    [AccountKey20 {
                        network: None,
                        key: id,
                    }],
                );
                Some((beneficiary, fee, para_id))
            }
            _ => {
                log::error!("NativeContainerTokensProcessor::process_native_token_transfer: invalid destination");
                None
            }
        };

        if let Some((beneficiary, container_fee, container_para_id)) = destination_info {
            let network = EthereumNetwork::get();
            let bridge_location = Location::new(2, GlobalConsensus(network));

            let total_fees = token_data.fee.saturating_add(container_fee);

            let container_location = Location::new(0, [Parachain(container_para_id)]);

            let token_split = token_location.interior().clone().split_global().ok();

            if let Some((_, interior)) = token_split {
                let container_token_from_tanssi = Location::new(0, interior);
                let reanchor_result = container_token_from_tanssi.reanchored(
                    &container_location,
                    &<T as pallet_xcm::Config>::UniversalLocation::get(),
                );

                if let Ok(token_location_reanchored) = reanchor_result {
                    let container_asset: Asset =
                        (token_location_reanchored, token_data.amount).into();
                    let tanssi_asset_fee: Asset = (Location::parent(), total_fees).into();
                    let inbound_queue_pallet_index = InboundQueuePalletInstance::get();

                    let remote_xcm = Xcm::<()>(vec![
                        DescendOrigin(PalletInstance(inbound_queue_pallet_index).into()),
                        UniversalOrigin(GlobalConsensus(network)),
                        WithdrawAsset(
                            vec![tanssi_asset_fee.clone(), container_asset.clone()].into(),
                        ),
                        BuyExecution {
                            fees: tanssi_asset_fee,
                            weight_limit: Unlimited,
                        },
                        DepositAsset {
                            assets: Definite(container_asset.into()),
                            beneficiary,
                        },
                        // When the execution finishes deposit any leftover assets to the ETH
                        // sovereign account on destination.
                        SetAppendix(Xcm(vec![DepositAsset {
                            assets: Wild(AllCounted(2)),
                            beneficiary: bridge_location,
                        }])),
                    ]);

                    send_xcm::<<T as pallet_xcm::Config>::XcmRouter>(
                        container_location.clone(),
                        remote_xcm.clone(),
                    )
                    .map(|(message_id, _price)| {
                        frame_system::Pallet::<T>::deposit_event(RuntimeEvent::XcmPallet(
                            pallet_xcm::Event::Sent {
                                origin: Here.into_location(),
                                destination: container_location,
                                message: remote_xcm,
                                message_id,
                            },
                        ));
                    })
                    .map_err(|e| {
                        log::error!(
                            "NativeContainerTokensProcessor: XCM send failed with error: {:?}",
                            e
                        );
                    })
                    .ok();
                } else {
                    log::error!(
                        "NativeContainerTokensProcessor: failed to reanchor token location"
                    );
                }
            } else {
                log::error!("NativeContainerTokensProcessor: failed to reanchor token location");
            }
        }
    }
}

#[cfg(feature = "runtime-benchmarks")]
mod benchmark_helper {
    use {
        crate::{
            bridge_to_ethereum_config::EthTokensProcessor, AccountId, Balances,
            EthereumBeaconClient, ForeignAssetsCreator, Runtime, RuntimeOrigin,
            SnowbridgeFeesAccount, UNITS,
        },
        frame_support::traits::fungible::Mutate,
        snowbridge_core::Channel,
        snowbridge_inbound_queue_primitives::{
            v1::{Envelope, MessageProcessor},
            EventFixture,
        },
        snowbridge_pallet_inbound_queue::Nonce,
        snowbridge_pallet_system::Channels,
        sp_runtime::DispatchResult,
        xcm::latest::Location,
    };

    pub struct EthSystemBenchHelper;

    impl snowbridge_pallet_system::BenchmarkHelper<RuntimeOrigin> for EthSystemBenchHelper {
        fn make_xcm_origin(location: Location) -> RuntimeOrigin {
            RuntimeOrigin::from(pallet_xcm::Origin::Xcm(location))
        }
    }

    impl snowbridge_pallet_inbound_queue::BenchmarkHelper<Runtime> for EthSystemBenchHelper {
        fn initialize_storage() -> EventFixture {
            // In our case send token command is the worst case to benchmark, but this might change in the future
            let submit_message =
                snowbridge_pallet_inbound_queue_fixtures::send_token::make_send_token_message();
            let envelope: Envelope = Envelope::try_from(&submit_message.event.event_log).unwrap();

            Channels::<Runtime>::set(
                envelope.channel_id,
                Some(Channel {
                    agent_id: Default::default(),
                    para_id: Default::default(),
                }),
            );

            Nonce::<Runtime>::insert(envelope.channel_id, 1);

            let eth_transfer_data =
                EthTokensProcessor::decode_message_for_eth_transfer(envelope.payload.as_slice())
                    .unwrap();

            ForeignAssetsCreator::create_foreign_asset(
                RuntimeOrigin::root(),
                eth_transfer_data.token_location,
                42,
                AccountId::new([0; 32]),
                true,
                1,
            )
            .expect("creating foreign asset");

            EthereumBeaconClient::store_finalized_header(
                submit_message.finalized_header,
                submit_message.block_roots_root,
            )
            .expect("storing finalized header");

            Balances::mint_into(&SnowbridgeFeesAccount::get(), 10 * UNITS)
                .expect("minting fees_account balance");

            submit_message
        }
    }

    pub struct WorstCaseMessageProcessor<P>(core::marker::PhantomData<P>);
    impl<P> MessageProcessor for WorstCaseMessageProcessor<P>
    where
        P: MessageProcessor,
    {
        fn can_process_message(_channel: &Channel, _envelope: &Envelope) -> bool {
            true
        }

        fn process_message(channel: Channel, envelope: Envelope) -> DispatchResult {
            P::process_message(channel, envelope)
        }
    }
}

#[cfg(any(test, feature = "testing-helpers"))]
mod test_helpers {
    use snowbridge_inbound_queue_primitives::{Log, Proof, VerificationError, Verifier};

    pub struct MockVerifier;

    impl Verifier for MockVerifier {
        fn verify(_: &Log, _: &Proof) -> Result<(), VerificationError> {
            Ok(())
        }
    }
}

parameter_types! {
    pub InboundQueuePalletInstance: u8 = <EthereumInboundQueue as PalletInfoAccess>::index() as u8;
}

pub type EthTokensProcessor = EthTokensLocalProcessor<
    Runtime,
    xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
    <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
    dancelight_runtime_constants::snowbridge::EthereumLocation,
    dancelight_runtime_constants::snowbridge::EthereumNetwork,
>;

#[cfg(not(feature = "runtime-benchmarks"))]
pub type NativeTokensProcessor = NativeTokenTransferMessageProcessor<Runtime>;

pub type NativeContainerProcessor = NativeContainerTokensProcessor<
    Runtime,
    dancelight_runtime_constants::snowbridge::EthereumLocation,
    dancelight_runtime_constants::snowbridge::EthereumNetwork,
    InboundQueuePalletInstance,
>;

impl snowbridge_pallet_inbound_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    #[cfg(all(not(test), not(feature = "testing-helpers")))]
    type Verifier = EthereumBeaconClient;
    #[cfg(any(test, feature = "testing-helpers"))]
    type Verifier = test_helpers::MockVerifier;
    type Token = Balances;
    // TODO: Revisit this when we enable xcmp messages
    type XcmSender = DoNothingRouter;
    type GatewayAddress = EthereumGatewayAddress;
    // TODO: Revisit this when we enable xcmp messages
    type MessageConverter = DoNothingConvertMessage;
    type ChannelLookup = EthereumSystem;
    type PricingParameters = EthereumSystem;
    type WeightInfo = weights::snowbridge_pallet_inbound_queue::SubstrateWeight<Runtime>;
    #[cfg(feature = "runtime-benchmarks")]
    type Helper = benchmark_helper::EthSystemBenchHelper;
    type WeightToFee = WeightToFee;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    // TODO: Revisit this when we enable xcmp messages
    type MaxMessageSize = ConstU32<2048>;
    type AssetTransactor = <xcm_config::XcmConfig as xcm_executor::Config>::AssetTransactor;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type MessageProcessor = (
        SymbioticMessageProcessor<Self>,
        GenericTokenMessageProcessor<
            Self,
            (NativeTokensProcessor, NativeContainerProcessor),
            EthTokensProcessor,
        >,
    );
    type RewardProcessor = RewardThroughFeesAccount<Self>;
    #[cfg(feature = "runtime-benchmarks")]
    type MessageProcessor = (benchmark_helper::WorstCaseMessageProcessor<EthTokensProcessor>,);
}
