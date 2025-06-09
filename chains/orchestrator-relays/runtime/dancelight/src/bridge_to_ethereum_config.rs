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

pub const SLOTS_PER_EPOCH: u32 = snowbridge_pallet_ethereum_client::config::SLOTS_PER_EPOCH as u32;
#[cfg(all(not(test), not(feature = "testing-helpers")))]
use crate::EthereumBeaconClient;

use sp_runtime::traits::MaybeEquivalence;
#[cfg(not(feature = "runtime-benchmarks"))]
use tp_bridge::symbiotic_message_processor::SymbioticMessageProcessor;

use {
    crate::{
        parameter_types, weights, xcm_config, AggregateMessageOrigin, Balance, Balances,
        EthereumInboundQueue, EthereumOutboundQueue, EthereumSovereignAccount, EthereumSystem,
        FixedU128, GetAggregateMessageOrigin, Keccak256, MessageQueue,
        OutboundMessageCommitmentRecorder, Runtime, RuntimeEvent, SnowbridgeFeesAccount,
        TokenLocationReanchored, TransactionByteFee, TreasuryAccount, WeightToFee, UNITS,
    },
    frame_support::{
        traits::{
            fungible::{Inspect, Mutate},
            tokens::{Fortitude, Preservation},
        },
        weights::ConstantMultiplier,
    },
    pallet_xcm::EnsureXcm,
    parity_scale_codec::DecodeAll,
    snowbridge_beacon_primitives::ForkVersions,
    snowbridge_core::{gwei, inbound::Message, meth, Channel, PricingParameters, Rewards},
    snowbridge_pallet_inbound_queue::RewardProcessor,
    snowbridge_pallet_outbound_queue::OnNewCommitment,
    snowbridge_router_primitives::inbound::{
        envelope::Envelope, Command, Destination, MessageProcessor, MessageV1, VersionedXcmMessage,
    },
    sp_core::{ConstU32, ConstU8, Get, H160, H256},
    sp_runtime::{traits::Zero, DispatchError, DispatchResult},
    tp_bridge::{DoNothingConvertMessage, DoNothingRouter, EthereumSystemHandler},
};

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
    type GasMeter = snowbridge_core::outbound::ConstantGasMeter;
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

/// `NativeTokenTransferMessageProcessor` is responsible for receiving and processing native tokens
/// sent from Ethereum. If the message is valid, it performs the token transfer
/// from the Ethereum sovereign account to the specified destination account.
pub struct NativeTokenTransferMessageProcessor<T>(sp_std::marker::PhantomData<T>);
impl<T> MessageProcessor for NativeTokenTransferMessageProcessor<T>
where
    T: snowbridge_pallet_inbound_queue::Config + pallet_ethereum_token_transfers::Config,
    T::AccountId: From<[u8; 32]>,
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

        // Try decode the message and check the token id is the expected one
        match VersionedXcmMessage::decode_all(&mut envelope.payload.as_slice()) {
            Ok(VersionedXcmMessage::V1(MessageV1 {
                command: Command::SendNativeToken { token_id, .. },
                ..
            })) => {
                let token_location = T::TokenLocationReanchored::get();

                if let Some(expected_token_id) = EthereumSystem::convert_back(&token_location) {
                    return token_id == expected_token_id;
                }
                return false;
            }
            _ => false,
        }
    }

    fn process_message(_channel: Channel, envelope: Envelope) -> DispatchResult {
        // - Decode payload as SendNativeToken
        let message = VersionedXcmMessage::decode_all(&mut envelope.payload.as_slice())
            .map_err(|_| DispatchError::Other("unable to parse the envelope payload"))?;

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

                T::Currency::transfer(
                    &sovereign_account,
                    &destination_account.into(),
                    amount.into(),
                    Preservation::Preserve,
                )?;

                Ok(())
            }
            _ => return Err(DispatchError::Other("unexpected message")),
        }
    }
}

#[cfg(feature = "runtime-benchmarks")]
mod benchmark_helper {
    use {
        crate::{EthereumBeaconClient, Runtime, RuntimeOrigin},
        snowbridge_beacon_primitives::BeaconHeader,
        snowbridge_core::Channel,
        snowbridge_pallet_system::Channels,
        snowbridge_router_primitives::inbound::{envelope::Envelope, MessageProcessor},
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
            let envelope: Envelope = Envelope::try_from(&submit_message.message.event_log).unwrap();

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
    use snowbridge_core::inbound::{Log, Proof, VerificationError, Verifier};

    pub struct MockVerifier;

    impl Verifier for MockVerifier {
        fn verify(_: &Log, _: &Proof) -> Result<(), VerificationError> {
            Ok(())
        }
    }
}

/// Rewards the relayer that processed a native token transfer message
/// using the FeesAccount configured in pallet_ethereum_token_transfers
pub struct RewardThroughFeesAccount<T>(sp_std::marker::PhantomData<T>);

impl<T> RewardProcessor<T> for RewardThroughFeesAccount<T>
where
    T: snowbridge_pallet_inbound_queue::Config + pallet_ethereum_token_transfers::Config,
    T::AccountId: From<sp_runtime::AccountId32>,
    <T::Token as Inspect<T::AccountId>>::Balance: From<u128>,
{
    fn process_reward(who: T::AccountId, _channel: Channel, message: Message) -> DispatchResult {
        let reward_amount = snowbridge_pallet_inbound_queue::Pallet::<T>::calculate_delivery_cost(
            message.encode().len() as u32,
        );

        let fees_account: T::AccountId = T::FeesAccount::get();

        let amount =
            T::Token::reducible_balance(&fees_account, Preservation::Preserve, Fortitude::Polite)
                .min(reward_amount);
        if !amount.is_zero() {
            T::Token::transfer(&fees_account, &who, amount, Preservation::Preserve)?;
        }

        Ok(())
    }
}

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
        NativeTokenTransferMessageProcessor<Self>,
    );
    type RewardProcessor = RewardThroughFeesAccount<Self>;
    #[cfg(feature = "runtime-benchmarks")]
    type MessageProcessor = (benchmark_helper::DoNothingMessageProcessor,);
}
