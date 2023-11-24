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

pub mod xcm_config;

#[cfg(feature = "std")]
use sp_version::NativeVersion;

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

pub mod migrations;
pub mod weights;

#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;

use {
    cumulus_pallet_parachain_system::{RelayChainStateProof, RelayNumberStrictlyIncreases},
    cumulus_primitives_core::{
        relay_chain::{self, BlockNumber as RelayBlockNumber, SessionIndex},
        BodyId, DmpMessageHandler, ParaId,
    },
    frame_support::{
        construct_runtime,
        dispatch::DispatchClass,
        pallet_prelude::DispatchResult,
        parameter_types,
        traits::{
            fungible::{Balanced, Credit},
            ConstU128, ConstU32, ConstU64, ConstU8, Contains, InstanceFilter, OffchainWorker,
            OnFinalize, OnIdle, OnInitialize, OnRuntimeUpgrade, ValidatorRegistration,
        },
        weights::{
            constants::{
                BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight,
                WEIGHT_REF_TIME_PER_SECOND,
            },
            ConstantMultiplier, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients,
            WeightToFeePolynomial,
        },
        PalletId,
    },
    frame_system::{
        limits::{BlockLength, BlockWeights},
        EnsureRoot,
    },
    nimbus_primitives::NimbusId,
    pallet_collator_assignment::{GetRandomnessForNextBlock, RotateCollatorsEveryNSessions},
    pallet_invulnerables::InvulnerableRewardDistribution,
    pallet_pooled_staking::traits::{IsCandidateEligible, Timer},
    pallet_registrar_runtime_api::ContainerChainGenesisData,
    pallet_services_payment::{ChargeForBlockCredit, ProvideBlockProductionCost},
    pallet_session::{SessionManager, ShouldEndSession},
    pallet_transaction_payment::{ConstFeeMultiplier, CurrencyAdapter, Multiplier},
    polkadot_runtime_common::BlockHashCount,
    scale_info::TypeInfo,
    smallvec::smallvec,
    sp_api::impl_runtime_apis,
    sp_core::{crypto::KeyTypeId, Decode, Encode, Get, MaxEncodedLen, OpaqueMetadata},
    sp_runtime::{
        create_runtime_str, generic, impl_opaque_keys,
        traits::{
            AccountIdConversion, AccountIdLookup, BlakeTwo256, Block as BlockT, Hash as HashT,
        },
        transaction_validity::{TransactionSource, TransactionValidity},
        AccountId32, ApplyExtrinsicResult,
    },
    sp_std::{marker::PhantomData, prelude::*},
    sp_version::RuntimeVersion,
    tp_traits::{GetSessionContainerChains, RemoveInvulnerables, RemoveParaIdsWithNoCredits},
};
pub use {
    dp_core::{AccountId, Address, Balance, BlockNumber, Hash, Header, Index, Signature},
    sp_runtime::{MultiAddress, Perbill, Permill},
};

const LOG_TARGET: &str = "runtime::moonbeam";

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
    pallet_maintenance_mode::ExecutiveHooks<Runtime>,
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

    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * 100 * MILLIDANCE * SUPPLY_FACTOR + (bytes as Balance) * STORAGE_BYTE_FEE
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
    pub struct SessionKeys {
        pub nimbus: Initializer,
    }
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("dancebox"),
    impl_name: create_runtime_str!("dancebox"),
    authoring_version: 1,
    spec_version: 400,
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
pub const MILLISECS_PER_BLOCK: u64 = 12000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000;
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
    type BaseCallFilter = MaintenanceMode;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = RuntimeBlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = RuntimeBlockLength;
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    /// The action to take on a Runtime Upgrade
    type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = tp_consensus::OnTimestampSet<
        <Self as pallet_author_inherent::Config>::SlotBeacon,
        ConstU64<{ SLOT_DURATION }>,
    >;
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    type WeightInfo = pallet_timestamp::weights::SubstrateWeight<Runtime>;
}

