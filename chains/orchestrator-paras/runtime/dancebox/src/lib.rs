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

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

extern crate alloc;

pub mod xcm_config;

#[cfg(feature = "std")]
use sp_version::NativeVersion;

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::{DispatchError, TransactionOutcome};

pub mod weights;

#[cfg(test)]
mod tests;

use {
    cumulus_pallet_parachain_system::{
        RelayChainStateProof, RelayNumberMonotonicallyIncreases, RelaychainDataProvider,
        RelaychainStateProvider,
    },
    cumulus_primitives_core::{
        relay_chain::{self, SessionIndex},
        AggregateMessageOrigin, BodyId, ParaId,
    },
    frame_support::{
        construct_runtime,
        dispatch::DispatchClass,
        genesis_builder_helper::{build_state, get_preset},
        pallet_prelude::DispatchResult,
        parameter_types,
        traits::{
            fungible::{Balanced, Credit, Inspect},
            tokens::{ConversionToAssetBalance, PayFromAccount, UnityAssetBalanceConversion},
            ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, Contains, EitherOfDiverse,
            InsideBoth, InstanceFilter, OnUnbalanced, ValidatorRegistration,
        },
        weights::{
            constants::{
                BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight,
                WEIGHT_REF_TIME_PER_SECOND,
            },
            ConstantMultiplier, Weight, WeightToFee as _, WeightToFeeCoefficient,
            WeightToFeeCoefficients, WeightToFeePolynomial,
        },
        PalletId,
    },
    frame_system::{
        limits::{BlockLength, BlockWeights},
        EnsureRoot, EnsureSigned,
    },
    nimbus_primitives::{NimbusId, SlotBeacon},
    pallet_collator_assignment::{GetRandomnessForNextBlock, RotateCollatorsEveryNSessions},
    pallet_invulnerables::InvulnerableRewardDistribution,
    pallet_pooled_staking::traits::IsCandidateEligible,
    pallet_registrar::RegistrarHooks,
    pallet_registrar_runtime_api::ContainerChainGenesisData,
    pallet_services_payment::{
        BalanceOf, ProvideBlockProductionCost, ProvideCollatorAssignmentCost,
    },
    pallet_session::{SessionManager, ShouldEndSession},
    pallet_stream_payment_runtime_api::{StreamPaymentApiError, StreamPaymentApiStatus},
    pallet_transaction_payment::FungibleAdapter,
    pallet_xcm_core_buyer::BuyingError,
    parity_scale_codec::DecodeWithMemTracking,
    polkadot_runtime_common::BlockHashCount,
    scale_info::prelude::format,
    smallvec::smallvec,
    sp_api::impl_runtime_apis,
    sp_consensus_aura::SlotDuration,
    sp_consensus_slots::Slot,
    sp_core::{crypto::KeyTypeId, Get, MaxEncodedLen, OpaqueMetadata, H256},
    sp_runtime::{
        generic, impl_opaque_keys,
        traits::{
            AccountIdConversion, AccountIdLookup, BlakeTwo256, Block as BlockT, ConvertInto,
            Hash as HashT, IdentityLookup, Verify,
        },
        transaction_validity::{TransactionSource, TransactionValidity},
        AccountId32, ApplyExtrinsicResult, Cow,
    },
    sp_std::{
        collections::{btree_map::BTreeMap, btree_set::BTreeSet},
        marker::PhantomData,
        prelude::*,
    },
    sp_version::RuntimeVersion,
    tanssi_runtime_common::SessionTimer,
    tp_stream_payment_common::StreamId,
    tp_traits::{
        apply, derive_storage_traits, GetContainerChainAuthor, GetHostConfiguration,
        GetSessionContainerChains, MaybeSelfChainBlockAuthor, NodeActivityTrackingHelper,
        ParaIdAssignmentHooks, RelayStorageRootProvider, RemoveInvulnerables, SlotFrequency,
    },
    tp_xcm_core_buyer::BuyCoreCollatorProof,
    xcm::Version as XcmVersion,
    xcm::{IntoVersion, VersionedAssetId, VersionedAssets, VersionedLocation, VersionedXcm},
    xcm_runtime_apis::{
        dry_run::{CallDryRunEffects, Error as XcmDryRunApiError, XcmDryRunEffects},
        fees::Error as XcmPaymentApiError,
    },
};
pub use {
    dp_core::{AccountId, Address, Balance, BlockNumber, Hash, Header, Index, Signature},
    sp_runtime::{MultiAddress, Perbill, Permill},
};
use {
    frame_support::{
        storage::{with_storage_layer, with_transaction},
        traits::{ExistenceRequirement, WithdrawReasons},
    },
    polkadot_runtime_common::SlowAdjustingFeeUpdate,
};

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

/// CollatorId type expected by this runtime.
pub type CollatorId = AccountId;

/// The `TxExtension` to the basic transaction logic.
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
        frame_metadata_hash_extension::CheckMetadataHash<Runtime>,
    ),
>;

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, TxExtension>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, TxExtension>;

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

/// DANCE, the native token, uses 12 decimals of precision.
pub mod currency {
    use super::Balance;

    // Provide a common factor between runtimes based on a supply of 10_000_000 tokens.
    pub const SUPPLY_FACTOR: Balance = 100;

    pub const MICRODANCE: Balance = 1_000_000;
    pub const MILLIDANCE: Balance = 1_000_000_000;
    pub const DANCE: Balance = 1_000_000_000_000;
    pub const KILODANCE: Balance = 1_000_000_000_000_000;

    pub const STORAGE_BYTE_FEE: Balance = 100 * MICRODANCE * SUPPLY_FACTOR;
    pub const STORAGE_ITEM_FEE: Balance = 100 * MILLIDANCE * SUPPLY_FACTOR;

    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * STORAGE_ITEM_FEE + (bytes as Balance) * STORAGE_BYTE_FEE
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
        sp_runtime::{
            generic,
            traits::{BlakeTwo256, Hash as HashT},
        },
    };

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
    /// Opaque block hash type.
    pub type Hash = <BlakeTwo256 as HashT>::Output;
    /// Opaque signature type.
    pub use super::Signature;
}

impl_opaque_keys! {
    pub struct SessionKeys {
        pub nimbus: Initializer,
    }
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: Cow::Borrowed("dancebox"),
    impl_name: Cow::Borrowed("dancebox"),
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

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000;
pub const CENTS: Balance = UNIT / 30_000;
pub const MILLIUNIT: Balance = 1_000_000_000;
pub const MICROUNIT: Balance = 1_000_000;
/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

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
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = MultiBlockMigrations;
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
    type ExtensionsWeightInfo = weights::frame_system_extensions::SubstrateWeight<Runtime>;
}

impl cumulus_pallet_weight_reclaim::Config for Runtime {
    type WeightInfo = weights::cumulus_pallet_weight_reclaim::SubstrateWeight<Runtime>;
}

impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = dp_consensus::OnTimestampSet<
        <Self as pallet_author_inherent::Config>::SlotBeacon,
        ConstU64<{ SLOT_DURATION }>,
    >;
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    type WeightInfo = weights::pallet_timestamp::SubstrateWeight<Runtime>;
}

pub struct CanAuthor;
impl nimbus_primitives::CanAuthor<NimbusId> for CanAuthor {
    fn can_author(author: &NimbusId, slot: &u32) -> bool {
        let authorities = AuthorityAssignment::collator_container_chain(Session::current_index())
            .expect("authorities should be set")
            .orchestrator_chain;

        if authorities.is_empty() {
            return false;
        }

        let author_index = (*slot as usize) % authorities.len();
        let expected_author = &authorities[author_index];

        expected_author == author
    }
    #[cfg(feature = "runtime-benchmarks")]
    fn get_authors(_slot: &u32) -> Vec<NimbusId> {
        AuthorityAssignment::collator_container_chain(Session::current_index())
            .expect("authorities should be set")
            .orchestrator_chain
    }
}

