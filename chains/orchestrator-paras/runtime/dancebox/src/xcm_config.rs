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

#[cfg(feature = "runtime-benchmarks")]
use crate::{CollatorAssignment, Session, System};
#[cfg(feature = "runtime-benchmarks")]
use pallet_session::ShouldEndSession;
#[cfg(feature = "runtime-benchmarks")]
use sp_std::{collections::btree_map::BTreeMap, vec};
#[cfg(feature = "runtime-benchmarks")]
use tp_traits::GetContainerChainAuthor;
use xcm::latest::WESTEND_GENESIS_HASH;
use {
    super::{
        currency::MICRODANCE, weights::xcm::XcmWeight as XcmGenericWeights, AccountId,
        AllPalletsWithSystem, AssetRate, Balance, Balances, BlockNumber, ForeignAssets,
        ForeignAssetsCreator, MaintenanceMode, MessageQueue, ParachainInfo, ParachainSystem,
        PolkadotXcm, Registrar, Runtime, RuntimeBlockWeights, RuntimeCall, RuntimeEvent,
        RuntimeOrigin, TransactionByteFee, WeightToFee, XcmpQueue,
    },
    crate::{get_para_id_authorities, weights, AuthorNoting},
    cumulus_primitives_core::{AggregateMessageOrigin, ParaId},
    frame_support::{
        parameter_types,
        traits::{Equals, Everything, Nothing, PalletInfoAccess, TransformOrigin},
        weights::Weight,
    },
    frame_system::{pallet_prelude::BlockNumberFor, EnsureRoot},
    nimbus_primitives::NimbusId,
    pallet_xcm::XcmPassthrough,
    pallet_xcm_core_buyer::{
        CheckCollatorValidity, GetParathreadMaxCorePrice, GetParathreadParams, GetPurchaseCoreCall,
        ParaIdIntoAccountTruncating, XCMNotifier,
    },
    parachains_common::message_queue::{NarrowOriginToSibling, ParaIdToSibling},
    parity_scale_codec::{Decode, DecodeWithMemTracking, Encode},
    polkadot_runtime_common::xcm_sender::ExponentialPrice,
    scale_info::TypeInfo,
    sp_consensus_slots::Slot,
    sp_core::{ConstU32, MaxEncodedLen},
    sp_runtime::{transaction_validity::TransactionPriority, Perbill},
    sp_std::vec::Vec,
    tp_traits::ParathreadParams,
    tp_xcm_commons::NativeAssetReserve,
    xcm::latest::prelude::*,
    xcm_builder::{
        AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
        AllowTopLevelPaidExecutionFrom, ConvertedConcreteId, EnsureXcmOrigin, FungibleAdapter,
        FungiblesAdapter, IsConcrete, NoChecking, ParentIsPreset, RelayChainAsNative,
        SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
        SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit, TrailingSetTopicAsId,
        UsingComponents, WeightInfoBounds, WithComputedOrigin, XcmFeeManagerFromComponents,
    },
    xcm_executor::{traits::JustTry, XcmExecutor},
};

parameter_types! {
    // Self Reserve location, defines the multilocation identifiying the self-reserve currency
    // This is used to match it also against our Balances pallet when we receive such
    // a Location: (Self Balances pallet index)
    // We use the RELATIVE multilocation
    pub SelfReserve: Location = Location {
        parents: 0,
        interior: [
            PalletInstance(<Balances as PalletInfoAccess>::index() as u8)
        ].into()
    };

    // One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
    pub UnitWeightCost: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);

    // TODO: revisit
    pub const RelayNetwork: NetworkId = NetworkId::ByGenesis(WESTEND_GENESIS_HASH);

    // The relay chain Origin type
    pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();

    pub const MaxAssetsIntoHolding: u32 = 64;

    /// Maximum number of instructions in a single XCM fragment. A sanity check against
    /// weight caculations getting too crazy.
    pub MaxInstructions: u32 = 100;

    // The universal location within the global consensus system
    pub UniversalLocation: InteriorLocation =
    [GlobalConsensus(RelayNetwork::get()), Parachain(ParachainInfo::parachain_id().into())].into();

    pub const BaseDeliveryFee: u128 = 100 * MICRODANCE;
    pub RootLocation: Location = Location::here();
}

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
    pub ReachableDest: Option<Location> = Some(Parent.into());
}

