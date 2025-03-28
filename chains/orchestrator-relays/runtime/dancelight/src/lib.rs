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

//! The Dancelight runtime for v1 parachains.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit.
#![recursion_limit = "512"]

extern crate alloc;

use frame_support::storage::{with_storage_layer, with_transaction};
// Fix compile error in impl_runtime_weights! macro
use {
    authority_discovery_primitives::AuthorityId as AuthorityDiscoveryId,
    beefy_primitives::{
        ecdsa_crypto::{AuthorityId as BeefyId, Signature as BeefySignature},
        mmr::{BeefyDataProvider, MmrLeafVersion},
    },
    cumulus_primitives_core::relay_chain::{HeadData, ValidationCode},
    dp_container_chain_genesis_data::ContainerChainGenesisDataItem,
    frame_support::{
        dispatch::DispatchResult,
        dynamic_params::{dynamic_pallet_params, dynamic_params},
        traits::{
            fungible::Inspect,
            tokens::{PayFromAccount, UnityAssetBalanceConversion},
            ConstBool, Contains, EverythingBut,
        },
    },
    frame_system::{pallet_prelude::BlockNumberFor, EnsureNever},
    nimbus_primitives::NimbusId,
    pallet_collator_assignment::{GetRandomnessForNextBlock, RotateCollatorsEveryNSessions},
    pallet_initializer as tanssi_initializer,
    pallet_invulnerables::InvulnerableRewardDistribution,
    pallet_registrar::Error as ContainerRegistrarError,
    pallet_registrar_runtime_api::ContainerChainGenesisData,
    pallet_services_payment::{ProvideBlockProductionCost, ProvideCollatorAssignmentCost},
    parachains_scheduler::common::Assignment,
    parity_scale_codec::{Decode, Encode, MaxEncodedLen},
    primitives::{
        slashing, vstaging::CandidateEvent, vstaging::CommittedCandidateReceiptV2,
        vstaging::CoreState, vstaging::ScrapedOnChainVotes, ApprovalVotingParams, BlockNumber,
        CandidateHash, CoreIndex, DisputeState, ExecutorParams, GroupRotationInfo, Hash,
        Id as ParaId, InboundDownwardMessage, InboundHrmpMessage, Moment, NodeFeatures, Nonce,
        OccupiedCoreAssumption, PersistedValidationData, SessionInfo, Signature,
        ValidationCodeHash, ValidatorId, ValidatorIndex, PARACHAIN_KEY_TYPE_ID,
    },
    runtime_common::{
        self as polkadot_runtime_common, impl_runtime_weights, impls::ToAuthor, paras_registrar,
        paras_sudo_wrapper, traits::Registrar as RegistrarInterface, BlockHashCount, BlockLength,
        SlowAdjustingFeeUpdate,
    },
    runtime_parachains::{
        configuration as parachains_configuration,
        disputes::{self as parachains_disputes, slashing as parachains_slashing},
        dmp as parachains_dmp, hrmp as parachains_hrmp,
        inclusion::{self as parachains_inclusion, UmpQueueId},
        initializer as parachains_initializer, on_demand as parachains_assigner_on_demand,
        origin as parachains_origin, paras as parachains_paras,
        paras_inherent as parachains_paras_inherent,
        runtime_api_impl::v11 as parachains_runtime_api_impl,
        scheduler as parachains_scheduler, session_info as parachains_session_info,
        shared as parachains_shared,
    },
    scale_info::TypeInfo,
    snowbridge_core::{
        outbound::{Command, Fee},
        ChannelId, PricingParameters,
    },
    snowbridge_pallet_outbound_queue::MerkleProof,
    sp_core::{storage::well_known_keys as StorageWellKnownKeys, Get},
    sp_genesis_builder::PresetId,
    sp_runtime::{
        traits::{BlockNumberProvider, ConvertInto},
        AccountId32,
    },
    sp_std::{
        cmp::Ordering,
        collections::{btree_map::BTreeMap, btree_set::BTreeSet, vec_deque::VecDeque},
        marker::PhantomData,
        prelude::*,
    },
    tp_bridge::ConvertLocation,
    tp_traits::{
        prod_or_fast_parameter_types, EraIndex, GetHostConfiguration, GetSessionContainerChains,
        ParaIdAssignmentHooks, RegistrarHandler, Slot, SlotFrequency,
    },
};

#[cfg(any(feature = "std", test))]
use sp_version::NativeVersion;
use {
    frame_support::{
        construct_runtime, derive_impl,
        genesis_builder_helper::{build_state, get_preset},
        parameter_types,
        traits::{
            fungible::{Balanced, Credit, HoldConsideration},
            EitherOf, EitherOfDiverse, EnsureOriginWithArg, InstanceFilter, KeyOwnerProofSystem,
            LinearStoragePrice, PrivilegeCmp, ProcessMessage, ProcessMessageError,
        },
        weights::{ConstantMultiplier, WeightMeter, WeightToFee as _},
        PalletId,
    },
    frame_system::EnsureRoot,
    pallet_grandpa::{fg_primitives, AuthorityId as GrandpaId},
    pallet_identity::legacy::IdentityInfo,
    pallet_session::historical as session_historical,
    pallet_transaction_payment::{FeeDetails, FungibleAdapter, RuntimeDispatchInfo},
    sp_core::{OpaqueMetadata, H256},
    sp_runtime::{
        generic, impl_opaque_keys,
        traits::{
            AccountIdConversion, BlakeTwo256, Block as BlockT, ConstU32, Convert, Hash as HashT,
            IdentityLookup, Keccak256, OpaqueKeys, SaturatedConversion, Verify, Zero,
        },
        transaction_validity::{TransactionPriority, TransactionSource, TransactionValidity},
        ApplyExtrinsicResult, Cow, FixedU128, KeyTypeId, Perbill, Percent, Permill, RuntimeDebug,
    },
    sp_staking::SessionIndex,
    sp_version::RuntimeVersion,
    xcm::{
        latest::prelude::*, IntoVersion, VersionedAssetId, VersionedAssets, VersionedLocation,
        VersionedXcm,
    },
};

pub use {
    frame_system::Call as SystemCall,
    pallet_balances::Call as BalancesCall,
    primitives::{AccountId, Balance},
};

#[cfg(feature = "runtime-benchmarks")]
use snowbridge_core::{AgentId, TokenId};

/// Constant values used within the runtime.
use dancelight_runtime_constants::{currency::*, fee::*, snowbridge::EthereumLocation, time::*};

// XCM configurations.
pub mod xcm_config;

pub mod bridge_to_ethereum_config;

// Weights
mod weights;

// Governance and configurations.
pub mod governance;
use {
    governance::{
        pallet_custom_origins, AuctionAdmin, Fellows, GeneralAdmin, Treasurer, TreasurySpender,
    },
    pallet_collator_assignment::CoreAllocationConfiguration,
    xcm_runtime_apis::fees::Error as XcmPaymentApiError,
};

#[cfg(test)]
mod tests;

pub mod genesis_config_presets;

impl_runtime_weights!(dancelight_runtime_constants);

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

/// Runtime version (Dancelight).
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: Cow::Borrowed("dancelight"),
    impl_name: Cow::Borrowed("tanssi-dancelight-v2.0"),
    authoring_version: 0,
    spec_version: 1300,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 26,
    system_version: 1,
};

/// The BABE epoch configuration at genesis.
pub const BABE_GENESIS_EPOCH_CONFIG: babe_primitives::BabeEpochConfiguration =
    babe_primitives::BabeEpochConfiguration {
        c: PRIMARY_PROBABILITY,
        allowed_slots: babe_primitives::AllowedSlots::PrimaryAndSecondaryVRFSlots,
    };

/// Native version.
#[cfg(any(feature = "std", test))]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

/// Aggregate message origin for the `MessageQueue` pallet.
///
/// Can be extended to serve further use-cases besides just UMP. Is stored in storage, so any change
/// to existing values will require a migration.
#[derive(Encode, Decode, Clone, MaxEncodedLen, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum AggregateMessageOrigin {
    /// Inbound upward message.
    #[codec(index = 0)]
    Ump(UmpQueueId),

    /// The message came from a snowbridge channel. It will be processed by `snowbridge_pallet_outbound_queue`.
    #[codec(index = 1)]
    Snowbridge(ChannelId),

    /// The message came from a snowbridge channel, and it's a custom message that only exists in Tanssi.
    /// This will be processed by `CustomProcessSnowbridgeMessage`.
    #[codec(index = 2)]
    SnowbridgeTanssi(ChannelId),
}

#[cfg(feature = "runtime-benchmarks")]
impl From<u32> for AggregateMessageOrigin {
    fn from(n: u32) -> Self {
        // Some dummy for the benchmarks.
        Self::Ump(UmpQueueId::Para(n.into()))
    }
}

pub struct GetAggregateMessageOrigin;

impl Convert<ChannelId, AggregateMessageOrigin> for GetAggregateMessageOrigin {
    fn convert(channel_id: ChannelId) -> AggregateMessageOrigin {
        AggregateMessageOrigin::Snowbridge(channel_id)
    }
}

impl Convert<UmpQueueId, AggregateMessageOrigin> for GetAggregateMessageOrigin {
    fn convert(queue_id: UmpQueueId) -> AggregateMessageOrigin {
        AggregateMessageOrigin::Ump(queue_id)
    }
}

pub struct GetAggregateMessageOriginTanssi;

impl Convert<ChannelId, AggregateMessageOrigin> for GetAggregateMessageOriginTanssi {
    fn convert(channel_id: ChannelId) -> AggregateMessageOrigin {
        AggregateMessageOrigin::SnowbridgeTanssi(channel_id)
    }
}

/// This is used by [parachains_inclusion::Pallet::on_queue_changed]
pub struct GetParaFromAggregateMessageOrigin;

impl Convert<AggregateMessageOrigin, ParaId> for GetParaFromAggregateMessageOrigin {
    fn convert(x: AggregateMessageOrigin) -> ParaId {
        match x {
            AggregateMessageOrigin::Ump(UmpQueueId::Para(para_id)) => para_id,
            AggregateMessageOrigin::Snowbridge(channel_id)
            | AggregateMessageOrigin::SnowbridgeTanssi(channel_id) => {
                // Read para id from EthereumSystem::channels storage map
                match EthereumSystem::channels(channel_id) {
                    Some(x) => x.para_id,
                    None => {
                        // This should be unreachable, but return para id 0 if channel does not exist
                        log::warn!(
                            "Got snowbridge message from channel that does not exist: {:?}",
                            channel_id
                        );
                        ParaId::from(0)
                    }
                }
            }
        }
    }
}

/// The relay register and deregister calls should no longer be necessary
/// Everything is handled by the containerRegistrar
pub struct IsRelayRegister;
impl Contains<RuntimeCall> for IsRelayRegister {
    fn contains(c: &RuntimeCall) -> bool {
        matches!(
            c,
            RuntimeCall::Registrar(paras_registrar::Call::register { .. })
        ) || matches!(
            c,
            RuntimeCall::Registrar(paras_registrar::Call::deregister { .. })
        )
    }
}

/// Dancelight shouold not permit parathread registration for now
/// TODO: remove once they are enabled
pub struct IsParathreadRegistrar;
impl Contains<RuntimeCall> for IsParathreadRegistrar {
    fn contains(c: &RuntimeCall) -> bool {
        matches!(
            c,
            RuntimeCall::ContainerRegistrar(pallet_registrar::Call::register_parathread { .. })
        )
    }
}

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;
    pub const SS58Prefix: u8 = 42;
}

#[derive_impl(frame_system::config_preludes::RelayChainDefaultConfig)]
impl frame_system::Config for Runtime {
    type BaseCallFilter = EverythingBut<(IsRelayRegister, IsParathreadRegistrar)>;
    type BlockWeights = BlockWeights;
    type BlockLength = BlockLength;
    type DbWeight = RocksDbWeight;
    type Nonce = Nonce;
    type Hash = Hash;
    type AccountId = AccountId;
    type Block = Block;
    type BlockHashCount = BlockHashCount;
    type Version = Version;
    type AccountData = pallet_balances::AccountData<Balance>;
    type SystemWeightInfo = weights::frame_system::SubstrateWeight<Runtime>;
    type SS58Prefix = SS58Prefix;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type MultiBlockMigrator = MultiBlockMigrations;
    type ExtensionsWeightInfo = weights::frame_system_extensions::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
        BlockWeights::get().max_block;
    pub const MaxScheduledPerBlock: u32 = 50;
    pub const NoPreimagePostponement: Option<u32> = Some(10);
}

/// Used the compare the privilege of an origin inside the scheduler.
pub struct OriginPrivilegeCmp;

impl PrivilegeCmp<OriginCaller> for OriginPrivilegeCmp {
    fn cmp_privilege(left: &OriginCaller, right: &OriginCaller) -> Option<Ordering> {
        if left == right {
            return Some(Ordering::Equal);
        }

        match (left, right) {
            // Root is greater than anything.
            (OriginCaller::system(frame_system::RawOrigin::Root), _) => Some(Ordering::Greater),
            // For every other origin we don't care, as they are not used for `ScheduleOrigin`.
            _ => None,
        }
    }
}

/// Dynamic params that can be adjusted at runtime.
#[dynamic_params(RuntimeParameters, pallet_parameters::Parameters::<Runtime>)]
pub mod dynamic_params {
    use super::*;

    #[dynamic_pallet_params]
    #[codec(index = 0)]
    pub mod preimage {
        use super::*;

        #[codec(index = 0)]
        pub static BaseDeposit: Balance = deposit(2, 64);

        #[codec(index = 1)]
        pub static ByteDeposit: Balance = deposit(0, 1);
    }
}

#[cfg(feature = "runtime-benchmarks")]
impl Default for RuntimeParameters {
    fn default() -> Self {
        RuntimeParameters::Preimage(dynamic_params::preimage::Parameters::BaseDeposit(
            dynamic_params::preimage::BaseDeposit,
            Some(1u32.into()),
        ))
    }
}

/// Defines what origin can modify which dynamic parameters.
pub struct DynamicParameterOrigin;
impl EnsureOriginWithArg<RuntimeOrigin, RuntimeParametersKey> for DynamicParameterOrigin {
    type Success = ();

