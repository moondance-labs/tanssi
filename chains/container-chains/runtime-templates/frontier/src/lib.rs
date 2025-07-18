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

extern crate alloc;

use cumulus_pallet_parachain_system::RelayNumberMonotonicallyIncreases;
#[cfg(feature = "std")]
use sp_version::NativeVersion;

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

pub mod migrations;
mod precompiles;
pub mod weights;
pub mod xcm_config;

use {
    crate::precompiles::TemplatePrecompiles,
    cumulus_primitives_core::AggregateMessageOrigin,
    dp_impl_tanssi_pallets_config::impl_tanssi_pallets_config,
    fp_account::EthereumSignature,
    fp_rpc::TransactionStatus,
    frame_support::{
        construct_runtime,
        dispatch::{DispatchClass, GetDispatchInfo},
        dynamic_params::{dynamic_pallet_params, dynamic_params},
        genesis_builder_helper::{build_state, get_preset},
        pallet_prelude::DispatchResult,
        parameter_types,
        traits::{
            fungible::{Balanced, Credit, Inspect},
            tokens::ConversionToAssetBalance,
            ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, Contains, FindAuthor, InsideBoth,
            InstanceFilter, OnFinalize, OnUnbalanced,
        },
        weights::{
            constants::{
                BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight,
                WEIGHT_REF_TIME_PER_SECOND,
            },
            ConstantMultiplier, Weight, WeightToFee as _, WeightToFeeCoefficient,
            WeightToFeeCoefficients, WeightToFeePolynomial,
        },
    },
    frame_system::{
        limits::{BlockLength, BlockWeights},
        EnsureRoot,
    },
    nimbus_primitives::{NimbusId, SlotBeacon},
    pallet_ethereum::{Call::transact, PostLogContent, Transaction as EthereumTransaction},
    pallet_evm::{
        Account as EVMAccount, EVMFungibleAdapter, EnsureAddressNever, EnsureAddressRoot,
        EnsureCreateOrigin, FeeCalculator, FrameSystemAccountProvider, GasWeightMapping,
        IdentityAddressMapping, OnChargeEVMTransaction as OnChargeEVMTransactionT, Runner,
    },
    pallet_transaction_payment::FungibleAdapter,
    parity_scale_codec::{Decode, DecodeWithMemTracking, Encode},
    polkadot_runtime_common::SlowAdjustingFeeUpdate,
    scale_info::TypeInfo,
    smallvec::smallvec,
    sp_api::impl_runtime_apis,
    sp_consensus_slots::{Slot, SlotDuration},
    sp_core::{Get, MaxEncodedLen, OpaqueMetadata, H160, H256, U256},
    sp_runtime::{
        generic, impl_opaque_keys,
        traits::{
            BlakeTwo256, Block as BlockT, DispatchInfoOf, Dispatchable, IdentifyAccount,
            IdentityLookup, PostDispatchInfoOf, UniqueSaturatedInto, Verify,
        },
        transaction_validity::{
            InvalidTransaction, TransactionSource, TransactionValidity, TransactionValidityError,
        },
        ApplyExtrinsicResult, BoundedVec, Cow,
    },
    sp_std::prelude::*,
    sp_version::RuntimeVersion,
    xcm::Version as XcmVersion,
    xcm::{IntoVersion, VersionedAssetId, VersionedAssets, VersionedLocation, VersionedXcm},
    xcm_runtime_apis::{
        dry_run::{CallDryRunEffects, Error as XcmDryRunApiError, XcmDryRunEffects},
        fees::Error as XcmPaymentApiError,
    },
};
pub use {
    sp_consensus_aura::sr25519::AuthorityId as AuraId,
    sp_runtime::{MultiAddress, Perbill, Permill},
};

// Polkadot imports
use polkadot_runtime_common::BlockHashCount;

pub type Precompiles = TemplatePrecompiles<Runtime>;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = EthereumSignature;

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
pub type Address = AccountId;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

pub type TxExtension = cumulus_pallet_weight_reclaim::StorageWeightReclaim<
    Runtime,
    (
        frame_system::CheckNonZeroSender<Runtime>,
        frame_system::CheckSpecVersion<Runtime>,
        frame_system::CheckTxVersion<Runtime>,
        frame_system::CheckGenesis<Runtime>,
        frame_system::CheckEra<Runtime>,
        frame_system::CheckNonce<Runtime>,
        frame_system::CheckWeight<Runtime>,
        pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
    ),
>;
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    fp_self_contained::UncheckedExtrinsic<Address, RuntimeCall, Signature, TxExtension>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic =
    fp_self_contained::CheckedExtrinsic<AccountId, RuntimeCall, TxExtension, H160>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, TxExtension>;

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

    pub const MICROUNIT: Balance = 1_000_000_000_000;
    pub const MILLIUNIT: Balance = 1_000_000_000_000_000;
    pub const UNIT: Balance = 1_000_000_000_000_000_000;
    pub const KILOUNIT: Balance = 1_000_000_000_000_000_000_000;

    pub const STORAGE_BYTE_FEE: Balance = 100 * MICROUNIT;

    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * 100 * MILLIUNIT + (bytes as Balance) * STORAGE_BYTE_FEE
    }
}

impl fp_self_contained::SelfContainedCall for RuntimeCall {
    type SignedInfo = H160;

    fn is_self_contained(&self) -> bool {
        match self {
            RuntimeCall::Ethereum(call) => call.is_self_contained(),
            _ => false,
        }
    }

    fn check_self_contained(&self) -> Option<Result<Self::SignedInfo, TransactionValidityError>> {
        match self {
            RuntimeCall::Ethereum(call) => call.check_self_contained(),
            _ => None,
        }
    }

