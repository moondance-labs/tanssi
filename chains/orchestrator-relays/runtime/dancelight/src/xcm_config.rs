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

//! XCM configuration for Dancelight.

use {
    super::{
        parachains_origin,
        weights::{self, xcm::XcmWeight},
        AccountId, AllPalletsWithSystem, Balance, Balances, Dmp, Fellows, ForeignAssets,
        ForeignAssetsCreator, ParaId, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin,
        TransactionByteFee, Treasury, WeightToFee, XcmPallet,
    },
    crate::{governance::StakingAdmin, EthereumSystem, SnowbridgeFeesAccount},
    dancelight_runtime_constants::{
        currency::CENTS,
        snowbridge::{EthereumLocation, EthereumNetwork},
        system_parachain::*,
        DANCELIGHT_GENESIS_HASH,
    },
    frame_support::{
        parameter_types,
        traits::{Contains, Disabled, Equals, Everything, Nothing},
        weights::Weight,
    },
    frame_system::EnsureRoot,
    runtime_common::{
        xcm_sender::{ChildParachainRouter, ExponentialPrice},
        ToAuthor,
    },
    snowbridge_core::{AgentId, ChannelId},
    sp_core::ConstU32,
    sp_runtime::traits::TryConvertInto,
    tanssi_runtime_common::relay::ExporterFeeHandler,
    tp_bridge::{
        container_token_to_ethereum_message_exporter::ContainerEthereumBlobExporter,
        snowbridge_outbound_token_transfer::{EthereumBlobExporter, SnowbrigeTokenTransferRouter},
        snowbridge_outbound_token_transfer_v2::EthereumBlobExporterV2,
        EthereumLocationsConverterFor,
    },
    tp_xcm_commons::{EthereumAssetReserve, NativeAssetReserve},
    xcm::{
        latest::prelude::{AssetId as XcmAssetId, *},
        opaque::latest::WESTEND_GENESIS_HASH,
    },
    xcm_builder::{
        AccountId32Aliases, AllowExplicitUnpaidExecutionFrom, AllowKnownQueryResponses,
        AllowSubscriptionsFrom, AllowTopLevelPaidExecutionFrom, ChildParachainAsNative,
        ChildParachainConvertsVia, ConvertedConcreteId, DescribeAllTerminal, DescribeFamily,
        FixedWeightBounds, FrameTransactionalProcessor, FungibleAdapter, FungiblesAdapter,
        HashedDescription, IsChildSystemParachain, IsConcrete, MintLocation, NoChecking,
        OriginToPluralityVoice, SignedAccountId32AsNative, SignedToAccountId32,
        SovereignSignedViaLocation, TakeWeightCredit, TrailingSetTopicAsId, UsingComponents,
        WeightInfoBounds, WithComputedOrigin, WithUniqueTopic, XcmFeeManagerFromComponents,
    },
    xcm_executor::XcmExecutor,
};

parameter_types! {
    pub TokenLocation: Location = Here.into_location();
    pub RootLocation: Location = Location::here();
    pub const ThisNetwork: NetworkId = NetworkId::ByGenesis(DANCELIGHT_GENESIS_HASH);
    pub UniversalLocation: InteriorLocation = ThisNetwork::get().into();
    pub CheckAccount: AccountId = XcmPallet::check_account();
    pub LocalCheckAccount: (AccountId, MintLocation) = (CheckAccount::get(), MintLocation::Local);
    pub TreasuryAccount: AccountId = Treasury::account_id();
}

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
    // Universal location for benchmarks that need to run through a para-id scenario
    pub UniversalLocationForParaIdBenchmarks: InteriorLocation = [GlobalConsensus(RelayNetwork::get()), Parachain(2000u32)].into();
}

pub type LocationConverter = (
    // We can convert a child parachain using the standard `AccountId` conversion.
    ChildParachainConvertsVia<ParaId, AccountId>,
    // We can directly alias an `AccountId32` into a local account.
    AccountId32Aliases<ThisNetwork, AccountId>,
    // Foreign locations alias into accounts according to a hash of their standard description.
    HashedDescription<AccountId, DescribeFamily<DescribeAllTerminal>>,
    // Ethereum contract sovereign account.
    // (Used to convert ethereum contract locations to sovereign account)
    EthereumLocationsConverterFor<AccountId>,
);