    fn try_origin(
        origin: RuntimeOrigin,
        key: &RuntimeParametersKey,
    ) -> Result<Self::Success, RuntimeOrigin> {
        use crate::RuntimeParametersKey::*;

        match key {
            Preimage(_) => frame_system::ensure_root(origin.clone()),
        }
        .map_err(|_| origin)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin(_key: &RuntimeParametersKey) -> Result<RuntimeOrigin, ()> {
        // Provide the origin for the parameter returned by `Default`:
        Ok(RuntimeOrigin::root())
    }
}

impl pallet_scheduler::Config for Runtime {
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeEvent = RuntimeEvent;
    type PalletsOrigin = OriginCaller;
    type RuntimeCall = RuntimeCall;
    type MaximumWeight = MaximumSchedulerWeight;
    // The goal of having ScheduleOrigin include AuctionAdmin is to allow the auctions track of
    // OpenGov to schedule periodic auctions.
    type ScheduleOrigin = EitherOf<EnsureRoot<AccountId>, AuctionAdmin>;
    type MaxScheduledPerBlock = MaxScheduledPerBlock;
    type WeightInfo = weights::pallet_scheduler::SubstrateWeight<Runtime>;
    type OriginPrivilegeCmp = OriginPrivilegeCmp;
    type Preimages = Preimage;
}

parameter_types! {
    pub const PreimageHoldReason: RuntimeHoldReason = RuntimeHoldReason::Preimage(pallet_preimage::HoldReason::Preimage);
}

impl pallet_preimage::Config for Runtime {
    type WeightInfo = weights::pallet_preimage::SubstrateWeight<Runtime>;
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type ManagerOrigin = EnsureRoot<AccountId>;
    type Consideration = HoldConsideration<
        AccountId,
        Balances,
        PreimageHoldReason,
        LinearStoragePrice<
            dynamic_params::preimage::BaseDeposit,
            dynamic_params::preimage::ByteDeposit,
            Balance,
        >,
    >;
}

parameter_types! {
    pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
    pub ReportLongevity: u64 = u64::from(EpochDurationInBlocks::get()) * 10;
}

impl pallet_babe::Config for Runtime {
    type EpochDuration = EpochDurationInBlocks;
    type ExpectedBlockTime = ExpectedBlockTime;
    // session module is the trigger
    type EpochChangeTrigger = pallet_babe::ExternalTrigger;
    type DisabledValidators = Session;
    // Not benchmarked in Kusama
    type WeightInfo = ();
    type MaxAuthorities = MaxAuthorities;
    type MaxNominators = ConstU32<0>;
    type KeyOwnerProof = sp_session::MembershipProof;
    type EquivocationReportSystem =
        pallet_babe::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
}

parameter_types! {
    pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = weights::pallet_balances::SubstrateWeight<Runtime>;
    type FreezeIdentifier = ();
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type MaxFreezes = ConstU32<1>;
    type DoneSlashHandler = ();
}

parameter_types! {
    pub const TransactionByteFee: Balance = 10 * MILLICENTS;
    /// This value increases the priority of `Operational` transactions by adding
    /// a "virtual tip" that's equal to the `OperationalFeeMultiplier * final_fee`.
    pub const OperationalFeeMultiplier: u8 = 5;
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = FungibleAdapter<Balances, ToAuthor<Runtime>>;
    type OperationalFeeMultiplier = OperationalFeeMultiplier;
    type WeightToFee = WeightToFee;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
    type WeightInfo = weights::pallet_transaction_payment::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}
impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = Babe;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = weights::pallet_timestamp::SubstrateWeight<Runtime>;
}

pub struct RewardPoints;

impl pallet_authorship::EventHandler<AccountId, BlockNumberFor<Runtime>> for RewardPoints {
    fn note_author(author: AccountId) {
        let whitelisted_validators =
            pallet_external_validators::WhitelistedValidatorsActiveEra::<Runtime>::get();
        // Do not reward whitelisted validators
        if !whitelisted_validators.contains(&author) {
            ExternalValidatorsRewards::reward_by_ids(vec![(author, 20u32)])
        }
    }
}

impl pallet_authorship::Config for Runtime {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
    type EventHandler = RewardPoints;
}

impl_opaque_keys! {
    pub struct SessionKeys {
        pub grandpa: Grandpa,
        pub babe: Babe,
        pub para_validator: Initializer,
        pub para_assignment: ParaSessionInfo,
        pub authority_discovery: AuthorityDiscovery,
        pub beefy: Beefy,
        pub nimbus: TanssiInitializer,
    }
}

/// Special `ValidatorIdOf` implementation that is just returning the input as result.
pub struct ValidatorIdOf;
impl sp_runtime::traits::Convert<AccountId, Option<AccountId>> for ValidatorIdOf {
    fn convert(a: AccountId) -> Option<AccountId> {
        Some(a)
    }
}

impl pallet_session::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = AccountId;
    type ValidatorIdOf = ValidatorIdOf;
    type ShouldEndSession = Babe;
    type NextSessionRotation = Babe;
    type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, ExternalValidators>;
    type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type Keys = SessionKeys;
    // TODO: Current benchmarking code for pallet_session requires that the runtime
    // uses pallet_staking, which we don't use. We need to make a PR to Substrate to
    // allow decoupling the benchmark from other pallets.
    // See https://github.com/paritytech/polkadot-sdk/blob/0845044454c005b577eab7afaea18583bd7e3dd3/substrate/frame/session/benchmarking/src/inner.rs#L38
    type WeightInfo = ();
}

pub struct FullIdentificationOf;
impl Convert<AccountId, Option<()>> for FullIdentificationOf {
    fn convert(_: AccountId) -> Option<()> {
        Some(())
    }
}

impl pallet_session::historical::Config for Runtime {
    type FullIdentification = ();
    type FullIdentificationOf = FullIdentificationOf;
}

parameter_types! {
    pub const BondingDuration: sp_staking::EraIndex = runtime_common::prod_or_fast!(28, 3);
}

parameter_types! {
    pub const ProposalBond: Permill = Permill::from_percent(5);
    pub const ProposalBondMinimum: Balance = 2000 * CENTS;
    pub const ProposalBondMaximum: Balance = 1 * GRAND;
    // We allow it to be 1 minute in fast mode to be able to test it
    pub const SpendPeriod: BlockNumber = runtime_common::prod_or_fast!(6 * DAYS, 1 * MINUTES);
    pub const Burn: Permill = Permill::from_perthousand(2);
    pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
    pub const PayoutSpendPeriod: BlockNumber = 30 * DAYS;
    // The asset's interior location for the paying account. This is the Treasury
    // pallet instance (which sits at index 18).
    pub TreasuryInteriorLocation: InteriorLocation = PalletInstance(18).into();

    pub const TipCountdown: BlockNumber = 1 * DAYS;
    pub const TipFindersFee: Percent = Percent::from_percent(20);
    pub const TipReportDepositBase: Balance = 100 * CENTS;
    pub const DataDepositPerByte: Balance = 1 * CENTS;
    pub const MaxApprovals: u32 = 100;
    pub const MaxAuthorities: u32 = 100_000;
    pub const MaxKeys: u32 = 10_000;
    pub const MaxPeerInHeartbeats: u32 = 10_000;
    pub const MaxBalance: Balance = Balance::max_value();
    pub TreasuryAccount: AccountId = Treasury::account_id();
    pub SnowbridgeFeesAccount: AccountId = PalletId(*b"sb/feeac").into_account_truncating();
}

#[cfg(feature = "runtime-benchmarks")]
pub struct TreasuryBenchmarkHelper<T>(PhantomData<T>);

#[cfg(feature = "runtime-benchmarks")]
use frame_support::traits::Currency;
#[cfg(feature = "runtime-benchmarks")]
use pallet_treasury::ArgumentsFactory;
use {
    frame_support::traits::{
        ExistenceRequirement, OnUnbalanced, ValidatorRegistration, WithdrawReasons,
    },
    pallet_services_payment::BalanceOf,
    runtime_parachains::configuration::HostConfiguration,
    sp_runtime::{DispatchError, TransactionOutcome},
};

#[cfg(feature = "runtime-benchmarks")]
impl<T> ArgumentsFactory<(), T::AccountId> for TreasuryBenchmarkHelper<T>
where
    T: pallet_treasury::Config,
    T::AccountId: From<[u8; 32]>,
{
    fn create_asset_kind(_seed: u32) {}

    fn create_beneficiary(seed: [u8; 32]) -> T::AccountId {
        let account: T::AccountId = seed.into();
        let balance = T::Currency::minimum_balance();
        let _ = T::Currency::make_free_balance_be(&account, balance);
        account
    }
}

impl pallet_treasury::Config for Runtime {
    type PalletId = TreasuryPalletId;
    type Currency = Balances;
    type RejectOrigin = EitherOfDiverse<EnsureRoot<AccountId>, Treasurer>;
    type RuntimeEvent = RuntimeEvent;
    type SpendPeriod = SpendPeriod;
    type Burn = Burn;
    type BurnDestination = ();
    type MaxApprovals = MaxApprovals;
    type WeightInfo = weights::pallet_treasury::SubstrateWeight<Runtime>;
    type SpendFunds = ();
    type SpendOrigin = TreasurySpender;
    type AssetKind = ();
    type Beneficiary = AccountId;
    type BeneficiaryLookup = IdentityLookup<Self::Beneficiary>;
    type Paymaster = PayFromAccount<Balances, TreasuryAccount>;
    type BalanceConverter = UnityAssetBalanceConversion;
    type PayoutPeriod = PayoutSpendPeriod;
    type BlockNumberProvider = System;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = TreasuryBenchmarkHelper<Runtime>;
}

impl pallet_offences::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
    type OnOffenceHandler = ExternalValidatorSlashes;
}

impl pallet_authority_discovery::Config for Runtime {
    type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
    pub const MaxSetIdSessionEntries: u32 = BondingDuration::get() * SessionsPerEra::get();
}

impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    // Not benchmarked in Kusama, benchmarking code also don't match WeightInfo trait.
    type WeightInfo = ();
    type MaxAuthorities = MaxAuthorities;
    type MaxNominators = ConstU32<0>;
    type MaxSetIdSessionEntries = MaxSetIdSessionEntries;
    type KeyOwnerProof = sp_session::MembershipProof;
    type EquivocationReportSystem =
        pallet_grandpa::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
}

/// Submits a transaction with the node's public and signature type. Adheres to the signed extension
/// format of the chain.
impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
    RuntimeCall: From<LocalCall>,
{
    fn create_signed_transaction<
        C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>,
    >(
        call: RuntimeCall,
        public: <Signature as Verify>::Signer,
        account: AccountId,
        nonce: <Runtime as frame_system::Config>::Nonce,
    ) -> Option<UncheckedExtrinsic> {
        use sp_runtime::traits::StaticLookup;
        // take the biggest period possible.
        let period = BlockHashCount::get()
            .checked_next_power_of_two()
            .map(|c| c / 2)
            .unwrap_or(2) as u64;

        let current_block = System::block_number()
            .saturated_into::<u64>()
            // The `System::block_number` is initialized with `n+1`,
            // so the actual block number is `n`.
            .saturating_sub(1);
        let tip = 0;
        let tx_ext: TxExtension = (
            frame_system::CheckNonZeroSender::<Runtime>::new(),
            frame_system::CheckSpecVersion::<Runtime>::new(),
            frame_system::CheckTxVersion::<Runtime>::new(),
            frame_system::CheckGenesis::<Runtime>::new(),
            frame_system::CheckMortality::<Runtime>::from(generic::Era::mortal(
                period,
                current_block,
            )),
            frame_system::CheckNonce::<Runtime>::from(nonce),
            frame_system::CheckWeight::<Runtime>::new(),
            pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
            //cumulus_primitives_storage_weight_reclaim::StorageWeightReclaim::<Runtime>::new(),
            frame_metadata_hash_extension::CheckMetadataHash::new(true),
        );
        let raw_payload = SignedPayload::new(call, tx_ext)
            .map_err(|e| {
                log::warn!("Unable to create signed payload: {:?}", e);
            })
            .ok()?;
        let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
        let (call, tx_ext, _) = raw_payload.deconstruct();
        let address = <Runtime as frame_system::Config>::Lookup::unlookup(account);
        let transaction = UncheckedExtrinsic::new_signed(call, address, signature, tx_ext);
        Some(transaction)
    }
}

impl frame_system::offchain::SigningTypes for Runtime {
    type Public = <Signature as Verify>::Signer;
    type Signature = Signature;
}

impl<C> frame_system::offchain::CreateTransactionBase<C> for Runtime
where
    RuntimeCall: From<C>,
{
    type Extrinsic = UncheckedExtrinsic;
    type RuntimeCall = RuntimeCall;
}

impl<LocalCall> frame_system::offchain::CreateInherent<LocalCall> for Runtime
where
    RuntimeCall: From<LocalCall>,
{
    fn create_inherent(call: RuntimeCall) -> UncheckedExtrinsic {
        UncheckedExtrinsic::new_bare(call)
    }
}

parameter_types! {
    // Minimum 100 bytes/STAR deposited (1 CENT/byte)
    pub const BasicDeposit: Balance = 1000 * CENTS;       // 258 bytes on-chain
    pub const ByteDeposit: Balance = deposit(0, 1);
    pub const UsernameDeposit: Balance = deposit(0, 32);
    pub const SubAccountDeposit: Balance = 200 * CENTS;   // 53 bytes on-chain
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
    type IdentityInformation = IdentityInfo<MaxAdditionalFields>;
    type MaxRegistrars = MaxRegistrars;
    type Slashed = Treasury;
    type ForceOrigin = EitherOf<EnsureRoot<Self::AccountId>, GeneralAdmin>;
    type RegistrarOrigin = EitherOf<EnsureRoot<Self::AccountId>, GeneralAdmin>;
    type OffchainSignature = Signature;
    type SigningPublicKey = <Signature as Verify>::Signer;
    type UsernameAuthorityOrigin = EnsureRoot<Self::AccountId>;
    type PendingUsernameExpiration = ConstU32<{ 7 * DAYS }>;
    type UsernameGracePeriod = ConstU32<{ 30 * DAYS }>;
    type MaxSuffixLength = ConstU32<7>;
    type MaxUsernameLength = ConstU32<32>;
    type WeightInfo = weights::pallet_identity::SubstrateWeight<Runtime>;
}

impl pallet_utility::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = weights::pallet_utility::SubstrateWeight<Runtime>;
}

