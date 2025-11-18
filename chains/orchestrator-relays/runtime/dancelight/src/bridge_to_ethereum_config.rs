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
use crate::EthereumInboundQueueV2;
use cumulus_primitives_core::Location;
use frame_support::dispatch::DispatchResult;
use frame_support::pallet_prelude::{DecodeWithMemTracking, Encode, TypeInfo};
use frame_support::traits::{fungible::Mutate, tokens::Preservation, EnqueueMessage, EnsureOrigin};
use frame_support::BoundedSlice;
use frame_system::EnsureRoot;
use frame_system::EnsureRootWithSuccess;
use pallet_ethereum_token_transfers::{
    origins::{ConvertAccountIdTo, ConvertUnitTo, EnsureEthereumTokenTransfersOrigin},
    pallet::TipHandler,
};
use parity_scale_codec::{Decode, MaxEncodedLen};
use snowbridge_core::reward::{AddTip, AddTipError, MessageId};
use snowbridge_outbound_queue_primitives::v2::{Message, SendMessage};
use snowbridge_outbound_queue_primitives::SendError;

#[cfg(not(feature = "runtime-benchmarks"))]
use {
    tanssi_runtime_common::relay::{
        NativeContainerTokensProcessor, NativeTokenTransferMessageProcessor,
    },
    tp_bridge::{
        symbiotic_message_processor::SymbioticMessageProcessor, GenericTokenInboundMessageProcessor,
    },
};

use crate::{AccountId, BridgeRelayers};
use dancelight_runtime_constants::snowbridge::EthereumLocation;
use tp_traits::BlockNumber;
use {
    crate::{
        parameter_types, weights, xcm_config, Balance, Balances, EthereumInboundQueue,
        EthereumOutboundQueue, EthereumSovereignAccount, EthereumSystem, FixedU128,
        GetAggregateMessageOrigin, Keccak256, MessageQueue, OutboundMessageCommitmentRecorder,
        Runtime, RuntimeEvent, SnowbridgeFeesAccount, TanssiAggregateMessageOrigin,
        TokenLocationReanchored, TransactionByteFee, TreasuryAccount, WeightToFee, UNITS,
    },
    frame_support::{
        traits::{EitherOf, MapSuccess, PalletInfoAccess},
        weights::ConstantMultiplier,
    },
    pallet_xcm::EnsureXcm,
    snowbridge_beacon_primitives::ForkVersions,
    snowbridge_core::{gwei, meth, PricingParameters, Rewards},
    snowbridge_pallet_outbound_queue::OnNewCommitment,
    sp_core::{ConstU32, ConstU8, H160, H256},
    tanssi_runtime_common::relay::{EthTokensLocalProcessor, RewardThroughFeesAccount},
    tp_bridge::{DoNothingConvertMessage, DoNothingRouter, EthereumSystemHandler},
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

impl pallet_outbound_message_commitment_recorder::Config for Runtime {}

// https://github.com/paritytech/polkadot-sdk/blob/2ae79be8e028a995b850621ee55f46c041eceefe/cumulus/parachains/runtimes/bridge-hubs/bridge-hub-westend/src/bridge_to_ethereum_config.rs#L105
impl snowbridge_pallet_outbound_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Hashing = Keccak256;
    type AggregateMessageOrigin = TanssiAggregateMessageOrigin;
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

/// Rewards for Snowbridge.Outbound in tanssi. Inbound in ETH
/// I wish we could have a single variant with location inside,
/// but the pallets require the copy trait, and ML does not
/// derive copy
#[derive(
    Clone,
    Copy,
    Debug,
    Decode,
    DecodeWithMemTracking,
    Encode,
    Eq,
    MaxEncodedLen,
    PartialEq,
    TypeInfo,
)]
pub enum BridgeReward {
    SnowbridgeRewardOutbound,
    SnowbridgeRewardInbound,
}

parameter_types! {
    pub SnowbridgeRewardOutbound: BridgeReward = BridgeReward::SnowbridgeRewardOutbound;
    pub SnowbridgeRewardInbound: BridgeReward = BridgeReward::SnowbridgeRewardInbound;
}