impl pallet_author_inherent::Config for Runtime {
    type AuthorId = NimbusId;
    type AccountLookup = dp_consensus::NimbusLookUp;
    type CanAuthor = CanAuthor;
    type SlotBeacon = dp_consensus::AuraDigestSlotBeacon<Runtime>;
    type WeightInfo = weights::pallet_author_inherent::SubstrateWeight<Runtime>;
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
    type MaxFreezes = ConstU32<10>;
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type DoneSlashHandler = ();
    type WeightInfo = weights::pallet_balances::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const TransactionByteFee: Balance = 1;
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction =
        FungibleAdapter<Balances, tanssi_runtime_common::DealWithFees<Runtime>>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = WeightToFee;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
    type WeightInfo = weights::pallet_transaction_payment::SubstrateWeight<Runtime>;
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

/// Only callable after `set_validation_data` is called which forms this proof the same way
fn relay_chain_state_proof() -> RelayChainStateProof {
    let relay_storage_root =
        RelaychainDataProvider::<Runtime>::current_relay_chain_state().state_root;
    let relay_chain_state = cumulus_pallet_parachain_system::RelayStateProof::<Runtime>::get()
        .expect("set in `set_validation_data`");
    RelayChainStateProof::new(ParachainInfo::get(), relay_storage_root, relay_chain_state)
        .expect("Invalid relay chain state proof, already constructed in `set_validation_data`")
}

pub struct BabeCurrentBlockRandomnessGetter;
impl BabeCurrentBlockRandomnessGetter {
    fn get_block_randomness() -> Option<Hash> {
        if cfg!(feature = "runtime-benchmarks") {
            // storage reads as per actual reads
            let _relay_storage_root =
                RelaychainDataProvider::<Runtime>::current_relay_chain_state().state_root;

            let _relay_chain_state =
                cumulus_pallet_parachain_system::RelayStateProof::<Runtime>::get();
            let benchmarking_babe_output = Hash::default();
            return Some(benchmarking_babe_output);
        }

        relay_chain_state_proof()
            .read_optional_entry::<Option<Hash>>(
                relay_chain::well_known_keys::CURRENT_BLOCK_RANDOMNESS,
            )
            .ok()
            .flatten()
            .flatten()
    }

    /// Return the block randomness from the relay mixed with the provided subject.
    /// This ensures that the randomness will be different on different pallets, as long as the subject is different.
    fn get_block_randomness_mixed(subject: &[u8]) -> Option<Hash> {
        Self::get_block_randomness()
            .map(|random_hash| mix_randomness::<Runtime>(random_hash, subject))
    }
}

/// Combines the vrf output of the previous relay block with the provided subject.
/// This ensures that the randomness will be different on different pallets, as long as the subject is different.
fn mix_randomness<T: frame_system::Config>(vrf_output: Hash, subject: &[u8]) -> T::Hash {
    let mut digest = Vec::new();
    digest.extend_from_slice(vrf_output.as_ref());
    digest.extend_from_slice(subject);

    T::Hashing::hash(digest.as_slice())
}

// Randomness trait
impl frame_support::traits::Randomness<Hash, BlockNumber> for BabeCurrentBlockRandomnessGetter {
    fn random(subject: &[u8]) -> (Hash, BlockNumber) {
        let block_number = frame_system::Pallet::<Runtime>::block_number();
        let randomness = Self::get_block_randomness_mixed(subject).unwrap_or_default();

        (randomness, block_number)
    }
}

pub struct OwnApplySession;
impl pallet_initializer::ApplyNewSession<Runtime> for OwnApplySession {
    fn apply_new_session(
        _changed: bool,
        session_index: u32,
        all_validators: Vec<(AccountId, NimbusId)>,
        queued: Vec<(AccountId, NimbusId)>,
    ) {
        // We first initialize Configuration
        Configuration::initializer_on_new_session(&session_index);
        // Next: Registrar
        Registrar::initializer_on_new_session(&session_index);
        // Next: AuthorityMapping
        AuthorityMapping::initializer_on_new_session(&session_index, &all_validators);

        let next_collators = queued.iter().map(|(k, _)| k.clone()).collect();

        // Next: CollatorAssignment
        let assignments =
            CollatorAssignment::initializer_on_new_session(&session_index, next_collators);

        let queued_id_to_nimbus_map = queued.iter().cloned().collect();
        AuthorityAssignment::initializer_on_new_session(
            &session_index,
            &queued_id_to_nimbus_map,
            &assignments.next_assignment,
        );

        // Next: InactivityTracking
        InactivityTracking::process_ended_session();
    }

    fn on_before_session_ending() {
        InactivityTracking::on_before_session_ending();
    }
}

impl pallet_initializer::Config for Runtime {
    type SessionIndex = u32;

    /// The identifier type for an authority.
    type AuthorityId = NimbusId;

    type SessionHandler = OwnApplySession;
}

impl parachain_info::Config for Runtime {}

/// Returns a list of collators by combining pallet_invulnerables and pallet_pooled_staking.
pub struct CollatorsFromInvulnerablesAndThenFromStaking;

/// Play the role of the session manager.
impl SessionManager<CollatorId> for CollatorsFromInvulnerablesAndThenFromStaking {
    fn new_session(index: SessionIndex) -> Option<Vec<CollatorId>> {
        if <frame_system::Pallet<Runtime>>::block_number() == 0 {
            // Do not show this log in genesis
            log::debug!(
                "assembling new collators for new session {} at #{:?}",
                index,
                <frame_system::Pallet<Runtime>>::block_number(),
            );
        } else {
            log::info!(
                "assembling new collators for new session {} at #{:?}",
                index,
                <frame_system::Pallet<Runtime>>::block_number(),
            );
        }

        let invulnerables = Invulnerables::invulnerables().to_vec();
        let candidates_staking =
            pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
        // Max number of collators is set in pallet_configuration
        let target_session_index = index.saturating_add(1);
        let max_collators =
            <Configuration as GetHostConfiguration<u32>>::max_collators(target_session_index);
        let collators = invulnerables
            .iter()
            .cloned()
            .chain(candidates_staking.into_iter().filter_map(|elig| {
                let cand = elig.candidate;
                if invulnerables.contains(&cand) {
                    // If a candidate is both in pallet_invulnerables and pallet_staking, do not count it twice
                    None
                } else {
                    Some(cand)
                }
            }))
            .take(max_collators as usize)
            .collect();

        // TODO: weight?
        /*
        frame_system::Pallet::<T>::register_extra_weight_unchecked(
            T::WeightInfo::new_session(invulnerables.len() as u32),
            DispatchClass::Mandatory,
        );
        */
        Some(collators)
    }
    fn start_session(_: SessionIndex) {
        // we don't care.
    }
    fn end_session(_: SessionIndex) {
        // we don't care.
    }
}

parameter_types! {
    pub const Period: u32 = prod_or_fast!(1 * HOURS, 1 * MINUTES);
    pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = CollatorId;
    // we don't have stash and controller, thus we don't need the convert as well.
    type ValidatorIdOf = ConvertInto;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionManager = CollatorsFromInvulnerablesAndThenFromStaking;
    // Essentially just Aura, but let's be pedantic.
    type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
    type Keys = SessionKeys;
    type WeightInfo = weights::pallet_session::SubstrateWeight<Runtime>;
    type DisablingStrategy = ();
}

/// Read full_rotation_period from pallet_configuration
pub struct ConfigurationCollatorRotationSessionPeriod;

impl Get<u32> for ConfigurationCollatorRotationSessionPeriod {
    fn get() -> u32 {
        Configuration::config().full_rotation_period
    }
}

pub struct BabeGetRandomnessForNextBlock;

impl GetRandomnessForNextBlock<u32> for BabeGetRandomnessForNextBlock {
    fn should_end_session(n: u32) -> bool {
        <Runtime as pallet_session::Config>::ShouldEndSession::should_end_session(n)
    }