parameter_types! {
    // One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
    pub const DepositBase: Balance = deposit(1, 88);
    // Additional storage item size of 32 bytes.
    pub const DepositFactor: Balance = deposit(0, 32);
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

parameter_types! {
    // One storage item; key size 32, value size 8; .
    pub const ProxyDepositBase: Balance = deposit(1, 8);
    // Additional storage item size of 33 bytes.
    pub const ProxyDepositFactor: Balance = deposit(0, 33);
    pub const MaxProxies: u16 = 32;
    pub const AnnouncementDepositBase: Balance = deposit(1, 8);
    pub const AnnouncementDepositFactor: Balance = deposit(0, 66);
    pub const MaxPending: u16 = 32;
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
    RuntimeDebug,
    MaxEncodedLen,
    TypeInfo,
)]
pub enum ProxyType {
    Any,
    NonTransfer,
    Governance,
    IdentityJudgement,
    CancelProxy,
    Auction,
    OnDemandOrdering,
    SudoRegistrar,
    SudoValidatorManagement,
    SessionKeyManagement,
    Staking,
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
            ProxyType::NonTransfer => matches!(
                c,
                RuntimeCall::System(..) |
				RuntimeCall::Babe(..) |
				RuntimeCall::Timestamp(..) |
				// Specifically omitting Indices `transfer`, `force_transfer`
				// Specifically omitting the entire Balances pallet
				RuntimeCall::Session(..) |
				RuntimeCall::Grandpa(..) |
				RuntimeCall::Treasury(..) |
				RuntimeCall::ConvictionVoting(..) |
				RuntimeCall::Referenda(..) |
				RuntimeCall::FellowshipCollective(..) |
				RuntimeCall::FellowshipReferenda(..) |
				RuntimeCall::Whitelist(..) |
				RuntimeCall::Utility(..) |
				RuntimeCall::Identity(..) |
				RuntimeCall::Scheduler(..) |
				RuntimeCall::Proxy(..) |
				RuntimeCall::Multisig(..) |
				RuntimeCall::Registrar(paras_registrar::Call::register {..}) |
				RuntimeCall::Registrar(paras_registrar::Call::deregister {..}) |
				// Specifically omitting Registrar `swap`
				RuntimeCall::Registrar(paras_registrar::Call::reserve {..})
            ),
            ProxyType::Governance => matches!(
                c,
                RuntimeCall::Utility(..) |
					// OpenGov calls
					RuntimeCall::ConvictionVoting(..) |
					RuntimeCall::Referenda(..) |
					RuntimeCall::FellowshipCollective(..) |
					RuntimeCall::FellowshipReferenda(..) |
					RuntimeCall::Whitelist(..)
            ),
            ProxyType::IdentityJudgement => matches!(
                c,
                RuntimeCall::Identity(pallet_identity::Call::provide_judgement { .. })
                    | RuntimeCall::Utility(..)
            ),
            ProxyType::CancelProxy => {
                matches!(
                    c,
                    RuntimeCall::Proxy(pallet_proxy::Call::reject_announcement { .. })
                )
            }
            ProxyType::Auction => {
                matches!(c, RuntimeCall::Registrar { .. } | RuntimeCall::Multisig(..))
            }
            ProxyType::OnDemandOrdering => matches!(c, RuntimeCall::OnDemandAssignmentProvider(..)),
            ProxyType::SudoRegistrar => match c {
                RuntimeCall::Sudo(pallet_sudo::Call::sudo { call: ref x }) => {
                    matches!(
                        x.as_ref(),
                        &RuntimeCall::DataPreservers(..)
                            | &RuntimeCall::Registrar(..)
                            | &RuntimeCall::ContainerRegistrar(..)
                            | &RuntimeCall::Paras(..)
                            | &RuntimeCall::ParasSudoWrapper(..)
                    )
                }
                _ => false,
            },
            ProxyType::SudoValidatorManagement => match c {
                RuntimeCall::Sudo(pallet_sudo::Call::sudo { call: ref x }) => {
                    matches!(
                        x.as_ref(),
                        &RuntimeCall::ExternalValidators(..)
                            | &RuntimeCall::ExternalValidatorSlashes(..)
                    )
                }
                _ => false,
            },
            ProxyType::SessionKeyManagement => {
                matches!(c, RuntimeCall::Session(..))
            }
            ProxyType::Staking => {
                matches!(c, RuntimeCall::Session(..) | RuntimeCall::PooledStaking(..))
            }
        }
    }
    fn is_superset(&self, o: &Self) -> bool {
        match (self, o) {
            (x, y) if x == y => true,
            (ProxyType::Any, _) => true,
            (_, ProxyType::Any) => false,
            (ProxyType::NonTransfer, _) => true,
            _ => false,
        }
    }
}

impl pallet_proxy::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type Currency = Balances;
    type ProxyType = ProxyType;
    type ProxyDepositBase = ProxyDepositBase;
    type ProxyDepositFactor = ProxyDepositFactor;
    type MaxProxies = MaxProxies;
    type WeightInfo = weights::pallet_proxy::SubstrateWeight<Runtime>;
    type MaxPending = MaxPending;
    type CallHasher = BlakeTwo256;
    type AnnouncementDepositBase = AnnouncementDepositBase;
    type AnnouncementDepositFactor = AnnouncementDepositFactor;
}

impl parachains_origin::Config for Runtime {}

impl parachains_configuration::Config for Runtime {
    type WeightInfo = weights::runtime_parachains_configuration::SubstrateWeight<Runtime>;
}

impl parachains_shared::Config for Runtime {
    type DisabledValidators = Session;
}

impl parachains_session_info::Config for Runtime {
    type ValidatorSet = Historical;
}

pub type RewardValidators =
    pallet_external_validators_rewards::RewardValidatorsWithEraPoints<Runtime>;

impl parachains_inclusion::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type DisputesHandler = ParasDisputes;
    type RewardValidators = RewardValidators;
    type AggregateMessageOrigin = AggregateMessageOrigin;
    type GetAggregateMessageOrigin = GetAggregateMessageOrigin;
    type GetParaFromAggregateMessageOrigin = GetParaFromAggregateMessageOrigin;
    type MessageQueue = MessageQueue;
    type WeightInfo = weights::runtime_parachains_inclusion::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const ParasUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
}

impl parachains_paras::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = weights::runtime_parachains_paras::SubstrateWeight<Runtime>;
    type UnsignedPriority = ParasUnsignedPriority;
    type QueueFootprinter = ParaInclusion;
    type NextSessionRotation = Babe;
    type OnNewHead = Registrar;
    type AssignCoretime = ();
}

parameter_types! {
    /// Amount of weight that can be spent per block to service messages.
    ///
    /// # WARNING
    ///
    /// This is not a good value for para-chains since the `Scheduler` already uses up to 80% block weight.
    pub MessageQueueServiceWeight: Weight = Perbill::from_percent(20) * BlockWeights::get().max_block;
    pub const MessageQueueHeapSize: u32 = 32 * 1024;
    pub const MessageQueueMaxStale: u32 = 96;
}

/// Message processor to handle any messages that were enqueued into the `MessageQueue` pallet.
pub struct MessageProcessor;
impl ProcessMessage for MessageProcessor {
    type Origin = AggregateMessageOrigin;

    fn process_message(
        message: &[u8],
        origin: Self::Origin,
        meter: &mut WeightMeter,
        id: &mut [u8; 32],
    ) -> Result<bool, ProcessMessageError> {
        match origin {
            AggregateMessageOrigin::Ump(UmpQueueId::Para(para)) => {
                xcm_builder::ProcessXcmMessage::<
                    Junction,
                    xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
                    RuntimeCall,
                >::process_message(
                    message, Junction::Parachain(para.into()), meter, id
                )
            }
            AggregateMessageOrigin::Snowbridge(_) => {
                snowbridge_pallet_outbound_queue::Pallet::<Runtime>::process_message(
                    message, origin, meter, id,
                )
            }
            AggregateMessageOrigin::SnowbridgeTanssi(_) => {
                tp_bridge::CustomProcessSnowbridgeMessage::<Runtime>::process_message(
                    message, origin, meter, id,
                )
            }
        }
    }
}

impl pallet_message_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Size = u32;
    type HeapSize = MessageQueueHeapSize;
    type MaxStale = MessageQueueMaxStale;
    type ServiceWeight = MessageQueueServiceWeight;
    type IdleMaxServiceWeight = MessageQueueServiceWeight;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type MessageProcessor = MessageProcessor;
    #[cfg(feature = "runtime-benchmarks")]
    type MessageProcessor =
        pallet_message_queue::mock_helpers::NoopMessageProcessor<AggregateMessageOrigin>;
    type QueueChangeHandler = ParaInclusion;
    type QueuePausedQuery = ();
    type WeightInfo = weights::pallet_message_queue::SubstrateWeight<Runtime>;
}

impl parachains_dmp::Config for Runtime {}

parameter_types! {
    pub const HrmpChannelSizeAndCapacityWithSystemRatio: Percent = Percent::from_percent(100);
}

impl parachains_hrmp::Config for Runtime {
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeEvent = RuntimeEvent;
    type ChannelManager = EnsureRoot<AccountId>;
    type Currency = Balances;
    type DefaultChannelSizeAndCapacityWithSystem =
        parachains_configuration::ActiveConfigHrmpChannelSizeAndCapacityRatio<
            Runtime,
            HrmpChannelSizeAndCapacityWithSystemRatio,
        >;
    type WeightInfo = weights::runtime_parachains_hrmp::SubstrateWeight<Runtime>;
    type VersionWrapper = XcmPallet;
}

impl parachains_paras_inherent::Config for Runtime {
    type WeightInfo = weights::runtime_parachains_paras_inherent::SubstrateWeight<Runtime>;
}

impl parachains_scheduler::Config for Runtime {
    // If you change this, make sure the `Assignment` type of the new provider is binary compatible,
    // otherwise provide a migration.
    type AssignmentProvider = CollatorAssignmentProvider;
}

pub struct CollatorAssignmentProvider;
impl parachains_scheduler::common::AssignmentProvider<BlockNumberFor<Runtime>>
    for CollatorAssignmentProvider
{
    fn pop_assignment_for_core(core_idx: CoreIndex) -> Option<Assignment> {
        let assigned_collators = TanssiCollatorAssignment::collator_container_chain();
        let assigned_paras: Vec<ParaId> = assigned_collators
            .container_chains
            .iter()
            .filter_map(|(&para_id, collators)| {
                if Paras::is_parachain(para_id) && collators.len() > 0 {
                    Some(para_id)
                } else {
                    None
                }
            })
            .collect();
        log::debug!("pop assigned collators {:?}", assigned_paras);
        log::debug!("looking for core idx {:?}", core_idx);

        if let Some(para_id) = assigned_paras.get(core_idx.0 as usize) {
            log::debug!("outputing assignment for  {:?}", para_id);

            Some(Assignment::Bulk(*para_id))
        } else {
            // We dont want to assign affinity to a parathread that has not collators assigned
            // Even if we did they would need their own collators to produce blocks, but for now
            // I prefer to forbid.
            // In this case the parathread would have bought the core for nothing
            let assignment =
                parachains_assigner_on_demand::Pallet::<Runtime>::pop_assignment_for_core(
                    core_idx,
                )?;

            // Let's check that we have collators before allowing an assignment
            if assigned_collators
                .container_chains
                .get(&assignment.para_id())
                .unwrap_or(&vec![])
                .len()
                > 0
            {
                Some(assignment)
            } else {
                None
            }
        }
    }
    fn report_processed(assignment: Assignment) {
        match assignment {
            Assignment::Pool {
                para_id,
                core_index,
            } => parachains_assigner_on_demand::Pallet::<Runtime>::report_processed(
                para_id, core_index,
            ),
            Assignment::Bulk(_) => {}
        }
    }
    /// Push an assignment back to the front of the queue.
    ///
    /// The assignment has not been processed yet. Typically used on session boundaries.
    /// Parameters:
    /// - `assignment`: The on demand assignment.
    fn push_back_assignment(assignment: Assignment) {
        match assignment {
            Assignment::Pool {
                para_id,
                core_index,
            } => parachains_assigner_on_demand::Pallet::<Runtime>::push_back_assignment(
                para_id, core_index,
            ),
            Assignment::Bulk(_) => {
                // Session changes are rough. We just drop assignments that did not make it on a
                // session boundary. This seems sensible as bulk is region based. Meaning, even if
                // we made the effort catching up on those dropped assignments, this would very
                // likely lead to other assignments not getting served at the "end" (when our
                // assignment set gets replaced).
            }
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn get_mock_assignment(_: CoreIndex, para_id: primitives::Id) -> Assignment {
        // Given that we are not tracking anything in `Bulk` assignments, it is safe to always
        // return a bulk assignment.
        Assignment::Bulk(para_id)
    }

    fn assignment_duplicated(assignment: &Assignment) {
        match assignment {
            Assignment::Pool {
                para_id,
                core_index,
            } => parachains_assigner_on_demand::Pallet::<Runtime>::assignment_duplicated(
                *para_id,
                *core_index,
            ),
            Assignment::Bulk(_) => {}
        }
    }
}

parameter_types! {
    pub const OnDemandTrafficDefaultValue: FixedU128 = FixedU128::from_u32(1);
    // Keep 2 blocks worth of revenue information.
    // We don't need this because it is only used by coretime and we don't have coretime,
    // but the pallet implicitly assumes that this bound is at least 1, so we use a low value
    // that won't cause problems.
    pub const MaxHistoricalRevenue: BlockNumber = 2;
    pub const OnDemandPalletId: PalletId = PalletId(*b"py/ondmd");
}

impl parachains_assigner_on_demand::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type TrafficDefaultValue = OnDemandTrafficDefaultValue;
    type WeightInfo = weights::runtime_parachains_assigner_on_demand::SubstrateWeight<Runtime>;
    type MaxHistoricalRevenue = MaxHistoricalRevenue;
    type PalletId = OnDemandPalletId;
}

impl parachains_initializer::Config for Runtime {
    type Randomness = pallet_babe::RandomnessFromOneEpochAgo<Runtime>;
    type ForceOrigin = EnsureRoot<AccountId>;
    type WeightInfo = weights::runtime_parachains_initializer::SubstrateWeight<Runtime>;
    type CoretimeOnNewSession = ();
}

impl parachains_disputes::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RewardValidators = RewardValidators;
    type SlashingHandler = parachains_slashing::SlashValidatorsForDisputes<ParasSlashing>;
    type WeightInfo = weights::runtime_parachains_disputes::SubstrateWeight<Runtime>;
}

impl parachains_slashing::Config for Runtime {
    type KeyOwnerProofSystem = Historical;
    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, ValidatorId)>>::Proof;
    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        ValidatorId,
    )>>::IdentificationTuple;
    type HandleReports = parachains_slashing::SlashingReportHandler<
        Self::KeyOwnerIdentification,
        Offences,
        ReportLongevity,
    >;
    type WeightInfo = parachains_slashing::TestWeightInfo;
    type BenchmarkingConfig = parachains_slashing::BenchConfig<200>;
}

parameter_types! {
    pub const ParaDeposit: Balance = 40 * UNITS;
}

impl paras_registrar::Config for Runtime {
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type OnSwap = ();
    type ParaDeposit = ParaDeposit;
    type DataDepositPerByte = DataDepositPerByte;
    type WeightInfo = weights::runtime_common_paras_registrar::SubstrateWeight<Runtime>;
}

impl pallet_parameters::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeParameters = RuntimeParameters;
    type AdminOrigin = DynamicParameterOrigin;
    type WeightInfo = weights::pallet_parameters::SubstrateWeight<Runtime>;
}

parameter_types! {
    // TODO: BondingDuration is set to 28 days on Polkadot,
    // check which value to use in Starlight.
    pub BeefySetIdSessionEntries: u32 = BondingDuration::get() * SessionsPerEra::get();
}

impl pallet_beefy::Config for Runtime {
    type BeefyId = BeefyId;
    type MaxAuthorities = MaxAuthorities;
    // MaxNominators is used in case we need to slash validators and check how many
    // nominators do they have as maximum.
    // This value is part of the parameters that are then used for extrinsics
    // weight computation.
    type MaxNominators = ConstU32<0>;
    type MaxSetIdSessionEntries = BeefySetIdSessionEntries;
    type OnNewValidatorSet = BeefyMmrLeaf;
    // There are currently no benchmarks for pallet_beefy.
    // https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame/beefy/src
    type WeightInfo = ();
    type KeyOwnerProof = <Historical as KeyOwnerProofSystem<(KeyTypeId, BeefyId)>>::Proof;
    type EquivocationReportSystem =
        pallet_beefy::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
    type AncestryHelper = BeefyMmrLeaf;
}

/// MMR helper types.
mod mmr {
    use super::Runtime;
    pub use pallet_mmr::primitives::*;

    pub type Leaf = <<Runtime as pallet_mmr::Config>::LeafData as LeafDataProvider>::LeafData;
    pub type Hashing = <Runtime as pallet_mmr::Config>::Hashing;
    pub type Hash = <Hashing as sp_runtime::traits::Hash>::Output;
}