    fn validate_self_contained(
        &self,
        info: &Self::SignedInfo,
        dispatch_info: &DispatchInfoOf<RuntimeCall>,
        len: usize,
    ) -> Option<TransactionValidity> {
        match self {
            RuntimeCall::Ethereum(call) => call.validate_self_contained(info, dispatch_info, len),
            _ => None,
        }
    }

    fn pre_dispatch_self_contained(
        &self,
        info: &Self::SignedInfo,
        dispatch_info: &DispatchInfoOf<RuntimeCall>,
        len: usize,
    ) -> Option<Result<(), TransactionValidityError>> {
        match self {
            RuntimeCall::Ethereum(call) => {
                call.pre_dispatch_self_contained(info, dispatch_info, len)
            }
            _ => None,
        }
    }

    fn apply_self_contained(
        self,
        info: Self::SignedInfo,
    ) -> Option<sp_runtime::DispatchResultWithInfo<PostDispatchInfoOf<Self>>> {
        match self {
            call @ RuntimeCall::Ethereum(pallet_ethereum::Call::transact { .. }) => {
                Some(call.dispatch(RuntimeOrigin::from(
                    pallet_ethereum::RawOrigin::EthereumTransaction(info),
                )))
            }
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct TransactionConverter;

impl fp_rpc::ConvertTransaction<UncheckedExtrinsic> for TransactionConverter {
    fn convert_transaction(&self, transaction: pallet_ethereum::Transaction) -> UncheckedExtrinsic {
        UncheckedExtrinsic::new_bare(
            pallet_ethereum::Call::<Runtime>::transact { transaction }.into(),
        )
    }
}

impl fp_rpc::ConvertTransaction<opaque::UncheckedExtrinsic> for TransactionConverter {
    fn convert_transaction(
        &self,
        transaction: pallet_ethereum::Transaction,
    ) -> opaque::UncheckedExtrinsic {
        let extrinsic = UncheckedExtrinsic::new_bare(
            pallet_ethereum::Call::<Runtime>::transact { transaction }.into(),
        );
        let encoded = extrinsic.encode();
        opaque::UncheckedExtrinsic::decode(&mut &encoded[..])
            .expect("Encoded extrinsic is always valid")
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
        // for benchmarks, we simply put a value to get a coefficeint of 1
        #[cfg(not(feature = "runtime-benchmarks"))]
        let p = currency::MILLIUNIT / 10;
        #[cfg(feature = "runtime-benchmarks")]
        let p = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());

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

mod impl_on_charge_evm_transaction;

impl_opaque_keys! {
    pub struct SessionKeys { }
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: Cow::Borrowed("frontier-template"),
    impl_name: Cow::Borrowed("frontier-template"),
    authoring_version: 1,
    spec_version: 1500,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
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

pub const EXISTENTIAL_DEPOSIT: Balance = 0;

/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

/// We allow for 2 seconds of compute with a 6 second average block time
const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(
    WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2),
    cumulus_primitives_core::relay_chain::MAX_POV_SIZE as u64,
);

/// We allow for 2 seconds of compute with a 6 second average block time
pub const WEIGHT_MILLISECS_PER_BLOCK: u64 = 2000;

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
    type Lookup = IdentityLookup<AccountId>;
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
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = MultiBlockMigrations;
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
    type ExtensionsWeightInfo = weights::frame_system_extensions::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const TransactionByteFee: Balance = 1;
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    // This will burn the fees
    type OnChargeTransaction = FungibleAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = WeightToFee;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
    type WeightInfo = weights::pallet_transaction_payment::SubstrateWeight<Runtime>;
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
    type DoneSlashHandler = ();
    type WeightInfo = weights::pallet_balances::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
    pub ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
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
    type SelfParaId = parachain_info::Pallet<Runtime>;
    type OutboundXcmpMessageSource = XcmpQueue;
    type DmpQueue = frame_support::traits::EnqueueWithOrigin<MessageQueue, RelayOrigin>;
    type ReservedDmpWeight = ReservedDmpWeight;
    type XcmpMessageHandler = XcmpQueue;
    type ReservedXcmpWeight = ReservedXcmpWeight;
    type CheckAssociatedRelayNumber = RelayNumberMonotonicallyIncreases;
    type ConsensusHook = ConsensusHook;
    type SelectCore = cumulus_pallet_parachain_system::DefaultCoreSelector<Runtime>;
}

pub struct ParaSlotProvider;
impl Get<(Slot, SlotDuration)> for ParaSlotProvider {
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
#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Encode,
    Decode,
    DecodeWithMemTracking,
    Debug,
    MaxEncodedLen,
    TypeInfo,
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

// Be careful: Each time this filter is modified, the substrate filter must also be modified
// consistently.
impl pallet_evm_precompile_proxy::EvmProxyCallFilter for ProxyType {
    fn is_evm_proxy_call_allowed(
        &self,
        call: &pallet_evm_precompile_proxy::EvmSubCall,
        recipient_has_code: bool,
        gas: u64,
    ) -> precompile_utils::EvmResult<bool> {
        Ok(match self {
            ProxyType::Any => true,
            ProxyType::NonTransfer => false,
            ProxyType::Governance => false,
            // The proxy precompile does not contain method cancel_proxy
            ProxyType::CancelProxy => false,
            ProxyType::Balances => {
                // Allow only "simple" accounts as recipient (no code nor precompile).
                // Note: Checking the presence of the code is not enough because some precompiles
                // have no code.
                !recipient_has_code
                    && !precompile_utils::precompile_set::is_precompile_or_fail::<Runtime>(
                        call.to.0, gas,
                    )?
            }
        })
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
    type ProxyDepositBase = ConstU128<{ currency::deposit(1, 8) }>;
    // Additional storage item size of 21 bytes (20 bytes AccountId + 1 byte sizeof(ProxyType)).
    type ProxyDepositFactor = ConstU128<{ currency::deposit(0, 21) }>;
    type MaxProxies = ConstU32<32>;
    type MaxPending = ConstU32<32>;
    type CallHasher = BlakeTwo256;
    type AnnouncementDepositBase = ConstU128<{ currency::deposit(1, 8) }>;
    // Additional storage item size of 56 bytes:
    // - 20 bytes AccountId
    // - 32 bytes Hasher (Blake2256)
    // - 4 bytes BlockNumber (u32)
    type AnnouncementDepositFactor = ConstU128<{ currency::deposit(0, 56) }>;
    type WeightInfo = weights::pallet_proxy::SubstrateWeight<Runtime>;
    type BlockNumberProvider = System;
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

impl cumulus_pallet_weight_reclaim::Config for Runtime {
    type WeightInfo = weights::cumulus_pallet_weight_reclaim::SubstrateWeight<Runtime>;
}

impl pallet_migrations::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MigrationsList = (migrations::TemplateMigrations<Runtime, XcmpQueue, PolkadotXcm>,);
    type XcmExecutionManager = XcmExecutionManager;
}

parameter_types! {
    pub MbmServiceWeight: Weight = Perbill::from_percent(80) * RuntimeBlockWeights::get().max_block;
}

impl pallet_multiblock_migrations::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type Migrations = ();
    // Benchmarks need mocked migrations to guarantee that they succeed.
    #[cfg(feature = "runtime-benchmarks")]
    type Migrations = pallet_multiblock_migrations::mock_helpers::MockedMigrations;
    type CursorMaxLen = ConstU32<65_536>;
    type IdentifierMaxLen = ConstU32<256>;
    type MigrationStatusHandler = ();
    type FailedMigrationHandler = MaintenanceMode;
    type MaxServiceWeight = MbmServiceWeight;
    type WeightInfo = weights::pallet_multiblock_migrations::SubstrateWeight<Runtime>;
}

/// Maintenance mode Call filter
pub struct MaintenanceFilter;
impl Contains<RuntimeCall> for MaintenanceFilter {
    fn contains(c: &RuntimeCall) -> bool {
        !matches!(
            c,
            RuntimeCall::Balances(_)
                | RuntimeCall::Ethereum(_)
                | RuntimeCall::EVM(_)
                | RuntimeCall::PolkadotXcm(_)
        )
    }
}

/// Normal Call Filter
/// We dont allow to create nor mint assets, this for now is disabled
/// We only allow transfers. For now creation of assets will go through
/// asset-manager, while minting/burning only happens through xcm messages
/// This can change in the future
pub struct NormalFilter;
impl Contains<RuntimeCall> for NormalFilter {
    fn contains(c: &RuntimeCall) -> bool {
        !matches!(
            c,
            // Filtering the EVM prevents possible re-entrancy from the precompiles which could
            // lead to unexpected scenarios.
            // See https://github.com/PureStake/sr-moonbeam/issues/30
            // Note: It is also assumed that EVM calls are only allowed through `Origin::Root` so
            // this can be seen as an additional security
            RuntimeCall::EVM(_)
        )
    }
}

impl pallet_maintenance_mode::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type NormalCallFilter = NormalFilter;
    type MaintenanceCallFilter = InsideBoth<MaintenanceFilter, NormalFilter>;
    type MaintenanceOrigin = EnsureRoot<AccountId>;
    type XcmExecutionManager = XcmExecutionManager;
}

#[dynamic_params(RuntimeParameters, pallet_parameters::Parameters::<Runtime>)]
pub mod dynamic_params {
    use super::*;

    #[dynamic_pallet_params]
    #[codec(index = 3)]
    pub mod contract_deploy_filter {
        #[codec(index = 0)]
        pub static AllowedAddressesToCreate: DeployFilter = DeployFilter::All;
        #[codec(index = 1)]
        pub static AllowedAddressesToCreateInner: DeployFilter = DeployFilter::All;
    }
}

impl pallet_parameters::Config for Runtime {
    type AdminOrigin = EnsureRoot<AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeParameters = RuntimeParameters;
    type WeightInfo = weights::pallet_parameters::SubstrateWeight<Runtime>;
}

#[cfg(feature = "runtime-benchmarks")]
impl Default for RuntimeParameters {
    fn default() -> Self {
        RuntimeParameters::ContractDeployFilter(
            dynamic_params::contract_deploy_filter::Parameters::AllowedAddressesToCreate(
                dynamic_params::contract_deploy_filter::AllowedAddressesToCreate,
                Some(DeployFilter::All),
            ),
        )
    }
}

#[derive(
    Clone, PartialEq, Encode, Decode, DecodeWithMemTracking, TypeInfo, Eq, MaxEncodedLen, Debug,
)]
pub enum DeployFilter {
    All,
    Whitelisted(BoundedVec<H160, ConstU32<100>>),
}

pub struct AddressFilter<Runtime, AddressList>(sp_std::marker::PhantomData<(Runtime, AddressList)>);
impl<Runtime, AddressList> EnsureCreateOrigin<Runtime> for AddressFilter<Runtime, AddressList>
where
    Runtime: pallet_evm::Config,
    AddressList: Get<DeployFilter>,
{
    fn check_create_origin(address: &H160) -> Result<(), pallet_evm::Error<Runtime>> {
        let deploy_filter: DeployFilter = AddressList::get();

        match deploy_filter {
            DeployFilter::All => Ok(()),
            DeployFilter::Whitelisted(addresses_vec) => {
                if !addresses_vec.contains(address) {
                    Err(pallet_evm::Error::<Runtime>::CreateOriginNotAllowed)
                } else {
                    Ok(())
                }
            }
        }
    }
}

impl pallet_evm_chain_id::Config for Runtime {}

pub struct FindAuthorAdapter;
impl FindAuthor<H160> for FindAuthorAdapter {
    fn find_author<'a, I>(digests: I) -> Option<H160>
    where
        I: 'a + IntoIterator<Item = (sp_runtime::ConsensusEngineId, &'a [u8])>,
    {
        if let Some(author) = AuthorInherent::find_author(digests) {
            return Some(H160::from_slice(&author.encode()[0..20]));
        }
        None
    }
}

