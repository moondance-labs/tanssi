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
        currency::MICROUNIT,
        precompiles::FOREIGN_ASSET_PRECOMPILE_ADDRESS_PREFIX,
        weights::{self, xcm::XcmWeight as XcmGenericWeights},
        AccountId, AllPalletsWithSystem, AssetRate, Balance, Balances, ForeignAssetsCreator,
        MaintenanceMode, MessageQueue, ParachainInfo, ParachainSystem, PolkadotXcm, Runtime,
        RuntimeBlockWeights, RuntimeCall, RuntimeEvent, RuntimeOrigin, TransactionByteFee,
        WeightToFee, XcmpQueue,
    },
    ccp_xcm::SignedToAccountKey20,
    cumulus_primitives_core::{AggregateMessageOrigin, ParaId},
    frame_support::{
        parameter_types,
        traits::{Disabled, Equals, Everything, Nothing, PalletInfoAccess, TransformOrigin},
        weights::Weight,
    },
    frame_system::EnsureRoot,
    pallet_foreign_asset_creator::{
        AssetBalance, AssetId as AssetIdOf, ForeignAssetCreatedHook, ForeignAssetDestroyedHook,
    },
    pallet_xcm::XcmPassthrough,
    pallet_xcm_executor_utils::{
        filters::{IsReserveFilter, IsTeleportFilter},
        DefaultTrustPolicy,
    },
    parachains_common::{
        message_queue::{NarrowOriginToSibling, ParaIdToSibling},
        xcm_config::AssetFeeAsExistentialDepositMultiplier,
    },
    polkadot_runtime_common::xcm_sender::ExponentialPrice,
    sp_core::{ConstU32, H160},
    sp_runtime::Perbill,
    sp_std::vec::Vec,
    xcm::latest::prelude::*,
    xcm_builder::{
        AccountKey20Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
        AllowTopLevelPaidExecutionFrom, ConvertedConcreteId, EnsureXcmOrigin, FungibleAdapter,
        IsConcrete, ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative,
        SiblingParachainConvertsVia, SignedAccountKey20AsNative, SovereignSignedViaLocation,
        TakeWeightCredit, UsingComponents, WeightInfoBounds, WithComputedOrigin,
        XcmFeeManagerFromComponents,
    },
    xcm_executor::XcmExecutor,
    xcm_primitives::AccountIdAssetIdConversion,
};
parameter_types! {
    // Self Reserve location, defines the multilocation identifiying the self-reserve currency
    // This is used to match it also against our Balances pallet when we receive such
    // a Location: (Self Balances pallet index)
    // We use the RELATIVE multilocation
    pub SelfReserve: Location = Location {
        parents:0,
        interior: [
            PalletInstance(<Balances as PalletInfoAccess>::index() as u8)
        ].into()
    };

    // One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
    pub UnitWeightCost: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);

    // TODO: revisit
    pub const RelayNetwork: NetworkId = NetworkId::Polkadot;

    // The relay chain Origin type
    pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();

    pub const MaxAssetsIntoHolding: u32 = 64;

    /// Maximum number of instructions in a single XCM fragment. A sanity check against
    /// weight caculations getting too crazy.
    pub MaxInstructions: u32 = 100;

    // The universal location within the global consensus system
    pub UniversalLocation: InteriorLocation = [GlobalConsensus(RelayNetwork::get()), Parachain(ParachainInfo::parachain_id().into())].into();

    pub const BaseDeliveryFee: u128 = 100 * MICROUNIT;
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

// For benchmarking, we cannot use the describeFamily
// the benchmark is written to be able to convert an AccountId32, but describeFamily prevents this
#[cfg(not(feature = "runtime-benchmarks"))]
type Descriptor = xcm_builder::DescribeFamily<xcm_builder::DescribeAllTerminal>;
#[cfg(feature = "runtime-benchmarks")]
type Descriptor = xcm_builder::DescribeAllTerminal;

/// Type for specifying how a `Location` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
    // The parent (Relay-chain) origin converts to the default `AccountId`.
    ParentIsPreset<AccountId>,
    // Sibling parachain origins convert to AccountId via the `ParaId::into`.
    SiblingParachainConvertsVia<polkadot_parachain_primitives::primitives::Sibling, AccountId>,
    // If we receive a Location of type AccountKey20, just generate a native account
    AccountKey20Aliases<RelayNetwork, AccountId>,
    // Generate remote accounts according to polkadot standards
    xcm_builder::HashedDescription<AccountId, Descriptor>,
);

/// Local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountKey20<RuntimeOrigin, AccountId, RelayNetwork>;

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
    SignedAccountKey20AsNative<RelayNetwork, RuntimeOrigin>,
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
    type IsReserve = IsReserveFilter<Runtime>;
    type IsTeleporter = IsTeleportFilter<Runtime>;
    type UniversalLocation = UniversalLocation;
    type Barrier = XcmBarrier;
    type Weigher = XcmWeigher;
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
    type WeightInfo = weights::pallet_xcm::SubstrateWeight<Runtime>;
    type AdminOrigin = EnsureRoot<AccountId>;
    type AuthorizedAliasConsideration = Disabled;
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
    pub MessageQueueServiceWeight: Weight = Perbill::from_percent(25) * RuntimeBlockWeights::get().max_block;
}