    fn get_randomness() -> [u8; 32] {
        let block_number = System::block_number();
        let random_seed = if block_number != 0 {
            if let Some(random_hash) =
                BabeCurrentBlockRandomnessGetter::get_block_randomness_mixed(b"CollatorAssignment")
            {
                // Return random_hash as a [u8; 32] instead of a Hash
                let mut buf = [0u8; 32];
                let len = sp_std::cmp::min(32, random_hash.as_ref().len());
                buf[..len].copy_from_slice(&random_hash.as_ref()[..len]);

                buf
            } else {
                // If there is no randomness (e.g when running in dev mode), return [0; 32]
                [0; 32]
            }
        } else {
            // In block 0 (genesis) there is no randomness
            [0; 32]
        };

        random_seed
    }
}

pub struct RemoveInvulnerablesImpl;

impl RemoveInvulnerables<CollatorId> for RemoveInvulnerablesImpl {
    fn remove_invulnerables(
        collators: &mut Vec<CollatorId>,
        num_invulnerables: usize,
    ) -> Vec<CollatorId> {
        if num_invulnerables == 0 {
            return vec![];
        }
        let all_invulnerables = pallet_invulnerables::Invulnerables::<Runtime>::get();
        if all_invulnerables.is_empty() {
            return vec![];
        }
        let mut invulnerables = vec![];
        // TODO: use binary_search when invulnerables are sorted
        collators.retain(|x| {
            if invulnerables.len() < num_invulnerables && all_invulnerables.contains(x) {
                invulnerables.push(x.clone());
                false
            } else {
                true
            }
        });

        invulnerables
    }
}

pub struct ParaIdAssignmentHooksImpl;

impl ParaIdAssignmentHooksImpl {
    fn charge_para_ids_internal(
        blocks_per_session: tp_traits::BlockNumber,
        para_id: ParaId,
        currently_assigned: &BTreeSet<ParaId>,
        maybe_tip: &Option<BalanceOf<Runtime>>,
    ) -> Result<Weight, DispatchError> {
        use frame_support::traits::Currency;
        type ServicePaymentCurrency = <Runtime as pallet_services_payment::Config>::Currency;

        // Check if the container chain has enough credits for a session assignments
        let maybe_assignment_imbalance =
            if  pallet_services_payment::Pallet::<Runtime>::burn_collator_assignment_free_credit_for_para(&para_id).is_err() {
                let (amount_to_charge, _weight) =
                    <Runtime as pallet_services_payment::Config>::ProvideCollatorAssignmentCost::collator_assignment_cost(&para_id);
                Some(<ServicePaymentCurrency as Currency<AccountId>>::withdraw(
                    &pallet_services_payment::Pallet::<Runtime>::parachain_tank(para_id),
                    amount_to_charge,
                    WithdrawReasons::FEE,
                    ExistenceRequirement::KeepAlive,
                )?)
            } else {
                None
            };

        if let Some(tip) = maybe_tip {
            if let Err(e) = pallet_services_payment::Pallet::<Runtime>::charge_tip(&para_id, tip) {
                // Return assignment imbalance to tank on error
                if let Some(assignment_imbalance) = maybe_assignment_imbalance {
                    <Runtime as pallet_services_payment::Config>::Currency::resolve_creating(
                        &pallet_services_payment::Pallet::<Runtime>::parachain_tank(para_id),
                        assignment_imbalance,
                    );
                }
                return Err(e);
            }
        }

        if let Some(assignment_imbalance) = maybe_assignment_imbalance {
            <Runtime as pallet_services_payment::Config>::OnChargeForCollatorAssignment::on_unbalanced(assignment_imbalance);
        }

        // If the para has been assigned collators for this session it must have enough block credits
        // for the current and the next session.
        let block_credits_needed = if currently_assigned.contains(&para_id) {
            blocks_per_session * 2
        } else {
            blocks_per_session
        };
        // Check if the container chain has enough credits for producing blocks
        let free_block_credits =
            pallet_services_payment::BlockProductionCredits::<Runtime>::get(para_id)
                .unwrap_or_default();
        let remaining_block_credits = block_credits_needed.saturating_sub(free_block_credits);
        let (block_production_costs, _) =
            <Runtime as pallet_services_payment::Config>::ProvideBlockProductionCost::block_cost(
                &para_id,
            );
        // Check if we can withdraw
        let remaining_block_credits_to_pay =
            u128::from(remaining_block_credits).saturating_mul(block_production_costs);
        let remaining_to_pay = remaining_block_credits_to_pay;
        // This should take into account whether we tank goes below ED
        // The true refers to keepAlive
        Balances::can_withdraw(
            &pallet_services_payment::Pallet::<Runtime>::parachain_tank(para_id),
            remaining_to_pay,
        )
        .into_result(true)?;
        // TODO: Have proper weight
        Ok(Weight::zero())
    }
}

impl<AC> ParaIdAssignmentHooks<BalanceOf<Runtime>, AC> for ParaIdAssignmentHooksImpl {
    fn pre_assignment(para_ids: &mut Vec<ParaId>, currently_assigned: &BTreeSet<ParaId>) {
        let blocks_per_session = Period::get();
        para_ids.retain(|para_id| {
            with_transaction(|| {
                let max_tip =
                    pallet_services_payment::MaxTip::<Runtime>::get(para_id).unwrap_or_default();
                TransactionOutcome::Rollback(Self::charge_para_ids_internal(
                    blocks_per_session,
                    *para_id,
                    currently_assigned,
                    &Some(max_tip),
                ))
            })
            .is_ok()
        });
    }

    fn post_assignment(
        current_assigned: &BTreeSet<ParaId>,
        new_assigned: &mut BTreeMap<ParaId, Vec<AC>>,
        maybe_tip: &Option<BalanceOf<Runtime>>,
    ) -> Weight {
        let blocks_per_session = Period::get();
        let mut total_weight = Weight::zero();
        new_assigned.retain(|&para_id, collators| {
            // Short-circuit in case collators are empty
            if collators.is_empty() {
                return true;
            }
            with_storage_layer(|| {
                Self::charge_para_ids_internal(
                    blocks_per_session,
                    para_id,
                    current_assigned,
                    maybe_tip,
                )
            })
            .inspect(|weight| {
                total_weight += *weight;
            })
            .is_ok()
        });
        total_weight
    }

    /// Make those para ids valid by giving them enough credits, for benchmarking.
    #[cfg(feature = "runtime-benchmarks")]
    fn make_valid_para_ids(para_ids: &[ParaId]) {
        use frame_support::assert_ok;

        let blocks_per_session = Period::get();
        // Enough credits to run any benchmark
        let block_credits = 20 * blocks_per_session;
        let session_credits = 20;

        for para_id in para_ids {
            assert_ok!(ServicesPayment::set_block_production_credits(
                RuntimeOrigin::root(),
                *para_id,
                block_credits,
            ));
            assert_ok!(ServicesPayment::set_collator_assignment_credits(
                RuntimeOrigin::root(),
                *para_id,
                session_credits,
            ));
        }
    }
}

impl pallet_collator_assignment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type HostConfiguration = Configuration;
    type ContainerChains = Registrar;
    type SessionIndex = u32;
    type SelfParaId = ParachainInfo;
    type ShouldRotateAllCollators =
        RotateCollatorsEveryNSessions<ConfigurationCollatorRotationSessionPeriod>;
    type Randomness =
        pallet_collator_assignment::ParachainRandomness<BabeGetRandomnessForNextBlock, Runtime>;
    type RemoveInvulnerables = RemoveInvulnerablesImpl;
    type ParaIdAssignmentHooks = ParaIdAssignmentHooksImpl;
    type CollatorAssignmentTip = ServicesPayment;
    type Currency = Balances;
    type ForceEmptyOrchestrator = ConstBool<false>;
    type CoreAllocationConfiguration = ();
    type WeightInfo = weights::pallet_collator_assignment::SubstrateWeight<Runtime>;
}

impl pallet_authority_assignment::Config for Runtime {
    type SessionIndex = u32;
    type AuthorityId = NimbusId;
}

pub const FIXED_BLOCK_PRODUCTION_COST: u128 = 1 * currency::MICRODANCE;
pub const FIXED_COLLATOR_ASSIGNMENT_COST: u128 = 100 * currency::MICRODANCE;

pub struct BlockProductionCost<Runtime>(PhantomData<Runtime>);
impl ProvideBlockProductionCost<Runtime> for BlockProductionCost<Runtime> {
    fn block_cost(_para_id: &ParaId) -> (u128, Weight) {
        (FIXED_BLOCK_PRODUCTION_COST, Weight::zero())
    }
}

pub struct CollatorAssignmentCost<Runtime>(PhantomData<Runtime>);
impl ProvideCollatorAssignmentCost<Runtime> for CollatorAssignmentCost<Runtime> {
    fn collator_assignment_cost(_para_id: &ParaId) -> (u128, Weight) {
        (FIXED_COLLATOR_ASSIGNMENT_COST, Weight::zero())
    }
}

parameter_types! {
    // 60 days worth of blocks
    pub const FreeBlockProductionCredits: BlockNumber = 60 * DAYS;
    // 60 days worth of blocks
    pub const FreeCollatorAssignmentCredits: u32 = FreeBlockProductionCredits::get()/Period::get();
}

