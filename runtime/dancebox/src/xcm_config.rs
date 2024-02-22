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
    super::{
        weights::xcm::XcmWeight as XcmGenericWeights, AccountId, AllPalletsWithSystem, AssetRate,
        Balance, Balances, ForeignAssetsCreator, MaintenanceMode, MessageQueue, ParachainInfo,
        ParachainSystem, PolkadotXcm, Runtime, RuntimeBlockWeights, RuntimeCall, RuntimeEvent,
        RuntimeOrigin, WeightToFee, XcmpQueue,
    },
    cumulus_primitives_core::{AggregateMessageOrigin, ParaId},
    frame_support::{
        parameter_types,
        traits::{Everything, Nothing, PalletInfoAccess, TransformOrigin},
        weights::Weight,
    },
    frame_system::EnsureRoot,
    pallet_xcm::XcmPassthrough,
    parachains_common::message_queue::{NarrowOriginToSibling, ParaIdToSibling},
    polkadot_runtime_common::xcm_sender::NoPriceForMessageDelivery,
    sp_core::ConstU32,
    sp_runtime::Perbill,
    staging_xcm::latest::prelude::*,
    staging_xcm_builder::{
        AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
        AllowTopLevelPaidExecutionFrom, ConvertedConcreteId, EnsureXcmOrigin, FungibleAdapter,
        IsConcrete, ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative,
        SiblingParachainConvertsVia, SignedAccountId32AsNative, SignedToAccountId32,
        SovereignSignedViaLocation, TakeWeightCredit, UsingComponents, WeightInfoBounds,
        WithComputedOrigin,
    },
    staging_xcm_executor::XcmExecutor,
};

parameter_types! {
    // Self Reserve location, defines the multilocation identifiying the self-reserve currency
    // This is used to match it also against our Balances pallet when we receive such
    // a MultiLocation: (Self Balances pallet index)
    // We use the RELATIVE multilocation
    pub SelfReserve: MultiLocation = MultiLocation {
        parents:0,
        interior: Junctions::X1(
            PalletInstance(<Balances as PalletInfoAccess>::index() as u8)
        )
    };

    // One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
    pub UnitWeightCost: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);

    // TODO: revisit
    pub const RelayNetwork: NetworkId = NetworkId::Westend;

    // The relay chain Origin type
    pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();

    pub const MaxAssetsIntoHolding: u32 = 64;

    /// Maximum number of instructions in a single XCM fragment. A sanity check against
    /// weight caculations getting too crazy.
    pub MaxInstructions: u32 = 100;

    // The universal location within the global consensus system
    pub UniversalLocation: InteriorMultiLocation =
    X2(GlobalConsensus(RelayNetwork::get()), Parachain(ParachainInfo::parachain_id().into()));
}

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
    pub ReachableDest: Option<MultiLocation> = Some(Parent.into());
}

pub type XcmBarrier = (
    // Weight that is paid for may be consumed.
    TakeWeightCredit,
    // Expected responses are OK.
    AllowKnownQueryResponses<PolkadotXcm>,
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

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
    // The parent (Relay-chain) origin converts to the default `AccountId`.
    ParentIsPreset<AccountId>,
    // Sibling parachain origins convert to AccountId via the `ParaId::into`.
    SiblingParachainConvertsVia<polkadot_parachain_primitives::primitives::Sibling, AccountId>,
    // If we receive a MultiLocation of type AccountKey20, just generate a native account
    AccountId32Aliases<RelayNetwork, AccountId>,
    // Generate remote accounts according to polkadot standards
    staging_xcm_builder::HashedDescription<
        AccountId,
        staging_xcm_builder::DescribeFamily<staging_xcm_builder::DescribeAllTerminal>,
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
    // Convert an XCM MultiLocation into a local account id:
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
    cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm, ()>,
    // ..and XCMP to communicate with the sibling chains.
    XcmpQueue,
);

pub struct XcmConfig;
impl staging_xcm_executor::Config for XcmConfig {
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
    type FeeManager = ();
    type MessageExporter = ();
    type UniversalAliases = Nothing;
    type CallDispatcher = RuntimeCall;
    type SafeCallFilter = Everything;
    type Aliasers = Nothing;
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
    type WeightInfo = pallet_xcm::TestWeightInfo;
    type AdminOrigin = EnsureRoot<AccountId>;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ChannelInfo = ParachainSystem;
    type VersionWrapper = PolkadotXcm;
    type ControllerOrigin = EnsureRoot<AccountId>;
    type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
    type WeightInfo = cumulus_pallet_xcmp_queue::weights::SubstrateWeight<Self>;
    type PriceForSiblingDelivery = NoPriceForMessageDelivery<ParaId>;
    // Enqueue XCMP messages from siblings for later processing.
    type XcmpQueue = TransformOrigin<MessageQueue, AggregateMessageOrigin, ParaId, ParaIdToSibling>;
    type MaxInboundSuspended = sp_core::ConstU32<1_000>;
}

impl cumulus_pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
}

