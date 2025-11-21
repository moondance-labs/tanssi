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
use cumulus_primitives_core::Location;
use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::{DecodeWithMemTracking, Encode, TypeInfo},
    traits::{fungible::Mutate, tokens::Preservation, ConstBool, EnqueueMessage, EnsureOrigin},
    BoundedSlice,
};
use frame_system::{EnsureRoot, EnsureRootWithSuccess};
use pallet_ethereum_token_transfers::{
    origins::{ConvertAccountIdTo, ConvertUnitTo, EnsureEthereumTokenTransfersOrigin},
    pallet::TipHandler,
};
use parity_scale_codec::{Decode, MaxEncodedLen};
use snowbridge_core::reward::MessageId;
use snowbridge_outbound_queue_primitives::v2::ConstantGasMeter as ConstantGasMeterV2;

#[cfg(not(feature = "runtime-benchmarks"))]
use {
    tanssi_runtime_common::relay::{
        NativeContainerTokensProcessor, NativeTokenTransferMessageProcessor,
    },
    tp_bridge::{
        symbiotic_message_processor::SymbioticMessageProcessor, GenericTokenInboundMessageProcessor,
    },
};

use crate::{AccountId, BridgeRelayers, EthereumInboundQueueV2};
use dancelight_runtime_constants::snowbridge::EthereumLocation;
use tp_traits::BlockNumber;
use {
    crate::{
        parameter_types, weights, xcm_config, Balance, Balances, EthereumInboundQueue,
        EthereumOutboundQueue, EthereumOutboundQueueV2, EthereumSovereignAccount, EthereumSystem,
        FixedU128, GetAggregateMessageOrigin, Keccak256, MessageQueue,
        OutboundMessageCommitmentRecorder, Runtime, RuntimeEvent, SnowbridgeFeesAccount,
        TanssiAggregateMessageOrigin, TokenLocationReanchored, TransactionByteFee, TreasuryAccount,
        WeightToFee, UNITS,
    },
    frame_support::{
        traits::{EitherOf, MapSuccess, PalletInfoAccess},
        weights::ConstantMultiplier,
    },
    pallet_xcm::EnsureXcm,
    snowbridge_beacon_primitives::ForkVersions,
    snowbridge_core::{gwei, meth, PricingParameters, Rewards},
    snowbridge_pallet_outbound_queue::OnNewCommitment,
    snowbridge_pallet_outbound_queue_v2::OnNewCommitment as OnNewCommitmentV2,
    sp_core::{ConstU32, ConstU8, H160, H256},
    tanssi_runtime_common::relay::{EthTokensLocalProcessor, RewardThroughFeesAccount},
    tp_bridge::{DoNothingConvertMessage, DoNothingRouter, EthereumSystemHandler},
    xcm::latest::{Asset, XcmContext},
    xcm_executor::traits::TransactAsset,
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

impl OnNewCommitmentV2 for CommitmentRecorder {
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
        reward: u128,
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
                    // Fees are collected by the snowbridge fees account and thus payed from it too
                    BridgeRewardBeneficiaries::LocalAccount(account_id) => {
                        Balances::transfer(
                            &SnowbridgeFeesAccount::get(),
                            &account_id,
                            reward.into(),
                            Preservation::Preserve,
                        )?;
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

impl snowbridge_pallet_system_v2::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OutboundQueue = EthereumOutboundQueueV2;
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

        // Tip handling depends on the queue type
        // Inbound: tip in ETH
        // Outbound: tip in Tanssi
        match message_id {
            MessageId::Inbound(_) => {
                let dummy_context = XcmContext {
                    origin: None,
                    message_id: Default::default(),
                    topic: None,
                };
                let asset: Asset = (EthereumLocation::get(), amount).into();
                // Here I am not sure I can work with any other thing than this.
                // In theory I could call foreignAssets directly, but I prefer not to.
                // In theory also, I could work with assetTransactor only for both cases
                // changing the token
                // TODO: revisit
                AssetTransactor::transfer_asset(
                    &asset,
                    &sender.clone().into(),
                    &<Runtime as pallet_ethereum_token_transfers::Config>::FeesAccount::get()
                        .into(),
                    &dummy_context,
                )
                .map_err(|e| {
                    log::debug!("Inbound tip addition failed with error {:?}", e);
                    sp_runtime::DispatchError::Other("TransferAsset failed for Inbound Fee")
                })?;
            }
            MessageId::Outbound(_) => {
                Balances::transfer(
                    &sender.clone(),
                    &<Runtime as pallet_ethereum_token_transfers::Config>::FeesAccount::get(),
                    amount,
                    Preservation::Expendable,
                )
                .map_err(|e| {
                    log::debug!("Outbound tip addition failed with error {:?}", e);
                    e
                })?;
            }
        };
        snowbridge_pallet_system_v2::Pallet::<Runtime>::add_tip(origin, sender, message_id, amount)
    }
    #[cfg(feature = "runtime-benchmarks")]
    fn set_tip(origin: crate::RuntimeOrigin, message_id: MessageId, amount: u128) {
        let sender = pallet_ethereum_token_transfers::origins::EnsureEthereumTokenTransfersOrigin::<
            Runtime,
        >::ensure_origin(origin.clone()).expect("origin does not match the expected origin");
        match message_id {
            MessageId::Inbound(_) => {
                // We need to mit the asset
                let (_asset_id, _asset_location) =
                    pallet_foreign_asset_creator::benchmarks::create_minted_asset::<Runtime>(
                        amount * 2,
                        sender,
                        Some(EthereumLocation::get()),
                    );
            }
            MessageId::Outbound(_) => {
                //TODO: fill by the outbound V2 PR.
            }
        }
    }
}
impl pallet_ethereum_token_transfers::Config for Runtime {
    type Currency = Balances;
    type OutboundQueue = EthereumOutboundQueue;
    type OutboundQueueV2 = EthereumOutboundQueueV2;
    type ShouldUseV2 = ConstBool<true>;
    type EthereumSystemHandler = EthereumSystemHandler<Runtime>;
    type EthereumSovereignAccount = EthereumSovereignAccount;
    type FeesAccount = SnowbridgeFeesAccount;
    type TokenLocationReanchored = TokenLocationReanchored;
    type TokenIdFromLocation = EthereumSystem;
    type UniversalLocation = xcm_config::UniversalLocation;
    type OriginToLocation = xcm_config::LocalOriginToLocation;
    type MinV2Reward = xcm_config::MinV2Reward;
    type EthereumLocation = EthereumLocation;
    type LocationHashOf = tp_bridge::TanssiAgentIdOf;
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
            bridge_to_ethereum_config::{EthTokensProcessor, EthereumGatewayAddress},
            AccountId, Balances, EthereumBeaconClient, ForeignAssetsCreator, Runtime,
            RuntimeOrigin, SnowbridgeFeesAccount, UNITS,
        },
        frame_support::traits::fungible::Mutate,
        snowbridge_beacon_primitives::BeaconHeader,
        snowbridge_core::Channel,
        snowbridge_inbound_queue_primitives::v2::{
            MessageProcessor as ProcessorV2, MessageProcessorError,
        },
        snowbridge_inbound_queue_primitives::{
            v1::{Envelope, MessageProcessor},
            EventFixture,
        },
        snowbridge_pallet_inbound_queue::Nonce,
        snowbridge_pallet_inbound_queue_v2::BenchmarkHelper as InboundQueueBenchmarkHelperV2,
        snowbridge_pallet_outbound_queue_v2::BenchmarkHelper as OutboundQueueBenchmarkHelperV2,
        snowbridge_pallet_system::Channels,
        sp_core::{H160, H256},
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

    impl snowbridge_pallet_system_v2::BenchmarkHelper<RuntimeOrigin> for () {
        fn make_xcm_origin(_location: Location) -> RuntimeOrigin {
            RuntimeOrigin::root()
        }
    }

    impl<T: snowbridge_pallet_outbound_queue_v2::Config> OutboundQueueBenchmarkHelperV2<T> for Runtime {
        fn initialize_storage(beacon_header: BeaconHeader, block_roots_root: H256) {
            EthereumBeaconClient::store_finalized_header(beacon_header, block_roots_root).unwrap();
            // Putting the gateway address in https://github.com/paritytech/polkadot-sdk/blob/c9879a5e3eeda1e8938ae7f6d06ec8df0a7a7da9/bridges/snowbridge/pallets/outbound-queue-v2/src/fixture.rs#L18C32-L18C72
            // Necessary to bench correctly
            EthereumGatewayAddress::set(&H160(hex_literal::hex!(
                "b1185ede04202fe62d38f5db72f71e38ff3e8305"
            )));
        }
    }
    impl<T: snowbridge_pallet_inbound_queue_v2::Config> InboundQueueBenchmarkHelperV2<T> for Runtime {
        fn initialize_storage(_beacon_header: BeaconHeader, _block_roots_root: H256) {
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

    impl<P> ProcessorV2<AccountId> for WorstCaseMessageProcessor<P>
    where
        P: ProcessorV2<AccountId>,
    {
        fn can_process_message(
            _channel: &AccountId,
            _envelope: &snowbridge_inbound_queue_primitives::v2::Message,
        ) -> bool {
            true
        }

        fn process_message(
            channel: AccountId,
            envelope: snowbridge_inbound_queue_primitives::v2::Message,
        ) -> Result<[u8; 32], MessageProcessorError> {
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
    type XcmSender = DoNothingRouter;
    type GatewayAddress = EthereumGatewayAddress;
    type MessageConverter = DoNothingConvertMessage;
    type ChannelLookup = EthereumSystem;
    type PricingParameters = EthereumSystem;
    type WeightInfo = weights::snowbridge_pallet_inbound_queue::SubstrateWeight<Runtime>;
    #[cfg(feature = "runtime-benchmarks")]
    type Helper = benchmark_helper::EthSystemBenchHelper;
    type WeightToFee = WeightToFee;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
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
    #[cfg(not(feature = "runtime-benchmarks"))]
    type MessageProcessor = (SymbioticMessageProcessor<Self>,);
    #[cfg(feature = "runtime-benchmarks")]
    type MessageProcessor =
        (benchmark_helper::WorstCaseMessageProcessor<tp_bridge::SymbioticMessageProcessor<Self>>,);
    type RewardKind = BridgeReward;
    type DefaultRewardKind = SnowbridgeRewardInbound;
    type RewardPayment = BridgeRelayers;
    #[cfg(feature = "runtime-benchmarks")]
    type Helper = Runtime;
    type WeightInfo = ();
}

// Outbound queue V2
// Should only used when detected an AliasOrigin instruction
// this is going to be a bit hard to do though, we will need to change a bunch of stuff
// The first thing we should see is whether regular transfers FROM tanssi work
// The container-chain exporter will come later

// For this it is mandatory to use the initiateTransfer xcmV5 instruction!
impl snowbridge_pallet_outbound_queue_v2::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Hashing = Keccak256;
    type MessageQueue = MessageQueue;
    // Maximum payload size for outbound messages.
    type MaxMessagePayloadSize = ConstU32<2048>;
    // Maximum number of outbound messages that can be committed per block.
    // It's benchmarked, including the entire process flow(initialize,submit,commit) in the
    // worst-case, Benchmark results in `../weights/snowbridge_pallet_outbound_queue_v2.
    // rs` show that the `process` function consumes less than 1% of the block capacity, which is
    // safe enough.
    type MaxMessagesPerBlock = ConstU32<32>;
    type GasMeter = ConstantGasMeterV2;
    type Balance = Balance;
    type WeightToFee = WeightToFee;
    #[cfg(all(not(test), not(feature = "testing-helpers")))]
    type Verifier = EthereumBeaconClient;
    #[cfg(any(test, feature = "testing-helpers"))]
    type Verifier = test_helpers::MockVerifier;
    type GatewayAddress = EthereumGatewayAddress;
    type WeightInfo = weights::snowbridge_pallet_outbound_queue_v2::SubstrateWeight<Runtime>;
    type EthereumNetwork = dancelight_runtime_constants::snowbridge::EthereumNetwork;
    type RewardKind = BridgeReward;
    type DefaultRewardKind = SnowbridgeRewardOutbound;
    type RewardPayment = BridgeRelayers;
    // Enable once we cherry-pick
    type OnNewCommitment = CommitmentRecorder;
    type AggregateMessageOrigin = TanssiAggregateMessageOrigin;
    #[cfg(feature = "runtime-benchmarks")]
    type Helper = Runtime;
}