impl pallet_services_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    /// Handler for fees
    type OnChargeForBlock = ();
    type OnChargeForCollatorAssignment = ();
    type OnChargeForCollatorAssignmentTip = ();
    /// Currency type for fee payment
    type Currency = Balances;
    /// Provider of a block cost which can adjust from block to block
    type ProvideBlockProductionCost = BlockProductionCost<Runtime>;
    /// Provider of a block cost which can adjust from block to block
    type ProvideCollatorAssignmentCost = CollatorAssignmentCost<Runtime>;
    /// The maximum number of block credits that can be accumulated
    type FreeBlockProductionCredits = FreeBlockProductionCredits;
    /// The maximum number of session credits that can be accumulated
    type FreeCollatorAssignmentCredits = FreeCollatorAssignmentCredits;
    type ManagerOrigin =
        EitherOfDiverse<pallet_registrar::EnsureSignedByManager<Runtime>, EnsureRoot<AccountId>>;
    type WeightInfo = weights::pallet_services_payment::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const ProfileDepositBaseFee: Balance = currency::STORAGE_ITEM_FEE;
    pub const ProfileDepositByteFee: Balance = currency::STORAGE_BYTE_FEE;
    #[derive(Clone)]
    pub const MaxAssignmentsPerParaId: u32 = 10;
    #[derive(Clone)]
    pub const MaxNodeUrlLen: u32 = 200;
}

pub type DataPreserversProfileId = u64;

impl pallet_data_preservers::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeHoldReason = RuntimeHoldReason;
    type Currency = Balances;
    type WeightInfo = weights::pallet_data_preservers::SubstrateWeight<Runtime>;

    type ProfileId = DataPreserversProfileId;
    type ProfileDeposit = tp_traits::BytesDeposit<ProfileDepositBaseFee, ProfileDepositByteFee>;
    type AssignmentProcessor = tp_data_preservers_common::AssignmentProcessor<Runtime>;

    type AssignmentOrigin = pallet_registrar::EnsureSignedByManager<Runtime>;
    type ForceSetProfileOrigin = EnsureRoot<AccountId>;

    type MaxAssignmentsPerParaId = MaxAssignmentsPerParaId;
    type MaxNodeUrlLen = MaxNodeUrlLen;
    type MaxParaIdsVecLen = MaxLengthParaIds;
}

impl pallet_author_noting::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ContainerChains = CollatorAssignment;
    type SlotBeacon = dp_consensus::AuraDigestSlotBeacon<Runtime>;
    type ContainerChainAuthor = CollatorAssignment;
    type AuthorNotingHook = (
        XcmCoreBuyer,
        InflationRewards,
        ServicesPayment,
        InactivityTracking,
    );
    type RelayOrPara = pallet_author_noting::ParaMode<
        cumulus_pallet_parachain_system::RelaychainDataProvider<Self>,
    >;
    type MaxContainerChains = MaxLengthParaIds;
    type WeightInfo = weights::pallet_author_noting::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const PotId: PalletId = PalletId(*b"PotStake");
    pub const MaxCandidates: u32 = 1000;
    pub const MinCandidates: u32 = 5;
    pub const SessionLength: BlockNumber = 5;
    pub const MaxInvulnerables: u32 = 100;
    pub const ExecutiveBody: BodyId = BodyId::Executive;
}

impl pallet_invulnerables::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type UpdateOrigin = EnsureRoot<AccountId>;
    type MaxInvulnerables = MaxInvulnerables;
    type CollatorId = <Self as frame_system::Config>::AccountId;
    type CollatorIdOf = ConvertInto;
    type CollatorRegistration = Session;
    type WeightInfo = weights::pallet_invulnerables::SubstrateWeight<Runtime>;
    #[cfg(feature = "runtime-benchmarks")]
    type Currency = Balances;
}

parameter_types! {
    #[derive(Clone)]
    pub const MaxLengthParaIds: u32 = 100u32;
    pub const MaxEncodedGenesisDataSize: u32 = 5_000_000u32; // 5MB
}

pub struct CurrentSessionIndexGetter;

impl tp_traits::GetSessionIndex<u32> for CurrentSessionIndexGetter {
    /// Returns current session index.
    fn session_index() -> u32 {
        Session::current_index()
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn skip_to_session(session_index: SessionIndex) {
        while Session::current_index() < session_index {
            Session::rotate_session();
        }
    }
}

impl pallet_configuration::Config for Runtime {
    type SessionDelay = ConstU32<2>;
    type SessionIndex = u32;
    type CurrentSessionIndex = CurrentSessionIndexGetter;
    type ForceEmptyOrchestrator = ConstBool<false>;
    type WeightInfo = weights::pallet_configuration::SubstrateWeight<Runtime>;
}

pub struct DanceboxRegistrarHooks;

impl RegistrarHooks for DanceboxRegistrarHooks {
    fn para_marked_valid_for_collating(para_id: ParaId) -> Weight {
        // Give free credits but only once per para id
        ServicesPayment::give_free_credits(&para_id)
    }

    fn para_deregistered(para_id: ParaId) -> Weight {
        // Clear pallet_author_noting storage
        if let Err(e) = AuthorNoting::kill_author_data(RuntimeOrigin::root(), para_id) {
            log::warn!(
                "Failed to kill_author_data after para id {} deregistered: {:?}",
                u32::from(para_id),
                e,
            );
        }
        // Remove bootnodes from pallet_data_preservers
        DataPreservers::para_deregistered(para_id);

        ServicesPayment::para_deregistered(para_id);

        XcmCoreBuyer::para_deregistered(para_id);

        Weight::default()
    }