pub struct CanAuthor;
impl nimbus_primitives::CanAuthor<NimbusId> for CanAuthor {
    fn can_author(author: &NimbusId, slot: &u32) -> bool {
        let authorities = AuthorityAssignment::collator_container_chain(Session::current_index())
            .expect("authorities should be set")
            .orchestrator_chain;

        if authorities.len() == 0 {
            return false;
        }

        let author_index = (*slot as usize) % authorities.len();
        let expected_author = &authorities[author_index];

        expected_author == author
    }
}

impl pallet_author_inherent::Config for Runtime {
    type AuthorId = NimbusId;
    type AccountLookup = tp_consensus::NimbusLookUp;
    type CanAuthor = CanAuthor;
    type SlotBeacon = tp_consensus::AuraDigestSlotBeacon<Runtime>;
    type WeightInfo = pallet_author_inherent::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
}

/// A reason for placing a hold on funds.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, MaxEncodedLen, Debug, TypeInfo,
)]
pub enum HoldReason {
    /// The Pooled Stake holds
    PooledStake,
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
    type FreezeIdentifier = [u8; 8];
    type MaxFreezes = ConstU32<0>;
    type RuntimeHoldReason = HoldReason;
    type MaxHolds = ConstU32<1>;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const TransactionByteFee: Balance = 1;
    pub const FeeMultiplier: Multiplier = Multiplier::from_u32(1);
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    // This will burn the fees
    type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = WeightToFee;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
}

parameter_types! {
    pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
    pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
}

impl cumulus_pallet_parachain_system::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnSystemEvent = ();
    type SelfParaId = parachain_info::Pallet<Runtime>;
    type OutboundXcmpMessageSource = XcmpQueue;
    type DmpMessageHandler = MaintenanceMode;
    type ReservedDmpWeight = ReservedDmpWeight;
    type XcmpMessageHandler = XcmpQueue;
    type ReservedXcmpWeight = ReservedXcmpWeight;
    type CheckAssociatedRelayNumber = RelayNumberStrictlyIncreases;
}

/// Only callable after `set_validation_data` is called which forms this proof the same way
fn relay_chain_state_proof() -> RelayChainStateProof {
    let relay_storage_root = ParachainSystem::validation_data()
        .expect("set in `set_validation_data`")
        .relay_parent_storage_root;
    let relay_chain_state =
        ParachainSystem::relay_state_proof().expect("set in `set_validation_data`");
    RelayChainStateProof::new(ParachainInfo::get(), relay_storage_root, relay_chain_state)
        .expect("Invalid relay chain state proof, already constructed in `set_validation_data`")
}

