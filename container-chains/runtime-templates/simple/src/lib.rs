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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use cumulus_pallet_parachain_system::RelayNumberMonotonicallyIncreases;
#[cfg(feature = "std")]
use sp_version::NativeVersion;

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

pub mod migrations;
pub mod weights;

pub use sp_runtime::{MultiAddress, Perbill, Permill};
use {
    cumulus_primitives_core::AggregateMessageOrigin,
    dp_impl_tanssi_pallets_config::impl_tanssi_pallets_config,
    frame_support::{
        construct_runtime,
        dispatch::DispatchClass,
        genesis_builder_helper::{build_config, create_default_config},
        pallet_prelude::DispatchResult,
        parameter_types,
        traits::{
            ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, Contains, InsideBoth, InstanceFilter,
        },
        weights::{
            constants::{
                BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight,
                WEIGHT_REF_TIME_PER_SECOND,
            },
            ConstantMultiplier, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients,
            WeightToFeePolynomial,
        },
    },
    frame_system::{
        limits::{BlockLength, BlockWeights},
        EnsureRoot,
    },
    nimbus_primitives::{NimbusId, SlotBeacon},
    pallet_transaction_payment::CurrencyAdapter,
    parity_scale_codec::{Decode, Encode},
    polkadot_runtime_common::SlowAdjustingFeeUpdate,
    scale_info::TypeInfo,
    smallvec::smallvec,
    sp_api::impl_runtime_apis,
    sp_consensus_slots::{Slot, SlotDuration},
    sp_core::{MaxEncodedLen, OpaqueMetadata},
    sp_runtime::{
        create_runtime_str, generic, impl_opaque_keys,
        traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, Verify},
        transaction_validity::{TransactionSource, TransactionValidity},
        ApplyExtrinsicResult, MultiSignature,
    },
    sp_std::prelude::*,
    sp_version::RuntimeVersion,
};

pub mod xcm_config;

// Polkadot imports
use polkadot_runtime_common::BlockHashCount;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

pub mod currency {
    use super::Balance;

    pub const MICROUNIT: Balance = 1_000_000;
    pub const MILLIUNIT: Balance = 1_000_000_000;
    pub const UNIT: Balance = 1_000_000_000_000;
    pub const KILOUNIT: Balance = 1_000_000_000_000_000;

    pub const STORAGE_BYTE_FEE: Balance = 100 * MICROUNIT;

    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * 100 * MILLIUNIT + (bytes as Balance) * STORAGE_BYTE_FEE
    }
}

/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
/// node's balance type.
///
/// This should typically create a mapping between the following ranges:
///   - `[0, MAXIMUM_BLOCK_WEIGHT]`
///   - `[Balance::min, Balance::max]`
///
/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
///   - Setting it to `0` will essentially disable the weight fee.
///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
    type Balance = Balance;
    fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
        // in Rococo, extrinsic base weight (smallest non-zero weight) is mapped to 1 MILLIUNIT:
        // in our template, we map to 1/10 of that, or 1/10 MILLIUNIT
        let p = MILLIUNIT / 10;
        let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
        smallvec![WeightToFeeCoefficient {
            degree: 1,
            negative: false,
            coeff_frac: Perbill::from_rational(p % q, q),
            coeff_integer: p / q,
        }]
    }
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use {
        super::*,
        sp_runtime::{generic, traits::BlakeTwo256},
    };

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
}

