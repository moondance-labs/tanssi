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
    frame_support::{dispatch::DispatchClass, weights::ConstantMultiplier},
    pallet_xcm::EnsureXcm,
    parity_scale_codec::DecodeAll,
    snowbridge_beacon_primitives::ForkVersions,
    snowbridge_core::{gwei, meth, Channel, PricingParameters, Rewards},
    snowbridge_inbound_queue_primitives::v1::{
        Command, Destination, Envelope, MessageProcessor, MessageV1, VersionedXcmMessage,
    },
    snowbridge_pallet_outbound_queue::OnNewCommitment,
    sp_core::{ConstU32, ConstU8, Get, H160, H256},
    sp_runtime::{DispatchError, DispatchResult},
    sp_std::{marker::PhantomData, vec},
    tanssi_runtime_common::relay::RewardThroughFeesAccount,
    tp_bridge::{DoNothingConvertMessage, DoNothingRouter, EthereumSystemHandler},
    xcm::latest::{
        prelude::*, Asset as XcmAsset, AssetId as XcmAssetId, Assets as XcmAssets, ExecuteXcm,
        Fungibility, Junctions::*,
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
        // Ensure that the message is intended for the current channel, para_id and agent_id
        if let Some(channel_info) =
            pallet_ethereum_token_transfers::CurrentChannelInfo::<Runtime>::get()
        {
            if envelope.channel_id != channel_info.channel_id
                || channel.para_id != channel_info.para_id
                || channel.agent_id != channel_info.agent_id
            {
                return false;
            }
        } else {
            return false;
        }

        // Check it is from the right gateway
        if envelope.gateway != T::GatewayAddress::get() {
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
            _ => Err(DispatchError::Other(
                "container transfers not supported yet",
            )),
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
            _ => return Err(DispatchError::Other("invalid destination")),
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

        let weight = XcmWeigher::weight(&mut xcm)
            .map_err(|()| DispatchError::Other("UnweighableMessage"))?;
        let mut message_id = xcm.using_encoded(sp_io::hashing::blake2_256);

        let outcome = XcmProcessor::prepare_and_execute(
            ethereum_location,
            xcm,
            &mut message_id,
            weight,
            weight,
        );

        frame_system::Pallet::<T>::register_extra_weight_unchecked(weight, DispatchClass::Normal);

        outcome.ensure_complete().map_err(|error| {
            log::error!(
                "EthTokensLocalProcessor: XCM execution failed with error {:?}",
                error
            );
            DispatchError::Other("LocalExecutionIncomplete")
        })?;

        Ok(())
    }
}

#[cfg(feature = "runtime-benchmarks")]
mod benchmark_helper {
    use {
        crate::{EthereumBeaconClient, Runtime, RuntimeOrigin},
        snowbridge_beacon_primitives::BeaconHeader,
        snowbridge_core::Channel,
        snowbridge_inbound_queue_primitives::v1::{Envelope, MessageProcessor},
        snowbridge_pallet_system::Channels,
        sp_core::H256,
        xcm::latest::Location,
    };

    pub struct EthSystemBenchHelper;

    impl snowbridge_pallet_system::BenchmarkHelper<RuntimeOrigin> for EthSystemBenchHelper {
        fn make_xcm_origin(location: Location) -> RuntimeOrigin {
            RuntimeOrigin::from(pallet_xcm::Origin::Xcm(location))
        }
    }

    impl snowbridge_pallet_inbound_queue::BenchmarkHelper<Runtime> for EthSystemBenchHelper {
        fn initialize_storage(beacon_header: BeaconHeader, block_roots_root: H256) {
            let submit_message = snowbridge_pallet_inbound_queue_fixtures::register_token::make_register_token_message();
            let envelope: Envelope = Envelope::try_from(&submit_message.event.event_log).unwrap();

            Channels::<Runtime>::set(
                envelope.channel_id,
                Some(Channel {
                    agent_id: Default::default(),
                    para_id: Default::default(),
                }),
            );

            EthereumBeaconClient::store_finalized_header(beacon_header, block_roots_root).unwrap();
        }
    }

    pub struct DoNothingMessageProcessor;

    impl MessageProcessor for DoNothingMessageProcessor {
        fn can_process_message(_: &Channel, _: &Envelope) -> bool {
            true
        }

        fn process_message(_: Channel, _: Envelope) -> Result<(), sp_runtime::DispatchError> {
            Ok(())
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

pub type EthTokensProcessor = EthTokensLocalProcessor<
    Runtime,
    xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
    <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
    dancelight_runtime_constants::snowbridge::EthereumLocation,
    dancelight_runtime_constants::snowbridge::EthereumNetwork,
>;

#[cfg(not(feature = "runtime-benchmarks"))]
pub type NativeTokensProcessor = NativeTokenTransferMessageProcessor<Runtime>;

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
        GenericTokenMessageProcessor<Self, NativeTokensProcessor, EthTokensProcessor>,
    );
    type RewardProcessor = RewardThroughFeesAccount<Self>;
    #[cfg(feature = "runtime-benchmarks")]
    type MessageProcessor = (benchmark_helper::DoNothingMessageProcessor,);
}