pub struct DoNothingMessageQueue;
impl EnqueueMessage<bridge_hub_common::AggregateMessageOrigin> for DoNothingMessageQueue {
    type MaxMessageLen = ();

    fn enqueue_message(
        _message: BoundedSlice<u8, Self::MaxMessageLen>,
        _origin: bridge_hub_common::AggregateMessageOrigin,
    ) {
    }

    fn enqueue_messages<'a>(
        _messages: impl Iterator<Item = BoundedSlice<'a, u8, Self::MaxMessageLen>>,
        _origin: bridge_hub_common::AggregateMessageOrigin,
    ) {
    }

    fn sweep_queue(_origin: bridge_hub_common::AggregateMessageOrigin) {}
}

#[derive(
    Clone, Debug, Decode, DecodeWithMemTracking, Encode, Eq, MaxEncodedLen, PartialEq, TypeInfo,
)]
pub enum BridgeRewardBeneficiaries {
    /// A local chain account.
    LocalAccount(AccountId),
}

impl From<sp_runtime::AccountId32> for BridgeRewardBeneficiaries {
    fn from(value: sp_runtime::AccountId32) -> Self {
        BridgeRewardBeneficiaries::LocalAccount(value)
    }
}

pub struct BridgeRewardPayer;
impl bp_relayers::PaymentProcedure<AccountId, BridgeReward, u128> for BridgeRewardPayer {
    type Error = sp_runtime::DispatchError;
    type Beneficiary = BridgeRewardBeneficiaries;

    fn pay_reward(
        _relayer: &AccountId,
        reward_kind: BridgeReward,
        _reward: u128,
        beneficiary: BridgeRewardBeneficiaries,
    ) -> Result<(), Self::Error> {
        match reward_kind {
            BridgeReward::SnowbridgeRewardInbound => {
                match beneficiary {
                    BridgeRewardBeneficiaries::LocalAccount(_account_id) => {
                        // TODO: Pay relayer from reward account in ETH.
                        // Mint reward directly with transactor
                        Ok(())
                    }
                }
            }
            BridgeReward::SnowbridgeRewardOutbound => {
                match beneficiary {
                    BridgeRewardBeneficiaries::LocalAccount(_account_id) => {
                        // TODO: Pay relayer from reward account in tanssi.
                        // Take from ethereum fees account
                        Ok(())
                    }
                }
            }
        }
    }
}

// These values are copied from bridge-hub-westend we will need to modify them
parameter_types! {
    pub storage RequiredStakeForStakeAndSlash: Balance = 1_000_000;
    pub const RelayerStakeLease: u32 = 8;
    pub const RelayerStakeReserveId: [u8; 8] = *b"brdgrlrs";
}

pub type BridgeRelayersInstance = ();
impl pallet_bridge_relayers::Config<BridgeRelayersInstance> for Runtime {
    type RuntimeEvent = RuntimeEvent;

    type RewardBalance = u128;
    type Reward = BridgeReward;
    type PaymentProcedure = BridgeRewardPayer;

    type StakeAndSlash = pallet_bridge_relayers::StakeAndSlashNamed<
        AccountId,
        BlockNumber,
        Balances,
        RelayerStakeReserveId,
        RequiredStakeForStakeAndSlash,
        RelayerStakeLease,
    >;
    type Balance = Balance;
    type WeightInfo = ();
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

// Added this since we do not have outbound queue integrated yet
pub struct DoNothingQueue;
impl SendMessage for DoNothingQueue {
    type Ticket = ();

    fn validate(_: &Message) -> Result<Self::Ticket, SendError> {
        Ok(())
    }

    fn deliver(_: Self::Ticket) -> Result<H256, SendError> {
        Ok(H256::zero())
    }
}

impl AddTip for DoNothingQueue {
    fn add_tip(_nonce: u64, _amount: u128) -> Result<(), AddTipError> {
        Ok(())
    }
}

impl snowbridge_pallet_system_v2::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OutboundQueue = DoNothingQueue;
    type InboundQueue = EthereumInboundQueueV2;
    type FrontendOrigin = EitherOf<
        MapSuccess<EnsureRoot<AccountId>, ConvertUnitTo<Location>>,
        MapSuccess<
            EnsureEthereumTokenTransfersOrigin<Runtime>,
            ConvertAccountIdTo<AccountId, Location, xcm_config::RelayNetwork>,
        >,
    >;
    type GovernanceOrigin = EnsureRootWithSuccess<AccountId, EthereumLocation>;
    type WeightInfo = ();
    #[cfg(feature = "runtime-benchmarks")]
    type Helper = ();
}