pub type XcmBarrier = (
    // Weight that is paid for may be consumed.
    TakeWeightCredit,
    // Expected responses are OK.
    TrailingSetTopicAsId<AllowKnownQueryResponses<PolkadotXcm>>,
    WithComputedOrigin<
        (
            // If the message is one that immediately attemps to pay for execution, then allow it.
            AllowTopLevelPaidExecutionFrom<Everything>,
            // Subscriptions for version tracking are OK.
            AllowSubscriptionsFrom<Everything>,
        ),
        UniversalLocation,
        ConstU32<8>,
    >,
);

/// Type for specifying how a `Location` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
    // The parent (Relay-chain) origin converts to the default `AccountId`.
    ParentIsPreset<AccountId>,
    // Sibling parachain origins convert to AccountId via the `ParaId::into`.
    SiblingParachainConvertsVia<polkadot_parachain_primitives::primitives::Sibling, AccountId>,
    // If we receive a Location of type AccountKey20, just generate a native account
    AccountId32Aliases<RelayNetwork, AccountId>,
    // Generate remote accounts according to polkadot standards
    xcm_builder::HashedDescription<
        AccountId,
        xcm_builder::DescribeFamily<xcm_builder::DescribeAllTerminal>,
    >,
);

/// Local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// Means for transacting the native currency on this chain.
pub type CurrencyTransactor = FungibleAdapter<
    // Use this currency:
    Balances,
    // Use this currency when it is a fungible asset matching the given location or name:
    IsConcrete<SelfReserve>,
    // Convert an XCM Location into a local account id:
    LocationToAccountId,
    // Our chain's account ID type (we can't get away without mentioning it explicitly):
    AccountId,
    // We don't track any teleports of `Balances`.
    (),
>;

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
    // Sovereign account converter; this attempts to derive an `AccountId` from the origin location
    // using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
    // foreign chains who want to have a local sovereign account on this chain which they control.
    SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
    // Native converter for Relay-chain (Parent) location; will convert to a `Relay` origin when
    // recognised.
    RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
    // Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
    // recognised.
    SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
    // Native signed account converter; this just converts an `AccountId32` origin into a normal
    // `RuntimeOrigin::Signed` origin of the same 32-byte value.
    SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
    // Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
    XcmPassthrough<RuntimeOrigin>,
);

/// Means for transacting assets on this chain.
pub type AssetTransactors = (CurrencyTransactor, ForeignFungiblesTransactor);
pub type XcmWeigher =
    WeightInfoBounds<XcmGenericWeights<RuntimeCall>, RuntimeCall, MaxInstructions>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
    // Two routers - use UMP to communicate with the relay chain:
    cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm, PriceForParentDelivery>,
    // ..and XCMP to communicate with the sibling chains.
    XcmpQueue,
);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    type RuntimeCall = RuntimeCall;
    type XcmSender = XcmRouter;
    type AssetTransactor = AssetTransactors;
    type OriginConverter = XcmOriginToTransactDispatchOrigin;
    type IsReserve = NativeAssetReserve;
    type IsTeleporter = ();
    type UniversalLocation = UniversalLocation;
    type Barrier = XcmBarrier;
    type Weigher = XcmWeigher;
    // Local token trader only
    // TODO: update once we have a way to do fees
    type Trader = (
        UsingComponents<WeightToFee, SelfReserve, AccountId, Balances, ()>,
        cumulus_primitives_utility::TakeFirstAssetTrader<
            AccountId,
            AssetRateAsMultiplier,
            // Use this currency when it is a fungible asset matching the given location or name:
            (ConvertedConcreteId<AssetId, Balance, ForeignAssetsCreator, JustTry>,),
            ForeignAssets,
            (),
        >,
    );
    type ResponseHandler = PolkadotXcm;
    type AssetTrap = PolkadotXcm;
    type AssetClaims = PolkadotXcm;
    type SubscriptionService = PolkadotXcm;
    type PalletInstancesInfo = AllPalletsWithSystem;
    type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
    type AssetLocker = ();
    type AssetExchanger = ();
    type FeeManager = XcmFeeManagerFromComponents<Equals<RootLocation>, ()>;
    type MessageExporter = ();
    type UniversalAliases = Nothing;
    type CallDispatcher = RuntimeCall;
    type SafeCallFilter = Everything;
    type Aliasers = Nothing;
    type TransactionalProcessor = xcm_builder::FrameTransactionalProcessor;
    type HrmpNewChannelOpenRequestHandler = ();
    type HrmpChannelAcceptedHandler = ();
    type HrmpChannelClosingHandler = ();
    type XcmRecorder = ();
    type XcmEventEmitter = PolkadotXcm;
}

