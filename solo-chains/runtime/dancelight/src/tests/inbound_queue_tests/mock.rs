// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>

use crate::symbiotic_message_processor::SymbioticMessageProcessor;
use frame_support::traits::fungible::Mutate;
use frame_support::traits::{UnixTime, ValidatorRegistration};
use frame_support::{derive_impl, parameter_types, traits::ConstU32, weights::IdentityFee};
use frame_system::EnsureRoot;
use hex_literal::hex;
use polkadot_parachain_primitives::primitives::Id as ParaId;
use snowbridge_beacon_primitives::{Fork, ForkVersions};
use snowbridge_core::inbound::Verifier;
use snowbridge_core::sibling_sovereign_account;
use snowbridge_core::{
    gwei,
    inbound::{Log, Proof, VerificationError},
    meth, Channel, ChannelId, PricingParameters, Rewards, StaticLookup,
};
use snowbridge_pallet_inbound_queue::xcm_message_processor::XcmMessageProcessor;
use snowbridge_router_primitives::inbound::envelope::Envelope;
use snowbridge_router_primitives::inbound::{MessageProcessor, MessageToXcm};
use sp_core::Encode;
use sp_core::{H160, H256};
use sp_runtime::{
    traits::{IdentifyAccount, IdentityLookup, Verify},
    BuildStorage, DispatchError, FixedU128, MultiSignature,
};
use sp_staking::SessionIndex;
use sp_std::{convert::From, default::Default};
use std::time::Duration;
use xcm::latest::SendError as XcmpSendError;
use xcm::{latest::SendXcm, prelude::*};
use xcm_executor::traits::TransactAsset;
use xcm_executor::AssetsInHolding;

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system::{Pallet, Call, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        EthereumBeaconClient: snowbridge_pallet_ethereum_client::{Pallet, Call, Storage, Event<T>},
        InboundQueue: snowbridge_pallet_inbound_queue::{Pallet, Call, Storage, Event<T>},
        ExternalValidators: pallet_external_validators::{Pallet, Call, Storage, Event<T>}
    }
);

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

type Balance = u128;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type AccountData = pallet_balances::AccountData<u128>;
    type Block = Block;
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 1;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
    type Balance = Balance;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
}

parameter_types! {
    pub const ChainForkVersions: ForkVersions = ForkVersions{
        genesis: Fork {
            version: [0, 0, 0, 1], // 0x00000001
            epoch: 0,
        },
        altair: Fork {
            version: [1, 0, 0, 1], // 0x01000001
            epoch: 0,
        },
        bellatrix: Fork {
            version: [2, 0, 0, 1], // 0x02000001
            epoch: 0,
        },
        capella: Fork {
            version: [3, 0, 0, 1], // 0x03000001
            epoch: 0,
        },
        deneb: Fork {
            version: [4, 0, 0, 1], // 0x04000001
            epoch: 4294967295,
        }
    };
}

impl snowbridge_pallet_ethereum_client::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type ForkVersions = ChainForkVersions;
    type WeightInfo = ();
}

// Mock verifier
pub struct MockVerifier;

impl Verifier for MockVerifier {
    fn verify(_: &Log, _: &Proof) -> Result<(), VerificationError> {
        Ok(())
    }
}

const GATEWAY_ADDRESS: [u8; 20] = hex!["eda338e4dc46038493b885327842fd3e301cab39"];

parameter_types! {
    pub const EthereumNetwork: xcm::v3::NetworkId = xcm::v3::NetworkId::Ethereum { chain_id: 11155111 };
    pub const GatewayAddress: H160 = H160(GATEWAY_ADDRESS);
    pub const CreateAssetCall: [u8;2] = [53, 0];
    pub const CreateAssetExecutionFee: u128 = 2_000_000_000;
    pub const CreateAssetDeposit: u128 = 100_000_000_000;
    pub const SendTokenExecutionFee: u128 = 1_000_000_000;
    pub const InitialFund: u128 = 1_000_000_000_000;
    pub const InboundQueuePalletInstance: u8 = 80;
}

#[cfg(feature = "runtime-benchmarks")]
impl<T: snowbridge_pallet_ethereum_client::Config> BenchmarkHelper<T> for Test {
    // not implemented since the MockVerifier is used for tests
    fn initialize_storage(_: BeaconHeader, _: H256) {}
}

// Mock XCM sender that always succeeds
pub struct MockXcmSender;

impl SendXcm for MockXcmSender {
    type Ticket = Xcm<()>;

    fn validate(
        dest: &mut Option<Location>,
        xcm: &mut Option<Xcm<()>>,
    ) -> SendResult<Self::Ticket> {
        if let Some(location) = dest {
            match location.unpack() {
                (_, [Parachain(1001)]) => return Err(XcmpSendError::NotApplicable),
                _ => Ok((xcm.clone().unwrap(), Assets::default())),
            }
        } else {
            Ok((xcm.clone().unwrap(), Assets::default()))
        }
    }

    fn deliver(xcm: Self::Ticket) -> core::result::Result<XcmHash, XcmpSendError> {
        let hash = xcm.using_encoded(sp_io::hashing::blake2_256);
        Ok(hash)
    }
}

parameter_types! {
    pub const OwnParaId: ParaId = ParaId::new(1013);
    pub Parameters: PricingParameters<u128> = PricingParameters {
        exchange_rate: FixedU128::from_rational(1, 400),
        fee_per_gas: gwei(20),
        rewards: Rewards { local: DOT, remote: meth(1) },
        multiplier: FixedU128::from_rational(1, 1),
    };
}

