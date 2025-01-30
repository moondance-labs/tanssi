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
    frame_support::{
        parameter_types,
        traits::{ConstU32, ConstU64},
    },
    pallet_balances::AccountData,
    parity_scale_codec::{Decode, Encode},
    snowbridge_core::{
        outbound::{Fee, Message, SendError, SendMessageFeeProvider},
        AgentId, ChannelId, ParaId, TokenId,
    },
    sp_core::H256,
    sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup, MaybeEquivalence},
        BuildStorage, DispatchResult,
    },
    sp_std::cell::RefCell,
    tp_bridge::TicketInfo,
    tp_traits::EthereumSystemChannelManager,
    xcm::prelude::*,
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
    /// Detect we called EthereumSystemHandler hook.
    pub static ETHEREUM_SYSTEM_HANDLER_NONCE: RefCell<u64> = const { RefCell::new(0) };
}

pub fn sent_ethereum_message_nonce() -> u64 {
    SENT_ETHEREUM_MESSAGE_NONCE.with(|q| (*q.borrow()))
}

pub fn ethereum_system_handler_nonce() -> u64 {
    ETHEREUM_SYSTEM_HANDLER_NONCE.with(|q| (*q.borrow()))
}

#[derive(Clone, Decode, Default, Encode)]
pub struct DummyTicket;

impl TicketInfo for DummyTicket {
    fn message_id(&self) -> H256 {
        H256::default()
    }
}

pub struct MockOkOutboundQueue;
impl snowbridge_core::outbound::SendMessage for MockOkOutboundQueue {
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

impl SendMessageFeeProvider for MockOkOutboundQueue {
    type Balance = u128;

    fn local_fee() -> Self::Balance {
        1
    }
}
pub struct EthereumSystemHandler;
impl EthereumSystemChannelManager for EthereumSystemHandler {
    fn create_channel(
        _channel_id: ChannelId,
        _agent_id: AgentId,
        _para_id: ParaId,
    ) -> DispatchResult {
        ETHEREUM_SYSTEM_HANDLER_NONCE.with(|r| *r.borrow_mut() += 1);
        Ok(())
    }
}

parameter_types! {
    pub TokenLocation: Location = Here.into();
    pub const EthereumSovereignAccount: u64 = 6;
    pub const FeesAccount: u64 = 7;
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

impl pallet_ethereum_token_transfers::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type OutboundQueue = MockOkOutboundQueue;
    type EthereumSystemHandler = EthereumSystemHandler;
    type EthereumSovereignAccount = EthereumSovereignAccount;
    type FeesAccount = FeesAccount;
    type TokenIdFromLocation = MockTokenIdConvert;
    type TokenLocationReanchored = TokenLocation;
    type WeightInfo = ();
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    let balances = vec![(1, 100), (2, 100), (3, 100), (4, 100), (5, 100)];
    pallet_balances::GenesisConfig::<Test> { balances }
        .assimilate_storage(&mut t)
        .unwrap();

    let ext: sp_io::TestExternalities = t.into();

    ext
}

pub const ALICE: u64 = 1;

pub const INIT_TIMESTAMP: u64 = 30_000;
pub const BLOCK_TIME: u64 = 1000;

pub fn run_to_block(n: u64) {
    let old_block_number = System::block_number();

    for x in old_block_number..n {
        System::reset_events();
        System::set_block_number(x + 1);
        Timestamp::set_timestamp(System::block_number() * BLOCK_TIME + INIT_TIMESTAMP);
    }
}