impl pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmRouter = XcmRouter;
    type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmExecuteFilter = Everything;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type XcmTeleportFilter = Nothing;
    type XcmReserveTransferFilter = Everything;
    type Weigher = XcmWeigher;
    type UniversalLocation = UniversalLocation;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
    type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
    type Currency = Balances;
    type CurrencyMatcher = ();
    type TrustedLockers = ();
    type SovereignAccountOf = LocationToAccountId;
    type MaxLockers = ConstU32<8>;
    type MaxRemoteLockConsumers = ConstU32<0>;
    type RemoteLockConsumerIdentifier = ();
    // TODO pallet-xcm weights
    type WeightInfo = weights::pallet_xcm::SubstrateWeight<Runtime>;
    type AdminOrigin = EnsureRoot<AccountId>;
    type AuthorizedAliasConsideration = (); // TODO: revisit this
}

pub type PriceForSiblingParachainDelivery =
    ExponentialPrice<SelfReserve, BaseDeliveryFee, TransactionByteFee, XcmpQueue>;

pub type PriceForParentDelivery =
    ExponentialPrice<SelfReserve, BaseDeliveryFee, TransactionByteFee, ParachainSystem>;

impl cumulus_pallet_xcmp_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ChannelInfo = ParachainSystem;
    type VersionWrapper = PolkadotXcm;
    type ControllerOrigin = EnsureRoot<AccountId>;
    type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
    type WeightInfo = weights::cumulus_pallet_xcmp_queue::SubstrateWeight<Runtime>;
    type PriceForSiblingDelivery = PriceForSiblingParachainDelivery;
    // Enqueue XCMP messages from siblings for later processing.
    type XcmpQueue = TransformOrigin<MessageQueue, AggregateMessageOrigin, ParaId, ParaIdToSibling>;
    type MaxInboundSuspended = sp_core::ConstU32<1_000>;
    type MaxActiveOutboundChannels = ConstU32<128>;
    type MaxPageSize = ConstU32<{ 103 * 1024 }>;
}

impl cumulus_pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
}

parameter_types! {
    // we just reuse the same deposits
    pub const ForeignAssetsAssetDeposit: Balance = 0;
    pub const ForeignAssetsAssetAccountDeposit: Balance = 0;
    pub const ForeignAssetsApprovalDeposit: Balance = 0;
    pub const ForeignAssetsAssetsStringLimit: u32 = 50;
    pub const ForeignAssetsMetadataDepositBase: Balance = 0;
    pub const ForeignAssetsMetadataDepositPerByte: Balance = 0;
    pub CheckingAccount: AccountId = PolkadotXcm::check_account();
}

#[cfg(feature = "runtime-benchmarks")]
/// Simple conversion of `u32` into an `AssetId` for use in benchmarking.
pub struct ForeignAssetBenchmarkHelper;
#[cfg(feature = "runtime-benchmarks")]
impl pallet_assets::BenchmarkHelper<AssetId> for ForeignAssetBenchmarkHelper {
    fn create_asset_id_parameter(id: u32) -> AssetId {
        id.try_into()
            .expect("number too large to create benchmarks")
    }
}
#[cfg(feature = "runtime-benchmarks")]
impl pallet_asset_rate::AssetKindFactory<AssetId> for ForeignAssetBenchmarkHelper {
    fn create_asset_kind(id: u32) -> AssetId {
        id.try_into()
            .expect("number too large to create benchmarks")
    }
}

pub type AssetId = u16;
pub type ForeignAssetsInstance = pallet_assets::Instance1;
impl pallet_assets::Config<ForeignAssetsInstance> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type AssetId = AssetId;
    type AssetIdParameter = AssetId;
    type Currency = Balances;
    type CreateOrigin = frame_support::traits::NeverEnsureOrigin<AccountId>;
    type ForceOrigin = EnsureRoot<AccountId>;
    type AssetDeposit = ForeignAssetsAssetDeposit;
    type MetadataDepositBase = ForeignAssetsMetadataDepositBase;
    type MetadataDepositPerByte = ForeignAssetsMetadataDepositPerByte;
    type ApprovalDeposit = ForeignAssetsApprovalDeposit;
    type StringLimit = ForeignAssetsAssetsStringLimit;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = weights::pallet_assets::SubstrateWeight<Runtime>;
    type CallbackHandle = ();
    type AssetAccountDeposit = ForeignAssetsAssetAccountDeposit;
    type RemoveItemsLimit = frame_support::traits::ConstU32<1000>;
    type Holder = (); // TODO: revisit this
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ForeignAssetBenchmarkHelper;
}