pub const DOT: u128 = 10_000_000_000;

pub const MOCK_CHANNEL_ID: [u8; 32] =
    hex!("c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539");

pub struct MockChannelLookup;
impl StaticLookup for MockChannelLookup {
    type Source = ChannelId;
    type Target = Channel;

    fn lookup(channel_id: Self::Source) -> Option<Self::Target> {
        if channel_id != MOCK_CHANNEL_ID.into() {
            return None;
        }
        Some(Channel {
            agent_id: H256::zero(),
            para_id: ASSET_HUB_PARAID.into(),
        })
    }
}

pub struct SuccessfulTransactor;
impl TransactAsset for SuccessfulTransactor {
    fn can_check_in(_origin: &Location, _what: &Asset, _context: &XcmContext) -> XcmResult {
        Ok(())
    }

    fn can_check_out(_dest: &Location, _what: &Asset, _context: &XcmContext) -> XcmResult {
        Ok(())
    }

    fn deposit_asset(_what: &Asset, _who: &Location, _context: Option<&XcmContext>) -> XcmResult {
        Ok(())
    }

    fn withdraw_asset(
        _what: &Asset,
        _who: &Location,
        _context: Option<&XcmContext>,
    ) -> Result<AssetsInHolding, XcmError> {
        Ok(AssetsInHolding::default())
    }

    fn internal_transfer_asset(
        _what: &Asset,
        _from: &Location,
        _to: &Location,
        _context: &XcmContext,
    ) -> Result<AssetsInHolding, XcmError> {
        Ok(AssetsInHolding::default())
    }
}

pub struct DummyPrefix;

impl MessageProcessor for DummyPrefix {
    fn can_process_message(_channel: &Channel, _envelope: &Envelope) -> bool {
        false
    }

    fn process_message(_channel: Channel, _envelope: Envelope) -> Result<(), DispatchError> {
        panic!("DummyPrefix::process_message shouldn't be called");
    }
}

pub struct DummySuffix;

impl MessageProcessor for DummySuffix {
    fn can_process_message(_channel: &Channel, _envelope: &Envelope) -> bool {
        true
    }

    fn process_message(_channel: Channel, _envelope: Envelope) -> Result<(), DispatchError> {
        panic!("DummySuffix::process_message shouldn't be called");
    }
}

impl snowbridge_pallet_inbound_queue::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Verifier = MockVerifier;
    type Token = Balances;
    type XcmSender = MockXcmSender;
    type WeightInfo = ();
    type GatewayAddress = GatewayAddress;
    type MessageConverter = MessageToXcm<
        CreateAssetCall,
        CreateAssetDeposit,
        InboundQueuePalletInstance,
        AccountId,
        Balance,
    >;
    type PricingParameters = Parameters;
    type ChannelLookup = MockChannelLookup;
    #[cfg(feature = "runtime-benchmarks")]
    type Helper = Test;
    type WeightToFee = IdentityFee<u128>;
    type LengthToFee = IdentityFee<u128>;
    type MaxMessageSize = ConstU32<1024>;
    type AssetTransactor = SuccessfulTransactor;
    type MessageProcessor = (
        DummyPrefix,
        XcmMessageProcessor<Test>,
        SymbioticMessageProcessor<Test>,
        DummySuffix,
    );
}

pub struct ValidatorIdOf;
impl sp_runtime::traits::Convert<AccountId, Option<AccountId>> for ValidatorIdOf {
    fn convert(a: AccountId) -> Option<AccountId> {
        Some(a)
    }
}

pub struct DummyValidatorRegistration;

impl ValidatorRegistration<AccountId> for DummyValidatorRegistration {
    fn is_registered(_id: &AccountId) -> bool {
        true
    }
}

parameter_types! {

    pub const MaxWhitelistedValidators: u32 = 100;
    pub const MaxExternalValidators: u32 = 100;
    pub const SessionsPerEra: SessionIndex = runtime_common::prod_or_fast!(6, 3);
}

pub struct DummyUnixTime;

impl UnixTime for DummyUnixTime {
    fn now() -> Duration {
        Duration::default()
    }
}

impl pallet_external_validators::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type UpdateOrigin = EnsureRoot<AccountId>;
    type HistoryDepth = ConstU32<84>;
    type MaxWhitelistedValidators = MaxWhitelistedValidators;
    type MaxExternalValidators = MaxExternalValidators;
    type ValidatorId = AccountId;
    type ValidatorIdOf = ValidatorIdOf;
    type ValidatorRegistration = DummyValidatorRegistration;
    type UnixTime = DummyUnixTime;
    type SessionsPerEra = SessionsPerEra;
    type OnEraStart = ();
    type OnEraEnd = ();
    type WeightInfo = ();
    #[cfg(feature = "runtime-benchmarks")]
    type Currency = Balances;
}

pub fn setup() {
    System::set_block_number(1);
    Balances::mint_into(
        &sibling_sovereign_account::<Test>(ASSET_HUB_PARAID.into()),
        InitialFund::get(),
    )
    .unwrap();
    Balances::mint_into(
        &sibling_sovereign_account::<Test>(TEMPLATE_PARAID.into()),
        InitialFund::get(),
    )
    .unwrap();
}

pub fn mock_ext() -> sp_io::TestExternalities {
    let storage = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    let mut ext: sp_io::TestExternalities = storage.into();
    ext.execute_with(setup);
    ext
}

pub const ASSET_HUB_PARAID: u32 = 1000u32;
pub const TEMPLATE_PARAID: u32 = 1001u32;