/// Current approximation of the gas/s consumption considering
/// EVM execution over compiled WASM (on 4.4Ghz CPU).
/// Given the 1000ms Weight, from which 75% only are used for transactions,
/// the total EVM execution gas limit is: GAS_PER_SECOND * 1 * 0.75 ~= 30_000_000.
pub const GAS_PER_SECOND: u64 = 40_000_000;

/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_REF_TIME_PER_SECOND / GAS_PER_SECOND;

parameter_types! {
    pub BlockGasLimit: U256
        = U256::from(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT.ref_time() / WEIGHT_PER_GAS);
    pub PrecompilesValue: TemplatePrecompiles<Runtime> = TemplatePrecompiles::<_>::new();
    pub WeightPerGas: Weight = Weight::from_parts(WEIGHT_PER_GAS, 0);
    pub SuicideQuickClearLimit: u32 = 0;
    pub GasLimitPovSizeRatio: u32 = 16;
    /// Hardcoding the value, since it is computed on block execution. Check calculations in the tests
    pub GasLimitStorageGrowthRatio: u64 = 1464;
}

impl_on_charge_evm_transaction!();
impl pallet_evm::Config for Runtime {
    type AccountProvider = FrameSystemAccountProvider<Runtime>;
    type FeeCalculator = BaseFee;
    type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
    type WeightPerGas = WeightPerGas;
    type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
    type CallOrigin = EnsureAddressRoot<AccountId>;
    type WithdrawOrigin = EnsureAddressNever<AccountId>;
    type AddressMapping = IdentityAddressMapping;
    type CreateOriginFilter =
        AddressFilter<Runtime, dynamic_params::contract_deploy_filter::AllowedAddressesToCreate>;
    type CreateInnerOriginFilter = AddressFilter<
        Runtime,
        dynamic_params::contract_deploy_filter::AllowedAddressesToCreateInner,
    >;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type PrecompilesType = TemplatePrecompiles<Self>;
    type PrecompilesValue = PrecompilesValue;
    type ChainId = EVMChainId;
    type BlockGasLimit = BlockGasLimit;
    type Runner = pallet_evm::runner::stack::Runner<Self>;
    type OnChargeTransaction = OnChargeEVMTransaction<()>;
    type OnCreate = ();
    type FindAuthor = FindAuthorAdapter;
    type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
    type GasLimitStorageGrowthRatio = GasLimitStorageGrowthRatio;
    type Timestamp = Timestamp;
    type WeightInfo = ();
}

