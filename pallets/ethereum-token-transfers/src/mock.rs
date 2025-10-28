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
    crate as pallet_ethereum_token_transfers,
    core::cell::RefCell,
    frame_support::{
        pallet_prelude::OriginTrait,
        parameter_types,
        traits::{ConstU32, ConstU64},
    },
    pallet_balances::AccountData,
    parity_scale_codec::{Decode, Encode},
    snowbridge_core::{
        location::{DescribeGlobalPrefix, DescribeTokenTerminal},
        AgentId, ChannelId, ParaId, TokenId,
    },
    snowbridge_outbound_queue_primitives::v1::{Fee, Message},
    snowbridge_outbound_queue_primitives::{SendError, SendMessageFeeProvider},
    sp_core::H256,
    sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup, MaybeEquivalence, TryConvert},
        BuildStorage,
    },
    std::marker::PhantomData,
    tp_bridge::{ChannelInfo, EthereumSystemChannelManager, TicketInfo},
    xcm::prelude::*,
    xcm_builder::{DescribeFamily, DescribeLocation, HashedDescription},
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances,
        Timestamp: pallet_timestamp,
        EthereumTokenTransfers: pallet_ethereum_token_transfers,
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
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = AccountData<u128>;
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

parameter_types! {
    pub const ExistentialDeposit: u64 = 5;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Balance = u128;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type ReserveIdentifier = [u8; 8];
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type FreezeIdentifier = ();
    type MaxLocks = ();
    type MaxReserves = MaxReserves;
    type MaxFreezes = ConstU32<0>;
    type DoneSlashHandler = ();
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<5>;
    type WeightInfo = ();
}

thread_local! {
    /// Detect we sent a message to Ethereum.
    pub static SENT_ETHEREUM_MESSAGE_NONCE: RefCell<u64> = const { RefCell::new(0) };
    /// Detect we sent a message to Ethereum in V2.
    pub static SENT_ETHEREUM_MESSAGE_NONCE_V2: RefCell<u64> = const { RefCell::new(0) };
    /// Detect we called EthereumSystemHandler hook.
    pub static ETHEREUM_SYSTEM_HANDLER_NONCE: RefCell<u64> = const { RefCell::new(0) };
}

pub fn sent_ethereum_message_nonce() -> u64 {
    SENT_ETHEREUM_MESSAGE_NONCE.with(|q| (*q.borrow()))
}

pub fn sent_ethereum_message_nonce_v2() -> u64 {
    SENT_ETHEREUM_MESSAGE_NONCE_V2.with(|q| (*q.borrow()))
}