impl pallet_foreign_asset_creator::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ForeignAsset = Location;
    type ForeignAssetCreatorOrigin = EnsureRoot<AccountId>;
    type ForeignAssetModifierOrigin = EnsureRoot<AccountId>;
    type ForeignAssetDestroyerOrigin = EnsureRoot<AccountId>;
    type Fungibles = ForeignAssets;
    type WeightInfo = weights::pallet_foreign_asset_creator::SubstrateWeight<Runtime>;
    type OnForeignAssetCreated = ();
    type OnForeignAssetDestroyed = ();
}

impl pallet_asset_rate::Config for Runtime {
    type CreateOrigin = EnsureRoot<AccountId>;
    type RemoveOrigin = EnsureRoot<AccountId>;
    type UpdateOrigin = EnsureRoot<AccountId>;
    type Currency = Balances;
    type AssetKind = AssetId;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = weights::pallet_asset_rate::SubstrateWeight<Runtime>;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ForeignAssetBenchmarkHelper;
}

/// Means for transacting foreign assets from different global consensus.
pub type ForeignFungiblesTransactor = FungiblesAdapter<
    // Use this fungibles implementation:
    ForeignAssets,
    // Use this currency when it is a fungible asset matching the given location or name:
    (ConvertedConcreteId<AssetId, Balance, ForeignAssetsCreator, JustTry>,),
    // Convert an XCM Location into a local account id:
    LocationToAccountId,
    // Our chain's account ID type (we can't get away without mentioning it explicitly):
    AccountId,
    // We dont need to check teleports here.
    NoChecking,
    // The account to use for tracking teleports.
    CheckingAccount,
>;

/// Multiplier used for dedicated `TakeFirstAssetTrader` with `ForeignAssets` instance.
pub type AssetRateAsMultiplier =
    parachains_common::xcm_config::AssetFeeAsExistentialDepositMultiplier<
        Runtime,
        WeightToFee,
        AssetRate,
        ForeignAssetsInstance,
    >;

parameter_types! {
    pub MessageQueueServiceWeight: Weight = Perbill::from_percent(25) * RuntimeBlockWeights::get().max_block;
}

impl pallet_message_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = weights::pallet_message_queue::SubstrateWeight<Runtime>;
    #[cfg(feature = "runtime-benchmarks")]
    type MessageProcessor = pallet_message_queue::mock_helpers::NoopMessageProcessor<
        cumulus_primitives_core::AggregateMessageOrigin,
    >;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type MessageProcessor =
        xcm_builder::ProcessXcmMessage<AggregateMessageOrigin, XcmExecutor<XcmConfig>, RuntimeCall>;
    type Size = u32;
    // The XCMP queue pallet is only ever able to handle the `Sibling(ParaId)` origin:
    type QueueChangeHandler = NarrowOriginToSibling<XcmpQueue>;
    // NarrowOriginToSibling calls XcmpQueue's is_pause if Origin is sibling. Allows all other origins
    type QueuePausedQuery = (MaintenanceMode, NarrowOriginToSibling<XcmpQueue>);
    // TODO verify values
    type HeapSize = sp_core::ConstU32<{ 64 * 1024 }>;
    type MaxStale = sp_core::ConstU32<8>;
    type ServiceWeight = MessageQueueServiceWeight;
    type IdleMaxServiceWeight = MessageQueueServiceWeight;
}

parameter_types! {
    pub const ParasUnsignedPriority: TransactionPriority = TransactionPriority::MAX;
    pub const XcmBuyExecutionDotRococo: u128 = XCM_BUY_EXECUTION_COST_ROCOCO;
}

pub const XCM_BUY_EXECUTION_COST_ROCOCO: u128 = 70_000_000 + 126_666_399;

pub struct XCMNotifierImpl;

impl XCMNotifier<Runtime> for XCMNotifierImpl {
    fn new_notify_query(
        responder: impl Into<Location>,
        notify: impl Into<RuntimeCall>,
        timeout: BlockNumberFor<Runtime>,
        match_querier: impl Into<Location>,
    ) -> u64 {
        pallet_xcm::Pallet::<Runtime>::new_notify_query(responder, notify, timeout, match_querier)
    }
}

parameter_types! {
    // TODO: used to be 100 but TS tests failed if we set it to 100, previously it used AdditionalTtlForInflightOrders with value 5
    pub const CoreBuyingXCMQueryTtl: BlockNumber = 5;
    pub const AdditionalTtlForInflightOrders: BlockNumber = 5;
    pub const PendingBlockTtl: BlockNumber = 10;
    pub BuyCoreSlotDrift: Slot = Slot::from(5u64);
}