parameter_types! {
    pub const PostBlockAndTxnHashes: PostLogContent = PostLogContent::BlockAndTxnHashes;
}

impl pallet_ethereum::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type StateRoot = pallet_ethereum::IntermediateStateRoot<Self::Version>;
    type PostLogContent = PostBlockAndTxnHashes;
    type ExtraDataLength = ConstU32<30>;
}

parameter_types! {
    pub BoundDivision: U256 = U256::from(1024);
}

parameter_types! {
    pub DefaultBaseFeePerGas: U256 = U256::from(2_000_000_000);
    pub DefaultElasticity: Permill = Permill::from_parts(125_000);
}

pub struct BaseFeeThreshold;
impl pallet_base_fee::BaseFeeThreshold for BaseFeeThreshold {
    fn lower() -> Permill {
        Permill::zero()
    }
    fn ideal() -> Permill {
        Permill::from_parts(500_000)
    }
    fn upper() -> Permill {
        Permill::from_parts(1_000_000)
    }
}

impl pallet_base_fee::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Threshold = BaseFeeThreshold;
    type DefaultBaseFeePerGas = DefaultBaseFeePerGas;
    type DefaultElasticity = DefaultElasticity;
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
    // One storage item; key size 32 + 20; value is size 4+4+16+20. Total = 1 * (52 + 44)
    pub const DepositBase: Balance = currency::deposit(1, 96);
    // Additional storage item size of 20 bytes.
    pub const DepositFactor: Balance = currency::deposit(0, 20);
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
    type BlockNumberProvider = System;
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
        MultiBlockMigrations: pallet_multiblock_migrations = 121,
        MaintenanceMode: pallet_maintenance_mode = 8,
        TxPause: pallet_tx_pause = 9,

        // Monetary stuff.
        Balances: pallet_balances = 10,

        // Other utilities
        Multisig: pallet_multisig = 16,
        Parameters: pallet_parameters = 17,

        // ContainerChain
        AuthoritiesNoting: pallet_cc_authorities_noting = 50,
        AuthorInherent: pallet_author_inherent = 51,

        // Frontier
        Ethereum: pallet_ethereum = 60,
        EVM: pallet_evm = 61,
        EVMChainId: pallet_evm_chain_id = 62,
        BaseFee: pallet_base_fee = 64,
        TransactionPayment: pallet_transaction_payment = 66,

        // XCM
        XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Storage, Event<T>} = 70,
        CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin} = 71,
        PolkadotXcm: pallet_xcm::{Pallet, Call, Storage, Event<T>, Origin, Config<T>} = 73,
        MessageQueue: pallet_message_queue::{Pallet, Call, Storage, Event<T>} = 74,
        ForeignAssets: pallet_assets::<Instance1>::{Pallet, Call, Storage, Event<T>} = 75,
        ForeignAssetsCreator: pallet_foreign_asset_creator::{Pallet, Call, Storage, Event<T>} = 76,
        AssetRate: pallet_asset_rate::{Pallet, Call, Storage, Event<T>} = 77,
        XcmExecutorUtils: pallet_xcm_executor_utils::{Pallet, Call, Storage, Event<T>} = 78,

        WeightReclaim: cumulus_pallet_weight_reclaim = 80,

        RootTesting: pallet_root_testing = 100,
        AsyncBacking: pallet_async_backing::{Pallet, Storage} = 110,
    }
);

