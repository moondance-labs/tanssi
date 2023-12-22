// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/types/types/registry";

import type {
    CumulusPalletDmpQueueCall,
    CumulusPalletDmpQueueConfigData,
    CumulusPalletDmpQueueError,
    CumulusPalletDmpQueueEvent,
    CumulusPalletDmpQueuePageIndexData,
    CumulusPalletParachainSystemCall,
    CumulusPalletParachainSystemCodeUpgradeAuthorization,
    CumulusPalletParachainSystemError,
    CumulusPalletParachainSystemEvent,
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot,
    CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity,
    CumulusPalletParachainSystemUnincludedSegmentAncestor,
    CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate,
    CumulusPalletParachainSystemUnincludedSegmentSegmentTracker,
    CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth,
    CumulusPalletXcmError,
    CumulusPalletXcmEvent,
    CumulusPalletXcmOrigin,
    CumulusPalletXcmpQueueCall,
    CumulusPalletXcmpQueueError,
    CumulusPalletXcmpQueueEvent,
    CumulusPalletXcmpQueueInboundChannelDetails,
    CumulusPalletXcmpQueueInboundState,
    CumulusPalletXcmpQueueOutboundChannelDetails,
    CumulusPalletXcmpQueueOutboundState,
    CumulusPalletXcmpQueueQueueConfigData,
    CumulusPrimitivesParachainInherentParachainInherentData,
    DanceboxRuntimeOriginCaller,
    DanceboxRuntimeProxyType,
    DanceboxRuntimeRuntime,
    DanceboxRuntimeRuntimeHoldReason,
    DanceboxRuntimeSessionKeys,
    DpCollatorAssignmentAssignedCollatorsAccountId32,
    DpCollatorAssignmentAssignedCollatorsPublic,
    FrameSupportDispatchDispatchClass,
    FrameSupportDispatchDispatchInfo,
    FrameSupportDispatchPays,
    FrameSupportDispatchPerDispatchClassU32,
    FrameSupportDispatchPerDispatchClassWeight,
    FrameSupportDispatchPerDispatchClassWeightsPerClass,
    FrameSupportDispatchRawOrigin,
    FrameSupportTokensMiscBalanceStatus,
    FrameSystemAccountInfo,
    FrameSystemCall,
    FrameSystemError,
    FrameSystemEvent,
    FrameSystemEventRecord,
    FrameSystemExtensionsCheckGenesis,
    FrameSystemExtensionsCheckNonZeroSender,
    FrameSystemExtensionsCheckNonce,
    FrameSystemExtensionsCheckSpecVersion,
    FrameSystemExtensionsCheckTxVersion,
    FrameSystemExtensionsCheckWeight,
    FrameSystemLastRuntimeUpgradeInfo,
    FrameSystemLimitsBlockLength,
    FrameSystemLimitsBlockWeights,
    FrameSystemLimitsWeightsPerClass,
    FrameSystemPhase,
    NimbusPrimitivesNimbusCryptoPublic,
    PalletAuthorInherentCall,
    PalletAuthorInherentError,
    PalletAuthorNotingCall,
    PalletAuthorNotingContainerChainBlockInfo,
    PalletAuthorNotingError,
    PalletAuthorNotingEvent,
    PalletAuthorityAssignmentCall,
    PalletBalancesAccountData,
    PalletBalancesBalanceLock,
    PalletBalancesCall,
    PalletBalancesError,
    PalletBalancesEvent,
    PalletBalancesIdAmount,
    PalletBalancesReasons,
    PalletBalancesReserveData,
    PalletCollatorAssignmentCall,
    PalletCollatorAssignmentEvent,
    PalletConfigurationCall,
    PalletConfigurationError,
    PalletConfigurationHostConfiguration,
    PalletDataPreserversCall,
    PalletDataPreserversError,
    PalletDataPreserversEvent,
    PalletInflationRewardsChainsToRewardValue,
    PalletInflationRewardsEvent,
    PalletInvulnerablesCall,
    PalletInvulnerablesError,
    PalletInvulnerablesEvent,
    PalletMaintenanceModeCall,
    PalletMaintenanceModeError,
    PalletMaintenanceModeEvent,
    PalletMigrationsError,
    PalletMigrationsEvent,
    PalletPooledStakingAllTargetPool,
    PalletPooledStakingCall,
    PalletPooledStakingCandidateEligibleCandidate,
    PalletPooledStakingError,
    PalletPooledStakingEvent,
    PalletPooledStakingHoldReason,
    PalletPooledStakingPendingOperationKey,
    PalletPooledStakingPendingOperationQuery,
    PalletPooledStakingPoolsKey,
    PalletPooledStakingSharesOrStake,
    PalletPooledStakingTargetPool,
    PalletProxyAnnouncement,
    PalletProxyCall,
    PalletProxyError,
    PalletProxyEvent,
    PalletProxyProxyDefinition,
    PalletRegistrarCall,
    PalletRegistrarDepositInfo,
    PalletRegistrarError,
    PalletRegistrarEvent,
    PalletRootTestingCall,
    PalletServicesPaymentCall,
    PalletServicesPaymentError,
    PalletServicesPaymentEvent,
    PalletSessionCall,
    PalletSessionError,
    PalletSessionEvent,
    PalletSudoCall,
    PalletSudoError,
    PalletSudoEvent,
    PalletTimestampCall,
    PalletTransactionPaymentChargeTransactionPayment,
    PalletTransactionPaymentEvent,
    PalletTransactionPaymentReleases,
    PalletTxPauseCall,
    PalletTxPauseError,
    PalletTxPauseEvent,
    PalletUtilityCall,
    PalletUtilityError,
    PalletUtilityEvent,
    PalletXcmCall,
    PalletXcmError,
    PalletXcmEvent,
    PalletXcmOrigin,
    PalletXcmQueryStatus,
    PalletXcmRemoteLockedFungibleRecord,
    PalletXcmVersionMigrationStage,
    ParachainInfoCall,
    PolkadotCorePrimitivesInboundDownwardMessage,
    PolkadotCorePrimitivesInboundHrmpMessage,
    PolkadotCorePrimitivesOutboundHrmpMessage,
    PolkadotParachainPrimitivesPrimitivesXcmpMessageFormat,
    PolkadotPrimitivesV6AbridgedHostConfiguration,
    PolkadotPrimitivesV6AbridgedHrmpChannel,
    PolkadotPrimitivesV6AsyncBackingAsyncBackingParams,
    PolkadotPrimitivesV6PersistedValidationData,
    PolkadotPrimitivesV6UpgradeGoAhead,
    PolkadotPrimitivesV6UpgradeRestriction,
    SpArithmeticArithmeticError,
    SpCoreCryptoKeyTypeId,
    SpCoreEcdsaSignature,
    SpCoreEd25519Signature,
    SpCoreSr25519Public,
    SpCoreSr25519Signature,
    SpCoreVoid,
    SpRuntimeDigest,
    SpRuntimeDigestDigestItem,
    SpRuntimeDispatchError,
    SpRuntimeModuleError,
    SpRuntimeMultiSignature,
    SpRuntimeTokenError,
    SpRuntimeTransactionalError,
    SpTrieStorageProof,
    SpVersionRuntimeVersion,
    SpWeightsRuntimeDbWeight,
    SpWeightsWeightV2Weight,
    StagingXcmV3MultiLocation,
    TpAuthorNotingInherentOwnParachainInherentData,
    TpContainerChainGenesisDataContainerChainGenesisData,
    TpContainerChainGenesisDataContainerChainGenesisDataItem,
    TpContainerChainGenesisDataProperties,
    TpContainerChainGenesisDataTokenMetadata,
    XcmDoubleEncoded,
    XcmV2BodyId,
    XcmV2BodyPart,
    XcmV2Instruction,
    XcmV2Junction,
    XcmV2MultiAsset,
    XcmV2MultiLocation,
    XcmV2MultiassetAssetId,
    XcmV2MultiassetAssetInstance,
    XcmV2MultiassetFungibility,
    XcmV2MultiassetMultiAssetFilter,
    XcmV2MultiassetMultiAssets,
    XcmV2MultiassetWildFungibility,
    XcmV2MultiassetWildMultiAsset,
    XcmV2MultilocationJunctions,
    XcmV2NetworkId,
    XcmV2OriginKind,
    XcmV2Response,
    XcmV2TraitsError,
    XcmV2WeightLimit,
    XcmV2Xcm,
    XcmV3Instruction,
    XcmV3Junction,
    XcmV3JunctionBodyId,
    XcmV3JunctionBodyPart,
    XcmV3JunctionNetworkId,
    XcmV3Junctions,
    XcmV3MaybeErrorCode,
    XcmV3MultiAsset,
    XcmV3MultiassetAssetId,
    XcmV3MultiassetAssetInstance,
    XcmV3MultiassetFungibility,
    XcmV3MultiassetMultiAssetFilter,
    XcmV3MultiassetMultiAssets,
    XcmV3MultiassetWildFungibility,
    XcmV3MultiassetWildMultiAsset,
    XcmV3PalletInfo,
    XcmV3QueryResponseInfo,
    XcmV3Response,
    XcmV3TraitsError,
    XcmV3TraitsOutcome,
    XcmV3WeightLimit,
    XcmV3Xcm,
    XcmVersionedAssetId,
    XcmVersionedMultiAssets,
    XcmVersionedMultiLocation,
    XcmVersionedResponse,
    XcmVersionedXcm,
} from "@polkadot/types/lookup";

