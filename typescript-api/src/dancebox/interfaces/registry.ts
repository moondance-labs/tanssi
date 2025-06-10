// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/types/types/registry";

import type {
    CumulusPalletParachainSystemCall,
    CumulusPalletParachainSystemError,
    CumulusPalletParachainSystemEvent,
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot,
    CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity,
    CumulusPalletParachainSystemUnincludedSegmentAncestor,
    CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate,
    CumulusPalletParachainSystemUnincludedSegmentSegmentTracker,
    CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth,
    CumulusPalletWeightReclaimStorageWeightReclaim,
    CumulusPalletXcmEvent,
    CumulusPalletXcmOrigin,
    CumulusPalletXcmpQueueCall,
    CumulusPalletXcmpQueueError,
    CumulusPalletXcmpQueueEvent,
    CumulusPalletXcmpQueueOutboundChannelDetails,
    CumulusPalletXcmpQueueOutboundState,
    CumulusPalletXcmpQueueQueueConfigData,
    CumulusPrimitivesCoreAggregateMessageOrigin,
    CumulusPrimitivesParachainInherentParachainInherentData,
    DanceboxRuntimeOriginCaller,
    DanceboxRuntimeProxyType,
    DanceboxRuntimeRuntime,
    DanceboxRuntimeRuntimeFreezeReason,
    DanceboxRuntimeRuntimeHoldReason,
    DanceboxRuntimeSessionKeys,
    DanceboxRuntimeXcmConfigRelayChain,
    DpCollatorAssignmentAssignedCollatorsAccountId32,
    DpCollatorAssignmentAssignedCollatorsPublic,
    DpContainerChainGenesisDataContainerChainGenesisData,
    DpContainerChainGenesisDataContainerChainGenesisDataItem,
    DpContainerChainGenesisDataProperties,
    DpContainerChainGenesisDataTokenMetadata,
    FrameMetadataHashExtensionCheckMetadataHash,
    FrameMetadataHashExtensionMode,
    FrameSupportDispatchDispatchClass,
    FrameSupportDispatchPays,
    FrameSupportDispatchPerDispatchClassU32,
    FrameSupportDispatchPerDispatchClassWeight,
    FrameSupportDispatchPerDispatchClassWeightsPerClass,
    FrameSupportDispatchRawOrigin,
    FrameSupportMessagesProcessMessageError,
    FrameSupportPalletId,
    FrameSupportTokensMiscBalanceStatus,
    FrameSupportTokensMiscIdAmountRuntimeFreezeReason,
    FrameSupportTokensMiscIdAmountRuntimeHoldReason,
    FrameSystemAccountInfo,
    FrameSystemCall,
    FrameSystemCodeUpgradeAuthorization,
    FrameSystemDispatchEventInfo,
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
    NimbusPrimitivesNimbusCryptoSignature,
    PalletAssetRateCall,
    PalletAssetRateError,
    PalletAssetRateEvent,
    PalletAssetsAccountStatus,
    PalletAssetsApproval,
    PalletAssetsAssetAccount,
    PalletAssetsAssetDetails,
    PalletAssetsAssetMetadata,
    PalletAssetsAssetStatus,
    PalletAssetsCall,
    PalletAssetsError,
    PalletAssetsEvent,
    PalletAssetsExistenceReason,
    PalletAuthorInherentCall,
    PalletAuthorInherentError,
    PalletAuthorNotingCall,
    PalletAuthorNotingError,
    PalletAuthorNotingEvent,
    PalletAuthorityAssignmentCall,
    PalletBalancesAccountData,
    PalletBalancesAdjustmentDirection,
    PalletBalancesBalanceLock,
    PalletBalancesCall,
    PalletBalancesError,
    PalletBalancesEvent,
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
    PalletDataPreserversHoldReason,
    PalletDataPreserversParaIdsFilter,
    PalletDataPreserversProfile,
    PalletDataPreserversProfileMode,
    PalletDataPreserversRegisteredProfile,
    PalletForeignAssetCreatorCall,
    PalletForeignAssetCreatorError,
    PalletForeignAssetCreatorEvent,
    PalletIdentityAuthorityProperties,
    PalletIdentityCall,
    PalletIdentityError,
    PalletIdentityEvent,
    PalletIdentityJudgement,
    PalletIdentityLegacyIdentityInfo,
    PalletIdentityProvider,
    PalletIdentityRegistrarInfo,
    PalletIdentityRegistration,
    PalletIdentityUsernameInformation,
    PalletInactivityTrackingActivityTrackingStatus,
    PalletInactivityTrackingCall,
    PalletInactivityTrackingError,
    PalletInactivityTrackingEvent,
    PalletInflationRewardsChainsToRewardValue,
    PalletInflationRewardsEvent,
    PalletInvulnerablesCall,
    PalletInvulnerablesError,
    PalletInvulnerablesEvent,
    PalletMaintenanceModeCall,
    PalletMaintenanceModeError,
    PalletMaintenanceModeEvent,
    PalletMessageQueueBookState,
    PalletMessageQueueCall,
    PalletMessageQueueError,
    PalletMessageQueueEvent,
    PalletMessageQueueNeighbours,
    PalletMessageQueuePage,
    PalletMigrationsActiveCursor,
    PalletMigrationsCall,
    PalletMigrationsError,
    PalletMigrationsEvent,
    PalletMigrationsHistoricCleanupSelector,
    PalletMigrationsMigrationCursor,
    PalletMultisigCall,
    PalletMultisigError,
    PalletMultisigEvent,
    PalletMultisigMultisig,
    PalletMultisigTimepoint,
    PalletPooledStakingCall,
    PalletPooledStakingCandidateEligibleCandidate,
    PalletPooledStakingError,
    PalletPooledStakingEvent,
    PalletPooledStakingHoldReason,
    PalletPooledStakingPendingOperationKey,
    PalletPooledStakingPendingOperationQuery,
    PalletPooledStakingPoolsActivePoolKind,
    PalletPooledStakingPoolsCandidateSummary,
    PalletPooledStakingPoolsKey,
    PalletPooledStakingPoolsPoolKind,
    PalletPooledStakingSharesOrStake,
    PalletProxyAnnouncement,
    PalletProxyCall,
    PalletProxyDepositKind,
    PalletProxyError,
    PalletProxyEvent,
    PalletProxyProxyDefinition,
    PalletRegistrarCall,
    PalletRegistrarDepositInfo,
    PalletRegistrarError,
    PalletRegistrarEvent,
    PalletRegistrarHoldReason,
    PalletRootTestingCall,
    PalletRootTestingEvent,
    PalletServicesPaymentCall,
    PalletServicesPaymentError,
    PalletServicesPaymentEvent,
    PalletSessionCall,
    PalletSessionError,
    PalletSessionEvent,
    PalletStreamPaymentCall,
    PalletStreamPaymentChangeKind,
    PalletStreamPaymentChangeRequest,
    PalletStreamPaymentDepositChange,
    PalletStreamPaymentError,
    PalletStreamPaymentEvent,
    PalletStreamPaymentFreezeReason,
    PalletStreamPaymentHoldReason,
    PalletStreamPaymentParty,
    PalletStreamPaymentStream,
    PalletStreamPaymentStreamConfig,
    PalletSudoCall,
    PalletSudoError,
    PalletSudoEvent,
    PalletTimestampCall,
    PalletTransactionPaymentChargeTransactionPayment,
    PalletTransactionPaymentEvent,
    PalletTransactionPaymentReleases,
    PalletTreasuryCall,
    PalletTreasuryError,
    PalletTreasuryEvent,
    PalletTreasuryPaymentState,
    PalletTreasuryProposal,
    PalletTreasurySpendStatus,
    PalletTxPauseCall,
    PalletTxPauseError,
    PalletTxPauseEvent,
    PalletUtilityCall,
    PalletUtilityError,
    PalletUtilityEvent,
    PalletXcmAuthorizedAliasesEntry,
    PalletXcmCall,
    PalletXcmCoreBuyerCall,
    PalletXcmCoreBuyerError,
    PalletXcmCoreBuyerEvent,
    PalletXcmCoreBuyerInFlightCoreBuyingOrder,
    PalletXcmCoreBuyerRelayXcmWeightConfigInner,
    PalletXcmError,
    PalletXcmEvent,
    PalletXcmMaxAuthorizedAliases,
    PalletXcmOrigin,
    PalletXcmQueryStatus,
    PalletXcmRemoteLockedFungibleRecord,
    PalletXcmVersionMigrationStage,
    PolkadotCorePrimitivesInboundDownwardMessage,
    PolkadotCorePrimitivesInboundHrmpMessage,
    PolkadotCorePrimitivesOutboundHrmpMessage,
    PolkadotPrimitivesV8AbridgedHostConfiguration,
    PolkadotPrimitivesV8AbridgedHrmpChannel,
    PolkadotPrimitivesV8AsyncBackingAsyncBackingParams,
    PolkadotPrimitivesV8PersistedValidationData,
    PolkadotPrimitivesV8UpgradeGoAhead,
    PolkadotPrimitivesV8UpgradeRestriction,
    SpArithmeticArithmeticError,
    SpCoreCryptoKeyTypeId,
    SpRuntimeDigest,
    SpRuntimeDigestDigestItem,
    SpRuntimeDispatchError,
    SpRuntimeModuleError,
    SpRuntimeMultiSignature,
    SpRuntimeProvingTrieTrieError,
    SpRuntimeTokenError,
    SpRuntimeTransactionalError,
    SpTrieStorageProof,
    SpVersionRuntimeVersion,
    SpWeightsRuntimeDbWeight,
    SpWeightsWeightV2Weight,
    StagingParachainInfoCall,
    StagingXcmExecutorAssetTransferTransferType,
    StagingXcmV3MultiLocation,
    StagingXcmV4Asset,
    StagingXcmV4AssetAssetFilter,
    StagingXcmV4AssetAssetId,
    StagingXcmV4AssetAssetInstance,
    StagingXcmV4AssetAssets,
    StagingXcmV4AssetFungibility,
    StagingXcmV4AssetWildAsset,
    StagingXcmV4AssetWildFungibility,
    StagingXcmV4Instruction,
    StagingXcmV4Junction,
    StagingXcmV4JunctionNetworkId,
    StagingXcmV4Junctions,
    StagingXcmV4Location,
    StagingXcmV4PalletInfo,
    StagingXcmV4QueryResponseInfo,
    StagingXcmV4Response,
    StagingXcmV4Xcm,
    StagingXcmV5Asset,
    StagingXcmV5AssetAssetFilter,
    StagingXcmV5AssetAssetId,
    StagingXcmV5AssetAssetInstance,
    StagingXcmV5AssetAssetTransferFilter,
    StagingXcmV5AssetAssets,
    StagingXcmV5AssetFungibility,
    StagingXcmV5AssetWildAsset,
    StagingXcmV5AssetWildFungibility,
    StagingXcmV5Hint,
    StagingXcmV5Instruction,
    StagingXcmV5Junction,
    StagingXcmV5JunctionNetworkId,
    StagingXcmV5Junctions,
    StagingXcmV5Location,
    StagingXcmV5PalletInfo,
    StagingXcmV5QueryResponseInfo,
    StagingXcmV5Response,
    StagingXcmV5TraitsOutcome,
    StagingXcmV5Xcm,
    TpAuthorNotingInherentOwnParachainInherentData,
    TpDataPreserversCommonAssignerExtra,
    TpDataPreserversCommonAssignmentWitness,
    TpDataPreserversCommonProviderRequest,
    TpStreamPaymentCommonAssetId,
    TpStreamPaymentCommonTimeUnit,
    TpTraitsContainerChainBlockInfo,
    TpTraitsFullRotationMode,
    TpTraitsFullRotationModes,
    TpTraitsParathreadParams,
    TpTraitsSlotFrequency,
    TpXcmCoreBuyerBuyCoreCollatorProof,
    XcmDoubleEncoded,
    XcmRuntimeApisAuthorizedAliasesOriginAliaser,
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
    XcmV3OriginKind,
    XcmV3PalletInfo,
    XcmV3QueryResponseInfo,
    XcmV3Response,
    XcmV3TraitsError,
    XcmV3TraitsSendError,
    XcmV3WeightLimit,
    XcmV3Xcm,
    XcmV5TraitsError,
    XcmVersionedAssetId,
    XcmVersionedAssets,
    XcmVersionedLocation,
    XcmVersionedResponse,
    XcmVersionedXcm,
} from "@polkadot/types/lookup";