/// Our asset transactor. This is what allows us to interest with the runtime facilities from the
/// point of view of XCM-only concepts like `Location` and `Asset`.
///
/// Ours is only aware of the Balances pallet, which is mapped to `StarLocation`.
pub type LocalAssetTransactor = FungibleAdapter<
    // Use this currency:
    Balances,
    // Use this currency when it is a fungible asset matching the given location or name:
    IsConcrete<TokenLocation>,
    // We can convert the Locations with our converter above:
    LocationConverter,
    // Our chain's account ID type (we can't get away without mentioning it explicitly):
    AccountId,
    // We track our teleports in/out to keep total issuance correct.
    LocalCheckAccount,
>;

/// Means for transacting foreign assets from different global consensus.
pub type ForeignFungiblesTransactor = FungiblesAdapter<
    // Use this fungibles implementation:
    ForeignAssets,
    // Use this currency when it is a fungible asset matching the given location or name:
    (ConvertedConcreteId<AssetId, Balance, ForeignAssetsCreator, TryConvertInto>,),
    // Convert an XCM Location into a local account id:
    LocationConverter,
    // Our chain's account ID type (we can't get away without mentioning it explicitly):
    AccountId,
    // We dont need to check teleports here.
    NoChecking,
    // The account to use for tracking teleports.
    CheckingAccount,
>;

/// The means that we convert an the XCM message origin location into a local dispatch origin.
type LocalOriginConverter = (
    // A `Signed` origin of the sovereign account that the original location controls.
    SovereignSignedViaLocation<LocationConverter, RuntimeOrigin>,
    // A child parachain, natively expressed, has the `Parachain` origin.
    ChildParachainAsNative<parachains_origin::Origin, RuntimeOrigin>,
    // The AccountId32 location type can be expressed natively as a `Signed` origin.
    SignedAccountId32AsNative<ThisNetwork, RuntimeOrigin>,
);

parameter_types! {
    /// The amount of weight an XCM operation takes. This is a safe overestimate.
    pub const BaseXcmWeight: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);
    /// The asset ID for the asset that we use to pay for message delivery fees.
    pub FeeAssetId: XcmAssetId = XcmAssetId(TokenLocation::get());
    /// The base fee for the message delivery fees.
    pub const BaseDeliveryFee: u128 = CENTS.saturating_mul(3);
}

pub type PriceForChildParachainDelivery =
    ExponentialPrice<FeeAssetId, BaseDeliveryFee, TransactionByteFee, Dmp>;

/// The XCM router. When we want to send an XCM message, we use this type. It amalgamates all of our
/// individual routers.
pub type XcmRouter = WithUniqueTopic<(
    // Use DMP to communicate with child parachains.
    ChildParachainRouter<Runtime, XcmPallet, PriceForChildParachainDelivery>,
    // Send Ethereum-native tokens back to Ethereum V2.
    SnowbrigeTokenTransferRouter<SnowbridgeExporterv2, UniversalLocation>,
    // Send Ethereum-native tokens back to Ethereum.
    SnowbrigeTokenTransferRouter<SnowbridgeExporter, UniversalLocation>,
)>;

parameter_types! {
    pub AssetHub: Location = Parachain(ASSET_HUB_ID).into_location();
    pub const RelayNetwork: NetworkId = NetworkId::ByGenesis(WESTEND_GENESIS_HASH);
    pub const MaxInstructions: u32 = 100;
    pub const MaxAssetsIntoHolding: u32 = 64;
}

pub struct OnlyParachains;
impl Contains<Location> for OnlyParachains {
    fn contains(loc: &Location) -> bool {
        matches!(loc.unpack(), (0, [Parachain(_)]))
    }
}