    fn check_valid_for_collating(para_id: ParaId) -> DispatchResult {
        // To be able to call mark_valid_for_collating, a container chain must have bootnodes
        DataPreservers::check_valid_for_collating(para_id)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn benchmarks_ensure_valid_for_collating(para_id: ParaId) {
        use {
            frame_support::traits::EnsureOriginWithArg,
            pallet_data_preservers::{ParaIdsFilter, Profile, ProfileMode},
        };

        let profile = Profile {
            url: b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9"
                    .to_vec()
                    .try_into()
                    .expect("to fit in BoundedVec"),
            para_ids: ParaIdsFilter::AnyParaId,
            mode: ProfileMode::Bootnode,
            assignment_request: tp_data_preservers_common::ProviderRequest::Free,
        };

        let profile_id = pallet_data_preservers::NextProfileId::<Runtime>::get();
        let profile_owner = AccountId::new([1u8; 32]);
        DataPreservers::force_create_profile(RuntimeOrigin::root(), profile, profile_owner)
            .expect("profile create to succeed");

        let para_manager =
            <Runtime as pallet_data_preservers::Config>::AssignmentOrigin::try_successful_origin(
                &para_id,
            )
            .expect("should be able to get para manager");

        DataPreservers::start_assignment(
            para_manager,
            profile_id,
            para_id,
            tp_data_preservers_common::AssignerExtra::Free,
        )
        .expect("assignement to work");

        assert!(
            pallet_data_preservers::Assignments::<Runtime>::get(para_id).contains(&profile_id),
            "profile should be correctly assigned"
        );
    }
}

pub struct PalletRelayStorageRootProvider;

impl RelayStorageRootProvider for PalletRelayStorageRootProvider {
    fn get_relay_storage_root(relay_block_number: u32) -> Option<H256> {
        pallet_relay_storage_roots::pallet::RelayStorageRoot::<Runtime>::get(relay_block_number)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn set_relay_storage_root(relay_block_number: u32, storage_root: Option<H256>) {
        pallet_relay_storage_roots::pallet::RelayStorageRootKeys::<Runtime>::mutate(|x| {
            if storage_root.is_some() {
                if x.is_full() {
                    let key = x.remove(0);
                    pallet_relay_storage_roots::pallet::RelayStorageRoot::<Runtime>::remove(key);
                }
                let pos = x.iter().position(|x| *x >= relay_block_number);
                if let Some(pos) = pos {
                    if x[pos] != relay_block_number {
                        x.try_insert(pos, relay_block_number).unwrap();
                    }
                } else {
                    // Push at end
                    x.try_push(relay_block_number).unwrap();
                }
            } else {
                let pos = x.iter().position(|x| *x == relay_block_number);
                if let Some(pos) = pos {
                    x.remove(pos);
                }
            }
        });
        pallet_relay_storage_roots::pallet::RelayStorageRoot::<Runtime>::set(
            relay_block_number,
            storage_root,
        );
    }
}

impl pallet_registrar::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RegistrarOrigin =
        EitherOfDiverse<pallet_registrar::EnsureSignedByManager<Runtime>, EnsureRoot<AccountId>>;
    type MarkValidForCollatingOrigin = EnsureRoot<AccountId>;
    type MaxLengthParaIds = MaxLengthParaIds;
    type MaxGenesisDataSize = MaxEncodedGenesisDataSize;
    type RegisterWithRelayProofOrigin = EnsureSigned<AccountId>;
    type RelayStorageRootProvider = PalletRelayStorageRootProvider;
    type SessionDelay = ConstU32<2>;
    type SessionIndex = u32;
    type CurrentSessionIndex = CurrentSessionIndexGetter;
    type Currency = Balances;
    type RegistrarHooks = DanceboxRegistrarHooks;
    type RuntimeHoldReason = RuntimeHoldReason;
    type InnerRegistrar = ();
    type WeightInfo = weights::pallet_registrar::SubstrateWeight<Runtime>;
    type DataDepositPerByte = DataDepositPerByte;
}

impl pallet_authority_mapping::Config for Runtime {
    type SessionIndex = u32;
    type SessionRemovalBoundary = ConstU32<2>;
    type AuthorityId = NimbusId;
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
#[apply(derive_storage_traits)]
#[derive(Copy, Ord, PartialOrd, MaxEncodedLen, DecodeWithMemTracking)]
#[allow(clippy::unnecessary_cast)]
pub enum ProxyType {
    /// All calls can be proxied. This is the trivial/most permissive filter.
    Any = 0,
    /// Only extrinsics that do not transfer funds.
    NonTransfer = 1,
    /// Only extrinsics related to governance (democracy and collectives).
    Governance = 2,
    /// Only extrinsics related to staking.
    Staking = 3,
    /// Allow to veto an announced proxy call.
    CancelProxy = 4,
    /// Allow extrinsic related to Balances.
    Balances = 5,
    /// Allow extrinsics related to Registrar
    Registrar = 6,
    /// Allow extrinsics related to Registrar that needs to be called through Sudo
    SudoRegistrar = 7,
    /// Allow extrinsics from the Session pallet for key management.
    SessionKeyManagement = 8,
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
                        | RuntimeCall::Registrar(..)
                )
            }
            // We don't have governance yet
            ProxyType::Governance => false,
            ProxyType::Staking => {
                matches!(c, RuntimeCall::Session(..) | RuntimeCall::PooledStaking(..))
            }
            ProxyType::CancelProxy => matches!(
                c,
                RuntimeCall::Proxy(pallet_proxy::Call::reject_announcement { .. })
            ),
            ProxyType::Balances => {
                matches!(c, RuntimeCall::Balances(..))
            }
            ProxyType::Registrar => {
                matches!(
                    c,
                    RuntimeCall::Registrar(..) | RuntimeCall::DataPreservers(..)
                )
            }
            ProxyType::SudoRegistrar => match c {
                RuntimeCall::Sudo(pallet_sudo::Call::sudo { call: ref x }) => {
                    matches!(
                        x.as_ref(),
                        &RuntimeCall::Registrar(..) | &RuntimeCall::DataPreservers(..)
                    )
                }
                _ => false,
            },
            ProxyType::SessionKeyManagement => {
                matches!(c, RuntimeCall::Session(..))
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
    // Additional storage item size of 33 bytes (32 bytes AccountId + 1 byte sizeof(ProxyType)).
    type ProxyDepositFactor = ConstU128<{ currency::deposit(0, 33) }>;
    type MaxProxies = ConstU32<32>;
    type MaxPending = ConstU32<32>;
    type CallHasher = BlakeTwo256;
    type AnnouncementDepositBase = ConstU128<{ currency::deposit(1, 8) }>;
    // Additional storage item size of 68 bytes:
    // - 32 bytes AccountId
    // - 32 bytes Hasher (Blake2256)
    // - 4 bytes BlockNumber (u32)
    type AnnouncementDepositFactor = ConstU128<{ currency::deposit(0, 68) }>;
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

impl pallet_migrations::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MigrationsList = (tanssi_runtime_common::migrations::DanceboxMigrations<Runtime>,);
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
            RuntimeCall::Balances(..)
                | RuntimeCall::Registrar(..)
                | RuntimeCall::Session(..)
                | RuntimeCall::System(..)
                | RuntimeCall::PooledStaking(..)
                | RuntimeCall::Utility(..)
                | RuntimeCall::PolkadotXcm(..)
        )
    }
}

/// Normal Call Filter
pub struct NormalFilter;
impl Contains<RuntimeCall> for NormalFilter {
    fn contains(_c: &RuntimeCall) -> bool {
        true
    }
}

impl pallet_maintenance_mode::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type NormalCallFilter = NormalFilter;
    type MaintenanceCallFilter = InsideBoth<MaintenanceFilter, NormalFilter>;
    type MaintenanceOrigin = EnsureRoot<AccountId>;
    type XcmExecutionManager = XcmExecutionManager;
}

parameter_types! {
    pub const MaxStorageRoots: u32 = 10; // 1 minute of relay blocks
}

impl pallet_relay_storage_roots::Config for Runtime {
    type RelaychainStateProvider = cumulus_pallet_parachain_system::RelaychainDataProvider<Self>;
    type MaxStorageRoots = MaxStorageRoots;
    type WeightInfo = weights::pallet_relay_storage_roots::SubstrateWeight<Runtime>;
}

impl pallet_root_testing::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
    pub StakingAccount: AccountId32 = PalletId(*b"POOLSTAK").into_account_truncating();
    pub const InitialManualClaimShareValue: u128 = currency::MILLIDANCE;
    pub const InitialAutoCompoundingShareValue: u128 = currency::MILLIDANCE;
    pub const MinimumSelfDelegation: u128 = 10 * currency::KILODANCE;
    pub const RewardsCollatorCommission: Perbill = Perbill::from_percent(20);
    // Need to wait 2 sessions before being able to join or leave staking pools
    pub const StakingSessionDelay: u32 = 2;
}

pub struct CandidateIsOnlineAndHasRegisteredKeys;
impl IsCandidateEligible<AccountId> for CandidateIsOnlineAndHasRegisteredKeys {
    fn is_candidate_eligible(a: &AccountId) -> bool {
        <Session as ValidatorRegistration<AccountId>>::is_registered(a)
            && !InactivityTracking::is_node_offline(a)
    }
    #[cfg(feature = "runtime-benchmarks")]
    fn make_candidate_eligible(a: &AccountId, eligible: bool) {
        use sp_core::crypto::UncheckedFrom;
        if eligible {
            let account_slice: &[u8; 32] = a.as_ref();
            let _ = Session::set_keys(
                RuntimeOrigin::signed(a.clone()),
                SessionKeys {
                    nimbus: NimbusId::unchecked_from(*account_slice),
                },
                vec![],
            );
        } else {
            let _ = Session::purge_keys(RuntimeOrigin::signed(a.clone()));
        }

        if InactivityTracking::is_node_offline(a) {
            InactivityTracking::make_node_online(a);
        }
    }
}

parameter_types! {
    pub const MaxCandidatesBufferSize: u32 = 100;
}
impl pallet_pooled_staking::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Balance = Balance;
    type StakingAccount = StakingAccount;
    type InitialManualClaimShareValue = InitialManualClaimShareValue;
    type InitialAutoCompoundingShareValue = InitialAutoCompoundingShareValue;
    type MinimumSelfDelegation = MinimumSelfDelegation;
    type RuntimeHoldReason = RuntimeHoldReason;
    type RewardsCollatorCommission = RewardsCollatorCommission;
    type JoiningRequestTimer = SessionTimer<Runtime, StakingSessionDelay>;
    type LeavingRequestTimer = SessionTimer<Runtime, StakingSessionDelay>;
    type EligibleCandidatesBufferSize = MaxCandidatesBufferSize;
    type EligibleCandidatesFilter = CandidateIsOnlineAndHasRegisteredKeys;
    type WeightInfo = weights::pallet_pooled_staking::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub ParachainBondAccount: AccountId32 = PalletId(*b"ParaBond").into_account_truncating();
    pub PendingRewardsAccount: AccountId32 = PalletId(*b"PENDREWD").into_account_truncating();
    // The equation to solve is:
    // initial_supply * (1.05) = initial_supply * (1+x)^5_259_600
    // we should solve for x = (1.05)^(1/5_259_600) -1 -> 0.000000009 per block or 9/1_000_000_000
    // 1% in the case of dev mode
    // TODO: better calculus for going from annual to block inflation (if it can be done)
    pub const InflationRate: Perbill = prod_or_fast!(Perbill::from_parts(9), Perbill::from_percent(1));

    // 30% for parachain bond, so 70% for staking
    pub const RewardsPortion: Perbill = Perbill::from_percent(70);
}

pub struct GetSelfChainBlockAuthor;
impl MaybeSelfChainBlockAuthor<AccountId32> for GetSelfChainBlockAuthor {
    fn get_block_author() -> Option<AccountId32> {
        // TODO: we should do a refactor here, and use either authority-mapping or collator-assignemnt
        // we should also make sure we actually account for the weight of these
        // although most of these should be cached as they are read every block
        let slot = u64::from(<Runtime as pallet_author_inherent::Config>::SlotBeacon::slot());
        let self_para_id = ParachainInfo::get();
        CollatorAssignment::author_for_slot(slot.into(), self_para_id)
    }
}