pub struct BabeCurrentBlockRandomnessGetter;
impl BabeCurrentBlockRandomnessGetter {
    fn get_block_randomness() -> Option<Hash> {
        if cfg!(feature = "runtime-benchmarks") {
            // storage reads as per actual reads
            let _relay_storage_root = ParachainSystem::validation_data();
            let _relay_chain_state = ParachainSystem::relay_state_proof();
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
    // TODO: audit usage of randomness API
    // https://github.com/paritytech/polkadot/issues/2601
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
impl SessionManager<AccountId> for CollatorsFromInvulnerablesAndThenFromStaking {
    fn new_session(index: SessionIndex) -> Option<Vec<AccountId>> {
        log::info!(
            "assembling new collators for new session {} at #{:?}",
            index,
            <frame_system::Pallet<Runtime>>::block_number(),
        );

        let invulnerables = Invulnerables::invulnerables().to_vec();
        let candidates_staking =
            pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
        // Max number of collators is set in pallet_configuration
        let max_collators = Configuration::config().max_collators;
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
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    // we don't have stash and controller, thus we don't need the convert as well.
    type ValidatorIdOf = pallet_invulnerables::IdentityCollator;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionManager = CollatorsFromInvulnerablesAndThenFromStaking;
    // Essentially just Aura, but let's be pedantic.
    type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
    type Keys = SessionKeys;
    type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
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
                // TODO: smoke test to ensure this never happens in a live network
                [0; 32]
            }
        } else {
            // In block 0 (genesis) there is randomness
            [0; 32]
        };

        random_seed
    }
}

pub struct RemoveInvulnerablesImpl;

impl RemoveInvulnerables<AccountId> for RemoveInvulnerablesImpl {
    fn remove_invulnerables(
        collators: &mut Vec<AccountId>,
        num_invulnerables: usize,
    ) -> Vec<AccountId> {
        if num_invulnerables == 0 {
            return vec![];
        }
        // TODO: check if this works on session changes
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

pub struct RemoveParaIdsWithNoCreditsImpl;

impl RemoveParaIdsWithNoCredits for RemoveParaIdsWithNoCreditsImpl {
    fn remove_para_ids_with_no_credits(para_ids: &mut Vec<ParaId>) {
        let blocks_per_session = Period::get();
        let credits_for_2_sessions = 2 * blocks_per_session;
        para_ids.retain(|para_id| {
            // Check if the container chain has enough credits for producing blocks for 2 sessions
            let credits = pallet_services_payment::BlockProductionCredits::<Runtime>::get(para_id)
                .unwrap_or_default();

            credits >= credits_for_2_sessions
        });
    }

    /// Make those para ids valid by giving them enough credits, for benchmarking.
    #[cfg(feature = "runtime-benchmarks")]
    fn make_valid_para_ids(para_ids: &[ParaId]) {
        use frame_support::assert_ok;

        let blocks_per_session = Period::get();
        // Enough credits to run any benchmark
        let credits = 20 * blocks_per_session;

        for para_id in para_ids {
            assert_ok!(ServicesPayment::set_credits(
                RuntimeOrigin::root(),
                *para_id,
                credits,
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
    type GetRandomnessForNextBlock = BabeGetRandomnessForNextBlock;
    type RemoveInvulnerables = RemoveInvulnerablesImpl;
    type RemoveParaIdsWithNoCredits = RemoveParaIdsWithNoCreditsImpl;
    type WeightInfo = pallet_collator_assignment::weights::SubstrateWeight<Runtime>;
}

impl pallet_authority_assignment::Config for Runtime {
    type SessionIndex = u32;
    type AuthorityId = NimbusId;
}

pub const FIXED_BLOCK_PRODUCTION_COST: u128 = 1 * currency::MICRODANCE;

pub struct BlockProductionCost<Runtime>(PhantomData<Runtime>);
impl ProvideBlockProductionCost<Runtime> for BlockProductionCost<Runtime> {
    fn block_cost(_para_id: &ParaId) -> (u128, Weight) {
        (FIXED_BLOCK_PRODUCTION_COST, Weight::zero())
    }
}

parameter_types! {
    // 60 days worth of blocks
    pub const MaxCreditsStored: BlockNumber = 60 * DAYS;
}

impl pallet_services_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    /// Handler for fees
    type OnChargeForBlockCredit = ChargeForBlockCredit<Runtime>;
    /// Currency type for fee payment
    type Currency = Balances;
    /// Provider of a block cost which can adjust from block to block
    type ProvideBlockProductionCost = BlockProductionCost<Runtime>;
    /// The maximum number of credits that can be accumulated
    type MaxCreditsStored = MaxCreditsStored;
    type WeightInfo = pallet_services_payment::weights::SubstrateWeight<Runtime>;
}

impl pallet_author_noting::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ContainerChains = Registrar;
    type SelfParaId = parachain_info::Pallet<Runtime>;
    type ContainerChainAuthor = CollatorAssignment;
    type RelayChainStateProvider = cumulus_pallet_parachain_system::RelaychainDataProvider<Self>;
    type AuthorNotingHook = (InflationRewards, ServicesPayment);
    type WeightInfo = pallet_author_noting::weights::SubstrateWeight<Runtime>;
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
    type CollatorIdOf = pallet_invulnerables::IdentityCollator;
    type CollatorRegistration = Session;
    type WeightInfo = pallet_invulnerables::weights::SubstrateWeight<Runtime>;
    #[cfg(feature = "runtime-benchmarks")]
    type Currency = Balances;
}

parameter_types! {
    pub const MaxLengthParaIds: u32 = 100u32;
    pub const MaxEncodedGenesisDataSize: u32 = 5_000_000u32; // 5MB
    pub const MaxBootNodes: u32 = 10;
    pub const MaxBootNodeUrlLen: u32 = 200;
}

pub struct CurrentSessionIndexGetter;

impl tp_traits::GetSessionIndex<u32> for CurrentSessionIndexGetter {
    /// Returns current session index.
    fn session_index() -> u32 {
        Session::current_index()
    }
}

impl pallet_configuration::Config for Runtime {
    type SessionDelay = ConstU32<2>;
    type SessionIndex = u32;
    type CurrentSessionIndex = CurrentSessionIndexGetter;
    type AuthorityId = NimbusId;
    type WeightInfo = pallet_configuration::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const DepositAmount: Balance = 100 * UNIT;
    pub const MaxLengthTokenSymbol: u32 = 255;
}
impl pallet_registrar::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RegistrarOrigin = EnsureRoot<AccountId>;
    type MaxLengthParaIds = MaxLengthParaIds;
    type MaxGenesisDataSize = MaxEncodedGenesisDataSize;
    type MaxBootNodes = MaxBootNodes;
    type MaxBootNodeUrlLen = MaxBootNodeUrlLen;
    type MaxLengthTokenSymbol = MaxLengthTokenSymbol;
    type SessionDelay = ConstU32<2>;
    type SessionIndex = u32;
    type CurrentSessionIndex = CurrentSessionIndexGetter;
    type Currency = Balances;
    type DepositAmount = DepositAmount;
    type WeightInfo = pallet_registrar::weights::SubstrateWeight<Runtime>;
}

impl pallet_authority_mapping::Config for Runtime {
    type SessionIndex = u32;
    type SessionRemovalBoundary = ConstU32<2>;
    type AuthorityId = NimbusId;
}

impl pallet_sudo::Config for Runtime {
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_sudo::weights::SubstrateWeight<Runtime>;
}

impl pallet_utility::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
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
    /// Only extrinsics related to staking.
    Staking = 3,
    /// Allow to veto an announced proxy call.
    CancelProxy = 4,
    /// Allow extrinsic related to Balances.
    Balances = 5,
}

impl Default for ProxyType {
    fn default() -> Self {
        Self::Any
    }
}

impl InstanceFilter<RuntimeCall> for ProxyType {
    fn filter(&self, c: &RuntimeCall) -> bool {
        match self {
            ProxyType::Any => true,
            ProxyType::NonTransfer => {
                matches!(
                    c,
                    RuntimeCall::System(..)
                        | RuntimeCall::ParachainSystem(..)
                        | RuntimeCall::Timestamp(..)
                        | RuntimeCall::Utility(..)
                        | RuntimeCall::Proxy(..)
                        | RuntimeCall::Registrar(..)
                )
            }
            ProxyType::Governance => matches!(c, RuntimeCall::Utility(..)),
            ProxyType::Staking => matches!(
                c,
                RuntimeCall::Session(..)
                    | RuntimeCall::Utility(..)
                    | RuntimeCall::PooledStaking(..)
            ),
            ProxyType::CancelProxy => matches!(
                c,
                RuntimeCall::Proxy(pallet_proxy::Call::reject_announcement { .. })
            ),
            ProxyType::Balances => {
                matches!(c, RuntimeCall::Balances(..) | RuntimeCall::Utility(..))
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
    type WeightInfo = pallet_proxy::weights::SubstrateWeight<Runtime>;
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
    type MigrationsList = (migrations::DanceboxMigrations<Runtime>,);
    type XcmExecutionManager = XcmExecutionManager;
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
/// We dont allow to create nor mint assets, this for now is disabled
/// We only allow transfers. For now creation of assets will go through
/// asset-manager, while minting/burning only happens through xcm messages
/// This can change in the future
pub struct NormalFilter;
impl Contains<RuntimeCall> for NormalFilter {
    fn contains(c: &RuntimeCall) -> bool {
        match c {
            // We filter anonymous proxy as they make "reserve" inconsistent
            // See: https://github.com/paritytech/substrate/blob/37cca710eed3dadd4ed5364c7686608f5175cce1/frame/proxy/src/lib.rs#L270 // editorconfig-checker-disable-line
            RuntimeCall::Proxy(method) => match method {
                pallet_proxy::Call::create_pure { .. } => false,
                pallet_proxy::Call::kill_pure { .. } => false,
                _ => true,
            },
            _ => true,
        }
    }
}

pub struct NormalDmpHandler;
impl DmpMessageHandler for NormalDmpHandler {
    // This implementation makes messages be queued
    // Since the limit is 0, messages are queued for next iteration
    fn handle_dmp_messages(
        iter: impl Iterator<Item = (RelayBlockNumber, Vec<u8>)>,
        limit: Weight,
    ) -> Weight {
        (if Migrations::should_pause_xcm() {
            DmpQueue::handle_dmp_messages(iter, Weight::zero())
        } else {
            DmpQueue::handle_dmp_messages(iter, limit)
        }) + <Runtime as frame_system::Config>::DbWeight::get().reads(1)
    }
}

pub struct MaintenanceDmpHandler;
impl DmpMessageHandler for MaintenanceDmpHandler {
    // This implementation makes messages be queued
    // Since the limit is 0, messages are queued for next iteration
    fn handle_dmp_messages(
        iter: impl Iterator<Item = (RelayBlockNumber, Vec<u8>)>,
        _limit: Weight,
    ) -> Weight {
        DmpQueue::handle_dmp_messages(iter, Weight::zero())
    }
}

/// The hooks we want to run in Maintenance Mode
pub struct MaintenanceHooks;

impl OnInitialize<BlockNumber> for MaintenanceHooks {
    fn on_initialize(n: BlockNumber) -> Weight {
        AllPalletsWithSystem::on_initialize(n)
    }
}

// We override onIdle for xcmQueue and dmpQueue pallets to not process messages inside it
impl OnIdle<BlockNumber> for MaintenanceHooks {
    fn on_idle(_n: BlockNumber, _max_weight: Weight) -> Weight {
        Weight::zero()
    }
}

impl OnRuntimeUpgrade for MaintenanceHooks {
    fn on_runtime_upgrade() -> Weight {
        AllPalletsWithSystem::on_runtime_upgrade()
    }
    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::DispatchError> {
        AllPalletsWithSystem::pre_upgrade()
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        AllPalletsWithSystem::post_upgrade(state)
    }

    #[cfg(feature = "try-runtime")]
    fn try_on_runtime_upgrade(checks: bool) -> Result<Weight, TryRuntimeError> {
        AllPalletsWithSystem::try_on_runtime_upgrade(checks)
    }
}

impl OnFinalize<BlockNumber> for MaintenanceHooks {
    fn on_finalize(n: BlockNumber) {
        AllPalletsWithSystem::on_finalize(n)
    }
}

impl OffchainWorker<BlockNumber> for MaintenanceHooks {
    fn offchain_worker(n: BlockNumber) {
        AllPalletsWithSystem::offchain_worker(n)
    }
}

impl pallet_maintenance_mode::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type NormalCallFilter = NormalFilter;
    type MaintenanceCallFilter = MaintenanceFilter;
    type MaintenanceOrigin = EnsureRoot<AccountId>;
    type XcmExecutionManager = XcmExecutionManager;
    type NormalDmpHandler = NormalDmpHandler;
    type MaintenanceDmpHandler = MaintenanceDmpHandler;
    // We use AllPalletsWithSystem because we dont want to change the hooks in normal
    // operation
    type NormalExecutiveHooks = AllPalletsWithSystem;
    type MaintenanceExecutiveHooks = MaintenanceHooks;
}

impl pallet_root_testing::Config for Runtime {}

parameter_types! {
    pub StakingAccount: AccountId32 = PalletId(*b"POOLSTAK").into_account_truncating();
    pub const CurrencyHoldReason: HoldReason = HoldReason::PooledStake;
    pub const InitialManualClaimShareValue: u128 = currency::MILLIDANCE;
    pub const InitialAutoCompoundingShareValue: u128 = currency::MILLIDANCE;
    pub const MinimumSelfDelegation: u128 = 10 * currency::KILODANCE;
    pub const RewardsCollatorCommission: Perbill = Perbill::from_percent(20);
    // Need to wait 2 sessions before being able to join or leave staking pools
    pub const StakingSessionDelay: u32 = 2;
}

pub struct SessionTimer<G>(PhantomData<G>);

impl<G> Timer for SessionTimer<G>
where
    G: Get<u32>,
{
    type Instant = u32;

    fn now() -> Self::Instant {
        Session::current_index()
    }

    fn is_elapsed(instant: &Self::Instant) -> bool {
        let delay = G::get();
        let Some(end) = instant.checked_add(delay) else {
            return false;
        };
        end <= Self::now()
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn elapsed_instant() -> Self::Instant {
        let delay = G::get();
        Self::now()
            .checked_add(delay)
            .expect("overflow when computing valid elapsed instant")
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn skip_to_elapsed() {
        let session_to_reach = Self::elapsed_instant();
        while Self::now() < session_to_reach {
            Session::rotate_session();
        }
    }
}

pub struct CandidateHasRegisteredKeys;
impl IsCandidateEligible<AccountId> for CandidateHasRegisteredKeys {
    fn is_candidate_eligible(a: &AccountId) -> bool {
        <Session as ValidatorRegistration<AccountId>>::is_registered(a)
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
    }
}

impl pallet_pooled_staking::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Balance = Balance;
    type CurrencyHoldReason = CurrencyHoldReason;
    type StakingAccount = StakingAccount;
    type InitialManualClaimShareValue = InitialManualClaimShareValue;
    type InitialAutoCompoundingShareValue = InitialAutoCompoundingShareValue;
    type MinimumSelfDelegation = MinimumSelfDelegation;
    type RewardsCollatorCommission = RewardsCollatorCommission;
    type JoiningRequestTimer = SessionTimer<StakingSessionDelay>;
    type LeavingRequestTimer = SessionTimer<StakingSessionDelay>;
    type EligibleCandidatesBufferSize = ConstU32<100>;
    type EligibleCandidatesFilter = CandidateHasRegisteredKeys;
    type WeightInfo = pallet_pooled_staking::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub ParachainBondAccount: AccountId32 = PalletId(*b"ParaBond").into_account_truncating();
    pub PendingRewardsAccount: AccountId32 = PalletId(*b"PENDREWD").into_account_truncating();
    // The equation to solve is:
    // initial_supply * (1.05) = initial_supply * (1+x)^2_629_800
    // we should solve for x = (1.05)^(1/2_629_800) -1 -> 0.000000019 per block or 19/1_000_000_000
    // 1% in the case of dev moed
    // TODO: check if we can put the prod inflation for tests too
    // TODO: better calculus for going from annual to block inflation (if it can be done)
    pub const InflationRate: Perbill = prod_or_fast!(Perbill::from_parts(19), Perbill::from_percent(1));

    // 30% for parachain bond, so 70% for staking
    pub const RewardsPortion: Perbill = Perbill::from_percent(70);
}

use {nimbus_primitives::SlotBeacon, tp_traits::GetContainerChainAuthor};

pub struct GetSelfChainBlockAuthor;
impl Get<AccountId32> for GetSelfChainBlockAuthor {
    fn get() -> AccountId32 {
        // TODO: we should do a refactor here, and use either authority-mapping or collator-assignemnt
        // we should also make sure we actually account for the weight of these
        // although most of these should be cached as they are read every block
        let slot = <Runtime as pallet_author_inherent::Config>::SlotBeacon::slot() as u64;
        let self_para_id = ParachainInfo::get();
        let author = CollatorAssignment::author_for_slot(slot.into(), self_para_id);
        author.expect("author should be set")
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

        // Monetary stuff.
        Balances: pallet_balances = 10,
        TransactionPayment: pallet_transaction_payment = 11,

        // ContainerChain management. It should go before Session for Genesis
        Registrar: pallet_registrar = 20,
        Configuration: pallet_configuration = 21,
        CollatorAssignment: pallet_collator_assignment = 22,
        Initializer: pallet_initializer = 23,
        AuthorNoting: pallet_author_noting = 24,
        AuthorityAssignment: pallet_authority_assignment = 25,
        ServicesPayment: pallet_services_payment = 26,

        // Collator support. The order of these 6 are important and shall not change.
        Invulnerables: pallet_invulnerables = 30,
        Session: pallet_session = 31,
        AuthorityMapping: pallet_authority_mapping = 32,
        AuthorInherent: pallet_author_inherent = 33,
        PooledStaking: pallet_pooled_staking = 34,
        // InflationRewards must be after Session and AuthorInherent
        InflationRewards: pallet_inflation_rewards = 35,

        //XCM
        XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>} = 50,
        CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin} = 51,
        DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>} = 52,
        PolkadotXcm: pallet_xcm::{Pallet, Call, Storage, Event<T>, Origin, Config<T>} = 53,

        RootTesting: pallet_root_testing = 100,
    }
);

#[cfg(feature = "runtime-benchmarks")]
mod benches {
    frame_benchmarking::define_benchmarks!(
        [frame_system, frame_system_benchmarking::Pallet::<Runtime>]
        [pallet_author_noting, AuthorNoting]
        [pallet_collator_assignment, CollatorAssignment]
        [pallet_configuration, Configuration]
        [pallet_registrar, Registrar]
        [pallet_invulnerables, Invulnerables]
        [pallet_pooled_staking, PooledStaking]
        [pallet_services_payment, ServicesPayment]
        [pallet_xcm_benchmarks::generic, pallet_xcm_benchmarks::generic::Pallet::<Runtime>]
    );
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

            let mut list = Vec::<BenchmarkList>::new();
            list_benchmarks!(list, extra);

            let storage_info = AllPalletsWithSystem::storage_info();
            (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig,
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{BenchmarkBatch, Benchmarking};
            use sp_core::storage::TrackedStorageKey;

            impl frame_system_benchmarking::Config for Runtime {}

            use staging_xcm::latest::prelude::*;
            use frame_benchmarking::BenchmarkError;

            impl pallet_xcm_benchmarks::Config for Runtime {
                type XcmConfig = xcm_config::XcmConfig;
                type AccountIdConverter = xcm_config::LocationToAccountId;
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
    }

    impl pallet_registrar_runtime_api::RegistrarApi<Block, ParaId, MaxLengthTokenSymbol> for Runtime {
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

            Registrar::session_container_chains(session_index).to_vec()
        }

        /// Fetch genesis data for this para id
        fn genesis_data(para_id: ParaId) -> Option<ContainerChainGenesisData<MaxLengthTokenSymbol>> {
            Registrar::para_genesis_data(para_id)
        }

        /// Fetch boot_nodes for this para id
        fn boot_nodes(para_id: ParaId) -> Vec<Vec<u8>> {
            let bounded_vec = Registrar::boot_nodes(para_id);

            bounded_vec.into_iter().map(|x| x.into()).collect()
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

    impl tp_consensus::TanssiAuthorityAssignmentApi<Block, NimbusId> for Runtime {
        /// Return the current authorities assigned to a given paraId
        fn para_id_authorities(para_id: ParaId) -> Option<Vec<NimbusId>> {
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

            if para_id == self_para_id {
                Some(assigned_authorities.orchestrator_chain)
            } else {
                assigned_authorities.container_chains.get(&para_id).cloned()
            }
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
}

struct CheckInherents;

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
    BlockExecutor = pallet_author_inherent::BlockExecutor::<Runtime, Executive>
    CheckInherents = CheckInherents,
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