impl pallet_mmr::Config for Runtime {
    const INDEXING_PREFIX: &'static [u8] = mmr::INDEXING_PREFIX;
    type Hashing = Keccak256;
    type OnNewRoot = pallet_beefy_mmr::DepositBeefyDigest<Runtime>;
    type WeightInfo = weights::pallet_mmr::SubstrateWeight<Runtime>;
    type LeafData = pallet_beefy_mmr::Pallet<Runtime>;
    type BlockHashProvider = pallet_mmr::DefaultBlockHashProvider<Runtime>;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ();
}

parameter_types! {
    pub LeafVersion: MmrLeafVersion = MmrLeafVersion::new(0, 0);
}

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
pub struct LeafExtraData {
    para_heads_root: H256,
    commitment_root: H256,
}

pub struct LeafExtraDataProvider;
impl BeefyDataProvider<LeafExtraData> for LeafExtraDataProvider {
    fn extra_data() -> LeafExtraData {
        let mut para_heads: Vec<(u32, Vec<u8>)> = parachains_paras::Parachains::<Runtime>::get()
            .into_iter()
            .filter_map(|id| {
                parachains_paras::Heads::<Runtime>::get(id).map(|head| (id.into(), head.0))
            })
            .collect();
        para_heads.sort();
        let para_heads_root = binary_merkle_tree::merkle_root::<mmr::Hashing, _>(
            para_heads.into_iter().map(|pair| pair.encode()),
        );

        let commitment_root =
            OutboundMessageCommitmentRecorder::take_commitment_root().unwrap_or_default();

        LeafExtraData {
            para_heads_root,
            commitment_root,
        }
    }
}

impl pallet_beefy_mmr::Config for Runtime {
    type LeafVersion = LeafVersion;
    type BeefyAuthorityToMerkleLeaf = pallet_beefy_mmr::BeefyEcdsaToEthereum;
    type LeafExtra = LeafExtraData;
    type BeefyDataProvider = LeafExtraDataProvider;
    type WeightInfo = weights::pallet_beefy_mmr::SubstrateWeight<Runtime>;
}

impl paras_sudo_wrapper::Config for Runtime {}

use {
    pallet_pooled_staking::traits::{IsCandidateEligible, Timer},
    pallet_staking::SessionInterface,
};

pub struct DancelightSessionInterface;
impl SessionInterface<AccountId> for DancelightSessionInterface {
    fn disable_validator(validator_index: u32) -> bool {
        Session::disable_index(validator_index)
    }

    fn validators() -> Vec<AccountId> {
        Session::validators()
    }

    fn prune_historical_up_to(up_to: SessionIndex) {
        Historical::prune_up_to(up_to);
    }
}

prod_or_fast_parameter_types! {
    pub const SessionsPerEra: SessionIndex = { prod: 6, fast: 3 };
    pub const SlashDeferDuration: EraIndex = { prod: 0, fast: 0 };
}

impl pallet_external_validators::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type UpdateOrigin = EnsureRoot<AccountId>;
    type HistoryDepth = ConstU32<84>;
    type MaxWhitelistedValidators = MaxWhitelistedValidators;
    type MaxExternalValidators = MaxExternalValidators;
    type ValidatorId = AccountId;
    type ValidatorIdOf = ValidatorIdOf;
    type ValidatorRegistration = Session;
    type UnixTime = Timestamp;
    type SessionsPerEra = SessionsPerEra;
    type OnEraStart = (ExternalValidatorSlashes, ExternalValidatorsRewards);
    type OnEraEnd = ExternalValidatorsRewards;
    type WeightInfo = weights::pallet_external_validators::SubstrateWeight<Runtime>;
    #[cfg(feature = "runtime-benchmarks")]
    type Currency = Balances;
}

pub struct TimestampProvider;
impl Get<u64> for TimestampProvider {
    fn get() -> u64 {
        Timestamp::get()
    }
}

parameter_types! {
    // Chain ID of Sepolia.
    // Output is: 34cdd3f84040fb44d70e83b892797846a8c0a556ce08cd470bf6d4cf7b94ff77
    pub EthereumSovereignAccount: AccountId =
        tp_bridge::EthereumLocationsConverterFor::<AccountId>::convert_location(
            &EthereumLocation::get()
        ).expect("to convert EthereumSovereignAccount");

    pub ExternalRewardsEraInflationProvider: u128 = CollatorsInflationRatePerBlock::get() * Balances::total_issuance();

    pub TokenLocationReanchored: Location = xcm_config::TokenLocation::get().reanchored(
        &EthereumLocation::get(),
        &xcm_config::UniversalLocation::get()
    ).expect("unable to reanchor reward token");
}

pub struct GetWhitelistedValidators;
impl Get<Vec<AccountId>> for GetWhitelistedValidators {
    fn get() -> Vec<AccountId> {
        pallet_external_validators::WhitelistedValidatorsActiveEra::<Runtime>::get().into()
    }
}

#[cfg(feature = "runtime-benchmarks")]
pub struct RewardsBenchHelper;
#[cfg(feature = "runtime-benchmarks")]
impl tp_bridge::TokenChannelSetterBenchmarkHelperTrait for RewardsBenchHelper {
    fn set_up_token(location: Location, token_id: TokenId) {
        snowbridge_pallet_system::ForeignToNativeId::<Runtime>::insert(&token_id, &location);
        snowbridge_pallet_system::NativeToForeignId::<Runtime>::insert(&location, &token_id);
    }

    fn set_up_channel(_channel_id: ChannelId, _para_id: ParaId, _agent_id: AgentId) {}
}
impl pallet_external_validators_rewards::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type EraIndexProvider = ExternalValidators;
    type HistoryDepth = ConstU32<64>;
    type BackingPoints = ConstU32<20>;
    type DisputeStatementPoints = ConstU32<20>;
    // TODO: add a proper way to retrieve the inflated tokens.
    // Will likely be through InflationRewards.

    type EraInflationProvider = ExternalRewardsEraInflationProvider;
    type ExternalIndexProvider = ExternalValidators;
    type GetWhitelistedValidators = GetWhitelistedValidators;
    type Hashing = Keccak256;
    type ValidateMessage = tp_bridge::MessageValidator<Runtime>;
    type OutboundQueue = tp_bridge::CustomSendMessage<Runtime, GetAggregateMessageOriginTanssi>;
    type Currency = Balances;
    type RewardsEthereumSovereignAccount = EthereumSovereignAccount;
    type TokenLocationReanchored = TokenLocationReanchored;
    type TokenIdFromLocation = EthereumSystem;
    type WeightInfo = weights::pallet_external_validators_rewards::SubstrateWeight<Runtime>;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = RewardsBenchHelper;
}

impl pallet_external_validator_slashes::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = AccountId;
    type ValidatorIdOf = ValidatorIdOf;
    type SlashDeferDuration = SlashDeferDuration;
    type BondingDuration = BondingDuration;
    type SlashId = u32;
    type SessionInterface = DancelightSessionInterface;
    type EraIndexProvider = ExternalValidators;
    type InvulnerablesProvider = ExternalValidators;
    type ValidateMessage = tp_bridge::MessageValidator<Runtime>;
    type OutboundQueue = tp_bridge::CustomSendMessage<Runtime, GetAggregateMessageOriginTanssi>;
    type ExternalIndexProvider = ExternalValidators;
    type QueuedSlashesProcessedPerBlock = ConstU32<10>;
    type WeightInfo = weights::pallet_external_validator_slashes::SubstrateWeight<Runtime>;
}

impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = weights::pallet_sudo::SubstrateWeight<Runtime>;
}

impl pallet_root_testing::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl pallet_asset_rate::Config for Runtime {
    type WeightInfo = weights::pallet_asset_rate::SubstrateWeight<Runtime>;
    type RuntimeEvent = RuntimeEvent;
    type CreateOrigin = EnsureRoot<AccountId>;
    type RemoveOrigin = EnsureRoot<AccountId>;
    type UpdateOrigin = EnsureRoot<AccountId>;
    type Currency = Balances;
    type AssetKind = <Runtime as pallet_treasury::Config>::AssetKind;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ();
}

parameter_types! {
    pub const MaxInvulnerables: u32 = 100;
    pub const MaxWhitelistedValidators: u32 = 100;
    pub const MaxExternalValidators: u32 = 100;
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

pub struct CurrentSessionIndexGetter;

impl tp_traits::GetSessionIndex<SessionIndex> for CurrentSessionIndexGetter {
    /// Returns current session index.
    fn session_index() -> SessionIndex {
        Session::current_index()
    }
}

impl pallet_configuration::Config for Runtime {
    type SessionDelay = ConstU32<2>;
    type SessionIndex = SessionIndex;
    type CurrentSessionIndex = CurrentSessionIndexGetter;
    type ForceEmptyOrchestrator = ConstBool<true>;
    type WeightInfo = weights::pallet_configuration::SubstrateWeight<Runtime>;
}

impl pallet_migrations::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MigrationsList = (tanssi_runtime_common::migrations::DancelightMigrations<Runtime>,);
    type XcmExecutionManager = ();
}

parameter_types! {
    pub MbmServiceWeight: Weight = Perbill::from_percent(80) * BlockWeights::get().max_block;
}

impl pallet_multiblock_migrations::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type Migrations = pallet_identity::migration::v2::LazyMigrationV1ToV2<Runtime>;
    // Benchmarks need mocked migrations to guarantee that they succeed.
    #[cfg(feature = "runtime-benchmarks")]
    type Migrations = pallet_multiblock_migrations::mock_helpers::MockedMigrations;
    type CursorMaxLen = ConstU32<65_536>;
    type IdentifierMaxLen = ConstU32<256>;
    type MigrationStatusHandler = ();
    type FailedMigrationHandler = frame_support::migrations::FreezeChainOnFailedMigration;
    type MaxServiceWeight = MbmServiceWeight;
    type WeightInfo = weights::pallet_multiblock_migrations::SubstrateWeight<Runtime>;
}

pub const FIXED_BLOCK_PRODUCTION_COST: u128 = 1 * MICROUNITS;
pub const FIXED_COLLATOR_ASSIGNMENT_COST: u128 = 100 * MICROUNITS;

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
    // 60 days worth of collator assignment
    pub const FreeCollatorAssignmentCredits: u32 = FreeBlockProductionCredits::get()/EpochDurationInBlocks::get();
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
    type ManagerOrigin = EnsureRoot<AccountId>;
    type WeightInfo = weights::pallet_services_payment::SubstrateWeight<Runtime>;
}

pub const OPEN_STREAM_HOLD_AMOUNT: u32 = 253;
parameter_types! {
    // 1 entry, storing 253 bytes on-chain in the worst case
    pub const OpenStreamHoldAmount: Balance = deposit(1, OPEN_STREAM_HOLD_AMOUNT);
}

impl pallet_stream_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type StreamId = tp_stream_payment_common::StreamId;
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
    pub const ProfileDepositBaseFee: Balance = STORAGE_ITEM_FEE;
    pub const ProfileDepositByteFee: Balance = STORAGE_BYTE_FEE;
    #[derive(Clone)]
    pub const MaxAssignmentsPerParaId: u32 = 10;
    #[derive(Clone)]
    pub const MaxNodeUrlLen: u32 = 200;
}

impl pallet_data_preservers::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeHoldReason = RuntimeHoldReason;
    type Currency = Balances;
    type WeightInfo = weights::pallet_data_preservers::SubstrateWeight<Runtime>;

    type ProfileId = u64;
    type ProfileDeposit = tp_traits::BytesDeposit<ProfileDepositBaseFee, ProfileDepositByteFee>;
    type AssignmentProcessor = tp_data_preservers_common::AssignmentProcessor<Runtime>;

    type AssignmentOrigin = pallet_registrar::EnsureSignedByManager<Runtime>;
    type ForceSetProfileOrigin = EnsureRoot<AccountId>;

    type MaxAssignmentsPerParaId = MaxAssignmentsPerParaId;
    type MaxNodeUrlLen = MaxNodeUrlLen;
    type MaxParaIdsVecLen = MaxLengthParaIds;
}

parameter_types! {
    pub DancelightBondAccount: AccountId32 = PalletId(*b"StarBond").into_account_truncating();
    pub PendingRewardsAccount: AccountId32 = PalletId(*b"PENDREWD").into_account_truncating();

    // 30% for dancelight bond, so 70% for staking
    pub const RewardsPortion: Perbill = Perbill::from_percent(70);
}

// We want a global annual inflation rate of 10%.
// It is compounded throught era inflations, which itself is split between:
// - Inflation for collators per block
// - Inflation for validators per era
// Computation is implemented in tests/inflation_rates.rs, with a test ensuring values from the
// runtime match the formulas. We write the results as constants here to ensure we don't perform
// computations at runtime.
prod_or_fast_parameter_types! {
    pub const CollatorsInflationRatePerBlock: Perbill = { prod: Perbill::from_parts(228), fast: Perbill::from_parts(228) };
    pub const ValidatorsInflationRatePerEra: Perbill = { prod: Perbill::from_parts(821534), fast: Perbill::from_parts(6843) };
}

pub struct OnUnbalancedInflation;
impl frame_support::traits::OnUnbalanced<Credit<AccountId, Balances>> for OnUnbalancedInflation {
    fn on_nonzero_unbalanced(credit: Credit<AccountId, Balances>) {
        let _ = <Balances as Balanced<_>>::resolve(&DancelightBondAccount::get(), credit);
    }
}

impl pallet_inflation_rewards::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type ContainerChains = ContainerRegistrar;
    type GetSelfChainBlockAuthor = ();
    type InflationRate = ValidatorsInflationRatePerEra;
    type OnUnbalanced = OnUnbalancedInflation;
    type PendingRewardsAccount = PendingRewardsAccount;
    type StakingRewardsDistributor = InvulnerableRewardDistribution<Self, Balances, PooledStaking>;
    type RewardsPortion = RewardsPortion;
}

parameter_types! {
    pub StakingAccount: AccountId32 = PalletId(*b"POOLSTAK").into_account_truncating();
    pub const InitialManualClaimShareValue: u128 = MILLIUNITS;
    pub const InitialAutoCompoundingShareValue: u128 = MILLIUNITS;
    pub const MinimumSelfDelegation: u128 = 10_000 * UNITS;
    pub const RewardsCollatorCommission: Perbill = Perbill::from_percent(20);
    // Need to wait 2 sessions before being able to join or leave staking pools
    pub const StakingSessionDelay: u32 = 2;
}

pub struct SessionTimer<Delay>(PhantomData<Delay>);