declare module "@polkadot/types/types/registry" {
    interface InterfaceTypes {
        CumulusPalletParachainSystemCall: CumulusPalletParachainSystemCall;
        CumulusPalletParachainSystemError: CumulusPalletParachainSystemError;
        CumulusPalletParachainSystemEvent: CumulusPalletParachainSystemEvent;
        CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot;
        CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity: CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity;
        CumulusPalletParachainSystemUnincludedSegmentAncestor: CumulusPalletParachainSystemUnincludedSegmentAncestor;
        CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate: CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate;
        CumulusPalletParachainSystemUnincludedSegmentSegmentTracker: CumulusPalletParachainSystemUnincludedSegmentSegmentTracker;
        CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth: CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth;
        CumulusPalletWeightReclaimStorageWeightReclaim: CumulusPalletWeightReclaimStorageWeightReclaim;
        CumulusPalletXcmEvent: CumulusPalletXcmEvent;
        CumulusPalletXcmOrigin: CumulusPalletXcmOrigin;
        CumulusPalletXcmpQueueCall: CumulusPalletXcmpQueueCall;
        CumulusPalletXcmpQueueError: CumulusPalletXcmpQueueError;
        CumulusPalletXcmpQueueEvent: CumulusPalletXcmpQueueEvent;
        CumulusPalletXcmpQueueOutboundChannelDetails: CumulusPalletXcmpQueueOutboundChannelDetails;
        CumulusPalletXcmpQueueOutboundState: CumulusPalletXcmpQueueOutboundState;
        CumulusPalletXcmpQueueQueueConfigData: CumulusPalletXcmpQueueQueueConfigData;
        CumulusPrimitivesCoreAggregateMessageOrigin: CumulusPrimitivesCoreAggregateMessageOrigin;
        CumulusPrimitivesParachainInherentParachainInherentData: CumulusPrimitivesParachainInherentParachainInherentData;
        DanceboxRuntimeOriginCaller: DanceboxRuntimeOriginCaller;
        DanceboxRuntimeProxyType: DanceboxRuntimeProxyType;
        DanceboxRuntimeRuntime: DanceboxRuntimeRuntime;
        DanceboxRuntimeRuntimeFreezeReason: DanceboxRuntimeRuntimeFreezeReason;
        DanceboxRuntimeRuntimeHoldReason: DanceboxRuntimeRuntimeHoldReason;
        DanceboxRuntimeSessionKeys: DanceboxRuntimeSessionKeys;
        DanceboxRuntimeXcmConfigRelayChain: DanceboxRuntimeXcmConfigRelayChain;
        DpCollatorAssignmentAssignedCollatorsAccountId32: DpCollatorAssignmentAssignedCollatorsAccountId32;
        DpCollatorAssignmentAssignedCollatorsPublic: DpCollatorAssignmentAssignedCollatorsPublic;
        DpContainerChainGenesisDataContainerChainGenesisData: DpContainerChainGenesisDataContainerChainGenesisData;
        DpContainerChainGenesisDataContainerChainGenesisDataItem: DpContainerChainGenesisDataContainerChainGenesisDataItem;
        DpContainerChainGenesisDataProperties: DpContainerChainGenesisDataProperties;
        DpContainerChainGenesisDataTokenMetadata: DpContainerChainGenesisDataTokenMetadata;
        FrameMetadataHashExtensionCheckMetadataHash: FrameMetadataHashExtensionCheckMetadataHash;
        FrameMetadataHashExtensionMode: FrameMetadataHashExtensionMode;
        FrameSupportDispatchDispatchClass: FrameSupportDispatchDispatchClass;
        FrameSupportDispatchPays: FrameSupportDispatchPays;
        FrameSupportDispatchPerDispatchClassU32: FrameSupportDispatchPerDispatchClassU32;
        FrameSupportDispatchPerDispatchClassWeight: FrameSupportDispatchPerDispatchClassWeight;
        FrameSupportDispatchPerDispatchClassWeightsPerClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
        FrameSupportDispatchRawOrigin: FrameSupportDispatchRawOrigin;
        FrameSupportMessagesProcessMessageError: FrameSupportMessagesProcessMessageError;
        FrameSupportPalletId: FrameSupportPalletId;
        FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
        FrameSupportTokensMiscIdAmountRuntimeFreezeReason: FrameSupportTokensMiscIdAmountRuntimeFreezeReason;
        FrameSupportTokensMiscIdAmountRuntimeHoldReason: FrameSupportTokensMiscIdAmountRuntimeHoldReason;
        FrameSystemAccountInfo: FrameSystemAccountInfo;
        FrameSystemCall: FrameSystemCall;
        FrameSystemCodeUpgradeAuthorization: FrameSystemCodeUpgradeAuthorization;
        FrameSystemDispatchEventInfo: FrameSystemDispatchEventInfo;
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
        NimbusPrimitivesNimbusCryptoSignature: NimbusPrimitivesNimbusCryptoSignature;
        PalletAssetRateCall: PalletAssetRateCall;
        PalletAssetRateError: PalletAssetRateError;
        PalletAssetRateEvent: PalletAssetRateEvent;
        PalletAssetsAccountStatus: PalletAssetsAccountStatus;
        PalletAssetsApproval: PalletAssetsApproval;
        PalletAssetsAssetAccount: PalletAssetsAssetAccount;
        PalletAssetsAssetDetails: PalletAssetsAssetDetails;
        PalletAssetsAssetMetadata: PalletAssetsAssetMetadata;
        PalletAssetsAssetStatus: PalletAssetsAssetStatus;
        PalletAssetsCall: PalletAssetsCall;
        PalletAssetsError: PalletAssetsError;
        PalletAssetsEvent: PalletAssetsEvent;
        PalletAssetsExistenceReason: PalletAssetsExistenceReason;
        PalletAuthorInherentCall: PalletAuthorInherentCall;
        PalletAuthorInherentError: PalletAuthorInherentError;
        PalletAuthorNotingCall: PalletAuthorNotingCall;
        PalletAuthorNotingError: PalletAuthorNotingError;
        PalletAuthorNotingEvent: PalletAuthorNotingEvent;
        PalletAuthorityAssignmentCall: PalletAuthorityAssignmentCall;
        PalletBalancesAccountData: PalletBalancesAccountData;
        PalletBalancesAdjustmentDirection: PalletBalancesAdjustmentDirection;
        PalletBalancesBalanceLock: PalletBalancesBalanceLock;
        PalletBalancesCall: PalletBalancesCall;
        PalletBalancesError: PalletBalancesError;
        PalletBalancesEvent: PalletBalancesEvent;
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
        PalletDataPreserversHoldReason: PalletDataPreserversHoldReason;
        PalletDataPreserversParaIdsFilter: PalletDataPreserversParaIdsFilter;
        PalletDataPreserversProfile: PalletDataPreserversProfile;
        PalletDataPreserversProfileMode: PalletDataPreserversProfileMode;
        PalletDataPreserversRegisteredProfile: PalletDataPreserversRegisteredProfile;
        PalletForeignAssetCreatorCall: PalletForeignAssetCreatorCall;
        PalletForeignAssetCreatorError: PalletForeignAssetCreatorError;
        PalletForeignAssetCreatorEvent: PalletForeignAssetCreatorEvent;
        PalletIdentityAuthorityProperties: PalletIdentityAuthorityProperties;
        PalletIdentityCall: PalletIdentityCall;
        PalletIdentityError: PalletIdentityError;
        PalletIdentityEvent: PalletIdentityEvent;
        PalletIdentityJudgement: PalletIdentityJudgement;
        PalletIdentityLegacyIdentityInfo: PalletIdentityLegacyIdentityInfo;
        PalletIdentityProvider: PalletIdentityProvider;
        PalletIdentityRegistrarInfo: PalletIdentityRegistrarInfo;
        PalletIdentityRegistration: PalletIdentityRegistration;
        PalletIdentityUsernameInformation: PalletIdentityUsernameInformation;
        PalletInactivityTrackingActivityTrackingStatus: PalletInactivityTrackingActivityTrackingStatus;
        PalletInactivityTrackingCall: PalletInactivityTrackingCall;
        PalletInactivityTrackingError: PalletInactivityTrackingError;
        PalletInactivityTrackingEvent: PalletInactivityTrackingEvent;
        PalletInflationRewardsChainsToRewardValue: PalletInflationRewardsChainsToRewardValue;
        PalletInflationRewardsEvent: PalletInflationRewardsEvent;
        PalletInvulnerablesCall: PalletInvulnerablesCall;
        PalletInvulnerablesError: PalletInvulnerablesError;
        PalletInvulnerablesEvent: PalletInvulnerablesEvent;
        PalletMaintenanceModeCall: PalletMaintenanceModeCall;
        PalletMaintenanceModeError: PalletMaintenanceModeError;
        PalletMaintenanceModeEvent: PalletMaintenanceModeEvent;
        PalletMessageQueueBookState: PalletMessageQueueBookState;
        PalletMessageQueueCall: PalletMessageQueueCall;
        PalletMessageQueueError: PalletMessageQueueError;
        PalletMessageQueueEvent: PalletMessageQueueEvent;
        PalletMessageQueueNeighbours: PalletMessageQueueNeighbours;
        PalletMessageQueuePage: PalletMessageQueuePage;
        PalletMigrationsActiveCursor: PalletMigrationsActiveCursor;
        PalletMigrationsCall: PalletMigrationsCall;
        PalletMigrationsError: PalletMigrationsError;
        PalletMigrationsEvent: PalletMigrationsEvent;
        PalletMigrationsHistoricCleanupSelector: PalletMigrationsHistoricCleanupSelector;
        PalletMigrationsMigrationCursor: PalletMigrationsMigrationCursor;
        PalletMultisigCall: PalletMultisigCall;
        PalletMultisigError: PalletMultisigError;
        PalletMultisigEvent: PalletMultisigEvent;
        PalletMultisigMultisig: PalletMultisigMultisig;
        PalletMultisigTimepoint: PalletMultisigTimepoint;
        PalletPooledStakingCall: PalletPooledStakingCall;
        PalletPooledStakingCandidateEligibleCandidate: PalletPooledStakingCandidateEligibleCandidate;
        PalletPooledStakingError: PalletPooledStakingError;
        PalletPooledStakingEvent: PalletPooledStakingEvent;
        PalletPooledStakingHoldReason: PalletPooledStakingHoldReason;
        PalletPooledStakingPendingOperationKey: PalletPooledStakingPendingOperationKey;
        PalletPooledStakingPendingOperationQuery: PalletPooledStakingPendingOperationQuery;
        PalletPooledStakingPoolsActivePoolKind: PalletPooledStakingPoolsActivePoolKind;
        PalletPooledStakingPoolsCandidateSummary: PalletPooledStakingPoolsCandidateSummary;
        PalletPooledStakingPoolsKey: PalletPooledStakingPoolsKey;
        PalletPooledStakingPoolsPoolKind: PalletPooledStakingPoolsPoolKind;
        PalletPooledStakingSharesOrStake: PalletPooledStakingSharesOrStake;
        PalletProxyAnnouncement: PalletProxyAnnouncement;
        PalletProxyCall: PalletProxyCall;
        PalletProxyDepositKind: PalletProxyDepositKind;
        PalletProxyError: PalletProxyError;
        PalletProxyEvent: PalletProxyEvent;
        PalletProxyProxyDefinition: PalletProxyProxyDefinition;
        PalletRegistrarCall: PalletRegistrarCall;
        PalletRegistrarDepositInfo: PalletRegistrarDepositInfo;
        PalletRegistrarError: PalletRegistrarError;
        PalletRegistrarEvent: PalletRegistrarEvent;
        PalletRegistrarHoldReason: PalletRegistrarHoldReason;
        PalletRootTestingCall: PalletRootTestingCall;
        PalletRootTestingEvent: PalletRootTestingEvent;
        PalletServicesPaymentCall: PalletServicesPaymentCall;
        PalletServicesPaymentError: PalletServicesPaymentError;
        PalletServicesPaymentEvent: PalletServicesPaymentEvent;
        PalletSessionCall: PalletSessionCall;
        PalletSessionError: PalletSessionError;
        PalletSessionEvent: PalletSessionEvent;
        PalletStreamPaymentCall: PalletStreamPaymentCall;
        PalletStreamPaymentChangeKind: PalletStreamPaymentChangeKind;
        PalletStreamPaymentChangeRequest: PalletStreamPaymentChangeRequest;
        PalletStreamPaymentDepositChange: PalletStreamPaymentDepositChange;
        PalletStreamPaymentError: PalletStreamPaymentError;
        PalletStreamPaymentEvent: PalletStreamPaymentEvent;
        PalletStreamPaymentFreezeReason: PalletStreamPaymentFreezeReason;
        PalletStreamPaymentHoldReason: PalletStreamPaymentHoldReason;
        PalletStreamPaymentParty: PalletStreamPaymentParty;
        PalletStreamPaymentStream: PalletStreamPaymentStream;
        PalletStreamPaymentStreamConfig: PalletStreamPaymentStreamConfig;
        PalletSudoCall: PalletSudoCall;
        PalletSudoError: PalletSudoError;
        PalletSudoEvent: PalletSudoEvent;
        PalletTimestampCall: PalletTimestampCall;
        PalletTransactionPaymentChargeTransactionPayment: PalletTransactionPaymentChargeTransactionPayment;
        PalletTransactionPaymentEvent: PalletTransactionPaymentEvent;
        PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
        PalletTreasuryCall: PalletTreasuryCall;
        PalletTreasuryError: PalletTreasuryError;
        PalletTreasuryEvent: PalletTreasuryEvent;
        PalletTreasuryPaymentState: PalletTreasuryPaymentState;
        PalletTreasuryProposal: PalletTreasuryProposal;
        PalletTreasurySpendStatus: PalletTreasurySpendStatus;
        PalletTxPauseCall: PalletTxPauseCall;
        PalletTxPauseError: PalletTxPauseError;
        PalletTxPauseEvent: PalletTxPauseEvent;
        PalletUtilityCall: PalletUtilityCall;
        PalletUtilityError: PalletUtilityError;
        PalletUtilityEvent: PalletUtilityEvent;
        PalletXcmAuthorizedAliasesEntry: PalletXcmAuthorizedAliasesEntry;
        PalletXcmCall: PalletXcmCall;
        PalletXcmCoreBuyerCall: PalletXcmCoreBuyerCall;
        PalletXcmCoreBuyerError: PalletXcmCoreBuyerError;
        PalletXcmCoreBuyerEvent: PalletXcmCoreBuyerEvent;
        PalletXcmCoreBuyerInFlightCoreBuyingOrder: PalletXcmCoreBuyerInFlightCoreBuyingOrder;
        PalletXcmCoreBuyerRelayXcmWeightConfigInner: PalletXcmCoreBuyerRelayXcmWeightConfigInner;
        PalletXcmError: PalletXcmError;
        PalletXcmEvent: PalletXcmEvent;
        PalletXcmMaxAuthorizedAliases: PalletXcmMaxAuthorizedAliases;
        PalletXcmOrigin: PalletXcmOrigin;
        PalletXcmQueryStatus: PalletXcmQueryStatus;
        PalletXcmRemoteLockedFungibleRecord: PalletXcmRemoteLockedFungibleRecord;
        PalletXcmVersionMigrationStage: PalletXcmVersionMigrationStage;
        PolkadotCorePrimitivesInboundDownwardMessage: PolkadotCorePrimitivesInboundDownwardMessage;
        PolkadotCorePrimitivesInboundHrmpMessage: PolkadotCorePrimitivesInboundHrmpMessage;
        PolkadotCorePrimitivesOutboundHrmpMessage: PolkadotCorePrimitivesOutboundHrmpMessage;
        PolkadotPrimitivesV8AbridgedHostConfiguration: PolkadotPrimitivesV8AbridgedHostConfiguration;
        PolkadotPrimitivesV8AbridgedHrmpChannel: PolkadotPrimitivesV8AbridgedHrmpChannel;
        PolkadotPrimitivesV8AsyncBackingAsyncBackingParams: PolkadotPrimitivesV8AsyncBackingAsyncBackingParams;
        PolkadotPrimitivesV8PersistedValidationData: PolkadotPrimitivesV8PersistedValidationData;
        PolkadotPrimitivesV8UpgradeGoAhead: PolkadotPrimitivesV8UpgradeGoAhead;
        PolkadotPrimitivesV8UpgradeRestriction: PolkadotPrimitivesV8UpgradeRestriction;
        SpArithmeticArithmeticError: SpArithmeticArithmeticError;
        SpCoreCryptoKeyTypeId: SpCoreCryptoKeyTypeId;
        SpRuntimeDigest: SpRuntimeDigest;
        SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
        SpRuntimeDispatchError: SpRuntimeDispatchError;
        SpRuntimeModuleError: SpRuntimeModuleError;
        SpRuntimeMultiSignature: SpRuntimeMultiSignature;
        SpRuntimeProvingTrieTrieError: SpRuntimeProvingTrieTrieError;
        SpRuntimeTokenError: SpRuntimeTokenError;
        SpRuntimeTransactionalError: SpRuntimeTransactionalError;
        SpTrieStorageProof: SpTrieStorageProof;
        SpVersionRuntimeVersion: SpVersionRuntimeVersion;
        SpWeightsRuntimeDbWeight: SpWeightsRuntimeDbWeight;
        SpWeightsWeightV2Weight: SpWeightsWeightV2Weight;
        StagingParachainInfoCall: StagingParachainInfoCall;
        StagingXcmExecutorAssetTransferTransferType: StagingXcmExecutorAssetTransferTransferType;
        StagingXcmV3MultiLocation: StagingXcmV3MultiLocation;
        StagingXcmV4Asset: StagingXcmV4Asset;
        StagingXcmV4AssetAssetFilter: StagingXcmV4AssetAssetFilter;
        StagingXcmV4AssetAssetId: StagingXcmV4AssetAssetId;
        StagingXcmV4AssetAssetInstance: StagingXcmV4AssetAssetInstance;
        StagingXcmV4AssetAssets: StagingXcmV4AssetAssets;
        StagingXcmV4AssetFungibility: StagingXcmV4AssetFungibility;
        StagingXcmV4AssetWildAsset: StagingXcmV4AssetWildAsset;
        StagingXcmV4AssetWildFungibility: StagingXcmV4AssetWildFungibility;
        StagingXcmV4Instruction: StagingXcmV4Instruction;
        StagingXcmV4Junction: StagingXcmV4Junction;
        StagingXcmV4JunctionNetworkId: StagingXcmV4JunctionNetworkId;
        StagingXcmV4Junctions: StagingXcmV4Junctions;
        StagingXcmV4Location: StagingXcmV4Location;
        StagingXcmV4PalletInfo: StagingXcmV4PalletInfo;
        StagingXcmV4QueryResponseInfo: StagingXcmV4QueryResponseInfo;
        StagingXcmV4Response: StagingXcmV4Response;
        StagingXcmV4Xcm: StagingXcmV4Xcm;
        StagingXcmV5Asset: StagingXcmV5Asset;
        StagingXcmV5AssetAssetFilter: StagingXcmV5AssetAssetFilter;
        StagingXcmV5AssetAssetId: StagingXcmV5AssetAssetId;
        StagingXcmV5AssetAssetInstance: StagingXcmV5AssetAssetInstance;
        StagingXcmV5AssetAssetTransferFilter: StagingXcmV5AssetAssetTransferFilter;
        StagingXcmV5AssetAssets: StagingXcmV5AssetAssets;
        StagingXcmV5AssetFungibility: StagingXcmV5AssetFungibility;
        StagingXcmV5AssetWildAsset: StagingXcmV5AssetWildAsset;
        StagingXcmV5AssetWildFungibility: StagingXcmV5AssetWildFungibility;
        StagingXcmV5Hint: StagingXcmV5Hint;
        StagingXcmV5Instruction: StagingXcmV5Instruction;
        StagingXcmV5Junction: StagingXcmV5Junction;
        StagingXcmV5JunctionNetworkId: StagingXcmV5JunctionNetworkId;
        StagingXcmV5Junctions: StagingXcmV5Junctions;
        StagingXcmV5Location: StagingXcmV5Location;
        StagingXcmV5PalletInfo: StagingXcmV5PalletInfo;
        StagingXcmV5QueryResponseInfo: StagingXcmV5QueryResponseInfo;
        StagingXcmV5Response: StagingXcmV5Response;
        StagingXcmV5TraitsOutcome: StagingXcmV5TraitsOutcome;
        StagingXcmV5Xcm: StagingXcmV5Xcm;
        TpAuthorNotingInherentOwnParachainInherentData: TpAuthorNotingInherentOwnParachainInherentData;
        TpDataPreserversCommonAssignerExtra: TpDataPreserversCommonAssignerExtra;
        TpDataPreserversCommonAssignmentWitness: TpDataPreserversCommonAssignmentWitness;
        TpDataPreserversCommonProviderRequest: TpDataPreserversCommonProviderRequest;
        TpStreamPaymentCommonAssetId: TpStreamPaymentCommonAssetId;
        TpStreamPaymentCommonTimeUnit: TpStreamPaymentCommonTimeUnit;
        TpTraitsContainerChainBlockInfo: TpTraitsContainerChainBlockInfo;
        TpTraitsFullRotationMode: TpTraitsFullRotationMode;
        TpTraitsFullRotationModes: TpTraitsFullRotationModes;
        TpTraitsParathreadParams: TpTraitsParathreadParams;
        TpTraitsSlotFrequency: TpTraitsSlotFrequency;
        TpXcmCoreBuyerBuyCoreCollatorProof: TpXcmCoreBuyerBuyCoreCollatorProof;
        XcmDoubleEncoded: XcmDoubleEncoded;
        XcmRuntimeApisAuthorizedAliasesOriginAliaser: XcmRuntimeApisAuthorizedAliasesOriginAliaser;
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
        XcmV3OriginKind: XcmV3OriginKind;
        XcmV3PalletInfo: XcmV3PalletInfo;
        XcmV3QueryResponseInfo: XcmV3QueryResponseInfo;
        XcmV3Response: XcmV3Response;
        XcmV3TraitsError: XcmV3TraitsError;
        XcmV3TraitsSendError: XcmV3TraitsSendError;
        XcmV3WeightLimit: XcmV3WeightLimit;
        XcmV3Xcm: XcmV3Xcm;
        XcmV5TraitsError: XcmV5TraitsError;
        XcmVersionedAssetId: XcmVersionedAssetId;
        XcmVersionedAssets: XcmVersionedAssets;
        XcmVersionedLocation: XcmVersionedLocation;
        XcmVersionedResponse: XcmVersionedResponse;
        XcmVersionedXcm: XcmVersionedXcm;
    } // InterfaceTypes
} // declare module