parameter_types! {
    pub const RelayOrigin: AggregateMessageOrigin = AggregateMessageOrigin::Parent;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
    type WeightInfo = cumulus_pallet_dmp_queue::weights::SubstrateWeight<Runtime>;
    type RuntimeEvent = RuntimeEvent;
    type DmpSink = frame_support::traits::EnqueueWithOrigin<MessageQueue, RelayOrigin>;
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
    type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
    type CallbackHandle = ();
    type AssetAccountDeposit = ForeignAssetsAssetAccountDeposit;
    type RemoveItemsLimit = frame_support::traits::ConstU32<1000>;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ForeignAssetBenchmarkHelper;
}

impl pallet_foreign_asset_creator::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ForeignAsset = MultiLocation;
    type ForeignAssetCreatorOrigin = EnsureRoot<AccountId>;
    type ForeignAssetModifierOrigin = EnsureRoot<AccountId>;
    type ForeignAssetDestroyerOrigin = EnsureRoot<AccountId>;
    type Fungibles = ForeignAssets;
    type WeightInfo = pallet_foreign_asset_creator::weights::SubstrateWeight<Runtime>;
}

impl pallet_asset_rate::Config for Runtime {
    type CreateOrigin = EnsureRoot<AccountId>;
    type RemoveOrigin = EnsureRoot<AccountId>;
    type UpdateOrigin = EnsureRoot<AccountId>;
    type Currency = Balances;
    type AssetKind = AssetId;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_asset_rate::weights::SubstrateWeight<Runtime>;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ForeignAssetBenchmarkHelper;
}

use {
    crate::ForeignAssets,
    staging_xcm_builder::{FungiblesAdapter, NoChecking},
    staging_xcm_executor::traits::JustTry,
};

/// Means for transacting foreign assets from different global consensus.
pub type ForeignFungiblesTransactor = FungiblesAdapter<
    // Use this fungibles implementation:
    ForeignAssets,
    // Use this currency when it is a fungible asset matching the given location or name:
    (ConvertedConcreteId<AssetId, Balance, ForeignAssetsCreator, JustTry>,),
    // Convert an XCM MultiLocation into a local account id:
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

// TODO: this should probably move to somewhere in the polkadot-sdk repo
pub struct NativeAssetReserve;
impl frame_support::traits::ContainsPair<MultiAsset, MultiLocation> for NativeAssetReserve {
    fn contains(asset: &MultiAsset, origin: &MultiLocation) -> bool {
        log::trace!(target: "xcm::contains", "NativeAssetReserve asset: {:?}, origin: {:?}", asset, origin);
        let reserve = if let Concrete(location) = &asset.id {
            if location.parents == 0 && !matches!(location.first_interior(), Some(Parachain(_))) {
                Some(MultiLocation::here())
            } else {
                location.chain_part()
            }
        } else {
            None
        };

        if let Some(ref reserve) = reserve {
            if reserve == origin {
                return true;
            }
        }
        false
    }
}

pub trait Parse {
    /// Returns the "chain" location part. It could be parent, sibling
    /// parachain, or child parachain.
    fn chain_part(&self) -> Option<MultiLocation>;
    /// Returns "non-chain" location part.
    fn non_chain_part(&self) -> Option<MultiLocation>;
}

impl Parse for MultiLocation {
    fn chain_part(&self) -> Option<MultiLocation> {
        match (self.parents, self.first_interior()) {
            // sibling parachain
            (1, Some(Parachain(id))) => Some(MultiLocation::new(1, X1(Parachain(*id)))),
            // parent
            (1, _) => Some(MultiLocation::parent()),
            // children parachain
            (0, Some(Parachain(id))) => Some(MultiLocation::new(0, X1(Parachain(*id)))),
            _ => None,
        }
    }

    fn non_chain_part(&self) -> Option<MultiLocation> {
        let mut junctions = *self.interior();
        while matches!(junctions.first(), Some(Parachain(_))) {
            let _ = junctions.take_first();
        }

        if junctions != Here {
            Some(MultiLocation::new(0, junctions))
        } else {
            None
        }
    }
}

parameter_types! {
    pub MessageQueueServiceWeight: Weight = Perbill::from_percent(25) * RuntimeBlockWeights::get().max_block;
}

impl pallet_message_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_message_queue::weights::SubstrateWeight<Runtime>;
    #[cfg(feature = "runtime-benchmarks")]
    type MessageProcessor = pallet_message_queue::mock_helpers::NoopMessageProcessor<
        cumulus_primitives_core::AggregateMessageOrigin,
    >;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type MessageProcessor = staging_xcm_builder::ProcessXcmMessage<
        AggregateMessageOrigin,
        XcmExecutor<XcmConfig>,
        RuntimeCall,
    >;
    type Size = u32;
    // The XCMP queue pallet is only ever able to handle the `Sibling(ParaId)` origin:
    type QueueChangeHandler = NarrowOriginToSibling<XcmpQueue>;
    // NarrowOriginToSibling calls XcmpQueue's is_pause if Origin is sibling. Allows all other origins
    type QueuePausedQuery = (MaintenanceMode, NarrowOriginToSibling<XcmpQueue>);
    // TODO verify values
    type HeapSize = sp_core::ConstU32<{ 64 * 1024 }>;
    type MaxStale = sp_core::ConstU32<8>;
    type ServiceWeight = MessageQueueServiceWeight;
}
