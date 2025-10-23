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
    crate::TicketInfo,
    bp_relayers::RewardLedger,
    core::cell::RefCell,
    frame_support::{
        derive_impl, parameter_types,
        traits::{ConstU128, ConstU32, ConstU64, ConstU8, Equals, QueueFootprint},
        weights::IdentityFee,
    },
    parity_scale_codec::{Decode, Encode, MaxEncodedLen},
    scale_info::TypeInfo,
    snowbridge_core::{gwei, ChannelId, PricingParameters, Rewards, PRIMARY_GOVERNANCE_CHANNEL},
    snowbridge_outbound_queue_primitives::{
        v1::ConstantGasMeter, v2::ConstantGasMeter as ConstantGasMeterV2,
    },
    sp_core::{H160, H256},
    sp_runtime::{
        traits::{BlakeTwo256, Convert, IdentityLookup, Keccak256},
        BoundedSlice, BuildStorage,
    },
    xcm::prelude::*,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
// Configure all pallets related to bridging
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,
        EthereumOutboundQueue: snowbridge_pallet_outbound_queue,
        EthereumOutboundQueueV2: snowbridge_pallet_outbound_queue_v2,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = sp_runtime::AccountId32;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u128>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type Nonce = u64;
    type Block = Block;
    type RuntimeTask = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
    type ExtensionsWeightInfo = ();
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
    type Balance = u128;
    type ExistentialDeposit = ConstU128<1>;
    type AccountStore = System;
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 5;
    pub const MaxReserves: u32 = 50;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<5>;
    type WeightInfo = ();
}

thread_local! {
    pub static LAST_DELIVERED_MESSAGE_QUEUE: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

pub fn last_delivered_message_queue() -> Vec<u8> {
    LAST_DELIVERED_MESSAGE_QUEUE.with(|q| {
        let vec_ref = q.borrow();
        vec_ref.clone()
    })
}

#[derive(Clone, Decode, Default, Encode)]
pub struct DummyTicket {
    message_id: H256,
}

impl TicketInfo for DummyTicket {
    fn message_id(&self) -> H256 {
        H256::default()
    }
}

pub struct MockMessageQueue;

impl<Origin: MaxEncodedLen> frame_support::traits::EnqueueMessage<Origin> for MockMessageQueue {
    type MaxMessageLen = ConstU32<2048>;
    fn enqueue_message(message: BoundedSlice<u8, Self::MaxMessageLen>, _: Origin) {
        LAST_DELIVERED_MESSAGE_QUEUE.with(|r| *r.borrow_mut() = message.to_vec());
    }
    fn enqueue_messages<'a>(
        _: impl Iterator<Item = BoundedSlice<'a, u8, Self::MaxMessageLen>>,
        _: Origin,
    ) {
    }
    fn sweep_queue(_: Origin) {}
    fn footprint(_: Origin) -> QueueFootprint {
        QueueFootprint::default()
    }
}

parameter_types! {
    pub const MaxMessagePayloadSize: u32 = 1024;
    pub const MaxMessagesPerBlock: u32 = 20;
    pub MockPricingParameters: PricingParameters<u128> = PricingParameters {
        exchange_rate: 1u128.into(),
        fee_per_gas: gwei(20),
        rewards: Rewards { local: 0, remote: 0u128.into() },
        multiplier: 1u128.into()
    };
    pub EthereumNetwork: NetworkId = NetworkId::Ethereum { chain_id: 11155111 };
    pub const GatewayAddress: H160 = H160(GATEWAY_ADDRESS);
    pub const AllowedChannels: ChannelId = PRIMARY_GOVERNANCE_CHANNEL;
    pub const OwnLocation: Location = Location {
        parents:0,
        interior: Here
    };
}

/// The aggregate origin of an inbound message.
/// This is specialized for BridgeHub, as the snowbridge-outbound-queue-pallet is also using
/// the shared MessageQueue pallet.
#[derive(Encode, Decode, Copy, MaxEncodedLen, Clone, Eq, PartialEq, TypeInfo, Debug)]
pub enum MockAggregateMessageOrigin {
    SnowbridgeTest(ChannelId),
    SnowbridgeTestV2(H256),
}