impl pallet_message_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = weights::pallet_message_queue::SubstrateWeight<Self>;
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
    type HeapSize = sp_core::ConstU32<{ 64 * 1024 }>;
    type MaxStale = sp_core::ConstU32<8>;
    type ServiceWeight = MessageQueueServiceWeight;
    type IdleMaxServiceWeight = MessageQueueServiceWeight;
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

// Instruct how to go from an H160 to an AssetID
// We just take the lowest 2 bytes
impl AccountIdAssetIdConversion<AccountId, AssetId> for Runtime {
    /// The way to convert an account to assetId is by ensuring that the prefix is [0xFF, 18]
    /// and by taking the lowest 2 bytes as the assetId
    fn account_to_asset_id(account: AccountId) -> Option<(Vec<u8>, AssetId)> {
        let h160_account: H160 = account.into();
        let mut data = [0u8; 2];
        let (prefix_part, id_part) = h160_account.as_fixed_bytes().split_at(18);
        if prefix_part == FOREIGN_ASSET_PRECOMPILE_ADDRESS_PREFIX {
            data.copy_from_slice(id_part);
            let asset_id: AssetId = u16::from_be_bytes(data);
            Some((prefix_part.to_vec(), asset_id))
        } else {
            None
        }
    }

    // The opposite conversion
    fn asset_id_to_account(prefix: &[u8], asset_id: AssetId) -> AccountId {
        let mut data = [0u8; 20];
        data[0..18].copy_from_slice(prefix);
        data[18..20].copy_from_slice(&asset_id.to_be_bytes());
        AccountId::from(data)
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

pub struct RevertCodePrecompileHook;

impl ForeignAssetCreatedHook<Location, AssetIdOf<Runtime>, AssetBalance<Runtime>>
    for RevertCodePrecompileHook
{
    fn on_asset_created(
        _foreign_asset: &Location,
        asset_id: &AssetIdOf<Runtime>,
        _min_balance: &AssetBalance<Runtime>,
    ) {
        let revert_bytecode = [0x60, 0x00, 0x60, 0x00, 0xFD].to_vec();
        let prefix_slice = [255u8; 18];
        let account_id = Runtime::asset_id_to_account(prefix_slice.as_slice(), *asset_id);

        let _ = pallet_evm::Pallet::<Runtime>::create_account(
            account_id.into(),
            revert_bytecode.clone(),
            None,
        );
    }
}

impl ForeignAssetDestroyedHook<Location, AssetIdOf<Runtime>> for RevertCodePrecompileHook {
    fn on_asset_destroyed(_foreign_asset: &Location, asset_id: &AssetIdOf<Runtime>) {
        let prefix_slice = [255u8; 18];
        let account_id = Runtime::asset_id_to_account(prefix_slice.as_slice(), *asset_id);

        pallet_evm::Pallet::<Runtime>::remove_account(&account_id.into());
    }
}

impl pallet_foreign_asset_creator::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ForeignAsset = Location;
    type ForeignAssetCreatorOrigin = EnsureRoot<AccountId>;
    type ForeignAssetModifierOrigin = EnsureRoot<AccountId>;
    type ForeignAssetDestroyerOrigin = EnsureRoot<AccountId>;
    type Fungibles = ForeignAssets;
    type WeightInfo = weights::pallet_foreign_asset_creator::SubstrateWeight<Runtime>;
    type OnForeignAssetCreated = RevertCodePrecompileHook;
    type OnForeignAssetDestroyed = RevertCodePrecompileHook;
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

parameter_types! {
    pub const TrustPolicyMaxAssets: u32 = 1000;
    pub const AllNativeTrustPolicy: DefaultTrustPolicy = DefaultTrustPolicy::AllNative;
    pub const AllNeverTrustPolicy: DefaultTrustPolicy = DefaultTrustPolicy::Never;
}
impl pallet_xcm_executor_utils::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type TrustPolicyMaxAssets = TrustPolicyMaxAssets;
    type ReserveDefaultTrustPolicy = AllNativeTrustPolicy;
    type SetReserveTrustOrigin = EnsureRoot<AccountId>;
    type TeleportDefaultTrustPolicy = AllNeverTrustPolicy;
    type SetTeleportTrustOrigin = EnsureRoot<AccountId>;
    type WeightInfo = weights::pallet_xcm_executor_utils::SubstrateWeight<Runtime>;
}

use {
    crate::ForeignAssets,
    xcm_builder::{FungiblesAdapter, NoChecking},
    xcm_executor::traits::JustTry,
};

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
    AssetFeeAsExistentialDepositMultiplier<Runtime, WeightToFee, AssetRate, ForeignAssetsInstance>;

#[test]
fn test_asset_id_to_account_conversion() {
    let prefix_slice = [255u8].repeat(18);
    let asset_ids_to_check = vec![0u16, 123u16, 3453u16, 10000u16, 65535u16];
    for current_asset_id in asset_ids_to_check {
        let account_id = Runtime::asset_id_to_account(prefix_slice.as_slice(), current_asset_id);
        assert_eq!(
            account_id.to_string().to_lowercase(),
            String::from("0xffffffffffffffffffffffffffffffffffff")
                + format!("{:04x}", current_asset_id).as_str()
        );
    }
}