pub struct EthereumTipForwarder<T>(core::marker::PhantomData<T>);
impl TipHandler<crate::RuntimeOrigin> for EthereumTipForwarder<Runtime> {
    fn add_tip(
        origin: crate::RuntimeOrigin,
        message_id: MessageId,
        amount: u128,
    ) -> DispatchResult {
        let sender = pallet_ethereum_token_transfers::origins::EnsureEthereumTokenTransfersOrigin::<
            Runtime,
        >::ensure_origin(origin.clone())?;

        Balances::transfer(
            &sender,
            &<Runtime as pallet_ethereum_token_transfers::Config>::FeesAccount::get(),
            amount.into(),
            Preservation::Preserve,
        )?;

        snowbridge_pallet_system_v2::Pallet::<Runtime>::add_tip(origin, sender, message_id, amount)
    }
}
impl pallet_ethereum_token_transfers::Config for Runtime {
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
    type TipHandler = EthereumTipForwarder<Runtime>;
    type PalletOrigin = Self::RuntimeOrigin;
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
        snowbridge_pallet_inbound_queue_v2::BenchmarkHelper as InboundQueueBenchmarkHelperV2,
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

    impl<T: snowbridge_pallet_inbound_queue_v2::Config> InboundQueueBenchmarkHelperV2<T> for Runtime {
        fn initialize_storage(beacon_header: BeaconHeader, block_roots_root: H256) {
            // TODO: fill this by inbound people
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

pub type AssetTransactor = <xcm_config::XcmConfig as xcm_executor::Config>::AssetTransactor;

pub type EthTokensProcessor = EthTokensLocalProcessor<
    Runtime,
    xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
    <xcm_config::XcmConfig as xcm_executor::Config>::Weigher,
    AssetTransactor,
    dancelight_runtime_constants::snowbridge::EthereumLocation,
    dancelight_runtime_constants::snowbridge::EthereumNetwork,
    frame_support::traits::ConstBool<true>,
>;

#[cfg(not(feature = "runtime-benchmarks"))]
pub type NativeTokensProcessor = NativeTokenTransferMessageProcessor<Runtime>;

#[cfg(not(feature = "runtime-benchmarks"))]
pub type NativeContainerProcessor = NativeContainerTokensProcessor<
    Runtime,
    AssetTransactor,
    dancelight_runtime_constants::snowbridge::EthereumLocation,
    dancelight_runtime_constants::snowbridge::EthereumNetwork,
    InboundQueuePalletInstance,
    TokenLocationReanchored,
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
    type AssetTransactor = AssetTransactor;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type MessageProcessor = (
        SymbioticMessageProcessor<Self>,
        GenericTokenInboundMessageProcessor<
            Self,
            (NativeTokensProcessor, NativeContainerProcessor),
            EthTokensProcessor,
        >,
    );
    type RewardProcessor = RewardThroughFeesAccount<Self>;
    #[cfg(feature = "runtime-benchmarks")]
    type MessageProcessor = (benchmark_helper::WorstCaseMessageProcessor<EthTokensProcessor>,);
}

impl snowbridge_pallet_inbound_queue_v2::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    #[cfg(all(not(test), not(feature = "testing-helpers")))]
    type Verifier = EthereumBeaconClient;
    #[cfg(any(test, feature = "testing-helpers"))]
    type Verifier = test_helpers::MockVerifier;
    // TODO: Revisit this when we enable xcmp messages
    type GatewayAddress = EthereumGatewayAddress;
    type MessageProcessor = (tp_bridge::SymbioticMessageProcessor<Self>,);
    type RewardKind = BridgeReward;
    type DefaultRewardKind = SnowbridgeRewardInbound;
    type RewardPayment = BridgeRelayers;
    type WeightInfo = ();
    #[cfg(feature = "runtime-benchmarks")]
    type Helper = Runtime;
}
