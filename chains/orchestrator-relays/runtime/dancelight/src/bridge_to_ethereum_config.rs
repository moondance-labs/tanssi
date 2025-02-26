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
#[cfg(not(test))]
use crate::EthereumBeaconClient;

use sp_runtime::traits::MaybeEquivalence;
#[cfg(not(feature = "runtime-benchmarks"))]
use tp_bridge::symbiotic_message_processor::SymbioticMessageProcessor;

use {
    crate::{
        parameter_types, weights, xcm_config, AggregateMessageOrigin, Balance, Balances,
        EthereumInboundQueue, EthereumOutboundQueue, EthereumSovereignAccount, EthereumSystem,
        FixedU128, GetAggregateMessageOrigin, Keccak256, MessageQueue,
        OutboundMessageCommitmentRecorder, Runtime, RuntimeEvent, TokenLocationReanchored,
        TransactionByteFee, TreasuryAccount, WeightToFee, UNITS,
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
    snowbridge_beacon_primitives::{Fork, ForkVersions},
    snowbridge_core::inbound::Message,
    snowbridge_core::{gwei, meth, Channel, PricingParameters, Rewards},
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

// For tests, benchmarks and fast-runtime configurations we use the mocked fork versions
#[cfg(any(
    feature = "std",
    feature = "fast-runtime",
    feature = "runtime-benchmarks",
    test
))]
parameter_types! {
    pub const ChainForkVersions: ForkVersions = ForkVersions {
        genesis: Fork {
            version: [0, 0, 0, 0], // 0x00000000
            epoch: 0,
        },
        altair: Fork {
            version: [1, 0, 0, 0], // 0x01000000
            epoch: 0,
        },
        bellatrix: Fork {
            version: [2, 0, 0, 0], // 0x02000000
            epoch: 0,
        },
        capella: Fork {
            version: [3, 0, 0, 0], // 0x03000000
            epoch: 0,
        },
        deneb: Fork {
            version: [4, 0, 0, 0], // 0x04000000
            epoch: 0,
        }
    };
}

// Holesky: https://github.com/eth-clients/holesky
// Fork versions: https://github.com/eth-clients/holesky/blob/main/metadata/config.yaml
#[cfg(not(any(
    feature = "std",
    feature = "fast-runtime",
    feature = "runtime-benchmarks",
    test
)))]
parameter_types! {
    pub const ChainForkVersions: ForkVersions = ForkVersions {
        genesis: Fork {
            version: hex_literal::hex!("01017000"), // 0x01017000
            epoch: 0,
        },
        altair: Fork {
            version: hex_literal::hex!("02017000"), // 0x02017000
            epoch: 0,
        },
        bellatrix: Fork {
            version: hex_literal::hex!("03017000"), // 0x03017000
            epoch: 0,
        },
        capella: Fork {
            version: hex_literal::hex!("04017000"), // 0x04017000
            epoch: 256,
        },
        deneb: Fork {
            version: hex_literal::hex!("05017000"), // 0x05017000
            epoch: 29696,
        },
    };
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
    type FeesAccount = TreasuryAccount;
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
                        fee,
                        ..
                    },
            }) => {
                if fee >= amount {
                    return Err(DispatchError::Other("fee is greater than amount"));
                }

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
    use snowbridge_beacon_primitives::BeaconHeader;
    use snowbridge_core::Channel;
    use snowbridge_pallet_system::Channels;
    use snowbridge_router_primitives::inbound::envelope::Envelope;
    use snowbridge_router_primitives::inbound::MessageProcessor;
    use sp_core::H256;
    use {
        crate::EthereumBeaconClient, crate::Runtime, crate::RuntimeOrigin, xcm::latest::Location,
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

#[cfg(test)]
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
pub struct RewardThroughTreasury<T>(sp_std::marker::PhantomData<T>);

impl<T> RewardProcessor<T> for RewardThroughTreasury<T>
where
    T: snowbridge_pallet_inbound_queue::Config + pallet_ethereum_token_transfers::Config,
    T::AccountId: From<sp_runtime::AccountId32>,
    <T::Token as Inspect<T::AccountId>>::Balance: From<u128>,
{
    fn process_reward(who: T::AccountId, _channel: Channel, message: Message) -> DispatchResult {
        let envelope = Envelope::try_from(&message.event_log)
            .map_err(|_| snowbridge_pallet_inbound_queue::Error::<T>::InvalidEnvelope)?;

        let reward_amount: <T::Token as Inspect<T::AccountId>>::Balance =
            match VersionedXcmMessage::decode_all(&mut envelope.payload.as_slice()) {
                Ok(VersionedXcmMessage::V1(MessageV1 { command, .. })) => match command {
                    Command::SendNativeToken { fee, .. }
                    | Command::SendToken { fee, .. }
                    | Command::RegisterToken { fee, .. } => fee.into(),
                },
                Err(_) => return Ok(()), // Do not reward if we cannot handle the message
            };

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
    #[cfg(not(test))]
    type Verifier = EthereumBeaconClient;
    #[cfg(test)]
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
    type RewardProcessor = RewardThroughTreasury<Self>;
    #[cfg(feature = "runtime-benchmarks")]
    type MessageProcessor = (benchmark_helper::DoNothingMessageProcessor,);
}