#[cfg(feature = "runtime-benchmarks")]
mod benches {
    frame_benchmarking::define_benchmarks!(
        [frame_system, frame_system_benchmarking::Pallet::<Runtime>]
        [frame_system_extensions, frame_system_benchmarking::extensions::Pallet::<Runtime>]
        [cumulus_pallet_parachain_system, ParachainSystem]
        [pallet_timestamp, Timestamp]
        [pallet_sudo, Sudo]
        [pallet_utility, Utility]
        [pallet_proxy, Proxy]
        [pallet_transaction_payment, TransactionPayment]
        [pallet_tx_pause, TxPause]
        [pallet_balances, Balances]
        [pallet_multiblock_migrations, MultiBlockMigrations]
        [pallet_multisig, Multisig]
        [pallet_parameters, Parameters]
        [pallet_cc_authorities_noting, AuthoritiesNoting]
        [pallet_author_inherent, AuthorInherent]
        [cumulus_pallet_xcmp_queue, XcmpQueue]
        [pallet_xcm, PalletXcmExtrinsicsBenchmark::<Runtime>]
        [pallet_xcm_benchmarks::generic, pallet_xcm_benchmarks::generic::Pallet::<Runtime>]
        [pallet_message_queue, MessageQueue]
        [pallet_assets, ForeignAssets]
        [pallet_foreign_asset_creator, ForeignAssetsCreator]
        [pallet_asset_rate, AssetRate]
        [pallet_xcm_executor_utils, XcmExecutorUtils]
        [cumulus_pallet_weight_reclaim, WeightReclaim]
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

        fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
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
            xt: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            // Filtered calls should not enter the tx pool as they'll fail if inserted.
            // If this call is not allowed, we return early.
            if !<Runtime as frame_system::Config>::BaseCallFilter::contains(&xt.0.function) {
                return InvalidTransaction::Call.into();
            }

            // This runtime uses Substrate's pallet transaction payment. This
            // makes the chain feel like a standard Substrate chain when submitting
            // frame transactions and using Substrate ecosystem tools. It has the downside that
            // transaction are not prioritized by gas_price. The following code reprioritizes
            // transactions to overcome this.
            //
            // A more elegant, ethereum-first solution is
            // a pallet that replaces pallet transaction payment, and allows users
            // to directly specify a gas price rather than computing an effective one.
            // #HopefullySomeday

            // First we pass the transactions to the standard FRAME executive. This calculates all the
            // necessary tags, longevity and other properties that we will leave unchanged.
            // This also assigns some priority that we don't care about and will overwrite next.
            let mut intermediate_valid = Executive::validate_transaction(source, xt.clone(), block_hash)?;

            let dispatch_info = xt.get_dispatch_info();

            // If this is a pallet ethereum transaction, then its priority is already set
            // according to effective priority fee from pallet ethereum. If it is any other kind of
            // transaction, we modify its priority. The goal is to arrive at a similar metric used
            // by pallet ethereum, which means we derive a fee-per-gas from the txn's tip and
            // weight.
            Ok(match &xt.0.function {
                RuntimeCall::Ethereum(transact { .. }) => intermediate_valid,
                _ if dispatch_info.class != DispatchClass::Normal => intermediate_valid,
                _ => {
                    let tip = match xt.0.preamble.to_signed() {
                        None => 0,
                        Some((_, _, ref signed_extra)) => {
                            // Yuck, this depends on the index of charge transaction in Signed Extra
                            let charge_transaction = &signed_extra.0.7;
                            charge_transaction.tip()
                        }
                    };

                    let effective_gas =
                        <Runtime as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
                            dispatch_info.total_weight()
                        );
                    let tip_per_gas = tip.checked_div(u128::from(effective_gas)).unwrap_or(0);

                    // Overwrite the original prioritization with this ethereum one
                    intermediate_valid.priority = tip_per_gas as u64;
                    intermediate_valid
                }
            })
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
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            build_state::<RuntimeGenesisConfig>(config)
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            get_preset::<RuntimeGenesisConfig>(id, |_| None)
        }
        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
            vec![]
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
            use frame_benchmarking::{BenchmarkList};
            use frame_support::traits::StorageInfoTrait;
            use pallet_xcm::benchmarking::Pallet as PalletXcmExtrinsicsBenchmark;

            let mut list = Vec::<BenchmarkList>::new();
            list_benchmarks!(list, extra);

            let storage_info = AllPalletsWithSystem::storage_info();
            (list, storage_info)
        }

        #[allow(non_local_definitions)]
        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig,
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, alloc::string::String> {
            use frame_benchmarking::{BenchmarkBatch, BenchmarkError};
            use sp_core::storage::TrackedStorageKey;
            use xcm::latest::prelude::*;
            impl frame_system_benchmarking::Config for Runtime {
                fn setup_set_code_requirements(code: &sp_std::vec::Vec<u8>) -> Result<(), BenchmarkError> {
                    ParachainSystem::initialize_for_set_code_benchmark(code.len() as u32);
                    Ok(())
                }

                fn verify_set_code() {
                    System::assert_last_event(cumulus_pallet_parachain_system::Event::<Runtime>::ValidationFunctionStored.into());
                }
            }
            use xcm_config::SelfReserve;

            parameter_types! {
                pub ExistentialDepositAsset: Option<Asset> = Some((
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
                fn valid_destination() -> Result<Location, BenchmarkError> {
                    Ok(Location::parent())
                }
                fn worst_case_holding(_depositable_count: u32) -> Assets {
                    // We only care for native asset until we support others
                    // TODO: refactor this case once other assets are supported
                    vec![Asset{
                        id: AssetId(SelfReserve::get()),
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

                fn worst_case_asset_exchange() -> Result<(Assets, Assets), BenchmarkError> {
                    Err(BenchmarkError::Skip)
                }

                fn universal_alias() -> Result<(Location, Junction), BenchmarkError> {
                    Err(BenchmarkError::Skip)
                }

                fn transact_origin_and_runtime_call() -> Result<(Location, RuntimeCall), BenchmarkError> {
                    Ok((Location::parent(), frame_system::Call::remark_with_event { remark: vec![] }.into()))
                }

                fn subscribe_origin() -> Result<Location, BenchmarkError> {
                    Ok(Location::parent())
                }

                fn fee_asset() -> Result<Asset, BenchmarkError> {
                    Ok(Asset {
                        id: AssetId(SelfReserve::get()),
                        fun: Fungible(crate::currency::MICROUNIT*100),
                    })
                }

                fn claimable_asset() -> Result<(Location, Location, Assets), BenchmarkError> {
                    let origin = Location::parent();
                    let assets: Assets = (Location::parent(), 1_000u128).into();
                    let ticket = Location { parents: 0, interior: Here };
                    Ok((origin, ticket, assets))
                }

                fn unlockable_asset() -> Result<(Location, Location, Asset), BenchmarkError> {
                    Err(BenchmarkError::Skip)
                }

                fn export_message_origin_and_destination(
                ) -> Result<(Location, NetworkId, InteriorLocation), BenchmarkError> {
                    Err(BenchmarkError::Skip)
                }

                fn alias_origin() -> Result<(Location, Location), BenchmarkError> {
                    Err(BenchmarkError::Skip)
                }
            }

            use pallet_xcm::benchmarking::Pallet as PalletXcmExtrinsicsBenchmark;
            impl pallet_xcm::benchmarking::Config for Runtime {
                type DeliveryHelper = ();
                fn get_asset() -> Asset {
                    Asset {
                        id: AssetId(SelfReserve::get()),
                        fun: Fungible(crate::currency::MICROUNIT),
                    }
                }

                fn reachable_dest() -> Option<Location> {
                    Some(Parent.into())
                }

                fn teleportable_asset_and_dest() -> Option<(Asset, Location)> {
                    let teleportable = crate::currency::MICROUNIT;
                    // Relay/native token can be teleported between AH and Relay.
                    Some((
                        Asset {
                            fun: Fungible(teleportable),
                            id: Parent.into()
                        },
                        Parent.into(),
                    ))
                }

                fn reserve_transferable_asset_and_dest() -> Option<(Asset, Location)> {
                    use xcm_config::SelfReserve;
                    // AH can reserve transfer native token to some random parachain.
                    let random_para_id = 43211234;

                    ParachainSystem::open_outbound_hrmp_channel_for_benchmarks_or_tests(
                        random_para_id.into()
                    );
                    let who = frame_benchmarking::whitelisted_caller();

                    // Give some multiple of the existential deposit
                    let balance = crate::currency::MICROUNIT* 1000;
                    let _ = <Balances as frame_support::traits::Currency<_>>::make_free_balance_be(
                        &who, balance,
                    );
                    Some((
                        Asset {
                            fun: Fungible(balance),
                            id: SelfReserve::get().into()
                        },
                        ParentThen(Parachain(random_para_id).into()).into(),
                    ))
                }

                fn set_up_complex_asset_transfer(
                ) -> Option<(Assets, u32, Location, Box<dyn FnOnce()>)> {
                    use xcm_config::SelfReserve;
                    // Transfer to Relay some local AH asset (local-reserve-transfer) while paying
                    // fees using teleported native token.
                    // (We don't care that Relay doesn't accept incoming unknown AH local asset)
                    let dest = Parent.into();

                    let fee_amount = crate::currency::MICROUNIT;
                    let fee_asset: Asset = (SelfReserve::get(), fee_amount).into();

                    let who = frame_benchmarking::whitelisted_caller();
                    // Give some multiple of the existential deposit
                    let balance = fee_amount + crate::currency::MICROUNIT * 1000;
                    let _ = <Balances as frame_support::traits::Currency<_>>::make_free_balance_be(
                        &who, balance,
                    );

                    // verify initial balance
                    assert_eq!(Balances::free_balance(who), balance);

                    // set up local asset
                    let asset_amount = 10u128;
                    let initial_asset_amount = asset_amount * 10;

                    let (asset_id, asset_location) = pallet_foreign_asset_creator::benchmarks::create_default_minted_asset::<Runtime>(
                        initial_asset_amount,
                        who
                    );

                    let transfer_asset: Asset = (asset_location, asset_amount).into();

                    let assets: Assets = vec![fee_asset.clone(), transfer_asset].into();
                    let fee_index = if assets.get(0).unwrap().eq(&fee_asset) { 0 } else { 1 };

                    // verify transferred successfully
                    let verify = Box::new(move || {
                        // verify native balance after transfer, decreased by transferred fee amount
                        // (plus transport fees)
                        assert!(Balances::free_balance(who) <= balance - fee_amount);
                        // verify asset balance decreased by exactly transferred amount
                        assert_eq!(
                            ForeignAssets::balance(asset_id, who),
                            initial_asset_amount - asset_amount,
                        );
                    });
                    Some((assets, fee_index, dest, verify))
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

    impl fp_rpc::EthereumRuntimeRPCApi<Block> for Runtime {
        fn chain_id() -> u64 {
            <Runtime as pallet_evm::Config>::ChainId::get()
        }

        fn account_basic(address: H160) -> EVMAccount {
            let (account, _) = pallet_evm::Pallet::<Runtime>::account_basic(&address);
            account
        }

        fn gas_price() -> U256 {
            let (gas_price, _) = <Runtime as pallet_evm::Config>::FeeCalculator::min_gas_price();
            gas_price
        }

        fn account_code_at(address: H160) -> Vec<u8> {
            pallet_evm::AccountCodes::<Runtime>::get(address)
        }

        fn author() -> H160 {
            <pallet_evm::Pallet<Runtime>>::find_author()
        }

        fn storage_at(address: H160, index: U256) -> H256 {
            let tmp = index.to_big_endian();
            pallet_evm::AccountStorages::<Runtime>::get(address, H256::from_slice(&tmp[..]))
        }

        fn call(
            from: H160,
            to: H160,
            data: Vec<u8>,
            value: U256,
            gas_limit: U256,
            max_fee_per_gas: Option<U256>,
            max_priority_fee_per_gas: Option<U256>,
            nonce: Option<U256>,
            estimate: bool,
            access_list: Option<Vec<(H160, Vec<H256>)>>,
        ) -> Result<pallet_evm::CallInfo, sp_runtime::DispatchError> {
            let config = if estimate {
                let mut config = <Runtime as pallet_evm::Config>::config().clone();
                config.estimate = true;
                Some(config)
            } else {
                None
            };
            let is_transactional = false;
            let validate = true;

            let transaction_data = pallet_ethereum::TransactionData::new(
                pallet_ethereum::TransactionAction::Call(to),
                                data.clone(),
                                nonce.unwrap_or_default(),
                                gas_limit,
                                None,
                                max_fee_per_gas.or(Some(U256::default())),
                                max_priority_fee_per_gas.or(Some(U256::default())),
                                value,
                                Some(<Runtime as pallet_evm::Config>::ChainId::get()),
                                access_list.clone().unwrap_or_default(),
                            );

            let gas_limit = gas_limit.min(u64::MAX.into()).low_u64();

            let (weight_limit, proof_size_base_cost) = pallet_ethereum::Pallet::<Runtime>::transaction_weight(&transaction_data);

            <Runtime as pallet_evm::Config>::Runner::call(
                from,
                to,
                data,
                value,
                gas_limit,
                max_fee_per_gas,
                max_priority_fee_per_gas,
                nonce,
                access_list.unwrap_or_default(),
                is_transactional,
                validate,
                weight_limit,
                proof_size_base_cost,
                config.as_ref().unwrap_or(<Runtime as pallet_evm::Config>::config()),
            ).map_err(|err| err.error.into())
        }

        fn create(
            from: H160,
            data: Vec<u8>,
            value: U256,
            gas_limit: U256,
            max_fee_per_gas: Option<U256>,
            max_priority_fee_per_gas: Option<U256>,
            nonce: Option<U256>,
            estimate: bool,
            access_list: Option<Vec<(H160, Vec<H256>)>>,
        ) -> Result<pallet_evm::CreateInfo, sp_runtime::DispatchError> {
            let config = if estimate {
                let mut config = <Runtime as pallet_evm::Config>::config().clone();
                config.estimate = true;
                Some(config)
            } else {
                None
            };
            let is_transactional = false;
            let validate = true;

            let transaction_data = pallet_ethereum::TransactionData::new(
                pallet_ethereum::TransactionAction::Create,
                data.clone(),
                nonce.unwrap_or_default(),
                gas_limit,
                None,
                max_fee_per_gas.or(Some(U256::default())),
                max_priority_fee_per_gas.or(Some(U256::default())),
                value,
                Some(<Runtime as pallet_evm::Config>::ChainId::get()),
                access_list.clone().unwrap_or_default(),
            );

            let gas_limit = gas_limit.min(u64::MAX.into()).low_u64();

            let (weight_limit, proof_size_base_cost) = pallet_ethereum::Pallet::<Runtime>::transaction_weight(&transaction_data);


            <Runtime as pallet_evm::Config>::Runner::create(
                from,
                data,
                value,
                gas_limit,
                max_fee_per_gas,
                max_priority_fee_per_gas,
                nonce,
                access_list.unwrap_or_default(),
                is_transactional,
                validate,
                weight_limit,
                proof_size_base_cost,
                config.as_ref().unwrap_or(<Runtime as pallet_evm::Config>::config()),
            ).map_err(|err| err.error.into())
        }

        fn current_transaction_statuses() -> Option<Vec<TransactionStatus>> {
            pallet_ethereum::CurrentTransactionStatuses::<Runtime>::get()
        }

        fn current_block() -> Option<pallet_ethereum::Block> {
            pallet_ethereum::CurrentBlock::<Runtime>::get()
        }

        fn current_receipts() -> Option<Vec<pallet_ethereum::Receipt>> {
            pallet_ethereum::CurrentReceipts::<Runtime>::get()
        }

        fn current_all() -> (
            Option<pallet_ethereum::Block>,
            Option<Vec<pallet_ethereum::Receipt>>,
            Option<Vec<TransactionStatus>>,
        ) {
            (
                pallet_ethereum::CurrentBlock::<Runtime>::get(),
                pallet_ethereum::CurrentReceipts::<Runtime>::get(),
                pallet_ethereum::CurrentTransactionStatuses::<Runtime>::get()
            )
        }

        fn extrinsic_filter(
            xts: Vec<<Block as BlockT>::Extrinsic>,
        ) -> Vec<EthereumTransaction> {
            xts.into_iter().filter_map(|xt| match xt.0.function {
                RuntimeCall::Ethereum(transact { transaction }) => Some(transaction),
                _ => None
            }).collect::<Vec<EthereumTransaction>>()
        }

        fn elasticity() -> Option<Permill> {
            Some(pallet_base_fee::Elasticity::<Runtime>::get())
        }

        fn gas_limit_multiplier_support() {}

        fn pending_block(xts: Vec<<Block as BlockT>::Extrinsic>) -> (Option<pallet_ethereum::Block>, Option<sp_std::prelude::Vec<TransactionStatus>>) {
            for ext in xts.into_iter() {
                let _ = Executive::apply_extrinsic(ext);
            }

            Ethereum::on_finalize(System::block_number() + 1);

            (
                pallet_ethereum::CurrentBlock::<Runtime>::get(),
                pallet_ethereum::CurrentTransactionStatuses::<Runtime>::get()
            )
        }

        fn initialize_pending_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header);
        }
    }

    impl fp_rpc::ConvertTransactionRuntimeApi<Block> for Runtime {
        fn convert_transaction(
            transaction: pallet_ethereum::Transaction
        ) -> <Block as BlockT>::Extrinsic {
            UncheckedExtrinsic::new_bare(
                pallet_ethereum::Call::<Runtime>::transact { transaction }.into(),
            )
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

    impl xcm_runtime_apis::fees::XcmPaymentApi<Block> for Runtime {
        fn query_acceptable_payment_assets(xcm_version: xcm::Version) -> Result<Vec<VersionedAssetId>, XcmPaymentApiError> {
            if !matches!(xcm_version, 3..=5) {
                return Err(XcmPaymentApiError::UnhandledXcmVersion);
            }

            Ok([VersionedAssetId::V5(xcm_config::SelfReserve::get().into())]
                .into_iter()
                .chain(
                    pallet_asset_rate::ConversionRateToNative::<Runtime>::iter_keys().filter_map(|asset_id_u16| {
                        pallet_foreign_asset_creator::AssetIdToForeignAsset::<Runtime>::get(asset_id_u16).map(|location| {
                            VersionedAssetId::V5(location.into())
                        }).or_else(|| {
                            log::warn!("Asset `{}` is present in pallet_asset_rate but not in pallet_foreign_asset_creator", asset_id_u16);
                            None
                        })
                    })
                )
                .filter_map(|asset| asset.into_version(xcm_version).map_err(|e| {
                    log::warn!("Failed to convert asset to version {}: {:?}", xcm_version, e);
                }).ok())
                .collect())
        }

        fn query_weight_to_asset_fee(weight: Weight, asset: VersionedAssetId) -> Result<u128, XcmPaymentApiError> {
            let local_asset = VersionedAssetId::V5(xcm_config::SelfReserve::get().into());
            let asset = asset
                .into_version(5)
                .map_err(|_| XcmPaymentApiError::VersionedConversionFailed)?;

            if asset == local_asset {
                Ok(WeightToFee::weight_to_fee(&weight))
            } else {
                let native_fee = WeightToFee::weight_to_fee(&weight);
                let asset_v5: xcm::latest::AssetId = asset.try_into().map_err(|_| XcmPaymentApiError::VersionedConversionFailed)?;
                let location: xcm::latest::Location = asset_v5.0;
                let asset_id = pallet_foreign_asset_creator::ForeignAssetToAssetId::<Runtime>::get(location).ok_or(XcmPaymentApiError::AssetNotFound)?;
                let asset_rate = AssetRate::to_asset_balance(native_fee, asset_id);
                match asset_rate {
                    Ok(x) => Ok(x),
                    Err(pallet_asset_rate::Error::UnknownAssetKind) => Err(XcmPaymentApiError::AssetNotFound),
                    // Error when converting native balance to asset balance, probably overflow
                    Err(_e) => Err(XcmPaymentApiError::WeightNotComputable),
                }
            }
        }

        fn query_xcm_weight(message: VersionedXcm<()>) -> Result<Weight, XcmPaymentApiError> {
            PolkadotXcm::query_xcm_weight(message)
        }

        fn query_delivery_fees(destination: VersionedLocation, message: VersionedXcm<()>) -> Result<VersionedAssets, XcmPaymentApiError> {
            PolkadotXcm::query_delivery_fees(destination, message)
        }
    }

    impl xcm_runtime_apis::dry_run::DryRunApi<Block, RuntimeCall, RuntimeEvent, OriginCaller> for Runtime {
        fn dry_run_call(origin: OriginCaller, call: RuntimeCall, result_xcms_version: XcmVersion) -> Result<CallDryRunEffects<RuntimeEvent>, XcmDryRunApiError> {
            PolkadotXcm::dry_run_call::<Runtime, xcm_config::XcmRouter, OriginCaller, RuntimeCall>(origin, call, result_xcms_version)
        }

        fn dry_run_xcm(origin_location: VersionedLocation, xcm: VersionedXcm<RuntimeCall>) -> Result<XcmDryRunEffects<RuntimeEvent>, XcmDryRunApiError> {
            PolkadotXcm::dry_run_xcm::<Runtime, xcm_config::XcmRouter, RuntimeCall, xcm_config::XcmConfig>(origin_location, xcm)
        }
    }

    impl xcm_runtime_apis::conversions::LocationToAccountApi<Block, AccountId> for Runtime {
        fn convert_location(location: VersionedLocation) -> Result<
            AccountId,
            xcm_runtime_apis::conversions::Error
        > {
            xcm_runtime_apis::conversions::LocationToAccountHelper::<
                AccountId,
                xcm_config::LocationToAccountId,
            >::convert_location(location)
        }
    }
}

#[allow(dead_code)]
struct CheckInherents;

// TODO: this should be removed but currently if we remove it the relay does not check anything
// related to other inherents that are not parachain-system
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Block storage limit in bytes. Set to 40 KB.
    const BLOCK_STORAGE_LIMIT: u64 = 40 * 1024;

    #[test]
    fn check_ratio_constant() {
        assert_eq!(
            BlockGasLimit::get().min(u64::MAX.into()).low_u64() / BLOCK_STORAGE_LIMIT,
            GasLimitStorageGrowthRatio::get()
        );
    }
}
