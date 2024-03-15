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
    CumulusPrimitivesParachainInherentParachainInherentData,
    DpCollatorAssignmentAssignedCollatorsAccountId32,
    DpCollatorAssignmentAssignedCollatorsPublic,
    FlashboxRuntimeOriginCaller,
    FlashboxRuntimeProxyType,
    FlashboxRuntimeRuntime,
    FlashboxRuntimeRuntimeFreezeReason,
    FlashboxRuntimeRuntimeHoldReason,
    FlashboxRuntimeSessionKeys,
    FlashboxRuntimeStreamPaymentAssetId,
    FlashboxRuntimeTimeUnit,
    FrameSupportDispatchDispatchClass,
    FrameSupportDispatchDispatchInfo,
    FrameSupportDispatchPays,
    FrameSupportDispatchPerDispatchClassU32,
    FrameSupportDispatchPerDispatchClassWeight,
    FrameSupportDispatchPerDispatchClassWeightsPerClass,
    FrameSupportDispatchRawOrigin,
    FrameSupportPalletId,
    FrameSupportTokensMiscBalanceStatus,
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
    PalletBalancesIdAmountRuntimeFreezeReason,
    PalletBalancesIdAmountRuntimeHoldReason,
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
    PalletMaintenanceModeCall,
    PalletMaintenanceModeError,
    PalletMaintenanceModeEvent,
    PalletMigrationsError,
    PalletMigrationsEvent,
    PalletMultisigCall,
    PalletMultisigError,
    PalletMultisigEvent,
    PalletMultisigMultisig,
    PalletMultisigTimepoint,
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
    PolkadotCorePrimitivesInboundDownwardMessage,
    PolkadotCorePrimitivesInboundHrmpMessage,
    PolkadotCorePrimitivesOutboundHrmpMessage,
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
    StagingParachainInfoCall,
    TpAuthorNotingInherentOwnParachainInherentData,
    TpContainerChainGenesisDataContainerChainGenesisData,
    TpContainerChainGenesisDataContainerChainGenesisDataItem,
    TpContainerChainGenesisDataProperties,
    TpContainerChainGenesisDataTokenMetadata,
    TpTraitsParathreadParams,
    TpTraitsSlotFrequency,
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
        CumulusPrimitivesParachainInherentParachainInherentData: CumulusPrimitivesParachainInherentParachainInherentData;
        DpCollatorAssignmentAssignedCollatorsAccountId32: DpCollatorAssignmentAssignedCollatorsAccountId32;
        DpCollatorAssignmentAssignedCollatorsPublic: DpCollatorAssignmentAssignedCollatorsPublic;
        FlashboxRuntimeOriginCaller: FlashboxRuntimeOriginCaller;
        FlashboxRuntimeProxyType: FlashboxRuntimeProxyType;
        FlashboxRuntimeRuntime: FlashboxRuntimeRuntime;
        FlashboxRuntimeRuntimeFreezeReason: FlashboxRuntimeRuntimeFreezeReason;
        FlashboxRuntimeRuntimeHoldReason: FlashboxRuntimeRuntimeHoldReason;
        FlashboxRuntimeSessionKeys: FlashboxRuntimeSessionKeys;
        FlashboxRuntimeStreamPaymentAssetId: FlashboxRuntimeStreamPaymentAssetId;
        FlashboxRuntimeTimeUnit: FlashboxRuntimeTimeUnit;
        FrameSupportDispatchDispatchClass: FrameSupportDispatchDispatchClass;
        FrameSupportDispatchDispatchInfo: FrameSupportDispatchDispatchInfo;
        FrameSupportDispatchPays: FrameSupportDispatchPays;
        FrameSupportDispatchPerDispatchClassU32: FrameSupportDispatchPerDispatchClassU32;
        FrameSupportDispatchPerDispatchClassWeight: FrameSupportDispatchPerDispatchClassWeight;
        FrameSupportDispatchPerDispatchClassWeightsPerClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
        FrameSupportDispatchRawOrigin: FrameSupportDispatchRawOrigin;
        FrameSupportPalletId: FrameSupportPalletId;
        FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
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
        PalletBalancesIdAmountRuntimeFreezeReason: PalletBalancesIdAmountRuntimeFreezeReason;
        PalletBalancesIdAmountRuntimeHoldReason: PalletBalancesIdAmountRuntimeHoldReason;
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
        PalletMaintenanceModeCall: PalletMaintenanceModeCall;
        PalletMaintenanceModeError: PalletMaintenanceModeError;
        PalletMaintenanceModeEvent: PalletMaintenanceModeEvent;
        PalletMigrationsError: PalletMigrationsError;
        PalletMigrationsEvent: PalletMigrationsEvent;
        PalletMultisigCall: PalletMultisigCall;
        PalletMultisigError: PalletMultisigError;
        PalletMultisigEvent: PalletMultisigEvent;
        PalletMultisigMultisig: PalletMultisigMultisig;
        PalletMultisigTimepoint: PalletMultisigTimepoint;
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
        PolkadotCorePrimitivesInboundDownwardMessage: PolkadotCorePrimitivesInboundDownwardMessage;
        PolkadotCorePrimitivesInboundHrmpMessage: PolkadotCorePrimitivesInboundHrmpMessage;
        PolkadotCorePrimitivesOutboundHrmpMessage: PolkadotCorePrimitivesOutboundHrmpMessage;
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
        StagingParachainInfoCall: StagingParachainInfoCall;
        TpAuthorNotingInherentOwnParachainInherentData: TpAuthorNotingInherentOwnParachainInherentData;
        TpContainerChainGenesisDataContainerChainGenesisData: TpContainerChainGenesisDataContainerChainGenesisData;
        TpContainerChainGenesisDataContainerChainGenesisDataItem: TpContainerChainGenesisDataContainerChainGenesisDataItem;
        TpContainerChainGenesisDataProperties: TpContainerChainGenesisDataProperties;
        TpContainerChainGenesisDataTokenMetadata: TpContainerChainGenesisDataTokenMetadata;
        TpTraitsParathreadParams: TpTraitsParathreadParams;
        TpTraitsSlotFrequency: TpTraitsSlotFrequency;
    } // InterfaceTypes
} // declare module
