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
    FrameSupportStorageDisabled,
    FrameSupportTokensMiscBalanceStatus,
    FrameSupportTokensMiscIdAmount,
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
    PalletEthereumTokenTransfersCall,
    PalletEthereumTokenTransfersError,
    PalletEthereumTokenTransfersEvent,
    PalletExternalValidatorSlashesCall,
    PalletExternalValidatorSlashesError,
    PalletExternalValidatorSlashesEvent,
    PalletExternalValidatorSlashesSlash,
    PalletExternalValidatorSlashesSlashingModeOption,
    PalletExternalValidatorsCall,
    PalletExternalValidatorsError,
    PalletExternalValidatorsEvent,
    PalletExternalValidatorsForcing,
    PalletExternalValidatorsRewardsEraRewardPoints,
    PalletExternalValidatorsRewardsEvent,
    PalletForeignAssetCreatorCall,
    PalletForeignAssetCreatorError,
    PalletForeignAssetCreatorEvent,
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
    PalletOffencesEvent,
    PalletOutboundMessageCommitmentRecorderEvent,
    PalletParametersCall,
    PalletParametersEvent,
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
    PalletPreimageCall,
    PalletPreimageError,
    PalletPreimageEvent,
    PalletPreimageHoldReason,
    PalletPreimageOldRequestStatus,
    PalletPreimageRequestStatus,
    PalletProxyAnnouncement,
    PalletProxyCall,
    PalletProxyDepositKind,
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
    PalletReferendaTrackDetails,
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
    PalletStreamPaymentCall,
    PalletStreamPaymentChangeKind,
    PalletStreamPaymentChangeRequest,
    PalletStreamPaymentDepositChange,
    PalletStreamPaymentError,
    PalletStreamPaymentEvent,
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
    PalletUtilityCall,
    PalletUtilityError,
    PalletUtilityEvent,
    PalletWhitelistCall,
    PalletWhitelistError,
    PalletWhitelistEvent,
    PalletXcmAuthorizedAliasesEntry,
    PalletXcmCall,
    PalletXcmError,
    PalletXcmEvent,
    PalletXcmHoldReason,
    PalletXcmMaxAuthorizedAliases,
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
    PolkadotPrimitivesV8CandidateCommitments,
    PolkadotPrimitivesV8DisputeState,
    PolkadotPrimitivesV8DisputeStatement,
    PolkadotPrimitivesV8DisputeStatementSet,
    PolkadotPrimitivesV8ExecutorParams,
    PolkadotPrimitivesV8ExecutorParamsExecutorParam,
    PolkadotPrimitivesV8IndexedVecGroupIndex,
    PolkadotPrimitivesV8IndexedVecValidatorIndex,
    PolkadotPrimitivesV8InvalidDisputeStatementKind,
    PolkadotPrimitivesV8PvfCheckStatement,
    PolkadotPrimitivesV8PvfExecKind,
    PolkadotPrimitivesV8PvfPrepKind,
    PolkadotPrimitivesV8SchedulerParams,
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
    PolkadotPrimitivesVstagingBackedCandidate,
    PolkadotPrimitivesVstagingCandidateDescriptorV2,
    PolkadotPrimitivesVstagingCandidateReceiptV2,
    PolkadotPrimitivesVstagingCommittedCandidateReceiptV2,
    PolkadotPrimitivesVstagingInherentData,
    PolkadotPrimitivesVstagingScrapedOnChainVotes,
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
    PolkadotRuntimeParachainsSharedAllowedRelayParentsTracker,
    PolkadotRuntimeParachainsSharedPalletCall,
    PolkadotRuntimeParachainsSharedRelayParentInfo,
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
    SnowbridgeCoreOperatingModeBasicOperatingMode,
    SnowbridgeCorePricingPricingParameters,
    SnowbridgeCorePricingRewards,
    SnowbridgeInboundQueuePrimitivesV1ConvertMessageError,
    SnowbridgeMilagroBlsKeysPublicKey,
    SnowbridgeOutboundQueuePrimitivesOperatingMode,
    SnowbridgeOutboundQueuePrimitivesSendError,
    SnowbridgeOutboundQueuePrimitivesV1MessageInitializer,
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
    SnowbridgeVerificationPrimitivesEventProof,
    SnowbridgeVerificationPrimitivesLog,
    SnowbridgeVerificationPrimitivesProof,
    SnowbridgeVerificationPrimitivesVerificationError,
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
    SpMmrPrimitivesAncestryProof,
    SpRuntimeBlakeTwo256,
    SpRuntimeDigest,
    SpRuntimeDigestDigestItem,
    SpRuntimeDispatchError,
    SpRuntimeDispatchErrorWithPostInfo,
    SpRuntimeHeader,
    SpRuntimeModuleError,
    SpRuntimeMultiSignature,
    SpRuntimeProvingTrieTrieError,
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
    TpBridgeChannelInfo,
    TpBridgeCommand,
    TpBridgeSlashData,
    TpDataPreserversCommonAssignerExtra,
    TpDataPreserversCommonAssignmentWitness,
    TpDataPreserversCommonProviderRequest,
    TpStreamPaymentCommonAssetId,
    TpStreamPaymentCommonTimeUnit,
    TpTraitsActiveEraInfo,
    TpTraitsContainerChainBlockInfo,
    TpTraitsFullRotationMode,
    TpTraitsFullRotationModes,
    TpTraitsParathreadParams,
    TpTraitsSlotFrequency,
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
        FrameSupportStorageDisabled: FrameSupportStorageDisabled;
        FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
        FrameSupportTokensMiscIdAmount: FrameSupportTokensMiscIdAmount;
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
        PalletEthereumTokenTransfersCall: PalletEthereumTokenTransfersCall;
        PalletEthereumTokenTransfersError: PalletEthereumTokenTransfersError;
        PalletEthereumTokenTransfersEvent: PalletEthereumTokenTransfersEvent;
        PalletExternalValidatorSlashesCall: PalletExternalValidatorSlashesCall;
        PalletExternalValidatorSlashesError: PalletExternalValidatorSlashesError;
        PalletExternalValidatorSlashesEvent: PalletExternalValidatorSlashesEvent;
        PalletExternalValidatorSlashesSlash: PalletExternalValidatorSlashesSlash;
        PalletExternalValidatorSlashesSlashingModeOption: PalletExternalValidatorSlashesSlashingModeOption;
        PalletExternalValidatorsCall: PalletExternalValidatorsCall;
        PalletExternalValidatorsError: PalletExternalValidatorsError;
        PalletExternalValidatorsEvent: PalletExternalValidatorsEvent;
        PalletExternalValidatorsForcing: PalletExternalValidatorsForcing;
        PalletExternalValidatorsRewardsEraRewardPoints: PalletExternalValidatorsRewardsEraRewardPoints;
        PalletExternalValidatorsRewardsEvent: PalletExternalValidatorsRewardsEvent;
        PalletForeignAssetCreatorCall: PalletForeignAssetCreatorCall;
        PalletForeignAssetCreatorError: PalletForeignAssetCreatorError;
        PalletForeignAssetCreatorEvent: PalletForeignAssetCreatorEvent;
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
        PalletOffencesEvent: PalletOffencesEvent;
        PalletOutboundMessageCommitmentRecorderEvent: PalletOutboundMessageCommitmentRecorderEvent;
        PalletParametersCall: PalletParametersCall;
        PalletParametersEvent: PalletParametersEvent;
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
        PalletPreimageCall: PalletPreimageCall;
        PalletPreimageError: PalletPreimageError;
        PalletPreimageEvent: PalletPreimageEvent;
        PalletPreimageHoldReason: PalletPreimageHoldReason;
        PalletPreimageOldRequestStatus: PalletPreimageOldRequestStatus;
        PalletPreimageRequestStatus: PalletPreimageRequestStatus;
        PalletProxyAnnouncement: PalletProxyAnnouncement;
        PalletProxyCall: PalletProxyCall;
        PalletProxyDepositKind: PalletProxyDepositKind;
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
        PalletReferendaTrackDetails: PalletReferendaTrackDetails;
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
        PalletStreamPaymentCall: PalletStreamPaymentCall;
        PalletStreamPaymentChangeKind: PalletStreamPaymentChangeKind;
        PalletStreamPaymentChangeRequest: PalletStreamPaymentChangeRequest;
        PalletStreamPaymentDepositChange: PalletStreamPaymentDepositChange;
        PalletStreamPaymentError: PalletStreamPaymentError;
        PalletStreamPaymentEvent: PalletStreamPaymentEvent;
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
        PalletUtilityCall: PalletUtilityCall;
        PalletUtilityError: PalletUtilityError;
        PalletUtilityEvent: PalletUtilityEvent;
        PalletWhitelistCall: PalletWhitelistCall;
        PalletWhitelistError: PalletWhitelistError;
        PalletWhitelistEvent: PalletWhitelistEvent;
        PalletXcmAuthorizedAliasesEntry: PalletXcmAuthorizedAliasesEntry;
        PalletXcmCall: PalletXcmCall;
        PalletXcmError: PalletXcmError;
        PalletXcmEvent: PalletXcmEvent;
        PalletXcmHoldReason: PalletXcmHoldReason;
        PalletXcmMaxAuthorizedAliases: PalletXcmMaxAuthorizedAliases;
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
        PolkadotPrimitivesV8CandidateCommitments: PolkadotPrimitivesV8CandidateCommitments;
        PolkadotPrimitivesV8DisputeState: PolkadotPrimitivesV8DisputeState;
        PolkadotPrimitivesV8DisputeStatement: PolkadotPrimitivesV8DisputeStatement;
        PolkadotPrimitivesV8DisputeStatementSet: PolkadotPrimitivesV8DisputeStatementSet;
        PolkadotPrimitivesV8ExecutorParams: PolkadotPrimitivesV8ExecutorParams;
        PolkadotPrimitivesV8ExecutorParamsExecutorParam: PolkadotPrimitivesV8ExecutorParamsExecutorParam;
        PolkadotPrimitivesV8IndexedVecGroupIndex: PolkadotPrimitivesV8IndexedVecGroupIndex;
        PolkadotPrimitivesV8IndexedVecValidatorIndex: PolkadotPrimitivesV8IndexedVecValidatorIndex;
        PolkadotPrimitivesV8InvalidDisputeStatementKind: PolkadotPrimitivesV8InvalidDisputeStatementKind;
        PolkadotPrimitivesV8PvfCheckStatement: PolkadotPrimitivesV8PvfCheckStatement;
        PolkadotPrimitivesV8PvfExecKind: PolkadotPrimitivesV8PvfExecKind;
        PolkadotPrimitivesV8PvfPrepKind: PolkadotPrimitivesV8PvfPrepKind;
        PolkadotPrimitivesV8SchedulerParams: PolkadotPrimitivesV8SchedulerParams;
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
        PolkadotPrimitivesVstagingBackedCandidate: PolkadotPrimitivesVstagingBackedCandidate;
        PolkadotPrimitivesVstagingCandidateDescriptorV2: PolkadotPrimitivesVstagingCandidateDescriptorV2;
        PolkadotPrimitivesVstagingCandidateReceiptV2: PolkadotPrimitivesVstagingCandidateReceiptV2;
        PolkadotPrimitivesVstagingCommittedCandidateReceiptV2: PolkadotPrimitivesVstagingCommittedCandidateReceiptV2;
        PolkadotPrimitivesVstagingInherentData: PolkadotPrimitivesVstagingInherentData;
        PolkadotPrimitivesVstagingScrapedOnChainVotes: PolkadotPrimitivesVstagingScrapedOnChainVotes;
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
        PolkadotRuntimeParachainsSharedAllowedRelayParentsTracker: PolkadotRuntimeParachainsSharedAllowedRelayParentsTracker;
        PolkadotRuntimeParachainsSharedPalletCall: PolkadotRuntimeParachainsSharedPalletCall;
        PolkadotRuntimeParachainsSharedRelayParentInfo: PolkadotRuntimeParachainsSharedRelayParentInfo;
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
        SnowbridgeCoreOperatingModeBasicOperatingMode: SnowbridgeCoreOperatingModeBasicOperatingMode;
        SnowbridgeCorePricingPricingParameters: SnowbridgeCorePricingPricingParameters;
        SnowbridgeCorePricingRewards: SnowbridgeCorePricingRewards;
        SnowbridgeInboundQueuePrimitivesV1ConvertMessageError: SnowbridgeInboundQueuePrimitivesV1ConvertMessageError;
        SnowbridgeMilagroBlsKeysPublicKey: SnowbridgeMilagroBlsKeysPublicKey;
        SnowbridgeOutboundQueuePrimitivesOperatingMode: SnowbridgeOutboundQueuePrimitivesOperatingMode;
        SnowbridgeOutboundQueuePrimitivesSendError: SnowbridgeOutboundQueuePrimitivesSendError;
        SnowbridgeOutboundQueuePrimitivesV1MessageInitializer: SnowbridgeOutboundQueuePrimitivesV1MessageInitializer;
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
        SnowbridgeVerificationPrimitivesEventProof: SnowbridgeVerificationPrimitivesEventProof;
        SnowbridgeVerificationPrimitivesLog: SnowbridgeVerificationPrimitivesLog;
        SnowbridgeVerificationPrimitivesProof: SnowbridgeVerificationPrimitivesProof;
        SnowbridgeVerificationPrimitivesVerificationError: SnowbridgeVerificationPrimitivesVerificationError;
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
        SpMmrPrimitivesAncestryProof: SpMmrPrimitivesAncestryProof;
        SpRuntimeBlakeTwo256: SpRuntimeBlakeTwo256;
        SpRuntimeDigest: SpRuntimeDigest;
        SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
        SpRuntimeDispatchError: SpRuntimeDispatchError;
        SpRuntimeDispatchErrorWithPostInfo: SpRuntimeDispatchErrorWithPostInfo;
        SpRuntimeHeader: SpRuntimeHeader;
        SpRuntimeModuleError: SpRuntimeModuleError;
        SpRuntimeMultiSignature: SpRuntimeMultiSignature;
        SpRuntimeProvingTrieTrieError: SpRuntimeProvingTrieTrieError;
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
        TpBridgeChannelInfo: TpBridgeChannelInfo;
        TpBridgeCommand: TpBridgeCommand;
        TpBridgeSlashData: TpBridgeSlashData;
        TpDataPreserversCommonAssignerExtra: TpDataPreserversCommonAssignerExtra;
        TpDataPreserversCommonAssignmentWitness: TpDataPreserversCommonAssignmentWitness;
        TpDataPreserversCommonProviderRequest: TpDataPreserversCommonProviderRequest;
        TpStreamPaymentCommonAssetId: TpStreamPaymentCommonAssetId;
        TpStreamPaymentCommonTimeUnit: TpStreamPaymentCommonTimeUnit;
        TpTraitsActiveEraInfo: TpTraitsActiveEraInfo;
        TpTraitsContainerChainBlockInfo: TpTraitsContainerChainBlockInfo;
        TpTraitsFullRotationMode: TpTraitsFullRotationMode;
        TpTraitsFullRotationModes: TpTraitsFullRotationModes;
        TpTraitsParathreadParams: TpTraitsParathreadParams;
        TpTraitsSlotFrequency: TpTraitsSlotFrequency;
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
