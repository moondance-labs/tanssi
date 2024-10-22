// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/types/types/registry";

import type {
    BinaryHeapEnqueuedOrder,
    BinaryHeapReverseQueueIndex,
    BitvecOrderLsb0,
    DancelightRuntimeDynamicParamsPreimageBaseDeposit,
    DancelightRuntimeDynamicParamsPreimageByteDeposit,
    DancelightRuntimeDynamicParamsPreimageParameters,
    DancelightRuntimeDynamicParamsPreimageParametersKey,
    DancelightRuntimeDynamicParamsPreimageParametersValue,
    DancelightRuntimeGovernanceOriginsPalletCustomOriginsOrigin,
    DancelightRuntimeOriginCaller,
    DancelightRuntimePreserversAssignmentPaymentExtra,
    DancelightRuntimePreserversAssignmentPaymentRequest,
    DancelightRuntimePreserversAssignmentPaymentWitness,
    DancelightRuntimeProxyType,
    DancelightRuntimeRuntime,
    DancelightRuntimeRuntimeHoldReason,
    DancelightRuntimeRuntimeParameters,
    DancelightRuntimeRuntimeParametersKey,
    DancelightRuntimeRuntimeParametersValue,
    DancelightRuntimeSessionKeys,
    DpCollatorAssignmentAssignedCollatorsAccountId32,
    DpCollatorAssignmentAssignedCollatorsPublic,
    DpContainerChainGenesisDataContainerChainGenesisData,
    DpContainerChainGenesisDataContainerChainGenesisDataItem,
    DpContainerChainGenesisDataProperties,
    DpContainerChainGenesisDataTokenMetadata,
    FinalityGrandpaEquivocationPrecommit,
    FinalityGrandpaEquivocationPrevote,
    FinalityGrandpaPrecommit,
    FinalityGrandpaPrevote,
    FrameSupportDispatchDispatchClass,
    FrameSupportDispatchDispatchInfo,
    FrameSupportDispatchPays,
    FrameSupportDispatchPerDispatchClassU32,
    FrameSupportDispatchPerDispatchClassWeight,
    FrameSupportDispatchPerDispatchClassWeightsPerClass,
    FrameSupportDispatchPostDispatchInfo,
    FrameSupportDispatchRawOrigin,
    FrameSupportMessagesProcessMessageError,
    FrameSupportPalletId,
    FrameSupportPreimagesBounded,
    FrameSupportScheduleDispatchTime,
    FrameSupportTokensMiscBalanceStatus,
    FrameSupportTokensMiscIdAmount,
    FrameSystemAccountInfo,
    FrameSystemCall,
    FrameSystemCodeUpgradeAuthorization,
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
    PalletAssetRateCall,
    PalletAssetRateError,
    PalletAssetRateEvent,
    PalletAuthorNotingCall,
    PalletAuthorNotingError,
    PalletAuthorNotingEvent,
    PalletAuthorityAssignmentCall,
    PalletBabeCall,
    PalletBabeError,
    PalletBalancesAccountData,
    PalletBalancesAdjustmentDirection,
    PalletBalancesBalanceLock,
    PalletBalancesCall,
    PalletBalancesError,
    PalletBalancesEvent,
    PalletBalancesReasons,
    PalletBalancesReserveData,
    PalletBeefyCall,
    PalletBeefyError,
    PalletCollatorAssignmentCall,
    PalletCollatorAssignmentEvent,
    PalletConfigurationCall,
    PalletConfigurationError,
    PalletConfigurationHostConfiguration,
    PalletConvictionVotingCall,
    PalletConvictionVotingConviction,
    PalletConvictionVotingDelegations,
    PalletConvictionVotingError,
    PalletConvictionVotingEvent,
    PalletConvictionVotingTally,
    PalletConvictionVotingVoteAccountVote,
    PalletConvictionVotingVoteCasting,
    PalletConvictionVotingVoteDelegating,
    PalletConvictionVotingVotePriorLock,
    PalletConvictionVotingVoteVoting,
    PalletDataPreserversCall,
    PalletDataPreserversError,
    PalletDataPreserversEvent,
    PalletDataPreserversHoldReason,
    PalletDataPreserversParaIdsFilter,
    PalletDataPreserversProfile,
    PalletDataPreserversProfileMode,
    PalletDataPreserversRegisteredProfile,
    PalletExternalValidatorsCall,
    PalletExternalValidatorsError,
    PalletExternalValidatorsEvent,
    PalletExternalValidatorsForcing,
    PalletGrandpaCall,
    PalletGrandpaError,
    PalletGrandpaEvent,
    PalletGrandpaStoredPendingChange,
    PalletGrandpaStoredState,
    PalletIdentityAuthorityProperties,
    PalletIdentityCall,
    PalletIdentityError,
    PalletIdentityEvent,
    PalletIdentityJudgement,
    PalletIdentityLegacyIdentityInfo,
    PalletIdentityRegistrarInfo,
    PalletIdentityRegistration,
    PalletInflationRewardsChainsToRewardValue,
    PalletInflationRewardsEvent,
    PalletInvulnerablesCall,
    PalletInvulnerablesError,
    PalletInvulnerablesEvent,
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
    PalletOffencesEvent,
    PalletParametersCall,
    PalletParametersEvent,
    PalletPreimageCall,
    PalletPreimageError,
    PalletPreimageEvent,
    PalletPreimageHoldReason,
    PalletPreimageOldRequestStatus,
    PalletPreimageRequestStatus,
    PalletProxyAnnouncement,
    PalletProxyCall,
    PalletProxyError,
    PalletProxyEvent,
    PalletProxyProxyDefinition,
    PalletRankedCollectiveCall,
    PalletRankedCollectiveError,
    PalletRankedCollectiveEvent,
    PalletRankedCollectiveMemberRecord,
    PalletRankedCollectiveTally,
    PalletRankedCollectiveVoteRecord,
    PalletReferendaCall,
    PalletReferendaCurve,
    PalletReferendaDecidingStatus,
    PalletReferendaDeposit,
    PalletReferendaError,
    PalletReferendaEvent,
    PalletReferendaReferendumInfoConvictionVotingTally,
    PalletReferendaReferendumInfoRankedCollectiveTally,
    PalletReferendaReferendumStatusConvictionVotingTally,
    PalletReferendaReferendumStatusRankedCollectiveTally,
    PalletReferendaTrackInfo,
    PalletRegistrarCall,
    PalletRegistrarDepositInfo,
    PalletRegistrarError,
    PalletRegistrarEvent,
    PalletRegistrarHoldReason,
    PalletRootTestingCall,
    PalletRootTestingEvent,
    PalletSchedulerCall,
    PalletSchedulerError,
    PalletSchedulerEvent,
    PalletSchedulerRetryConfig,
    PalletSchedulerScheduled,
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
    PalletTreasuryCall,
    PalletTreasuryError,
    PalletTreasuryEvent,
    PalletTreasuryPaymentState,
    PalletTreasuryProposal,
    PalletTreasurySpendStatus,
    PalletUtilityCall,
    PalletUtilityError,
    PalletUtilityEvent,
    PalletWhitelistCall,
    PalletWhitelistError,
    PalletWhitelistEvent,
    PalletXcmCall,
    PalletXcmError,
    PalletXcmEvent,
    PalletXcmOrigin,
    PalletXcmQueryStatus,
    PalletXcmRemoteLockedFungibleRecord,
    PalletXcmVersionMigrationStage,
    PolkadotCorePrimitivesInboundDownwardMessage,
    PolkadotCorePrimitivesInboundHrmpMessage,
    PolkadotCorePrimitivesOutboundHrmpMessage,
    PolkadotParachainPrimitivesPrimitivesHrmpChannelId,
    PolkadotPrimitivesV7ApprovalVotingParams,
    PolkadotPrimitivesV7AssignmentAppPublic,
    PolkadotPrimitivesV7AsyncBackingAsyncBackingParams,
    PolkadotPrimitivesV7BackedCandidate,
    PolkadotPrimitivesV7CandidateCommitments,
    PolkadotPrimitivesV7CandidateDescriptor,
    PolkadotPrimitivesV7CandidateReceipt,
    PolkadotPrimitivesV7CollatorAppPublic,
    PolkadotPrimitivesV7CollatorAppSignature,
    PolkadotPrimitivesV7CommittedCandidateReceipt,
    PolkadotPrimitivesV7DisputeState,
    PolkadotPrimitivesV7DisputeStatement,
    PolkadotPrimitivesV7DisputeStatementSet,
    PolkadotPrimitivesV7ExecutorParams,
    PolkadotPrimitivesV7ExecutorParamsExecutorParam,
    PolkadotPrimitivesV7IndexedVecGroupIndex,
    PolkadotPrimitivesV7IndexedVecValidatorIndex,
    PolkadotPrimitivesV7InherentData,
    PolkadotPrimitivesV7InvalidDisputeStatementKind,
    PolkadotPrimitivesV7PvfCheckStatement,
    PolkadotPrimitivesV7PvfExecKind,
    PolkadotPrimitivesV7PvfPrepKind,
    PolkadotPrimitivesV7ScrapedOnChainVotes,
    PolkadotPrimitivesV7SessionInfo,
    PolkadotPrimitivesV7SignedUncheckedSigned,
    PolkadotPrimitivesV7SlashingDisputeProof,
    PolkadotPrimitivesV7SlashingDisputesTimeSlot,
    PolkadotPrimitivesV7SlashingPendingSlashes,
    PolkadotPrimitivesV7SlashingSlashingOffenceKind,
    PolkadotPrimitivesV7UpgradeGoAhead,
    PolkadotPrimitivesV7UpgradeRestriction,
    PolkadotPrimitivesV7ValidDisputeStatementKind,
    PolkadotPrimitivesV7ValidatorAppPublic,
    PolkadotPrimitivesV7ValidatorAppSignature,
    PolkadotPrimitivesV7ValidityAttestation,
    PolkadotPrimitivesVstagingSchedulerParams,
    PolkadotRuntimeCommonParasRegistrarPalletCall,
    PolkadotRuntimeCommonParasRegistrarPalletError,
    PolkadotRuntimeCommonParasRegistrarPalletEvent,
    PolkadotRuntimeCommonParasRegistrarParaInfo,
    PolkadotRuntimeCommonParasSudoWrapperPalletCall,
    PolkadotRuntimeCommonParasSudoWrapperPalletError,
    PolkadotRuntimeParachainsAssignerOnDemandPalletCall,
    PolkadotRuntimeParachainsAssignerOnDemandPalletError,
    PolkadotRuntimeParachainsAssignerOnDemandPalletEvent,
    PolkadotRuntimeParachainsAssignerOnDemandTypesCoreAffinityCount,
    PolkadotRuntimeParachainsAssignerOnDemandTypesEnqueuedOrder,
    PolkadotRuntimeParachainsAssignerOnDemandTypesQueueStatusType,
    PolkadotRuntimeParachainsConfigurationHostConfiguration,
    PolkadotRuntimeParachainsConfigurationPalletCall,
    PolkadotRuntimeParachainsConfigurationPalletError,
    PolkadotRuntimeParachainsDisputesDisputeLocation,
    PolkadotRuntimeParachainsDisputesDisputeResult,
    PolkadotRuntimeParachainsDisputesPalletCall,
    PolkadotRuntimeParachainsDisputesPalletError,
    PolkadotRuntimeParachainsDisputesPalletEvent,
    PolkadotRuntimeParachainsDisputesSlashingPalletCall,
    PolkadotRuntimeParachainsDisputesSlashingPalletError,
    PolkadotRuntimeParachainsHrmpHrmpChannel,
    PolkadotRuntimeParachainsHrmpHrmpOpenChannelRequest,
    PolkadotRuntimeParachainsHrmpPalletCall,
    PolkadotRuntimeParachainsHrmpPalletError,
    PolkadotRuntimeParachainsHrmpPalletEvent,
    PolkadotRuntimeParachainsInclusionAggregateMessageOrigin,
    PolkadotRuntimeParachainsInclusionCandidatePendingAvailability,
    PolkadotRuntimeParachainsInclusionPalletCall,
    PolkadotRuntimeParachainsInclusionPalletError,
    PolkadotRuntimeParachainsInclusionPalletEvent,
    PolkadotRuntimeParachainsInclusionUmpQueueId,
    PolkadotRuntimeParachainsInitializerBufferedSessionChange,
    PolkadotRuntimeParachainsInitializerPalletCall,
    PolkadotRuntimeParachainsOriginPalletOrigin,
    PolkadotRuntimeParachainsParasInherentPalletCall,
    PolkadotRuntimeParachainsParasInherentPalletError,
    PolkadotRuntimeParachainsParasPalletCall,
    PolkadotRuntimeParachainsParasPalletError,
    PolkadotRuntimeParachainsParasPalletEvent,
    PolkadotRuntimeParachainsParasParaGenesisArgs,
    PolkadotRuntimeParachainsParasParaLifecycle,
    PolkadotRuntimeParachainsParasParaPastCodeMeta,
    PolkadotRuntimeParachainsParasPvfCheckActiveVoteState,
    PolkadotRuntimeParachainsParasPvfCheckCause,
    PolkadotRuntimeParachainsParasReplacementTimes,
    PolkadotRuntimeParachainsParasUpgradeStrategy,
    PolkadotRuntimeParachainsSchedulerCommonAssignment,
    PolkadotRuntimeParachainsSchedulerPalletCoreOccupied,
    PolkadotRuntimeParachainsSchedulerPalletParasEntry,
    PolkadotRuntimeParachainsSharedAllowedRelayParentsTracker,
    PolkadotRuntimeParachainsSharedPalletCall,
    SnowbridgeAmclBls381Big,
    SnowbridgeAmclBls381Ecp,
    SnowbridgeAmclBls381Fp,
    SnowbridgeBeaconPrimitivesBeaconHeader,
    SnowbridgeBeaconPrimitivesBlsBlsError,
    SnowbridgeBeaconPrimitivesCompactBeaconState,
    SnowbridgeBeaconPrimitivesFork,
    SnowbridgeBeaconPrimitivesForkVersions,
    SnowbridgeBeaconPrimitivesPublicKey,
    SnowbridgeBeaconPrimitivesSignature,
    SnowbridgeBeaconPrimitivesSyncAggregate,
    SnowbridgeBeaconPrimitivesSyncCommittee,
    SnowbridgeBeaconPrimitivesSyncCommitteePrepared,
    SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate,
    SnowbridgeBeaconPrimitivesUpdatesNextSyncCommitteeUpdate,
    SnowbridgeBeaconPrimitivesUpdatesUpdate,
    SnowbridgeCoreOperatingModeBasicOperatingMode,
    SnowbridgeMilagroBlsKeysPublicKey,
    SnowbridgePalletEthereumClientCall,
    SnowbridgePalletEthereumClientError,
    SnowbridgePalletEthereumClientEvent,
    SpArithmeticArithmeticError,
    SpAuthorityDiscoveryAppPublic,
    SpConsensusBabeAllowedSlots,
    SpConsensusBabeAppPublic,
    SpConsensusBabeBabeEpochConfiguration,
    SpConsensusBabeDigestsNextConfigDescriptor,
    SpConsensusBabeDigestsPreDigest,
    SpConsensusBabeDigestsPrimaryPreDigest,
    SpConsensusBabeDigestsSecondaryPlainPreDigest,
    SpConsensusBabeDigestsSecondaryVRFPreDigest,
    SpConsensusBeefyCommitment,
    SpConsensusBeefyDoubleVotingProof,
    SpConsensusBeefyEcdsaCryptoPublic,
    SpConsensusBeefyEcdsaCryptoSignature,
    SpConsensusBeefyForkVotingProof,
    SpConsensusBeefyFutureBlockVotingProof,
    SpConsensusBeefyMmrBeefyAuthoritySet,
    SpConsensusBeefyPayload,
    SpConsensusBeefyVoteMessage,
    SpConsensusGrandpaAppPublic,
    SpConsensusGrandpaAppSignature,
    SpConsensusGrandpaEquivocation,
    SpConsensusGrandpaEquivocationProof,
    SpConsensusSlotsEquivocationProof,
    SpCoreCryptoKeyTypeId,
    SpCoreSr25519VrfVrfSignature,
    SpCoreVoid,
    SpMmrPrimitivesAncestryProof,
    SpRuntimeBlakeTwo256,
    SpRuntimeDigest,
    SpRuntimeDigestDigestItem,
    SpRuntimeDispatchError,
    SpRuntimeDispatchErrorWithPostInfo,
    SpRuntimeHeader,
    SpRuntimeModuleError,
    SpRuntimeMultiSignature,
    SpRuntimeTokenError,
    SpRuntimeTransactionalError,
    SpSessionMembershipProof,
    SpStakingOffenceOffenceDetails,
    SpTrieStorageProof,
    SpVersionRuntimeVersion,
    SpWeightsRuntimeDbWeight,
    SpWeightsWeightV2Weight,
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
    StagingXcmV4TraitsOutcome,
    StagingXcmV4Xcm,
    TpTraitsActiveEraInfo,
    TpTraitsContainerChainBlockInfo,
    TpTraitsParathreadParams,
    TpTraitsSlotFrequency,
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
    XcmV3OriginKind,
    XcmV3PalletInfo,
    XcmV3QueryResponseInfo,
    XcmV3Response,
    XcmV3TraitsError,
    XcmV3WeightLimit,
    XcmV3Xcm,
    XcmVersionedAssetId,
    XcmVersionedAssets,
    XcmVersionedLocation,
    XcmVersionedResponse,
    XcmVersionedXcm,
} from "@polkadot/types/lookup";