pub struct LocalPlurality;
impl Contains<Location> for LocalPlurality {
    fn contains(loc: &Location) -> bool {
        matches!(loc.unpack(), (0, [Plurality { .. }]))
    }
}

/// The barriers one of which must be passed for an XCM message to be executed.
pub type Barrier = TrailingSetTopicAsId<(
    // Weight that is paid for may be consumed.
    TakeWeightCredit,
    // Expected responses are OK.
    AllowKnownQueryResponses<XcmPallet>,
    WithComputedOrigin<
        (
            // If the message is one that immediately attempts to pay for execution, then allow it.
            AllowTopLevelPaidExecutionFrom<Everything>,
            // Messages coming from system parachains need not pay for execution.
            AllowExplicitUnpaidExecutionFrom<IsChildSystemParachain<ParaId>>,
            // Subscriptions for version tracking are OK.
            AllowSubscriptionsFrom<OnlyParachains>,
        ),
        UniversalLocation,
        ConstU32<8>,
    >,
)>;

/// Locations that will not be charged fees in the executor, neither for execution nor delivery.
/// We only waive fees for system functions, which these locations represent.
pub type WaivedLocations = Equals<RootLocation>;
pub type XcmWeigher = WeightInfoBounds<XcmWeight<RuntimeCall>, RuntimeCall, MaxInstructions>;

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    type RuntimeCall = RuntimeCall;
    type XcmSender = XcmRouter;
    type AssetTransactor = (LocalAssetTransactor, ForeignFungiblesTransactor);
    type OriginConverter = LocalOriginConverter;
    type IsReserve = (
        NativeAssetReserve,
        EthereumAssetReserve<EthereumLocation, EthereumNetwork>,
    );
    type IsTeleporter = ();
    type UniversalLocation = UniversalLocation;
    type Barrier = Barrier;
    type Weigher = XcmWeigher;
    type Trader =
        UsingComponents<WeightToFee, TokenLocation, AccountId, Balances, ToAuthor<Runtime>>;
    type ResponseHandler = XcmPallet;
    type AssetTrap = XcmPallet;
    type AssetLocker = ();
    type AssetExchanger = ();
    type AssetClaims = XcmPallet;
    type SubscriptionService = XcmPallet;
    type PalletInstancesInfo = AllPalletsWithSystem;
    type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
    type FeeManager = XcmFeeManagerFromComponents<
        WaivedLocations,
        ExporterFeeHandler<Self::AssetTransactor, SnowbridgeFeesAccount, SnowbridgeFeesAccount>,
    >;
    type MessageExporter = ContainerToSnowbridgeMessageExporter;
    type UniversalAliases = Nothing;
    type CallDispatcher = RuntimeCall;
    type SafeCallFilter = Everything;
    type Aliasers = Nothing;
    type TransactionalProcessor = FrameTransactionalProcessor;
    type HrmpNewChannelOpenRequestHandler = ();
    type HrmpChannelAcceptedHandler = ();
    type HrmpChannelClosingHandler = ();
    type XcmRecorder = ();
    type XcmEventEmitter = XcmPallet;
}

parameter_types! {
    pub const CollectiveBodyId: BodyId = BodyId::Unit;
    // StakingAdmin pluralistic body.
    pub const StakingAdminBodyId: BodyId = BodyId::Defense;
    // Fellows pluralistic body.
    pub const FellowsBodyId: BodyId = BodyId::Technical;
}

/// Type to convert an `Origin` type value into a `Location` value which represents an interior
/// location of this chain.
pub type LocalOriginToLocation = (
    // And a usual Signed origin to be used in XCM as a corresponding AccountId32
    SignedToAccountId32<RuntimeOrigin, AccountId, ThisNetwork>,
);

/// Type to convert the `StakingAdmin` origin to a Plurality `Location` value.
pub type StakingAdminToPlurality =
    OriginToPluralityVoice<RuntimeOrigin, StakingAdmin, StakingAdminBodyId>;

/// Type to convert the Fellows origin to a Plurality `Location` value.
pub type FellowsToPlurality = OriginToPluralityVoice<RuntimeOrigin, Fellows, FellowsBodyId>;