declare module "@polkadot/types/types/registry" {
    interface InterfaceTypes {
        CumulusPalletDmpQueueCall: CumulusPalletDmpQueueCall;
        CumulusPalletDmpQueueConfigData: CumulusPalletDmpQueueConfigData;
        CumulusPalletDmpQueueError: CumulusPalletDmpQueueError;
        CumulusPalletDmpQueueEvent: CumulusPalletDmpQueueEvent;
        CumulusPalletDmpQueuePageIndexData: CumulusPalletDmpQueuePageIndexData;
        CumulusPalletParachainSystemCall: CumulusPalletParachainSystemCall;
        CumulusPalletParachainSystemCodeUpgradeAuthorization: CumulusPalletParachainSystemCodeUpgradeAuthorization;
        CumulusPalletParachainSystemError: CumulusPalletParachainSystemError;
        CumulusPalletParachainSystemEvent: CumulusPalletParachainSystemEvent;
        CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot;
        CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity: CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity;
        CumulusPalletParachainSystemUnincludedSegmentAncestor: CumulusPalletParachainSystemUnincludedSegmentAncestor;
        CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate: CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate;
        CumulusPalletParachainSystemUnincludedSegmentSegmentTracker: CumulusPalletParachainSystemUnincludedSegmentSegmentTracker;
        CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth: CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth;
        CumulusPalletXcmError: CumulusPalletXcmError;
        CumulusPalletXcmEvent: CumulusPalletXcmEvent;
        CumulusPalletXcmOrigin: CumulusPalletXcmOrigin;
        CumulusPalletXcmpQueueCall: CumulusPalletXcmpQueueCall;
        CumulusPalletXcmpQueueError: CumulusPalletXcmpQueueError;
        CumulusPalletXcmpQueueEvent: CumulusPalletXcmpQueueEvent;
        CumulusPalletXcmpQueueInboundChannelDetails: CumulusPalletXcmpQueueInboundChannelDetails;
        CumulusPalletXcmpQueueInboundState: CumulusPalletXcmpQueueInboundState;
        CumulusPalletXcmpQueueOutboundChannelDetails: CumulusPalletXcmpQueueOutboundChannelDetails;
        CumulusPalletXcmpQueueOutboundState: CumulusPalletXcmpQueueOutboundState;
        CumulusPalletXcmpQueueQueueConfigData: CumulusPalletXcmpQueueQueueConfigData;
        CumulusPrimitivesParachainInherentParachainInherentData: CumulusPrimitivesParachainInherentParachainInherentData;
        DanceboxRuntimeOriginCaller: DanceboxRuntimeOriginCaller;
        DanceboxRuntimeProxyType: DanceboxRuntimeProxyType;
        DanceboxRuntimeRuntime: DanceboxRuntimeRuntime;
        DanceboxRuntimeRuntimeHoldReason: DanceboxRuntimeRuntimeHoldReason;
        DanceboxRuntimeSessionKeys: DanceboxRuntimeSessionKeys;
        DpCollatorAssignmentAssignedCollatorsAccountId32: DpCollatorAssignmentAssignedCollatorsAccountId32;
        DpCollatorAssignmentAssignedCollatorsPublic: DpCollatorAssignmentAssignedCollatorsPublic;
        FrameSupportDispatchDispatchClass: FrameSupportDispatchDispatchClass;
        FrameSupportDispatchDispatchInfo: FrameSupportDispatchDispatchInfo;
        FrameSupportDispatchPays: FrameSupportDispatchPays;
        FrameSupportDispatchPerDispatchClassU32: FrameSupportDispatchPerDispatchClassU32;
        FrameSupportDispatchPerDispatchClassWeight: FrameSupportDispatchPerDispatchClassWeight;
        FrameSupportDispatchPerDispatchClassWeightsPerClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
        FrameSupportDispatchRawOrigin: FrameSupportDispatchRawOrigin;
        FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
        FrameSystemAccountInfo: FrameSystemAccountInfo;
        FrameSystemCall: FrameSystemCall;
        FrameSystemError: FrameSystemError;
        FrameSystemEvent: FrameSystemEvent;
        FrameSystemEventRecord: FrameSystemEventRecord;
        FrameSystemExtensionsCheckGenesis: FrameSystemExtensionsCheckGenesis;
        FrameSystemExtensionsCheckNonZeroSender: FrameSystemExtensionsCheckNonZeroSender;
        FrameSystemExtensionsCheckNonce: FrameSystemExtensionsCheckNonce;
        FrameSystemExtensionsCheckSpecVersion: FrameSystemExtensionsCheckSpecVersion;
        FrameSystemExtensionsCheckTxVersion: FrameSystemExtensionsCheckTxVersion;
        FrameSystemExtensionsCheckWeight: FrameSystemExtensionsCheckWeight;
        FrameSystemLastRuntimeUpgradeInfo: FrameSystemLastRuntimeUpgradeInfo;
        FrameSystemLimitsBlockLength: FrameSystemLimitsBlockLength;
        FrameSystemLimitsBlockWeights: FrameSystemLimitsBlockWeights;
        FrameSystemLimitsWeightsPerClass: FrameSystemLimitsWeightsPerClass;
        FrameSystemPhase: FrameSystemPhase;
        NimbusPrimitivesNimbusCryptoPublic: NimbusPrimitivesNimbusCryptoPublic;
        PalletAuthorInherentCall: PalletAuthorInherentCall;
        PalletAuthorInherentError: PalletAuthorInherentError;
        PalletAuthorNotingCall: PalletAuthorNotingCall;
        PalletAuthorNotingContainerChainBlockInfo: PalletAuthorNotingContainerChainBlockInfo;
        PalletAuthorNotingError: PalletAuthorNotingError;
        PalletAuthorNotingEvent: PalletAuthorNotingEvent;
        PalletAuthorityAssignmentCall: PalletAuthorityAssignmentCall;
        PalletBalancesAccountData: PalletBalancesAccountData;
        PalletBalancesBalanceLock: PalletBalancesBalanceLock;
        PalletBalancesCall: PalletBalancesCall;
        PalletBalancesError: PalletBalancesError;
        PalletBalancesEvent: PalletBalancesEvent;
        PalletBalancesIdAmount: PalletBalancesIdAmount;
        PalletBalancesReasons: PalletBalancesReasons;
        PalletBalancesReserveData: PalletBalancesReserveData;
        PalletCollatorAssignmentCall: PalletCollatorAssignmentCall;
        PalletCollatorAssignmentEvent: PalletCollatorAssignmentEvent;
        PalletConfigurationCall: PalletConfigurationCall;
        PalletConfigurationError: PalletConfigurationError;
        PalletConfigurationHostConfiguration: PalletConfigurationHostConfiguration;
        PalletDataPreserversCall: PalletDataPreserversCall;
        PalletDataPreserversError: PalletDataPreserversError;
        PalletDataPreserversEvent: PalletDataPreserversEvent;
        PalletInflationRewardsChainsToRewardValue: PalletInflationRewardsChainsToRewardValue;
        PalletInflationRewardsEvent: PalletInflationRewardsEvent;
        PalletInvulnerablesCall: PalletInvulnerablesCall;
        PalletInvulnerablesError: PalletInvulnerablesError;
        PalletInvulnerablesEvent: PalletInvulnerablesEvent;
        PalletMaintenanceModeCall: PalletMaintenanceModeCall;
        PalletMaintenanceModeError: PalletMaintenanceModeError;
        PalletMaintenanceModeEvent: PalletMaintenanceModeEvent;
        PalletMigrationsError: PalletMigrationsError;
        PalletMigrationsEvent: PalletMigrationsEvent;
        PalletPooledStakingAllTargetPool: PalletPooledStakingAllTargetPool;
        PalletPooledStakingCall: PalletPooledStakingCall;
        PalletPooledStakingCandidateEligibleCandidate: PalletPooledStakingCandidateEligibleCandidate;
        PalletPooledStakingError: PalletPooledStakingError;
        PalletPooledStakingEvent: PalletPooledStakingEvent;
        PalletPooledStakingHoldReason: PalletPooledStakingHoldReason;
        PalletPooledStakingPendingOperationKey: PalletPooledStakingPendingOperationKey;
        PalletPooledStakingPendingOperationQuery: PalletPooledStakingPendingOperationQuery;
        PalletPooledStakingPoolsKey: PalletPooledStakingPoolsKey;
        PalletPooledStakingSharesOrStake: PalletPooledStakingSharesOrStake;
        PalletPooledStakingTargetPool: PalletPooledStakingTargetPool;
        PalletProxyAnnouncement: PalletProxyAnnouncement;
        PalletProxyCall: PalletProxyCall;
        PalletProxyError: PalletProxyError;
        PalletProxyEvent: PalletProxyEvent;
        PalletProxyProxyDefinition: PalletProxyProxyDefinition;
        PalletRegistrarCall: PalletRegistrarCall;
        PalletRegistrarDepositInfo: PalletRegistrarDepositInfo;
        PalletRegistrarError: PalletRegistrarError;
        PalletRegistrarEvent: PalletRegistrarEvent;
        PalletRootTestingCall: PalletRootTestingCall;
        PalletServicesPaymentCall: PalletServicesPaymentCall;
        PalletServicesPaymentError: PalletServicesPaymentError;
        PalletServicesPaymentEvent: PalletServicesPaymentEvent;
        PalletSessionCall: PalletSessionCall;
        PalletSessionError: PalletSessionError;
        PalletSessionEvent: PalletSessionEvent;
        PalletSudoCall: PalletSudoCall;
        PalletSudoError: PalletSudoError;
        PalletSudoEvent: PalletSudoEvent;
        PalletTimestampCall: PalletTimestampCall;
        PalletTransactionPaymentChargeTransactionPayment: PalletTransactionPaymentChargeTransactionPayment;
        PalletTransactionPaymentEvent: PalletTransactionPaymentEvent;
        PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
        PalletTxPauseCall: PalletTxPauseCall;
        PalletTxPauseError: PalletTxPauseError;
        PalletTxPauseEvent: PalletTxPauseEvent;
        PalletUtilityCall: PalletUtilityCall;
        PalletUtilityError: PalletUtilityError;
        PalletUtilityEvent: PalletUtilityEvent;
        PalletXcmCall: PalletXcmCall;
        PalletXcmError: PalletXcmError;
        PalletXcmEvent: PalletXcmEvent;
        PalletXcmOrigin: PalletXcmOrigin;
        PalletXcmQueryStatus: PalletXcmQueryStatus;
        PalletXcmRemoteLockedFungibleRecord: PalletXcmRemoteLockedFungibleRecord;
        PalletXcmVersionMigrationStage: PalletXcmVersionMigrationStage;
        ParachainInfoCall: ParachainInfoCall;
        PolkadotCorePrimitivesInboundDownwardMessage: PolkadotCorePrimitivesInboundDownwardMessage;
        PolkadotCorePrimitivesInboundHrmpMessage: PolkadotCorePrimitivesInboundHrmpMessage;
        PolkadotCorePrimitivesOutboundHrmpMessage: PolkadotCorePrimitivesOutboundHrmpMessage;
        PolkadotParachainPrimitivesPrimitivesXcmpMessageFormat: PolkadotParachainPrimitivesPrimitivesXcmpMessageFormat;
        PolkadotPrimitivesV6AbridgedHostConfiguration: PolkadotPrimitivesV6AbridgedHostConfiguration;
        PolkadotPrimitivesV6AbridgedHrmpChannel: PolkadotPrimitivesV6AbridgedHrmpChannel;
        PolkadotPrimitivesV6AsyncBackingAsyncBackingParams: PolkadotPrimitivesV6AsyncBackingAsyncBackingParams;
        PolkadotPrimitivesV6PersistedValidationData: PolkadotPrimitivesV6PersistedValidationData;
        PolkadotPrimitivesV6UpgradeGoAhead: PolkadotPrimitivesV6UpgradeGoAhead;
        PolkadotPrimitivesV6UpgradeRestriction: PolkadotPrimitivesV6UpgradeRestriction;
        SpArithmeticArithmeticError: SpArithmeticArithmeticError;
        SpCoreCryptoKeyTypeId: SpCoreCryptoKeyTypeId;
        SpCoreEcdsaSignature: SpCoreEcdsaSignature;
        SpCoreEd25519Signature: SpCoreEd25519Signature;
        SpCoreSr25519Public: SpCoreSr25519Public;
        SpCoreSr25519Signature: SpCoreSr25519Signature;
        SpCoreVoid: SpCoreVoid;
        SpRuntimeDigest: SpRuntimeDigest;
        SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
        SpRuntimeDispatchError: SpRuntimeDispatchError;
        SpRuntimeModuleError: SpRuntimeModuleError;
        SpRuntimeMultiSignature: SpRuntimeMultiSignature;
        SpRuntimeTokenError: SpRuntimeTokenError;
        SpRuntimeTransactionalError: SpRuntimeTransactionalError;
        SpTrieStorageProof: SpTrieStorageProof;
        SpVersionRuntimeVersion: SpVersionRuntimeVersion;
        SpWeightsRuntimeDbWeight: SpWeightsRuntimeDbWeight;
        SpWeightsWeightV2Weight: SpWeightsWeightV2Weight;
        StagingXcmV3MultiLocation: StagingXcmV3MultiLocation;
        TpAuthorNotingInherentOwnParachainInherentData: TpAuthorNotingInherentOwnParachainInherentData;
        TpContainerChainGenesisDataContainerChainGenesisData: TpContainerChainGenesisDataContainerChainGenesisData;
        TpContainerChainGenesisDataContainerChainGenesisDataItem: TpContainerChainGenesisDataContainerChainGenesisDataItem;
        TpContainerChainGenesisDataProperties: TpContainerChainGenesisDataProperties;
        TpContainerChainGenesisDataTokenMetadata: TpContainerChainGenesisDataTokenMetadata;
        XcmDoubleEncoded: XcmDoubleEncoded;
        XcmV2BodyId: XcmV2BodyId;
        XcmV2BodyPart: XcmV2BodyPart;
        XcmV2Instruction: XcmV2Instruction;
        XcmV2Junction: XcmV2Junction;
        XcmV2MultiAsset: XcmV2MultiAsset;
        XcmV2MultiLocation: XcmV2MultiLocation;
        XcmV2MultiassetAssetId: XcmV2MultiassetAssetId;
        XcmV2MultiassetAssetInstance: XcmV2MultiassetAssetInstance;
        XcmV2MultiassetFungibility: XcmV2MultiassetFungibility;
        XcmV2MultiassetMultiAssetFilter: XcmV2MultiassetMultiAssetFilter;
        XcmV2MultiassetMultiAssets: XcmV2MultiassetMultiAssets;
        XcmV2MultiassetWildFungibility: XcmV2MultiassetWildFungibility;
        XcmV2MultiassetWildMultiAsset: XcmV2MultiassetWildMultiAsset;
        XcmV2MultilocationJunctions: XcmV2MultilocationJunctions;
        XcmV2NetworkId: XcmV2NetworkId;
        XcmV2OriginKind: XcmV2OriginKind;
        XcmV2Response: XcmV2Response;
        XcmV2TraitsError: XcmV2TraitsError;
        XcmV2WeightLimit: XcmV2WeightLimit;
        XcmV2Xcm: XcmV2Xcm;
        XcmV3Instruction: XcmV3Instruction;
        XcmV3Junction: XcmV3Junction;
        XcmV3JunctionBodyId: XcmV3JunctionBodyId;
        XcmV3JunctionBodyPart: XcmV3JunctionBodyPart;
        XcmV3JunctionNetworkId: XcmV3JunctionNetworkId;
        XcmV3Junctions: XcmV3Junctions;
        XcmV3MaybeErrorCode: XcmV3MaybeErrorCode;
        XcmV3MultiAsset: XcmV3MultiAsset;
        XcmV3MultiassetAssetId: XcmV3MultiassetAssetId;
        XcmV3MultiassetAssetInstance: XcmV3MultiassetAssetInstance;
        XcmV3MultiassetFungibility: XcmV3MultiassetFungibility;
        XcmV3MultiassetMultiAssetFilter: XcmV3MultiassetMultiAssetFilter;
        XcmV3MultiassetMultiAssets: XcmV3MultiassetMultiAssets;
        XcmV3MultiassetWildFungibility: XcmV3MultiassetWildFungibility;
        XcmV3MultiassetWildMultiAsset: XcmV3MultiassetWildMultiAsset;
        XcmV3PalletInfo: XcmV3PalletInfo;
        XcmV3QueryResponseInfo: XcmV3QueryResponseInfo;
        XcmV3Response: XcmV3Response;
        XcmV3TraitsError: XcmV3TraitsError;
        XcmV3TraitsOutcome: XcmV3TraitsOutcome;
        XcmV3WeightLimit: XcmV3WeightLimit;
        XcmV3Xcm: XcmV3Xcm;
        XcmVersionedAssetId: XcmVersionedAssetId;
        XcmVersionedMultiAssets: XcmVersionedMultiAssets;
        XcmVersionedMultiLocation: XcmVersionedMultiLocation;
        XcmVersionedResponse: XcmVersionedResponse;
        XcmVersionedXcm: XcmVersionedXcm;
    } // InterfaceTypes
} // declare module