impl<Delay> Timer for SessionTimer<Delay>
where
    Delay: Get<u32>,
{
    type Instant = u32;

    fn now() -> Self::Instant {
        Session::current_index()
    }

    fn is_elapsed(instant: &Self::Instant) -> bool {
        let delay = Delay::get();
        let Some(end) = instant.checked_add(delay) else {
            return false;
        };
        end <= Self::now()
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn elapsed_instant() -> Self::Instant {
        let delay = Delay::get();
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
        use crate::genesis_config_presets::get_authority_keys_from_seed;

        if eligible {
            let a_u8: &[u8] = a.as_ref();
            let seed = scale_info::prelude::format!("{:?}", a_u8);
            let authority_keys = get_authority_keys_from_seed(&seed);
            let _ = Session::set_keys(
                RuntimeOrigin::signed(a.clone()),
                SessionKeys {
                    grandpa: authority_keys.grandpa,
                    babe: authority_keys.babe,
                    para_validator: authority_keys.para_validator,
                    para_assignment: authority_keys.para_assignment,
                    authority_discovery: authority_keys.authority_discovery,
                    beefy: authority_keys.beefy,
                    nimbus: authority_keys.nimbus,
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
    type StakingAccount = StakingAccount;
    type InitialManualClaimShareValue = InitialManualClaimShareValue;
    type InitialAutoCompoundingShareValue = InitialAutoCompoundingShareValue;
    type MinimumSelfDelegation = MinimumSelfDelegation;
    type RuntimeHoldReason = RuntimeHoldReason;
    type RewardsCollatorCommission = RewardsCollatorCommission;
    type JoiningRequestTimer = SessionTimer<StakingSessionDelay>;
    type LeavingRequestTimer = SessionTimer<StakingSessionDelay>;
    type EligibleCandidatesBufferSize = ConstU32<100>;
    type EligibleCandidatesFilter = CandidateHasRegisteredKeys;
    type WeightInfo = weights::pallet_pooled_staking::SubstrateWeight<Runtime>;
}
pub struct MockCurrentSessionGetter;

impl tp_traits::GetSessionIndex<u32> for MockCurrentSessionGetter {
    fn session_index() -> u32 {
        1
    }
}

impl pallet_inactivity_tracking::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type CollatorId = AccountId;
    type MaxInactiveSessions = ConstU32<5>;
    type MaxCollatorsPerSession = ConstU32<100>;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type CurrentSessionIndex = CurrentSessionIndexGetter;
    #[cfg(feature = "runtime-benchmarks")]
    type CurrentSessionIndex = MockCurrentSessionGetter;
    type GetSelfChainBlockAuthor = ();
    type WeightInfo = weights::pallet_inactivity_tracking::SubstrateWeight<Runtime>;
}

construct_runtime! {
    pub enum Runtime
    {
        // Basic stuff; balances is uncallable initially.
        System: frame_system = 0,

        // Babe must be before session.
        Babe: pallet_babe = 1,

        Timestamp: pallet_timestamp = 2,
        Balances: pallet_balances = 3,
        Parameters: pallet_parameters = 4,
        TransactionPayment: pallet_transaction_payment = 5,

        // Consensus support.
        // Authorship must be before session in order to note author in the correct session and era.
        Authorship: pallet_authorship = 6,
        Offences: pallet_offences = 7,
        Historical: session_historical = 8,

        // Container stuff should go before session
        // Container stuff starts at index 10
        ContainerRegistrar: pallet_registrar = 10,
        CollatorConfiguration: pallet_configuration = 11,
        TanssiInitializer: tanssi_initializer = 12,
        TanssiInvulnerables: pallet_invulnerables = 13,
        TanssiCollatorAssignment: pallet_collator_assignment = 14,
        TanssiAuthorityAssignment: pallet_authority_assignment = 15,
        TanssiAuthorityMapping: pallet_authority_mapping = 16,
        AuthorNoting: pallet_author_noting = 17,
        ServicesPayment: pallet_services_payment = 18,
        DataPreservers: pallet_data_preservers = 19,

        // Validator stuff
        ExternalValidators: pallet_external_validators = 20,
        ExternalValidatorSlashes: pallet_external_validator_slashes = 21,
        ExternalValidatorsRewards: pallet_external_validators_rewards = 22,

        // Bridging stuff - 1
        EthereumOutboundQueue: snowbridge_pallet_outbound_queue = 23,
        EthereumInboundQueue: snowbridge_pallet_inbound_queue = 24,
        EthereumSystem: snowbridge_pallet_system = 25,
        OutboundMessageCommitmentRecorder: pallet_outbound_message_commitment_recorder = 26,
        EthereumTokenTransfers: pallet_ethereum_token_transfers = 27,

        // Session management
        Session: pallet_session = 30,
        Grandpa: pallet_grandpa = 31,
        AuthorityDiscovery: pallet_authority_discovery = 32,

        // InflationRewards must be after Session
        InflationRewards: pallet_inflation_rewards = 33,
        PooledStaking: pallet_pooled_staking = 34,
        InactivityTracking: pallet_inactivity_tracking = 35,

        // Governance stuff; uncallable initially.
        Treasury: pallet_treasury = 40,
        ConvictionVoting: pallet_conviction_voting = 41,
        Referenda: pallet_referenda = 42,
        //	pub type FellowshipCollectiveInstance = pallet_ranked_collective::Instance1;
        FellowshipCollective: pallet_ranked_collective::<Instance1> = 43,
        // pub type FellowshipReferendaInstance = pallet_referenda::Instance2;
        FellowshipReferenda: pallet_referenda::<Instance2> = 44,
        Origins: pallet_custom_origins = 45,
        Whitelist: pallet_whitelist = 46,

        // Parachains pallets. Start indices at 50 to leave room.
        ParachainsOrigin: parachains_origin = 50,
        Configuration: parachains_configuration = 51,
        ParasShared: parachains_shared = 52,
        ParaInclusion: parachains_inclusion = 53,
        ParaInherent: parachains_paras_inherent = 54,
        ParaScheduler: parachains_scheduler = 55,
        Paras: parachains_paras = 56,
        Initializer: parachains_initializer = 57,
        Dmp: parachains_dmp = 58,
        Hrmp: parachains_hrmp = 60,
        ParaSessionInfo: parachains_session_info = 61,
        ParasDisputes: parachains_disputes = 62,
        ParasSlashing: parachains_slashing = 63,
        MessageQueue: pallet_message_queue = 64,
        OnDemandAssignmentProvider: parachains_assigner_on_demand = 65,

        // Parachain Onboarding Pallets. Start indices at 70 to leave room.
        Registrar: paras_registrar = 70,

        // Utility module.
        Utility: pallet_utility = 80,

        // Less simple identity module.
        Identity: pallet_identity = 81,

        // System scheduler.
        Scheduler: pallet_scheduler = 82,

        // Proxy module. Late addition.
        Proxy: pallet_proxy = 83,

        // Multisig module. Late addition.
        Multisig: pallet_multisig = 84,

        // Preimage registrar.
        Preimage: pallet_preimage = 85,

        // Asset rate.
        AssetRate: pallet_asset_rate = 86,

        // Pallet for sending XCM.
        XcmPallet: pallet_xcm = 90,

        StreamPayment: pallet_stream_payment = 100,

        // Migration stuff
        Migrations: pallet_migrations = 120,
        MultiBlockMigrations: pallet_multiblock_migrations = 121,

        // BEEFY Bridges support.
        Beefy: pallet_beefy = 240,
        // MMR leaf construction must be after session in order to have a leaf's next_auth_set
        // refer to block<N>.
        Mmr: pallet_mmr = 241,
        BeefyMmrLeaf: pallet_beefy_mmr = 242,
        EthereumBeaconClient: snowbridge_pallet_ethereum_client = 243,

        ParasSudoWrapper: paras_sudo_wrapper = 250,

        // Root testing pallet.
        RootTesting: pallet_root_testing = 249,

        // Sudo.
        Sudo: pallet_sudo = 255,
    }
}

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// `BlockId` type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The `TxExtension` to the basic transaction logic.
pub type TxExtension = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckMortality<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
    frame_metadata_hash_extension::CheckMetadataHash<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, TxExtension>;

/// The runtime migrations per release.
#[allow(deprecated, missing_docs)]
pub mod migrations {
    /// Unreleased migrations. Add new ones here:
    pub type Unreleased = ();
}

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    migrations::Unreleased,
>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, TxExtension>;

parameter_types! {
    #[derive(Clone)]
    pub const MaxLengthParaIds: u32 = 100u32;
    pub const MaxEncodedGenesisDataSize: u32 = 5_000_000u32; // 5MB
}

pub struct InnerDancelightRegistrar<Runtime, AccountId, RegistrarManager, RegistrarWeightInfo>(
    PhantomData<(Runtime, AccountId, RegistrarManager, RegistrarWeightInfo)>,
);
impl<Runtime, AccountId, RegistrarManager, RegistrarWeightInfo> RegistrarHandler<AccountId>
    for InnerDancelightRegistrar<Runtime, AccountId, RegistrarManager, RegistrarWeightInfo>
where
    RegistrarManager: RegistrarInterface<AccountId = AccountId>,
    RegistrarWeightInfo: paras_registrar::WeightInfo,
    Runtime: pallet_registrar::Config + paras_registrar::Config,
    sp_runtime::AccountId32: From<AccountId>,
{
    fn register(
        who: AccountId,
        id: ParaId,
        genesis_storage: &[ContainerChainGenesisDataItem],
        head_data: Option<HeadData>,
    ) -> DispatchResult {
        // Return early if head_data is not specified
        let genesis_head = match head_data {
            Some(data) => data,
            None => return Err(ContainerRegistrarError::<Runtime>::HeadDataNecessary.into()),
        };

        // Check if the wasm code is present in storage
        let validation_code = match genesis_storage
            .iter()
            .find(|item| item.key == StorageWellKnownKeys::CODE)
        {
            Some(item) => ValidationCode(item.value.clone()),
            None => return Err(ContainerRegistrarError::<Runtime>::WasmCodeNecessary.into()),
        };

        // Try to register the parachain
        // Using register extrinsic instead of `RegistrarInterface` trait because we want
        // to check that the para id has been reserved.
        Registrar::register(
            RuntimeOrigin::signed(who.into()),
            id,
            genesis_head,
            validation_code,
        )
    }

    fn schedule_para_upgrade(id: ParaId) -> DispatchResult {
        // Return Ok() if the paraId is already a parachain in the relay context
        if !RegistrarManager::is_parachain(id) {
            return RegistrarManager::make_parachain(id);
        }
        Ok(())
    }

    fn schedule_para_downgrade(id: ParaId) -> DispatchResult {
        // Return Ok() if the paraId is already a parathread in the relay context
        if !RegistrarManager::is_parathread(id) {
            return RegistrarManager::make_parathread(id);
        }
        Ok(())
    }

    fn deregister(id: ParaId) {
        if let Err(e) = RegistrarManager::deregister(id) {
            log::warn!(
                "Failed to deregister para id {} in relay chain: {:?}",
                u32::from(id),
                e,
            );
        }
    }

    fn deregister_weight() -> Weight {
        RegistrarWeightInfo::deregister()
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn bench_head_data() -> Option<HeadData> {
        let head_data = HeadData(vec![1; 10]);
        Some(head_data)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn add_trusted_validation_code(code: Vec<u8>) {
        Paras::add_trusted_validation_code(RuntimeOrigin::root(), code.into()).unwrap();
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn registrar_new_session(session: u32) {
        benchmark_helpers::run_to_session(session)
    }
    #[cfg(feature = "runtime-benchmarks")]
    fn prepare_chain_registration(id: ParaId, who: AccountId) {
        use frame_support::assert_ok;
        paras_registrar::NextFreeParaId::<Runtime>::put(id);
        assert_eq!(paras_registrar::NextFreeParaId::<Runtime>::get(), id);
        assert_ok!(Registrar::reserve(RuntimeOrigin::signed(who.into())));
    }
}

impl pallet_registrar::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RegistrarOrigin =
        EitherOfDiverse<pallet_registrar::EnsureSignedByManager<Runtime>, EnsureRoot<AccountId>>;
    type MarkValidForCollatingOrigin = EnsureRoot<AccountId>;
    type MaxLengthParaIds = MaxLengthParaIds;
    type MaxGenesisDataSize = MaxEncodedGenesisDataSize;
    type RegisterWithRelayProofOrigin = EnsureNever<AccountId>;
    type RelayStorageRootProvider = ();
    type SessionDelay = ConstU32<2>;
    type SessionIndex = u32;
    type CurrentSessionIndex = CurrentSessionIndexGetter;
    type Currency = Balances;
    type RegistrarHooks = DancelightRegistrarHooks;
    type RuntimeHoldReason = RuntimeHoldReason;

    type InnerRegistrar = InnerDancelightRegistrar<
        Runtime,
        AccountId,
        Registrar,
        weights::runtime_common_paras_registrar::SubstrateWeight<Runtime>,
    >;
    type WeightInfo = weights::pallet_registrar::SubstrateWeight<Runtime>;
    type DataDepositPerByte = DataDepositPerByte;
}

pub struct DancelightRegistrarHooks;

impl pallet_registrar::RegistrarHooks for DancelightRegistrarHooks {
    fn para_marked_valid_for_collating(para_id: ParaId) -> Weight {
        // Give free credits but only once per para id
        ServicesPayment::give_free_credits(&para_id)
    }

    fn para_deregistered(para_id: ParaId) -> Weight {
        if let Err(e) = AuthorNoting::kill_author_data(RuntimeOrigin::root(), para_id) {
            log::warn!(
                "Failed to kill_author_data after para id {} deregistered: {:?}",
                u32::from(para_id),
                e,
            );
        }

        /*
        XcmCoreBuyer::para_deregistered(para_id);
        */

        // Remove bootnodes from pallet_data_preservers
        DataPreservers::para_deregistered(para_id);

        ServicesPayment::para_deregistered(para_id);

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
        .expect("assignment to work");

        assert!(
            pallet_data_preservers::Assignments::<Runtime>::get(para_id).contains(&profile_id),
            "profile should be correctly assigned"
        );
    }
}

pub struct BabeSlotBeacon;

impl BlockNumberProvider for BabeSlotBeacon {
    type BlockNumber = u32;

    fn current_block_number() -> Self::BlockNumber {
        // TODO: nimbus_primitives::SlotBeacon requires u32, but this is a u64 in pallet_babe, and
        // also it gets converted to u64 in pallet_author_noting, so let's do something to remove
        // this intermediate u32 conversion, such as using a different trait
        u64::from(pallet_babe::CurrentSlot::<Runtime>::get()) as u32
    }
}

impl pallet_author_noting::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ContainerChains = TanssiCollatorAssignment;
    type SlotBeacon = BabeSlotBeacon;
    type ContainerChainAuthor = TanssiCollatorAssignment;
    type AuthorNotingHook = (InflationRewards, ServicesPayment, InactivityTracking);
    type RelayOrPara = pallet_author_noting::RelayMode;
    type MaxContainerChains = MaxLengthParaIds;
    type WeightInfo = weights::pallet_author_noting::SubstrateWeight<Runtime>;
}

frame_support::ord_parameter_types! {
    pub const MigController: AccountId = AccountId::from(hex_literal::hex!("52bc71c1eca5353749542dfdf0af97bf764f9c2f44e860cd485f1cd86400f649"));
}

#[cfg(feature = "runtime-benchmarks")]
mod benches {
    frame_benchmarking::define_benchmarks!(
        // Polkadot
        // NOTE: Make sure to prefix these with `runtime_common::` so
        // the that path resolves correctly in the generated file.
        [runtime_common::paras_registrar, Registrar]
        [runtime_parachains::configuration, Configuration]
        [runtime_parachains::hrmp, Hrmp]
        [runtime_parachains::disputes, ParasDisputes]
        [runtime_parachains::inclusion, ParaInclusion]
        [runtime_parachains::initializer, Initializer]
        [runtime_parachains::paras_inherent, ParaInherent]
        [runtime_parachains::paras, Paras]
        [runtime_parachains::assigner_on_demand, OnDemandAssignmentProvider]
        // Substrate
        [pallet_balances, Balances]
        [frame_benchmarking::baseline, Baseline::<Runtime>]
        [pallet_conviction_voting, ConvictionVoting]
        [pallet_identity, Identity]
        [pallet_message_queue, MessageQueue]
        [pallet_multiblock_migrations, MultiBlockMigrations]
        [pallet_multisig, Multisig]
        [pallet_parameters, Parameters]
        [pallet_preimage, Preimage]
        [pallet_proxy, Proxy]
        [pallet_ranked_collective, FellowshipCollective]
        [pallet_referenda, Referenda]
        [pallet_scheduler, Scheduler]
        [pallet_sudo, Sudo]
        [frame_system, SystemBench::<Runtime>]
        [frame_system_extensions, frame_system_benchmarking::extensions::Pallet::<Runtime>]
        [pallet_timestamp, Timestamp]
        [pallet_transaction_payment, TransactionPayment]
        [pallet_treasury, Treasury]
        [pallet_utility, Utility]
        [pallet_asset_rate, AssetRate]
        [pallet_whitelist, Whitelist]
        [pallet_services_payment, ServicesPayment]
        [pallet_mmr, Mmr]
        [pallet_beefy_mmr, BeefyMmrLeaf]
        [pallet_multiblock_migrations, MultiBlockMigrations]

        // Tanssi
        [pallet_author_noting, AuthorNoting]
        [pallet_registrar, ContainerRegistrar]
        [pallet_collator_assignment, TanssiCollatorAssignment]
        [pallet_external_validators, ExternalValidators]
        [pallet_external_validators_rewards, ExternalValidatorsRewards]
        [pallet_external_validator_slashes, ExternalValidatorSlashes]
        [pallet_invulnerables, TanssiInvulnerables]
        [pallet_data_preservers, DataPreservers]
        [pallet_pooled_staking, PooledStaking]
        [pallet_inactivity_tracking, InactivityTracking]
        [pallet_configuration, CollatorConfiguration]
        [pallet_stream_payment, StreamPayment]

        // XCM
        [pallet_xcm, PalletXcmExtrinsicsBenchmark::<Runtime>]
        [pallet_xcm_benchmarks::fungible, pallet_xcm_benchmarks::fungible::Pallet::<Runtime>]
        [pallet_xcm_benchmarks::generic, pallet_xcm_benchmarks::generic::Pallet::<Runtime>]
        // Bridges
        [pallet_ethereum_token_transfers, EthereumTokenTransfers]
        [snowbridge_pallet_ethereum_client, EthereumBeaconClient]
        [snowbridge_pallet_outbound_queue, EthereumOutboundQueue]
        [snowbridge_pallet_system, EthereumSystem]
        [snowbridge_pallet_inbound_queue, EthereumInboundQueue]
    );
}

sp_api::impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block);
        }

        fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
            Executive::initialize_block(header)
        }
    }

    impl xcm_runtime_apis::fees::XcmPaymentApi<Block> for Runtime {
        fn query_acceptable_payment_assets(xcm_version: xcm::Version) -> Result<Vec<VersionedAssetId>, XcmPaymentApiError> {
            if !matches!(xcm_version, 3..=5) {
                return Err(XcmPaymentApiError::UnhandledXcmVersion);
            }
            Ok([VersionedAssetId::V5(xcm_config::TokenLocation::get().into())]
                .into_iter()
                .filter_map(|asset| asset.into_version(xcm_version).ok())
                .collect())
        }

        fn query_weight_to_asset_fee(weight: Weight, asset: VersionedAssetId) -> Result<u128, XcmPaymentApiError> {
            let local_asset = VersionedAssetId::V5(xcm_config::TokenLocation::get().into());
            let asset = asset
                .into_version(5)
                .map_err(|_| XcmPaymentApiError::VersionedConversionFailed)?;

            if  asset != local_asset { return Err(XcmPaymentApiError::AssetNotFound); }

            Ok(WeightToFee::weight_to_fee(&weight))
        }

        fn query_xcm_weight(message: VersionedXcm<()>) -> Result<Weight, XcmPaymentApiError> {
            XcmPallet::query_xcm_weight(message)
        }

        fn query_delivery_fees(destination: VersionedLocation, message: VersionedXcm<()>) -> Result<VersionedAssets, XcmPaymentApiError> {
            XcmPallet::query_delivery_fees(destination, message)
        }
    }

    impl xcm_runtime_apis::conversions::LocationToAccountApi<Block, AccountId> for Runtime {
        fn convert_location(location: VersionedLocation) -> Result<
            AccountId,
            xcm_runtime_apis::conversions::Error
        > {
            xcm_runtime_apis::conversions::LocationToAccountHelper::<
                AccountId,
                xcm_config::LocationConverter,
            >::convert_location(location)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> sp_std::vec::Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl block_builder_api::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: inherents::InherentData,
        ) -> inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl tx_pool_api::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl offchain_primitives::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    #[api_version(11)]
    impl primitives::runtime_api::ParachainHost<Block> for Runtime {
        fn validators() -> Vec<ValidatorId> {
            parachains_runtime_api_impl::validators::<Runtime>()
        }

        fn validator_groups() -> (Vec<Vec<ValidatorIndex>>, GroupRotationInfo<BlockNumber>) {
            parachains_runtime_api_impl::validator_groups::<Runtime>()
        }

        fn availability_cores() -> Vec<CoreState<Hash, BlockNumber>> {
            parachains_runtime_api_impl::availability_cores::<Runtime>()
        }

        fn persisted_validation_data(para_id: ParaId, assumption: OccupiedCoreAssumption)
            -> Option<PersistedValidationData<Hash, BlockNumber>> {
            parachains_runtime_api_impl::persisted_validation_data::<Runtime>(para_id, assumption)
        }

        fn assumed_validation_data(
            para_id: ParaId,
            expected_persisted_validation_data_hash: Hash,
        ) -> Option<(PersistedValidationData<Hash, BlockNumber>, ValidationCodeHash)> {
            parachains_runtime_api_impl::assumed_validation_data::<Runtime>(
                para_id,
                expected_persisted_validation_data_hash,
            )
        }

        fn check_validation_outputs(
            para_id: ParaId,
            outputs: primitives::CandidateCommitments,
        ) -> bool {
            parachains_runtime_api_impl::check_validation_outputs::<Runtime>(para_id, outputs)
        }

        fn session_index_for_child() -> SessionIndex {
            parachains_runtime_api_impl::session_index_for_child::<Runtime>()
        }

        fn validation_code(para_id: ParaId, assumption: OccupiedCoreAssumption)
            -> Option<ValidationCode> {
            parachains_runtime_api_impl::validation_code::<Runtime>(para_id, assumption)
        }

        fn candidate_pending_availability(para_id: ParaId) -> Option<CommittedCandidateReceiptV2<Hash>> {
            #[allow(deprecated)]
            parachains_runtime_api_impl::candidate_pending_availability::<Runtime>(para_id)
        }

        fn candidate_events() -> Vec<CandidateEvent<Hash>> {
            parachains_runtime_api_impl::candidate_events::<Runtime, _>(|ev| {
                match ev {
                    RuntimeEvent::ParaInclusion(ev) => {
                        Some(ev)
                    }
                    _ => None,
                }
            })
        }

        fn session_info(index: SessionIndex) -> Option<SessionInfo> {
            parachains_runtime_api_impl::session_info::<Runtime>(index)
        }

        fn session_executor_params(session_index: SessionIndex) -> Option<ExecutorParams> {
            parachains_runtime_api_impl::session_executor_params::<Runtime>(session_index)
        }

        fn dmq_contents(recipient: ParaId) -> Vec<InboundDownwardMessage<BlockNumber>> {
            parachains_runtime_api_impl::dmq_contents::<Runtime>(recipient)
        }

        fn inbound_hrmp_channels_contents(
            recipient: ParaId
        ) -> BTreeMap<ParaId, Vec<InboundHrmpMessage<BlockNumber>>> {
            parachains_runtime_api_impl::inbound_hrmp_channels_contents::<Runtime>(recipient)
        }

        fn validation_code_by_hash(hash: ValidationCodeHash) -> Option<ValidationCode> {
            parachains_runtime_api_impl::validation_code_by_hash::<Runtime>(hash)
        }

        fn on_chain_votes() -> Option<ScrapedOnChainVotes<Hash>> {
            parachains_runtime_api_impl::on_chain_votes::<Runtime>()
        }

        fn submit_pvf_check_statement(
            stmt: primitives::PvfCheckStatement,
            signature: primitives::ValidatorSignature
        ) {
            parachains_runtime_api_impl::submit_pvf_check_statement::<Runtime>(stmt, signature)
        }

        fn pvfs_require_precheck() -> Vec<ValidationCodeHash> {
            parachains_runtime_api_impl::pvfs_require_precheck::<Runtime>()
        }

        fn validation_code_hash(para_id: ParaId, assumption: OccupiedCoreAssumption)
            -> Option<ValidationCodeHash>
        {
            parachains_runtime_api_impl::validation_code_hash::<Runtime>(para_id, assumption)
        }

        fn disputes() -> Vec<(SessionIndex, CandidateHash, DisputeState<BlockNumber>)> {
            parachains_runtime_api_impl::get_session_disputes::<Runtime>()
        }

        fn unapplied_slashes(
        ) -> Vec<(SessionIndex, CandidateHash, slashing::PendingSlashes)> {
            parachains_runtime_api_impl::unapplied_slashes::<Runtime>()
        }

        fn key_ownership_proof(
            validator_id: ValidatorId,
        ) -> Option<slashing::OpaqueKeyOwnershipProof> {
            use parity_scale_codec::Encode;

            Historical::prove((PARACHAIN_KEY_TYPE_ID, validator_id))
                .map(|p| p.encode())
                .map(slashing::OpaqueKeyOwnershipProof::new)
        }

        fn submit_report_dispute_lost(
            dispute_proof: slashing::DisputeProof,
            key_ownership_proof: slashing::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            parachains_runtime_api_impl::submit_unsigned_slashing_report::<Runtime>(
                dispute_proof,
                key_ownership_proof,
            )
        }

        fn minimum_backing_votes() -> u32 {
            parachains_runtime_api_impl::minimum_backing_votes::<Runtime>()
        }

        fn para_backing_state(para_id: ParaId) -> Option<primitives::vstaging::async_backing::BackingState> {
            parachains_runtime_api_impl::backing_state::<Runtime>(para_id)
        }

        fn async_backing_params() -> primitives::AsyncBackingParams {
            parachains_runtime_api_impl::async_backing_params::<Runtime>()
        }

        fn approval_voting_params() -> ApprovalVotingParams {
            parachains_runtime_api_impl::approval_voting_params::<Runtime>()
        }

        fn disabled_validators() -> Vec<ValidatorIndex> {
            parachains_runtime_api_impl::disabled_validators::<Runtime>()
        }

        fn node_features() -> NodeFeatures {
            parachains_runtime_api_impl::node_features::<Runtime>()
        }

        fn claim_queue() -> BTreeMap<CoreIndex, VecDeque<ParaId>> {
            parachains_runtime_api_impl::claim_queue::<Runtime>()
        }

        fn candidates_pending_availability(para_id: ParaId) -> Vec<CommittedCandidateReceiptV2<Hash>> {
            parachains_runtime_api_impl::candidates_pending_availability::<Runtime>(para_id)
        }
    }

    #[api_version(5)]
    impl beefy_primitives::BeefyApi<Block, BeefyId> for Runtime {
        fn beefy_genesis() -> Option<BlockNumber> {
            pallet_beefy::GenesisBlock::<Runtime>::get()
        }

        fn validator_set() -> Option<beefy_primitives::ValidatorSet<BeefyId>> {
            Beefy::validator_set()
        }

        fn submit_report_double_voting_unsigned_extrinsic(
            equivocation_proof: beefy_primitives::DoubleVotingProof<
                BlockNumber,
                BeefyId,
                BeefySignature,
            >,
            key_owner_proof: beefy_primitives::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            let key_owner_proof = key_owner_proof.decode()?;

            Beefy::submit_unsigned_double_voting_report(
                equivocation_proof,
                key_owner_proof,
            )
        }

        fn submit_report_fork_voting_unsigned_extrinsic(
            equivocation_proof:
                beefy_primitives::ForkVotingProof<
                    <Block as BlockT>::Header,
                    BeefyId,
                    sp_runtime::OpaqueValue
                >,
            key_owner_proof: beefy_primitives::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            Beefy::submit_unsigned_fork_voting_report(
                equivocation_proof.try_into()?,
                key_owner_proof.decode()?,
            )
        }

        fn submit_report_future_block_voting_unsigned_extrinsic(
            equivocation_proof: beefy_primitives::FutureBlockVotingProof<BlockNumber, BeefyId>,
            key_owner_proof: beefy_primitives::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            Beefy::submit_unsigned_future_block_voting_report(
                equivocation_proof,
                key_owner_proof.decode()?,
            )
        }

        fn generate_key_ownership_proof(
            _set_id: beefy_primitives::ValidatorSetId,
            authority_id: BeefyId,
        ) -> Option<beefy_primitives::OpaqueKeyOwnershipProof> {
            Historical::prove((beefy_primitives::KEY_TYPE, authority_id))
                .map(|p| p.encode())
                .map(beefy_primitives::OpaqueKeyOwnershipProof::new)
        }

        fn generate_ancestry_proof(
            prev_block_number: BlockNumber,
            best_known_block_number: Option<BlockNumber>,
        ) -> Option<sp_runtime::OpaqueValue> {
            use beefy_primitives::AncestryHelper;

            BeefyMmrLeaf::generate_proof(prev_block_number, best_known_block_number)
                .map(|p| p.encode())
                .map(sp_runtime::OpaqueValue::new)
        }
    }

    #[api_version(2)]
    impl mmr::MmrApi<Block, mmr::Hash, BlockNumber> for Runtime {
        fn mmr_root() -> Result<mmr::Hash, mmr::Error> {
            Ok(pallet_mmr::RootHash::<Runtime>::get())
        }

        fn mmr_leaf_count() -> Result<mmr::LeafIndex, mmr::Error> {
            Ok(pallet_mmr::NumberOfLeaves::<Runtime>::get())
        }

        fn generate_proof(
            block_numbers: Vec<BlockNumber>,
            best_known_block_number: Option<BlockNumber>,
        ) -> Result<(Vec<mmr::EncodableOpaqueLeaf>, mmr::LeafProof<mmr::Hash>), mmr::Error> {
            Mmr::generate_proof(block_numbers, best_known_block_number).map(
                |(leaves, proof)| {
                    (
                        leaves
                            .into_iter()
                            .map(|leaf| mmr::EncodableOpaqueLeaf::from_leaf(&leaf))
                            .collect(),
                        proof,
                    )
                },
            )
        }

        fn verify_proof(leaves: Vec<mmr::EncodableOpaqueLeaf>, proof: mmr::LeafProof<mmr::Hash>)
            -> Result<(), mmr::Error>
        {
            let leaves = leaves.into_iter().map(|leaf|
                leaf.into_opaque_leaf()
                .try_decode()
                .ok_or(mmr::Error::Verify)).collect::<Result<Vec<mmr::Leaf>, mmr::Error>>()?;
            Mmr::verify_leaves(leaves, proof)
        }

        fn verify_proof_stateless(
            root: mmr::Hash,
            leaves: Vec<mmr::EncodableOpaqueLeaf>,
            proof: mmr::LeafProof<mmr::Hash>
        ) -> Result<(), mmr::Error> {
            let nodes = leaves.into_iter().map(|leaf|mmr::DataOrHash::Data(leaf.into_opaque_leaf())).collect();
            pallet_mmr::verify_leaves_proof::<mmr::Hashing, _>(root, nodes, proof)
        }
    }

    impl fg_primitives::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> Vec<(GrandpaId, u64)> {
            Grandpa::grandpa_authorities()
        }

        fn current_set_id() -> fg_primitives::SetId {
            Grandpa::current_set_id()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            equivocation_proof: fg_primitives::EquivocationProof<
                <Block as BlockT>::Hash,
                sp_runtime::traits::NumberFor<Block>,
            >,
            key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            let key_owner_proof = key_owner_proof.decode()?;

            Grandpa::submit_unsigned_equivocation_report(
                equivocation_proof,
                key_owner_proof,
            )
        }

        fn generate_key_ownership_proof(
            _set_id: fg_primitives::SetId,
            authority_id: fg_primitives::AuthorityId,
        ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
            use parity_scale_codec::Encode;

            Historical::prove((fg_primitives::KEY_TYPE, authority_id))
                .map(|p| p.encode())
                .map(fg_primitives::OpaqueKeyOwnershipProof::new)
        }
    }

    impl babe_primitives::BabeApi<Block> for Runtime {
        fn configuration() -> babe_primitives::BabeConfiguration {
            let epoch_config = Babe::epoch_config().unwrap_or(BABE_GENESIS_EPOCH_CONFIG);
            babe_primitives::BabeConfiguration {
                slot_duration: Babe::slot_duration(),
                epoch_length: EpochDurationInBlocks::get().into(),
                c: epoch_config.c,
                authorities: Babe::authorities().to_vec(),
                randomness: Babe::randomness(),
                allowed_slots: epoch_config.allowed_slots,
            }
        }

        fn current_epoch_start() -> babe_primitives::Slot {
            Babe::current_epoch_start()
        }

        fn current_epoch() -> babe_primitives::Epoch {
            Babe::current_epoch()
        }

        fn next_epoch() -> babe_primitives::Epoch {
            Babe::next_epoch()
        }

        fn generate_key_ownership_proof(
            _slot: babe_primitives::Slot,
            authority_id: babe_primitives::AuthorityId,
        ) -> Option<babe_primitives::OpaqueKeyOwnershipProof> {
            use parity_scale_codec::Encode;

            Historical::prove((babe_primitives::KEY_TYPE, authority_id))
                .map(|p| p.encode())
                .map(babe_primitives::OpaqueKeyOwnershipProof::new)
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            equivocation_proof: babe_primitives::EquivocationProof<<Block as BlockT>::Header>,
            key_owner_proof: babe_primitives::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            let key_owner_proof = key_owner_proof.decode()?;

            Babe::submit_unsigned_equivocation_report(
                equivocation_proof,
                key_owner_proof,
            )
        }
    }

    impl authority_discovery_primitives::AuthorityDiscoveryApi<Block> for Runtime {
        fn authorities() -> Vec<AuthorityDiscoveryId> {
            parachains_runtime_api_impl::relevant_authority_ids::<Runtime>()
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

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
        fn account_nonce(account: AccountId) -> Nonce {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
        Block,
        Balance,
    > for Runtime {
        fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl pallet_beefy_mmr::BeefyMmrApi<Block, Hash> for RuntimeApi {
        fn authority_set_proof() -> beefy_primitives::mmr::BeefyAuthoritySet<Hash> {
            BeefyMmrLeaf::authority_set_proof()
        }

        fn next_authority_set_proof() -> beefy_primitives::mmr::BeefyNextAuthoritySet<Hash> {
            BeefyMmrLeaf::next_authority_set_proof()
        }
    }

    #[cfg(feature = "try-runtime")]
    impl frame_try_runtime::TryRuntime<Block> for Runtime {
        fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
            log::info!("try-runtime::on_runtime_upgrade dancelight.");
            let weight = Executive::try_runtime_upgrade(checks).unwrap();
            (weight, BlockWeights::get().max_block)
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

    impl pallet_registrar_runtime_api::RegistrarApi<Block, ParaId> for Runtime {
        /// Return the registered para ids
        fn registered_paras() -> Vec<ParaId> {
            // We should return the container-chains for the session in which we are kicking in
            // We could potentially predict whether the next block will yield a session change as in dancebox but this
            // is innecesary: the dancelight blocks are being produced by validators, and therefore it should never
            // stall because of any collator-rotation. Therefore it suffices for collators to predict the chain in
            // which they have to collate after the session-change block.
            let session_index = Session::current_index();
            let container_chains = ContainerRegistrar::session_container_chains(session_index);
            let mut para_ids = vec![];
            para_ids.extend(container_chains.parachains);
            para_ids.extend(container_chains.parathreads.into_iter().map(|(para_id, _)| para_id));

            para_ids
        }

        /// Fetch genesis data for this para id
        fn genesis_data(para_id: ParaId) -> Option<ContainerChainGenesisData> {
            ContainerRegistrar::para_genesis_data(para_id)
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
            ContainerRegistrar::parathread_params(para_id).map(|params| {
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

    impl pallet_external_validators_rewards_runtime_api::ExternalValidatorsRewardsApi<Block, AccountId, EraIndex> for Runtime
        where
        EraIndex: parity_scale_codec::Codec,
    {
        fn generate_rewards_merkle_proof(account_id: AccountId, era_index: EraIndex) -> Option<MerkleProof> {
            ExternalValidatorsRewards::generate_rewards_merkle_proof(account_id, era_index)
        }

        fn verify_rewards_merkle_proof(merkle_proof: MerkleProof) -> bool {
            ExternalValidatorsRewards::verify_rewards_merkle_proof(merkle_proof)
        }
    }

    impl dp_consensus::TanssiAuthorityAssignmentApi<Block, NimbusId> for Runtime {
        /// Return the current authorities assigned to a given paraId
        fn para_id_authorities(para_id: ParaId) -> Option<Vec<NimbusId>> {
            let session_index = Session::current_index();
            let assigned_authorities = TanssiAuthorityAssignment::collator_container_chain(session_index)?;

            assigned_authorities.container_chains.get(&para_id).cloned()
        }

        /// Return the paraId assigned to a given authority
        fn check_para_id_assignment(authority: NimbusId) -> Option<ParaId> {
            let session_index = Session::current_index();
            let assigned_authorities = TanssiAuthorityAssignment::collator_container_chain(session_index)?;
            // This self_para_id is used to detect assignments to orchestrator, in this runtime the
            // orchestrator will always be empty so we can set it to any value
            let self_para_id = 0u32.into();

            assigned_authorities.para_id_of(&authority, self_para_id)
        }

        /// Return the paraId assigned to a given authority on the next session.
        /// On session boundary this returns the same as `check_para_id_assignment`.
        fn check_para_id_assignment_next_session(authority: NimbusId) -> Option<ParaId> {
            let session_index = Session::current_index() + 1;
            let assigned_authorities = TanssiAuthorityAssignment::collator_container_chain(session_index)?;
            // This self_para_id is used to detect assignments to orchestrator, in this runtime the
            // orchestrator will always be empty so we can set it to any value
            let self_para_id = 0u32.into();

            assigned_authorities.para_id_of(&authority, self_para_id)
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

    impl snowbridge_outbound_queue_runtime_api::OutboundQueueApi<Block, Balance> for Runtime {
        fn prove_message(leaf_index: u64) -> Option<MerkleProof> {
            snowbridge_pallet_outbound_queue::api::prove_message::<Runtime>(leaf_index)
        }

        fn calculate_fee(command: Command, parameters: Option<PricingParameters<Balance>>) -> Fee<Balance> {
            snowbridge_pallet_outbound_queue::api::calculate_fee::<Runtime>(command, parameters)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn benchmark_metadata(extra: bool) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {
            use frame_benchmarking::{Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;

            use frame_system_benchmarking::Pallet as SystemBench;
            use frame_benchmarking::baseline::Pallet as Baseline;

            use pallet_xcm::benchmarking::Pallet as PalletXcmExtrinsicsBenchmark;

            let mut list = Vec::<BenchmarkList>::new();
            list_benchmarks!(list, extra);

            let storage_info = AllPalletsWithSystem::storage_info();
            (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig,
        ) -> Result<
            Vec<frame_benchmarking::BenchmarkBatch>,
            alloc::string::String,
        > {
            use frame_support::traits::WhitelistedStorageKeys;
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, BenchmarkError};
            use frame_system_benchmarking::Pallet as SystemBench;
            use frame_benchmarking::baseline::Pallet as Baseline;
            use pallet_xcm::benchmarking::Pallet as PalletXcmExtrinsicsBenchmark;
            use sp_storage::TrackedStorageKey;
            use xcm::latest::prelude::*;
            use xcm_config::{
                AssetHub, LocalCheckAccount, LocationConverter, TokenLocation, XcmConfig,
            };

            parameter_types! {
                pub ExistentialDepositAsset: Option<Asset> = Some((
                    TokenLocation::get(),
                    ExistentialDeposit::get()
                ).into());
                pub AssetHubParaId: ParaId = dancelight_runtime_constants::system_parachain::ASSET_HUB_ID.into();
                pub const RandomParaId: ParaId = ParaId::new(43211234);
            }

            impl frame_system_benchmarking::Config for Runtime {}
            impl frame_benchmarking::baseline::Config for Runtime {}
            impl pallet_xcm::benchmarking::Config for Runtime {
                type DeliveryHelper = (
                    runtime_common::xcm_sender::ToParachainDeliveryHelper<
                        XcmConfig,
                        ExistentialDepositAsset,
                        xcm_config::PriceForChildParachainDelivery,
                        AssetHubParaId,
                        (),
                    >,
                    runtime_common::xcm_sender::ToParachainDeliveryHelper<
                        XcmConfig,
                        ExistentialDepositAsset,
                        xcm_config::PriceForChildParachainDelivery,
                        RandomParaId,
                        (),
                    >
                );

                fn reachable_dest() -> Option<Location> {
                    Some(crate::xcm_config::AssetHub::get())
                }

                fn teleportable_asset_and_dest() -> Option<(Asset, Location)> {
                    // Relay/native token can be teleported to/from AH.
                    Some((
                        Asset {
                            fun: Fungible(ExistentialDeposit::get()),
                            id: AssetId(Here.into())
                        },
                        crate::xcm_config::AssetHub::get(),
                    ))
                }

                fn reserve_transferable_asset_and_dest() -> Option<(Asset, Location)> {
                    // Relay can reserve transfer native token to some random parachain.
                    Some((
                        Asset {
                            fun: Fungible(ExistentialDeposit::get()),
                            id: AssetId(Here.into())
                        },
                        Parachain(RandomParaId::get().into()).into(),
                    ))
                }

                fn set_up_complex_asset_transfer(
                ) -> Option<(Assets, u32, Location, Box<dyn FnOnce()>)> {
                    // Relay supports only native token, either reserve transfer it to non-system parachains,
                    // or teleport it to system parachain. Use the teleport case for benchmarking as it's
                    // slightly heavier.
                    // Relay/native token can be teleported to/from AH.
                    let native_location = Here.into();
                    let dest = crate::xcm_config::AssetHub::get();
                    pallet_xcm::benchmarking::helpers::native_teleport_as_asset_transfer::<Runtime>(
                        native_location,
                        dest
                    )
                }

                fn get_asset() -> Asset {
                    Asset {
                        id: AssetId(Location::here()),
                        fun: Fungible(ExistentialDeposit::get()),
                    }
                }
            }
            impl pallet_xcm_benchmarks::Config for Runtime {
                type XcmConfig = XcmConfig;
                type AccountIdConverter = LocationConverter;
                type DeliveryHelper = runtime_common::xcm_sender::ToParachainDeliveryHelper<
                    XcmConfig,
                    ExistentialDepositAsset,
                    xcm_config::PriceForChildParachainDelivery,
                    AssetHubParaId,
                    (),
                >;
                fn valid_destination() -> Result<Location, BenchmarkError> {
                    Ok(AssetHub::get())
                }
                fn worst_case_holding(_depositable_count: u32) -> Assets {
                    // Dancelight only knows about STAR
                    vec![Asset{
                        id: AssetId(TokenLocation::get()),
                        fun: Fungible(1_000_000 * UNITS),
                    }].into()
                }
            }

            parameter_types! {
                pub TrustedTeleporter: Option<(Location, Asset)> = Some((
                    AssetHub::get(),
                    Asset { fun: Fungible(1 * UNITS), id: AssetId(TokenLocation::get()) },
                ));
                pub TrustedReserve: Option<(Location, Asset)> = None;
            }

            impl pallet_xcm_benchmarks::fungible::Config for Runtime {
                type TransactAsset = Balances;

                type CheckedAccount = LocalCheckAccount;
                type TrustedTeleporter = TrustedTeleporter;
                type TrustedReserve = TrustedReserve;

                fn get_asset() -> Asset {
                    Asset {
                        id: AssetId(TokenLocation::get()),
                        fun: Fungible(1 * UNITS),
                    }
                }
            }

            impl pallet_xcm_benchmarks::generic::Config for Runtime {
                type TransactAsset = Balances;
                type RuntimeCall = RuntimeCall;

                fn worst_case_response() -> (u64, Response) {
                    (0u64, Response::Version(Default::default()))
                }

                fn worst_case_asset_exchange() -> Result<(Assets, Assets), BenchmarkError> {
                    // Dancelight doesn't support asset exchanges
                    Err(BenchmarkError::Skip)
                }

                fn universal_alias() -> Result<(Location, Junction), BenchmarkError> {
                    // The XCM executor of Dancelight doesn't have a configured `UniversalAliases`
                    Err(BenchmarkError::Skip)
                }

                fn transact_origin_and_runtime_call() -> Result<(Location, RuntimeCall), BenchmarkError> {
                    Ok((AssetHub::get(), frame_system::Call::remark_with_event { remark: vec![] }.into()))
                }

                fn subscribe_origin() -> Result<Location, BenchmarkError> {
                    Ok(AssetHub::get())
                }

                fn claimable_asset() -> Result<(Location, Location, Assets), BenchmarkError> {
                    let origin = AssetHub::get();
                    let assets: Assets = (AssetId(TokenLocation::get()), 1_000 * UNITS).into();
                    let ticket = Location { parents: 0, interior: Here };
                    Ok((origin, ticket, assets))
                }

                fn fee_asset() -> Result<Asset, BenchmarkError> {
                    Ok(Asset {
                        id: AssetId(TokenLocation::get()),
                        fun: Fungible(1_000_000 * UNITS),
                    })
                }

                fn unlockable_asset() -> Result<(Location, Location, Asset), BenchmarkError> {
                    // Dancelight doesn't support asset locking
                    Err(BenchmarkError::Skip)
                }

                fn export_message_origin_and_destination(
                ) -> Result<(Location, NetworkId, InteriorLocation), BenchmarkError> {
                    // Dancelight doesn't support exporting messages
                    Err(BenchmarkError::Skip)
                }

                fn alias_origin() -> Result<(Location, Location), BenchmarkError> {
                    // The XCM executor of Dancelight doesn't have a configured `Aliasers`
                    Err(BenchmarkError::Skip)
                }
            }

            let mut whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();
            let treasury_key = frame_system::Account::<Runtime>::hashed_key_for(Treasury::account_id());
            whitelist.push(treasury_key.to_vec().into());

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);

            add_benchmarks!(params, batches);

            Ok(batches)
        }
    }

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            build_state::<RuntimeGenesisConfig>(config)
        }

        fn get_preset(id: &Option<PresetId>) -> Option<Vec<u8>> {
            get_preset::<RuntimeGenesisConfig>(id, &genesis_config_presets::get_preset)
        }

        fn preset_names() -> Vec<PresetId> {
            vec![
                PresetId::from("local_testnet"),
                PresetId::from("development"),
                PresetId::from("staging_testnet"),
                PresetId::from("wococo_local_testnet"),
                PresetId::from("versi_local_testnet"),
            ]
        }
    }
}

pub struct OwnApplySession;
impl tanssi_initializer::ApplyNewSession<Runtime> for OwnApplySession {
    fn apply_new_session(
        _changed: bool,
        session_index: u32,
        _all_validators: Vec<(AccountId, nimbus_primitives::NimbusId)>,
        _queued: Vec<(AccountId, nimbus_primitives::NimbusId)>,
    ) {
        // Order is same as in tanssi
        // 1.
        // We first initialize Configuration
        CollatorConfiguration::initializer_on_new_session(&session_index);
        // 2. Second, registrar
        ContainerRegistrar::initializer_on_new_session(&session_index);

        let invulnerables = TanssiInvulnerables::invulnerables().to_vec();
        let candidates_staking =
            pallet_pooled_staking::SortedEligibleCandidates::<Runtime>::get().to_vec();
        // Max number of collators is set in pallet_configuration
        let target_session_index = session_index.saturating_add(1);
        let max_collators = <CollatorConfiguration as GetHostConfiguration<u32>>::max_collators(
            target_session_index,
        );
        let next_collators: Vec<_> = invulnerables
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

        // Queue next session keys.
        let queued_amalgamated = next_collators
            .into_iter()
            .filter_map(|a| {
                let k = pallet_session::NextKeys::<Runtime>::get(&a)?;

                Some((a, k.nimbus))
            })
            .collect::<Vec<_>>();

        let next_collators_accounts = queued_amalgamated.iter().map(|(a, _)| a.clone()).collect();

        // 3. AuthorityMapping
        if session_index.is_zero() {
            // On the genesis sesion index we need to store current as well
            TanssiAuthorityMapping::initializer_on_new_session(&session_index, &queued_amalgamated);
        }
        // Otherwise we always store one sessio ahead
        // IMPORTANT: this changes with respect to dancebox/flashbox because here we dont have
        // the current collators and their keys.
        // In contrast, we have the keys for the validators only
        TanssiAuthorityMapping::initializer_on_new_session(
            &(session_index + 1),
            &queued_amalgamated,
        );

        // 4. CollatorAssignment
        // Unlike in tanssi, where the input to this function are the correct
        // queued keys & collators, here we get the input refers to the validators
        // and not the collators. Therefore we need to do a similar thing that
        // pallet-session does but in this function
        // This is, get the collators, fetch their respective keys, and queue the
        // assignment

        // CollatorAssignment
        let assignments = TanssiCollatorAssignment::initializer_on_new_session(
            &session_index,
            next_collators_accounts,
        );

        // 5. AuthorityAssignment
        let queued_id_to_nimbus_map = queued_amalgamated.iter().cloned().collect();
        TanssiAuthorityAssignment::initializer_on_new_session(
            &session_index,
            &queued_id_to_nimbus_map,
            &assignments.next_assignment,
        );
        // 6. InactivityTracking
        InactivityTracking::process_ended_session();
    }
}
parameter_types! {
    pub MockParaId :ParaId = 0u32.into();
}

impl tanssi_initializer::Config for Runtime {
    type SessionIndex = u32;

    /// The identifier type for an authority.
    type AuthorityId = nimbus_primitives::NimbusId;

    type SessionHandler = OwnApplySession;
}

pub struct BabeCurrentBlockRandomnessGetter;
impl BabeCurrentBlockRandomnessGetter {
    fn get_block_randomness() -> Option<[u8; 32]> {
        // In a relay context we get block randomness from Babe's AuthorVrfRandomness
        Babe::author_vrf_randomness()
    }

    fn get_block_randomness_mixed(subject: &[u8]) -> Option<Hash> {
        Self::get_block_randomness()
            .map(|random_hash| mix_randomness::<Runtime>(random_hash, subject))
    }
}

/// Combines the vrf output of the previous block with the provided subject.
/// This ensures that the randomness will be different on different pallets, as long as the subject is different.
pub fn mix_randomness<T: frame_system::Config>(vrf_output: [u8; 32], subject: &[u8]) -> T::Hash {
    let mut digest = Vec::new();
    digest.extend_from_slice(vrf_output.as_ref());
    digest.extend_from_slice(subject);

    T::Hashing::hash(digest.as_slice())
}

/// Read full_rotation_period from pallet_configuration
pub struct ConfigurationCollatorRotationSessionPeriod;

impl Get<u32> for ConfigurationCollatorRotationSessionPeriod {
    fn get() -> u32 {
        CollatorConfiguration::config().full_rotation_period
    }
}

// CollatorAssignment expects to set up the rotation's randomness seed on the
// on_finalize hook of the block prior to the actual session change.
// So should_end_session should be true on the last block of the current session
pub struct BabeGetRandomnessForNextBlock;
impl GetRandomnessForNextBlock<u32> for BabeGetRandomnessForNextBlock {
    fn should_end_session(n: u32) -> bool {
        // Check if next slot there is a session change
        n != 1 && {
            let diff = Babe::current_slot()
                .saturating_add(1u64)
                .saturating_sub(Babe::current_epoch_start());
            *diff >= Babe::current_epoch().duration
        }
    }

    fn get_randomness() -> [u8; 32] {
        let block_number = System::block_number();
        let random_seed = if block_number != 0 {
            if let Some(random_hash) = {
                BabeCurrentBlockRandomnessGetter::get_block_randomness_mixed(b"CollatorAssignment")
            } {
                // Return random_hash as a [u8; 32] instead of a Hash
                let mut buf = [0u8; 32];
                let len = sp_std::cmp::min(32, random_hash.as_ref().len());
                buf[..len].copy_from_slice(&random_hash.as_ref()[..len]);

                buf
            } else {
                // If there is no randomness return [0; 32]
                [0; 32]
            }
        } else {
            // In block 0 (genesis) there is no randomness
            [0; 32]
        };

        random_seed
    }
}

// Randomness trait
impl frame_support::traits::Randomness<Hash, BlockNumber> for BabeCurrentBlockRandomnessGetter {
    fn random(subject: &[u8]) -> (Hash, BlockNumber) {
        let block_number = frame_system::Pallet::<Runtime>::block_number();
        let randomness = Self::get_block_randomness_mixed(subject).unwrap_or_default();

        (randomness, block_number)
    }
}

pub struct ParaIdAssignmentHooksImpl;

impl ParaIdAssignmentHooksImpl {
    fn charge_para_ids_internal(
        blocks_per_session: BlockNumber,
        para_id: ParaId,
        currently_assigned: &BTreeSet<ParaId>,
        maybe_tip: &Option<BalanceOf<Runtime>>,
    ) -> Result<Weight, DispatchError> {
        use frame_support::traits::Currency;
        type ServicePaymentCurrency = <Runtime as pallet_services_payment::Config>::Currency;

        // Check if the container chain has enough credits for a session assignments
        let maybe_assignment_imbalance =
            if pallet_services_payment::Pallet::<Runtime>::burn_collator_assignment_free_credit_for_para(&para_id).is_err() {
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
        let blocks_per_session = EpochDurationInBlocks::get();
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
        let blocks_per_session = EpochDurationInBlocks::get();
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

        let blocks_per_session = EpochDurationInBlocks::get();
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

fn host_config_at_session(
    session_index_to_consider: SessionIndex,
) -> HostConfiguration<BlockNumber> {
    let active_config = runtime_parachains::configuration::ActiveConfig::<Runtime>::get();

    let mut pending_configs = runtime_parachains::configuration::PendingConfigs::<Runtime>::get();

    // We are not making any assumptions about number of configurations existing in pending config
    // storage item.
    // First remove any pending configs greater than session index in consideration
    pending_configs = pending_configs
        .into_iter()
        .filter(|element| element.0 <= session_index_to_consider)
        .collect::<Vec<_>>();
    // Reverse sorting by the session index
    pending_configs.sort_by(|a, b| b.0.cmp(&a.0));

    if pending_configs.is_empty() {
        active_config
    } else {
        // We will take first pending config which should be as close to the session index as possible
        pending_configs
            .first()
            .expect("already checked for emptiness above")
            .1
            .clone()
    }
}

pub struct GetCoreAllocationConfigurationImpl;

impl Get<Option<CoreAllocationConfiguration>> for GetCoreAllocationConfigurationImpl {
    fn get() -> Option<CoreAllocationConfiguration> {
        // We do not have to check for session ending as new session always starts at block initialization which means
        // whenever this is called, we are either in old session or in start of a one
        // as on block initialization epoch index have been incremented and by extension session has been changed.
        let session_index_to_consider = Session::current_index() + 1;

        let max_parachain_percentage =
            CollatorConfiguration::max_parachain_cores_percentage(session_index_to_consider)
                .unwrap_or(Perbill::from_percent(50));

        let config_to_consider = host_config_at_session(session_index_to_consider);

        Some(CoreAllocationConfiguration {
            core_count: config_to_consider.scheduler_params.num_cores,
            max_parachain_percentage,
        })
    }
}

impl pallet_collator_assignment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type HostConfiguration = CollatorConfiguration;
    type ContainerChains = ContainerRegistrar;
    type SessionIndex = u32;
    type SelfParaId = MockParaId;
    type ShouldRotateAllCollators =
        RotateCollatorsEveryNSessions<ConfigurationCollatorRotationSessionPeriod>;
    type GetRandomnessForNextBlock = BabeGetRandomnessForNextBlock;
    type RemoveInvulnerables = ();
    type ParaIdAssignmentHooks = ParaIdAssignmentHooksImpl;
    type CollatorAssignmentTip = ServicesPayment;
    type Currency = Balances;
    type ForceEmptyOrchestrator = ConstBool<true>;
    type CoreAllocationConfiguration = GetCoreAllocationConfigurationImpl;
    type WeightInfo = weights::pallet_collator_assignment::SubstrateWeight<Runtime>;
}

impl pallet_authority_assignment::Config for Runtime {
    type SessionIndex = u32;
    type AuthorityId = nimbus_primitives::NimbusId;
}

impl pallet_authority_mapping::Config for Runtime {
    type SessionIndex = u32;
    type SessionRemovalBoundary = ConstU32<3>;
    type AuthorityId = nimbus_primitives::NimbusId;
}

#[cfg(feature = "runtime-benchmarks")]
mod benchmark_helpers {
    use {
        super::*,
        babe_primitives::{
            digests::{PreDigest, SecondaryPlainPreDigest},
            BABE_ENGINE_ID,
        },
        frame_support::traits::Hooks,
        sp_runtime::{Digest, DigestItem},
    };

    fn end_block() {
        Babe::on_finalize(System::block_number());
        Session::on_finalize(System::block_number());
        Grandpa::on_finalize(System::block_number());
        TransactionPayment::on_finalize(System::block_number());
        Initializer::on_finalize(System::block_number());
        ContainerRegistrar::on_finalize(System::block_number());
        TanssiCollatorAssignment::on_finalize(System::block_number());
    }

    pub fn insert_authorities_and_slot_digests(slot: u64) {
        let pre_digest = Digest {
            logs: vec![DigestItem::PreRuntime(
                BABE_ENGINE_ID,
                PreDigest::SecondaryPlain(SecondaryPlainPreDigest {
                    slot: slot.into(),
                    authority_index: 0,
                })
                .encode(),
            )],
        };

        System::reset_events();
        System::initialize(
            &(System::block_number() + 1),
            &System::parent_hash(),
            &pre_digest,
        );
    }

    pub fn current_slot() -> u64 {
        Babe::current_slot().into()
    }

    fn start_block() {
        insert_authorities_and_slot_digests(current_slot() + 1);

        // Initialize the new block
        Babe::on_initialize(System::block_number());
        ContainerRegistrar::on_initialize(System::block_number());
        Session::on_initialize(System::block_number());
        Initializer::on_initialize(System::block_number());
        TanssiCollatorAssignment::on_initialize(System::block_number());
        InflationRewards::on_initialize(System::block_number());
    }

    pub fn session_to_block(n: u32) -> u32 {
        // let block_number = flashbox_runtime::Period::get() * n;
        let block_number = Babe::current_epoch().duration.saturated_into::<u32>() * n;

        // Add 1 because the block that emits the NewSession event cannot contain any extrinsics,
        // so this is the first block of the new session that can actually be used
        block_number + 1
    }

    pub fn run_to_block(n: u32) {
        while System::block_number() < n {
            run_block();
        }
    }

    pub fn run_block() {
        end_block();

        start_block()
    }

    pub fn run_to_session(n: u32) {
        run_to_block(session_to_block(n));
    }
}

#[cfg(all(test, feature = "try-runtime"))]
mod remote_tests {
    use {
        super::*,
        frame_try_runtime::{runtime_decl_for_try_runtime::TryRuntime, UpgradeCheckSelect},
        remote_externalities::{
            Builder, Mode, OfflineConfig, OnlineConfig, SnapshotConfig, Transport,
        },
        std::env::var,
    };

    #[tokio::test]
    async fn run_migrations() {
        if var("RUN_MIGRATION_TESTS").is_err() {
            return;
        }

        sp_tracing::try_init_simple();
        let transport: Transport = var("WS")
            .unwrap_or("wss://dancelight-rpc.polkadot.io:443".to_string())
            .into();
        let maybe_state_snapshot: Option<SnapshotConfig> = var("SNAP").map(|s| s.into()).ok();
        let mut ext = Builder::<Block>::default()
            .mode(if let Some(state_snapshot) = maybe_state_snapshot {
                Mode::OfflineOrElseOnline(
                    OfflineConfig {
                        state_snapshot: state_snapshot.clone(),
                    },
                    OnlineConfig {
                        transport,
                        state_snapshot: Some(state_snapshot),
                        ..Default::default()
                    },
                )
            } else {
                Mode::Online(OnlineConfig {
                    transport,
                    ..Default::default()
                })
            })
            .build()
            .await
            .unwrap();
        ext.execute_with(|| Runtime::on_runtime_upgrade(UpgradeCheckSelect::PreAndPost));
    }
}