impl_opaque_keys! {
    pub struct SessionKeys { }
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("container-chain-template"),
    impl_name: create_runtime_str!("container-chain-template"),
    authoring_version: 1,
    spec_version: 700,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

pub const SUPPLY_FACTOR: Balance = 100;

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLIUNIT: Balance = 1_000_000_000;
pub const MICROUNIT: Balance = 1_000_000;

pub const STORAGE_BYTE_FEE: Balance = 100 * MICROUNIT * SUPPLY_FACTOR;

pub const fn deposit(items: u32, bytes: u32) -> Balance {
    items as Balance * 100 * MILLIUNIT * SUPPLY_FACTOR + (bytes as Balance) * STORAGE_BYTE_FEE
}

/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

/// We allow for 0.5 of a second of compute with a 12 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(
    WEIGHT_REF_TIME_PER_SECOND.saturating_div(2),
    cumulus_primitives_core::relay_chain::MAX_POV_SIZE as u64,
);

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;

    // This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
    //  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
    // `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
    // the lazy contract deletion.
    pub RuntimeBlockLength: BlockLength =
        BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
        .base_block(BlockExecutionWeight::get())
        .for_class(DispatchClass::all(), |weights| {
            weights.base_extrinsic = ExtrinsicBaseWeight::get();
        })
        .for_class(DispatchClass::Normal, |weights| {
            weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
        })
        .for_class(DispatchClass::Operational, |weights| {
            weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
            // Operational transactions have some extra reserved space, so that they
            // are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
            weights.reserved = Some(
                MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
            );
        })
        .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
        .build_or_panic();
    pub const SS58Prefix: u16 = 42;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The aggregated dispatch type that is available for extrinsics.
    type RuntimeCall = RuntimeCall;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    /// The index type for storing how many extrinsics an account has signed.
    type Nonce = Index;
    /// The index type for blocks.
    type Block = Block;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    /// The ubiquitous origin type.
    type RuntimeOrigin = RuntimeOrigin;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// Runtime version.
    type Version = Version;
    /// Converts a module to an index of this module in the runtime.
    type PalletInfo = PalletInfo;
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = InsideBoth<MaintenanceMode, TxPause>;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = weights::frame_system::SubstrateWeight<Runtime>;
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = RuntimeBlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = RuntimeBlockLength;
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    /// The action to take on a Runtime Upgrade
    type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type RuntimeTask = RuntimeTask;
}

parameter_types! {
    pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type MaxReserves = ConstU32<50>;
    type ReserveIdentifier = [u8; 8];
    type FreezeIdentifier = RuntimeFreezeReason;
    type MaxFreezes = ConstU32<0>;
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type MaxHolds = ConstU32<0>;
    type WeightInfo = weights::pallet_balances::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const TransactionByteFee: Balance = 1;
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    // This will burn the fees
    type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = WeightToFee;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
}

parameter_types! {
    pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
    pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
    pub const RelayOrigin: AggregateMessageOrigin = AggregateMessageOrigin::Parent;
}

pub const RELAY_CHAIN_SLOT_DURATION_MILLIS: u32 = 6000;
pub const UNINCLUDED_SEGMENT_CAPACITY: u32 = 3;
pub const BLOCK_PROCESSING_VELOCITY: u32 = 1;

type ConsensusHook = pallet_async_backing::consensus_hook::FixedVelocityConsensusHook<
    Runtime,
    BLOCK_PROCESSING_VELOCITY,
    UNINCLUDED_SEGMENT_CAPACITY,
>;

impl cumulus_pallet_parachain_system::Config for Runtime {
    type WeightInfo = weights::cumulus_pallet_parachain_system::SubstrateWeight<Runtime>;
    type RuntimeEvent = RuntimeEvent;
    type OnSystemEvent = ();
    type OutboundXcmpMessageSource = XcmpQueue;
    type SelfParaId = parachain_info::Pallet<Runtime>;
    type DmpQueue = frame_support::traits::EnqueueWithOrigin<MessageQueue, RelayOrigin>;
    type ReservedDmpWeight = ReservedDmpWeight;
    type XcmpMessageHandler = XcmpQueue;
    type ReservedXcmpWeight = ReservedXcmpWeight;
    type CheckAssociatedRelayNumber = RelayNumberMonotonicallyIncreases;
    type ConsensusHook = ConsensusHook;
}

pub struct ParaSlotProvider;
impl sp_core::Get<(Slot, SlotDuration)> for ParaSlotProvider {
    fn get() -> (Slot, SlotDuration) {
        let slot = u64::from(<Runtime as pallet_author_inherent::Config>::SlotBeacon::slot());
        (Slot::from(slot), SlotDuration::from_millis(SLOT_DURATION))
    }
}

parameter_types! {
    pub const ExpectedBlockTime: u64 = MILLISECS_PER_BLOCK;
}