pub struct OnUnbalancedInflation;
impl frame_support::traits::OnUnbalanced<Credit<AccountId, Balances>> for OnUnbalancedInflation {
    fn on_nonzero_unbalanced(credit: Credit<AccountId, Balances>) {
        let _ = <Balances as Balanced<_>>::resolve(&ParachainBondAccount::get(), credit);
    }
}

impl pallet_inflation_rewards::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type ContainerChains = Registrar;
    type GetSelfChainBlockAuthor = GetSelfChainBlockAuthor;
    type InflationRate = InflationRate;
    type OnUnbalanced = OnUnbalancedInflation;
    type PendingRewardsAccount = PendingRewardsAccount;
    type StakingRewardsDistributor = InvulnerableRewardDistribution<Self, Balances, PooledStaking>;
    type RewardsPortion = RewardsPortion;
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

parameter_types! {
    // 1 entry, storing 253 bytes on-chain in the worst case
    pub const OpenStreamHoldAmount: Balance = currency::deposit(1, 253);
}

impl pallet_stream_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type StreamId = StreamId;
    type TimeUnit = tp_stream_payment_common::TimeUnit;
    type Balance = Balance;
    type AssetId = tp_stream_payment_common::AssetId;
    type AssetsManager = tp_stream_payment_common::AssetsManager<Runtime>;
    type Currency = Balances;
    type OpenStreamHoldAmount = OpenStreamHoldAmount;
    type RuntimeHoldReason = RuntimeHoldReason;
    type TimeProvider = tp_stream_payment_common::TimeProvider<Runtime>;
    type WeightInfo = weights::pallet_stream_payment::SubstrateWeight<Runtime>;
}

parameter_types! {
    // 1 entry, storing 258 bytes on-chain
    pub const BasicDeposit: Balance = currency::deposit(1, 258);
    // 1 entry, storing 53 bytes on-chain
    pub const SubAccountDeposit: Balance = currency::deposit(1, 53);
    // Additional bytes adds 0 entries, storing 1 byte on-chain
    pub const ByteDeposit: Balance = currency::deposit(0, 1);
    pub const UsernameDeposit: Balance = currency::deposit(0, 32);
    pub const MaxSubAccounts: u32 = 100;
    pub const MaxAdditionalFields: u32 = 100;
    pub const MaxRegistrars: u32 = 20;
}

impl pallet_identity::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type BasicDeposit = BasicDeposit;
    type ByteDeposit = ByteDeposit;
    type UsernameDeposit = UsernameDeposit;
    type SubAccountDeposit = SubAccountDeposit;
    type MaxSubAccounts = MaxSubAccounts;
    type MaxRegistrars = MaxRegistrars;
    type IdentityInformation = pallet_identity::legacy::IdentityInfo<MaxAdditionalFields>;
    // Slashed balances are burnt
    type Slashed = ();
    type ForceOrigin = EnsureRoot<AccountId>;
    type RegistrarOrigin = EnsureRoot<AccountId>;
    type OffchainSignature = Signature;
    type SigningPublicKey = <Signature as Verify>::Signer;
    type UsernameAuthorityOrigin = EnsureRoot<Self::AccountId>;
    type PendingUsernameExpiration = ConstU32<{ 7 * DAYS }>;
    type UsernameGracePeriod = ConstU32<{ 30 * DAYS }>;
    type MaxSuffixLength = ConstU32<7>;
    type MaxUsernameLength = ConstU32<32>;
    type WeightInfo = weights::pallet_identity::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const TreasuryId: PalletId = PalletId(*b"tns/tsry");
    pub const ProposalBond: Permill = Permill::from_percent(5);
    pub TreasuryAccount: AccountId = Treasury::account_id();
    pub const MaxBalance: Balance = Balance::MAX;
    // We allow it to be 1 minute in fast mode to be able to test it
    pub const SpendPeriod: BlockNumber = prod_or_fast!(6 * DAYS, 1 * MINUTES);
    pub const DataDepositPerByte: Balance = 1 * CENTS;
}

impl pallet_treasury::Config for Runtime {
    type PalletId = TreasuryId;
    type Currency = Balances;

    type RejectOrigin = EnsureRoot<AccountId>;
    type RuntimeEvent = RuntimeEvent;
    // If proposal gets rejected, bond goes to treasury
    type SpendPeriod = SpendPeriod;
    type Burn = ();
    type BurnDestination = ();
    type MaxApprovals = ConstU32<100>;
    type WeightInfo = weights::pallet_treasury::SubstrateWeight<Runtime>;
    type SpendFunds = ();
    type SpendOrigin =
        frame_system::EnsureWithSuccess<EnsureRoot<AccountId>, AccountId, MaxBalance>;
    type AssetKind = ();
    type Beneficiary = AccountId;
    type BeneficiaryLookup = IdentityLookup<AccountId>;
    type Paymaster = PayFromAccount<Balances, TreasuryAccount>;
    // TODO: implement pallet-asset-rate to allow the treasury to spend other assets
    type BalanceConverter = UnityAssetBalanceConversion;
    type PayoutPeriod = ConstU32<{ 30 * DAYS }>;
    type BlockNumberProvider = System;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = tanssi_runtime_common::benchmarking::TreasuryBenchmarkHelper<Runtime>;
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
    type BlockNumberProvider = System;
}
parameter_types! {
    pub const MaxInactiveSessions: u32 = 5;
}
impl pallet_inactivity_tracking::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxInactiveSessions = MaxInactiveSessions;
    type MaxCollatorsPerSession = MaxCandidatesBufferSize;
    type MaxContainerChains = MaxLengthParaIds;
    type CurrentSessionIndex = CurrentSessionIndexGetter;
    type CurrentCollatorsFetcher = CollatorAssignment;
    type GetSelfChainBlockAuthor = GetSelfChainBlockAuthor;
    type ParaFilter = tp_parathread_filter_common::ExcludeAllParathreadsFilter<Runtime>;
    type InvulnerablesFilter = tp_invulnerables_filter_common::InvulnerablesFilter<Runtime>;
    type CollatorStakeHelper = PooledStaking;
    type WeightInfo = weights::pallet_inactivity_tracking::SubstrateWeight<Runtime>;
}

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
        TransactionPayment: pallet_transaction_payment = 11,
        StreamPayment: pallet_stream_payment = 12,

        // Other utilities
        Identity: pallet_identity = 15,
        Multisig: pallet_multisig = 16,

        // ContainerChain management. It should go before Session for Genesis
        Registrar: pallet_registrar = 20,
        Configuration: pallet_configuration = 21,
        CollatorAssignment: pallet_collator_assignment = 22,
        Initializer: pallet_initializer = 23,
        AuthorNoting: pallet_author_noting = 24,
        AuthorityAssignment: pallet_authority_assignment = 25,
        ServicesPayment: pallet_services_payment = 26,
        DataPreservers: pallet_data_preservers = 27,

        // Collator support. The order of these 6 are important and shall not change.
        Invulnerables: pallet_invulnerables = 30,
        Session: pallet_session = 31,
        AuthorityMapping: pallet_authority_mapping = 32,
        AuthorInherent: pallet_author_inherent = 33,
        PooledStaking: pallet_pooled_staking = 34,
        // InflationRewards must be after Session and AuthorInherent
        InflationRewards: pallet_inflation_rewards = 35,
        InactivityTracking: pallet_inactivity_tracking = 36,

        // Treasury stuff.
        Treasury: pallet_treasury::{Pallet, Storage, Config<T>, Event<T>, Call} = 40,

        //XCM
        XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>} = 50,
        CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin} = 51,
        PolkadotXcm: pallet_xcm::{Pallet, Call, Storage, Event<T>, Origin, Config<T>} = 53,
        ForeignAssets: pallet_assets::<Instance1>::{Pallet, Call, Storage, Event<T>} = 54,
        ForeignAssetsCreator: pallet_foreign_asset_creator::{Pallet, Call, Storage, Event<T>} = 55,
        AssetRate: pallet_asset_rate::{Pallet, Call, Storage, Event<T>} = 56,
        MessageQueue: pallet_message_queue::{Pallet, Call, Storage, Event<T>} = 57,
        XcmCoreBuyer: pallet_xcm_core_buyer = 58,

        // More system support stuff
        RelayStorageRoots: pallet_relay_storage_roots = 60,
        WeightReclaim: cumulus_pallet_weight_reclaim = 61,

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
        [pallet_stream_payment, StreamPayment]
        [pallet_identity, Identity]
        [pallet_multiblock_migrations, MultiBlockMigrations]
        [pallet_multisig, Multisig]
        [pallet_registrar, Registrar]
        [pallet_configuration, Configuration]
        [pallet_collator_assignment, CollatorAssignment]
        [pallet_author_noting, AuthorNoting]
        [pallet_services_payment, ServicesPayment]
        [pallet_data_preservers, DataPreservers]
        [pallet_invulnerables, Invulnerables]
        [pallet_session, SessionBench::<Runtime>]
        [pallet_author_inherent, AuthorInherent]
        [pallet_pooled_staking, PooledStaking]
        [pallet_inactivity_tracking, InactivityTracking]
        [pallet_treasury, Treasury]
        [cumulus_pallet_xcmp_queue, XcmpQueue]
        // XCM
        [pallet_xcm, PalletXcmExtrinsicsBenchmark::<Runtime>]
        [pallet_xcm_benchmarks::fungible, pallet_xcm_benchmarks::fungible::Pallet::<Runtime>]
        [pallet_xcm_benchmarks::generic, pallet_xcm_benchmarks::generic::Pallet::<Runtime>]

        [pallet_assets, ForeignAssets]
        [pallet_foreign_asset_creator, ForeignAssetsCreator]
        [pallet_asset_rate, AssetRate]
        [pallet_message_queue, MessageQueue]
        [pallet_xcm_core_buyer, XcmCoreBuyer]
        [pallet_relay_storage_roots, RelayStorageRoots]
        [cumulus_pallet_weight_reclaim, WeightReclaim]
    );
}

