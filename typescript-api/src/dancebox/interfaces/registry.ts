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
    DanceboxRuntimeHoldReason,
    DanceboxRuntimeOriginCaller,
    DanceboxRuntimeProxyType,
    DanceboxRuntimeRuntime,
    DanceboxRuntimeSessionKeys,
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
    PalletConfigurationCall,
    PalletConfigurationError,
    PalletConfigurationHostConfiguration,
    PalletInitializerBufferedSessionChange,
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
    PolkadotPrimitivesV5AbridgedHostConfiguration,
    PolkadotPrimitivesV5AbridgedHrmpChannel,
    PolkadotPrimitivesV5PersistedValidationData,
    PolkadotPrimitivesV5UpgradeGoAhead,
    PolkadotPrimitivesV5UpgradeRestriction,
    PolkadotPrimitivesVstagingAsyncBackingParams,
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
    StagingXcmDoubleEncoded,
    StagingXcmV2BodyId,
    StagingXcmV2BodyPart,
    StagingXcmV2Instruction,
    StagingXcmV2Junction,
    StagingXcmV2MultiAsset,
    StagingXcmV2MultiLocation,
    StagingXcmV2MultiassetAssetId,
    StagingXcmV2MultiassetAssetInstance,
    StagingXcmV2MultiassetFungibility,
    StagingXcmV2MultiassetMultiAssetFilter,
    StagingXcmV2MultiassetMultiAssets,
    StagingXcmV2MultiassetWildFungibility,
    StagingXcmV2MultiassetWildMultiAsset,
    StagingXcmV2MultilocationJunctions,
    StagingXcmV2NetworkId,
    StagingXcmV2OriginKind,
    StagingXcmV2Response,
    StagingXcmV2TraitsError,
    StagingXcmV2WeightLimit,
    StagingXcmV2Xcm,
    StagingXcmV3Instruction,
    StagingXcmV3Junction,
    StagingXcmV3JunctionBodyId,
    StagingXcmV3JunctionBodyPart,
    StagingXcmV3JunctionNetworkId,
    StagingXcmV3Junctions,
    StagingXcmV3MaybeErrorCode,
    StagingXcmV3MultiAsset,
    StagingXcmV3MultiLocation,
    StagingXcmV3MultiassetAssetId,
    StagingXcmV3MultiassetAssetInstance,
    StagingXcmV3MultiassetFungibility,
    StagingXcmV3MultiassetMultiAssetFilter,
    StagingXcmV3MultiassetMultiAssets,
    StagingXcmV3MultiassetWildFungibility,
    StagingXcmV3MultiassetWildMultiAsset,
    StagingXcmV3PalletInfo,
    StagingXcmV3QueryResponseInfo,
    StagingXcmV3Response,
    StagingXcmV3TraitsError,
    StagingXcmV3TraitsOutcome,
    StagingXcmV3WeightLimit,
    StagingXcmV3Xcm,
    StagingXcmVersionedAssetId,
    StagingXcmVersionedMultiAssets,
    StagingXcmVersionedMultiLocation,
    StagingXcmVersionedResponse,
    StagingXcmVersionedXcm,
    TpAuthorNotingInherentOwnParachainInherentData,
    TpCollatorAssignmentAssignedCollatorsAccountId32,
    TpCollatorAssignmentAssignedCollatorsPublic,
    TpContainerChainGenesisDataContainerChainGenesisData,
    TpContainerChainGenesisDataContainerChainGenesisDataItem,
    TpContainerChainGenesisDataProperties,
    TpContainerChainGenesisDataTokenMetadata,
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
        DanceboxRuntimeHoldReason: DanceboxRuntimeHoldReason;
        DanceboxRuntimeOriginCaller: DanceboxRuntimeOriginCaller;
        DanceboxRuntimeProxyType: DanceboxRuntimeProxyType;
        DanceboxRuntimeRuntime: DanceboxRuntimeRuntime;
        DanceboxRuntimeSessionKeys: DanceboxRuntimeSessionKeys;
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
        PalletConfigurationCall: PalletConfigurationCall;
        PalletConfigurationError: PalletConfigurationError;
        PalletConfigurationHostConfiguration: PalletConfigurationHostConfiguration;
        PalletInitializerBufferedSessionChange: PalletInitializerBufferedSessionChange;
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
        PolkadotPrimitivesV5AbridgedHostConfiguration: PolkadotPrimitivesV5AbridgedHostConfiguration;
        PolkadotPrimitivesV5AbridgedHrmpChannel: PolkadotPrimitivesV5AbridgedHrmpChannel;
        PolkadotPrimitivesV5PersistedValidationData: PolkadotPrimitivesV5PersistedValidationData;
        PolkadotPrimitivesV5UpgradeGoAhead: PolkadotPrimitivesV5UpgradeGoAhead;
        PolkadotPrimitivesV5UpgradeRestriction: PolkadotPrimitivesV5UpgradeRestriction;
        PolkadotPrimitivesVstagingAsyncBackingParams: PolkadotPrimitivesVstagingAsyncBackingParams;
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
        StagingXcmDoubleEncoded: StagingXcmDoubleEncoded;
        StagingXcmV2BodyId: StagingXcmV2BodyId;
        StagingXcmV2BodyPart: StagingXcmV2BodyPart;
        StagingXcmV2Instruction: StagingXcmV2Instruction;
        StagingXcmV2Junction: StagingXcmV2Junction;
        StagingXcmV2MultiAsset: StagingXcmV2MultiAsset;
        StagingXcmV2MultiLocation: StagingXcmV2MultiLocation;
        StagingXcmV2MultiassetAssetId: StagingXcmV2MultiassetAssetId;
        StagingXcmV2MultiassetAssetInstance: StagingXcmV2MultiassetAssetInstance;
        StagingXcmV2MultiassetFungibility: StagingXcmV2MultiassetFungibility;
        StagingXcmV2MultiassetMultiAssetFilter: StagingXcmV2MultiassetMultiAssetFilter;
        StagingXcmV2MultiassetMultiAssets: StagingXcmV2MultiassetMultiAssets;
        StagingXcmV2MultiassetWildFungibility: StagingXcmV2MultiassetWildFungibility;
        StagingXcmV2MultiassetWildMultiAsset: StagingXcmV2MultiassetWildMultiAsset;
        StagingXcmV2MultilocationJunctions: StagingXcmV2MultilocationJunctions;
        StagingXcmV2NetworkId: StagingXcmV2NetworkId;
        StagingXcmV2OriginKind: StagingXcmV2OriginKind;
        StagingXcmV2Response: StagingXcmV2Response;
        StagingXcmV2TraitsError: StagingXcmV2TraitsError;
        StagingXcmV2WeightLimit: StagingXcmV2WeightLimit;
        StagingXcmV2Xcm: StagingXcmV2Xcm;
        StagingXcmV3Instruction: StagingXcmV3Instruction;
        StagingXcmV3Junction: StagingXcmV3Junction;
        StagingXcmV3JunctionBodyId: StagingXcmV3JunctionBodyId;
        StagingXcmV3JunctionBodyPart: StagingXcmV3JunctionBodyPart;
        StagingXcmV3JunctionNetworkId: StagingXcmV3JunctionNetworkId;
        StagingXcmV3Junctions: StagingXcmV3Junctions;
        StagingXcmV3MaybeErrorCode: StagingXcmV3MaybeErrorCode;
        StagingXcmV3MultiAsset: StagingXcmV3MultiAsset;
        StagingXcmV3MultiLocation: StagingXcmV3MultiLocation;
        StagingXcmV3MultiassetAssetId: StagingXcmV3MultiassetAssetId;
        StagingXcmV3MultiassetAssetInstance: StagingXcmV3MultiassetAssetInstance;
        StagingXcmV3MultiassetFungibility: StagingXcmV3MultiassetFungibility;
        StagingXcmV3MultiassetMultiAssetFilter: StagingXcmV3MultiassetMultiAssetFilter;
        StagingXcmV3MultiassetMultiAssets: StagingXcmV3MultiassetMultiAssets;
        StagingXcmV3MultiassetWildFungibility: StagingXcmV3MultiassetWildFungibility;
        StagingXcmV3MultiassetWildMultiAsset: StagingXcmV3MultiassetWildMultiAsset;
        StagingXcmV3PalletInfo: StagingXcmV3PalletInfo;
        StagingXcmV3QueryResponseInfo: StagingXcmV3QueryResponseInfo;
        StagingXcmV3Response: StagingXcmV3Response;
        StagingXcmV3TraitsError: StagingXcmV3TraitsError;
        StagingXcmV3TraitsOutcome: StagingXcmV3TraitsOutcome;
        StagingXcmV3WeightLimit: StagingXcmV3WeightLimit;
        StagingXcmV3Xcm: StagingXcmV3Xcm;
        StagingXcmVersionedAssetId: StagingXcmVersionedAssetId;
        StagingXcmVersionedMultiAssets: StagingXcmVersionedMultiAssets;
        StagingXcmVersionedMultiLocation: StagingXcmVersionedMultiLocation;
        StagingXcmVersionedResponse: StagingXcmVersionedResponse;
        StagingXcmVersionedXcm: StagingXcmVersionedXcm;
        TpAuthorNotingInherentOwnParachainInherentData: TpAuthorNotingInherentOwnParachainInherentData;
        TpCollatorAssignmentAssignedCollatorsAccountId32: TpCollatorAssignmentAssignedCollatorsAccountId32;
        TpCollatorAssignmentAssignedCollatorsPublic: TpCollatorAssignmentAssignedCollatorsPublic;
        TpContainerChainGenesisDataContainerChainGenesisData: TpContainerChainGenesisDataContainerChainGenesisData;
        TpContainerChainGenesisDataContainerChainGenesisDataItem: TpContainerChainGenesisDataContainerChainGenesisDataItem;
        TpContainerChainGenesisDataProperties: TpContainerChainGenesisDataProperties;
        TpContainerChainGenesisDataTokenMetadata: TpContainerChainGenesisDataTokenMetadata;
    } // InterfaceTypes
} // declare module