impl pallet_async_backing::Config for Runtime {
    type AllowMultipleBlocksPerSlot = ConstBool<true>;
    type GetAndVerifySlot =
        pallet_async_backing::ParaSlot<RELAY_CHAIN_SLOT_DURATION_MILLIS, ParaSlotProvider>;
    type ExpectedBlockTime = ExpectedBlockTime;
}

impl parachain_info::Config for Runtime {}

parameter_types! {
    pub const Period: u32 = 6 * HOURS;
    pub const Offset: u32 = 0;
}

impl pallet_sudo::Config for Runtime {
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = weights::pallet_sudo::SubstrateWeight<Runtime>;
}

impl pallet_utility::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = weights::pallet_utility::SubstrateWeight<Runtime>;
}

/// The type used to represent the kinds of proxying allowed.
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug, MaxEncodedLen, TypeInfo,
)]
#[allow(clippy::unnecessary_cast)]
pub enum ProxyType {
    /// All calls can be proxied. This is the trivial/most permissive filter.
    Any = 0,
    /// Only extrinsics that do not transfer funds.
    NonTransfer = 1,
    /// Only extrinsics related to governance (democracy and collectives).
    Governance = 2,
    /// Allow to veto an announced proxy call.
    CancelProxy = 3,
    /// Allow extrinsic related to Balances.
    Balances = 4,
}

impl Default for ProxyType {
    fn default() -> Self {
        Self::Any
    }
}

impl InstanceFilter<RuntimeCall> for ProxyType {
    fn filter(&self, c: &RuntimeCall) -> bool {
        // Since proxy filters are respected in all dispatches of the Utility
        // pallet, it should never need to be filtered by any proxy.
        if let RuntimeCall::Utility(..) = c {
            return true;
        }

        match self {
            ProxyType::Any => true,
            ProxyType::NonTransfer => {
                matches!(
                    c,
                    RuntimeCall::System(..)
                        | RuntimeCall::ParachainSystem(..)
                        | RuntimeCall::Timestamp(..)
                        | RuntimeCall::Proxy(..)
                )
            }
            // We don't have governance yet
            ProxyType::Governance => false,
            ProxyType::CancelProxy => matches!(
                c,
                RuntimeCall::Proxy(pallet_proxy::Call::reject_announcement { .. })
            ),
            ProxyType::Balances => {
                matches!(c, RuntimeCall::Balances(..))
            }
        }
    }

    fn is_superset(&self, o: &Self) -> bool {
        match (self, o) {
            (x, y) if x == y => true,
            (ProxyType::Any, _) => true,
            (_, ProxyType::Any) => false,
            _ => false,
        }
    }
}

impl pallet_proxy::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type Currency = Balances;
    type ProxyType = ProxyType;
    // One storage item; key size 32, value size 8
    type ProxyDepositBase = ConstU128<{ deposit(1, 8) }>;
    // Additional storage item size of 33 bytes (32 bytes AccountId + 1 byte sizeof(ProxyType)).
    type ProxyDepositFactor = ConstU128<{ deposit(0, 33) }>;
    type MaxProxies = ConstU32<32>;
    type MaxPending = ConstU32<32>;
    type CallHasher = BlakeTwo256;
    type AnnouncementDepositBase = ConstU128<{ deposit(1, 8) }>;
    // Additional storage item size of 68 bytes:
    // - 32 bytes AccountId
    // - 32 bytes Hasher (Blake2256)
    // - 4 bytes BlockNumber (u32)
    type AnnouncementDepositFactor = ConstU128<{ deposit(0, 68) }>;
    type WeightInfo = weights::pallet_proxy::SubstrateWeight<Runtime>;
}

pub struct XcmExecutionManager;
impl xcm_primitives::PauseXcmExecution for XcmExecutionManager {
    fn suspend_xcm_execution() -> DispatchResult {
        XcmpQueue::suspend_xcm_execution(RuntimeOrigin::root())
    }
    fn resume_xcm_execution() -> DispatchResult {
        XcmpQueue::resume_xcm_execution(RuntimeOrigin::root())
    }
}

