// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/types/types/registry";

import type {
    BinaryHeapEnqueuedOrder,
    BinaryHeapReverseQueueIndex,
    BitvecOrderLsb0,
    DancelightRuntimeAggregateMessageOrigin,
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
    FrameMetadataHashExtensionCheckMetadataHash,
    FrameMetadataHashExtensionMode,
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
    PalletExternalValidatorSlashesCall,
    PalletExternalValidatorSlashesError,
    PalletExternalValidatorSlashesEvent,
    PalletExternalValidatorSlashesSlash,
    PalletExternalValidatorsCall,
    PalletExternalValidatorsError,
    PalletExternalValidatorsEvent,
    PalletExternalValidatorsForcing,
    PalletExternalValidatorsRewardsEraRewardPoints,
    PalletExternalValidatorsRewardsEvent,
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
    PalletOutboundMessageCommitmentRecorderEvent,
    PalletParametersCall,
    PalletParametersEvent,
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
    PolkadotPrimitivesV8ApprovalVotingParams,
    PolkadotPrimitivesV8AssignmentAppPublic,
    PolkadotPrimitivesV8AsyncBackingAsyncBackingParams,
    PolkadotPrimitivesV8BackedCandidate,
    PolkadotPrimitivesV8CandidateCommitments,
    PolkadotPrimitivesV8CandidateDescriptor,
    PolkadotPrimitivesV8CandidateReceipt,
    PolkadotPrimitivesV8CollatorAppPublic,
    PolkadotPrimitivesV8CollatorAppSignature,
    PolkadotPrimitivesV8CommittedCandidateReceipt,
    PolkadotPrimitivesV8DisputeState,
    PolkadotPrimitivesV8DisputeStatement,
    PolkadotPrimitivesV8DisputeStatementSet,
    PolkadotPrimitivesV8ExecutorParams,
    PolkadotPrimitivesV8ExecutorParamsExecutorParam,
    PolkadotPrimitivesV8IndexedVecGroupIndex,
    PolkadotPrimitivesV8IndexedVecValidatorIndex,
    PolkadotPrimitivesV8InherentData,
    PolkadotPrimitivesV8InvalidDisputeStatementKind,
    PolkadotPrimitivesV8PvfCheckStatement,
    PolkadotPrimitivesV8PvfExecKind,
    PolkadotPrimitivesV8PvfPrepKind,
    PolkadotPrimitivesV8SchedulerParams,
    PolkadotPrimitivesV8ScrapedOnChainVotes,
    PolkadotPrimitivesV8SessionInfo,
    PolkadotPrimitivesV8SignedUncheckedSigned,
    PolkadotPrimitivesV8SlashingDisputeProof,
    PolkadotPrimitivesV8SlashingDisputesTimeSlot,
    PolkadotPrimitivesV8SlashingPendingSlashes,
    PolkadotPrimitivesV8SlashingSlashingOffenceKind,
    PolkadotPrimitivesV8UpgradeGoAhead,
    PolkadotPrimitivesV8UpgradeRestriction,
    PolkadotPrimitivesV8ValidDisputeStatementKind,
    PolkadotPrimitivesV8ValidatorAppPublic,
    PolkadotPrimitivesV8ValidatorAppSignature,
    PolkadotPrimitivesV8ValidityAttestation,
    PolkadotRuntimeCommonParasRegistrarPalletCall,
    PolkadotRuntimeCommonParasRegistrarPalletError,
    PolkadotRuntimeCommonParasRegistrarPalletEvent,
    PolkadotRuntimeCommonParasRegistrarParaInfo,
    PolkadotRuntimeCommonParasSudoWrapperPalletCall,
    PolkadotRuntimeCommonParasSudoWrapperPalletError,
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
    PolkadotRuntimeParachainsInclusionCandidatePendingAvailability,
    PolkadotRuntimeParachainsInclusionPalletCall,
    PolkadotRuntimeParachainsInclusionPalletError,
    PolkadotRuntimeParachainsInclusionPalletEvent,
    PolkadotRuntimeParachainsInclusionUmpQueueId,
    PolkadotRuntimeParachainsInitializerBufferedSessionChange,
    PolkadotRuntimeParachainsInitializerPalletCall,
    PolkadotRuntimeParachainsOnDemandPalletCall,
    PolkadotRuntimeParachainsOnDemandPalletError,
    PolkadotRuntimeParachainsOnDemandPalletEvent,
    PolkadotRuntimeParachainsOnDemandTypesCoreAffinityCount,
    PolkadotRuntimeParachainsOnDemandTypesEnqueuedOrder,
    PolkadotRuntimeParachainsOnDemandTypesQueueStatusType,
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
    SnowbridgeBeaconPrimitivesAncestryProof,
    SnowbridgeBeaconPrimitivesBeaconHeader,
    SnowbridgeBeaconPrimitivesBlsBlsError,
    SnowbridgeBeaconPrimitivesCompactBeaconState,
    SnowbridgeBeaconPrimitivesDenebExecutionPayloadHeader,
    SnowbridgeBeaconPrimitivesExecutionPayloadHeader,
    SnowbridgeBeaconPrimitivesExecutionProof,
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
    SnowbridgeBeaconPrimitivesVersionedExecutionPayloadHeader,
    SnowbridgeCoreAssetMetadata,
    SnowbridgeCoreChannel,
    SnowbridgeCoreChannelId,
    SnowbridgeCoreInboundLog,
    SnowbridgeCoreInboundMessage,
    SnowbridgeCoreInboundProof,
    SnowbridgeCoreInboundVerificationError,
    SnowbridgeCoreOperatingModeBasicOperatingMode,
    SnowbridgeCoreOutboundSendError,
    SnowbridgeCoreOutboundV1Initializer,
    SnowbridgeCoreOutboundV1OperatingMode,
    SnowbridgeCorePricingPricingParameters,
    SnowbridgeCorePricingRewards,
    SnowbridgeMilagroBlsKeysPublicKey,
    SnowbridgePalletEthereumClientCall,
    SnowbridgePalletEthereumClientError,
    SnowbridgePalletEthereumClientEvent,
    SnowbridgePalletInboundQueueCall,
    SnowbridgePalletInboundQueueError,
    SnowbridgePalletInboundQueueEvent,
    SnowbridgePalletInboundQueueSendError,
    SnowbridgePalletOutboundQueueCall,
    SnowbridgePalletOutboundQueueCommittedMessage,
    SnowbridgePalletOutboundQueueError,
    SnowbridgePalletOutboundQueueEvent,
    SnowbridgePalletSystemCall,
    SnowbridgePalletSystemError,
    SnowbridgePalletSystemEvent,
    SnowbridgeRouterPrimitivesInboundConvertMessageError,
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
    TpBridgeCommand,
    TpTraitsActiveEraInfo,
    TpTraitsContainerChainBlockInfo,
    TpTraitsFullRotationMode,
    TpTraitsFullRotationModes,
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
        DancelightRuntimeAggregateMessageOrigin: DancelightRuntimeAggregateMessageOrigin;
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
        FrameMetadataHashExtensionCheckMetadataHash: FrameMetadataHashExtensionCheckMetadataHash;
        FrameMetadataHashExtensionMode: FrameMetadataHashExtensionMode;
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
        PalletExternalValidatorSlashesCall: PalletExternalValidatorSlashesCall;
        PalletExternalValidatorSlashesError: PalletExternalValidatorSlashesError;
        PalletExternalValidatorSlashesEvent: PalletExternalValidatorSlashesEvent;
        PalletExternalValidatorSlashesSlash: PalletExternalValidatorSlashesSlash;
        PalletExternalValidatorsCall: PalletExternalValidatorsCall;
        PalletExternalValidatorsError: PalletExternalValidatorsError;
        PalletExternalValidatorsEvent: PalletExternalValidatorsEvent;
        PalletExternalValidatorsForcing: PalletExternalValidatorsForcing;
        PalletExternalValidatorsRewardsEraRewardPoints: PalletExternalValidatorsRewardsEraRewardPoints;
        PalletExternalValidatorsRewardsEvent: PalletExternalValidatorsRewardsEvent;
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
        PalletOutboundMessageCommitmentRecorderEvent: PalletOutboundMessageCommitmentRecorderEvent;
        PalletParametersCall: PalletParametersCall;
        PalletParametersEvent: PalletParametersEvent;
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
        PolkadotPrimitivesV8ApprovalVotingParams: PolkadotPrimitivesV8ApprovalVotingParams;
        PolkadotPrimitivesV8AssignmentAppPublic: PolkadotPrimitivesV8AssignmentAppPublic;
        PolkadotPrimitivesV8AsyncBackingAsyncBackingParams: PolkadotPrimitivesV8AsyncBackingAsyncBackingParams;
        PolkadotPrimitivesV8BackedCandidate: PolkadotPrimitivesV8BackedCandidate;
        PolkadotPrimitivesV8CandidateCommitments: PolkadotPrimitivesV8CandidateCommitments;
        PolkadotPrimitivesV8CandidateDescriptor: PolkadotPrimitivesV8CandidateDescriptor;
        PolkadotPrimitivesV8CandidateReceipt: PolkadotPrimitivesV8CandidateReceipt;
        PolkadotPrimitivesV8CollatorAppPublic: PolkadotPrimitivesV8CollatorAppPublic;
        PolkadotPrimitivesV8CollatorAppSignature: PolkadotPrimitivesV8CollatorAppSignature;
        PolkadotPrimitivesV8CommittedCandidateReceipt: PolkadotPrimitivesV8CommittedCandidateReceipt;
        PolkadotPrimitivesV8DisputeState: PolkadotPrimitivesV8DisputeState;
        PolkadotPrimitivesV8DisputeStatement: PolkadotPrimitivesV8DisputeStatement;
        PolkadotPrimitivesV8DisputeStatementSet: PolkadotPrimitivesV8DisputeStatementSet;
        PolkadotPrimitivesV8ExecutorParams: PolkadotPrimitivesV8ExecutorParams;
        PolkadotPrimitivesV8ExecutorParamsExecutorParam: PolkadotPrimitivesV8ExecutorParamsExecutorParam;
        PolkadotPrimitivesV8IndexedVecGroupIndex: PolkadotPrimitivesV8IndexedVecGroupIndex;
        PolkadotPrimitivesV8IndexedVecValidatorIndex: PolkadotPrimitivesV8IndexedVecValidatorIndex;
        PolkadotPrimitivesV8InherentData: PolkadotPrimitivesV8InherentData;
        PolkadotPrimitivesV8InvalidDisputeStatementKind: PolkadotPrimitivesV8InvalidDisputeStatementKind;
        PolkadotPrimitivesV8PvfCheckStatement: PolkadotPrimitivesV8PvfCheckStatement;
        PolkadotPrimitivesV8PvfExecKind: PolkadotPrimitivesV8PvfExecKind;
        PolkadotPrimitivesV8PvfPrepKind: PolkadotPrimitivesV8PvfPrepKind;
        PolkadotPrimitivesV8SchedulerParams: PolkadotPrimitivesV8SchedulerParams;
        PolkadotPrimitivesV8ScrapedOnChainVotes: PolkadotPrimitivesV8ScrapedOnChainVotes;
        PolkadotPrimitivesV8SessionInfo: PolkadotPrimitivesV8SessionInfo;
        PolkadotPrimitivesV8SignedUncheckedSigned: PolkadotPrimitivesV8SignedUncheckedSigned;
        PolkadotPrimitivesV8SlashingDisputeProof: PolkadotPrimitivesV8SlashingDisputeProof;
        PolkadotPrimitivesV8SlashingDisputesTimeSlot: PolkadotPrimitivesV8SlashingDisputesTimeSlot;
        PolkadotPrimitivesV8SlashingPendingSlashes: PolkadotPrimitivesV8SlashingPendingSlashes;
        PolkadotPrimitivesV8SlashingSlashingOffenceKind: PolkadotPrimitivesV8SlashingSlashingOffenceKind;
        PolkadotPrimitivesV8UpgradeGoAhead: PolkadotPrimitivesV8UpgradeGoAhead;
        PolkadotPrimitivesV8UpgradeRestriction: PolkadotPrimitivesV8UpgradeRestriction;
        PolkadotPrimitivesV8ValidDisputeStatementKind: PolkadotPrimitivesV8ValidDisputeStatementKind;
        PolkadotPrimitivesV8ValidatorAppPublic: PolkadotPrimitivesV8ValidatorAppPublic;
        PolkadotPrimitivesV8ValidatorAppSignature: PolkadotPrimitivesV8ValidatorAppSignature;
        PolkadotPrimitivesV8ValidityAttestation: PolkadotPrimitivesV8ValidityAttestation;
        PolkadotRuntimeCommonParasRegistrarPalletCall: PolkadotRuntimeCommonParasRegistrarPalletCall;
        PolkadotRuntimeCommonParasRegistrarPalletError: PolkadotRuntimeCommonParasRegistrarPalletError;
        PolkadotRuntimeCommonParasRegistrarPalletEvent: PolkadotRuntimeCommonParasRegistrarPalletEvent;
        PolkadotRuntimeCommonParasRegistrarParaInfo: PolkadotRuntimeCommonParasRegistrarParaInfo;
        PolkadotRuntimeCommonParasSudoWrapperPalletCall: PolkadotRuntimeCommonParasSudoWrapperPalletCall;
        PolkadotRuntimeCommonParasSudoWrapperPalletError: PolkadotRuntimeCommonParasSudoWrapperPalletError;
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
        PolkadotRuntimeParachainsInclusionCandidatePendingAvailability: PolkadotRuntimeParachainsInclusionCandidatePendingAvailability;
        PolkadotRuntimeParachainsInclusionPalletCall: PolkadotRuntimeParachainsInclusionPalletCall;
        PolkadotRuntimeParachainsInclusionPalletError: PolkadotRuntimeParachainsInclusionPalletError;
        PolkadotRuntimeParachainsInclusionPalletEvent: PolkadotRuntimeParachainsInclusionPalletEvent;
        PolkadotRuntimeParachainsInclusionUmpQueueId: PolkadotRuntimeParachainsInclusionUmpQueueId;
        PolkadotRuntimeParachainsInitializerBufferedSessionChange: PolkadotRuntimeParachainsInitializerBufferedSessionChange;
        PolkadotRuntimeParachainsInitializerPalletCall: PolkadotRuntimeParachainsInitializerPalletCall;
        PolkadotRuntimeParachainsOnDemandPalletCall: PolkadotRuntimeParachainsOnDemandPalletCall;
        PolkadotRuntimeParachainsOnDemandPalletError: PolkadotRuntimeParachainsOnDemandPalletError;
        PolkadotRuntimeParachainsOnDemandPalletEvent: PolkadotRuntimeParachainsOnDemandPalletEvent;
        PolkadotRuntimeParachainsOnDemandTypesCoreAffinityCount: PolkadotRuntimeParachainsOnDemandTypesCoreAffinityCount;
        PolkadotRuntimeParachainsOnDemandTypesEnqueuedOrder: PolkadotRuntimeParachainsOnDemandTypesEnqueuedOrder;
        PolkadotRuntimeParachainsOnDemandTypesQueueStatusType: PolkadotRuntimeParachainsOnDemandTypesQueueStatusType;
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
        SnowbridgeBeaconPrimitivesAncestryProof: SnowbridgeBeaconPrimitivesAncestryProof;
        SnowbridgeBeaconPrimitivesBeaconHeader: SnowbridgeBeaconPrimitivesBeaconHeader;
        SnowbridgeBeaconPrimitivesBlsBlsError: SnowbridgeBeaconPrimitivesBlsBlsError;
        SnowbridgeBeaconPrimitivesCompactBeaconState: SnowbridgeBeaconPrimitivesCompactBeaconState;
        SnowbridgeBeaconPrimitivesDenebExecutionPayloadHeader: SnowbridgeBeaconPrimitivesDenebExecutionPayloadHeader;
        SnowbridgeBeaconPrimitivesExecutionPayloadHeader: SnowbridgeBeaconPrimitivesExecutionPayloadHeader;
        SnowbridgeBeaconPrimitivesExecutionProof: SnowbridgeBeaconPrimitivesExecutionProof;
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
        SnowbridgeBeaconPrimitivesVersionedExecutionPayloadHeader: SnowbridgeBeaconPrimitivesVersionedExecutionPayloadHeader;
        SnowbridgeCoreAssetMetadata: SnowbridgeCoreAssetMetadata;
        SnowbridgeCoreChannel: SnowbridgeCoreChannel;
        SnowbridgeCoreChannelId: SnowbridgeCoreChannelId;
        SnowbridgeCoreInboundLog: SnowbridgeCoreInboundLog;
        SnowbridgeCoreInboundMessage: SnowbridgeCoreInboundMessage;
        SnowbridgeCoreInboundProof: SnowbridgeCoreInboundProof;
        SnowbridgeCoreInboundVerificationError: SnowbridgeCoreInboundVerificationError;
        SnowbridgeCoreOperatingModeBasicOperatingMode: SnowbridgeCoreOperatingModeBasicOperatingMode;
        SnowbridgeCoreOutboundSendError: SnowbridgeCoreOutboundSendError;
        SnowbridgeCoreOutboundV1Initializer: SnowbridgeCoreOutboundV1Initializer;
        SnowbridgeCoreOutboundV1OperatingMode: SnowbridgeCoreOutboundV1OperatingMode;
        SnowbridgeCorePricingPricingParameters: SnowbridgeCorePricingPricingParameters;
        SnowbridgeCorePricingRewards: SnowbridgeCorePricingRewards;
        SnowbridgeMilagroBlsKeysPublicKey: SnowbridgeMilagroBlsKeysPublicKey;
        SnowbridgePalletEthereumClientCall: SnowbridgePalletEthereumClientCall;
        SnowbridgePalletEthereumClientError: SnowbridgePalletEthereumClientError;
        SnowbridgePalletEthereumClientEvent: SnowbridgePalletEthereumClientEvent;
        SnowbridgePalletInboundQueueCall: SnowbridgePalletInboundQueueCall;
        SnowbridgePalletInboundQueueError: SnowbridgePalletInboundQueueError;
        SnowbridgePalletInboundQueueEvent: SnowbridgePalletInboundQueueEvent;
        SnowbridgePalletInboundQueueSendError: SnowbridgePalletInboundQueueSendError;
        SnowbridgePalletOutboundQueueCall: SnowbridgePalletOutboundQueueCall;
        SnowbridgePalletOutboundQueueCommittedMessage: SnowbridgePalletOutboundQueueCommittedMessage;
        SnowbridgePalletOutboundQueueError: SnowbridgePalletOutboundQueueError;
        SnowbridgePalletOutboundQueueEvent: SnowbridgePalletOutboundQueueEvent;
        SnowbridgePalletSystemCall: SnowbridgePalletSystemCall;
        SnowbridgePalletSystemError: SnowbridgePalletSystemError;
        SnowbridgePalletSystemEvent: SnowbridgePalletSystemEvent;
        SnowbridgeRouterPrimitivesInboundConvertMessageError: SnowbridgeRouterPrimitivesInboundConvertMessageError;
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
        TpBridgeCommand: TpBridgeCommand;
        TpTraitsActiveEraInfo: TpTraitsActiveEraInfo;
        TpTraitsContainerChainBlockInfo: TpTraitsContainerChainBlockInfo;
        TpTraitsFullRotationMode: TpTraitsFullRotationMode;
        TpTraitsFullRotationModes: TpTraitsFullRotationModes;
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