pub fn ethereum_system_handler_nonce() -> u64 {
    ETHEREUM_SYSTEM_HANDLER_NONCE.with(|q| (*q.borrow()))
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

pub struct MockOkOutboundQueue;
impl snowbridge_outbound_queue_primitives::v1::SendMessage for MockOkOutboundQueue {
    type Ticket = DummyTicket;

    fn deliver(_: Self::Ticket) -> Result<H256, SendError> {
        // Every time we hit deliver, increment the nonce
        SENT_ETHEREUM_MESSAGE_NONCE.with(|r| *r.borrow_mut() += 1);
        Ok(H256::zero())
    }

    fn validate(
        _message: &Message,
    ) -> Result<(Self::Ticket, Fee<<Self as SendMessageFeeProvider>::Balance>), SendError> {
        Ok((
            DummyTicket::default(),
            Fee {
                local: 20u128,
                remote: 30u128,
            },
        ))
    }
}

pub struct MockOkOutboundQueueV2;
impl snowbridge_outbound_queue_primitives::v2::SendMessage for MockOkOutboundQueueV2 {
    type Ticket = DummyTicket;

    fn deliver(_: Self::Ticket) -> Result<H256, snowbridge_outbound_queue_primitives::SendError> {
        // Every time we hit deliver, increment the nonce
        SENT_ETHEREUM_MESSAGE_NONCE_V2.with(|r| *r.borrow_mut() += 1);
        Ok(H256::zero())
    }

    fn validate(
        _message: &snowbridge_outbound_queue_primitives::v2::Message,
    ) -> Result<Self::Ticket, snowbridge_outbound_queue_primitives::SendError> {
        Ok(DummyTicket::default())
    }
}

impl SendMessageFeeProvider for MockOkOutboundQueue {
    type Balance = u128;

    fn local_fee() -> Self::Balance {
        1
    }
}
pub struct EthereumSystemHandler;
impl EthereumSystemChannelManager for EthereumSystemHandler {
    fn create_channel(channel_id: ChannelId, agent_id: AgentId, para_id: ParaId) -> ChannelInfo {
        ETHEREUM_SYSTEM_HANDLER_NONCE.with(|r| *r.borrow_mut() += 1);
        ChannelInfo {
            channel_id,
            agent_id,
            para_id,
        }
    }
}

parameter_types! {
    pub TokenLocation: Location = Here.into();
    pub const EthereumSovereignAccount: u64 = 6;
    pub const FeesAccount: u64 = 7;
    pub storage ShouldUseV2: bool = false;
    pub EthereumNetwork: NetworkId = NetworkId::Ethereum { chain_id: 11155111 };
    pub EthereumLocation: Location = Location::new(1, EthereumNetwork::get());
    pub const MinV2Reward: u128 = 1u128;
    pub const ThisNetwork: NetworkId = NetworkId::Polkadot;
    pub UniversalLocation: InteriorLocation = ThisNetwork::get().into();
}

pub struct MockTokenIdConvert;
impl MaybeEquivalence<TokenId, Location> for MockTokenIdConvert {
    fn convert(_id: &TokenId) -> Option<Location> {
        Some(Location::parent())
    }
    fn convert_back(loc: &Location) -> Option<TokenId> {
        if *loc == Location::here() {
            Some(H256::repeat_byte(0x01))
        } else {
            None
        }
    }
}

/// `Convert` implementation to convert from some a `Signed` (system) `Origin` into an
/// `AccountIndex64`.
///
/// Typically used when configuring `pallet-xcm` in tests to allow `u64` accounts to dispatch an XCM
/// from an `AccountIndex64` origin.
pub struct MockSignedToAccountIndex64<RuntimeOrigin, AccountId>(
    PhantomData<(RuntimeOrigin, AccountId)>,
);
impl<RuntimeOrigin: OriginTrait + Clone, AccountId: Into<u64>> TryConvert<RuntimeOrigin, Location>
    for MockSignedToAccountIndex64<RuntimeOrigin, AccountId>
where
    RuntimeOrigin::PalletsOrigin: From<frame_system::RawOrigin<AccountId>>
        + TryInto<frame_system::RawOrigin<AccountId>, Error = RuntimeOrigin::PalletsOrigin>,
{
    fn try_convert(o: RuntimeOrigin) -> Result<Location, RuntimeOrigin> {
        o.try_with_caller(|caller| match caller.try_into() {
            Ok(frame_system::RawOrigin::Signed(who)) => Ok(Junction::AccountIndex64 {
                network: None,
                index: who.into(),
            }
            .into()),
            Ok(other) => Err(other.into()),
            Err(other) => Err(other),
        })
    }
}

pub struct DescribeAccountAccountIndex64Terminal;
impl DescribeLocation for DescribeAccountAccountIndex64Terminal {
    fn describe_location(l: &Location) -> Option<Vec<u8>> {
        match l.unpack() {
            (0, [AccountIndex64 { index, .. }]) => {
                if index == &PROHIBITED_ACCOUNT {
                    None
                } else {
                    println!("index g{:?}", index);
                }
            }
            _ => return None,
        }
    }
}

pub type LocalOriginToLocation = MockSignedToAccountIndex64<RuntimeOrigin, u64>;

pub type MockAgentIdOf =
    HashedDescription<H256, DescribeGlobalPrefix<DescribeAccountAccountIndex64Terminal>>;
impl pallet_ethereum_token_transfers::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type OutboundQueue = MockOkOutboundQueue;
    type OutboundQueueV2 = MockOkOutboundQueueV2;
    type EthereumSystemHandler = EthereumSystemHandler;
    type EthereumSovereignAccount = EthereumSovereignAccount;
    type FeesAccount = FeesAccount;
    type TokenIdFromLocation = MockTokenIdConvert;
    type TokenLocationReanchored = TokenLocation;
    type ShouldUseV2 = ShouldUseV2;
    type LocationHashOf = MockAgentIdOf;
    type EthereumLocation = EthereumLocation;
    type MinV2Reward = MinV2Reward;
    type OriginToLocation = LocalOriginToLocation;
    type UniversalLocation = UniversalLocation;
    type WeightInfo = ();
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    let balances = vec![(1, 100), (2, 100), (3, 100), (4, 100), (5, 100)];
    pallet_balances::GenesisConfig::<Test> {
        balances,
        ..Default::default()
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let ext: sp_io::TestExternalities = t.into();

    ext
}

pub const ALICE: u64 = 1;
pub const PROHIBITED_ACCOUNT: u64 = 5;

pub fn run_to_block(n: u64) {
    System::run_to_block_with::<AllPalletsWithSystem>(n, frame_system::RunToBlockHooks::default());
}