impl pallet_migrations::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MigrationsList = (migrations::TemplateMigrations<Runtime, XcmpQueue, PolkadotXcm>,);
    type XcmExecutionManager = XcmExecutionManager;
}

/// Maintenance mode Call filter
pub struct MaintenanceFilter;
impl Contains<RuntimeCall> for MaintenanceFilter {
    fn contains(c: &RuntimeCall) -> bool {
        !matches!(c, RuntimeCall::Balances(_) | RuntimeCall::PolkadotXcm(_))
    }
}

/// Normal Call Filter
/// We dont allow to create nor mint assets, this for now is disabled
/// We only allow transfers. For now creation of assets will go through
/// asset-manager, while minting/burning only happens through xcm messages
/// This can change in the future
pub struct NormalFilter;
impl Contains<RuntimeCall> for NormalFilter {
    fn contains(_c: &RuntimeCall) -> bool {
        true
    }
}

impl pallet_maintenance_mode::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type NormalCallFilter = NormalFilter;
    type MaintenanceCallFilter = MaintenanceFilter;
    type MaintenanceOrigin = EnsureRoot<AccountId>;
    type XcmExecutionManager = XcmExecutionManager;
}

impl pallet_root_testing::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl pallet_tx_pause::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type PauseOrigin = EnsureRoot<AccountId>;
    type UnpauseOrigin = EnsureRoot<AccountId>;
    type WhitelistedCalls = ();
    type MaxNameLen = ConstU32<256>;
    type WeightInfo = weights::pallet_tx_pause::SubstrateWeight<Runtime>;
}

impl dp_impl_tanssi_pallets_config::Config for Runtime {
    const SLOT_DURATION: u64 = SLOT_DURATION;
    type TimestampWeights = weights::pallet_timestamp::SubstrateWeight<Runtime>;
    type AuthorInherentWeights = weights::pallet_author_inherent::SubstrateWeight<Runtime>;
    type AuthoritiesNotingWeights = weights::pallet_cc_authorities_noting::SubstrateWeight<Runtime>;
}

parameter_types! {
    // One storage item; key size 32; value is size 4+4+16+32. Total = 1 * (32 + 56)
    pub const DepositBase: Balance = currency::deposit(1, 88);
    // Additional storage item size of 32 bytes.
    pub const DepositFactor: Balance = currency::deposit(0, 32);
    pub const MaxSignatories: u32 = 100;
}

impl pallet_multisig::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type Currency = Balances;
    type DepositBase = DepositBase;
    type DepositFactor = DepositFactor;
    type MaxSignatories = MaxSignatories;
    type WeightInfo = weights::pallet_multisig::SubstrateWeight<Runtime>;
}

impl_tanssi_pallets_config!(Runtime);

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub enum Runtime
    {
        // System support stuff.
        System: frame_system = 0,
        ParachainSystem: cumulus_pallet_parachain_system = 1,
        Timestamp: pallet_timestamp = 2,
        ParachainInfo: parachain_info = 3,
        Sudo: pallet_sudo = 4,
        Utility: pallet_utility = 5,
        Proxy: pallet_proxy = 6,
        Migrations: pallet_migrations = 7,
        MaintenanceMode: pallet_maintenance_mode = 8,
        TxPause: pallet_tx_pause = 9,

        // Monetary stuff.
        Balances: pallet_balances = 10,
        TransactionPayment: pallet_transaction_payment = 11,

        // Other utilities
        Multisig: pallet_multisig = 16,

        // ContainerChain Author Verification
        AuthoritiesNoting: pallet_cc_authorities_noting = 50,
        AuthorInherent: pallet_author_inherent = 51,

        // XCM
        XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Storage, Event<T>} = 70,
        CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin} = 71,
        DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>} = 72,
        PolkadotXcm: pallet_xcm::{Pallet, Call, Storage, Event<T>, Origin, Config<T>} = 73,
        MessageQueue: pallet_message_queue::{Pallet, Call, Storage, Event<T>} = 74,
        ForeignAssets: pallet_assets::<Instance1>::{Pallet, Call, Storage, Event<T>} = 75,
        ForeignAssetsCreator: pallet_foreign_asset_creator::{Pallet, Call, Storage, Event<T>} = 76,
        AssetRate: pallet_asset_rate::{Pallet, Call, Storage, Event<T>} = 77,
        XcmExecutorUtils: pallet_xcm_executor_utils::{Pallet, Call, Storage, Event<T>} = 78,

        RootTesting: pallet_root_testing = 100,
        AsyncBacking: pallet_async_backing::{Pallet, Storage} = 110,

    }
);