pub struct MockGetAggregateMessageOrigin;

impl Convert<ChannelId, MockAggregateMessageOrigin> for MockGetAggregateMessageOrigin {
    fn convert(channel_id: ChannelId) -> MockAggregateMessageOrigin {
        MockAggregateMessageOrigin::SnowbridgeTest(channel_id)
    }
}

impl Convert<H256, MockAggregateMessageOrigin> for MockGetAggregateMessageOrigin {
    fn convert(origin: H256) -> MockAggregateMessageOrigin {
        MockAggregateMessageOrigin::SnowbridgeTestV2(origin)
    }
}

impl From<H256> for MockAggregateMessageOrigin {
    fn from(origin: H256) -> MockAggregateMessageOrigin {
        MockAggregateMessageOrigin::SnowbridgeTestV2(origin)
    }
}

impl snowbridge_pallet_outbound_queue::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Hashing = Keccak256;
    type MessageQueue = MockMessageQueue;
    type Decimals = ConstU8<10>;
    type MaxMessagePayloadSize = MaxMessagePayloadSize;
    type MaxMessagesPerBlock = MaxMessagesPerBlock;
    type GasMeter = ConstantGasMeter;
    type Balance = u128;
    type PricingParameters = MockPricingParameters;
    type Channels = Equals<AllowedChannels>;
    type WeightToFee = IdentityFee<u128>;
    type AggregateMessageOrigin = MockAggregateMessageOrigin;
    type GetAggregateMessageOrigin = MockGetAggregateMessageOrigin;
    type OnNewCommitment = ();
    type WeightInfo = ();
}

// Mock verifier
pub struct MockVerifier;

impl snowbridge_outbound_queue_primitives::Verifier for MockVerifier {
    fn verify(
        _: &snowbridge_outbound_queue_primitives::Log,
        _: &snowbridge_outbound_queue_primitives::Proof,
    ) -> Result<(), snowbridge_outbound_queue_primitives::VerificationError> {
        Ok(())
    }
}

pub struct MockRewardLedger;
impl RewardLedger<sp_runtime::AccountId32, (), u128> for MockRewardLedger {
    fn register_reward(_relayer: &sp_runtime::AccountId32, _reward: (), _reward_balance: u128) {}
}

const GATEWAY_ADDRESS: [u8; 20] = hex_literal::hex!["b1185ede04202fe62d38f5db72f71e38ff3e8305"];

impl snowbridge_pallet_outbound_queue_v2::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Verifier = MockVerifier;
    type GatewayAddress = GatewayAddress;
    type Hashing = Keccak256;
    type MessageQueue = MockMessageQueue;
    type MaxMessagePayloadSize = MaxMessagePayloadSize;
    type MaxMessagesPerBlock = MaxMessagesPerBlock;
    type GasMeter = ConstantGasMeterV2;
    type Balance = u128;
    type WeightToFee = IdentityFee<u128>;
    type WeightInfo = ();
    type RewardPayment = MockRewardLedger;
    type EthereumNetwork = EthereumNetwork;
    type RewardKind = ();
    type DefaultRewardKind = ();
    type AggregateMessageOrigin = MockAggregateMessageOrigin;
    #[cfg(feature = "runtime-benchmarks")]
    type Helper = Test;
}
pub fn new_test_ext() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    let ext: sp_io::TestExternalities = t.into();

    ext
}

pub fn run_to_block(n: u64) {
    System::run_to_block_with::<AllPalletsWithSystem>(n, frame_system::RunToBlockHooks::default());
}

#[cfg(feature = "runtime-benchmarks")]
impl<T: snowbridge_pallet_outbound_queue_v2::Config>
    snowbridge_pallet_outbound_queue_v2::BenchmarkHelper<T> for Test
{
    // not implemented since the MockVerifier is used for tests
    fn initialize_storage(_: snowbridge_beacon_primitives::BeaconHeader, _: H256) {}
}