declare module "@polkadot/types/types/registry" {
    interface InterfaceTypes {
        BinaryHeapEnqueuedOrder: BinaryHeapEnqueuedOrder;
        BinaryHeapReverseQueueIndex: BinaryHeapReverseQueueIndex;
        BitvecOrderLsb0: BitvecOrderLsb0;
        DancelightRuntimeDynamicParamsPreimageBaseDeposit: DancelightRuntimeDynamicParamsPreimageBaseDeposit;
        DancelightRuntimeDynamicParamsPreimageByteDeposit: DancelightRuntimeDynamicParamsPreimageByteDeposit;
        DancelightRuntimeDynamicParamsPreimageParameters: DancelightRuntimeDynamicParamsPreimageParameters;
        DancelightRuntimeDynamicParamsPreimageParametersKey: DancelightRuntimeDynamicParamsPreimageParametersKey;
        DancelightRuntimeDynamicParamsPreimageParametersValue: DancelightRuntimeDynamicParamsPreimageParametersValue;
        DancelightRuntimeGovernanceOriginsPalletCustomOriginsOrigin: DancelightRuntimeGovernanceOriginsPalletCustomOriginsOrigin;
        DancelightRuntimeOriginCaller: DancelightRuntimeOriginCaller;
        DancelightRuntimePreserversAssignmentPaymentExtra: DancelightRuntimePreserversAssignmentPaymentExtra;
        DancelightRuntimePreserversAssignmentPaymentRequest: DancelightRuntimePreserversAssignmentPaymentRequest;
        DancelightRuntimePreserversAssignmentPaymentWitness: DancelightRuntimePreserversAssignmentPaymentWitness;
        DancelightRuntimeProxyType: DancelightRuntimeProxyType;
        DancelightRuntimeRuntime: DancelightRuntimeRuntime;
        DancelightRuntimeRuntimeHoldReason: DancelightRuntimeRuntimeHoldReason;
        DancelightRuntimeRuntimeParameters: DancelightRuntimeRuntimeParameters;
        DancelightRuntimeRuntimeParametersKey: DancelightRuntimeRuntimeParametersKey;
        DancelightRuntimeRuntimeParametersValue: DancelightRuntimeRuntimeParametersValue;
        DancelightRuntimeSessionKeys: DancelightRuntimeSessionKeys;
        DpCollatorAssignmentAssignedCollatorsAccountId32: DpCollatorAssignmentAssignedCollatorsAccountId32;
        DpCollatorAssignmentAssignedCollatorsPublic: DpCollatorAssignmentAssignedCollatorsPublic;
        DpContainerChainGenesisDataContainerChainGenesisData: DpContainerChainGenesisDataContainerChainGenesisData;
        DpContainerChainGenesisDataContainerChainGenesisDataItem: DpContainerChainGenesisDataContainerChainGenesisDataItem;
        DpContainerChainGenesisDataProperties: DpContainerChainGenesisDataProperties;
        DpContainerChainGenesisDataTokenMetadata: DpContainerChainGenesisDataTokenMetadata;
        FinalityGrandpaEquivocationPrecommit: FinalityGrandpaEquivocationPrecommit;
        FinalityGrandpaEquivocationPrevote: FinalityGrandpaEquivocationPrevote;
        FinalityGrandpaPrecommit: FinalityGrandpaPrecommit;
        FinalityGrandpaPrevote: FinalityGrandpaPrevote;
        FrameSupportDispatchDispatchClass: FrameSupportDispatchDispatchClass;
        FrameSupportDispatchDispatchInfo: FrameSupportDispatchDispatchInfo;
        FrameSupportDispatchPays: FrameSupportDispatchPays;
        FrameSupportDispatchPerDispatchClassU32: FrameSupportDispatchPerDispatchClassU32;
        FrameSupportDispatchPerDispatchClassWeight: FrameSupportDispatchPerDispatchClassWeight;
        FrameSupportDispatchPerDispatchClassWeightsPerClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
        FrameSupportDispatchPostDispatchInfo: FrameSupportDispatchPostDispatchInfo;
        FrameSupportDispatchRawOrigin: FrameSupportDispatchRawOrigin;
        FrameSupportMessagesProcessMessageError: FrameSupportMessagesProcessMessageError;
        FrameSupportPalletId: FrameSupportPalletId;
        FrameSupportPreimagesBounded: FrameSupportPreimagesBounded;
        FrameSupportScheduleDispatchTime: FrameSupportScheduleDispatchTime;
        FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
        FrameSupportTokensMiscIdAmount: FrameSupportTokensMiscIdAmount;
        FrameSystemAccountInfo: FrameSystemAccountInfo;
        FrameSystemCall: FrameSystemCall;
        FrameSystemCodeUpgradeAuthorization: FrameSystemCodeUpgradeAuthorization;
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
        PalletAssetRateCall: PalletAssetRateCall;
        PalletAssetRateError: PalletAssetRateError;
        PalletAssetRateEvent: PalletAssetRateEvent;
        PalletAuthorNotingCall: PalletAuthorNotingCall;
        PalletAuthorNotingError: PalletAuthorNotingError;
        PalletAuthorNotingEvent: PalletAuthorNotingEvent;
        PalletAuthorityAssignmentCall: PalletAuthorityAssignmentCall;
        PalletBabeCall: PalletBabeCall;
        PalletBabeError: PalletBabeError;
        PalletBalancesAccountData: PalletBalancesAccountData;
        PalletBalancesAdjustmentDirection: PalletBalancesAdjustmentDirection;
        PalletBalancesBalanceLock: PalletBalancesBalanceLock;
        PalletBalancesCall: PalletBalancesCall;
        PalletBalancesError: PalletBalancesError;
        PalletBalancesEvent: PalletBalancesEvent;
        PalletBalancesReasons: PalletBalancesReasons;
        PalletBalancesReserveData: PalletBalancesReserveData;
        PalletBeefyCall: PalletBeefyCall;
        PalletBeefyError: PalletBeefyError;
        PalletCollatorAssignmentCall: PalletCollatorAssignmentCall;
        PalletCollatorAssignmentEvent: PalletCollatorAssignmentEvent;
        PalletConfigurationCall: PalletConfigurationCall;
        PalletConfigurationError: PalletConfigurationError;
        PalletConfigurationHostConfiguration: PalletConfigurationHostConfiguration;
        PalletConvictionVotingCall: PalletConvictionVotingCall;
        PalletConvictionVotingConviction: PalletConvictionVotingConviction;
        PalletConvictionVotingDelegations: PalletConvictionVotingDelegations;
        PalletConvictionVotingError: PalletConvictionVotingError;
        PalletConvictionVotingEvent: PalletConvictionVotingEvent;
        PalletConvictionVotingTally: PalletConvictionVotingTally;
        PalletConvictionVotingVoteAccountVote: PalletConvictionVotingVoteAccountVote;
        PalletConvictionVotingVoteCasting: PalletConvictionVotingVoteCasting;
        PalletConvictionVotingVoteDelegating: PalletConvictionVotingVoteDelegating;
        PalletConvictionVotingVotePriorLock: PalletConvictionVotingVotePriorLock;
        PalletConvictionVotingVoteVoting: PalletConvictionVotingVoteVoting;
        PalletDataPreserversCall: PalletDataPreserversCall;
        PalletDataPreserversError: PalletDataPreserversError;
        PalletDataPreserversEvent: PalletDataPreserversEvent;
        PalletDataPreserversHoldReason: PalletDataPreserversHoldReason;
        PalletDataPreserversParaIdsFilter: PalletDataPreserversParaIdsFilter;
        PalletDataPreserversProfile: PalletDataPreserversProfile;
        PalletDataPreserversProfileMode: PalletDataPreserversProfileMode;
        PalletDataPreserversRegisteredProfile: PalletDataPreserversRegisteredProfile;
        PalletExternalValidatorsCall: PalletExternalValidatorsCall;
        PalletExternalValidatorsError: PalletExternalValidatorsError;
        PalletExternalValidatorsEvent: PalletExternalValidatorsEvent;
        PalletExternalValidatorsForcing: PalletExternalValidatorsForcing;
        PalletGrandpaCall: PalletGrandpaCall;
        PalletGrandpaError: PalletGrandpaError;
        PalletGrandpaEvent: PalletGrandpaEvent;
        PalletGrandpaStoredPendingChange: PalletGrandpaStoredPendingChange;
        PalletGrandpaStoredState: PalletGrandpaStoredState;
        PalletIdentityAuthorityProperties: PalletIdentityAuthorityProperties;
        PalletIdentityCall: PalletIdentityCall;
        PalletIdentityError: PalletIdentityError;
        PalletIdentityEvent: PalletIdentityEvent;
        PalletIdentityJudgement: PalletIdentityJudgement;
        PalletIdentityLegacyIdentityInfo: PalletIdentityLegacyIdentityInfo;
        PalletIdentityRegistrarInfo: PalletIdentityRegistrarInfo;
        PalletIdentityRegistration: PalletIdentityRegistration;
        PalletInflationRewardsChainsToRewardValue: PalletInflationRewardsChainsToRewardValue;
        PalletInflationRewardsEvent: PalletInflationRewardsEvent;
        PalletInvulnerablesCall: PalletInvulnerablesCall;
        PalletInvulnerablesError: PalletInvulnerablesError;
        PalletInvulnerablesEvent: PalletInvulnerablesEvent;
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
        PalletOffencesEvent: PalletOffencesEvent;
        PalletParametersCall: PalletParametersCall;
        PalletParametersEvent: PalletParametersEvent;
        PalletPreimageCall: PalletPreimageCall;
        PalletPreimageError: PalletPreimageError;
        PalletPreimageEvent: PalletPreimageEvent;
        PalletPreimageHoldReason: PalletPreimageHoldReason;
        PalletPreimageOldRequestStatus: PalletPreimageOldRequestStatus;
        PalletPreimageRequestStatus: PalletPreimageRequestStatus;
        PalletProxyAnnouncement: PalletProxyAnnouncement;
        PalletProxyCall: PalletProxyCall;
        PalletProxyError: PalletProxyError;
        PalletProxyEvent: PalletProxyEvent;
        PalletProxyProxyDefinition: PalletProxyProxyDefinition;
        PalletRankedCollectiveCall: PalletRankedCollectiveCall;
        PalletRankedCollectiveError: PalletRankedCollectiveError;
        PalletRankedCollectiveEvent: PalletRankedCollectiveEvent;
        PalletRankedCollectiveMemberRecord: PalletRankedCollectiveMemberRecord;
        PalletRankedCollectiveTally: PalletRankedCollectiveTally;
        PalletRankedCollectiveVoteRecord: PalletRankedCollectiveVoteRecord;
        PalletReferendaCall: PalletReferendaCall;
        PalletReferendaCurve: PalletReferendaCurve;
        PalletReferendaDecidingStatus: PalletReferendaDecidingStatus;
        PalletReferendaDeposit: PalletReferendaDeposit;
        PalletReferendaError: PalletReferendaError;
        PalletReferendaEvent: PalletReferendaEvent;
        PalletReferendaReferendumInfoConvictionVotingTally: PalletReferendaReferendumInfoConvictionVotingTally;
        PalletReferendaReferendumInfoRankedCollectiveTally: PalletReferendaReferendumInfoRankedCollectiveTally;
        PalletReferendaReferendumStatusConvictionVotingTally: PalletReferendaReferendumStatusConvictionVotingTally;
        PalletReferendaReferendumStatusRankedCollectiveTally: PalletReferendaReferendumStatusRankedCollectiveTally;
        PalletReferendaTrackInfo: PalletReferendaTrackInfo;
        PalletRegistrarCall: PalletRegistrarCall;
        PalletRegistrarDepositInfo: PalletRegistrarDepositInfo;
        PalletRegistrarError: PalletRegistrarError;
        PalletRegistrarEvent: PalletRegistrarEvent;
        PalletRegistrarHoldReason: PalletRegistrarHoldReason;
        PalletRootTestingCall: PalletRootTestingCall;
        PalletRootTestingEvent: PalletRootTestingEvent;
        PalletSchedulerCall: PalletSchedulerCall;
        PalletSchedulerError: PalletSchedulerError;
        PalletSchedulerEvent: PalletSchedulerEvent;
        PalletSchedulerRetryConfig: PalletSchedulerRetryConfig;
        PalletSchedulerScheduled: PalletSchedulerScheduled;
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
        PalletTreasuryCall: PalletTreasuryCall;
        PalletTreasuryError: PalletTreasuryError;
        PalletTreasuryEvent: PalletTreasuryEvent;
        PalletTreasuryPaymentState: PalletTreasuryPaymentState;
        PalletTreasuryProposal: PalletTreasuryProposal;
        PalletTreasurySpendStatus: PalletTreasurySpendStatus;
        PalletUtilityCall: PalletUtilityCall;
        PalletUtilityError: PalletUtilityError;
        PalletUtilityEvent: PalletUtilityEvent;
        PalletWhitelistCall: PalletWhitelistCall;
        PalletWhitelistError: PalletWhitelistError;
        PalletWhitelistEvent: PalletWhitelistEvent;
        PalletXcmCall: PalletXcmCall;
        PalletXcmError: PalletXcmError;
        PalletXcmEvent: PalletXcmEvent;
        PalletXcmOrigin: PalletXcmOrigin;
        PalletXcmQueryStatus: PalletXcmQueryStatus;
        PalletXcmRemoteLockedFungibleRecord: PalletXcmRemoteLockedFungibleRecord;
        PalletXcmVersionMigrationStage: PalletXcmVersionMigrationStage;
        PolkadotCorePrimitivesInboundDownwardMessage: PolkadotCorePrimitivesInboundDownwardMessage;
        PolkadotCorePrimitivesInboundHrmpMessage: PolkadotCorePrimitivesInboundHrmpMessage;
        PolkadotCorePrimitivesOutboundHrmpMessage: PolkadotCorePrimitivesOutboundHrmpMessage;
        PolkadotParachainPrimitivesPrimitivesHrmpChannelId: PolkadotParachainPrimitivesPrimitivesHrmpChannelId;
        PolkadotPrimitivesV7ApprovalVotingParams: PolkadotPrimitivesV7ApprovalVotingParams;
        PolkadotPrimitivesV7AssignmentAppPublic: PolkadotPrimitivesV7AssignmentAppPublic;
        PolkadotPrimitivesV7AsyncBackingAsyncBackingParams: PolkadotPrimitivesV7AsyncBackingAsyncBackingParams;
        PolkadotPrimitivesV7BackedCandidate: PolkadotPrimitivesV7BackedCandidate;
        PolkadotPrimitivesV7CandidateCommitments: PolkadotPrimitivesV7CandidateCommitments;
        PolkadotPrimitivesV7CandidateDescriptor: PolkadotPrimitivesV7CandidateDescriptor;
        PolkadotPrimitivesV7CandidateReceipt: PolkadotPrimitivesV7CandidateReceipt;
        PolkadotPrimitivesV7CollatorAppPublic: PolkadotPrimitivesV7CollatorAppPublic;
        PolkadotPrimitivesV7CollatorAppSignature: PolkadotPrimitivesV7CollatorAppSignature;
        PolkadotPrimitivesV7CommittedCandidateReceipt: PolkadotPrimitivesV7CommittedCandidateReceipt;
        PolkadotPrimitivesV7DisputeState: PolkadotPrimitivesV7DisputeState;
        PolkadotPrimitivesV7DisputeStatement: PolkadotPrimitivesV7DisputeStatement;
        PolkadotPrimitivesV7DisputeStatementSet: PolkadotPrimitivesV7DisputeStatementSet;
        PolkadotPrimitivesV7ExecutorParams: PolkadotPrimitivesV7ExecutorParams;
        PolkadotPrimitivesV7ExecutorParamsExecutorParam: PolkadotPrimitivesV7ExecutorParamsExecutorParam;
        PolkadotPrimitivesV7IndexedVecGroupIndex: PolkadotPrimitivesV7IndexedVecGroupIndex;
        PolkadotPrimitivesV7IndexedVecValidatorIndex: PolkadotPrimitivesV7IndexedVecValidatorIndex;
        PolkadotPrimitivesV7InherentData: PolkadotPrimitivesV7InherentData;
        PolkadotPrimitivesV7InvalidDisputeStatementKind: PolkadotPrimitivesV7InvalidDisputeStatementKind;
        PolkadotPrimitivesV7PvfCheckStatement: PolkadotPrimitivesV7PvfCheckStatement;
        PolkadotPrimitivesV7PvfExecKind: PolkadotPrimitivesV7PvfExecKind;
        PolkadotPrimitivesV7PvfPrepKind: PolkadotPrimitivesV7PvfPrepKind;
        PolkadotPrimitivesV7ScrapedOnChainVotes: PolkadotPrimitivesV7ScrapedOnChainVotes;
        PolkadotPrimitivesV7SessionInfo: PolkadotPrimitivesV7SessionInfo;
        PolkadotPrimitivesV7SignedUncheckedSigned: PolkadotPrimitivesV7SignedUncheckedSigned;
        PolkadotPrimitivesV7SlashingDisputeProof: PolkadotPrimitivesV7SlashingDisputeProof;
        PolkadotPrimitivesV7SlashingDisputesTimeSlot: PolkadotPrimitivesV7SlashingDisputesTimeSlot;
        PolkadotPrimitivesV7SlashingPendingSlashes: PolkadotPrimitivesV7SlashingPendingSlashes;
        PolkadotPrimitivesV7SlashingSlashingOffenceKind: PolkadotPrimitivesV7SlashingSlashingOffenceKind;
        PolkadotPrimitivesV7UpgradeGoAhead: PolkadotPrimitivesV7UpgradeGoAhead;
        PolkadotPrimitivesV7UpgradeRestriction: PolkadotPrimitivesV7UpgradeRestriction;
        PolkadotPrimitivesV7ValidDisputeStatementKind: PolkadotPrimitivesV7ValidDisputeStatementKind;
        PolkadotPrimitivesV7ValidatorAppPublic: PolkadotPrimitivesV7ValidatorAppPublic;
        PolkadotPrimitivesV7ValidatorAppSignature: PolkadotPrimitivesV7ValidatorAppSignature;
        PolkadotPrimitivesV7ValidityAttestation: PolkadotPrimitivesV7ValidityAttestation;
        PolkadotPrimitivesVstagingSchedulerParams: PolkadotPrimitivesVstagingSchedulerParams;
        PolkadotRuntimeCommonParasRegistrarPalletCall: PolkadotRuntimeCommonParasRegistrarPalletCall;
        PolkadotRuntimeCommonParasRegistrarPalletError: PolkadotRuntimeCommonParasRegistrarPalletError;
        PolkadotRuntimeCommonParasRegistrarPalletEvent: PolkadotRuntimeCommonParasRegistrarPalletEvent;
        PolkadotRuntimeCommonParasRegistrarParaInfo: PolkadotRuntimeCommonParasRegistrarParaInfo;
        PolkadotRuntimeCommonParasSudoWrapperPalletCall: PolkadotRuntimeCommonParasSudoWrapperPalletCall;
        PolkadotRuntimeCommonParasSudoWrapperPalletError: PolkadotRuntimeCommonParasSudoWrapperPalletError;
        PolkadotRuntimeParachainsAssignerOnDemandPalletCall: PolkadotRuntimeParachainsAssignerOnDemandPalletCall;
        PolkadotRuntimeParachainsAssignerOnDemandPalletError: PolkadotRuntimeParachainsAssignerOnDemandPalletError;
        PolkadotRuntimeParachainsAssignerOnDemandPalletEvent: PolkadotRuntimeParachainsAssignerOnDemandPalletEvent;
        PolkadotRuntimeParachainsAssignerOnDemandTypesCoreAffinityCount: PolkadotRuntimeParachainsAssignerOnDemandTypesCoreAffinityCount;
        PolkadotRuntimeParachainsAssignerOnDemandTypesEnqueuedOrder: PolkadotRuntimeParachainsAssignerOnDemandTypesEnqueuedOrder;
        PolkadotRuntimeParachainsAssignerOnDemandTypesQueueStatusType: PolkadotRuntimeParachainsAssignerOnDemandTypesQueueStatusType;
        PolkadotRuntimeParachainsConfigurationHostConfiguration: PolkadotRuntimeParachainsConfigurationHostConfiguration;
        PolkadotRuntimeParachainsConfigurationPalletCall: PolkadotRuntimeParachainsConfigurationPalletCall;
        PolkadotRuntimeParachainsConfigurationPalletError: PolkadotRuntimeParachainsConfigurationPalletError;
        PolkadotRuntimeParachainsDisputesDisputeLocation: PolkadotRuntimeParachainsDisputesDisputeLocation;
        PolkadotRuntimeParachainsDisputesDisputeResult: PolkadotRuntimeParachainsDisputesDisputeResult;
        PolkadotRuntimeParachainsDisputesPalletCall: PolkadotRuntimeParachainsDisputesPalletCall;
        PolkadotRuntimeParachainsDisputesPalletError: PolkadotRuntimeParachainsDisputesPalletError;
        PolkadotRuntimeParachainsDisputesPalletEvent: PolkadotRuntimeParachainsDisputesPalletEvent;
        PolkadotRuntimeParachainsDisputesSlashingPalletCall: PolkadotRuntimeParachainsDisputesSlashingPalletCall;
        PolkadotRuntimeParachainsDisputesSlashingPalletError: PolkadotRuntimeParachainsDisputesSlashingPalletError;
        PolkadotRuntimeParachainsHrmpHrmpChannel: PolkadotRuntimeParachainsHrmpHrmpChannel;
        PolkadotRuntimeParachainsHrmpHrmpOpenChannelRequest: PolkadotRuntimeParachainsHrmpHrmpOpenChannelRequest;
        PolkadotRuntimeParachainsHrmpPalletCall: PolkadotRuntimeParachainsHrmpPalletCall;
        PolkadotRuntimeParachainsHrmpPalletError: PolkadotRuntimeParachainsHrmpPalletError;
        PolkadotRuntimeParachainsHrmpPalletEvent: PolkadotRuntimeParachainsHrmpPalletEvent;
        PolkadotRuntimeParachainsInclusionAggregateMessageOrigin: PolkadotRuntimeParachainsInclusionAggregateMessageOrigin;
        PolkadotRuntimeParachainsInclusionCandidatePendingAvailability: PolkadotRuntimeParachainsInclusionCandidatePendingAvailability;
        PolkadotRuntimeParachainsInclusionPalletCall: PolkadotRuntimeParachainsInclusionPalletCall;
        PolkadotRuntimeParachainsInclusionPalletError: PolkadotRuntimeParachainsInclusionPalletError;
        PolkadotRuntimeParachainsInclusionPalletEvent: PolkadotRuntimeParachainsInclusionPalletEvent;
        PolkadotRuntimeParachainsInclusionUmpQueueId: PolkadotRuntimeParachainsInclusionUmpQueueId;
        PolkadotRuntimeParachainsInitializerBufferedSessionChange: PolkadotRuntimeParachainsInitializerBufferedSessionChange;
        PolkadotRuntimeParachainsInitializerPalletCall: PolkadotRuntimeParachainsInitializerPalletCall;
        PolkadotRuntimeParachainsOriginPalletOrigin: PolkadotRuntimeParachainsOriginPalletOrigin;
        PolkadotRuntimeParachainsParasInherentPalletCall: PolkadotRuntimeParachainsParasInherentPalletCall;
        PolkadotRuntimeParachainsParasInherentPalletError: PolkadotRuntimeParachainsParasInherentPalletError;
        PolkadotRuntimeParachainsParasPalletCall: PolkadotRuntimeParachainsParasPalletCall;
        PolkadotRuntimeParachainsParasPalletError: PolkadotRuntimeParachainsParasPalletError;
        PolkadotRuntimeParachainsParasPalletEvent: PolkadotRuntimeParachainsParasPalletEvent;
        PolkadotRuntimeParachainsParasParaGenesisArgs: PolkadotRuntimeParachainsParasParaGenesisArgs;
        PolkadotRuntimeParachainsParasParaLifecycle: PolkadotRuntimeParachainsParasParaLifecycle;
        PolkadotRuntimeParachainsParasParaPastCodeMeta: PolkadotRuntimeParachainsParasParaPastCodeMeta;
        PolkadotRuntimeParachainsParasPvfCheckActiveVoteState: PolkadotRuntimeParachainsParasPvfCheckActiveVoteState;
        PolkadotRuntimeParachainsParasPvfCheckCause: PolkadotRuntimeParachainsParasPvfCheckCause;
        PolkadotRuntimeParachainsParasReplacementTimes: PolkadotRuntimeParachainsParasReplacementTimes;
        PolkadotRuntimeParachainsParasUpgradeStrategy: PolkadotRuntimeParachainsParasUpgradeStrategy;
        PolkadotRuntimeParachainsSchedulerCommonAssignment: PolkadotRuntimeParachainsSchedulerCommonAssignment;
        PolkadotRuntimeParachainsSchedulerPalletCoreOccupied: PolkadotRuntimeParachainsSchedulerPalletCoreOccupied;
        PolkadotRuntimeParachainsSchedulerPalletParasEntry: PolkadotRuntimeParachainsSchedulerPalletParasEntry;
        PolkadotRuntimeParachainsSharedAllowedRelayParentsTracker: PolkadotRuntimeParachainsSharedAllowedRelayParentsTracker;
        PolkadotRuntimeParachainsSharedPalletCall: PolkadotRuntimeParachainsSharedPalletCall;
        SnowbridgeAmclBls381Big: SnowbridgeAmclBls381Big;
        SnowbridgeAmclBls381Ecp: SnowbridgeAmclBls381Ecp;
        SnowbridgeAmclBls381Fp: SnowbridgeAmclBls381Fp;
        SnowbridgeBeaconPrimitivesBeaconHeader: SnowbridgeBeaconPrimitivesBeaconHeader;
        SnowbridgeBeaconPrimitivesBlsBlsError: SnowbridgeBeaconPrimitivesBlsBlsError;
        SnowbridgeBeaconPrimitivesCompactBeaconState: SnowbridgeBeaconPrimitivesCompactBeaconState;
        SnowbridgeBeaconPrimitivesFork: SnowbridgeBeaconPrimitivesFork;
        SnowbridgeBeaconPrimitivesForkVersions: SnowbridgeBeaconPrimitivesForkVersions;
        SnowbridgeBeaconPrimitivesPublicKey: SnowbridgeBeaconPrimitivesPublicKey;
        SnowbridgeBeaconPrimitivesSignature: SnowbridgeBeaconPrimitivesSignature;
        SnowbridgeBeaconPrimitivesSyncAggregate: SnowbridgeBeaconPrimitivesSyncAggregate;
        SnowbridgeBeaconPrimitivesSyncCommittee: SnowbridgeBeaconPrimitivesSyncCommittee;
        SnowbridgeBeaconPrimitivesSyncCommitteePrepared: SnowbridgeBeaconPrimitivesSyncCommitteePrepared;
        SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate: SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate;
        SnowbridgeBeaconPrimitivesUpdatesNextSyncCommitteeUpdate: SnowbridgeBeaconPrimitivesUpdatesNextSyncCommitteeUpdate;
        SnowbridgeBeaconPrimitivesUpdatesUpdate: SnowbridgeBeaconPrimitivesUpdatesUpdate;
        SnowbridgeCoreOperatingModeBasicOperatingMode: SnowbridgeCoreOperatingModeBasicOperatingMode;
        SnowbridgeMilagroBlsKeysPublicKey: SnowbridgeMilagroBlsKeysPublicKey;
        SnowbridgePalletEthereumClientCall: SnowbridgePalletEthereumClientCall;
        SnowbridgePalletEthereumClientError: SnowbridgePalletEthereumClientError;
        SnowbridgePalletEthereumClientEvent: SnowbridgePalletEthereumClientEvent;
        SpArithmeticArithmeticError: SpArithmeticArithmeticError;
        SpAuthorityDiscoveryAppPublic: SpAuthorityDiscoveryAppPublic;
        SpConsensusBabeAllowedSlots: SpConsensusBabeAllowedSlots;
        SpConsensusBabeAppPublic: SpConsensusBabeAppPublic;
        SpConsensusBabeBabeEpochConfiguration: SpConsensusBabeBabeEpochConfiguration;
        SpConsensusBabeDigestsNextConfigDescriptor: SpConsensusBabeDigestsNextConfigDescriptor;
        SpConsensusBabeDigestsPreDigest: SpConsensusBabeDigestsPreDigest;
        SpConsensusBabeDigestsPrimaryPreDigest: SpConsensusBabeDigestsPrimaryPreDigest;
        SpConsensusBabeDigestsSecondaryPlainPreDigest: SpConsensusBabeDigestsSecondaryPlainPreDigest;
        SpConsensusBabeDigestsSecondaryVRFPreDigest: SpConsensusBabeDigestsSecondaryVRFPreDigest;
        SpConsensusBeefyCommitment: SpConsensusBeefyCommitment;
        SpConsensusBeefyDoubleVotingProof: SpConsensusBeefyDoubleVotingProof;
        SpConsensusBeefyEcdsaCryptoPublic: SpConsensusBeefyEcdsaCryptoPublic;
        SpConsensusBeefyEcdsaCryptoSignature: SpConsensusBeefyEcdsaCryptoSignature;
        SpConsensusBeefyForkVotingProof: SpConsensusBeefyForkVotingProof;
        SpConsensusBeefyFutureBlockVotingProof: SpConsensusBeefyFutureBlockVotingProof;
        SpConsensusBeefyMmrBeefyAuthoritySet: SpConsensusBeefyMmrBeefyAuthoritySet;
        SpConsensusBeefyPayload: SpConsensusBeefyPayload;
        SpConsensusBeefyVoteMessage: SpConsensusBeefyVoteMessage;
        SpConsensusGrandpaAppPublic: SpConsensusGrandpaAppPublic;
        SpConsensusGrandpaAppSignature: SpConsensusGrandpaAppSignature;
        SpConsensusGrandpaEquivocation: SpConsensusGrandpaEquivocation;
        SpConsensusGrandpaEquivocationProof: SpConsensusGrandpaEquivocationProof;
        SpConsensusSlotsEquivocationProof: SpConsensusSlotsEquivocationProof;
        SpCoreCryptoKeyTypeId: SpCoreCryptoKeyTypeId;
        SpCoreSr25519VrfVrfSignature: SpCoreSr25519VrfVrfSignature;
        SpCoreVoid: SpCoreVoid;
        SpMmrPrimitivesAncestryProof: SpMmrPrimitivesAncestryProof;
        SpRuntimeBlakeTwo256: SpRuntimeBlakeTwo256;
        SpRuntimeDigest: SpRuntimeDigest;
        SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
        SpRuntimeDispatchError: SpRuntimeDispatchError;
        SpRuntimeDispatchErrorWithPostInfo: SpRuntimeDispatchErrorWithPostInfo;
        SpRuntimeHeader: SpRuntimeHeader;
        SpRuntimeModuleError: SpRuntimeModuleError;
        SpRuntimeMultiSignature: SpRuntimeMultiSignature;
        SpRuntimeTokenError: SpRuntimeTokenError;
        SpRuntimeTransactionalError: SpRuntimeTransactionalError;
        SpSessionMembershipProof: SpSessionMembershipProof;
        SpStakingOffenceOffenceDetails: SpStakingOffenceOffenceDetails;
        SpTrieStorageProof: SpTrieStorageProof;
        SpVersionRuntimeVersion: SpVersionRuntimeVersion;
        SpWeightsRuntimeDbWeight: SpWeightsRuntimeDbWeight;
        SpWeightsWeightV2Weight: SpWeightsWeightV2Weight;
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
        StagingXcmV4TraitsOutcome: StagingXcmV4TraitsOutcome;
        StagingXcmV4Xcm: StagingXcmV4Xcm;
        TpTraitsActiveEraInfo: TpTraitsActiveEraInfo;
        TpTraitsContainerChainBlockInfo: TpTraitsContainerChainBlockInfo;
        TpTraitsParathreadParams: TpTraitsParathreadParams;
        TpTraitsSlotFrequency: TpTraitsSlotFrequency;
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
        XcmV3OriginKind: XcmV3OriginKind;
        XcmV3PalletInfo: XcmV3PalletInfo;
        XcmV3QueryResponseInfo: XcmV3QueryResponseInfo;
        XcmV3Response: XcmV3Response;
        XcmV3TraitsError: XcmV3TraitsError;
        XcmV3WeightLimit: XcmV3WeightLimit;
        XcmV3Xcm: XcmV3Xcm;
        XcmVersionedAssetId: XcmVersionedAssetId;
        XcmVersionedAssets: XcmVersionedAssets;
        XcmVersionedLocation: XcmVersionedLocation;
        XcmVersionedResponse: XcmVersionedResponse;
        XcmVersionedXcm: XcmVersionedXcm;
    } // InterfaceTypes
} // declare module