pub fn get_para_id_authorities(para_id: ParaId) -> Option<Vec<NimbusId>> {
    let parent_number = System::block_number();

    let should_end_session =
        <Runtime as pallet_session::Config>::ShouldEndSession::should_end_session(
            parent_number + 1,
        );

    let session_index = if should_end_session {
        Session::current_index() + 1
    } else {
        Session::current_index()
    };

    let assigned_authorities = AuthorityAssignment::collator_container_chain(session_index)?;

    let self_para_id = ParachainInfo::get();

    if para_id == self_para_id {
        Some(assigned_authorities.orchestrator_chain)
    } else {
        assigned_authorities.container_chains.get(&para_id).cloned()
    }
}

impl_runtime_apis! {
    impl sp_consensus_aura::AuraApi<Block, NimbusId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(SLOT_DURATION)
        }

        fn authorities() -> Vec<NimbusId> {

            // Check whether we need to fetch the next authorities or current ones
            let parent_number = System::block_number();
            let should_end_session = <Runtime as pallet_session::Config>::ShouldEndSession::should_end_session(parent_number + 1);

            let session_index = if should_end_session {
                Session::current_index() +1
            }
            else {
                Session::current_index()
            };

            pallet_authority_assignment::CollatorContainerChain::<Runtime>::get(session_index)
                .expect("authorities for current session should exist")
                .orchestrator_chain
        }
    }

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
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
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
            use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
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
            use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
            impl cumulus_pallet_session_benchmarking::Config for Runtime {}

            impl frame_system_benchmarking::Config for Runtime {
                fn setup_set_code_requirements(code: &sp_std::vec::Vec<u8>) -> Result<(), BenchmarkError> {
                    ParachainSystem::initialize_for_set_code_benchmark(code.len() as u32);
                    Ok(())
                }

                fn verify_set_code() {
                    System::assert_last_event(cumulus_pallet_parachain_system::Event::<Runtime>::ValidationFunctionStored.into());
                }
            }

            use xcm::latest::prelude::*;
            use crate::xcm_config::SelfReserve;

            parameter_types! {
                pub ExistentialDepositAsset: Option<Asset> = Some((
                    SelfReserve::get(),
                    ExistentialDeposit::get()
                ).into());
                pub TrustedReserve: Option<(Location, Asset)> = Some(
                    (
                        Location::parent(),
                        Asset {
                            id: AssetId(Location::parent()),
                            fun: Fungible(ExistentialDeposit::get() * 100),
                        },
                    )
                );
            }

            impl pallet_xcm_benchmarks::fungible::Config for Runtime {
                type TransactAsset = Balances;
                type CheckedAccount = ();
                type TrustedTeleporter = ();
                type TrustedReserve = TrustedReserve;

                fn get_asset() -> Asset {
                    Asset {
                        id: AssetId(SelfReserve::get()),
                        fun: Fungible(ExistentialDeposit::get() * 100),
                    }
                }
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
                        fun: Fungible(ExistentialDeposit::get()*100),
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
                        fun: Fungible(ExistentialDeposit::get()),
                    }
                }

                fn reachable_dest() -> Option<Location> {
                    Some(Parent.into())
                }

                fn teleportable_asset_and_dest() -> Option<(Asset, Location)> {
                    // Relay/native token can be teleported between AH and Relay.
                    Some((
                        Asset {
                            fun: Fungible(EXISTENTIAL_DEPOSIT),
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
                    let balance = EXISTENTIAL_DEPOSIT * 1000;
                    let _ = <Balances as frame_support::traits::Currency<_>>::make_free_balance_be(
                        &who, balance,
                    );
                    Some((
                        Asset {
                            fun: Fungible(EXISTENTIAL_DEPOSIT),
                            id: AssetId(SelfReserve::get())
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

                    let fee_amount = EXISTENTIAL_DEPOSIT;
                    let fee_asset: Asset = (SelfReserve::get(), fee_amount).into();

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

                    // inject it into pallet-foreign-asset-creator.
                    let (asset_id, asset_location) = pallet_foreign_asset_creator::benchmarks::create_default_minted_asset::<Runtime>(
                        initial_asset_amount,
                        who.clone()
                    );
                    let transfer_asset: Asset = (asset_location, asset_amount).into();

                    let assets: Assets = vec![fee_asset.clone(), transfer_asset].into();
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

    impl pallet_collator_assignment_runtime_api::CollatorAssignmentApi<Block, AccountId, ParaId> for Runtime {
        /// Return the parachain that the given `AccountId` is collating for.
        /// Returns `None` if the `AccountId` is not collating.
        fn current_collator_parachain_assignment(account: AccountId) -> Option<ParaId> {
            let assigned_collators = CollatorAssignment::collator_container_chain();
            let self_para_id = ParachainInfo::get();

            assigned_collators.para_id_of(&account, self_para_id)
        }

        /// Return the parachain that the given `AccountId` will be collating for
        /// in the next session change.
        /// Returns `None` if the `AccountId` will not be collating.
        fn future_collator_parachain_assignment(account: AccountId) -> Option<ParaId> {
            let assigned_collators = CollatorAssignment::pending_collator_container_chain();

            match assigned_collators {
                Some(assigned_collators) => {
                    let self_para_id = ParachainInfo::get();

                    assigned_collators.para_id_of(&account, self_para_id)
                }
                None => {
                    Self::current_collator_parachain_assignment(account)
                }
            }

        }

        /// Return the list of collators of the given `ParaId`.
        /// Returns `None` if the `ParaId` is not in the registrar.
        fn parachain_collators(para_id: ParaId) -> Option<Vec<AccountId>> {
            let assigned_collators = CollatorAssignment::collator_container_chain();
            let self_para_id = ParachainInfo::get();

            if para_id == self_para_id {
                Some(assigned_collators.orchestrator_chain)
            } else {
                assigned_collators.container_chains.get(&para_id).cloned()
            }
        }

        /// Returns the list of `ParaId` of registered chains with at least some
        /// collators. This filters out parachains with no assigned collators.
        /// Since runtime APIs are called on top of a parent block, we need to be carefull
        /// at session boundaries. If the next block will change session, this function returns
        /// the parachains relevant for the next session.
        fn parachains_with_some_collators() -> Vec<ParaId> {
            use tp_traits::{GetContainerChainsWithCollators, ForSession};

            // We should return the container-chains for the session in which we are kicking in
            let parent_number = System::block_number();
            let should_end_session = <Runtime as pallet_session::Config>::ShouldEndSession::should_end_session(parent_number + 1);
            let for_session = if should_end_session { ForSession::Next } else { ForSession::Current };

            CollatorAssignment::container_chains_with_collators(for_session)
                .into_iter()
                .filter_map(
                    |(para_id, collators)| (!collators.is_empty()).then_some(para_id)
                ).collect()
        }
    }

    impl pallet_registrar_runtime_api::RegistrarApi<Block, ParaId> for Runtime {
        /// Return the registered para ids
        fn registered_paras() -> Vec<ParaId> {
            // We should return the container-chains for the session in which we are kicking in
            let parent_number = System::block_number();
            let should_end_session = <Runtime as pallet_session::Config>::ShouldEndSession::should_end_session(parent_number + 1);

            let session_index = if should_end_session {
                Session::current_index() +1
            }
            else {
                Session::current_index()
            };

            let container_chains = Registrar::session_container_chains(session_index);
            let mut para_ids = vec![];
            para_ids.extend(container_chains.parachains);
            para_ids.extend(container_chains.parathreads.into_iter().map(|(para_id, _)| para_id));

            para_ids
        }

        /// Fetch genesis data for this para id
        fn genesis_data(para_id: ParaId) -> Option<ContainerChainGenesisData> {
            Registrar::para_genesis_data(para_id)
        }

        /// Fetch boot_nodes for this para id
        fn boot_nodes(para_id: ParaId) -> Vec<Vec<u8>> {
            DataPreservers::assignments_profiles(para_id)
                .filter(|profile| profile.mode == pallet_data_preservers::ProfileMode::Bootnode)
                .map(|profile| profile.url.into())
                .collect()
        }
    }

    impl pallet_registrar_runtime_api::OnDemandBlockProductionApi<Block, ParaId, Slot> for Runtime {
        /// Returns slot frequency for particular para thread. Slot frequency specifies amount of slot
        /// need to be passed between two parathread blocks. It is expressed as `(min, max)` pair where `min`
        /// indicates amount of slot must pass before we produce another block and `max` indicates amount of
        /// blocks before this parathread must produce the block.
        ///
        /// Simply put, parathread must produce a block after `min`  but before `(min+max)` slots.
        ///
        /// # Returns
        ///
        /// * `Some(slot_frequency)`.
        /// * `None` if the `para_id` is not a parathread.
        fn parathread_slot_frequency(para_id: ParaId) -> Option<SlotFrequency> {
            Registrar::parathread_params(para_id).map(|params| {
                params.slot_frequency
            })
        }
    }

    impl pallet_author_noting_runtime_api::AuthorNotingApi<Block, AccountId, BlockNumber, ParaId> for Runtime
        where
        AccountId: parity_scale_codec::Codec,
        BlockNumber: parity_scale_codec::Codec,
        ParaId: parity_scale_codec::Codec,
    {
        fn latest_block_number(para_id: ParaId) -> Option<BlockNumber> {
            AuthorNoting::latest_author(para_id).map(|info| info.block_number)
        }

        fn latest_author(para_id: ParaId) -> Option<AccountId> {
            AuthorNoting::latest_author(para_id).map(|info| info.author)
        }
    }

    impl dp_consensus::TanssiAuthorityAssignmentApi<Block, NimbusId> for Runtime {
        /// Return the current authorities assigned to a given paraId
        fn para_id_authorities(para_id: ParaId) -> Option<Vec<NimbusId>> {
            get_para_id_authorities(para_id)
        }

        /// Return the paraId assigned to a given authority
        fn check_para_id_assignment(authority: NimbusId) -> Option<ParaId> {
            let parent_number = System::block_number();
            let should_end_session = <Runtime as pallet_session::Config>::ShouldEndSession::should_end_session(parent_number + 1);

            let session_index = if should_end_session {
                Session::current_index() +1
            }
            else {
                Session::current_index()
            };
            let assigned_authorities = AuthorityAssignment::collator_container_chain(session_index)?;
            let self_para_id = ParachainInfo::get();

            assigned_authorities.para_id_of(&authority, self_para_id)
        }

        /// Return the paraId assigned to a given authority on the next session.
        /// On session boundary this returns the same as `check_para_id_assignment`.
        fn check_para_id_assignment_next_session(authority: NimbusId) -> Option<ParaId> {
            let session_index = Session::current_index() + 1;
            let assigned_authorities = AuthorityAssignment::collator_container_chain(session_index)?;
            let self_para_id = ParachainInfo::get();

            assigned_authorities.para_id_of(&authority, self_para_id)
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

    impl pallet_stream_payment_runtime_api::StreamPaymentApi<Block, StreamId, Balance, Balance>
    for Runtime {
        fn stream_payment_status(
            stream_id: StreamId,
            now: Option<Balance>,
        ) -> Result<StreamPaymentApiStatus<Balance>, StreamPaymentApiError> {
            match StreamPayment::stream_payment_status(stream_id, now) {
                Ok(pallet_stream_payment::StreamPaymentStatus {
                    payment, deposit_left, stalled
                }) => Ok(StreamPaymentApiStatus {
                    payment, deposit_left, stalled
                }),
                Err(pallet_stream_payment::Error::<Runtime>::UnknownStreamId)
                => Err(StreamPaymentApiError::UnknownStreamId),
                Err(e) => Err(StreamPaymentApiError::Other(format!("{e:?}")))
            }
        }
    }

    impl pallet_data_preservers_runtime_api::DataPreserversApi<Block, DataPreserversProfileId, ParaId> for Runtime {
        fn get_active_assignment(
            profile_id: DataPreserversProfileId,
        ) -> pallet_data_preservers_runtime_api::Assignment<ParaId> {
            use pallet_data_preservers_runtime_api::Assignment;
            use pallet_stream_payment::StreamPaymentStatus;

            let Some((para_id, witness)) = pallet_data_preservers::Profiles::<Runtime>::get(profile_id)
                .and_then(|x| x.assignment) else
            {
                return Assignment::NotAssigned;
            };

            match witness {
                tp_data_preservers_common::AssignmentWitness::Free => Assignment::Active(para_id),
                tp_data_preservers_common::AssignmentWitness::StreamPayment { stream_id } => {
                    // Error means no Stream exists with that ID or some issue occured when computing
                    // the status. In that case we cannot consider the assignment as active.
                    let Ok(StreamPaymentStatus { stalled, .. }) = StreamPayment::stream_payment_status( stream_id, None) else {
                        return Assignment::Inactive(para_id);
                    };

                    if stalled {
                        Assignment::Inactive(para_id)
                    } else {
                        Assignment::Active(para_id)
                    }
                },
            }
        }
    }

    impl dp_slot_duration_runtime_api::TanssiSlotDurationApi<Block> for Runtime {
        fn slot_duration() -> u64 {
            SLOT_DURATION
        }
    }

    impl pallet_services_payment_runtime_api::ServicesPaymentApi<Block, Balance, ParaId> for Runtime {
        fn block_cost(para_id: ParaId) -> Balance {
            let (block_production_costs, _) = <Runtime as pallet_services_payment::Config>::ProvideBlockProductionCost::block_cost(&para_id);
            block_production_costs
        }

        fn collator_assignment_cost(para_id: ParaId) -> Balance {
            let (collator_assignment_costs, _) = <Runtime as pallet_services_payment::Config>::ProvideCollatorAssignmentCost::collator_assignment_cost(&para_id);
            collator_assignment_costs
        }
    }

    impl pallet_xcm_core_buyer_runtime_api::XCMCoreBuyerApi<Block, BlockNumber, ParaId, NimbusId> for Runtime {
        fn is_core_buying_allowed(para_id: ParaId, collator_public_key: NimbusId) -> Result<(), BuyingError<BlockNumber>> {
            XcmCoreBuyer::is_core_buying_allowed(para_id, Some(collator_public_key))
        }

        fn create_buy_core_unsigned_extrinsic(para_id: ParaId, proof: BuyCoreCollatorProof<NimbusId>) -> Box<<Block as BlockT>::Extrinsic> {
            let call = RuntimeCall::XcmCoreBuyer(pallet_xcm_core_buyer::Call::buy_core {
                para_id,
                proof
            });

            let unsigned_extrinsic = UncheckedExtrinsic::new_bare(call);

            Box::new(unsigned_extrinsic)
        }

        fn get_buy_core_signature_nonce(para_id: ParaId) -> u64 {
            pallet_xcm_core_buyer::CollatorSignatureNonce::<Runtime>::get(para_id)
        }

        fn get_buy_core_slot_drift() -> Slot {
            <Runtime as pallet_xcm_core_buyer::Config>::BuyCoreSlotDrift::get()
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

#[macro_export]
macro_rules! prod_or_fast {
    ($prod:expr, $test:expr) => {
        if cfg!(feature = "fast-runtime") {
            $test
        } else {
            $prod
        }
    };
    ($prod:expr, $test:expr, $env:expr) => {
        if cfg!(feature = "fast-runtime") {
            core::option_env!($env)
                .map(|s| s.parse().ok())
                .flatten()
                .unwrap_or($test)
        } else {
            $prod
        }
    };
}