impl pallet_xcm_core_buyer::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;

    type XcmSender = XcmRouter;
    type GetPurchaseCoreCall = EncodedCallToBuyCore;
    type GetParathreadAccountId = ParaIdIntoAccountTruncating;
    type GetParathreadMaxCorePrice = GetMaxCorePriceFromServicesPayment;
    type SelfParaId = parachain_info::Pallet<Runtime>;
    type RelayChain = RelayChain;
    type GetParathreadParams = GetParathreadParamsImpl;
    type CheckCollatorValidity = CheckCollatorValidityImpl;
    type UnsignedPriority = ParasUnsignedPriority;
    type PendingBlocksTtl = PendingBlockTtl;
    type CoreBuyingXCMQueryTtl = CoreBuyingXCMQueryTtl;
    type AdditionalTtlForInflightOrders = AdditionalTtlForInflightOrders;
    type BuyCoreSlotDrift = BuyCoreSlotDrift;
    type UniversalLocation = UniversalLocation;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type XCMNotifier = XCMNotifierImpl;
    type LatestAuthorInfoFetcher = AuthorNoting;
    type SlotBeacon = dp_consensus::AuraDigestSlotBeacon<Runtime>;
    type CollatorPublicKey = NimbusId;
    type WeightInfo = weights::pallet_xcm_core_buyer::SubstrateWeight<Runtime>;
}

pub struct GetParathreadParamsImpl;

impl GetParathreadParams for GetParathreadParamsImpl {
    fn get_parathread_params(para_id: ParaId) -> Option<ParathreadParams> {
        Registrar::parathread_params(para_id)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn set_parathread_params(para_id: ParaId, parathread_params: Option<ParathreadParams>) {
        if let Some(parathread_params) = parathread_params {
            pallet_registrar::ParathreadParams::<Runtime>::insert(para_id, parathread_params);
        } else {
            pallet_registrar::ParathreadParams::<Runtime>::remove(para_id);
        }
    }
}

pub struct CheckCollatorValidityImpl;

impl CheckCollatorValidity<AccountId, NimbusId> for CheckCollatorValidityImpl {
    fn is_valid_collator(para_id: ParaId, public_key: NimbusId) -> bool {
        let maybe_public_keys = get_para_id_authorities(para_id);
        maybe_public_keys.is_some_and(|public_keys| public_keys.contains(&public_key))
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn set_valid_collator(para_id: ParaId, account_id: AccountId, public_key: NimbusId) {
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

        pallet_authority_mapping::AuthorityIdMapping::<Runtime>::insert(
            session_index,
            BTreeMap::from_iter([(public_key, account_id.clone())]),
        );

        CollatorAssignment::set_authors_for_para_id(para_id, vec![account_id]);
    }
}

/// Relay chains supported by pallet_xcm_core_buyer, each relay chain has different
/// pallet indices for pallet_on_demand_assignment_provider
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Encode,
    Decode,
    TypeInfo,
    MaxEncodedLen,
    DecodeWithMemTracking,
)]
pub enum RelayChain {
    #[default]
    Westend,
    Rococo,
}

pub struct EncodedCallToBuyCore;

impl GetPurchaseCoreCall<RelayChain> for EncodedCallToBuyCore {
    fn get_encoded(relay_chain: RelayChain, max_amount: u128, para_id: ParaId) -> Vec<u8> {
        match relay_chain {
            RelayChain::Westend => {
                let call = tanssi_relay_encoder::westend::RelayCall::OnDemandAssignmentProvider(
                    tanssi_relay_encoder::westend::OnDemandAssignmentProviderCall::PlaceOrderAllowDeath {
                        max_amount,
                        para_id,
                    },
                );

                call.encode()
            }
            RelayChain::Rococo => {
                let call = tanssi_relay_encoder::rococo::RelayCall::OnDemandAssignmentProvider(
                    tanssi_relay_encoder::rococo::OnDemandAssignmentProviderCall::PlaceOrderAllowDeath {
                        max_amount,
                        para_id,
                    },
                );

                call.encode()
            }
        }
    }
}

pub struct GetMaxCorePriceFromServicesPayment;

impl GetParathreadMaxCorePrice for GetMaxCorePriceFromServicesPayment {
    fn get_max_core_price(para_id: ParaId) -> Option<u128> {
        pallet_services_payment::MaxCorePrice::<Runtime>::get(para_id)
    }
}