/// Type to convert a pallet `Origin` type value into a `Location` value which represents an
/// interior location of this chain for a destination chain.
pub type LocalPalletOriginToLocation = (
    // StakingAdmin origin to be used in XCM as a corresponding Plurality `Location` value.
    StakingAdminToPlurality,
    // Fellows origin to be used in XCM as a corresponding Plurality `Location` value.
    FellowsToPlurality,
);

impl pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    // Note that this configuration of `SendXcmOrigin` is different from the one present in
    // production.
    type SendXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmRouter = XcmRouter;
    // Anyone can execute XCM messages locally.
    type ExecuteXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmExecuteFilter = Everything;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type XcmTeleportFilter = Nothing;
    // Anyone is able to use reserve transfers regardless of who they are and what they want to
    // transfer.
    type XcmReserveTransferFilter = Everything;
    type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
    type UniversalLocation = UniversalLocation;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
    type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
    type Currency = Balances;
    type CurrencyMatcher = IsConcrete<TokenLocation>;
    type TrustedLockers = ();
    type SovereignAccountOf = LocationConverter;
    type MaxLockers = ConstU32<8>;
    type MaxRemoteLockConsumers = ConstU32<0>;
    type RemoteLockConsumerIdentifier = ();
    type WeightInfo = weights::pallet_xcm::SubstrateWeight<Runtime>;
    type AdminOrigin = EnsureRoot<AccountId>;
    type AuthorizedAliasConsideration = Disabled;
}

parameter_types! {
    // TODO: revisit these values in the future
    pub const ForeignAssetsAssetDeposit: Balance = 0;
    pub const ForeignAssetsAssetAccountDeposit: Balance = 0;
    pub const ForeignAssetsApprovalDeposit: Balance = 0;
    pub const ForeignAssetsAssetsStringLimit: u32 = 50;
    pub const ForeignAssetsMetadataDepositBase: Balance = 0;
    pub const ForeignAssetsMetadataDepositPerByte: Balance = 0;
    pub CheckingAccount: AccountId = XcmPallet::check_account();
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
    type Holder = ();
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

parameter_types! {
    pub SnowbridgeChannelInfo: Option<(ChannelId, AgentId)> =
        pallet_ethereum_token_transfers::CurrentChannelInfo::<Runtime>::get()
            .map(|x| (x.channel_id, x.agent_id));

    pub const MinV2Reward: u128 = 1u128;
    pub MinSnowbridgeV2Reward: Asset = (TokenLocation::get(), MinV2Reward::get()).into();
}

/// Exports message to the Ethereum Gateway contract.
pub type SnowbridgeExporter = EthereumBlobExporter<
    UniversalLocation,
    EthereumNetwork,
    snowbridge_pallet_outbound_queue::Pallet<Runtime>,
    EthereumSystem,
    SnowbridgeChannelInfo,
>;

/// Exports message to the Ethereum Gateway contract.
pub type SnowbridgeExporterv2 = EthereumBlobExporterV2<
    UniversalLocation,
    EthereumNetwork,
    snowbridge_pallet_outbound_queue_v2::Pallet<Runtime>,
    EthereumSystem,
    MinSnowbridgeV2Reward,
>;

/// Exports message to the Ethereum Gateway contract.
pub type ContainerToSnowbridgeMessageExporter = ContainerEthereumBlobExporter<
    UniversalLocation,
    EthereumNetwork,
    EthereumLocation,
    snowbridge_pallet_outbound_queue::Pallet<Runtime>,
    EthereumSystem,
    SnowbridgeChannelInfo,
>;

/// Exports message to the Ethereum Gateway contract.
/// TODO:CHANGE
pub type ContainerToSnowbridgeMessageExporterV2 = ContainerEthereumBlobExporter<
    UniversalLocation,
    EthereumNetwork,
    EthereumLocation,
    snowbridge_pallet_outbound_queue_v2::Pallet<Runtime>,
    EthereumSystem,
    SnowbridgeChannelInfo,
>;