#[cfg(feature = "runtime-benchmarks")]
mod benches {
    frame_benchmarking::define_benchmarks!(
        [frame_system, frame_system_benchmarking::Pallet::<Runtime>]
        [cumulus_pallet_parachain_system, ParachainSystem]
        [pallet_timestamp, Timestamp]
        [pallet_sudo, Sudo]
        [pallet_utility, Utility]
        [pallet_proxy, Proxy]
        [pallet_tx_pause, TxPause]
        [pallet_balances, Balances]
        [pallet_multisig, Multisig]
        [pallet_cc_authorities_noting, AuthoritiesNoting]
        [pallet_author_inherent, AuthorInherent]
        [cumulus_pallet_xcmp_queue, XcmpQueue]
        [cumulus_pallet_dmp_queue, DmpQueue]
        [pallet_xcm, PalletXcmExtrinsicsBenchmark::<Runtime>]
        [pallet_xcm_benchmarks::generic, pallet_xcm_benchmarks::generic::Pallet::<Runtime>]
        [pallet_message_queue, MessageQueue]
        [pallet_assets, ForeignAssets]
        [pallet_foreign_asset_creator, ForeignAssetsCreator]
        [pallet_asset_rate, AssetRate]
        [pallet_xcm_executor_utils, XcmExecutorUtils]
    );
}

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
            SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
        fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
            ParachainSystem::collect_collation_info(header)
        }
    }

    impl async_backing_primitives::UnincludedSegmentApi<Block> for Runtime {
        fn can_build_upon(
            included_hash: <Block as BlockT>::Hash,
            slot: async_backing_primitives::Slot,
        ) -> bool {
            ConsensusHook::can_build_upon(included_hash, slot)
        }
    }

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn create_default_config() -> Vec<u8> {
            create_default_config::<RuntimeGenesisConfig>()
        }

        fn build_config(config: Vec<u8>) -> sp_genesis_builder::Result {
            build_config::<RuntimeGenesisConfig>(config)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn benchmark_metadata(
            extra: bool,
        ) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {
            use frame_benchmarking::{Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;
            use pallet_xcm::benchmarking::Pallet as PalletXcmExtrinsicsBenchmark;

            let mut list = Vec::<BenchmarkList>::new();
            list_benchmarks!(list, extra);

            let storage_info = AllPalletsWithSystem::storage_info();
            (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig,
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{BenchmarkBatch, Benchmarking, BenchmarkError};
            use sp_core::storage::TrackedStorageKey;
            use staging_xcm::latest::prelude::*;
            impl frame_system_benchmarking::Config for Runtime {
                fn setup_set_code_requirements(code: &sp_std::vec::Vec<u8>) -> Result<(), BenchmarkError> {
                    ParachainSystem::initialize_for_set_code_benchmark(code.len() as u32);
                    Ok(())
                }

                fn verify_set_code() {
                    System::assert_last_event(cumulus_pallet_parachain_system::Event::<Runtime>::ValidationFunctionStored.into());
                }
            }
            use crate::xcm_config::SelfReserve;
            parameter_types! {
                pub ExistentialDepositAsset: Option<MultiAsset> = Some((
                    SelfReserve::get(),
                    ExistentialDeposit::get()
                ).into());
            }

            impl pallet_xcm_benchmarks::Config for Runtime {
                type XcmConfig = xcm_config::XcmConfig;
                type AccountIdConverter = xcm_config::LocationToAccountId;
                type DeliveryHelper = cumulus_primitives_utility::ToParentDeliveryHelper<
                xcm_config::XcmConfig,
                ExistentialDepositAsset,
                xcm_config::PriceForParentDelivery,
                >;
                fn valid_destination() -> Result<MultiLocation, BenchmarkError> {
                    Ok(MultiLocation::parent())
                }
                fn worst_case_holding(_depositable_count: u32) -> MultiAssets {
                    // We only care for native asset until we support others
                    // TODO: refactor this case once other assets are supported
                    vec![MultiAsset{
                        id: Concrete(MultiLocation::here()),
                        fun: Fungible(u128::MAX),
                    }].into()
                }
            }

            impl pallet_xcm_benchmarks::generic::Config for Runtime {
                type TransactAsset = Balances;
                type RuntimeCall = RuntimeCall;

                fn worst_case_response() -> (u64, Response) {
                    (0u64, Response::Version(Default::default()))
                }

                fn worst_case_asset_exchange() -> Result<(MultiAssets, MultiAssets), BenchmarkError> {
                    Err(BenchmarkError::Skip)
                }

                fn universal_alias() -> Result<(MultiLocation, Junction), BenchmarkError> {
                    Err(BenchmarkError::Skip)
                }

                fn transact_origin_and_runtime_call() -> Result<(MultiLocation, RuntimeCall), BenchmarkError> {
                    Ok((MultiLocation::parent(), frame_system::Call::remark_with_event { remark: vec![] }.into()))
                }

                fn subscribe_origin() -> Result<MultiLocation, BenchmarkError> {
                    Ok(MultiLocation::parent())
                }

                fn claimable_asset() -> Result<(MultiLocation, MultiLocation, MultiAssets), BenchmarkError> {
                    let origin = MultiLocation::parent();
                    let assets: MultiAssets = (Concrete(MultiLocation::parent()), 1_000u128).into();
                    let ticket = MultiLocation { parents: 0, interior: Here };
                    Ok((origin, ticket, assets))
                }

                fn unlockable_asset() -> Result<(MultiLocation, MultiLocation, MultiAsset), BenchmarkError> {
                    Err(BenchmarkError::Skip)
                }

                fn export_message_origin_and_destination(
                ) -> Result<(MultiLocation, NetworkId, InteriorMultiLocation), BenchmarkError> {
                    Err(BenchmarkError::Skip)
                }

                fn alias_origin() -> Result<(MultiLocation, MultiLocation), BenchmarkError> {
                    Err(BenchmarkError::Skip)
                }
            }

            use pallet_xcm::benchmarking::Pallet as PalletXcmExtrinsicsBenchmark;
            impl pallet_xcm::benchmarking::Config for Runtime {
                fn reachable_dest() -> Option<MultiLocation> {
                    Some(Parent.into())
                }

                fn teleportable_asset_and_dest() -> Option<(MultiAsset, MultiLocation)> {
                    // Relay/native token can be teleported between AH and Relay.
                    Some((
                        MultiAsset {
                            fun: Fungible(EXISTENTIAL_DEPOSIT),
                            id: Concrete(Parent.into())
                        },
                        Parent.into(),
                    ))
                }

                fn reserve_transferable_asset_and_dest() -> Option<(MultiAsset, MultiLocation)> {
                    use xcm_config::SelfReserve;
                    // AH can reserve transfer native token to some random parachain.
                    let random_para_id = 43211234;
                    ParachainSystem::open_outbound_hrmp_channel_for_benchmarks_or_tests(
                        random_para_id.into()
                    );
                    Some((
                        MultiAsset {
                            fun: Fungible(EXISTENTIAL_DEPOSIT),
                            id: Concrete(SelfReserve::get())
                        },
                        ParentThen(Parachain(random_para_id).into()).into(),
                    ))
                }

                fn set_up_complex_asset_transfer(
                ) -> Option<(MultiAssets, u32, MultiLocation, Box<dyn FnOnce()>)> {
                    use xcm_config::SelfReserve;
                    // Transfer to Relay some local AH asset (local-reserve-transfer) while paying
                    // fees using teleported native token.
                    // (We don't care that Relay doesn't accept incoming unknown AH local asset)
                    let dest = Parent.into();

                    let fee_amount = EXISTENTIAL_DEPOSIT;
                    let fee_asset: MultiAsset = (SelfReserve::get(), fee_amount).into();

                    let who = frame_benchmarking::whitelisted_caller();
                    // Give some multiple of the existential deposit
                    let balance = fee_amount + EXISTENTIAL_DEPOSIT * 1000;
                    let _ = <Balances as frame_support::traits::Currency<_>>::make_free_balance_be(
                        &who, balance,
                    );

                    // verify initial balance
                    assert_eq!(Balances::free_balance(&who), balance);

                    // set up local asset
                    let asset_amount = 10u128;
                    let initial_asset_amount = asset_amount * 10;

                    let (asset_id, asset_location) = pallet_foreign_asset_creator::benchmarks::create_default_minted_asset::<Runtime>(
                        initial_asset_amount,
                        who.clone()
                    );

                    let transfer_asset: MultiAsset = (asset_location, asset_amount).into();

                    let assets: MultiAssets = vec![fee_asset.clone(), transfer_asset].into();
                    let fee_index = if assets.get(0).unwrap().eq(&fee_asset) { 0 } else { 1 };

                    // verify transferred successfully
                    let verify = Box::new(move || {
                        // verify native balance after transfer, decreased by transferred fee amount
                        // (plus transport fees)
                        assert!(Balances::free_balance(&who) <= balance - fee_amount);
                        // verify asset balance decreased by exactly transferred amount
                        assert_eq!(
                            ForeignAssets::balance(asset_id, &who),
                            initial_asset_amount - asset_amount,
                        );
                    });
                    Some((assets, fee_index as u32, dest, verify))
                }
            }

            let whitelist: Vec<TrackedStorageKey> = vec![
                // Block Number
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac")
                    .to_vec()
                    .into(),
                // Total Issuance
                hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80")
                    .to_vec()
                    .into(),
                // Execution Phase
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a")
                    .to_vec()
                    .into(),
                // Event Count
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850")
                    .to_vec()
                    .into(),
                // System Events
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7")
                    .to_vec()
                    .into(),
                // The transactional storage limit.
                hex_literal::hex!("3a7472616e73616374696f6e5f6c6576656c3a")
                    .to_vec()
                    .into(),

                // ParachainInfo ParachainId
                hex_literal::hex!(  "0d715f2646c8f85767b5d2764bb2782604a74d81251e398fd8a0a4d55023bb3f")
                    .to_vec()
                    .into(),
            ];

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);

            add_benchmarks!(params, batches);

            Ok(batches)
        }
    }

    #[cfg(feature = "try-runtime")]
    impl frame_try_runtime::TryRuntime<Block> for Runtime {
        fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
            let weight = Executive::try_runtime_upgrade(checks).unwrap();
            (weight, RuntimeBlockWeights::get().max_block)
        }

        fn execute_block(
            block: Block,
            state_root_check: bool,
            signature_check: bool,
            select: frame_try_runtime::TryStateSelect,
        ) -> Weight {
            // NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
            // have a backtrace here.
            Executive::try_execute_block(block, state_root_check, signature_check, select).unwrap()
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
    for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }

        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }

        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }

        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl dp_slot_duration_runtime_api::TanssiSlotDurationApi<Block> for Runtime {
        fn slot_duration() -> u64 {
            SLOT_DURATION
        }
    }
}

#[allow(dead_code)]
struct CheckInherents;

#[allow(deprecated)]
impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
    fn check_inherents(
        block: &Block,
        relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
    ) -> sp_inherents::CheckInherentsResult {
        let relay_chain_slot = relay_state_proof
            .read_slot()
            .expect("Could not read the relay chain slot from the proof");

        let inherent_data =
            cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
                relay_chain_slot,
                sp_std::time::Duration::from_secs(6),
            )
            .create_inherent_data()
            .expect("Could not create the timestamp inherent data");

        inherent_data.check_extrinsics(block)
    }
}

cumulus_pallet_parachain_system::register_validate_block! {
    Runtime = Runtime,
    CheckInherents = CheckInherents,
    BlockExecutor = pallet_author_inherent::BlockExecutor::<Runtime, Executive>,
}
