// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/types/lookup";

import type { Data } from "@polkadot/types";
import type {
    BTreeMap,
    BTreeSet,
    BitVec,
    Bytes,
    Compact,
    Enum,
    Null,
    Option,
    Result,
    Struct,
    Text,
    U256,
    U8aFixed,
    Vec,
    bool,
    i32,
    i64,
    u128,
    u16,
    u32,
    u64,
    u8,
} from "@polkadot/types-codec";
import type { ITuple } from "@polkadot/types-codec/types";
import type { Vote } from "@polkadot/types/interfaces/elections";
import type { AccountId32, Call, H160, H256, MultiAddress, Perbill } from "@polkadot/types/interfaces/runtime";
import type { Event } from "@polkadot/types/interfaces/system";

declare module "@polkadot/types/lookup" {
    /** @name FrameSystemAccountInfo (3) */
    interface FrameSystemAccountInfo extends Struct {
        readonly nonce: u32;
        readonly consumers: u32;
        readonly providers: u32;
        readonly sufficients: u32;
        readonly data: PalletBalancesAccountData;
    }

    /** @name PalletBalancesAccountData (5) */
    interface PalletBalancesAccountData extends Struct {
        readonly free: u128;
        readonly reserved: u128;
        readonly frozen: u128;
        readonly flags: u128;
    }

    /** @name FrameSupportDispatchPerDispatchClassWeight (9) */
    interface FrameSupportDispatchPerDispatchClassWeight extends Struct {
        readonly normal: SpWeightsWeightV2Weight;
        readonly operational: SpWeightsWeightV2Weight;
        readonly mandatory: SpWeightsWeightV2Weight;
    }

    /** @name SpWeightsWeightV2Weight (10) */
    interface SpWeightsWeightV2Weight extends Struct {
        readonly refTime: Compact<u64>;
        readonly proofSize: Compact<u64>;
    }

    /** @name SpRuntimeDigest (15) */
    interface SpRuntimeDigest extends Struct {
        readonly logs: Vec<SpRuntimeDigestDigestItem>;
    }

    /** @name SpRuntimeDigestDigestItem (17) */
    interface SpRuntimeDigestDigestItem extends Enum {
        readonly isOther: boolean;
        readonly asOther: Bytes;
        readonly isConsensus: boolean;
        readonly asConsensus: ITuple<[U8aFixed, Bytes]>;
        readonly isSeal: boolean;
        readonly asSeal: ITuple<[U8aFixed, Bytes]>;
        readonly isPreRuntime: boolean;
        readonly asPreRuntime: ITuple<[U8aFixed, Bytes]>;
        readonly isRuntimeEnvironmentUpdated: boolean;
        readonly type: "Other" | "Consensus" | "Seal" | "PreRuntime" | "RuntimeEnvironmentUpdated";
    }

    /** @name FrameSystemEventRecord (20) */
    interface FrameSystemEventRecord extends Struct {
        readonly phase: FrameSystemPhase;
        readonly event: Event;
        readonly topics: Vec<H256>;
    }

    /** @name FrameSystemEvent (22) */
    interface FrameSystemEvent extends Enum {
        readonly isExtrinsicSuccess: boolean;
        readonly asExtrinsicSuccess: {
            readonly dispatchInfo: FrameSystemDispatchEventInfo;
        } & Struct;
        readonly isExtrinsicFailed: boolean;
        readonly asExtrinsicFailed: {
            readonly dispatchError: SpRuntimeDispatchError;
            readonly dispatchInfo: FrameSystemDispatchEventInfo;
        } & Struct;
        readonly isCodeUpdated: boolean;
        readonly isNewAccount: boolean;
        readonly asNewAccount: {
            readonly account: AccountId32;
        } & Struct;
        readonly isKilledAccount: boolean;
        readonly asKilledAccount: {
            readonly account: AccountId32;
        } & Struct;
        readonly isRemarked: boolean;
        readonly asRemarked: {
            readonly sender: AccountId32;
            readonly hash_: H256;
        } & Struct;
        readonly isUpgradeAuthorized: boolean;
        readonly asUpgradeAuthorized: {
            readonly codeHash: H256;
            readonly checkVersion: bool;
        } & Struct;
        readonly isRejectedInvalidAuthorizedUpgrade: boolean;
        readonly asRejectedInvalidAuthorizedUpgrade: {
            readonly codeHash: H256;
            readonly error: SpRuntimeDispatchError;
        } & Struct;
        readonly type:
            | "ExtrinsicSuccess"
            | "ExtrinsicFailed"
            | "CodeUpdated"
            | "NewAccount"
            | "KilledAccount"
            | "Remarked"
            | "UpgradeAuthorized"
            | "RejectedInvalidAuthorizedUpgrade";
    }

    /** @name FrameSystemDispatchEventInfo (23) */
    interface FrameSystemDispatchEventInfo extends Struct {
        readonly weight: SpWeightsWeightV2Weight;
        readonly class: FrameSupportDispatchDispatchClass;
        readonly paysFee: FrameSupportDispatchPays;
    }

    /** @name FrameSupportDispatchDispatchClass (24) */
    interface FrameSupportDispatchDispatchClass extends Enum {
        readonly isNormal: boolean;
        readonly isOperational: boolean;
        readonly isMandatory: boolean;
        readonly type: "Normal" | "Operational" | "Mandatory";
    }

    /** @name FrameSupportDispatchPays (25) */
    interface FrameSupportDispatchPays extends Enum {
        readonly isYes: boolean;
        readonly isNo: boolean;
        readonly type: "Yes" | "No";
    }

    /** @name SpRuntimeDispatchError (26) */
    interface SpRuntimeDispatchError extends Enum {
        readonly isOther: boolean;
        readonly isCannotLookup: boolean;
        readonly isBadOrigin: boolean;
        readonly isModule: boolean;
        readonly asModule: SpRuntimeModuleError;
        readonly isConsumerRemaining: boolean;
        readonly isNoProviders: boolean;
        readonly isTooManyConsumers: boolean;
        readonly isToken: boolean;
        readonly asToken: SpRuntimeTokenError;
        readonly isArithmetic: boolean;
        readonly asArithmetic: SpArithmeticArithmeticError;
        readonly isTransactional: boolean;
        readonly asTransactional: SpRuntimeTransactionalError;
        readonly isExhausted: boolean;
        readonly isCorruption: boolean;
        readonly isUnavailable: boolean;
        readonly isRootNotAllowed: boolean;
        readonly isTrie: boolean;
        readonly asTrie: SpRuntimeProvingTrieTrieError;
        readonly type:
            | "Other"
            | "CannotLookup"
            | "BadOrigin"
            | "Module"
            | "ConsumerRemaining"
            | "NoProviders"
            | "TooManyConsumers"
            | "Token"
            | "Arithmetic"
            | "Transactional"
            | "Exhausted"
            | "Corruption"
            | "Unavailable"
            | "RootNotAllowed"
            | "Trie";
    }

    /** @name SpRuntimeModuleError (27) */
    interface SpRuntimeModuleError extends Struct {
        readonly index: u8;
        readonly error: U8aFixed;
    }

    /** @name SpRuntimeTokenError (28) */
    interface SpRuntimeTokenError extends Enum {
        readonly isFundsUnavailable: boolean;
        readonly isOnlyProvider: boolean;
        readonly isBelowMinimum: boolean;
        readonly isCannotCreate: boolean;
        readonly isUnknownAsset: boolean;
        readonly isFrozen: boolean;
        readonly isUnsupported: boolean;
        readonly isCannotCreateHold: boolean;
        readonly isNotExpendable: boolean;
        readonly isBlocked: boolean;
        readonly type:
            | "FundsUnavailable"
            | "OnlyProvider"
            | "BelowMinimum"
            | "CannotCreate"
            | "UnknownAsset"
            | "Frozen"
            | "Unsupported"
            | "CannotCreateHold"
            | "NotExpendable"
            | "Blocked";
    }

    /** @name SpArithmeticArithmeticError (29) */
    interface SpArithmeticArithmeticError extends Enum {
        readonly isUnderflow: boolean;
        readonly isOverflow: boolean;
        readonly isDivisionByZero: boolean;
        readonly type: "Underflow" | "Overflow" | "DivisionByZero";
    }

    /** @name SpRuntimeTransactionalError (30) */
    interface SpRuntimeTransactionalError extends Enum {
        readonly isLimitReached: boolean;
        readonly isNoLayer: boolean;
        readonly type: "LimitReached" | "NoLayer";
    }

    /** @name SpRuntimeProvingTrieTrieError (31) */
    interface SpRuntimeProvingTrieTrieError extends Enum {
        readonly isInvalidStateRoot: boolean;
        readonly isIncompleteDatabase: boolean;
        readonly isValueAtIncompleteKey: boolean;
        readonly isDecoderError: boolean;
        readonly isInvalidHash: boolean;
        readonly isDuplicateKey: boolean;
        readonly isExtraneousNode: boolean;
        readonly isExtraneousValue: boolean;
        readonly isExtraneousHashReference: boolean;
        readonly isInvalidChildReference: boolean;
        readonly isValueMismatch: boolean;
        readonly isIncompleteProof: boolean;
        readonly isRootMismatch: boolean;
        readonly isDecodeError: boolean;
        readonly type:
            | "InvalidStateRoot"
            | "IncompleteDatabase"
            | "ValueAtIncompleteKey"
            | "DecoderError"
            | "InvalidHash"
            | "DuplicateKey"
            | "ExtraneousNode"
            | "ExtraneousValue"
            | "ExtraneousHashReference"
            | "InvalidChildReference"
            | "ValueMismatch"
            | "IncompleteProof"
            | "RootMismatch"
            | "DecodeError";
    }

    /** @name PalletBalancesEvent (32) */
    interface PalletBalancesEvent extends Enum {
        readonly isEndowed: boolean;
        readonly asEndowed: {
            readonly account: AccountId32;
            readonly freeBalance: u128;
        } & Struct;
        readonly isDustLost: boolean;
        readonly asDustLost: {
            readonly account: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isTransfer: boolean;
        readonly asTransfer: {
            readonly from: AccountId32;
            readonly to: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isBalanceSet: boolean;
        readonly asBalanceSet: {
            readonly who: AccountId32;
            readonly free: u128;
        } & Struct;
        readonly isReserved: boolean;
        readonly asReserved: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isUnreserved: boolean;
        readonly asUnreserved: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isReserveRepatriated: boolean;
        readonly asReserveRepatriated: {
            readonly from: AccountId32;
            readonly to: AccountId32;
            readonly amount: u128;
            readonly destinationStatus: FrameSupportTokensMiscBalanceStatus;
        } & Struct;
        readonly isDeposit: boolean;
        readonly asDeposit: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isWithdraw: boolean;
        readonly asWithdraw: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isSlashed: boolean;
        readonly asSlashed: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isMinted: boolean;
        readonly asMinted: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isBurned: boolean;
        readonly asBurned: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isSuspended: boolean;
        readonly asSuspended: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isRestored: boolean;
        readonly asRestored: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isUpgraded: boolean;
        readonly asUpgraded: {
            readonly who: AccountId32;
        } & Struct;
        readonly isIssued: boolean;
        readonly asIssued: {
            readonly amount: u128;
        } & Struct;
        readonly isRescinded: boolean;
        readonly asRescinded: {
            readonly amount: u128;
        } & Struct;
        readonly isLocked: boolean;
        readonly asLocked: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isUnlocked: boolean;
        readonly asUnlocked: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isFrozen: boolean;
        readonly asFrozen: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isThawed: boolean;
        readonly asThawed: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isTotalIssuanceForced: boolean;
        readonly asTotalIssuanceForced: {
            readonly old: u128;
            readonly new_: u128;
        } & Struct;
        readonly type:
            | "Endowed"
            | "DustLost"
            | "Transfer"
            | "BalanceSet"
            | "Reserved"
            | "Unreserved"
            | "ReserveRepatriated"
            | "Deposit"
            | "Withdraw"
            | "Slashed"
            | "Minted"
            | "Burned"
            | "Suspended"
            | "Restored"
            | "Upgraded"
            | "Issued"
            | "Rescinded"
            | "Locked"
            | "Unlocked"
            | "Frozen"
            | "Thawed"
            | "TotalIssuanceForced";
    }

    /** @name FrameSupportTokensMiscBalanceStatus (33) */
    interface FrameSupportTokensMiscBalanceStatus extends Enum {
        readonly isFree: boolean;
        readonly isReserved: boolean;
        readonly type: "Free" | "Reserved";
    }

    /** @name PalletParametersEvent (34) */
    interface PalletParametersEvent extends Enum {
        readonly isUpdated: boolean;
        readonly asUpdated: {
            readonly key: DancelightRuntimeRuntimeParametersKey;
            readonly oldValue: Option<DancelightRuntimeRuntimeParametersValue>;
            readonly newValue: Option<DancelightRuntimeRuntimeParametersValue>;
        } & Struct;
        readonly type: "Updated";
    }

    /** @name DancelightRuntimeRuntimeParametersKey (35) */
    interface DancelightRuntimeRuntimeParametersKey extends Enum {
        readonly isPreimage: boolean;
        readonly asPreimage: DancelightRuntimeDynamicParamsPreimageParametersKey;
        readonly type: "Preimage";
    }

    /** @name DancelightRuntimeDynamicParamsPreimageParametersKey (36) */
    interface DancelightRuntimeDynamicParamsPreimageParametersKey extends Enum {
        readonly isBaseDeposit: boolean;
        readonly isByteDeposit: boolean;
        readonly type: "BaseDeposit" | "ByteDeposit";
    }

    /** @name DancelightRuntimeDynamicParamsPreimageBaseDeposit (37) */
    type DancelightRuntimeDynamicParamsPreimageBaseDeposit = Null;

    /** @name DancelightRuntimeDynamicParamsPreimageByteDeposit (38) */
    type DancelightRuntimeDynamicParamsPreimageByteDeposit = Null;

    /** @name DancelightRuntimeRuntimeParametersValue (40) */
    interface DancelightRuntimeRuntimeParametersValue extends Enum {
        readonly isPreimage: boolean;
        readonly asPreimage: DancelightRuntimeDynamicParamsPreimageParametersValue;
        readonly type: "Preimage";
    }

    /** @name DancelightRuntimeDynamicParamsPreimageParametersValue (41) */
    interface DancelightRuntimeDynamicParamsPreimageParametersValue extends Enum {
        readonly isBaseDeposit: boolean;
        readonly asBaseDeposit: u128;
        readonly isByteDeposit: boolean;
        readonly asByteDeposit: u128;
        readonly type: "BaseDeposit" | "ByteDeposit";
    }

    /** @name PalletTransactionPaymentEvent (42) */
    interface PalletTransactionPaymentEvent extends Enum {
        readonly isTransactionFeePaid: boolean;
        readonly asTransactionFeePaid: {
            readonly who: AccountId32;
            readonly actualFee: u128;
            readonly tip: u128;
        } & Struct;
        readonly type: "TransactionFeePaid";
    }

    /** @name PalletOffencesEvent (43) */
    interface PalletOffencesEvent extends Enum {
        readonly isOffence: boolean;
        readonly asOffence: {
            readonly kind: U8aFixed;
            readonly timeslot: Bytes;
        } & Struct;
        readonly type: "Offence";
    }

    /** @name PalletSessionHistoricalPalletEvent (45) */
    interface PalletSessionHistoricalPalletEvent extends Enum {
        readonly isRootStored: boolean;
        readonly asRootStored: {
            readonly index: u32;
        } & Struct;
        readonly isRootsPruned: boolean;
        readonly asRootsPruned: {
            readonly upTo: u32;
        } & Struct;
        readonly type: "RootStored" | "RootsPruned";
    }

    /** @name PalletRegistrarEvent (46) */
    interface PalletRegistrarEvent extends Enum {
        readonly isParaIdRegistered: boolean;
        readonly asParaIdRegistered: {
            readonly paraId: u32;
        } & Struct;
        readonly isParaIdDeregistered: boolean;
        readonly asParaIdDeregistered: {
            readonly paraId: u32;
        } & Struct;
        readonly isParaIdValidForCollating: boolean;
        readonly asParaIdValidForCollating: {
            readonly paraId: u32;
        } & Struct;
        readonly isParaIdPaused: boolean;
        readonly asParaIdPaused: {
            readonly paraId: u32;
        } & Struct;
        readonly isParaIdUnpaused: boolean;
        readonly asParaIdUnpaused: {
            readonly paraId: u32;
        } & Struct;
        readonly isParathreadParamsChanged: boolean;
        readonly asParathreadParamsChanged: {
            readonly paraId: u32;
        } & Struct;
        readonly isParaManagerChanged: boolean;
        readonly asParaManagerChanged: {
            readonly paraId: u32;
            readonly managerAddress: AccountId32;
        } & Struct;
        readonly type:
            | "ParaIdRegistered"
            | "ParaIdDeregistered"
            | "ParaIdValidForCollating"
            | "ParaIdPaused"
            | "ParaIdUnpaused"
            | "ParathreadParamsChanged"
            | "ParaManagerChanged";
    }

    /** @name PalletInvulnerablesEvent (48) */
    interface PalletInvulnerablesEvent extends Enum {
        readonly isInvulnerableAdded: boolean;
        readonly asInvulnerableAdded: {
            readonly accountId: AccountId32;
        } & Struct;
        readonly isInvulnerableRemoved: boolean;
        readonly asInvulnerableRemoved: {
            readonly accountId: AccountId32;
        } & Struct;
        readonly type: "InvulnerableAdded" | "InvulnerableRemoved";
    }

    /** @name PalletCollatorAssignmentEvent (49) */
    interface PalletCollatorAssignmentEvent extends Enum {
        readonly isNewPendingAssignment: boolean;
        readonly asNewPendingAssignment: {
            readonly randomSeed: U8aFixed;
            readonly fullRotation: bool;
            readonly targetSession: u32;
            readonly fullRotationMode: TpTraitsFullRotationModes;
        } & Struct;
        readonly type: "NewPendingAssignment";
    }

    /** @name TpTraitsFullRotationModes (50) */
    interface TpTraitsFullRotationModes extends Struct {
        readonly orchestrator: TpTraitsFullRotationMode;
        readonly parachain: TpTraitsFullRotationMode;
        readonly parathread: TpTraitsFullRotationMode;
    }

    /** @name TpTraitsFullRotationMode (51) */
    interface TpTraitsFullRotationMode extends Enum {
        readonly isRotateAll: boolean;
        readonly isKeepAll: boolean;
        readonly isKeepCollators: boolean;
        readonly asKeepCollators: {
            readonly keep: u32;
        } & Struct;
        readonly isKeepPerbill: boolean;
        readonly asKeepPerbill: {
            readonly percentage: Perbill;
        } & Struct;
        readonly type: "RotateAll" | "KeepAll" | "KeepCollators" | "KeepPerbill";
    }

    /** @name PalletAuthorNotingEvent (53) */
    interface PalletAuthorNotingEvent extends Enum {
        readonly isLatestAuthorChanged: boolean;
        readonly asLatestAuthorChanged: {
            readonly paraId: u32;
            readonly blockNumber: u32;
            readonly newAuthor: AccountId32;
            readonly latestSlotNumber: u64;
        } & Struct;
        readonly isRemovedAuthorData: boolean;
        readonly asRemovedAuthorData: {
            readonly paraId: u32;
        } & Struct;
        readonly type: "LatestAuthorChanged" | "RemovedAuthorData";
    }

    /** @name PalletServicesPaymentEvent (55) */
    interface PalletServicesPaymentEvent extends Enum {
        readonly isCreditsPurchased: boolean;
        readonly asCreditsPurchased: {
            readonly paraId: u32;
            readonly payer: AccountId32;
            readonly credit: u128;
        } & Struct;
        readonly isBlockProductionCreditBurned: boolean;
        readonly asBlockProductionCreditBurned: {
            readonly paraId: u32;
            readonly creditsRemaining: u32;
        } & Struct;
        readonly isCollatorAssignmentCreditBurned: boolean;
        readonly asCollatorAssignmentCreditBurned: {
            readonly paraId: u32;
            readonly creditsRemaining: u32;
        } & Struct;
        readonly isCollatorAssignmentTipCollected: boolean;
        readonly asCollatorAssignmentTipCollected: {
            readonly paraId: u32;
            readonly payer: AccountId32;
            readonly tip: u128;
        } & Struct;
        readonly isBlockProductionCreditsSet: boolean;
        readonly asBlockProductionCreditsSet: {
            readonly paraId: u32;
            readonly credits: u32;
        } & Struct;
        readonly isRefundAddressUpdated: boolean;
        readonly asRefundAddressUpdated: {
            readonly paraId: u32;
            readonly refundAddress: Option<AccountId32>;
        } & Struct;
        readonly isMaxCorePriceUpdated: boolean;
        readonly asMaxCorePriceUpdated: {
            readonly paraId: u32;
            readonly maxCorePrice: u128;
        } & Struct;
        readonly isCollatorAssignmentCreditsSet: boolean;
        readonly asCollatorAssignmentCreditsSet: {
            readonly paraId: u32;
            readonly credits: u32;
        } & Struct;
        readonly type:
            | "CreditsPurchased"
            | "BlockProductionCreditBurned"
            | "CollatorAssignmentCreditBurned"
            | "CollatorAssignmentTipCollected"
            | "BlockProductionCreditsSet"
            | "RefundAddressUpdated"
            | "MaxCorePriceUpdated"
            | "CollatorAssignmentCreditsSet";
    }

    /** @name PalletDataPreserversEvent (57) */
    interface PalletDataPreserversEvent extends Enum {
        readonly isBootNodesChanged: boolean;
        readonly asBootNodesChanged: {
            readonly paraId: u32;
        } & Struct;
        readonly isProfileCreated: boolean;
        readonly asProfileCreated: {
            readonly account: AccountId32;
            readonly profileId: u64;
            readonly deposit: u128;
        } & Struct;
        readonly isProfileUpdated: boolean;
        readonly asProfileUpdated: {
            readonly profileId: u64;
            readonly oldDeposit: u128;
            readonly newDeposit: u128;
        } & Struct;
        readonly isProfileDeleted: boolean;
        readonly asProfileDeleted: {
            readonly profileId: u64;
            readonly releasedDeposit: u128;
        } & Struct;
        readonly isAssignmentStarted: boolean;
        readonly asAssignmentStarted: {
            readonly profileId: u64;
            readonly paraId: u32;
        } & Struct;
        readonly isAssignmentStopped: boolean;
        readonly asAssignmentStopped: {
            readonly profileId: u64;
            readonly paraId: u32;
        } & Struct;
        readonly type:
            | "BootNodesChanged"
            | "ProfileCreated"
            | "ProfileUpdated"
            | "ProfileDeleted"
            | "AssignmentStarted"
            | "AssignmentStopped";
    }

    /** @name PalletExternalValidatorsEvent (58) */
    interface PalletExternalValidatorsEvent extends Enum {
        readonly isWhitelistedValidatorAdded: boolean;
        readonly asWhitelistedValidatorAdded: {
            readonly accountId: AccountId32;
        } & Struct;
        readonly isWhitelistedValidatorRemoved: boolean;
        readonly asWhitelistedValidatorRemoved: {
            readonly accountId: AccountId32;
        } & Struct;
        readonly isNewEra: boolean;
        readonly asNewEra: {
            readonly era: u32;
        } & Struct;
        readonly isForceEra: boolean;
        readonly asForceEra: {
            readonly mode: PalletExternalValidatorsForcing;
        } & Struct;
        readonly isExternalValidatorsSet: boolean;
        readonly asExternalValidatorsSet: {
            readonly validators: Vec<AccountId32>;
            readonly externalIndex: u64;
        } & Struct;
        readonly type:
            | "WhitelistedValidatorAdded"
            | "WhitelistedValidatorRemoved"
            | "NewEra"
            | "ForceEra"
            | "ExternalValidatorsSet";
    }

    /** @name PalletExternalValidatorsForcing (59) */
    interface PalletExternalValidatorsForcing extends Enum {
        readonly isNotForcing: boolean;
        readonly isForceNew: boolean;
        readonly isForceNone: boolean;
        readonly isForceAlways: boolean;
        readonly type: "NotForcing" | "ForceNew" | "ForceNone" | "ForceAlways";
    }

    /** @name PalletExternalValidatorSlashesEvent (61) */
    interface PalletExternalValidatorSlashesEvent extends Enum {
        readonly isSlashReported: boolean;
        readonly asSlashReported: {
            readonly validator: AccountId32;
            readonly fraction: Perbill;
            readonly slashEra: u32;
        } & Struct;
        readonly isSlashesMessageSent: boolean;
        readonly asSlashesMessageSent: {
            readonly messageId: H256;
            readonly slashesCommand: TpBridgeCommand;
        } & Struct;
        readonly type: "SlashReported" | "SlashesMessageSent";
    }

    /** @name TpBridgeCommand (62) */
    interface TpBridgeCommand extends Enum {
        readonly isTest: boolean;
        readonly asTest: Bytes;
        readonly isReportRewards: boolean;
        readonly asReportRewards: {
            readonly externalIdx: u64;
            readonly eraIndex: u32;
            readonly totalPoints: u128;
            readonly tokensInflated: u128;
            readonly rewardsMerkleRoot: H256;
            readonly tokenId: H256;
        } & Struct;
        readonly isReportSlashes: boolean;
        readonly asReportSlashes: {
            readonly eraIndex: u32;
            readonly slashes: Vec<TpBridgeSlashData>;
        } & Struct;
        readonly type: "Test" | "ReportRewards" | "ReportSlashes";
    }

    /** @name TpBridgeSlashData (64) */
    interface TpBridgeSlashData extends Struct {
        readonly encodedValidatorId: Bytes;
        readonly slashFraction: u32;
        readonly externalIdx: u64;
    }

    /** @name PalletExternalValidatorsRewardsEvent (65) */
    interface PalletExternalValidatorsRewardsEvent extends Enum {
        readonly isRewardsMessageSent: boolean;
        readonly asRewardsMessageSent: {
            readonly messageId: H256;
            readonly rewardsCommand: TpBridgeCommand;
        } & Struct;
        readonly type: "RewardsMessageSent";
    }

    /** @name SnowbridgePalletOutboundQueueEvent (66) */
    interface SnowbridgePalletOutboundQueueEvent extends Enum {
        readonly isMessageQueued: boolean;
        readonly asMessageQueued: {
            readonly id: H256;
        } & Struct;
        readonly isMessageAccepted: boolean;
        readonly asMessageAccepted: {
            readonly id: H256;
            readonly nonce: u64;
        } & Struct;
        readonly isMessagesCommitted: boolean;
        readonly asMessagesCommitted: {
            readonly root: H256;
            readonly count: u64;
        } & Struct;
        readonly isOperatingModeChanged: boolean;
        readonly asOperatingModeChanged: {
            readonly mode: SnowbridgeCoreOperatingModeBasicOperatingMode;
        } & Struct;
        readonly type: "MessageQueued" | "MessageAccepted" | "MessagesCommitted" | "OperatingModeChanged";
    }

    /** @name SnowbridgeCoreOperatingModeBasicOperatingMode (67) */
    interface SnowbridgeCoreOperatingModeBasicOperatingMode extends Enum {
        readonly isNormal: boolean;
        readonly isHalted: boolean;
        readonly type: "Normal" | "Halted";
    }

    /** @name SnowbridgePalletInboundQueueEvent (68) */
    interface SnowbridgePalletInboundQueueEvent extends Enum {
        readonly isMessageReceived: boolean;
        readonly asMessageReceived: {
            readonly channelId: SnowbridgeCoreChannelId;
            readonly nonce: u64;
            readonly messageId: U8aFixed;
            readonly feeBurned: u128;
        } & Struct;
        readonly isOperatingModeChanged: boolean;
        readonly asOperatingModeChanged: {
            readonly mode: SnowbridgeCoreOperatingModeBasicOperatingMode;
        } & Struct;
        readonly type: "MessageReceived" | "OperatingModeChanged";
    }

    /** @name SnowbridgeCoreChannelId (69) */
    interface SnowbridgeCoreChannelId extends U8aFixed {}

    /** @name SnowbridgePalletSystemEvent (70) */
    interface SnowbridgePalletSystemEvent extends Enum {
        readonly isUpgrade: boolean;
        readonly asUpgrade: {
            readonly implAddress: H160;
            readonly implCodeHash: H256;
            readonly initializerParamsHash: Option<H256>;
        } & Struct;
        readonly isCreateAgent: boolean;
        readonly asCreateAgent: {
            readonly location: StagingXcmV5Location;
            readonly agentId: H256;
        } & Struct;
        readonly isCreateChannel: boolean;
        readonly asCreateChannel: {
            readonly channelId: SnowbridgeCoreChannelId;
            readonly agentId: H256;
        } & Struct;
        readonly isUpdateChannel: boolean;
        readonly asUpdateChannel: {
            readonly channelId: SnowbridgeCoreChannelId;
            readonly mode: SnowbridgeOutboundQueuePrimitivesOperatingMode;
        } & Struct;
        readonly isSetOperatingMode: boolean;
        readonly asSetOperatingMode: {
            readonly mode: SnowbridgeOutboundQueuePrimitivesOperatingMode;
        } & Struct;
        readonly isTransferNativeFromAgent: boolean;
        readonly asTransferNativeFromAgent: {
            readonly agentId: H256;
            readonly recipient: H160;
            readonly amount: u128;
        } & Struct;
        readonly isSetTokenTransferFees: boolean;
        readonly asSetTokenTransferFees: {
            readonly createAssetXcm: u128;
            readonly transferAssetXcm: u128;
            readonly registerToken: U256;
        } & Struct;
        readonly isPricingParametersChanged: boolean;
        readonly asPricingParametersChanged: {
            readonly params: SnowbridgeCorePricingPricingParameters;
        } & Struct;
        readonly isRegisterToken: boolean;
        readonly asRegisterToken: {
            readonly location: XcmVersionedLocation;
            readonly foreignTokenId: H256;
        } & Struct;
        readonly type:
            | "Upgrade"
            | "CreateAgent"
            | "CreateChannel"
            | "UpdateChannel"
            | "SetOperatingMode"
            | "TransferNativeFromAgent"
            | "SetTokenTransferFees"
            | "PricingParametersChanged"
            | "RegisterToken";
    }

    /** @name StagingXcmV5Location (74) */
    interface StagingXcmV5Location extends Struct {
        readonly parents: u8;
        readonly interior: StagingXcmV5Junctions;
    }

    /** @name StagingXcmV5Junctions (75) */
    interface StagingXcmV5Junctions extends Enum {
        readonly isHere: boolean;
        readonly isX1: boolean;
        readonly asX1: Vec<StagingXcmV5Junction>;
        readonly isX2: boolean;
        readonly asX2: Vec<StagingXcmV5Junction>;
        readonly isX3: boolean;
        readonly asX3: Vec<StagingXcmV5Junction>;
        readonly isX4: boolean;
        readonly asX4: Vec<StagingXcmV5Junction>;
        readonly isX5: boolean;
        readonly asX5: Vec<StagingXcmV5Junction>;
        readonly isX6: boolean;
        readonly asX6: Vec<StagingXcmV5Junction>;
        readonly isX7: boolean;
        readonly asX7: Vec<StagingXcmV5Junction>;
        readonly isX8: boolean;
        readonly asX8: Vec<StagingXcmV5Junction>;
        readonly type: "Here" | "X1" | "X2" | "X3" | "X4" | "X5" | "X6" | "X7" | "X8";
    }

    /** @name StagingXcmV5Junction (77) */
    interface StagingXcmV5Junction extends Enum {
        readonly isParachain: boolean;
        readonly asParachain: Compact<u32>;
        readonly isAccountId32: boolean;
        readonly asAccountId32: {
            readonly network: Option<StagingXcmV5JunctionNetworkId>;
            readonly id: U8aFixed;
        } & Struct;
        readonly isAccountIndex64: boolean;
        readonly asAccountIndex64: {
            readonly network: Option<StagingXcmV5JunctionNetworkId>;
            readonly index: Compact<u64>;
        } & Struct;
        readonly isAccountKey20: boolean;
        readonly asAccountKey20: {
            readonly network: Option<StagingXcmV5JunctionNetworkId>;
            readonly key: U8aFixed;
        } & Struct;
        readonly isPalletInstance: boolean;
        readonly asPalletInstance: u8;
        readonly isGeneralIndex: boolean;
        readonly asGeneralIndex: Compact<u128>;
        readonly isGeneralKey: boolean;
        readonly asGeneralKey: {
            readonly length: u8;
            readonly data: U8aFixed;
        } & Struct;
        readonly isOnlyChild: boolean;
        readonly isPlurality: boolean;
        readonly asPlurality: {
            readonly id: XcmV3JunctionBodyId;
            readonly part: XcmV3JunctionBodyPart;
        } & Struct;
        readonly isGlobalConsensus: boolean;
        readonly asGlobalConsensus: StagingXcmV5JunctionNetworkId;
        readonly type:
            | "Parachain"
            | "AccountId32"
            | "AccountIndex64"
            | "AccountKey20"
            | "PalletInstance"
            | "GeneralIndex"
            | "GeneralKey"
            | "OnlyChild"
            | "Plurality"
            | "GlobalConsensus";
    }

    /** @name StagingXcmV5JunctionNetworkId (80) */
    interface StagingXcmV5JunctionNetworkId extends Enum {
        readonly isByGenesis: boolean;
        readonly asByGenesis: U8aFixed;
        readonly isByFork: boolean;
        readonly asByFork: {
            readonly blockNumber: u64;
            readonly blockHash: U8aFixed;
        } & Struct;
        readonly isPolkadot: boolean;
        readonly isKusama: boolean;
        readonly isEthereum: boolean;
        readonly asEthereum: {
            readonly chainId: Compact<u64>;
        } & Struct;
        readonly isBitcoinCore: boolean;
        readonly isBitcoinCash: boolean;
        readonly isPolkadotBulletin: boolean;
        readonly type:
            | "ByGenesis"
            | "ByFork"
            | "Polkadot"
            | "Kusama"
            | "Ethereum"
            | "BitcoinCore"
            | "BitcoinCash"
            | "PolkadotBulletin";
    }

    /** @name XcmV3JunctionBodyId (82) */
    interface XcmV3JunctionBodyId extends Enum {
        readonly isUnit: boolean;
        readonly isMoniker: boolean;
        readonly asMoniker: U8aFixed;
        readonly isIndex: boolean;
        readonly asIndex: Compact<u32>;
        readonly isExecutive: boolean;
        readonly isTechnical: boolean;
        readonly isLegislative: boolean;
        readonly isJudicial: boolean;
        readonly isDefense: boolean;
        readonly isAdministration: boolean;
        readonly isTreasury: boolean;
        readonly type:
            | "Unit"
            | "Moniker"
            | "Index"
            | "Executive"
            | "Technical"
            | "Legislative"
            | "Judicial"
            | "Defense"
            | "Administration"
            | "Treasury";
    }

    /** @name XcmV3JunctionBodyPart (83) */
    interface XcmV3JunctionBodyPart extends Enum {
        readonly isVoice: boolean;
        readonly isMembers: boolean;
        readonly asMembers: {
            readonly count: Compact<u32>;
        } & Struct;
        readonly isFraction: boolean;
        readonly asFraction: {
            readonly nom: Compact<u32>;
            readonly denom: Compact<u32>;
        } & Struct;
        readonly isAtLeastProportion: boolean;
        readonly asAtLeastProportion: {
            readonly nom: Compact<u32>;
            readonly denom: Compact<u32>;
        } & Struct;
        readonly isMoreThanProportion: boolean;
        readonly asMoreThanProportion: {
            readonly nom: Compact<u32>;
            readonly denom: Compact<u32>;
        } & Struct;
        readonly type: "Voice" | "Members" | "Fraction" | "AtLeastProportion" | "MoreThanProportion";
    }

    /** @name SnowbridgeOutboundQueuePrimitivesOperatingMode (91) */
    interface SnowbridgeOutboundQueuePrimitivesOperatingMode extends Enum {
        readonly isNormal: boolean;
        readonly isRejectingOutboundMessages: boolean;
        readonly type: "Normal" | "RejectingOutboundMessages";
    }

    /** @name SnowbridgeCorePricingPricingParameters (94) */
    interface SnowbridgeCorePricingPricingParameters extends Struct {
        readonly exchangeRate: u128;
        readonly rewards: SnowbridgeCorePricingRewards;
        readonly feePerGas: U256;
        readonly multiplier: u128;
    }

    /** @name SnowbridgeCorePricingRewards (96) */
    interface SnowbridgeCorePricingRewards extends Struct {
        readonly local: u128;
        readonly remote: U256;
    }

    /** @name XcmVersionedLocation (97) */
    interface XcmVersionedLocation extends Enum {
        readonly isV3: boolean;
        readonly asV3: StagingXcmV3MultiLocation;
        readonly isV4: boolean;
        readonly asV4: StagingXcmV4Location;
        readonly isV5: boolean;
        readonly asV5: StagingXcmV5Location;
        readonly type: "V3" | "V4" | "V5";
    }

    /** @name StagingXcmV3MultiLocation (98) */
    interface StagingXcmV3MultiLocation extends Struct {
        readonly parents: u8;
        readonly interior: XcmV3Junctions;
    }

    /** @name XcmV3Junctions (99) */
    interface XcmV3Junctions extends Enum {
        readonly isHere: boolean;
        readonly isX1: boolean;
        readonly asX1: XcmV3Junction;
        readonly isX2: boolean;
        readonly asX2: ITuple<[XcmV3Junction, XcmV3Junction]>;
        readonly isX3: boolean;
        readonly asX3: ITuple<[XcmV3Junction, XcmV3Junction, XcmV3Junction]>;
        readonly isX4: boolean;
        readonly asX4: ITuple<[XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction]>;
        readonly isX5: boolean;
        readonly asX5: ITuple<[XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction]>;
        readonly isX6: boolean;
        readonly asX6: ITuple<
            [XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction]
        >;
        readonly isX7: boolean;
        readonly asX7: ITuple<
            [XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction, XcmV3Junction]
        >;
        readonly isX8: boolean;
        readonly asX8: ITuple<
            [
                XcmV3Junction,
                XcmV3Junction,
                XcmV3Junction,
                XcmV3Junction,
                XcmV3Junction,
                XcmV3Junction,
                XcmV3Junction,
                XcmV3Junction,
            ]
        >;
        readonly type: "Here" | "X1" | "X2" | "X3" | "X4" | "X5" | "X6" | "X7" | "X8";
    }

    /** @name XcmV3Junction (100) */
    interface XcmV3Junction extends Enum {
        readonly isParachain: boolean;
        readonly asParachain: Compact<u32>;
        readonly isAccountId32: boolean;
        readonly asAccountId32: {
            readonly network: Option<XcmV3JunctionNetworkId>;
            readonly id: U8aFixed;
        } & Struct;
        readonly isAccountIndex64: boolean;
        readonly asAccountIndex64: {
            readonly network: Option<XcmV3JunctionNetworkId>;
            readonly index: Compact<u64>;
        } & Struct;
        readonly isAccountKey20: boolean;
        readonly asAccountKey20: {
            readonly network: Option<XcmV3JunctionNetworkId>;
            readonly key: U8aFixed;
        } & Struct;
        readonly isPalletInstance: boolean;
        readonly asPalletInstance: u8;
        readonly isGeneralIndex: boolean;
        readonly asGeneralIndex: Compact<u128>;
        readonly isGeneralKey: boolean;
        readonly asGeneralKey: {
            readonly length: u8;
            readonly data: U8aFixed;
        } & Struct;
        readonly isOnlyChild: boolean;
        readonly isPlurality: boolean;
        readonly asPlurality: {
            readonly id: XcmV3JunctionBodyId;
            readonly part: XcmV3JunctionBodyPart;
        } & Struct;
        readonly isGlobalConsensus: boolean;
        readonly asGlobalConsensus: XcmV3JunctionNetworkId;
        readonly type:
            | "Parachain"
            | "AccountId32"
            | "AccountIndex64"
            | "AccountKey20"
            | "PalletInstance"
            | "GeneralIndex"
            | "GeneralKey"
            | "OnlyChild"
            | "Plurality"
            | "GlobalConsensus";
    }

    /** @name XcmV3JunctionNetworkId (102) */
    interface XcmV3JunctionNetworkId extends Enum {
        readonly isByGenesis: boolean;
        readonly asByGenesis: U8aFixed;
        readonly isByFork: boolean;
        readonly asByFork: {
            readonly blockNumber: u64;
            readonly blockHash: U8aFixed;
        } & Struct;
        readonly isPolkadot: boolean;
        readonly isKusama: boolean;
        readonly isWestend: boolean;
        readonly isRococo: boolean;
        readonly isWococo: boolean;
        readonly isEthereum: boolean;
        readonly asEthereum: {
            readonly chainId: Compact<u64>;
        } & Struct;
        readonly isBitcoinCore: boolean;
        readonly isBitcoinCash: boolean;
        readonly isPolkadotBulletin: boolean;
        readonly type:
            | "ByGenesis"
            | "ByFork"
            | "Polkadot"
            | "Kusama"
            | "Westend"
            | "Rococo"
            | "Wococo"
            | "Ethereum"
            | "BitcoinCore"
            | "BitcoinCash"
            | "PolkadotBulletin";
    }

    /** @name StagingXcmV4Location (103) */
    interface StagingXcmV4Location extends Struct {
        readonly parents: u8;
        readonly interior: StagingXcmV4Junctions;
    }

    /** @name StagingXcmV4Junctions (104) */
    interface StagingXcmV4Junctions extends Enum {
        readonly isHere: boolean;
        readonly isX1: boolean;
        readonly asX1: StagingXcmV4Junction;
        readonly isX2: boolean;
        readonly asX2: StagingXcmV4Junction;
        readonly isX3: boolean;
        readonly asX3: StagingXcmV4Junction;
        readonly isX4: boolean;
        readonly asX4: StagingXcmV4Junction;
        readonly isX5: boolean;
        readonly asX5: StagingXcmV4Junction;
        readonly isX6: boolean;
        readonly asX6: StagingXcmV4Junction;
        readonly isX7: boolean;
        readonly asX7: StagingXcmV4Junction;
        readonly isX8: boolean;
        readonly asX8: StagingXcmV4Junction;
        readonly type: "Here" | "X1" | "X2" | "X3" | "X4" | "X5" | "X6" | "X7" | "X8";
    }

    /** @name StagingXcmV4Junction (106) */
    interface StagingXcmV4Junction extends Enum {
        readonly isParachain: boolean;
        readonly asParachain: Compact<u32>;
        readonly isAccountId32: boolean;
        readonly asAccountId32: {
            readonly network: Option<StagingXcmV4JunctionNetworkId>;
            readonly id: U8aFixed;
        } & Struct;
        readonly isAccountIndex64: boolean;
        readonly asAccountIndex64: {
            readonly network: Option<StagingXcmV4JunctionNetworkId>;
            readonly index: Compact<u64>;
        } & Struct;
        readonly isAccountKey20: boolean;
        readonly asAccountKey20: {
            readonly network: Option<StagingXcmV4JunctionNetworkId>;
            readonly key: U8aFixed;
        } & Struct;
        readonly isPalletInstance: boolean;
        readonly asPalletInstance: u8;
        readonly isGeneralIndex: boolean;
        readonly asGeneralIndex: Compact<u128>;
        readonly isGeneralKey: boolean;
        readonly asGeneralKey: {
            readonly length: u8;
            readonly data: U8aFixed;
        } & Struct;
        readonly isOnlyChild: boolean;
        readonly isPlurality: boolean;
        readonly asPlurality: {
            readonly id: XcmV3JunctionBodyId;
            readonly part: XcmV3JunctionBodyPart;
        } & Struct;
        readonly isGlobalConsensus: boolean;
        readonly asGlobalConsensus: StagingXcmV4JunctionNetworkId;
        readonly type:
            | "Parachain"
            | "AccountId32"
            | "AccountIndex64"
            | "AccountKey20"
            | "PalletInstance"
            | "GeneralIndex"
            | "GeneralKey"
            | "OnlyChild"
            | "Plurality"
            | "GlobalConsensus";
    }

    /** @name StagingXcmV4JunctionNetworkId (108) */
    interface StagingXcmV4JunctionNetworkId extends Enum {
        readonly isByGenesis: boolean;
        readonly asByGenesis: U8aFixed;
        readonly isByFork: boolean;
        readonly asByFork: {
            readonly blockNumber: u64;
            readonly blockHash: U8aFixed;
        } & Struct;
        readonly isPolkadot: boolean;
        readonly isKusama: boolean;
        readonly isWestend: boolean;
        readonly isRococo: boolean;
        readonly isWococo: boolean;
        readonly isEthereum: boolean;
        readonly asEthereum: {
            readonly chainId: Compact<u64>;
        } & Struct;
        readonly isBitcoinCore: boolean;
        readonly isBitcoinCash: boolean;
        readonly isPolkadotBulletin: boolean;
        readonly type:
            | "ByGenesis"
            | "ByFork"
            | "Polkadot"
            | "Kusama"
            | "Westend"
            | "Rococo"
            | "Wococo"
            | "Ethereum"
            | "BitcoinCore"
            | "BitcoinCash"
            | "PolkadotBulletin";
    }

    /** @name PalletOutboundMessageCommitmentRecorderEvent (116) */
    interface PalletOutboundMessageCommitmentRecorderEvent extends Enum {
        readonly isNewCommitmentRootRecorded: boolean;
        readonly asNewCommitmentRootRecorded: {
            readonly commitment: H256;
        } & Struct;
        readonly isCommitmentRootRead: boolean;
        readonly asCommitmentRootRead: {
            readonly commitment: H256;
        } & Struct;
        readonly type: "NewCommitmentRootRecorded" | "CommitmentRootRead";
    }

    /** @name PalletEthereumTokenTransfersEvent (117) */
    interface PalletEthereumTokenTransfersEvent extends Enum {
        readonly isChannelInfoSet: boolean;
        readonly asChannelInfoSet: {
            readonly channelInfo: TpBridgeChannelInfo;
        } & Struct;
        readonly isNativeTokenTransferred: boolean;
        readonly asNativeTokenTransferred: {
            readonly messageId: H256;
            readonly channelId: SnowbridgeCoreChannelId;
            readonly source: AccountId32;
            readonly recipient: H160;
            readonly tokenId: H256;
            readonly amount: u128;
            readonly fee: u128;
        } & Struct;
        readonly type: "ChannelInfoSet" | "NativeTokenTransferred";
    }

    /** @name TpBridgeChannelInfo (118) */
    interface TpBridgeChannelInfo extends Struct {
        readonly channelId: SnowbridgeCoreChannelId;
        readonly paraId: u32;
        readonly agentId: H256;
    }

    /** @name PalletSessionEvent (119) */
    interface PalletSessionEvent extends Enum {
        readonly isNewSession: boolean;
        readonly asNewSession: {
            readonly sessionIndex: u32;
        } & Struct;
        readonly isNewQueued: boolean;
        readonly isValidatorDisabled: boolean;
        readonly asValidatorDisabled: {
            readonly validator: AccountId32;
        } & Struct;
        readonly isValidatorReenabled: boolean;
        readonly asValidatorReenabled: {
            readonly validator: AccountId32;
        } & Struct;
        readonly type: "NewSession" | "NewQueued" | "ValidatorDisabled" | "ValidatorReenabled";
    }

    /** @name PalletGrandpaEvent (120) */
    interface PalletGrandpaEvent extends Enum {
        readonly isNewAuthorities: boolean;
        readonly asNewAuthorities: {
            readonly authoritySet: Vec<ITuple<[SpConsensusGrandpaAppPublic, u64]>>;
        } & Struct;
        readonly isPaused: boolean;
        readonly isResumed: boolean;
        readonly type: "NewAuthorities" | "Paused" | "Resumed";
    }

    /** @name SpConsensusGrandpaAppPublic (123) */
    interface SpConsensusGrandpaAppPublic extends U8aFixed {}

    /** @name PalletInflationRewardsEvent (124) */
    interface PalletInflationRewardsEvent extends Enum {
        readonly isRewardedOrchestrator: boolean;
        readonly asRewardedOrchestrator: {
            readonly accountId: AccountId32;
            readonly balance: u128;
        } & Struct;
        readonly isRewardedContainer: boolean;
        readonly asRewardedContainer: {
            readonly accountId: AccountId32;
            readonly paraId: u32;
            readonly balance: u128;
        } & Struct;
        readonly type: "RewardedOrchestrator" | "RewardedContainer";
    }

    /** @name PalletPooledStakingEvent (125) */
    interface PalletPooledStakingEvent extends Enum {
        readonly isUpdatedCandidatePosition: boolean;
        readonly asUpdatedCandidatePosition: {
            readonly candidate: AccountId32;
            readonly stake: u128;
            readonly selfDelegation: u128;
            readonly before: Option<u32>;
            readonly after: Option<u32>;
        } & Struct;
        readonly isRequestedDelegate: boolean;
        readonly asRequestedDelegate: {
            readonly candidate: AccountId32;
            readonly delegator: AccountId32;
            readonly pool: PalletPooledStakingPoolsActivePoolKind;
            readonly pending: u128;
        } & Struct;
        readonly isExecutedDelegate: boolean;
        readonly asExecutedDelegate: {
            readonly candidate: AccountId32;
            readonly delegator: AccountId32;
            readonly pool: PalletPooledStakingPoolsActivePoolKind;
            readonly staked: u128;
            readonly released: u128;
        } & Struct;
        readonly isRequestedUndelegate: boolean;
        readonly asRequestedUndelegate: {
            readonly candidate: AccountId32;
            readonly delegator: AccountId32;
            readonly from: PalletPooledStakingPoolsActivePoolKind;
            readonly pending: u128;
            readonly released: u128;
        } & Struct;
        readonly isExecutedUndelegate: boolean;
        readonly asExecutedUndelegate: {
            readonly candidate: AccountId32;
            readonly delegator: AccountId32;
            readonly released: u128;
        } & Struct;
        readonly isIncreasedStake: boolean;
        readonly asIncreasedStake: {
            readonly candidate: AccountId32;
            readonly stakeDiff: u128;
        } & Struct;
        readonly isDecreasedStake: boolean;
        readonly asDecreasedStake: {
            readonly candidate: AccountId32;
            readonly stakeDiff: u128;
        } & Struct;
        readonly isStakedAutoCompounding: boolean;
        readonly asStakedAutoCompounding: {
            readonly candidate: AccountId32;
            readonly delegator: AccountId32;
            readonly shares: u128;
            readonly stake: u128;
        } & Struct;
        readonly isUnstakedAutoCompounding: boolean;
        readonly asUnstakedAutoCompounding: {
            readonly candidate: AccountId32;
            readonly delegator: AccountId32;
            readonly shares: u128;
            readonly stake: u128;
        } & Struct;
        readonly isStakedManualRewards: boolean;
        readonly asStakedManualRewards: {
            readonly candidate: AccountId32;
            readonly delegator: AccountId32;
            readonly shares: u128;
            readonly stake: u128;
        } & Struct;
        readonly isUnstakedManualRewards: boolean;
        readonly asUnstakedManualRewards: {
            readonly candidate: AccountId32;
            readonly delegator: AccountId32;
            readonly shares: u128;
            readonly stake: u128;
        } & Struct;
        readonly isRewardedCollator: boolean;
        readonly asRewardedCollator: {
            readonly collator: AccountId32;
            readonly autoCompoundingRewards: u128;
            readonly manualClaimRewards: u128;
        } & Struct;
        readonly isRewardedDelegators: boolean;
        readonly asRewardedDelegators: {
            readonly collator: AccountId32;
            readonly autoCompoundingRewards: u128;
            readonly manualClaimRewards: u128;
        } & Struct;
        readonly isClaimedManualRewards: boolean;
        readonly asClaimedManualRewards: {
            readonly candidate: AccountId32;
            readonly delegator: AccountId32;
            readonly rewards: u128;
        } & Struct;
        readonly isSwappedPool: boolean;
        readonly asSwappedPool: {
            readonly candidate: AccountId32;
            readonly delegator: AccountId32;
            readonly sourcePool: PalletPooledStakingPoolsActivePoolKind;
            readonly sourceShares: u128;
            readonly sourceStake: u128;
            readonly targetShares: u128;
            readonly targetStake: u128;
            readonly pendingLeaving: u128;
            readonly released: u128;
        } & Struct;
        readonly type:
            | "UpdatedCandidatePosition"
            | "RequestedDelegate"
            | "ExecutedDelegate"
            | "RequestedUndelegate"
            | "ExecutedUndelegate"
            | "IncreasedStake"
            | "DecreasedStake"
            | "StakedAutoCompounding"
            | "UnstakedAutoCompounding"
            | "StakedManualRewards"
            | "UnstakedManualRewards"
            | "RewardedCollator"
            | "RewardedDelegators"
            | "ClaimedManualRewards"
            | "SwappedPool";
    }

    /** @name PalletPooledStakingPoolsActivePoolKind (127) */
    interface PalletPooledStakingPoolsActivePoolKind extends Enum {
        readonly isAutoCompounding: boolean;
        readonly isManualRewards: boolean;
        readonly type: "AutoCompounding" | "ManualRewards";
    }

    /** @name PalletInactivityTrackingEvent (128) */
    interface PalletInactivityTrackingEvent extends Enum {
        readonly isActivityTrackingStatusSet: boolean;
        readonly asActivityTrackingStatusSet: {
            readonly status: PalletInactivityTrackingActivityTrackingStatus;
        } & Struct;
        readonly isCollatorStatusUpdated: boolean;
        readonly asCollatorStatusUpdated: {
            readonly collator: AccountId32;
            readonly isOffline: bool;
        } & Struct;
        readonly type: "ActivityTrackingStatusSet" | "CollatorStatusUpdated";
    }

    /** @name PalletInactivityTrackingActivityTrackingStatus (129) */
    interface PalletInactivityTrackingActivityTrackingStatus extends Enum {
        readonly isEnabled: boolean;
        readonly asEnabled: {
            readonly start: u32;
            readonly end: u32;
        } & Struct;
        readonly isDisabled: boolean;
        readonly asDisabled: {
            readonly end: u32;
        } & Struct;
        readonly type: "Enabled" | "Disabled";
    }

    /** @name PalletTreasuryEvent (130) */
    interface PalletTreasuryEvent extends Enum {
        readonly isSpending: boolean;
        readonly asSpending: {
            readonly budgetRemaining: u128;
        } & Struct;
        readonly isAwarded: boolean;
        readonly asAwarded: {
            readonly proposalIndex: u32;
            readonly award: u128;
            readonly account: AccountId32;
        } & Struct;
        readonly isBurnt: boolean;
        readonly asBurnt: {
            readonly burntFunds: u128;
        } & Struct;
        readonly isRollover: boolean;
        readonly asRollover: {
            readonly rolloverBalance: u128;
        } & Struct;
        readonly isDeposit: boolean;
        readonly asDeposit: {
            readonly value: u128;
        } & Struct;
        readonly isSpendApproved: boolean;
        readonly asSpendApproved: {
            readonly proposalIndex: u32;
            readonly amount: u128;
            readonly beneficiary: AccountId32;
        } & Struct;
        readonly isUpdatedInactive: boolean;
        readonly asUpdatedInactive: {
            readonly reactivated: u128;
            readonly deactivated: u128;
        } & Struct;
        readonly isAssetSpendApproved: boolean;
        readonly asAssetSpendApproved: {
            readonly index: u32;
            readonly assetKind: Null;
            readonly amount: u128;
            readonly beneficiary: AccountId32;
            readonly validFrom: u32;
            readonly expireAt: u32;
        } & Struct;
        readonly isAssetSpendVoided: boolean;
        readonly asAssetSpendVoided: {
            readonly index: u32;
        } & Struct;
        readonly isPaid: boolean;
        readonly asPaid: {
            readonly index: u32;
            readonly paymentId: Null;
        } & Struct;
        readonly isPaymentFailed: boolean;
        readonly asPaymentFailed: {
            readonly index: u32;
            readonly paymentId: Null;
        } & Struct;
        readonly isSpendProcessed: boolean;
        readonly asSpendProcessed: {
            readonly index: u32;
        } & Struct;
        readonly type:
            | "Spending"
            | "Awarded"
            | "Burnt"
            | "Rollover"
            | "Deposit"
            | "SpendApproved"
            | "UpdatedInactive"
            | "AssetSpendApproved"
            | "AssetSpendVoided"
            | "Paid"
            | "PaymentFailed"
            | "SpendProcessed";
    }

    /** @name PalletConvictionVotingEvent (132) */
    interface PalletConvictionVotingEvent extends Enum {
        readonly isDelegated: boolean;
        readonly asDelegated: ITuple<[AccountId32, AccountId32]>;
        readonly isUndelegated: boolean;
        readonly asUndelegated: AccountId32;
        readonly isVoted: boolean;
        readonly asVoted: {
            readonly who: AccountId32;
            readonly vote: PalletConvictionVotingVoteAccountVote;
        } & Struct;
        readonly isVoteRemoved: boolean;
        readonly asVoteRemoved: {
            readonly who: AccountId32;
            readonly vote: PalletConvictionVotingVoteAccountVote;
        } & Struct;
        readonly isVoteUnlocked: boolean;
        readonly asVoteUnlocked: {
            readonly who: AccountId32;
            readonly class: u16;
        } & Struct;
        readonly type: "Delegated" | "Undelegated" | "Voted" | "VoteRemoved" | "VoteUnlocked";
    }

    /** @name PalletConvictionVotingVoteAccountVote (133) */
    interface PalletConvictionVotingVoteAccountVote extends Enum {
        readonly isStandard: boolean;
        readonly asStandard: {
            readonly vote: Vote;
            readonly balance: u128;
        } & Struct;
        readonly isSplit: boolean;
        readonly asSplit: {
            readonly aye: u128;
            readonly nay: u128;
        } & Struct;
        readonly isSplitAbstain: boolean;
        readonly asSplitAbstain: {
            readonly aye: u128;
            readonly nay: u128;
            readonly abstain: u128;
        } & Struct;
        readonly type: "Standard" | "Split" | "SplitAbstain";
    }

    /** @name PalletReferendaEvent (136) */
    interface PalletReferendaEvent extends Enum {
        readonly isSubmitted: boolean;
        readonly asSubmitted: {
            readonly index: u32;
            readonly track: u16;
            readonly proposal: FrameSupportPreimagesBounded;
        } & Struct;
        readonly isDecisionDepositPlaced: boolean;
        readonly asDecisionDepositPlaced: {
            readonly index: u32;
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isDecisionDepositRefunded: boolean;
        readonly asDecisionDepositRefunded: {
            readonly index: u32;
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isDepositSlashed: boolean;
        readonly asDepositSlashed: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isDecisionStarted: boolean;
        readonly asDecisionStarted: {
            readonly index: u32;
            readonly track: u16;
            readonly proposal: FrameSupportPreimagesBounded;
            readonly tally: PalletConvictionVotingTally;
        } & Struct;
        readonly isConfirmStarted: boolean;
        readonly asConfirmStarted: {
            readonly index: u32;
        } & Struct;
        readonly isConfirmAborted: boolean;
        readonly asConfirmAborted: {
            readonly index: u32;
        } & Struct;
        readonly isConfirmed: boolean;
        readonly asConfirmed: {
            readonly index: u32;
            readonly tally: PalletConvictionVotingTally;
        } & Struct;
        readonly isApproved: boolean;
        readonly asApproved: {
            readonly index: u32;
        } & Struct;
        readonly isRejected: boolean;
        readonly asRejected: {
            readonly index: u32;
            readonly tally: PalletConvictionVotingTally;
        } & Struct;
        readonly isTimedOut: boolean;
        readonly asTimedOut: {
            readonly index: u32;
            readonly tally: PalletConvictionVotingTally;
        } & Struct;
        readonly isCancelled: boolean;
        readonly asCancelled: {
            readonly index: u32;
            readonly tally: PalletConvictionVotingTally;
        } & Struct;
        readonly isKilled: boolean;
        readonly asKilled: {
            readonly index: u32;
            readonly tally: PalletConvictionVotingTally;
        } & Struct;
        readonly isSubmissionDepositRefunded: boolean;
        readonly asSubmissionDepositRefunded: {
            readonly index: u32;
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isMetadataSet: boolean;
        readonly asMetadataSet: {
            readonly index: u32;
            readonly hash_: H256;
        } & Struct;
        readonly isMetadataCleared: boolean;
        readonly asMetadataCleared: {
            readonly index: u32;
            readonly hash_: H256;
        } & Struct;
        readonly type:
            | "Submitted"
            | "DecisionDepositPlaced"
            | "DecisionDepositRefunded"
            | "DepositSlashed"
            | "DecisionStarted"
            | "ConfirmStarted"
            | "ConfirmAborted"
            | "Confirmed"
            | "Approved"
            | "Rejected"
            | "TimedOut"
            | "Cancelled"
            | "Killed"
            | "SubmissionDepositRefunded"
            | "MetadataSet"
            | "MetadataCleared";
    }

    /** @name FrameSupportPreimagesBounded (137) */
    interface FrameSupportPreimagesBounded extends Enum {
        readonly isLegacy: boolean;
        readonly asLegacy: {
            readonly hash_: H256;
        } & Struct;
        readonly isInline: boolean;
        readonly asInline: Bytes;
        readonly isLookup: boolean;
        readonly asLookup: {
            readonly hash_: H256;
            readonly len: u32;
        } & Struct;
        readonly type: "Legacy" | "Inline" | "Lookup";
    }

    /** @name FrameSystemCall (139) */
    interface FrameSystemCall extends Enum {
        readonly isRemark: boolean;
        readonly asRemark: {
            readonly remark: Bytes;
        } & Struct;
        readonly isSetHeapPages: boolean;
        readonly asSetHeapPages: {
            readonly pages: u64;
        } & Struct;
        readonly isSetCode: boolean;
        readonly asSetCode: {
            readonly code: Bytes;
        } & Struct;
        readonly isSetCodeWithoutChecks: boolean;
        readonly asSetCodeWithoutChecks: {
            readonly code: Bytes;
        } & Struct;
        readonly isSetStorage: boolean;
        readonly asSetStorage: {
            readonly items: Vec<ITuple<[Bytes, Bytes]>>;
        } & Struct;
        readonly isKillStorage: boolean;
        readonly asKillStorage: {
            readonly keys_: Vec<Bytes>;
        } & Struct;
        readonly isKillPrefix: boolean;
        readonly asKillPrefix: {
            readonly prefix: Bytes;
            readonly subkeys: u32;
        } & Struct;
        readonly isRemarkWithEvent: boolean;
        readonly asRemarkWithEvent: {
            readonly remark: Bytes;
        } & Struct;
        readonly isAuthorizeUpgrade: boolean;
        readonly asAuthorizeUpgrade: {
            readonly codeHash: H256;
        } & Struct;
        readonly isAuthorizeUpgradeWithoutChecks: boolean;
        readonly asAuthorizeUpgradeWithoutChecks: {
            readonly codeHash: H256;
        } & Struct;
        readonly isApplyAuthorizedUpgrade: boolean;
        readonly asApplyAuthorizedUpgrade: {
            readonly code: Bytes;
        } & Struct;
        readonly type:
            | "Remark"
            | "SetHeapPages"
            | "SetCode"
            | "SetCodeWithoutChecks"
            | "SetStorage"
            | "KillStorage"
            | "KillPrefix"
            | "RemarkWithEvent"
            | "AuthorizeUpgrade"
            | "AuthorizeUpgradeWithoutChecks"
            | "ApplyAuthorizedUpgrade";
    }

    /** @name PalletBabeCall (143) */
    interface PalletBabeCall extends Enum {
        readonly isReportEquivocation: boolean;
        readonly asReportEquivocation: {
            readonly equivocationProof: SpConsensusSlotsEquivocationProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly isReportEquivocationUnsigned: boolean;
        readonly asReportEquivocationUnsigned: {
            readonly equivocationProof: SpConsensusSlotsEquivocationProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly isPlanConfigChange: boolean;
        readonly asPlanConfigChange: {
            readonly config: SpConsensusBabeDigestsNextConfigDescriptor;
        } & Struct;
        readonly type: "ReportEquivocation" | "ReportEquivocationUnsigned" | "PlanConfigChange";
    }

    /** @name SpConsensusSlotsEquivocationProof (144) */
    interface SpConsensusSlotsEquivocationProof extends Struct {
        readonly offender: SpConsensusBabeAppPublic;
        readonly slot: u64;
        readonly firstHeader: SpRuntimeHeader;
        readonly secondHeader: SpRuntimeHeader;
    }

    /** @name SpRuntimeHeader (145) */
    interface SpRuntimeHeader extends Struct {
        readonly parentHash: H256;
        readonly number: Compact<u32>;
        readonly stateRoot: H256;
        readonly extrinsicsRoot: H256;
        readonly digest: SpRuntimeDigest;
    }

    /** @name SpConsensusBabeAppPublic (146) */
    interface SpConsensusBabeAppPublic extends U8aFixed {}

    /** @name SpSessionMembershipProof (147) */
    interface SpSessionMembershipProof extends Struct {
        readonly session: u32;
        readonly trieNodes: Vec<Bytes>;
        readonly validatorCount: u32;
    }

    /** @name SpConsensusBabeDigestsNextConfigDescriptor (148) */
    interface SpConsensusBabeDigestsNextConfigDescriptor extends Enum {
        readonly isV1: boolean;
        readonly asV1: {
            readonly c: ITuple<[u64, u64]>;
            readonly allowedSlots: SpConsensusBabeAllowedSlots;
        } & Struct;
        readonly type: "V1";
    }

    /** @name SpConsensusBabeAllowedSlots (150) */
    interface SpConsensusBabeAllowedSlots extends Enum {
        readonly isPrimarySlots: boolean;
        readonly isPrimaryAndSecondaryPlainSlots: boolean;
        readonly isPrimaryAndSecondaryVRFSlots: boolean;
        readonly type: "PrimarySlots" | "PrimaryAndSecondaryPlainSlots" | "PrimaryAndSecondaryVRFSlots";
    }

    /** @name PalletTimestampCall (151) */
    interface PalletTimestampCall extends Enum {
        readonly isSet: boolean;
        readonly asSet: {
            readonly now: Compact<u64>;
        } & Struct;
        readonly type: "Set";
    }

    /** @name PalletBalancesCall (152) */
    interface PalletBalancesCall extends Enum {
        readonly isTransferAllowDeath: boolean;
        readonly asTransferAllowDeath: {
            readonly dest: MultiAddress;
            readonly value: Compact<u128>;
        } & Struct;
        readonly isForceTransfer: boolean;
        readonly asForceTransfer: {
            readonly source: MultiAddress;
            readonly dest: MultiAddress;
            readonly value: Compact<u128>;
        } & Struct;
        readonly isTransferKeepAlive: boolean;
        readonly asTransferKeepAlive: {
            readonly dest: MultiAddress;
            readonly value: Compact<u128>;
        } & Struct;
        readonly isTransferAll: boolean;
        readonly asTransferAll: {
            readonly dest: MultiAddress;
            readonly keepAlive: bool;
        } & Struct;
        readonly isForceUnreserve: boolean;
        readonly asForceUnreserve: {
            readonly who: MultiAddress;
            readonly amount: u128;
        } & Struct;
        readonly isUpgradeAccounts: boolean;
        readonly asUpgradeAccounts: {
            readonly who: Vec<AccountId32>;
        } & Struct;
        readonly isForceSetBalance: boolean;
        readonly asForceSetBalance: {
            readonly who: MultiAddress;
            readonly newFree: Compact<u128>;
        } & Struct;
        readonly isForceAdjustTotalIssuance: boolean;
        readonly asForceAdjustTotalIssuance: {
            readonly direction: PalletBalancesAdjustmentDirection;
            readonly delta: Compact<u128>;
        } & Struct;
        readonly isBurn: boolean;
        readonly asBurn: {
            readonly value: Compact<u128>;
            readonly keepAlive: bool;
        } & Struct;
        readonly type:
            | "TransferAllowDeath"
            | "ForceTransfer"
            | "TransferKeepAlive"
            | "TransferAll"
            | "ForceUnreserve"
            | "UpgradeAccounts"
            | "ForceSetBalance"
            | "ForceAdjustTotalIssuance"
            | "Burn";
    }

    /** @name PalletBalancesAdjustmentDirection (155) */
    interface PalletBalancesAdjustmentDirection extends Enum {
        readonly isIncrease: boolean;
        readonly isDecrease: boolean;
        readonly type: "Increase" | "Decrease";
    }

    /** @name PalletParametersCall (156) */
    interface PalletParametersCall extends Enum {
        readonly isSetParameter: boolean;
        readonly asSetParameter: {
            readonly keyValue: DancelightRuntimeRuntimeParameters;
        } & Struct;
        readonly type: "SetParameter";
    }

    /** @name DancelightRuntimeRuntimeParameters (157) */
    interface DancelightRuntimeRuntimeParameters extends Enum {
        readonly isPreimage: boolean;
        readonly asPreimage: DancelightRuntimeDynamicParamsPreimageParameters;
        readonly type: "Preimage";
    }

    /** @name DancelightRuntimeDynamicParamsPreimageParameters (158) */
    interface DancelightRuntimeDynamicParamsPreimageParameters extends Enum {
        readonly isBaseDeposit: boolean;
        readonly asBaseDeposit: ITuple<[DancelightRuntimeDynamicParamsPreimageBaseDeposit, Option<u128>]>;
        readonly isByteDeposit: boolean;
        readonly asByteDeposit: ITuple<[DancelightRuntimeDynamicParamsPreimageByteDeposit, Option<u128>]>;
        readonly type: "BaseDeposit" | "ByteDeposit";
    }

    /** @name PalletRegistrarCall (160) */
    interface PalletRegistrarCall extends Enum {
        readonly isRegister: boolean;
        readonly asRegister: {
            readonly paraId: u32;
            readonly genesisData: DpContainerChainGenesisDataContainerChainGenesisData;
            readonly headData: Option<Bytes>;
        } & Struct;
        readonly isDeregister: boolean;
        readonly asDeregister: {
            readonly paraId: u32;
        } & Struct;
        readonly isMarkValidForCollating: boolean;
        readonly asMarkValidForCollating: {
            readonly paraId: u32;
        } & Struct;
        readonly isPauseContainerChain: boolean;
        readonly asPauseContainerChain: {
            readonly paraId: u32;
        } & Struct;
        readonly isUnpauseContainerChain: boolean;
        readonly asUnpauseContainerChain: {
            readonly paraId: u32;
        } & Struct;
        readonly isRegisterParathread: boolean;
        readonly asRegisterParathread: {
            readonly paraId: u32;
            readonly slotFrequency: TpTraitsSlotFrequency;
            readonly genesisData: DpContainerChainGenesisDataContainerChainGenesisData;
            readonly headData: Option<Bytes>;
        } & Struct;
        readonly isSetParathreadParams: boolean;
        readonly asSetParathreadParams: {
            readonly paraId: u32;
            readonly slotFrequency: TpTraitsSlotFrequency;
        } & Struct;
        readonly isSetParaManager: boolean;
        readonly asSetParaManager: {
            readonly paraId: u32;
            readonly managerAddress: AccountId32;
        } & Struct;
        readonly isRegisterWithRelayProof: boolean;
        readonly asRegisterWithRelayProof: {
            readonly paraId: u32;
            readonly parathreadParams: Option<TpTraitsParathreadParams>;
            readonly relayProofBlockNumber: u32;
            readonly relayStorageProof: SpTrieStorageProof;
            readonly managerSignature: SpRuntimeMultiSignature;
            readonly genesisData: DpContainerChainGenesisDataContainerChainGenesisData;
            readonly headData: Option<Bytes>;
        } & Struct;
        readonly isDeregisterWithRelayProof: boolean;
        readonly asDeregisterWithRelayProof: {
            readonly paraId: u32;
            readonly relayProofBlockNumber: u32;
            readonly relayStorageProof: SpTrieStorageProof;
        } & Struct;
        readonly type:
            | "Register"
            | "Deregister"
            | "MarkValidForCollating"
            | "PauseContainerChain"
            | "UnpauseContainerChain"
            | "RegisterParathread"
            | "SetParathreadParams"
            | "SetParaManager"
            | "RegisterWithRelayProof"
            | "DeregisterWithRelayProof";
    }

    /** @name DpContainerChainGenesisDataContainerChainGenesisData (161) */
    interface DpContainerChainGenesisDataContainerChainGenesisData extends Struct {
        readonly storage: Vec<DpContainerChainGenesisDataContainerChainGenesisDataItem>;
        readonly name: Bytes;
        readonly id: Bytes;
        readonly forkId: Option<Bytes>;
        readonly extensions: Bytes;
        readonly properties: DpContainerChainGenesisDataProperties;
    }

    /** @name DpContainerChainGenesisDataContainerChainGenesisDataItem (163) */
    interface DpContainerChainGenesisDataContainerChainGenesisDataItem extends Struct {
        readonly key: Bytes;
        readonly value: Bytes;
    }

    /** @name DpContainerChainGenesisDataProperties (167) */
    interface DpContainerChainGenesisDataProperties extends Struct {
        readonly tokenMetadata: DpContainerChainGenesisDataTokenMetadata;
        readonly isEthereum: bool;
    }

    /** @name DpContainerChainGenesisDataTokenMetadata (168) */
    interface DpContainerChainGenesisDataTokenMetadata extends Struct {
        readonly tokenSymbol: Bytes;
        readonly ss58Format: u32;
        readonly tokenDecimals: u32;
    }

    /** @name TpTraitsSlotFrequency (172) */
    interface TpTraitsSlotFrequency extends Struct {
        readonly min: u32;
        readonly max: u32;
    }

    /** @name TpTraitsParathreadParams (174) */
    interface TpTraitsParathreadParams extends Struct {
        readonly slotFrequency: TpTraitsSlotFrequency;
    }

    /** @name SpTrieStorageProof (175) */
    interface SpTrieStorageProof extends Struct {
        readonly trieNodes: BTreeSet<Bytes>;
    }

    /** @name SpRuntimeMultiSignature (177) */
    interface SpRuntimeMultiSignature extends Enum {
        readonly isEd25519: boolean;
        readonly asEd25519: U8aFixed;
        readonly isSr25519: boolean;
        readonly asSr25519: U8aFixed;
        readonly isEcdsa: boolean;
        readonly asEcdsa: U8aFixed;
        readonly type: "Ed25519" | "Sr25519" | "Ecdsa";
    }

    /** @name PalletConfigurationCall (180) */
    interface PalletConfigurationCall extends Enum {
        readonly isSetMaxCollators: boolean;
        readonly asSetMaxCollators: {
            readonly new_: u32;
        } & Struct;
        readonly isSetMinOrchestratorCollators: boolean;
        readonly asSetMinOrchestratorCollators: {
            readonly new_: u32;
        } & Struct;
        readonly isSetMaxOrchestratorCollators: boolean;
        readonly asSetMaxOrchestratorCollators: {
            readonly new_: u32;
        } & Struct;
        readonly isSetCollatorsPerContainer: boolean;
        readonly asSetCollatorsPerContainer: {
            readonly new_: u32;
        } & Struct;
        readonly isSetFullRotationPeriod: boolean;
        readonly asSetFullRotationPeriod: {
            readonly new_: u32;
        } & Struct;
        readonly isSetCollatorsPerParathread: boolean;
        readonly asSetCollatorsPerParathread: {
            readonly new_: u32;
        } & Struct;
        readonly isSetParathreadsPerCollator: boolean;
        readonly asSetParathreadsPerCollator: {
            readonly new_: u32;
        } & Struct;
        readonly isSetTargetContainerChainFullness: boolean;
        readonly asSetTargetContainerChainFullness: {
            readonly new_: Perbill;
        } & Struct;
        readonly isSetMaxParachainCoresPercentage: boolean;
        readonly asSetMaxParachainCoresPercentage: {
            readonly new_: Option<Perbill>;
        } & Struct;
        readonly isSetFullRotationMode: boolean;
        readonly asSetFullRotationMode: {
            readonly orchestrator: Option<TpTraitsFullRotationMode>;
            readonly parachain: Option<TpTraitsFullRotationMode>;
            readonly parathread: Option<TpTraitsFullRotationMode>;
        } & Struct;
        readonly isSetBypassConsistencyCheck: boolean;
        readonly asSetBypassConsistencyCheck: {
            readonly new_: bool;
        } & Struct;
        readonly type:
            | "SetMaxCollators"
            | "SetMinOrchestratorCollators"
            | "SetMaxOrchestratorCollators"
            | "SetCollatorsPerContainer"
            | "SetFullRotationPeriod"
            | "SetCollatorsPerParathread"
            | "SetParathreadsPerCollator"
            | "SetTargetContainerChainFullness"
            | "SetMaxParachainCoresPercentage"
            | "SetFullRotationMode"
            | "SetBypassConsistencyCheck";
    }

    /** @name PalletInvulnerablesCall (183) */
    interface PalletInvulnerablesCall extends Enum {
        readonly isAddInvulnerable: boolean;
        readonly asAddInvulnerable: {
            readonly who: AccountId32;
        } & Struct;
        readonly isRemoveInvulnerable: boolean;
        readonly asRemoveInvulnerable: {
            readonly who: AccountId32;
        } & Struct;
        readonly type: "AddInvulnerable" | "RemoveInvulnerable";
    }

    /** @name PalletCollatorAssignmentCall (184) */
    type PalletCollatorAssignmentCall = Null;

    /** @name PalletAuthorityAssignmentCall (185) */
    type PalletAuthorityAssignmentCall = Null;

    /** @name PalletAuthorNotingCall (186) */
    interface PalletAuthorNotingCall extends Enum {
        readonly isSetLatestAuthorData: boolean;
        readonly asSetLatestAuthorData: {
            readonly data: Null;
        } & Struct;
        readonly isSetAuthor: boolean;
        readonly asSetAuthor: {
            readonly paraId: u32;
            readonly blockNumber: u32;
            readonly author: AccountId32;
            readonly latestSlotNumber: u64;
        } & Struct;
        readonly isKillAuthorData: boolean;
        readonly asKillAuthorData: {
            readonly paraId: u32;
        } & Struct;
        readonly type: "SetLatestAuthorData" | "SetAuthor" | "KillAuthorData";
    }

    /** @name PalletServicesPaymentCall (187) */
    interface PalletServicesPaymentCall extends Enum {
        readonly isPurchaseCredits: boolean;
        readonly asPurchaseCredits: {
            readonly paraId: u32;
            readonly credit: u128;
        } & Struct;
        readonly isSetBlockProductionCredits: boolean;
        readonly asSetBlockProductionCredits: {
            readonly paraId: u32;
            readonly freeBlockCredits: u32;
        } & Struct;
        readonly isSetGivenFreeCredits: boolean;
        readonly asSetGivenFreeCredits: {
            readonly paraId: u32;
            readonly givenFreeCredits: bool;
        } & Struct;
        readonly isSetRefundAddress: boolean;
        readonly asSetRefundAddress: {
            readonly paraId: u32;
            readonly refundAddress: Option<AccountId32>;
        } & Struct;
        readonly isSetCollatorAssignmentCredits: boolean;
        readonly asSetCollatorAssignmentCredits: {
            readonly paraId: u32;
            readonly freeCollatorAssignmentCredits: u32;
        } & Struct;
        readonly isSetMaxCorePrice: boolean;
        readonly asSetMaxCorePrice: {
            readonly paraId: u32;
            readonly maxCorePrice: u128;
        } & Struct;
        readonly isSetMaxTip: boolean;
        readonly asSetMaxTip: {
            readonly paraId: u32;
            readonly maxTip: Option<u128>;
        } & Struct;
        readonly type:
            | "PurchaseCredits"
            | "SetBlockProductionCredits"
            | "SetGivenFreeCredits"
            | "SetRefundAddress"
            | "SetCollatorAssignmentCredits"
            | "SetMaxCorePrice"
            | "SetMaxTip";
    }

    /** @name PalletDataPreserversCall (188) */
    interface PalletDataPreserversCall extends Enum {
        readonly isCreateProfile: boolean;
        readonly asCreateProfile: {
            readonly profile: PalletDataPreserversProfile;
        } & Struct;
        readonly isUpdateProfile: boolean;
        readonly asUpdateProfile: {
            readonly profileId: u64;
            readonly profile: PalletDataPreserversProfile;
        } & Struct;
        readonly isDeleteProfile: boolean;
        readonly asDeleteProfile: {
            readonly profileId: u64;
        } & Struct;
        readonly isForceCreateProfile: boolean;
        readonly asForceCreateProfile: {
            readonly profile: PalletDataPreserversProfile;
            readonly forAccount: AccountId32;
        } & Struct;
        readonly isForceUpdateProfile: boolean;
        readonly asForceUpdateProfile: {
            readonly profileId: u64;
            readonly profile: PalletDataPreserversProfile;
        } & Struct;
        readonly isForceDeleteProfile: boolean;
        readonly asForceDeleteProfile: {
            readonly profileId: u64;
        } & Struct;
        readonly isStartAssignment: boolean;
        readonly asStartAssignment: {
            readonly profileId: u64;
            readonly paraId: u32;
            readonly assignerParam: TpDataPreserversCommonAssignerExtra;
        } & Struct;
        readonly isStopAssignment: boolean;
        readonly asStopAssignment: {
            readonly profileId: u64;
            readonly paraId: u32;
        } & Struct;
        readonly isForceStartAssignment: boolean;
        readonly asForceStartAssignment: {
            readonly profileId: u64;
            readonly paraId: u32;
            readonly assignmentWitness: TpDataPreserversCommonAssignmentWitness;
        } & Struct;
        readonly type:
            | "CreateProfile"
            | "UpdateProfile"
            | "DeleteProfile"
            | "ForceCreateProfile"
            | "ForceUpdateProfile"
            | "ForceDeleteProfile"
            | "StartAssignment"
            | "StopAssignment"
            | "ForceStartAssignment";
    }

    /** @name PalletDataPreserversProfile (189) */
    interface PalletDataPreserversProfile extends Struct {
        readonly url: Bytes;
        readonly paraIds: PalletDataPreserversParaIdsFilter;
        readonly mode: PalletDataPreserversProfileMode;
        readonly assignmentRequest: TpDataPreserversCommonProviderRequest;
    }

    /** @name PalletDataPreserversParaIdsFilter (191) */
    interface PalletDataPreserversParaIdsFilter extends Enum {
        readonly isAnyParaId: boolean;
        readonly isWhitelist: boolean;
        readonly asWhitelist: BTreeSet<u32>;
        readonly isBlacklist: boolean;
        readonly asBlacklist: BTreeSet<u32>;
        readonly type: "AnyParaId" | "Whitelist" | "Blacklist";
    }

    /** @name PalletDataPreserversProfileMode (195) */
    interface PalletDataPreserversProfileMode extends Enum {
        readonly isBootnode: boolean;
        readonly isRpc: boolean;
        readonly asRpc: {
            readonly supportsEthereumRpcs: bool;
        } & Struct;
        readonly type: "Bootnode" | "Rpc";
    }

    /** @name TpDataPreserversCommonProviderRequest (196) */
    interface TpDataPreserversCommonProviderRequest extends Enum {
        readonly isFree: boolean;
        readonly isStreamPayment: boolean;
        readonly asStreamPayment: {
            readonly config: PalletStreamPaymentStreamConfig;
        } & Struct;
        readonly type: "Free" | "StreamPayment";
    }

    /** @name PalletStreamPaymentStreamConfig (197) */
    interface PalletStreamPaymentStreamConfig extends Struct {
        readonly timeUnit: TpStreamPaymentCommonTimeUnit;
        readonly assetId: TpStreamPaymentCommonAssetId;
        readonly rate: u128;
        readonly minimumRequestDeadlineDelay: u128;
        readonly softMinimumDeposit: u128;
    }

    /** @name TpStreamPaymentCommonTimeUnit (198) */
    interface TpStreamPaymentCommonTimeUnit extends Enum {
        readonly isBlockNumber: boolean;
        readonly isTimestamp: boolean;
        readonly type: "BlockNumber" | "Timestamp";
    }

    /** @name TpStreamPaymentCommonAssetId (199) */
    interface TpStreamPaymentCommonAssetId extends Enum {
        readonly isNative: boolean;
        readonly type: "Native";
    }

    /** @name TpDataPreserversCommonAssignerExtra (200) */
    interface TpDataPreserversCommonAssignerExtra extends Enum {
        readonly isFree: boolean;
        readonly isStreamPayment: boolean;
        readonly asStreamPayment: {
            readonly initialDeposit: u128;
        } & Struct;
        readonly type: "Free" | "StreamPayment";
    }

    /** @name TpDataPreserversCommonAssignmentWitness (201) */
    interface TpDataPreserversCommonAssignmentWitness extends Enum {
        readonly isFree: boolean;
        readonly isStreamPayment: boolean;
        readonly asStreamPayment: {
            readonly streamId: u64;
        } & Struct;
        readonly type: "Free" | "StreamPayment";
    }

    /** @name PalletExternalValidatorsCall (202) */
    interface PalletExternalValidatorsCall extends Enum {
        readonly isSkipExternalValidators: boolean;
        readonly asSkipExternalValidators: {
            readonly skip: bool;
        } & Struct;
        readonly isAddWhitelisted: boolean;
        readonly asAddWhitelisted: {
            readonly who: AccountId32;
        } & Struct;
        readonly isRemoveWhitelisted: boolean;
        readonly asRemoveWhitelisted: {
            readonly who: AccountId32;
        } & Struct;
        readonly isForceEra: boolean;
        readonly asForceEra: {
            readonly mode: PalletExternalValidatorsForcing;
        } & Struct;
        readonly isSetExternalValidators: boolean;
        readonly asSetExternalValidators: {
            readonly validators: Vec<AccountId32>;
            readonly externalIndex: u64;
        } & Struct;
        readonly type:
            | "SkipExternalValidators"
            | "AddWhitelisted"
            | "RemoveWhitelisted"
            | "ForceEra"
            | "SetExternalValidators";
    }

    /** @name PalletExternalValidatorSlashesCall (203) */
    interface PalletExternalValidatorSlashesCall extends Enum {
        readonly isCancelDeferredSlash: boolean;
        readonly asCancelDeferredSlash: {
            readonly era: u32;
            readonly slashIndices: Vec<u32>;
        } & Struct;
        readonly isForceInjectSlash: boolean;
        readonly asForceInjectSlash: {
            readonly era: u32;
            readonly validator: AccountId32;
            readonly percentage: Perbill;
            readonly externalIdx: u64;
        } & Struct;
        readonly isRootTestSendMsgToEth: boolean;
        readonly asRootTestSendMsgToEth: {
            readonly nonce: H256;
            readonly numMsgs: u32;
            readonly msgSize: u32;
        } & Struct;
        readonly isSetSlashingMode: boolean;
        readonly asSetSlashingMode: {
            readonly mode: PalletExternalValidatorSlashesSlashingModeOption;
        } & Struct;
        readonly type: "CancelDeferredSlash" | "ForceInjectSlash" | "RootTestSendMsgToEth" | "SetSlashingMode";
    }

    /** @name PalletExternalValidatorSlashesSlashingModeOption (205) */
    interface PalletExternalValidatorSlashesSlashingModeOption extends Enum {
        readonly isEnabled: boolean;
        readonly isLogOnly: boolean;
        readonly isDisabled: boolean;
        readonly type: "Enabled" | "LogOnly" | "Disabled";
    }

    /** @name SnowbridgePalletOutboundQueueCall (206) */
    interface SnowbridgePalletOutboundQueueCall extends Enum {
        readonly isSetOperatingMode: boolean;
        readonly asSetOperatingMode: {
            readonly mode: SnowbridgeCoreOperatingModeBasicOperatingMode;
        } & Struct;
        readonly type: "SetOperatingMode";
    }

    /** @name SnowbridgePalletInboundQueueCall (207) */
    interface SnowbridgePalletInboundQueueCall extends Enum {
        readonly isSubmit: boolean;
        readonly asSubmit: {
            readonly event: SnowbridgeVerificationPrimitivesEventProof;
        } & Struct;
        readonly isSetOperatingMode: boolean;
        readonly asSetOperatingMode: {
            readonly mode: SnowbridgeCoreOperatingModeBasicOperatingMode;
        } & Struct;
        readonly type: "Submit" | "SetOperatingMode";
    }

    /** @name SnowbridgeVerificationPrimitivesEventProof (208) */
    interface SnowbridgeVerificationPrimitivesEventProof extends Struct {
        readonly eventLog: SnowbridgeVerificationPrimitivesLog;
        readonly proof: SnowbridgeVerificationPrimitivesProof;
    }

    /** @name SnowbridgeVerificationPrimitivesLog (209) */
    interface SnowbridgeVerificationPrimitivesLog extends Struct {
        readonly address: H160;
        readonly topics: Vec<H256>;
        readonly data: Bytes;
    }

    /** @name SnowbridgeVerificationPrimitivesProof (211) */
    interface SnowbridgeVerificationPrimitivesProof extends Struct {
        readonly receiptProof: ITuple<[Vec<Bytes>, Vec<Bytes>]>;
        readonly executionProof: SnowbridgeBeaconPrimitivesExecutionProof;
    }

    /** @name SnowbridgeBeaconPrimitivesExecutionProof (213) */
    interface SnowbridgeBeaconPrimitivesExecutionProof extends Struct {
        readonly header: SnowbridgeBeaconPrimitivesBeaconHeader;
        readonly ancestryProof: Option<SnowbridgeBeaconPrimitivesAncestryProof>;
        readonly executionHeader: SnowbridgeBeaconPrimitivesVersionedExecutionPayloadHeader;
        readonly executionBranch: Vec<H256>;
    }

    /** @name SnowbridgeBeaconPrimitivesBeaconHeader (214) */
    interface SnowbridgeBeaconPrimitivesBeaconHeader extends Struct {
        readonly slot: u64;
        readonly proposerIndex: u64;
        readonly parentRoot: H256;
        readonly stateRoot: H256;
        readonly bodyRoot: H256;
    }

    /** @name SnowbridgeBeaconPrimitivesAncestryProof (216) */
    interface SnowbridgeBeaconPrimitivesAncestryProof extends Struct {
        readonly headerBranch: Vec<H256>;
        readonly finalizedBlockRoot: H256;
    }

    /** @name SnowbridgeBeaconPrimitivesVersionedExecutionPayloadHeader (217) */
    interface SnowbridgeBeaconPrimitivesVersionedExecutionPayloadHeader extends Enum {
        readonly isCapella: boolean;
        readonly asCapella: SnowbridgeBeaconPrimitivesExecutionPayloadHeader;
        readonly isDeneb: boolean;
        readonly asDeneb: SnowbridgeBeaconPrimitivesDenebExecutionPayloadHeader;
        readonly type: "Capella" | "Deneb";
    }

    /** @name SnowbridgeBeaconPrimitivesExecutionPayloadHeader (218) */
    interface SnowbridgeBeaconPrimitivesExecutionPayloadHeader extends Struct {
        readonly parentHash: H256;
        readonly feeRecipient: H160;
        readonly stateRoot: H256;
        readonly receiptsRoot: H256;
        readonly logsBloom: Bytes;
        readonly prevRandao: H256;
        readonly blockNumber: u64;
        readonly gasLimit: u64;
        readonly gasUsed: u64;
        readonly timestamp: u64;
        readonly extraData: Bytes;
        readonly baseFeePerGas: U256;
        readonly blockHash: H256;
        readonly transactionsRoot: H256;
        readonly withdrawalsRoot: H256;
    }

    /** @name SnowbridgeBeaconPrimitivesDenebExecutionPayloadHeader (219) */
    interface SnowbridgeBeaconPrimitivesDenebExecutionPayloadHeader extends Struct {
        readonly parentHash: H256;
        readonly feeRecipient: H160;
        readonly stateRoot: H256;
        readonly receiptsRoot: H256;
        readonly logsBloom: Bytes;
        readonly prevRandao: H256;
        readonly blockNumber: u64;
        readonly gasLimit: u64;
        readonly gasUsed: u64;
        readonly timestamp: u64;
        readonly extraData: Bytes;
        readonly baseFeePerGas: U256;
        readonly blockHash: H256;
        readonly transactionsRoot: H256;
        readonly withdrawalsRoot: H256;
        readonly blobGasUsed: u64;
        readonly excessBlobGas: u64;
    }

    /** @name SnowbridgePalletSystemCall (220) */
    interface SnowbridgePalletSystemCall extends Enum {
        readonly isUpgrade: boolean;
        readonly asUpgrade: {
            readonly implAddress: H160;
            readonly implCodeHash: H256;
            readonly initializer: Option<SnowbridgeOutboundQueuePrimitivesV1MessageInitializer>;
        } & Struct;
        readonly isSetOperatingMode: boolean;
        readonly asSetOperatingMode: {
            readonly mode: SnowbridgeOutboundQueuePrimitivesOperatingMode;
        } & Struct;
        readonly isSetPricingParameters: boolean;
        readonly asSetPricingParameters: {
            readonly params: SnowbridgeCorePricingPricingParameters;
        } & Struct;
        readonly isForceUpdateChannel: boolean;
        readonly asForceUpdateChannel: {
            readonly channelId: SnowbridgeCoreChannelId;
            readonly mode: SnowbridgeOutboundQueuePrimitivesOperatingMode;
        } & Struct;
        readonly isForceTransferNativeFromAgent: boolean;
        readonly asForceTransferNativeFromAgent: {
            readonly location: XcmVersionedLocation;
            readonly recipient: H160;
            readonly amount: u128;
        } & Struct;
        readonly isSetTokenTransferFees: boolean;
        readonly asSetTokenTransferFees: {
            readonly createAssetXcm: u128;
            readonly transferAssetXcm: u128;
            readonly registerToken: U256;
        } & Struct;
        readonly isRegisterToken: boolean;
        readonly asRegisterToken: {
            readonly location: XcmVersionedLocation;
            readonly metadata: SnowbridgeCoreAssetMetadata;
        } & Struct;
        readonly type:
            | "Upgrade"
            | "SetOperatingMode"
            | "SetPricingParameters"
            | "ForceUpdateChannel"
            | "ForceTransferNativeFromAgent"
            | "SetTokenTransferFees"
            | "RegisterToken";
    }

    /** @name SnowbridgeOutboundQueuePrimitivesV1MessageInitializer (222) */
    interface SnowbridgeOutboundQueuePrimitivesV1MessageInitializer extends Struct {
        readonly params: Bytes;
        readonly maximumRequiredGas: u64;
    }

    /** @name SnowbridgeCoreAssetMetadata (223) */
    interface SnowbridgeCoreAssetMetadata extends Struct {
        readonly name: Bytes;
        readonly symbol: Bytes;
        readonly decimals: u8;
    }

    /** @name PalletEthereumTokenTransfersCall (225) */
    interface PalletEthereumTokenTransfersCall extends Enum {
        readonly isSetTokenTransferChannel: boolean;
        readonly asSetTokenTransferChannel: {
            readonly channelId: SnowbridgeCoreChannelId;
            readonly agentId: H256;
            readonly paraId: u32;
        } & Struct;
        readonly isTransferNativeToken: boolean;
        readonly asTransferNativeToken: {
            readonly amount: u128;
            readonly recipient: H160;
        } & Struct;
        readonly type: "SetTokenTransferChannel" | "TransferNativeToken";
    }

    /** @name PalletSessionCall (226) */
    interface PalletSessionCall extends Enum {
        readonly isSetKeys: boolean;
        readonly asSetKeys: {
            readonly keys_: DancelightRuntimeSessionKeys;
            readonly proof: Bytes;
        } & Struct;
        readonly isPurgeKeys: boolean;
        readonly type: "SetKeys" | "PurgeKeys";
    }

    /** @name DancelightRuntimeSessionKeys (227) */
    interface DancelightRuntimeSessionKeys extends Struct {
        readonly grandpa: SpConsensusGrandpaAppPublic;
        readonly babe: SpConsensusBabeAppPublic;
        readonly paraValidator: PolkadotPrimitivesV8ValidatorAppPublic;
        readonly paraAssignment: PolkadotPrimitivesV8AssignmentAppPublic;
        readonly authorityDiscovery: SpAuthorityDiscoveryAppPublic;
        readonly beefy: SpConsensusBeefyEcdsaCryptoPublic;
        readonly nimbus: NimbusPrimitivesNimbusCryptoPublic;
    }

    /** @name PolkadotPrimitivesV8ValidatorAppPublic (228) */
    interface PolkadotPrimitivesV8ValidatorAppPublic extends U8aFixed {}

    /** @name PolkadotPrimitivesV8AssignmentAppPublic (229) */
    interface PolkadotPrimitivesV8AssignmentAppPublic extends U8aFixed {}

    /** @name SpAuthorityDiscoveryAppPublic (230) */
    interface SpAuthorityDiscoveryAppPublic extends U8aFixed {}

    /** @name SpConsensusBeefyEcdsaCryptoPublic (231) */
    interface SpConsensusBeefyEcdsaCryptoPublic extends U8aFixed {}

    /** @name NimbusPrimitivesNimbusCryptoPublic (233) */
    interface NimbusPrimitivesNimbusCryptoPublic extends U8aFixed {}

    /** @name PalletGrandpaCall (234) */
    interface PalletGrandpaCall extends Enum {
        readonly isReportEquivocation: boolean;
        readonly asReportEquivocation: {
            readonly equivocationProof: SpConsensusGrandpaEquivocationProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly isReportEquivocationUnsigned: boolean;
        readonly asReportEquivocationUnsigned: {
            readonly equivocationProof: SpConsensusGrandpaEquivocationProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly isNoteStalled: boolean;
        readonly asNoteStalled: {
            readonly delay: u32;
            readonly bestFinalizedBlockNumber: u32;
        } & Struct;
        readonly type: "ReportEquivocation" | "ReportEquivocationUnsigned" | "NoteStalled";
    }

    /** @name SpConsensusGrandpaEquivocationProof (235) */
    interface SpConsensusGrandpaEquivocationProof extends Struct {
        readonly setId: u64;
        readonly equivocation: SpConsensusGrandpaEquivocation;
    }

    /** @name SpConsensusGrandpaEquivocation (236) */
    interface SpConsensusGrandpaEquivocation extends Enum {
        readonly isPrevote: boolean;
        readonly asPrevote: FinalityGrandpaEquivocationPrevote;
        readonly isPrecommit: boolean;
        readonly asPrecommit: FinalityGrandpaEquivocationPrecommit;
        readonly type: "Prevote" | "Precommit";
    }

    /** @name FinalityGrandpaEquivocationPrevote (237) */
    interface FinalityGrandpaEquivocationPrevote extends Struct {
        readonly roundNumber: u64;
        readonly identity: SpConsensusGrandpaAppPublic;
        readonly first: ITuple<[FinalityGrandpaPrevote, SpConsensusGrandpaAppSignature]>;
        readonly second: ITuple<[FinalityGrandpaPrevote, SpConsensusGrandpaAppSignature]>;
    }

    /** @name FinalityGrandpaPrevote (238) */
    interface FinalityGrandpaPrevote extends Struct {
        readonly targetHash: H256;
        readonly targetNumber: u32;
    }

    /** @name SpConsensusGrandpaAppSignature (239) */
    interface SpConsensusGrandpaAppSignature extends U8aFixed {}

    /** @name FinalityGrandpaEquivocationPrecommit (241) */
    interface FinalityGrandpaEquivocationPrecommit extends Struct {
        readonly roundNumber: u64;
        readonly identity: SpConsensusGrandpaAppPublic;
        readonly first: ITuple<[FinalityGrandpaPrecommit, SpConsensusGrandpaAppSignature]>;
        readonly second: ITuple<[FinalityGrandpaPrecommit, SpConsensusGrandpaAppSignature]>;
    }

    /** @name FinalityGrandpaPrecommit (242) */
    interface FinalityGrandpaPrecommit extends Struct {
        readonly targetHash: H256;
        readonly targetNumber: u32;
    }

    /** @name PalletPooledStakingCall (244) */
    interface PalletPooledStakingCall extends Enum {
        readonly isRebalanceHold: boolean;
        readonly asRebalanceHold: {
            readonly candidate: AccountId32;
            readonly delegator: AccountId32;
            readonly pool: PalletPooledStakingPoolsPoolKind;
        } & Struct;
        readonly isRequestDelegate: boolean;
        readonly asRequestDelegate: {
            readonly candidate: AccountId32;
            readonly pool: PalletPooledStakingPoolsActivePoolKind;
            readonly stake: u128;
        } & Struct;
        readonly isExecutePendingOperations: boolean;
        readonly asExecutePendingOperations: {
            readonly operations: Vec<PalletPooledStakingPendingOperationQuery>;
        } & Struct;
        readonly isRequestUndelegate: boolean;
        readonly asRequestUndelegate: {
            readonly candidate: AccountId32;
            readonly pool: PalletPooledStakingPoolsActivePoolKind;
            readonly amount: PalletPooledStakingSharesOrStake;
        } & Struct;
        readonly isClaimManualRewards: boolean;
        readonly asClaimManualRewards: {
            readonly pairs: Vec<ITuple<[AccountId32, AccountId32]>>;
        } & Struct;
        readonly isUpdateCandidatePosition: boolean;
        readonly asUpdateCandidatePosition: {
            readonly candidates: Vec<AccountId32>;
        } & Struct;
        readonly isSwapPool: boolean;
        readonly asSwapPool: {
            readonly candidate: AccountId32;
            readonly sourcePool: PalletPooledStakingPoolsActivePoolKind;
            readonly amount: PalletPooledStakingSharesOrStake;
        } & Struct;
        readonly type:
            | "RebalanceHold"
            | "RequestDelegate"
            | "ExecutePendingOperations"
            | "RequestUndelegate"
            | "ClaimManualRewards"
            | "UpdateCandidatePosition"
            | "SwapPool";
    }

    /** @name PalletPooledStakingPoolsPoolKind (245) */
    interface PalletPooledStakingPoolsPoolKind extends Enum {
        readonly isJoining: boolean;
        readonly isAutoCompounding: boolean;
        readonly isManualRewards: boolean;
        readonly isLeaving: boolean;
        readonly type: "Joining" | "AutoCompounding" | "ManualRewards" | "Leaving";
    }

    /** @name PalletPooledStakingPendingOperationQuery (247) */
    interface PalletPooledStakingPendingOperationQuery extends Struct {
        readonly delegator: AccountId32;
        readonly operation: PalletPooledStakingPendingOperationKey;
    }

    /** @name PalletPooledStakingPendingOperationKey (248) */
    interface PalletPooledStakingPendingOperationKey extends Enum {
        readonly isJoiningAutoCompounding: boolean;
        readonly asJoiningAutoCompounding: {
            readonly candidate: AccountId32;
            readonly at: u32;
        } & Struct;
        readonly isJoiningManualRewards: boolean;
        readonly asJoiningManualRewards: {
            readonly candidate: AccountId32;
            readonly at: u32;
        } & Struct;
        readonly isLeaving: boolean;
        readonly asLeaving: {
            readonly candidate: AccountId32;
            readonly at: u32;
        } & Struct;
        readonly type: "JoiningAutoCompounding" | "JoiningManualRewards" | "Leaving";
    }

    /** @name PalletPooledStakingSharesOrStake (249) */
    interface PalletPooledStakingSharesOrStake extends Enum {
        readonly isShares: boolean;
        readonly asShares: u128;
        readonly isStake: boolean;
        readonly asStake: u128;
        readonly type: "Shares" | "Stake";
    }

    /** @name PalletInactivityTrackingCall (252) */
    interface PalletInactivityTrackingCall extends Enum {
        readonly isSetInactivityTrackingStatus: boolean;
        readonly asSetInactivityTrackingStatus: {
            readonly enableInactivityTracking: bool;
        } & Struct;
        readonly isEnableOfflineMarking: boolean;
        readonly asEnableOfflineMarking: {
            readonly value: bool;
        } & Struct;
        readonly isSetOffline: boolean;
        readonly isSetOnline: boolean;
        readonly isNotifyInactiveCollator: boolean;
        readonly asNotifyInactiveCollator: {
            readonly collator: AccountId32;
        } & Struct;
        readonly type:
            | "SetInactivityTrackingStatus"
            | "EnableOfflineMarking"
            | "SetOffline"
            | "SetOnline"
            | "NotifyInactiveCollator";
    }

    /** @name PalletTreasuryCall (253) */
    interface PalletTreasuryCall extends Enum {
        readonly isSpendLocal: boolean;
        readonly asSpendLocal: {
            readonly amount: Compact<u128>;
            readonly beneficiary: MultiAddress;
        } & Struct;
        readonly isRemoveApproval: boolean;
        readonly asRemoveApproval: {
            readonly proposalId: Compact<u32>;
        } & Struct;
        readonly isSpend: boolean;
        readonly asSpend: {
            readonly assetKind: Null;
            readonly amount: Compact<u128>;
            readonly beneficiary: AccountId32;
            readonly validFrom: Option<u32>;
        } & Struct;
        readonly isPayout: boolean;
        readonly asPayout: {
            readonly index: u32;
        } & Struct;
        readonly isCheckStatus: boolean;
        readonly asCheckStatus: {
            readonly index: u32;
        } & Struct;
        readonly isVoidSpend: boolean;
        readonly asVoidSpend: {
            readonly index: u32;
        } & Struct;
        readonly type: "SpendLocal" | "RemoveApproval" | "Spend" | "Payout" | "CheckStatus" | "VoidSpend";
    }

    /** @name PalletConvictionVotingCall (254) */
    interface PalletConvictionVotingCall extends Enum {
        readonly isVote: boolean;
        readonly asVote: {
            readonly pollIndex: Compact<u32>;
            readonly vote: PalletConvictionVotingVoteAccountVote;
        } & Struct;
        readonly isDelegate: boolean;
        readonly asDelegate: {
            readonly class: u16;
            readonly to: MultiAddress;
            readonly conviction: PalletConvictionVotingConviction;
            readonly balance: u128;
        } & Struct;
        readonly isUndelegate: boolean;
        readonly asUndelegate: {
            readonly class: u16;
        } & Struct;
        readonly isUnlock: boolean;
        readonly asUnlock: {
            readonly class: u16;
            readonly target: MultiAddress;
        } & Struct;
        readonly isRemoveVote: boolean;
        readonly asRemoveVote: {
            readonly class: Option<u16>;
            readonly index: u32;
        } & Struct;
        readonly isRemoveOtherVote: boolean;
        readonly asRemoveOtherVote: {
            readonly target: MultiAddress;
            readonly class: u16;
            readonly index: u32;
        } & Struct;
        readonly type: "Vote" | "Delegate" | "Undelegate" | "Unlock" | "RemoveVote" | "RemoveOtherVote";
    }

    /** @name PalletConvictionVotingConviction (255) */
    interface PalletConvictionVotingConviction extends Enum {
        readonly isNone: boolean;
        readonly isLocked1x: boolean;
        readonly isLocked2x: boolean;
        readonly isLocked3x: boolean;
        readonly isLocked4x: boolean;
        readonly isLocked5x: boolean;
        readonly isLocked6x: boolean;
        readonly type: "None" | "Locked1x" | "Locked2x" | "Locked3x" | "Locked4x" | "Locked5x" | "Locked6x";
    }

    /** @name PalletReferendaCall (257) */
    interface PalletReferendaCall extends Enum {
        readonly isSubmit: boolean;
        readonly asSubmit: {
            readonly proposalOrigin: DancelightRuntimeOriginCaller;
            readonly proposal: FrameSupportPreimagesBounded;
            readonly enactmentMoment: FrameSupportScheduleDispatchTime;
        } & Struct;
        readonly isPlaceDecisionDeposit: boolean;
        readonly asPlaceDecisionDeposit: {
            readonly index: u32;
        } & Struct;
        readonly isRefundDecisionDeposit: boolean;
        readonly asRefundDecisionDeposit: {
            readonly index: u32;
        } & Struct;
        readonly isCancel: boolean;
        readonly asCancel: {
            readonly index: u32;
        } & Struct;
        readonly isKill: boolean;
        readonly asKill: {
            readonly index: u32;
        } & Struct;
        readonly isNudgeReferendum: boolean;
        readonly asNudgeReferendum: {
            readonly index: u32;
        } & Struct;
        readonly isOneFewerDeciding: boolean;
        readonly asOneFewerDeciding: {
            readonly track: u16;
        } & Struct;
        readonly isRefundSubmissionDeposit: boolean;
        readonly asRefundSubmissionDeposit: {
            readonly index: u32;
        } & Struct;
        readonly isSetMetadata: boolean;
        readonly asSetMetadata: {
            readonly index: u32;
            readonly maybeHash: Option<H256>;
        } & Struct;
        readonly type:
            | "Submit"
            | "PlaceDecisionDeposit"
            | "RefundDecisionDeposit"
            | "Cancel"
            | "Kill"
            | "NudgeReferendum"
            | "OneFewerDeciding"
            | "RefundSubmissionDeposit"
            | "SetMetadata";
    }

    /** @name DancelightRuntimeOriginCaller (258) */
    interface DancelightRuntimeOriginCaller extends Enum {
        readonly isSystem: boolean;
        readonly asSystem: FrameSupportDispatchRawOrigin;
        readonly isOrigins: boolean;
        readonly asOrigins: DancelightRuntimeGovernanceOriginsPalletCustomOriginsOrigin;
        readonly isOpenTechCommitteeCollective: boolean;
        readonly asOpenTechCommitteeCollective: PalletCollectiveRawOrigin;
        readonly isParachainsOrigin: boolean;
        readonly asParachainsOrigin: PolkadotRuntimeParachainsOriginPalletOrigin;
        readonly isXcmPallet: boolean;
        readonly asXcmPallet: PalletXcmOrigin;
        readonly type: "System" | "Origins" | "OpenTechCommitteeCollective" | "ParachainsOrigin" | "XcmPallet";
    }

    /** @name FrameSupportDispatchRawOrigin (259) */
    interface FrameSupportDispatchRawOrigin extends Enum {
        readonly isRoot: boolean;
        readonly isSigned: boolean;
        readonly asSigned: AccountId32;
        readonly isNone: boolean;
        readonly isAuthorized: boolean;
        readonly type: "Root" | "Signed" | "None" | "Authorized";
    }

    /** @name DancelightRuntimeGovernanceOriginsPalletCustomOriginsOrigin (260) */
    interface DancelightRuntimeGovernanceOriginsPalletCustomOriginsOrigin extends Enum {
        readonly isStakingAdmin: boolean;
        readonly isTreasurer: boolean;
        readonly isFellowshipAdmin: boolean;
        readonly isGeneralAdmin: boolean;
        readonly isAuctionAdmin: boolean;
        readonly isLeaseAdmin: boolean;
        readonly isReferendumCanceller: boolean;
        readonly isReferendumKiller: boolean;
        readonly isSmallTipper: boolean;
        readonly isBigTipper: boolean;
        readonly isSmallSpender: boolean;
        readonly isMediumSpender: boolean;
        readonly isBigSpender: boolean;
        readonly isWhitelistedCaller: boolean;
        readonly isFellowshipInitiates: boolean;
        readonly isFellows: boolean;
        readonly isFellowshipExperts: boolean;
        readonly isFellowshipMasters: boolean;
        readonly isFellowship1Dan: boolean;
        readonly isFellowship2Dan: boolean;
        readonly isFellowship3Dan: boolean;
        readonly isFellowship4Dan: boolean;
        readonly isFellowship5Dan: boolean;
        readonly isFellowship6Dan: boolean;
        readonly isFellowship7Dan: boolean;
        readonly isFellowship8Dan: boolean;
        readonly isFellowship9Dan: boolean;
        readonly type:
            | "StakingAdmin"
            | "Treasurer"
            | "FellowshipAdmin"
            | "GeneralAdmin"
            | "AuctionAdmin"
            | "LeaseAdmin"
            | "ReferendumCanceller"
            | "ReferendumKiller"
            | "SmallTipper"
            | "BigTipper"
            | "SmallSpender"
            | "MediumSpender"
            | "BigSpender"
            | "WhitelistedCaller"
            | "FellowshipInitiates"
            | "Fellows"
            | "FellowshipExperts"
            | "FellowshipMasters"
            | "Fellowship1Dan"
            | "Fellowship2Dan"
            | "Fellowship3Dan"
            | "Fellowship4Dan"
            | "Fellowship5Dan"
            | "Fellowship6Dan"
            | "Fellowship7Dan"
            | "Fellowship8Dan"
            | "Fellowship9Dan";
    }

    /** @name PalletCollectiveRawOrigin (261) */
    interface PalletCollectiveRawOrigin extends Enum {
        readonly isMembers: boolean;
        readonly asMembers: ITuple<[u32, u32]>;
        readonly isMember: boolean;
        readonly asMember: AccountId32;
        readonly isPhantom: boolean;
        readonly type: "Members" | "Member" | "Phantom";
    }

    /** @name PolkadotRuntimeParachainsOriginPalletOrigin (262) */
    interface PolkadotRuntimeParachainsOriginPalletOrigin extends Enum {
        readonly isParachain: boolean;
        readonly asParachain: u32;
        readonly type: "Parachain";
    }

    /** @name PalletXcmOrigin (263) */
    interface PalletXcmOrigin extends Enum {
        readonly isXcm: boolean;
        readonly asXcm: StagingXcmV5Location;
        readonly isResponse: boolean;
        readonly asResponse: StagingXcmV5Location;
        readonly type: "Xcm" | "Response";
    }

    /** @name FrameSupportScheduleDispatchTime (264) */
    interface FrameSupportScheduleDispatchTime extends Enum {
        readonly isAt: boolean;
        readonly asAt: u32;
        readonly isAfter: boolean;
        readonly asAfter: u32;
        readonly type: "At" | "After";
    }

    /** @name PalletRankedCollectiveCall (265) */
    interface PalletRankedCollectiveCall extends Enum {
        readonly isAddMember: boolean;
        readonly asAddMember: {
            readonly who: MultiAddress;
        } & Struct;
        readonly isPromoteMember: boolean;
        readonly asPromoteMember: {
            readonly who: MultiAddress;
        } & Struct;
        readonly isDemoteMember: boolean;
        readonly asDemoteMember: {
            readonly who: MultiAddress;
        } & Struct;
        readonly isRemoveMember: boolean;
        readonly asRemoveMember: {
            readonly who: MultiAddress;
            readonly minRank: u16;
        } & Struct;
        readonly isVote: boolean;
        readonly asVote: {
            readonly poll: u32;
            readonly aye: bool;
        } & Struct;
        readonly isCleanupPoll: boolean;
        readonly asCleanupPoll: {
            readonly pollIndex: u32;
            readonly max: u32;
        } & Struct;
        readonly isExchangeMember: boolean;
        readonly asExchangeMember: {
            readonly who: MultiAddress;
            readonly newWho: MultiAddress;
        } & Struct;
        readonly type:
            | "AddMember"
            | "PromoteMember"
            | "DemoteMember"
            | "RemoveMember"
            | "Vote"
            | "CleanupPoll"
            | "ExchangeMember";
    }

    /** @name PalletWhitelistCall (267) */
    interface PalletWhitelistCall extends Enum {
        readonly isWhitelistCall: boolean;
        readonly asWhitelistCall: {
            readonly callHash: H256;
        } & Struct;
        readonly isRemoveWhitelistedCall: boolean;
        readonly asRemoveWhitelistedCall: {
            readonly callHash: H256;
        } & Struct;
        readonly isDispatchWhitelistedCall: boolean;
        readonly asDispatchWhitelistedCall: {
            readonly callHash: H256;
            readonly callEncodedLen: u32;
            readonly callWeightWitness: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isDispatchWhitelistedCallWithPreimage: boolean;
        readonly asDispatchWhitelistedCallWithPreimage: {
            readonly call: Call;
        } & Struct;
        readonly type:
            | "WhitelistCall"
            | "RemoveWhitelistedCall"
            | "DispatchWhitelistedCall"
            | "DispatchWhitelistedCallWithPreimage";
    }

    /** @name PalletCollectiveCall (268) */
    interface PalletCollectiveCall extends Enum {
        readonly isSetMembers: boolean;
        readonly asSetMembers: {
            readonly newMembers: Vec<AccountId32>;
            readonly prime: Option<AccountId32>;
            readonly oldCount: u32;
        } & Struct;
        readonly isExecute: boolean;
        readonly asExecute: {
            readonly proposal: Call;
            readonly lengthBound: Compact<u32>;
        } & Struct;
        readonly isPropose: boolean;
        readonly asPropose: {
            readonly threshold: Compact<u32>;
            readonly proposal: Call;
            readonly lengthBound: Compact<u32>;
        } & Struct;
        readonly isVote: boolean;
        readonly asVote: {
            readonly proposal: H256;
            readonly index: Compact<u32>;
            readonly approve: bool;
        } & Struct;
        readonly isDisapproveProposal: boolean;
        readonly asDisapproveProposal: {
            readonly proposalHash: H256;
        } & Struct;
        readonly isClose: boolean;
        readonly asClose: {
            readonly proposalHash: H256;
            readonly index: Compact<u32>;
            readonly proposalWeightBound: SpWeightsWeightV2Weight;
            readonly lengthBound: Compact<u32>;
        } & Struct;
        readonly isKill: boolean;
        readonly asKill: {
            readonly proposalHash: H256;
        } & Struct;
        readonly isReleaseProposalCost: boolean;
        readonly asReleaseProposalCost: {
            readonly proposalHash: H256;
        } & Struct;
        readonly type:
            | "SetMembers"
            | "Execute"
            | "Propose"
            | "Vote"
            | "DisapproveProposal"
            | "Close"
            | "Kill"
            | "ReleaseProposalCost";
    }

    /** @name PolkadotRuntimeParachainsConfigurationPalletCall (269) */
    interface PolkadotRuntimeParachainsConfigurationPalletCall extends Enum {
        readonly isSetValidationUpgradeCooldown: boolean;
        readonly asSetValidationUpgradeCooldown: {
            readonly new_: u32;
        } & Struct;
        readonly isSetValidationUpgradeDelay: boolean;
        readonly asSetValidationUpgradeDelay: {
            readonly new_: u32;
        } & Struct;
        readonly isSetCodeRetentionPeriod: boolean;
        readonly asSetCodeRetentionPeriod: {
            readonly new_: u32;
        } & Struct;
        readonly isSetMaxCodeSize: boolean;
        readonly asSetMaxCodeSize: {
            readonly new_: u32;
        } & Struct;
        readonly isSetMaxPovSize: boolean;
        readonly asSetMaxPovSize: {
            readonly new_: u32;
        } & Struct;
        readonly isSetMaxHeadDataSize: boolean;
        readonly asSetMaxHeadDataSize: {
            readonly new_: u32;
        } & Struct;
        readonly isSetCoretimeCores: boolean;
        readonly asSetCoretimeCores: {
            readonly new_: u32;
        } & Struct;
        readonly isSetGroupRotationFrequency: boolean;
        readonly asSetGroupRotationFrequency: {
            readonly new_: u32;
        } & Struct;
        readonly isSetParasAvailabilityPeriod: boolean;
        readonly asSetParasAvailabilityPeriod: {
            readonly new_: u32;
        } & Struct;
        readonly isSetSchedulingLookahead: boolean;
        readonly asSetSchedulingLookahead: {
            readonly new_: u32;
        } & Struct;
        readonly isSetMaxValidatorsPerCore: boolean;
        readonly asSetMaxValidatorsPerCore: {
            readonly new_: Option<u32>;
        } & Struct;
        readonly isSetMaxValidators: boolean;
        readonly asSetMaxValidators: {
            readonly new_: Option<u32>;
        } & Struct;
        readonly isSetDisputePeriod: boolean;
        readonly asSetDisputePeriod: {
            readonly new_: u32;
        } & Struct;
        readonly isSetDisputePostConclusionAcceptancePeriod: boolean;
        readonly asSetDisputePostConclusionAcceptancePeriod: {
            readonly new_: u32;
        } & Struct;
        readonly isSetNoShowSlots: boolean;
        readonly asSetNoShowSlots: {
            readonly new_: u32;
        } & Struct;
        readonly isSetNDelayTranches: boolean;
        readonly asSetNDelayTranches: {
            readonly new_: u32;
        } & Struct;
        readonly isSetZerothDelayTrancheWidth: boolean;
        readonly asSetZerothDelayTrancheWidth: {
            readonly new_: u32;
        } & Struct;
        readonly isSetNeededApprovals: boolean;
        readonly asSetNeededApprovals: {
            readonly new_: u32;
        } & Struct;
        readonly isSetRelayVrfModuloSamples: boolean;
        readonly asSetRelayVrfModuloSamples: {
            readonly new_: u32;
        } & Struct;
        readonly isSetMaxUpwardQueueCount: boolean;
        readonly asSetMaxUpwardQueueCount: {
            readonly new_: u32;
        } & Struct;
        readonly isSetMaxUpwardQueueSize: boolean;
        readonly asSetMaxUpwardQueueSize: {
            readonly new_: u32;
        } & Struct;
        readonly isSetMaxDownwardMessageSize: boolean;
        readonly asSetMaxDownwardMessageSize: {
            readonly new_: u32;
        } & Struct;
        readonly isSetMaxUpwardMessageSize: boolean;
        readonly asSetMaxUpwardMessageSize: {
            readonly new_: u32;
        } & Struct;
        readonly isSetMaxUpwardMessageNumPerCandidate: boolean;
        readonly asSetMaxUpwardMessageNumPerCandidate: {
            readonly new_: u32;
        } & Struct;
        readonly isSetHrmpOpenRequestTtl: boolean;
        readonly asSetHrmpOpenRequestTtl: {
            readonly new_: u32;
        } & Struct;
        readonly isSetHrmpSenderDeposit: boolean;
        readonly asSetHrmpSenderDeposit: {
            readonly new_: u128;
        } & Struct;
        readonly isSetHrmpRecipientDeposit: boolean;
        readonly asSetHrmpRecipientDeposit: {
            readonly new_: u128;
        } & Struct;
        readonly isSetHrmpChannelMaxCapacity: boolean;
        readonly asSetHrmpChannelMaxCapacity: {
            readonly new_: u32;
        } & Struct;
        readonly isSetHrmpChannelMaxTotalSize: boolean;
        readonly asSetHrmpChannelMaxTotalSize: {
            readonly new_: u32;
        } & Struct;
        readonly isSetHrmpMaxParachainInboundChannels: boolean;
        readonly asSetHrmpMaxParachainInboundChannels: {
            readonly new_: u32;
        } & Struct;
        readonly isSetHrmpChannelMaxMessageSize: boolean;
        readonly asSetHrmpChannelMaxMessageSize: {
            readonly new_: u32;
        } & Struct;
        readonly isSetHrmpMaxParachainOutboundChannels: boolean;
        readonly asSetHrmpMaxParachainOutboundChannels: {
            readonly new_: u32;
        } & Struct;
        readonly isSetHrmpMaxMessageNumPerCandidate: boolean;
        readonly asSetHrmpMaxMessageNumPerCandidate: {
            readonly new_: u32;
        } & Struct;
        readonly isSetPvfVotingTtl: boolean;
        readonly asSetPvfVotingTtl: {
            readonly new_: u32;
        } & Struct;
        readonly isSetMinimumValidationUpgradeDelay: boolean;
        readonly asSetMinimumValidationUpgradeDelay: {
            readonly new_: u32;
        } & Struct;
        readonly isSetBypassConsistencyCheck: boolean;
        readonly asSetBypassConsistencyCheck: {
            readonly new_: bool;
        } & Struct;
        readonly isSetAsyncBackingParams: boolean;
        readonly asSetAsyncBackingParams: {
            readonly new_: PolkadotPrimitivesV8AsyncBackingAsyncBackingParams;
        } & Struct;
        readonly isSetExecutorParams: boolean;
        readonly asSetExecutorParams: {
            readonly new_: PolkadotPrimitivesV8ExecutorParams;
        } & Struct;
        readonly isSetOnDemandBaseFee: boolean;
        readonly asSetOnDemandBaseFee: {
            readonly new_: u128;
        } & Struct;
        readonly isSetOnDemandFeeVariability: boolean;
        readonly asSetOnDemandFeeVariability: {
            readonly new_: Perbill;
        } & Struct;
        readonly isSetOnDemandQueueMaxSize: boolean;
        readonly asSetOnDemandQueueMaxSize: {
            readonly new_: u32;
        } & Struct;
        readonly isSetOnDemandTargetQueueUtilization: boolean;
        readonly asSetOnDemandTargetQueueUtilization: {
            readonly new_: Perbill;
        } & Struct;
        readonly isSetMinimumBackingVotes: boolean;
        readonly asSetMinimumBackingVotes: {
            readonly new_: u32;
        } & Struct;
        readonly isSetNodeFeature: boolean;
        readonly asSetNodeFeature: {
            readonly index: u8;
            readonly value: bool;
        } & Struct;
        readonly isSetApprovalVotingParams: boolean;
        readonly asSetApprovalVotingParams: {
            readonly new_: PolkadotPrimitivesV8ApprovalVotingParams;
        } & Struct;
        readonly isSetSchedulerParams: boolean;
        readonly asSetSchedulerParams: {
            readonly new_: PolkadotPrimitivesV8SchedulerParams;
        } & Struct;
        readonly type:
            | "SetValidationUpgradeCooldown"
            | "SetValidationUpgradeDelay"
            | "SetCodeRetentionPeriod"
            | "SetMaxCodeSize"
            | "SetMaxPovSize"
            | "SetMaxHeadDataSize"
            | "SetCoretimeCores"
            | "SetGroupRotationFrequency"
            | "SetParasAvailabilityPeriod"
            | "SetSchedulingLookahead"
            | "SetMaxValidatorsPerCore"
            | "SetMaxValidators"
            | "SetDisputePeriod"
            | "SetDisputePostConclusionAcceptancePeriod"
            | "SetNoShowSlots"
            | "SetNDelayTranches"
            | "SetZerothDelayTrancheWidth"
            | "SetNeededApprovals"
            | "SetRelayVrfModuloSamples"
            | "SetMaxUpwardQueueCount"
            | "SetMaxUpwardQueueSize"
            | "SetMaxDownwardMessageSize"
            | "SetMaxUpwardMessageSize"
            | "SetMaxUpwardMessageNumPerCandidate"
            | "SetHrmpOpenRequestTtl"
            | "SetHrmpSenderDeposit"
            | "SetHrmpRecipientDeposit"
            | "SetHrmpChannelMaxCapacity"
            | "SetHrmpChannelMaxTotalSize"
            | "SetHrmpMaxParachainInboundChannels"
            | "SetHrmpChannelMaxMessageSize"
            | "SetHrmpMaxParachainOutboundChannels"
            | "SetHrmpMaxMessageNumPerCandidate"
            | "SetPvfVotingTtl"
            | "SetMinimumValidationUpgradeDelay"
            | "SetBypassConsistencyCheck"
            | "SetAsyncBackingParams"
            | "SetExecutorParams"
            | "SetOnDemandBaseFee"
            | "SetOnDemandFeeVariability"
            | "SetOnDemandQueueMaxSize"
            | "SetOnDemandTargetQueueUtilization"
            | "SetMinimumBackingVotes"
            | "SetNodeFeature"
            | "SetApprovalVotingParams"
            | "SetSchedulerParams";
    }

    /** @name PolkadotPrimitivesV8AsyncBackingAsyncBackingParams (270) */
    interface PolkadotPrimitivesV8AsyncBackingAsyncBackingParams extends Struct {
        readonly maxCandidateDepth: u32;
        readonly allowedAncestryLen: u32;
    }

    /** @name PolkadotPrimitivesV8ExecutorParams (271) */
    interface PolkadotPrimitivesV8ExecutorParams extends Vec<PolkadotPrimitivesV8ExecutorParamsExecutorParam> {}

    /** @name PolkadotPrimitivesV8ExecutorParamsExecutorParam (273) */
    interface PolkadotPrimitivesV8ExecutorParamsExecutorParam extends Enum {
        readonly isMaxMemoryPages: boolean;
        readonly asMaxMemoryPages: u32;
        readonly isStackLogicalMax: boolean;
        readonly asStackLogicalMax: u32;
        readonly isStackNativeMax: boolean;
        readonly asStackNativeMax: u32;
        readonly isPrecheckingMaxMemory: boolean;
        readonly asPrecheckingMaxMemory: u64;
        readonly isPvfPrepTimeout: boolean;
        readonly asPvfPrepTimeout: ITuple<[PolkadotPrimitivesV8PvfPrepKind, u64]>;
        readonly isPvfExecTimeout: boolean;
        readonly asPvfExecTimeout: ITuple<[PolkadotPrimitivesV8PvfExecKind, u64]>;
        readonly isWasmExtBulkMemory: boolean;
        readonly type:
            | "MaxMemoryPages"
            | "StackLogicalMax"
            | "StackNativeMax"
            | "PrecheckingMaxMemory"
            | "PvfPrepTimeout"
            | "PvfExecTimeout"
            | "WasmExtBulkMemory";
    }

    /** @name PolkadotPrimitivesV8PvfPrepKind (274) */
    interface PolkadotPrimitivesV8PvfPrepKind extends Enum {
        readonly isPrecheck: boolean;
        readonly isPrepare: boolean;
        readonly type: "Precheck" | "Prepare";
    }

    /** @name PolkadotPrimitivesV8PvfExecKind (275) */
    interface PolkadotPrimitivesV8PvfExecKind extends Enum {
        readonly isBacking: boolean;
        readonly isApproval: boolean;
        readonly type: "Backing" | "Approval";
    }

    /** @name PolkadotPrimitivesV8ApprovalVotingParams (276) */
    interface PolkadotPrimitivesV8ApprovalVotingParams extends Struct {
        readonly maxApprovalCoalesceCount: u32;
    }

    /** @name PolkadotPrimitivesV8SchedulerParams (277) */
    interface PolkadotPrimitivesV8SchedulerParams extends Struct {
        readonly groupRotationFrequency: u32;
        readonly parasAvailabilityPeriod: u32;
        readonly maxValidatorsPerCore: Option<u32>;
        readonly lookahead: u32;
        readonly numCores: u32;
        readonly maxAvailabilityTimeouts: u32;
        readonly onDemandQueueMaxSize: u32;
        readonly onDemandTargetQueueUtilization: Perbill;
        readonly onDemandFeeVariability: Perbill;
        readonly onDemandBaseFee: u128;
        readonly ttl: u32;
    }

    /** @name PolkadotRuntimeParachainsSharedPalletCall (278) */
    type PolkadotRuntimeParachainsSharedPalletCall = Null;

    /** @name PolkadotRuntimeParachainsInclusionPalletCall (279) */
    type PolkadotRuntimeParachainsInclusionPalletCall = Null;

    /** @name PolkadotRuntimeParachainsParasInherentPalletCall (280) */
    interface PolkadotRuntimeParachainsParasInherentPalletCall extends Enum {
        readonly isEnter: boolean;
        readonly asEnter: {
            readonly data: PolkadotPrimitivesVstagingInherentData;
        } & Struct;
        readonly type: "Enter";
    }

    /** @name PolkadotPrimitivesVstagingInherentData (281) */
    interface PolkadotPrimitivesVstagingInherentData extends Struct {
        readonly bitfields: Vec<PolkadotPrimitivesV8SignedUncheckedSigned>;
        readonly backedCandidates: Vec<PolkadotPrimitivesVstagingBackedCandidate>;
        readonly disputes: Vec<PolkadotPrimitivesV8DisputeStatementSet>;
        readonly parentHeader: SpRuntimeHeader;
    }

    /** @name PolkadotPrimitivesV8SignedUncheckedSigned (283) */
    interface PolkadotPrimitivesV8SignedUncheckedSigned extends Struct {
        readonly payload: BitVec;
        readonly validatorIndex: u32;
        readonly signature: PolkadotPrimitivesV8ValidatorAppSignature;
    }

    /** @name BitvecOrderLsb0 (286) */
    type BitvecOrderLsb0 = Null;

    /** @name PolkadotPrimitivesV8ValidatorAppSignature (288) */
    interface PolkadotPrimitivesV8ValidatorAppSignature extends U8aFixed {}

    /** @name PolkadotPrimitivesVstagingBackedCandidate (290) */
    interface PolkadotPrimitivesVstagingBackedCandidate extends Struct {
        readonly candidate: PolkadotPrimitivesVstagingCommittedCandidateReceiptV2;
        readonly validityVotes: Vec<PolkadotPrimitivesV8ValidityAttestation>;
        readonly validatorIndices: BitVec;
    }

    /** @name PolkadotPrimitivesVstagingCommittedCandidateReceiptV2 (291) */
    interface PolkadotPrimitivesVstagingCommittedCandidateReceiptV2 extends Struct {
        readonly descriptor: PolkadotPrimitivesVstagingCandidateDescriptorV2;
        readonly commitments: PolkadotPrimitivesV8CandidateCommitments;
    }

    /** @name PolkadotPrimitivesVstagingCandidateDescriptorV2 (292) */
    interface PolkadotPrimitivesVstagingCandidateDescriptorV2 extends Struct {
        readonly paraId: u32;
        readonly relayParent: H256;
        readonly version: u8;
        readonly coreIndex: u16;
        readonly sessionIndex: u32;
        readonly reserved1: U8aFixed;
        readonly persistedValidationDataHash: H256;
        readonly povHash: H256;
        readonly erasureRoot: H256;
        readonly reserved2: U8aFixed;
        readonly paraHead: H256;
        readonly validationCodeHash: H256;
    }

    /** @name PolkadotPrimitivesV8CandidateCommitments (296) */
    interface PolkadotPrimitivesV8CandidateCommitments extends Struct {
        readonly upwardMessages: Vec<Bytes>;
        readonly horizontalMessages: Vec<PolkadotCorePrimitivesOutboundHrmpMessage>;
        readonly newValidationCode: Option<Bytes>;
        readonly headData: Bytes;
        readonly processedDownwardMessages: u32;
        readonly hrmpWatermark: u32;
    }

    /** @name PolkadotCorePrimitivesOutboundHrmpMessage (299) */
    interface PolkadotCorePrimitivesOutboundHrmpMessage extends Struct {
        readonly recipient: u32;
        readonly data: Bytes;
    }

    /** @name PolkadotPrimitivesV8ValidityAttestation (304) */
    interface PolkadotPrimitivesV8ValidityAttestation extends Enum {
        readonly isImplicit: boolean;
        readonly asImplicit: PolkadotPrimitivesV8ValidatorAppSignature;
        readonly isExplicit: boolean;
        readonly asExplicit: PolkadotPrimitivesV8ValidatorAppSignature;
        readonly type: "Implicit" | "Explicit";
    }

    /** @name PolkadotPrimitivesV8DisputeStatementSet (306) */
    interface PolkadotPrimitivesV8DisputeStatementSet extends Struct {
        readonly candidateHash: H256;
        readonly session: u32;
        readonly statements: Vec<
            ITuple<[PolkadotPrimitivesV8DisputeStatement, u32, PolkadotPrimitivesV8ValidatorAppSignature]>
        >;
    }

    /** @name PolkadotPrimitivesV8DisputeStatement (310) */
    interface PolkadotPrimitivesV8DisputeStatement extends Enum {
        readonly isValid: boolean;
        readonly asValid: PolkadotPrimitivesV8ValidDisputeStatementKind;
        readonly isInvalid: boolean;
        readonly asInvalid: PolkadotPrimitivesV8InvalidDisputeStatementKind;
        readonly type: "Valid" | "Invalid";
    }

    /** @name PolkadotPrimitivesV8ValidDisputeStatementKind (311) */
    interface PolkadotPrimitivesV8ValidDisputeStatementKind extends Enum {
        readonly isExplicit: boolean;
        readonly isBackingSeconded: boolean;
        readonly asBackingSeconded: H256;
        readonly isBackingValid: boolean;
        readonly asBackingValid: H256;
        readonly isApprovalChecking: boolean;
        readonly isApprovalCheckingMultipleCandidates: boolean;
        readonly asApprovalCheckingMultipleCandidates: Vec<H256>;
        readonly type:
            | "Explicit"
            | "BackingSeconded"
            | "BackingValid"
            | "ApprovalChecking"
            | "ApprovalCheckingMultipleCandidates";
    }

    /** @name PolkadotPrimitivesV8InvalidDisputeStatementKind (313) */
    interface PolkadotPrimitivesV8InvalidDisputeStatementKind extends Enum {
        readonly isExplicit: boolean;
        readonly type: "Explicit";
    }

    /** @name PolkadotRuntimeParachainsParasPalletCall (314) */
    interface PolkadotRuntimeParachainsParasPalletCall extends Enum {
        readonly isForceSetCurrentCode: boolean;
        readonly asForceSetCurrentCode: {
            readonly para: u32;
            readonly newCode: Bytes;
        } & Struct;
        readonly isForceSetCurrentHead: boolean;
        readonly asForceSetCurrentHead: {
            readonly para: u32;
            readonly newHead: Bytes;
        } & Struct;
        readonly isForceScheduleCodeUpgrade: boolean;
        readonly asForceScheduleCodeUpgrade: {
            readonly para: u32;
            readonly newCode: Bytes;
            readonly relayParentNumber: u32;
        } & Struct;
        readonly isForceNoteNewHead: boolean;
        readonly asForceNoteNewHead: {
            readonly para: u32;
            readonly newHead: Bytes;
        } & Struct;
        readonly isForceQueueAction: boolean;
        readonly asForceQueueAction: {
            readonly para: u32;
        } & Struct;
        readonly isAddTrustedValidationCode: boolean;
        readonly asAddTrustedValidationCode: {
            readonly validationCode: Bytes;
        } & Struct;
        readonly isPokeUnusedValidationCode: boolean;
        readonly asPokeUnusedValidationCode: {
            readonly validationCodeHash: H256;
        } & Struct;
        readonly isIncludePvfCheckStatement: boolean;
        readonly asIncludePvfCheckStatement: {
            readonly stmt: PolkadotPrimitivesV8PvfCheckStatement;
            readonly signature: PolkadotPrimitivesV8ValidatorAppSignature;
        } & Struct;
        readonly isForceSetMostRecentContext: boolean;
        readonly asForceSetMostRecentContext: {
            readonly para: u32;
            readonly context: u32;
        } & Struct;
        readonly isRemoveUpgradeCooldown: boolean;
        readonly asRemoveUpgradeCooldown: {
            readonly para: u32;
        } & Struct;
        readonly isAuthorizeForceSetCurrentCodeHash: boolean;
        readonly asAuthorizeForceSetCurrentCodeHash: {
            readonly para: u32;
            readonly newCodeHash: H256;
            readonly validPeriod: u32;
        } & Struct;
        readonly isApplyAuthorizedForceSetCurrentCode: boolean;
        readonly asApplyAuthorizedForceSetCurrentCode: {
            readonly para: u32;
            readonly newCode: Bytes;
        } & Struct;
        readonly type:
            | "ForceSetCurrentCode"
            | "ForceSetCurrentHead"
            | "ForceScheduleCodeUpgrade"
            | "ForceNoteNewHead"
            | "ForceQueueAction"
            | "AddTrustedValidationCode"
            | "PokeUnusedValidationCode"
            | "IncludePvfCheckStatement"
            | "ForceSetMostRecentContext"
            | "RemoveUpgradeCooldown"
            | "AuthorizeForceSetCurrentCodeHash"
            | "ApplyAuthorizedForceSetCurrentCode";
    }

    /** @name PolkadotPrimitivesV8PvfCheckStatement (315) */
    interface PolkadotPrimitivesV8PvfCheckStatement extends Struct {
        readonly accept: bool;
        readonly subject: H256;
        readonly sessionIndex: u32;
        readonly validatorIndex: u32;
    }

    /** @name PolkadotRuntimeParachainsInitializerPalletCall (316) */
    interface PolkadotRuntimeParachainsInitializerPalletCall extends Enum {
        readonly isForceApprove: boolean;
        readonly asForceApprove: {
            readonly upTo: u32;
        } & Struct;
        readonly type: "ForceApprove";
    }

    /** @name PolkadotRuntimeParachainsHrmpPalletCall (317) */
    interface PolkadotRuntimeParachainsHrmpPalletCall extends Enum {
        readonly isHrmpInitOpenChannel: boolean;
        readonly asHrmpInitOpenChannel: {
            readonly recipient: u32;
            readonly proposedMaxCapacity: u32;
            readonly proposedMaxMessageSize: u32;
        } & Struct;
        readonly isHrmpAcceptOpenChannel: boolean;
        readonly asHrmpAcceptOpenChannel: {
            readonly sender: u32;
        } & Struct;
        readonly isHrmpCloseChannel: boolean;
        readonly asHrmpCloseChannel: {
            readonly channelId: PolkadotParachainPrimitivesPrimitivesHrmpChannelId;
        } & Struct;
        readonly isForceCleanHrmp: boolean;
        readonly asForceCleanHrmp: {
            readonly para: u32;
            readonly numInbound: u32;
            readonly numOutbound: u32;
        } & Struct;
        readonly isForceProcessHrmpOpen: boolean;
        readonly asForceProcessHrmpOpen: {
            readonly channels: u32;
        } & Struct;
        readonly isForceProcessHrmpClose: boolean;
        readonly asForceProcessHrmpClose: {
            readonly channels: u32;
        } & Struct;
        readonly isHrmpCancelOpenRequest: boolean;
        readonly asHrmpCancelOpenRequest: {
            readonly channelId: PolkadotParachainPrimitivesPrimitivesHrmpChannelId;
            readonly openRequests: u32;
        } & Struct;
        readonly isForceOpenHrmpChannel: boolean;
        readonly asForceOpenHrmpChannel: {
            readonly sender: u32;
            readonly recipient: u32;
            readonly maxCapacity: u32;
            readonly maxMessageSize: u32;
        } & Struct;
        readonly isEstablishSystemChannel: boolean;
        readonly asEstablishSystemChannel: {
            readonly sender: u32;
            readonly recipient: u32;
        } & Struct;
        readonly isPokeChannelDeposits: boolean;
        readonly asPokeChannelDeposits: {
            readonly sender: u32;
            readonly recipient: u32;
        } & Struct;
        readonly isEstablishChannelWithSystem: boolean;
        readonly asEstablishChannelWithSystem: {
            readonly targetSystemChain: u32;
        } & Struct;
        readonly type:
            | "HrmpInitOpenChannel"
            | "HrmpAcceptOpenChannel"
            | "HrmpCloseChannel"
            | "ForceCleanHrmp"
            | "ForceProcessHrmpOpen"
            | "ForceProcessHrmpClose"
            | "HrmpCancelOpenRequest"
            | "ForceOpenHrmpChannel"
            | "EstablishSystemChannel"
            | "PokeChannelDeposits"
            | "EstablishChannelWithSystem";
    }

    /** @name PolkadotParachainPrimitivesPrimitivesHrmpChannelId (318) */
    interface PolkadotParachainPrimitivesPrimitivesHrmpChannelId extends Struct {
        readonly sender: u32;
        readonly recipient: u32;
    }

    /** @name PolkadotRuntimeParachainsDisputesPalletCall (319) */
    interface PolkadotRuntimeParachainsDisputesPalletCall extends Enum {
        readonly isForceUnfreeze: boolean;
        readonly type: "ForceUnfreeze";
    }

    /** @name PolkadotRuntimeParachainsDisputesSlashingPalletCall (320) */
    interface PolkadotRuntimeParachainsDisputesSlashingPalletCall extends Enum {
        readonly isReportDisputeLostUnsigned: boolean;
        readonly asReportDisputeLostUnsigned: {
            readonly disputeProof: PolkadotPrimitivesVstagingDisputeProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly type: "ReportDisputeLostUnsigned";
    }

    /** @name PolkadotPrimitivesVstagingDisputeProof (321) */
    interface PolkadotPrimitivesVstagingDisputeProof extends Struct {
        readonly timeSlot: PolkadotPrimitivesV8SlashingDisputesTimeSlot;
        readonly kind: PolkadotPrimitivesVstagingDisputeOffenceKind;
        readonly validatorIndex: u32;
        readonly validatorId: PolkadotPrimitivesV8ValidatorAppPublic;
    }

    /** @name PolkadotPrimitivesV8SlashingDisputesTimeSlot (322) */
    interface PolkadotPrimitivesV8SlashingDisputesTimeSlot extends Struct {
        readonly sessionIndex: u32;
        readonly candidateHash: H256;
    }

    /** @name PolkadotPrimitivesVstagingDisputeOffenceKind (323) */
    interface PolkadotPrimitivesVstagingDisputeOffenceKind extends Enum {
        readonly isForInvalidBacked: boolean;
        readonly isAgainstValid: boolean;
        readonly isForInvalidApproved: boolean;
        readonly type: "ForInvalidBacked" | "AgainstValid" | "ForInvalidApproved";
    }

    /** @name PalletMessageQueueCall (324) */
    interface PalletMessageQueueCall extends Enum {
        readonly isReapPage: boolean;
        readonly asReapPage: {
            readonly messageOrigin: DancelightRuntimeAggregateMessageOrigin;
            readonly pageIndex: u32;
        } & Struct;
        readonly isExecuteOverweight: boolean;
        readonly asExecuteOverweight: {
            readonly messageOrigin: DancelightRuntimeAggregateMessageOrigin;
            readonly page: u32;
            readonly index: u32;
            readonly weightLimit: SpWeightsWeightV2Weight;
        } & Struct;
        readonly type: "ReapPage" | "ExecuteOverweight";
    }

    /** @name DancelightRuntimeAggregateMessageOrigin (325) */
    interface DancelightRuntimeAggregateMessageOrigin extends Enum {
        readonly isUmp: boolean;
        readonly asUmp: PolkadotRuntimeParachainsInclusionUmpQueueId;
        readonly isSnowbridge: boolean;
        readonly asSnowbridge: SnowbridgeCoreChannelId;
        readonly isSnowbridgeTanssi: boolean;
        readonly asSnowbridgeTanssi: SnowbridgeCoreChannelId;
        readonly type: "Ump" | "Snowbridge" | "SnowbridgeTanssi";
    }

    /** @name PolkadotRuntimeParachainsInclusionUmpQueueId (326) */
    interface PolkadotRuntimeParachainsInclusionUmpQueueId extends Enum {
        readonly isPara: boolean;
        readonly asPara: u32;
        readonly type: "Para";
    }

    /** @name PolkadotRuntimeParachainsOnDemandPalletCall (327) */
    interface PolkadotRuntimeParachainsOnDemandPalletCall extends Enum {
        readonly isPlaceOrderAllowDeath: boolean;
        readonly asPlaceOrderAllowDeath: {
            readonly maxAmount: u128;
            readonly paraId: u32;
        } & Struct;
        readonly isPlaceOrderKeepAlive: boolean;
        readonly asPlaceOrderKeepAlive: {
            readonly maxAmount: u128;
            readonly paraId: u32;
        } & Struct;
        readonly isPlaceOrderWithCredits: boolean;
        readonly asPlaceOrderWithCredits: {
            readonly maxAmount: u128;
            readonly paraId: u32;
        } & Struct;
        readonly type: "PlaceOrderAllowDeath" | "PlaceOrderKeepAlive" | "PlaceOrderWithCredits";
    }

    /** @name PolkadotRuntimeCommonParasRegistrarPalletCall (328) */
    interface PolkadotRuntimeCommonParasRegistrarPalletCall extends Enum {
        readonly isRegister: boolean;
        readonly asRegister: {
            readonly id: u32;
            readonly genesisHead: Bytes;
            readonly validationCode: Bytes;
        } & Struct;
        readonly isForceRegister: boolean;
        readonly asForceRegister: {
            readonly who: AccountId32;
            readonly deposit: u128;
            readonly id: u32;
            readonly genesisHead: Bytes;
            readonly validationCode: Bytes;
        } & Struct;
        readonly isDeregister: boolean;
        readonly asDeregister: {
            readonly id: u32;
        } & Struct;
        readonly isSwap: boolean;
        readonly asSwap: {
            readonly id: u32;
            readonly other: u32;
        } & Struct;
        readonly isRemoveLock: boolean;
        readonly asRemoveLock: {
            readonly para: u32;
        } & Struct;
        readonly isReserve: boolean;
        readonly isAddLock: boolean;
        readonly asAddLock: {
            readonly para: u32;
        } & Struct;
        readonly isScheduleCodeUpgrade: boolean;
        readonly asScheduleCodeUpgrade: {
            readonly para: u32;
            readonly newCode: Bytes;
        } & Struct;
        readonly isSetCurrentHead: boolean;
        readonly asSetCurrentHead: {
            readonly para: u32;
            readonly newHead: Bytes;
        } & Struct;
        readonly type:
            | "Register"
            | "ForceRegister"
            | "Deregister"
            | "Swap"
            | "RemoveLock"
            | "Reserve"
            | "AddLock"
            | "ScheduleCodeUpgrade"
            | "SetCurrentHead";
    }

    /** @name PalletUtilityCall (329) */
    interface PalletUtilityCall extends Enum {
        readonly isBatch: boolean;
        readonly asBatch: {
            readonly calls: Vec<Call>;
        } & Struct;
        readonly isAsDerivative: boolean;
        readonly asAsDerivative: {
            readonly index: u16;
            readonly call: Call;
        } & Struct;
        readonly isBatchAll: boolean;
        readonly asBatchAll: {
            readonly calls: Vec<Call>;
        } & Struct;
        readonly isDispatchAs: boolean;
        readonly asDispatchAs: {
            readonly asOrigin: DancelightRuntimeOriginCaller;
            readonly call: Call;
        } & Struct;
        readonly isForceBatch: boolean;
        readonly asForceBatch: {
            readonly calls: Vec<Call>;
        } & Struct;
        readonly isWithWeight: boolean;
        readonly asWithWeight: {
            readonly call: Call;
            readonly weight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isIfElse: boolean;
        readonly asIfElse: {
            readonly main: Call;
            readonly fallback: Call;
        } & Struct;
        readonly isDispatchAsFallible: boolean;
        readonly asDispatchAsFallible: {
            readonly asOrigin: DancelightRuntimeOriginCaller;
            readonly call: Call;
        } & Struct;
        readonly type:
            | "Batch"
            | "AsDerivative"
            | "BatchAll"
            | "DispatchAs"
            | "ForceBatch"
            | "WithWeight"
            | "IfElse"
            | "DispatchAsFallible";
    }

    /** @name PalletIdentityCall (331) */
    interface PalletIdentityCall extends Enum {
        readonly isAddRegistrar: boolean;
        readonly asAddRegistrar: {
            readonly account: MultiAddress;
        } & Struct;
        readonly isSetIdentity: boolean;
        readonly asSetIdentity: {
            readonly info: PalletIdentityLegacyIdentityInfo;
        } & Struct;
        readonly isSetSubs: boolean;
        readonly asSetSubs: {
            readonly subs: Vec<ITuple<[AccountId32, Data]>>;
        } & Struct;
        readonly isClearIdentity: boolean;
        readonly isRequestJudgement: boolean;
        readonly asRequestJudgement: {
            readonly regIndex: Compact<u32>;
            readonly maxFee: Compact<u128>;
        } & Struct;
        readonly isCancelRequest: boolean;
        readonly asCancelRequest: {
            readonly regIndex: u32;
        } & Struct;
        readonly isSetFee: boolean;
        readonly asSetFee: {
            readonly index: Compact<u32>;
            readonly fee: Compact<u128>;
        } & Struct;
        readonly isSetAccountId: boolean;
        readonly asSetAccountId: {
            readonly index: Compact<u32>;
            readonly new_: MultiAddress;
        } & Struct;
        readonly isSetFields: boolean;
        readonly asSetFields: {
            readonly index: Compact<u32>;
            readonly fields: u64;
        } & Struct;
        readonly isProvideJudgement: boolean;
        readonly asProvideJudgement: {
            readonly regIndex: Compact<u32>;
            readonly target: MultiAddress;
            readonly judgement: PalletIdentityJudgement;
            readonly identity: H256;
        } & Struct;
        readonly isKillIdentity: boolean;
        readonly asKillIdentity: {
            readonly target: MultiAddress;
        } & Struct;
        readonly isAddSub: boolean;
        readonly asAddSub: {
            readonly sub: MultiAddress;
            readonly data: Data;
        } & Struct;
        readonly isRenameSub: boolean;
        readonly asRenameSub: {
            readonly sub: MultiAddress;
            readonly data: Data;
        } & Struct;
        readonly isRemoveSub: boolean;
        readonly asRemoveSub: {
            readonly sub: MultiAddress;
        } & Struct;
        readonly isQuitSub: boolean;
        readonly isAddUsernameAuthority: boolean;
        readonly asAddUsernameAuthority: {
            readonly authority: MultiAddress;
            readonly suffix: Bytes;
            readonly allocation: u32;
        } & Struct;
        readonly isRemoveUsernameAuthority: boolean;
        readonly asRemoveUsernameAuthority: {
            readonly suffix: Bytes;
            readonly authority: MultiAddress;
        } & Struct;
        readonly isSetUsernameFor: boolean;
        readonly asSetUsernameFor: {
            readonly who: MultiAddress;
            readonly username: Bytes;
            readonly signature: Option<SpRuntimeMultiSignature>;
            readonly useAllocation: bool;
        } & Struct;
        readonly isAcceptUsername: boolean;
        readonly asAcceptUsername: {
            readonly username: Bytes;
        } & Struct;
        readonly isRemoveExpiredApproval: boolean;
        readonly asRemoveExpiredApproval: {
            readonly username: Bytes;
        } & Struct;
        readonly isSetPrimaryUsername: boolean;
        readonly asSetPrimaryUsername: {
            readonly username: Bytes;
        } & Struct;
        readonly isUnbindUsername: boolean;
        readonly asUnbindUsername: {
            readonly username: Bytes;
        } & Struct;
        readonly isRemoveUsername: boolean;
        readonly asRemoveUsername: {
            readonly username: Bytes;
        } & Struct;
        readonly isKillUsername: boolean;
        readonly asKillUsername: {
            readonly username: Bytes;
        } & Struct;
        readonly type:
            | "AddRegistrar"
            | "SetIdentity"
            | "SetSubs"
            | "ClearIdentity"
            | "RequestJudgement"
            | "CancelRequest"
            | "SetFee"
            | "SetAccountId"
            | "SetFields"
            | "ProvideJudgement"
            | "KillIdentity"
            | "AddSub"
            | "RenameSub"
            | "RemoveSub"
            | "QuitSub"
            | "AddUsernameAuthority"
            | "RemoveUsernameAuthority"
            | "SetUsernameFor"
            | "AcceptUsername"
            | "RemoveExpiredApproval"
            | "SetPrimaryUsername"
            | "UnbindUsername"
            | "RemoveUsername"
            | "KillUsername";
    }

    /** @name PalletIdentityLegacyIdentityInfo (332) */
    interface PalletIdentityLegacyIdentityInfo extends Struct {
        readonly additional: Vec<ITuple<[Data, Data]>>;
        readonly display: Data;
        readonly legal: Data;
        readonly web: Data;
        readonly riot: Data;
        readonly email: Data;
        readonly pgpFingerprint: Option<U8aFixed>;
        readonly image: Data;
        readonly twitter: Data;
    }

    /** @name PalletIdentityJudgement (368) */
    interface PalletIdentityJudgement extends Enum {
        readonly isUnknown: boolean;
        readonly isFeePaid: boolean;
        readonly asFeePaid: u128;
        readonly isReasonable: boolean;
        readonly isKnownGood: boolean;
        readonly isOutOfDate: boolean;
        readonly isLowQuality: boolean;
        readonly isErroneous: boolean;
        readonly type: "Unknown" | "FeePaid" | "Reasonable" | "KnownGood" | "OutOfDate" | "LowQuality" | "Erroneous";
    }

    /** @name PalletSchedulerCall (370) */
    interface PalletSchedulerCall extends Enum {
        readonly isSchedule: boolean;
        readonly asSchedule: {
            readonly when: u32;
            readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
            readonly priority: u8;
            readonly call: Call;
        } & Struct;
        readonly isCancel: boolean;
        readonly asCancel: {
            readonly when: u32;
            readonly index: u32;
        } & Struct;
        readonly isScheduleNamed: boolean;
        readonly asScheduleNamed: {
            readonly id: U8aFixed;
            readonly when: u32;
            readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
            readonly priority: u8;
            readonly call: Call;
        } & Struct;
        readonly isCancelNamed: boolean;
        readonly asCancelNamed: {
            readonly id: U8aFixed;
        } & Struct;
        readonly isScheduleAfter: boolean;
        readonly asScheduleAfter: {
            readonly after: u32;
            readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
            readonly priority: u8;
            readonly call: Call;
        } & Struct;
        readonly isScheduleNamedAfter: boolean;
        readonly asScheduleNamedAfter: {
            readonly id: U8aFixed;
            readonly after: u32;
            readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
            readonly priority: u8;
            readonly call: Call;
        } & Struct;
        readonly isSetRetry: boolean;
        readonly asSetRetry: {
            readonly task: ITuple<[u32, u32]>;
            readonly retries: u8;
            readonly period: u32;
        } & Struct;
        readonly isSetRetryNamed: boolean;
        readonly asSetRetryNamed: {
            readonly id: U8aFixed;
            readonly retries: u8;
            readonly period: u32;
        } & Struct;
        readonly isCancelRetry: boolean;
        readonly asCancelRetry: {
            readonly task: ITuple<[u32, u32]>;
        } & Struct;
        readonly isCancelRetryNamed: boolean;
        readonly asCancelRetryNamed: {
            readonly id: U8aFixed;
        } & Struct;
        readonly type:
            | "Schedule"
            | "Cancel"
            | "ScheduleNamed"
            | "CancelNamed"
            | "ScheduleAfter"
            | "ScheduleNamedAfter"
            | "SetRetry"
            | "SetRetryNamed"
            | "CancelRetry"
            | "CancelRetryNamed";
    }

    /** @name PalletProxyCall (373) */
    interface PalletProxyCall extends Enum {
        readonly isProxy: boolean;
        readonly asProxy: {
            readonly real: MultiAddress;
            readonly forceProxyType: Option<DancelightRuntimeProxyType>;
            readonly call: Call;
        } & Struct;
        readonly isAddProxy: boolean;
        readonly asAddProxy: {
            readonly delegate: MultiAddress;
            readonly proxyType: DancelightRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly isRemoveProxy: boolean;
        readonly asRemoveProxy: {
            readonly delegate: MultiAddress;
            readonly proxyType: DancelightRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly isRemoveProxies: boolean;
        readonly isCreatePure: boolean;
        readonly asCreatePure: {
            readonly proxyType: DancelightRuntimeProxyType;
            readonly delay: u32;
            readonly index: u16;
        } & Struct;
        readonly isKillPure: boolean;
        readonly asKillPure: {
            readonly spawner: MultiAddress;
            readonly proxyType: DancelightRuntimeProxyType;
            readonly index: u16;
            readonly height: Compact<u32>;
            readonly extIndex: Compact<u32>;
        } & Struct;
        readonly isAnnounce: boolean;
        readonly asAnnounce: {
            readonly real: MultiAddress;
            readonly callHash: H256;
        } & Struct;
        readonly isRemoveAnnouncement: boolean;
        readonly asRemoveAnnouncement: {
            readonly real: MultiAddress;
            readonly callHash: H256;
        } & Struct;
        readonly isRejectAnnouncement: boolean;
        readonly asRejectAnnouncement: {
            readonly delegate: MultiAddress;
            readonly callHash: H256;
        } & Struct;
        readonly isProxyAnnounced: boolean;
        readonly asProxyAnnounced: {
            readonly delegate: MultiAddress;
            readonly real: MultiAddress;
            readonly forceProxyType: Option<DancelightRuntimeProxyType>;
            readonly call: Call;
        } & Struct;
        readonly isPokeDeposit: boolean;
        readonly type:
            | "Proxy"
            | "AddProxy"
            | "RemoveProxy"
            | "RemoveProxies"
            | "CreatePure"
            | "KillPure"
            | "Announce"
            | "RemoveAnnouncement"
            | "RejectAnnouncement"
            | "ProxyAnnounced"
            | "PokeDeposit";
    }

    /** @name DancelightRuntimeProxyType (375) */
    interface DancelightRuntimeProxyType extends Enum {
        readonly isAny: boolean;
        readonly isNonTransfer: boolean;
        readonly isGovernance: boolean;
        readonly isIdentityJudgement: boolean;
        readonly isCancelProxy: boolean;
        readonly isAuction: boolean;
        readonly isOnDemandOrdering: boolean;
        readonly isSudoRegistrar: boolean;
        readonly isSudoValidatorManagement: boolean;
        readonly isSessionKeyManagement: boolean;
        readonly isStaking: boolean;
        readonly isBalances: boolean;
        readonly type:
            | "Any"
            | "NonTransfer"
            | "Governance"
            | "IdentityJudgement"
            | "CancelProxy"
            | "Auction"
            | "OnDemandOrdering"
            | "SudoRegistrar"
            | "SudoValidatorManagement"
            | "SessionKeyManagement"
            | "Staking"
            | "Balances";
    }

    /** @name PalletMultisigCall (376) */
    interface PalletMultisigCall extends Enum {
        readonly isAsMultiThreshold1: boolean;
        readonly asAsMultiThreshold1: {
            readonly otherSignatories: Vec<AccountId32>;
            readonly call: Call;
        } & Struct;
        readonly isAsMulti: boolean;
        readonly asAsMulti: {
            readonly threshold: u16;
            readonly otherSignatories: Vec<AccountId32>;
            readonly maybeTimepoint: Option<PalletMultisigTimepoint>;
            readonly call: Call;
            readonly maxWeight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isApproveAsMulti: boolean;
        readonly asApproveAsMulti: {
            readonly threshold: u16;
            readonly otherSignatories: Vec<AccountId32>;
            readonly maybeTimepoint: Option<PalletMultisigTimepoint>;
            readonly callHash: U8aFixed;
            readonly maxWeight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isCancelAsMulti: boolean;
        readonly asCancelAsMulti: {
            readonly threshold: u16;
            readonly otherSignatories: Vec<AccountId32>;
            readonly timepoint: PalletMultisigTimepoint;
            readonly callHash: U8aFixed;
        } & Struct;
        readonly isPokeDeposit: boolean;
        readonly asPokeDeposit: {
            readonly threshold: u16;
            readonly otherSignatories: Vec<AccountId32>;
            readonly callHash: U8aFixed;
        } & Struct;
        readonly type: "AsMultiThreshold1" | "AsMulti" | "ApproveAsMulti" | "CancelAsMulti" | "PokeDeposit";
    }

    /** @name PalletMultisigTimepoint (378) */
    interface PalletMultisigTimepoint extends Struct {
        readonly height: u32;
        readonly index: u32;
    }

    /** @name PalletPreimageCall (379) */
    interface PalletPreimageCall extends Enum {
        readonly isNotePreimage: boolean;
        readonly asNotePreimage: {
            readonly bytes: Bytes;
        } & Struct;
        readonly isUnnotePreimage: boolean;
        readonly asUnnotePreimage: {
            readonly hash_: H256;
        } & Struct;
        readonly isRequestPreimage: boolean;
        readonly asRequestPreimage: {
            readonly hash_: H256;
        } & Struct;
        readonly isUnrequestPreimage: boolean;
        readonly asUnrequestPreimage: {
            readonly hash_: H256;
        } & Struct;
        readonly isEnsureUpdated: boolean;
        readonly asEnsureUpdated: {
            readonly hashes: Vec<H256>;
        } & Struct;
        readonly type: "NotePreimage" | "UnnotePreimage" | "RequestPreimage" | "UnrequestPreimage" | "EnsureUpdated";
    }

    /** @name PalletAssetRateCall (380) */
    interface PalletAssetRateCall extends Enum {
        readonly isCreate: boolean;
        readonly asCreate: {
            readonly assetKind: Null;
            readonly rate: u128;
        } & Struct;
        readonly isUpdate: boolean;
        readonly asUpdate: {
            readonly assetKind: Null;
            readonly rate: u128;
        } & Struct;
        readonly isRemove: boolean;
        readonly asRemove: {
            readonly assetKind: Null;
        } & Struct;
        readonly type: "Create" | "Update" | "Remove";
    }

    /** @name PalletAssetsCall (381) */
    interface PalletAssetsCall extends Enum {
        readonly isCreate: boolean;
        readonly asCreate: {
            readonly id: u16;
            readonly admin: MultiAddress;
            readonly minBalance: u128;
        } & Struct;
        readonly isForceCreate: boolean;
        readonly asForceCreate: {
            readonly id: u16;
            readonly owner: MultiAddress;
            readonly isSufficient: bool;
            readonly minBalance: Compact<u128>;
        } & Struct;
        readonly isStartDestroy: boolean;
        readonly asStartDestroy: {
            readonly id: u16;
        } & Struct;
        readonly isDestroyAccounts: boolean;
        readonly asDestroyAccounts: {
            readonly id: u16;
        } & Struct;
        readonly isDestroyApprovals: boolean;
        readonly asDestroyApprovals: {
            readonly id: u16;
        } & Struct;
        readonly isFinishDestroy: boolean;
        readonly asFinishDestroy: {
            readonly id: u16;
        } & Struct;
        readonly isMint: boolean;
        readonly asMint: {
            readonly id: u16;
            readonly beneficiary: MultiAddress;
            readonly amount: Compact<u128>;
        } & Struct;
        readonly isBurn: boolean;
        readonly asBurn: {
            readonly id: u16;
            readonly who: MultiAddress;
            readonly amount: Compact<u128>;
        } & Struct;
        readonly isTransfer: boolean;
        readonly asTransfer: {
            readonly id: u16;
            readonly target: MultiAddress;
            readonly amount: Compact<u128>;
        } & Struct;
        readonly isTransferKeepAlive: boolean;
        readonly asTransferKeepAlive: {
            readonly id: u16;
            readonly target: MultiAddress;
            readonly amount: Compact<u128>;
        } & Struct;
        readonly isForceTransfer: boolean;
        readonly asForceTransfer: {
            readonly id: u16;
            readonly source: MultiAddress;
            readonly dest: MultiAddress;
            readonly amount: Compact<u128>;
        } & Struct;
        readonly isFreeze: boolean;
        readonly asFreeze: {
            readonly id: u16;
            readonly who: MultiAddress;
        } & Struct;
        readonly isThaw: boolean;
        readonly asThaw: {
            readonly id: u16;
            readonly who: MultiAddress;
        } & Struct;
        readonly isFreezeAsset: boolean;
        readonly asFreezeAsset: {
            readonly id: u16;
        } & Struct;
        readonly isThawAsset: boolean;
        readonly asThawAsset: {
            readonly id: u16;
        } & Struct;
        readonly isTransferOwnership: boolean;
        readonly asTransferOwnership: {
            readonly id: u16;
            readonly owner: MultiAddress;
        } & Struct;
        readonly isSetTeam: boolean;
        readonly asSetTeam: {
            readonly id: u16;
            readonly issuer: MultiAddress;
            readonly admin: MultiAddress;
            readonly freezer: MultiAddress;
        } & Struct;
        readonly isSetMetadata: boolean;
        readonly asSetMetadata: {
            readonly id: u16;
            readonly name: Bytes;
            readonly symbol: Bytes;
            readonly decimals: u8;
        } & Struct;
        readonly isClearMetadata: boolean;
        readonly asClearMetadata: {
            readonly id: u16;
        } & Struct;
        readonly isForceSetMetadata: boolean;
        readonly asForceSetMetadata: {
            readonly id: u16;
            readonly name: Bytes;
            readonly symbol: Bytes;
            readonly decimals: u8;
            readonly isFrozen: bool;
        } & Struct;
        readonly isForceClearMetadata: boolean;
        readonly asForceClearMetadata: {
            readonly id: u16;
        } & Struct;
        readonly isForceAssetStatus: boolean;
        readonly asForceAssetStatus: {
            readonly id: u16;
            readonly owner: MultiAddress;
            readonly issuer: MultiAddress;
            readonly admin: MultiAddress;
            readonly freezer: MultiAddress;
            readonly minBalance: Compact<u128>;
            readonly isSufficient: bool;
            readonly isFrozen: bool;
        } & Struct;
        readonly isApproveTransfer: boolean;
        readonly asApproveTransfer: {
            readonly id: u16;
            readonly delegate: MultiAddress;
            readonly amount: Compact<u128>;
        } & Struct;
        readonly isCancelApproval: boolean;
        readonly asCancelApproval: {
            readonly id: u16;
            readonly delegate: MultiAddress;
        } & Struct;
        readonly isForceCancelApproval: boolean;
        readonly asForceCancelApproval: {
            readonly id: u16;
            readonly owner: MultiAddress;
            readonly delegate: MultiAddress;
        } & Struct;
        readonly isTransferApproved: boolean;
        readonly asTransferApproved: {
            readonly id: u16;
            readonly owner: MultiAddress;
            readonly destination: MultiAddress;
            readonly amount: Compact<u128>;
        } & Struct;
        readonly isTouch: boolean;
        readonly asTouch: {
            readonly id: u16;
        } & Struct;
        readonly isRefund: boolean;
        readonly asRefund: {
            readonly id: u16;
            readonly allowBurn: bool;
        } & Struct;
        readonly isSetMinBalance: boolean;
        readonly asSetMinBalance: {
            readonly id: u16;
            readonly minBalance: u128;
        } & Struct;
        readonly isTouchOther: boolean;
        readonly asTouchOther: {
            readonly id: u16;
            readonly who: MultiAddress;
        } & Struct;
        readonly isRefundOther: boolean;
        readonly asRefundOther: {
            readonly id: u16;
            readonly who: MultiAddress;
        } & Struct;
        readonly isBlock: boolean;
        readonly asBlock: {
            readonly id: u16;
            readonly who: MultiAddress;
        } & Struct;
        readonly isTransferAll: boolean;
        readonly asTransferAll: {
            readonly id: u16;
            readonly dest: MultiAddress;
            readonly keepAlive: bool;
        } & Struct;
        readonly type:
            | "Create"
            | "ForceCreate"
            | "StartDestroy"
            | "DestroyAccounts"
            | "DestroyApprovals"
            | "FinishDestroy"
            | "Mint"
            | "Burn"
            | "Transfer"
            | "TransferKeepAlive"
            | "ForceTransfer"
            | "Freeze"
            | "Thaw"
            | "FreezeAsset"
            | "ThawAsset"
            | "TransferOwnership"
            | "SetTeam"
            | "SetMetadata"
            | "ClearMetadata"
            | "ForceSetMetadata"
            | "ForceClearMetadata"
            | "ForceAssetStatus"
            | "ApproveTransfer"
            | "CancelApproval"
            | "ForceCancelApproval"
            | "TransferApproved"
            | "Touch"
            | "Refund"
            | "SetMinBalance"
            | "TouchOther"
            | "RefundOther"
            | "Block"
            | "TransferAll";
    }

    /** @name PalletForeignAssetCreatorCall (382) */
    interface PalletForeignAssetCreatorCall extends Enum {
        readonly isCreateForeignAsset: boolean;
        readonly asCreateForeignAsset: {
            readonly foreignAsset: StagingXcmV5Location;
            readonly assetId: u16;
            readonly admin: AccountId32;
            readonly isSufficient: bool;
            readonly minBalance: u128;
        } & Struct;
        readonly isChangeExistingAssetType: boolean;
        readonly asChangeExistingAssetType: {
            readonly assetId: u16;
            readonly newForeignAsset: StagingXcmV5Location;
        } & Struct;
        readonly isRemoveExistingAssetType: boolean;
        readonly asRemoveExistingAssetType: {
            readonly assetId: u16;
        } & Struct;
        readonly isDestroyForeignAsset: boolean;
        readonly asDestroyForeignAsset: {
            readonly assetId: u16;
        } & Struct;
        readonly type:
            | "CreateForeignAsset"
            | "ChangeExistingAssetType"
            | "RemoveExistingAssetType"
            | "DestroyForeignAsset";
    }

    /** @name PalletXcmCall (383) */
    interface PalletXcmCall extends Enum {
        readonly isSend: boolean;
        readonly asSend: {
            readonly dest: XcmVersionedLocation;
            readonly message: XcmVersionedXcm;
        } & Struct;
        readonly isTeleportAssets: boolean;
        readonly asTeleportAssets: {
            readonly dest: XcmVersionedLocation;
            readonly beneficiary: XcmVersionedLocation;
            readonly assets: XcmVersionedAssets;
            readonly feeAssetItem: u32;
        } & Struct;
        readonly isReserveTransferAssets: boolean;
        readonly asReserveTransferAssets: {
            readonly dest: XcmVersionedLocation;
            readonly beneficiary: XcmVersionedLocation;
            readonly assets: XcmVersionedAssets;
            readonly feeAssetItem: u32;
        } & Struct;
        readonly isExecute: boolean;
        readonly asExecute: {
            readonly message: XcmVersionedXcm;
            readonly maxWeight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isForceXcmVersion: boolean;
        readonly asForceXcmVersion: {
            readonly location: StagingXcmV5Location;
            readonly version: u32;
        } & Struct;
        readonly isForceDefaultXcmVersion: boolean;
        readonly asForceDefaultXcmVersion: {
            readonly maybeXcmVersion: Option<u32>;
        } & Struct;
        readonly isForceSubscribeVersionNotify: boolean;
        readonly asForceSubscribeVersionNotify: {
            readonly location: XcmVersionedLocation;
        } & Struct;
        readonly isForceUnsubscribeVersionNotify: boolean;
        readonly asForceUnsubscribeVersionNotify: {
            readonly location: XcmVersionedLocation;
        } & Struct;
        readonly isLimitedReserveTransferAssets: boolean;
        readonly asLimitedReserveTransferAssets: {
            readonly dest: XcmVersionedLocation;
            readonly beneficiary: XcmVersionedLocation;
            readonly assets: XcmVersionedAssets;
            readonly feeAssetItem: u32;
            readonly weightLimit: XcmV3WeightLimit;
        } & Struct;
        readonly isLimitedTeleportAssets: boolean;
        readonly asLimitedTeleportAssets: {
            readonly dest: XcmVersionedLocation;
            readonly beneficiary: XcmVersionedLocation;
            readonly assets: XcmVersionedAssets;
            readonly feeAssetItem: u32;
            readonly weightLimit: XcmV3WeightLimit;
        } & Struct;
        readonly isForceSuspension: boolean;
        readonly asForceSuspension: {
            readonly suspended: bool;
        } & Struct;
        readonly isTransferAssets: boolean;
        readonly asTransferAssets: {
            readonly dest: XcmVersionedLocation;
            readonly beneficiary: XcmVersionedLocation;
            readonly assets: XcmVersionedAssets;
            readonly feeAssetItem: u32;
            readonly weightLimit: XcmV3WeightLimit;
        } & Struct;
        readonly isClaimAssets: boolean;
        readonly asClaimAssets: {
            readonly assets: XcmVersionedAssets;
            readonly beneficiary: XcmVersionedLocation;
        } & Struct;
        readonly isTransferAssetsUsingTypeAndThen: boolean;
        readonly asTransferAssetsUsingTypeAndThen: {
            readonly dest: XcmVersionedLocation;
            readonly assets: XcmVersionedAssets;
            readonly assetsTransferType: StagingXcmExecutorAssetTransferTransferType;
            readonly remoteFeesId: XcmVersionedAssetId;
            readonly feesTransferType: StagingXcmExecutorAssetTransferTransferType;
            readonly customXcmOnDest: XcmVersionedXcm;
            readonly weightLimit: XcmV3WeightLimit;
        } & Struct;
        readonly isAddAuthorizedAlias: boolean;
        readonly asAddAuthorizedAlias: {
            readonly aliaser: XcmVersionedLocation;
            readonly expires: Option<u64>;
        } & Struct;
        readonly isRemoveAuthorizedAlias: boolean;
        readonly asRemoveAuthorizedAlias: {
            readonly aliaser: XcmVersionedLocation;
        } & Struct;
        readonly isRemoveAllAuthorizedAliases: boolean;
        readonly type:
            | "Send"
            | "TeleportAssets"
            | "ReserveTransferAssets"
            | "Execute"
            | "ForceXcmVersion"
            | "ForceDefaultXcmVersion"
            | "ForceSubscribeVersionNotify"
            | "ForceUnsubscribeVersionNotify"
            | "LimitedReserveTransferAssets"
            | "LimitedTeleportAssets"
            | "ForceSuspension"
            | "TransferAssets"
            | "ClaimAssets"
            | "TransferAssetsUsingTypeAndThen"
            | "AddAuthorizedAlias"
            | "RemoveAuthorizedAlias"
            | "RemoveAllAuthorizedAliases";
    }

    /** @name XcmVersionedXcm (384) */
    interface XcmVersionedXcm extends Enum {
        readonly isV3: boolean;
        readonly asV3: XcmV3Xcm;
        readonly isV4: boolean;
        readonly asV4: StagingXcmV4Xcm;
        readonly isV5: boolean;
        readonly asV5: StagingXcmV5Xcm;
        readonly type: "V3" | "V4" | "V5";
    }

    /** @name XcmV3Xcm (385) */
    interface XcmV3Xcm extends Vec<XcmV3Instruction> {}

    /** @name XcmV3Instruction (387) */
    interface XcmV3Instruction extends Enum {
        readonly isWithdrawAsset: boolean;
        readonly asWithdrawAsset: XcmV3MultiassetMultiAssets;
        readonly isReserveAssetDeposited: boolean;
        readonly asReserveAssetDeposited: XcmV3MultiassetMultiAssets;
        readonly isReceiveTeleportedAsset: boolean;
        readonly asReceiveTeleportedAsset: XcmV3MultiassetMultiAssets;
        readonly isQueryResponse: boolean;
        readonly asQueryResponse: {
            readonly queryId: Compact<u64>;
            readonly response: XcmV3Response;
            readonly maxWeight: SpWeightsWeightV2Weight;
            readonly querier: Option<StagingXcmV3MultiLocation>;
        } & Struct;
        readonly isTransferAsset: boolean;
        readonly asTransferAsset: {
            readonly assets: XcmV3MultiassetMultiAssets;
            readonly beneficiary: StagingXcmV3MultiLocation;
        } & Struct;
        readonly isTransferReserveAsset: boolean;
        readonly asTransferReserveAsset: {
            readonly assets: XcmV3MultiassetMultiAssets;
            readonly dest: StagingXcmV3MultiLocation;
            readonly xcm: XcmV3Xcm;
        } & Struct;
        readonly isTransact: boolean;
        readonly asTransact: {
            readonly originKind: XcmV3OriginKind;
            readonly requireWeightAtMost: SpWeightsWeightV2Weight;
            readonly call: XcmDoubleEncoded;
        } & Struct;
        readonly isHrmpNewChannelOpenRequest: boolean;
        readonly asHrmpNewChannelOpenRequest: {
            readonly sender: Compact<u32>;
            readonly maxMessageSize: Compact<u32>;
            readonly maxCapacity: Compact<u32>;
        } & Struct;
        readonly isHrmpChannelAccepted: boolean;
        readonly asHrmpChannelAccepted: {
            readonly recipient: Compact<u32>;
        } & Struct;
        readonly isHrmpChannelClosing: boolean;
        readonly asHrmpChannelClosing: {
            readonly initiator: Compact<u32>;
            readonly sender: Compact<u32>;
            readonly recipient: Compact<u32>;
        } & Struct;
        readonly isClearOrigin: boolean;
        readonly isDescendOrigin: boolean;
        readonly asDescendOrigin: XcmV3Junctions;
        readonly isReportError: boolean;
        readonly asReportError: XcmV3QueryResponseInfo;
        readonly isDepositAsset: boolean;
        readonly asDepositAsset: {
            readonly assets: XcmV3MultiassetMultiAssetFilter;
            readonly beneficiary: StagingXcmV3MultiLocation;
        } & Struct;
        readonly isDepositReserveAsset: boolean;
        readonly asDepositReserveAsset: {
            readonly assets: XcmV3MultiassetMultiAssetFilter;
            readonly dest: StagingXcmV3MultiLocation;
            readonly xcm: XcmV3Xcm;
        } & Struct;
        readonly isExchangeAsset: boolean;
        readonly asExchangeAsset: {
            readonly give: XcmV3MultiassetMultiAssetFilter;
            readonly want: XcmV3MultiassetMultiAssets;
            readonly maximal: bool;
        } & Struct;
        readonly isInitiateReserveWithdraw: boolean;
        readonly asInitiateReserveWithdraw: {
            readonly assets: XcmV3MultiassetMultiAssetFilter;
            readonly reserve: StagingXcmV3MultiLocation;
            readonly xcm: XcmV3Xcm;
        } & Struct;
        readonly isInitiateTeleport: boolean;
        readonly asInitiateTeleport: {
            readonly assets: XcmV3MultiassetMultiAssetFilter;
            readonly dest: StagingXcmV3MultiLocation;
            readonly xcm: XcmV3Xcm;
        } & Struct;
        readonly isReportHolding: boolean;
        readonly asReportHolding: {
            readonly responseInfo: XcmV3QueryResponseInfo;
            readonly assets: XcmV3MultiassetMultiAssetFilter;
        } & Struct;
        readonly isBuyExecution: boolean;
        readonly asBuyExecution: {
            readonly fees: XcmV3MultiAsset;
            readonly weightLimit: XcmV3WeightLimit;
        } & Struct;
        readonly isRefundSurplus: boolean;
        readonly isSetErrorHandler: boolean;
        readonly asSetErrorHandler: XcmV3Xcm;
        readonly isSetAppendix: boolean;
        readonly asSetAppendix: XcmV3Xcm;
        readonly isClearError: boolean;
        readonly isClaimAsset: boolean;
        readonly asClaimAsset: {
            readonly assets: XcmV3MultiassetMultiAssets;
            readonly ticket: StagingXcmV3MultiLocation;
        } & Struct;
        readonly isTrap: boolean;
        readonly asTrap: Compact<u64>;
        readonly isSubscribeVersion: boolean;
        readonly asSubscribeVersion: {
            readonly queryId: Compact<u64>;
            readonly maxResponseWeight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isUnsubscribeVersion: boolean;
        readonly isBurnAsset: boolean;
        readonly asBurnAsset: XcmV3MultiassetMultiAssets;
        readonly isExpectAsset: boolean;
        readonly asExpectAsset: XcmV3MultiassetMultiAssets;
        readonly isExpectOrigin: boolean;
        readonly asExpectOrigin: Option<StagingXcmV3MultiLocation>;
        readonly isExpectError: boolean;
        readonly asExpectError: Option<ITuple<[u32, XcmV3TraitsError]>>;
        readonly isExpectTransactStatus: boolean;
        readonly asExpectTransactStatus: XcmV3MaybeErrorCode;
        readonly isQueryPallet: boolean;
        readonly asQueryPallet: {
            readonly moduleName: Bytes;
            readonly responseInfo: XcmV3QueryResponseInfo;
        } & Struct;
        readonly isExpectPallet: boolean;
        readonly asExpectPallet: {
            readonly index: Compact<u32>;
            readonly name: Bytes;
            readonly moduleName: Bytes;
            readonly crateMajor: Compact<u32>;
            readonly minCrateMinor: Compact<u32>;
        } & Struct;
        readonly isReportTransactStatus: boolean;
        readonly asReportTransactStatus: XcmV3QueryResponseInfo;
        readonly isClearTransactStatus: boolean;
        readonly isUniversalOrigin: boolean;
        readonly asUniversalOrigin: XcmV3Junction;
        readonly isExportMessage: boolean;
        readonly asExportMessage: {
            readonly network: XcmV3JunctionNetworkId;
            readonly destination: XcmV3Junctions;
            readonly xcm: XcmV3Xcm;
        } & Struct;
        readonly isLockAsset: boolean;
        readonly asLockAsset: {
            readonly asset: XcmV3MultiAsset;
            readonly unlocker: StagingXcmV3MultiLocation;
        } & Struct;
        readonly isUnlockAsset: boolean;
        readonly asUnlockAsset: {
            readonly asset: XcmV3MultiAsset;
            readonly target: StagingXcmV3MultiLocation;
        } & Struct;
        readonly isNoteUnlockable: boolean;
        readonly asNoteUnlockable: {
            readonly asset: XcmV3MultiAsset;
            readonly owner: StagingXcmV3MultiLocation;
        } & Struct;
        readonly isRequestUnlock: boolean;
        readonly asRequestUnlock: {
            readonly asset: XcmV3MultiAsset;
            readonly locker: StagingXcmV3MultiLocation;
        } & Struct;
        readonly isSetFeesMode: boolean;
        readonly asSetFeesMode: {
            readonly jitWithdraw: bool;
        } & Struct;
        readonly isSetTopic: boolean;
        readonly asSetTopic: U8aFixed;
        readonly isClearTopic: boolean;
        readonly isAliasOrigin: boolean;
        readonly asAliasOrigin: StagingXcmV3MultiLocation;
        readonly isUnpaidExecution: boolean;
        readonly asUnpaidExecution: {
            readonly weightLimit: XcmV3WeightLimit;
            readonly checkOrigin: Option<StagingXcmV3MultiLocation>;
        } & Struct;
        readonly type:
            | "WithdrawAsset"
            | "ReserveAssetDeposited"
            | "ReceiveTeleportedAsset"
            | "QueryResponse"
            | "TransferAsset"
            | "TransferReserveAsset"
            | "Transact"
            | "HrmpNewChannelOpenRequest"
            | "HrmpChannelAccepted"
            | "HrmpChannelClosing"
            | "ClearOrigin"
            | "DescendOrigin"
            | "ReportError"
            | "DepositAsset"
            | "DepositReserveAsset"
            | "ExchangeAsset"
            | "InitiateReserveWithdraw"
            | "InitiateTeleport"
            | "ReportHolding"
            | "BuyExecution"
            | "RefundSurplus"
            | "SetErrorHandler"
            | "SetAppendix"
            | "ClearError"
            | "ClaimAsset"
            | "Trap"
            | "SubscribeVersion"
            | "UnsubscribeVersion"
            | "BurnAsset"
            | "ExpectAsset"
            | "ExpectOrigin"
            | "ExpectError"
            | "ExpectTransactStatus"
            | "QueryPallet"
            | "ExpectPallet"
            | "ReportTransactStatus"
            | "ClearTransactStatus"
            | "UniversalOrigin"
            | "ExportMessage"
            | "LockAsset"
            | "UnlockAsset"
            | "NoteUnlockable"
            | "RequestUnlock"
            | "SetFeesMode"
            | "SetTopic"
            | "ClearTopic"
            | "AliasOrigin"
            | "UnpaidExecution";
    }

    /** @name XcmV3MultiassetMultiAssets (388) */
    interface XcmV3MultiassetMultiAssets extends Vec<XcmV3MultiAsset> {}

    /** @name XcmV3MultiAsset (390) */
    interface XcmV3MultiAsset extends Struct {
        readonly id: XcmV3MultiassetAssetId;
        readonly fun: XcmV3MultiassetFungibility;
    }

    /** @name XcmV3MultiassetAssetId (391) */
    interface XcmV3MultiassetAssetId extends Enum {
        readonly isConcrete: boolean;
        readonly asConcrete: StagingXcmV3MultiLocation;
        readonly isAbstract: boolean;
        readonly asAbstract: U8aFixed;
        readonly type: "Concrete" | "Abstract";
    }

    /** @name XcmV3MultiassetFungibility (392) */
    interface XcmV3MultiassetFungibility extends Enum {
        readonly isFungible: boolean;
        readonly asFungible: Compact<u128>;
        readonly isNonFungible: boolean;
        readonly asNonFungible: XcmV3MultiassetAssetInstance;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name XcmV3MultiassetAssetInstance (393) */
    interface XcmV3MultiassetAssetInstance extends Enum {
        readonly isUndefined: boolean;
        readonly isIndex: boolean;
        readonly asIndex: Compact<u128>;
        readonly isArray4: boolean;
        readonly asArray4: U8aFixed;
        readonly isArray8: boolean;
        readonly asArray8: U8aFixed;
        readonly isArray16: boolean;
        readonly asArray16: U8aFixed;
        readonly isArray32: boolean;
        readonly asArray32: U8aFixed;
        readonly type: "Undefined" | "Index" | "Array4" | "Array8" | "Array16" | "Array32";
    }

    /** @name XcmV3Response (394) */
    interface XcmV3Response extends Enum {
        readonly isNull: boolean;
        readonly isAssets: boolean;
        readonly asAssets: XcmV3MultiassetMultiAssets;
        readonly isExecutionResult: boolean;
        readonly asExecutionResult: Option<ITuple<[u32, XcmV3TraitsError]>>;
        readonly isVersion: boolean;
        readonly asVersion: u32;
        readonly isPalletsInfo: boolean;
        readonly asPalletsInfo: Vec<XcmV3PalletInfo>;
        readonly isDispatchResult: boolean;
        readonly asDispatchResult: XcmV3MaybeErrorCode;
        readonly type: "Null" | "Assets" | "ExecutionResult" | "Version" | "PalletsInfo" | "DispatchResult";
    }

    /** @name XcmV3TraitsError (397) */
    interface XcmV3TraitsError extends Enum {
        readonly isOverflow: boolean;
        readonly isUnimplemented: boolean;
        readonly isUntrustedReserveLocation: boolean;
        readonly isUntrustedTeleportLocation: boolean;
        readonly isLocationFull: boolean;
        readonly isLocationNotInvertible: boolean;
        readonly isBadOrigin: boolean;
        readonly isInvalidLocation: boolean;
        readonly isAssetNotFound: boolean;
        readonly isFailedToTransactAsset: boolean;
        readonly isNotWithdrawable: boolean;
        readonly isLocationCannotHold: boolean;
        readonly isExceedsMaxMessageSize: boolean;
        readonly isDestinationUnsupported: boolean;
        readonly isTransport: boolean;
        readonly isUnroutable: boolean;
        readonly isUnknownClaim: boolean;
        readonly isFailedToDecode: boolean;
        readonly isMaxWeightInvalid: boolean;
        readonly isNotHoldingFees: boolean;
        readonly isTooExpensive: boolean;
        readonly isTrap: boolean;
        readonly asTrap: u64;
        readonly isExpectationFalse: boolean;
        readonly isPalletNotFound: boolean;
        readonly isNameMismatch: boolean;
        readonly isVersionIncompatible: boolean;
        readonly isHoldingWouldOverflow: boolean;
        readonly isExportError: boolean;
        readonly isReanchorFailed: boolean;
        readonly isNoDeal: boolean;
        readonly isFeesNotMet: boolean;
        readonly isLockError: boolean;
        readonly isNoPermission: boolean;
        readonly isUnanchored: boolean;
        readonly isNotDepositable: boolean;
        readonly isUnhandledXcmVersion: boolean;
        readonly isWeightLimitReached: boolean;
        readonly asWeightLimitReached: SpWeightsWeightV2Weight;
        readonly isBarrier: boolean;
        readonly isWeightNotComputable: boolean;
        readonly isExceedsStackLimit: boolean;
        readonly type:
            | "Overflow"
            | "Unimplemented"
            | "UntrustedReserveLocation"
            | "UntrustedTeleportLocation"
            | "LocationFull"
            | "LocationNotInvertible"
            | "BadOrigin"
            | "InvalidLocation"
            | "AssetNotFound"
            | "FailedToTransactAsset"
            | "NotWithdrawable"
            | "LocationCannotHold"
            | "ExceedsMaxMessageSize"
            | "DestinationUnsupported"
            | "Transport"
            | "Unroutable"
            | "UnknownClaim"
            | "FailedToDecode"
            | "MaxWeightInvalid"
            | "NotHoldingFees"
            | "TooExpensive"
            | "Trap"
            | "ExpectationFalse"
            | "PalletNotFound"
            | "NameMismatch"
            | "VersionIncompatible"
            | "HoldingWouldOverflow"
            | "ExportError"
            | "ReanchorFailed"
            | "NoDeal"
            | "FeesNotMet"
            | "LockError"
            | "NoPermission"
            | "Unanchored"
            | "NotDepositable"
            | "UnhandledXcmVersion"
            | "WeightLimitReached"
            | "Barrier"
            | "WeightNotComputable"
            | "ExceedsStackLimit";
    }

    /** @name XcmV3PalletInfo (399) */
    interface XcmV3PalletInfo extends Struct {
        readonly index: Compact<u32>;
        readonly name: Bytes;
        readonly moduleName: Bytes;
        readonly major: Compact<u32>;
        readonly minor: Compact<u32>;
        readonly patch: Compact<u32>;
    }

    /** @name XcmV3MaybeErrorCode (402) */
    interface XcmV3MaybeErrorCode extends Enum {
        readonly isSuccess: boolean;
        readonly isError: boolean;
        readonly asError: Bytes;
        readonly isTruncatedError: boolean;
        readonly asTruncatedError: Bytes;
        readonly type: "Success" | "Error" | "TruncatedError";
    }

    /** @name XcmV3OriginKind (405) */
    interface XcmV3OriginKind extends Enum {
        readonly isNative: boolean;
        readonly isSovereignAccount: boolean;
        readonly isSuperuser: boolean;
        readonly isXcm: boolean;
        readonly type: "Native" | "SovereignAccount" | "Superuser" | "Xcm";
    }

    /** @name XcmDoubleEncoded (406) */
    interface XcmDoubleEncoded extends Struct {
        readonly encoded: Bytes;
    }

    /** @name XcmV3QueryResponseInfo (407) */
    interface XcmV3QueryResponseInfo extends Struct {
        readonly destination: StagingXcmV3MultiLocation;
        readonly queryId: Compact<u64>;
        readonly maxWeight: SpWeightsWeightV2Weight;
    }

    /** @name XcmV3MultiassetMultiAssetFilter (408) */
    interface XcmV3MultiassetMultiAssetFilter extends Enum {
        readonly isDefinite: boolean;
        readonly asDefinite: XcmV3MultiassetMultiAssets;
        readonly isWild: boolean;
        readonly asWild: XcmV3MultiassetWildMultiAsset;
        readonly type: "Definite" | "Wild";
    }

    /** @name XcmV3MultiassetWildMultiAsset (409) */
    interface XcmV3MultiassetWildMultiAsset extends Enum {
        readonly isAll: boolean;
        readonly isAllOf: boolean;
        readonly asAllOf: {
            readonly id: XcmV3MultiassetAssetId;
            readonly fun: XcmV3MultiassetWildFungibility;
        } & Struct;
        readonly isAllCounted: boolean;
        readonly asAllCounted: Compact<u32>;
        readonly isAllOfCounted: boolean;
        readonly asAllOfCounted: {
            readonly id: XcmV3MultiassetAssetId;
            readonly fun: XcmV3MultiassetWildFungibility;
            readonly count: Compact<u32>;
        } & Struct;
        readonly type: "All" | "AllOf" | "AllCounted" | "AllOfCounted";
    }

    /** @name XcmV3MultiassetWildFungibility (410) */
    interface XcmV3MultiassetWildFungibility extends Enum {
        readonly isFungible: boolean;
        readonly isNonFungible: boolean;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name XcmV3WeightLimit (411) */
    interface XcmV3WeightLimit extends Enum {
        readonly isUnlimited: boolean;
        readonly isLimited: boolean;
        readonly asLimited: SpWeightsWeightV2Weight;
        readonly type: "Unlimited" | "Limited";
    }

    /** @name StagingXcmV4Xcm (412) */
    interface StagingXcmV4Xcm extends Vec<StagingXcmV4Instruction> {}

    /** @name StagingXcmV4Instruction (414) */
    interface StagingXcmV4Instruction extends Enum {
        readonly isWithdrawAsset: boolean;
        readonly asWithdrawAsset: StagingXcmV4AssetAssets;
        readonly isReserveAssetDeposited: boolean;
        readonly asReserveAssetDeposited: StagingXcmV4AssetAssets;
        readonly isReceiveTeleportedAsset: boolean;
        readonly asReceiveTeleportedAsset: StagingXcmV4AssetAssets;
        readonly isQueryResponse: boolean;
        readonly asQueryResponse: {
            readonly queryId: Compact<u64>;
            readonly response: StagingXcmV4Response;
            readonly maxWeight: SpWeightsWeightV2Weight;
            readonly querier: Option<StagingXcmV4Location>;
        } & Struct;
        readonly isTransferAsset: boolean;
        readonly asTransferAsset: {
            readonly assets: StagingXcmV4AssetAssets;
            readonly beneficiary: StagingXcmV4Location;
        } & Struct;
        readonly isTransferReserveAsset: boolean;
        readonly asTransferReserveAsset: {
            readonly assets: StagingXcmV4AssetAssets;
            readonly dest: StagingXcmV4Location;
            readonly xcm: StagingXcmV4Xcm;
        } & Struct;
        readonly isTransact: boolean;
        readonly asTransact: {
            readonly originKind: XcmV3OriginKind;
            readonly requireWeightAtMost: SpWeightsWeightV2Weight;
            readonly call: XcmDoubleEncoded;
        } & Struct;
        readonly isHrmpNewChannelOpenRequest: boolean;
        readonly asHrmpNewChannelOpenRequest: {
            readonly sender: Compact<u32>;
            readonly maxMessageSize: Compact<u32>;
            readonly maxCapacity: Compact<u32>;
        } & Struct;
        readonly isHrmpChannelAccepted: boolean;
        readonly asHrmpChannelAccepted: {
            readonly recipient: Compact<u32>;
        } & Struct;
        readonly isHrmpChannelClosing: boolean;
        readonly asHrmpChannelClosing: {
            readonly initiator: Compact<u32>;
            readonly sender: Compact<u32>;
            readonly recipient: Compact<u32>;
        } & Struct;
        readonly isClearOrigin: boolean;
        readonly isDescendOrigin: boolean;
        readonly asDescendOrigin: StagingXcmV4Junctions;
        readonly isReportError: boolean;
        readonly asReportError: StagingXcmV4QueryResponseInfo;
        readonly isDepositAsset: boolean;
        readonly asDepositAsset: {
            readonly assets: StagingXcmV4AssetAssetFilter;
            readonly beneficiary: StagingXcmV4Location;
        } & Struct;
        readonly isDepositReserveAsset: boolean;
        readonly asDepositReserveAsset: {
            readonly assets: StagingXcmV4AssetAssetFilter;
            readonly dest: StagingXcmV4Location;
            readonly xcm: StagingXcmV4Xcm;
        } & Struct;
        readonly isExchangeAsset: boolean;
        readonly asExchangeAsset: {
            readonly give: StagingXcmV4AssetAssetFilter;
            readonly want: StagingXcmV4AssetAssets;
            readonly maximal: bool;
        } & Struct;
        readonly isInitiateReserveWithdraw: boolean;
        readonly asInitiateReserveWithdraw: {
            readonly assets: StagingXcmV4AssetAssetFilter;
            readonly reserve: StagingXcmV4Location;
            readonly xcm: StagingXcmV4Xcm;
        } & Struct;
        readonly isInitiateTeleport: boolean;
        readonly asInitiateTeleport: {
            readonly assets: StagingXcmV4AssetAssetFilter;
            readonly dest: StagingXcmV4Location;
            readonly xcm: StagingXcmV4Xcm;
        } & Struct;
        readonly isReportHolding: boolean;
        readonly asReportHolding: {
            readonly responseInfo: StagingXcmV4QueryResponseInfo;
            readonly assets: StagingXcmV4AssetAssetFilter;
        } & Struct;
        readonly isBuyExecution: boolean;
        readonly asBuyExecution: {
            readonly fees: StagingXcmV4Asset;
            readonly weightLimit: XcmV3WeightLimit;
        } & Struct;
        readonly isRefundSurplus: boolean;
        readonly isSetErrorHandler: boolean;
        readonly asSetErrorHandler: StagingXcmV4Xcm;
        readonly isSetAppendix: boolean;
        readonly asSetAppendix: StagingXcmV4Xcm;
        readonly isClearError: boolean;
        readonly isClaimAsset: boolean;
        readonly asClaimAsset: {
            readonly assets: StagingXcmV4AssetAssets;
            readonly ticket: StagingXcmV4Location;
        } & Struct;
        readonly isTrap: boolean;
        readonly asTrap: Compact<u64>;
        readonly isSubscribeVersion: boolean;
        readonly asSubscribeVersion: {
            readonly queryId: Compact<u64>;
            readonly maxResponseWeight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isUnsubscribeVersion: boolean;
        readonly isBurnAsset: boolean;
        readonly asBurnAsset: StagingXcmV4AssetAssets;
        readonly isExpectAsset: boolean;
        readonly asExpectAsset: StagingXcmV4AssetAssets;
        readonly isExpectOrigin: boolean;
        readonly asExpectOrigin: Option<StagingXcmV4Location>;
        readonly isExpectError: boolean;
        readonly asExpectError: Option<ITuple<[u32, XcmV3TraitsError]>>;
        readonly isExpectTransactStatus: boolean;
        readonly asExpectTransactStatus: XcmV3MaybeErrorCode;
        readonly isQueryPallet: boolean;
        readonly asQueryPallet: {
            readonly moduleName: Bytes;
            readonly responseInfo: StagingXcmV4QueryResponseInfo;
        } & Struct;
        readonly isExpectPallet: boolean;
        readonly asExpectPallet: {
            readonly index: Compact<u32>;
            readonly name: Bytes;
            readonly moduleName: Bytes;
            readonly crateMajor: Compact<u32>;
            readonly minCrateMinor: Compact<u32>;
        } & Struct;
        readonly isReportTransactStatus: boolean;
        readonly asReportTransactStatus: StagingXcmV4QueryResponseInfo;
        readonly isClearTransactStatus: boolean;
        readonly isUniversalOrigin: boolean;
        readonly asUniversalOrigin: StagingXcmV4Junction;
        readonly isExportMessage: boolean;
        readonly asExportMessage: {
            readonly network: StagingXcmV4JunctionNetworkId;
            readonly destination: StagingXcmV4Junctions;
            readonly xcm: StagingXcmV4Xcm;
        } & Struct;
        readonly isLockAsset: boolean;
        readonly asLockAsset: {
            readonly asset: StagingXcmV4Asset;
            readonly unlocker: StagingXcmV4Location;
        } & Struct;
        readonly isUnlockAsset: boolean;
        readonly asUnlockAsset: {
            readonly asset: StagingXcmV4Asset;
            readonly target: StagingXcmV4Location;
        } & Struct;
        readonly isNoteUnlockable: boolean;
        readonly asNoteUnlockable: {
            readonly asset: StagingXcmV4Asset;
            readonly owner: StagingXcmV4Location;
        } & Struct;
        readonly isRequestUnlock: boolean;
        readonly asRequestUnlock: {
            readonly asset: StagingXcmV4Asset;
            readonly locker: StagingXcmV4Location;
        } & Struct;
        readonly isSetFeesMode: boolean;
        readonly asSetFeesMode: {
            readonly jitWithdraw: bool;
        } & Struct;
        readonly isSetTopic: boolean;
        readonly asSetTopic: U8aFixed;
        readonly isClearTopic: boolean;
        readonly isAliasOrigin: boolean;
        readonly asAliasOrigin: StagingXcmV4Location;
        readonly isUnpaidExecution: boolean;
        readonly asUnpaidExecution: {
            readonly weightLimit: XcmV3WeightLimit;
            readonly checkOrigin: Option<StagingXcmV4Location>;
        } & Struct;
        readonly type:
            | "WithdrawAsset"
            | "ReserveAssetDeposited"
            | "ReceiveTeleportedAsset"
            | "QueryResponse"
            | "TransferAsset"
            | "TransferReserveAsset"
            | "Transact"
            | "HrmpNewChannelOpenRequest"
            | "HrmpChannelAccepted"
            | "HrmpChannelClosing"
            | "ClearOrigin"
            | "DescendOrigin"
            | "ReportError"
            | "DepositAsset"
            | "DepositReserveAsset"
            | "ExchangeAsset"
            | "InitiateReserveWithdraw"
            | "InitiateTeleport"
            | "ReportHolding"
            | "BuyExecution"
            | "RefundSurplus"
            | "SetErrorHandler"
            | "SetAppendix"
            | "ClearError"
            | "ClaimAsset"
            | "Trap"
            | "SubscribeVersion"
            | "UnsubscribeVersion"
            | "BurnAsset"
            | "ExpectAsset"
            | "ExpectOrigin"
            | "ExpectError"
            | "ExpectTransactStatus"
            | "QueryPallet"
            | "ExpectPallet"
            | "ReportTransactStatus"
            | "ClearTransactStatus"
            | "UniversalOrigin"
            | "ExportMessage"
            | "LockAsset"
            | "UnlockAsset"
            | "NoteUnlockable"
            | "RequestUnlock"
            | "SetFeesMode"
            | "SetTopic"
            | "ClearTopic"
            | "AliasOrigin"
            | "UnpaidExecution";
    }

    /** @name StagingXcmV4AssetAssets (415) */
    interface StagingXcmV4AssetAssets extends Vec<StagingXcmV4Asset> {}

    /** @name StagingXcmV4Asset (417) */
    interface StagingXcmV4Asset extends Struct {
        readonly id: StagingXcmV4AssetAssetId;
        readonly fun: StagingXcmV4AssetFungibility;
    }

    /** @name StagingXcmV4AssetAssetId (418) */
    interface StagingXcmV4AssetAssetId extends StagingXcmV4Location {}

    /** @name StagingXcmV4AssetFungibility (419) */
    interface StagingXcmV4AssetFungibility extends Enum {
        readonly isFungible: boolean;
        readonly asFungible: Compact<u128>;
        readonly isNonFungible: boolean;
        readonly asNonFungible: StagingXcmV4AssetAssetInstance;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name StagingXcmV4AssetAssetInstance (420) */
    interface StagingXcmV4AssetAssetInstance extends Enum {
        readonly isUndefined: boolean;
        readonly isIndex: boolean;
        readonly asIndex: Compact<u128>;
        readonly isArray4: boolean;
        readonly asArray4: U8aFixed;
        readonly isArray8: boolean;
        readonly asArray8: U8aFixed;
        readonly isArray16: boolean;
        readonly asArray16: U8aFixed;
        readonly isArray32: boolean;
        readonly asArray32: U8aFixed;
        readonly type: "Undefined" | "Index" | "Array4" | "Array8" | "Array16" | "Array32";
    }

    /** @name StagingXcmV4Response (421) */
    interface StagingXcmV4Response extends Enum {
        readonly isNull: boolean;
        readonly isAssets: boolean;
        readonly asAssets: StagingXcmV4AssetAssets;
        readonly isExecutionResult: boolean;
        readonly asExecutionResult: Option<ITuple<[u32, XcmV3TraitsError]>>;
        readonly isVersion: boolean;
        readonly asVersion: u32;
        readonly isPalletsInfo: boolean;
        readonly asPalletsInfo: Vec<StagingXcmV4PalletInfo>;
        readonly isDispatchResult: boolean;
        readonly asDispatchResult: XcmV3MaybeErrorCode;
        readonly type: "Null" | "Assets" | "ExecutionResult" | "Version" | "PalletsInfo" | "DispatchResult";
    }

    /** @name StagingXcmV4PalletInfo (423) */
    interface StagingXcmV4PalletInfo extends Struct {
        readonly index: Compact<u32>;
        readonly name: Bytes;
        readonly moduleName: Bytes;
        readonly major: Compact<u32>;
        readonly minor: Compact<u32>;
        readonly patch: Compact<u32>;
    }

    /** @name StagingXcmV4QueryResponseInfo (427) */
    interface StagingXcmV4QueryResponseInfo extends Struct {
        readonly destination: StagingXcmV4Location;
        readonly queryId: Compact<u64>;
        readonly maxWeight: SpWeightsWeightV2Weight;
    }

    /** @name StagingXcmV4AssetAssetFilter (428) */
    interface StagingXcmV4AssetAssetFilter extends Enum {
        readonly isDefinite: boolean;
        readonly asDefinite: StagingXcmV4AssetAssets;
        readonly isWild: boolean;
        readonly asWild: StagingXcmV4AssetWildAsset;
        readonly type: "Definite" | "Wild";
    }

    /** @name StagingXcmV4AssetWildAsset (429) */
    interface StagingXcmV4AssetWildAsset extends Enum {
        readonly isAll: boolean;
        readonly isAllOf: boolean;
        readonly asAllOf: {
            readonly id: StagingXcmV4AssetAssetId;
            readonly fun: StagingXcmV4AssetWildFungibility;
        } & Struct;
        readonly isAllCounted: boolean;
        readonly asAllCounted: Compact<u32>;
        readonly isAllOfCounted: boolean;
        readonly asAllOfCounted: {
            readonly id: StagingXcmV4AssetAssetId;
            readonly fun: StagingXcmV4AssetWildFungibility;
            readonly count: Compact<u32>;
        } & Struct;
        readonly type: "All" | "AllOf" | "AllCounted" | "AllOfCounted";
    }

    /** @name StagingXcmV4AssetWildFungibility (430) */
    interface StagingXcmV4AssetWildFungibility extends Enum {
        readonly isFungible: boolean;
        readonly isNonFungible: boolean;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name StagingXcmV5Xcm (431) */
    interface StagingXcmV5Xcm extends Vec<StagingXcmV5Instruction> {}

    /** @name StagingXcmV5Instruction (433) */
    interface StagingXcmV5Instruction extends Enum {
        readonly isWithdrawAsset: boolean;
        readonly asWithdrawAsset: StagingXcmV5AssetAssets;
        readonly isReserveAssetDeposited: boolean;
        readonly asReserveAssetDeposited: StagingXcmV5AssetAssets;
        readonly isReceiveTeleportedAsset: boolean;
        readonly asReceiveTeleportedAsset: StagingXcmV5AssetAssets;
        readonly isQueryResponse: boolean;
        readonly asQueryResponse: {
            readonly queryId: Compact<u64>;
            readonly response: StagingXcmV5Response;
            readonly maxWeight: SpWeightsWeightV2Weight;
            readonly querier: Option<StagingXcmV5Location>;
        } & Struct;
        readonly isTransferAsset: boolean;
        readonly asTransferAsset: {
            readonly assets: StagingXcmV5AssetAssets;
            readonly beneficiary: StagingXcmV5Location;
        } & Struct;
        readonly isTransferReserveAsset: boolean;
        readonly asTransferReserveAsset: {
            readonly assets: StagingXcmV5AssetAssets;
            readonly dest: StagingXcmV5Location;
            readonly xcm: StagingXcmV5Xcm;
        } & Struct;
        readonly isTransact: boolean;
        readonly asTransact: {
            readonly originKind: XcmV3OriginKind;
            readonly fallbackMaxWeight: Option<SpWeightsWeightV2Weight>;
            readonly call: XcmDoubleEncoded;
        } & Struct;
        readonly isHrmpNewChannelOpenRequest: boolean;
        readonly asHrmpNewChannelOpenRequest: {
            readonly sender: Compact<u32>;
            readonly maxMessageSize: Compact<u32>;
            readonly maxCapacity: Compact<u32>;
        } & Struct;
        readonly isHrmpChannelAccepted: boolean;
        readonly asHrmpChannelAccepted: {
            readonly recipient: Compact<u32>;
        } & Struct;
        readonly isHrmpChannelClosing: boolean;
        readonly asHrmpChannelClosing: {
            readonly initiator: Compact<u32>;
            readonly sender: Compact<u32>;
            readonly recipient: Compact<u32>;
        } & Struct;
        readonly isClearOrigin: boolean;
        readonly isDescendOrigin: boolean;
        readonly asDescendOrigin: StagingXcmV5Junctions;
        readonly isReportError: boolean;
        readonly asReportError: StagingXcmV5QueryResponseInfo;
        readonly isDepositAsset: boolean;
        readonly asDepositAsset: {
            readonly assets: StagingXcmV5AssetAssetFilter;
            readonly beneficiary: StagingXcmV5Location;
        } & Struct;
        readonly isDepositReserveAsset: boolean;
        readonly asDepositReserveAsset: {
            readonly assets: StagingXcmV5AssetAssetFilter;
            readonly dest: StagingXcmV5Location;
            readonly xcm: StagingXcmV5Xcm;
        } & Struct;
        readonly isExchangeAsset: boolean;
        readonly asExchangeAsset: {
            readonly give: StagingXcmV5AssetAssetFilter;
            readonly want: StagingXcmV5AssetAssets;
            readonly maximal: bool;
        } & Struct;
        readonly isInitiateReserveWithdraw: boolean;
        readonly asInitiateReserveWithdraw: {
            readonly assets: StagingXcmV5AssetAssetFilter;
            readonly reserve: StagingXcmV5Location;
            readonly xcm: StagingXcmV5Xcm;
        } & Struct;
        readonly isInitiateTeleport: boolean;
        readonly asInitiateTeleport: {
            readonly assets: StagingXcmV5AssetAssetFilter;
            readonly dest: StagingXcmV5Location;
            readonly xcm: StagingXcmV5Xcm;
        } & Struct;
        readonly isReportHolding: boolean;
        readonly asReportHolding: {
            readonly responseInfo: StagingXcmV5QueryResponseInfo;
            readonly assets: StagingXcmV5AssetAssetFilter;
        } & Struct;
        readonly isBuyExecution: boolean;
        readonly asBuyExecution: {
            readonly fees: StagingXcmV5Asset;
            readonly weightLimit: XcmV3WeightLimit;
        } & Struct;
        readonly isRefundSurplus: boolean;
        readonly isSetErrorHandler: boolean;
        readonly asSetErrorHandler: StagingXcmV5Xcm;
        readonly isSetAppendix: boolean;
        readonly asSetAppendix: StagingXcmV5Xcm;
        readonly isClearError: boolean;
        readonly isClaimAsset: boolean;
        readonly asClaimAsset: {
            readonly assets: StagingXcmV5AssetAssets;
            readonly ticket: StagingXcmV5Location;
        } & Struct;
        readonly isTrap: boolean;
        readonly asTrap: Compact<u64>;
        readonly isSubscribeVersion: boolean;
        readonly asSubscribeVersion: {
            readonly queryId: Compact<u64>;
            readonly maxResponseWeight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isUnsubscribeVersion: boolean;
        readonly isBurnAsset: boolean;
        readonly asBurnAsset: StagingXcmV5AssetAssets;
        readonly isExpectAsset: boolean;
        readonly asExpectAsset: StagingXcmV5AssetAssets;
        readonly isExpectOrigin: boolean;
        readonly asExpectOrigin: Option<StagingXcmV5Location>;
        readonly isExpectError: boolean;
        readonly asExpectError: Option<ITuple<[u32, XcmV5TraitsError]>>;
        readonly isExpectTransactStatus: boolean;
        readonly asExpectTransactStatus: XcmV3MaybeErrorCode;
        readonly isQueryPallet: boolean;
        readonly asQueryPallet: {
            readonly moduleName: Bytes;
            readonly responseInfo: StagingXcmV5QueryResponseInfo;
        } & Struct;
        readonly isExpectPallet: boolean;
        readonly asExpectPallet: {
            readonly index: Compact<u32>;
            readonly name: Bytes;
            readonly moduleName: Bytes;
            readonly crateMajor: Compact<u32>;
            readonly minCrateMinor: Compact<u32>;
        } & Struct;
        readonly isReportTransactStatus: boolean;
        readonly asReportTransactStatus: StagingXcmV5QueryResponseInfo;
        readonly isClearTransactStatus: boolean;
        readonly isUniversalOrigin: boolean;
        readonly asUniversalOrigin: StagingXcmV5Junction;
        readonly isExportMessage: boolean;
        readonly asExportMessage: {
            readonly network: StagingXcmV5JunctionNetworkId;
            readonly destination: StagingXcmV5Junctions;
            readonly xcm: StagingXcmV5Xcm;
        } & Struct;
        readonly isLockAsset: boolean;
        readonly asLockAsset: {
            readonly asset: StagingXcmV5Asset;
            readonly unlocker: StagingXcmV5Location;
        } & Struct;
        readonly isUnlockAsset: boolean;
        readonly asUnlockAsset: {
            readonly asset: StagingXcmV5Asset;
            readonly target: StagingXcmV5Location;
        } & Struct;
        readonly isNoteUnlockable: boolean;
        readonly asNoteUnlockable: {
            readonly asset: StagingXcmV5Asset;
            readonly owner: StagingXcmV5Location;
        } & Struct;
        readonly isRequestUnlock: boolean;
        readonly asRequestUnlock: {
            readonly asset: StagingXcmV5Asset;
            readonly locker: StagingXcmV5Location;
        } & Struct;
        readonly isSetFeesMode: boolean;
        readonly asSetFeesMode: {
            readonly jitWithdraw: bool;
        } & Struct;
        readonly isSetTopic: boolean;
        readonly asSetTopic: U8aFixed;
        readonly isClearTopic: boolean;
        readonly isAliasOrigin: boolean;
        readonly asAliasOrigin: StagingXcmV5Location;
        readonly isUnpaidExecution: boolean;
        readonly asUnpaidExecution: {
            readonly weightLimit: XcmV3WeightLimit;
            readonly checkOrigin: Option<StagingXcmV5Location>;
        } & Struct;
        readonly isPayFees: boolean;
        readonly asPayFees: {
            readonly asset: StagingXcmV5Asset;
        } & Struct;
        readonly isInitiateTransfer: boolean;
        readonly asInitiateTransfer: {
            readonly destination: StagingXcmV5Location;
            readonly remoteFees: Option<StagingXcmV5AssetAssetTransferFilter>;
            readonly preserveOrigin: bool;
            readonly assets: Vec<StagingXcmV5AssetAssetTransferFilter>;
            readonly remoteXcm: StagingXcmV5Xcm;
        } & Struct;
        readonly isExecuteWithOrigin: boolean;
        readonly asExecuteWithOrigin: {
            readonly descendantOrigin: Option<StagingXcmV5Junctions>;
            readonly xcm: StagingXcmV5Xcm;
        } & Struct;
        readonly isSetHints: boolean;
        readonly asSetHints: {
            readonly hints: Vec<StagingXcmV5Hint>;
        } & Struct;
        readonly type:
            | "WithdrawAsset"
            | "ReserveAssetDeposited"
            | "ReceiveTeleportedAsset"
            | "QueryResponse"
            | "TransferAsset"
            | "TransferReserveAsset"
            | "Transact"
            | "HrmpNewChannelOpenRequest"
            | "HrmpChannelAccepted"
            | "HrmpChannelClosing"
            | "ClearOrigin"
            | "DescendOrigin"
            | "ReportError"
            | "DepositAsset"
            | "DepositReserveAsset"
            | "ExchangeAsset"
            | "InitiateReserveWithdraw"
            | "InitiateTeleport"
            | "ReportHolding"
            | "BuyExecution"
            | "RefundSurplus"
            | "SetErrorHandler"
            | "SetAppendix"
            | "ClearError"
            | "ClaimAsset"
            | "Trap"
            | "SubscribeVersion"
            | "UnsubscribeVersion"
            | "BurnAsset"
            | "ExpectAsset"
            | "ExpectOrigin"
            | "ExpectError"
            | "ExpectTransactStatus"
            | "QueryPallet"
            | "ExpectPallet"
            | "ReportTransactStatus"
            | "ClearTransactStatus"
            | "UniversalOrigin"
            | "ExportMessage"
            | "LockAsset"
            | "UnlockAsset"
            | "NoteUnlockable"
            | "RequestUnlock"
            | "SetFeesMode"
            | "SetTopic"
            | "ClearTopic"
            | "AliasOrigin"
            | "UnpaidExecution"
            | "PayFees"
            | "InitiateTransfer"
            | "ExecuteWithOrigin"
            | "SetHints";
    }

    /** @name StagingXcmV5AssetAssets (434) */
    interface StagingXcmV5AssetAssets extends Vec<StagingXcmV5Asset> {}

    /** @name StagingXcmV5Asset (436) */
    interface StagingXcmV5Asset extends Struct {
        readonly id: StagingXcmV5AssetAssetId;
        readonly fun: StagingXcmV5AssetFungibility;
    }

    /** @name StagingXcmV5AssetAssetId (437) */
    interface StagingXcmV5AssetAssetId extends StagingXcmV5Location {}

    /** @name StagingXcmV5AssetFungibility (438) */
    interface StagingXcmV5AssetFungibility extends Enum {
        readonly isFungible: boolean;
        readonly asFungible: Compact<u128>;
        readonly isNonFungible: boolean;
        readonly asNonFungible: StagingXcmV5AssetAssetInstance;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name StagingXcmV5AssetAssetInstance (439) */
    interface StagingXcmV5AssetAssetInstance extends Enum {
        readonly isUndefined: boolean;
        readonly isIndex: boolean;
        readonly asIndex: Compact<u128>;
        readonly isArray4: boolean;
        readonly asArray4: U8aFixed;
        readonly isArray8: boolean;
        readonly asArray8: U8aFixed;
        readonly isArray16: boolean;
        readonly asArray16: U8aFixed;
        readonly isArray32: boolean;
        readonly asArray32: U8aFixed;
        readonly type: "Undefined" | "Index" | "Array4" | "Array8" | "Array16" | "Array32";
    }

    /** @name StagingXcmV5Response (440) */
    interface StagingXcmV5Response extends Enum {
        readonly isNull: boolean;
        readonly isAssets: boolean;
        readonly asAssets: StagingXcmV5AssetAssets;
        readonly isExecutionResult: boolean;
        readonly asExecutionResult: Option<ITuple<[u32, XcmV5TraitsError]>>;
        readonly isVersion: boolean;
        readonly asVersion: u32;
        readonly isPalletsInfo: boolean;
        readonly asPalletsInfo: Vec<StagingXcmV5PalletInfo>;
        readonly isDispatchResult: boolean;
        readonly asDispatchResult: XcmV3MaybeErrorCode;
        readonly type: "Null" | "Assets" | "ExecutionResult" | "Version" | "PalletsInfo" | "DispatchResult";
    }

    /** @name XcmV5TraitsError (443) */
    interface XcmV5TraitsError extends Enum {
        readonly isOverflow: boolean;
        readonly isUnimplemented: boolean;
        readonly isUntrustedReserveLocation: boolean;
        readonly isUntrustedTeleportLocation: boolean;
        readonly isLocationFull: boolean;
        readonly isLocationNotInvertible: boolean;
        readonly isBadOrigin: boolean;
        readonly isInvalidLocation: boolean;
        readonly isAssetNotFound: boolean;
        readonly isFailedToTransactAsset: boolean;
        readonly isNotWithdrawable: boolean;
        readonly isLocationCannotHold: boolean;
        readonly isExceedsMaxMessageSize: boolean;
        readonly isDestinationUnsupported: boolean;
        readonly isTransport: boolean;
        readonly isUnroutable: boolean;
        readonly isUnknownClaim: boolean;
        readonly isFailedToDecode: boolean;
        readonly isMaxWeightInvalid: boolean;
        readonly isNotHoldingFees: boolean;
        readonly isTooExpensive: boolean;
        readonly isTrap: boolean;
        readonly asTrap: u64;
        readonly isExpectationFalse: boolean;
        readonly isPalletNotFound: boolean;
        readonly isNameMismatch: boolean;
        readonly isVersionIncompatible: boolean;
        readonly isHoldingWouldOverflow: boolean;
        readonly isExportError: boolean;
        readonly isReanchorFailed: boolean;
        readonly isNoDeal: boolean;
        readonly isFeesNotMet: boolean;
        readonly isLockError: boolean;
        readonly isNoPermission: boolean;
        readonly isUnanchored: boolean;
        readonly isNotDepositable: boolean;
        readonly isTooManyAssets: boolean;
        readonly isUnhandledXcmVersion: boolean;
        readonly isWeightLimitReached: boolean;
        readonly asWeightLimitReached: SpWeightsWeightV2Weight;
        readonly isBarrier: boolean;
        readonly isWeightNotComputable: boolean;
        readonly isExceedsStackLimit: boolean;
        readonly type:
            | "Overflow"
            | "Unimplemented"
            | "UntrustedReserveLocation"
            | "UntrustedTeleportLocation"
            | "LocationFull"
            | "LocationNotInvertible"
            | "BadOrigin"
            | "InvalidLocation"
            | "AssetNotFound"
            | "FailedToTransactAsset"
            | "NotWithdrawable"
            | "LocationCannotHold"
            | "ExceedsMaxMessageSize"
            | "DestinationUnsupported"
            | "Transport"
            | "Unroutable"
            | "UnknownClaim"
            | "FailedToDecode"
            | "MaxWeightInvalid"
            | "NotHoldingFees"
            | "TooExpensive"
            | "Trap"
            | "ExpectationFalse"
            | "PalletNotFound"
            | "NameMismatch"
            | "VersionIncompatible"
            | "HoldingWouldOverflow"
            | "ExportError"
            | "ReanchorFailed"
            | "NoDeal"
            | "FeesNotMet"
            | "LockError"
            | "NoPermission"
            | "Unanchored"
            | "NotDepositable"
            | "TooManyAssets"
            | "UnhandledXcmVersion"
            | "WeightLimitReached"
            | "Barrier"
            | "WeightNotComputable"
            | "ExceedsStackLimit";
    }

    /** @name StagingXcmV5PalletInfo (445) */
    interface StagingXcmV5PalletInfo extends Struct {
        readonly index: Compact<u32>;
        readonly name: Bytes;
        readonly moduleName: Bytes;
        readonly major: Compact<u32>;
        readonly minor: Compact<u32>;
        readonly patch: Compact<u32>;
    }

    /** @name StagingXcmV5QueryResponseInfo (450) */
    interface StagingXcmV5QueryResponseInfo extends Struct {
        readonly destination: StagingXcmV5Location;
        readonly queryId: Compact<u64>;
        readonly maxWeight: SpWeightsWeightV2Weight;
    }

    /** @name StagingXcmV5AssetAssetFilter (451) */
    interface StagingXcmV5AssetAssetFilter extends Enum {
        readonly isDefinite: boolean;
        readonly asDefinite: StagingXcmV5AssetAssets;
        readonly isWild: boolean;
        readonly asWild: StagingXcmV5AssetWildAsset;
        readonly type: "Definite" | "Wild";
    }

    /** @name StagingXcmV5AssetWildAsset (452) */
    interface StagingXcmV5AssetWildAsset extends Enum {
        readonly isAll: boolean;
        readonly isAllOf: boolean;
        readonly asAllOf: {
            readonly id: StagingXcmV5AssetAssetId;
            readonly fun: StagingXcmV5AssetWildFungibility;
        } & Struct;
        readonly isAllCounted: boolean;
        readonly asAllCounted: Compact<u32>;
        readonly isAllOfCounted: boolean;
        readonly asAllOfCounted: {
            readonly id: StagingXcmV5AssetAssetId;
            readonly fun: StagingXcmV5AssetWildFungibility;
            readonly count: Compact<u32>;
        } & Struct;
        readonly type: "All" | "AllOf" | "AllCounted" | "AllOfCounted";
    }

    /** @name StagingXcmV5AssetWildFungibility (453) */
    interface StagingXcmV5AssetWildFungibility extends Enum {
        readonly isFungible: boolean;
        readonly isNonFungible: boolean;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name StagingXcmV5AssetAssetTransferFilter (455) */
    interface StagingXcmV5AssetAssetTransferFilter extends Enum {
        readonly isTeleport: boolean;
        readonly asTeleport: StagingXcmV5AssetAssetFilter;
        readonly isReserveDeposit: boolean;
        readonly asReserveDeposit: StagingXcmV5AssetAssetFilter;
        readonly isReserveWithdraw: boolean;
        readonly asReserveWithdraw: StagingXcmV5AssetAssetFilter;
        readonly type: "Teleport" | "ReserveDeposit" | "ReserveWithdraw";
    }

    /** @name StagingXcmV5Hint (460) */
    interface StagingXcmV5Hint extends Enum {
        readonly isAssetClaimer: boolean;
        readonly asAssetClaimer: {
            readonly location: StagingXcmV5Location;
        } & Struct;
        readonly type: "AssetClaimer";
    }

    /** @name XcmVersionedAssets (462) */
    interface XcmVersionedAssets extends Enum {
        readonly isV3: boolean;
        readonly asV3: XcmV3MultiassetMultiAssets;
        readonly isV4: boolean;
        readonly asV4: StagingXcmV4AssetAssets;
        readonly isV5: boolean;
        readonly asV5: StagingXcmV5AssetAssets;
        readonly type: "V3" | "V4" | "V5";
    }

    /** @name StagingXcmExecutorAssetTransferTransferType (474) */
    interface StagingXcmExecutorAssetTransferTransferType extends Enum {
        readonly isTeleport: boolean;
        readonly isLocalReserve: boolean;
        readonly isDestinationReserve: boolean;
        readonly isRemoteReserve: boolean;
        readonly asRemoteReserve: XcmVersionedLocation;
        readonly type: "Teleport" | "LocalReserve" | "DestinationReserve" | "RemoteReserve";
    }

    /** @name XcmVersionedAssetId (475) */
    interface XcmVersionedAssetId extends Enum {
        readonly isV3: boolean;
        readonly asV3: XcmV3MultiassetAssetId;
        readonly isV4: boolean;
        readonly asV4: StagingXcmV4AssetAssetId;
        readonly isV5: boolean;
        readonly asV5: StagingXcmV5AssetAssetId;
        readonly type: "V3" | "V4" | "V5";
    }

    /** @name PalletStreamPaymentCall (477) */
    interface PalletStreamPaymentCall extends Enum {
        readonly isOpenStream: boolean;
        readonly asOpenStream: {
            readonly target: AccountId32;
            readonly config: PalletStreamPaymentStreamConfig;
            readonly initialDeposit: u128;
        } & Struct;
        readonly isCloseStream: boolean;
        readonly asCloseStream: {
            readonly streamId: u64;
        } & Struct;
        readonly isPerformPayment: boolean;
        readonly asPerformPayment: {
            readonly streamId: u64;
        } & Struct;
        readonly isRequestChange: boolean;
        readonly asRequestChange: {
            readonly streamId: u64;
            readonly kind: PalletStreamPaymentChangeKind;
            readonly newConfig: PalletStreamPaymentStreamConfig;
            readonly depositChange: Option<PalletStreamPaymentDepositChange>;
        } & Struct;
        readonly isAcceptRequestedChange: boolean;
        readonly asAcceptRequestedChange: {
            readonly streamId: u64;
            readonly requestNonce: u32;
            readonly depositChange: Option<PalletStreamPaymentDepositChange>;
        } & Struct;
        readonly isCancelChangeRequest: boolean;
        readonly asCancelChangeRequest: {
            readonly streamId: u64;
        } & Struct;
        readonly isImmediatelyChangeDeposit: boolean;
        readonly asImmediatelyChangeDeposit: {
            readonly streamId: u64;
            readonly assetId: TpStreamPaymentCommonAssetId;
            readonly change: PalletStreamPaymentDepositChange;
        } & Struct;
        readonly type:
            | "OpenStream"
            | "CloseStream"
            | "PerformPayment"
            | "RequestChange"
            | "AcceptRequestedChange"
            | "CancelChangeRequest"
            | "ImmediatelyChangeDeposit";
    }

    /** @name PalletStreamPaymentChangeKind (478) */
    interface PalletStreamPaymentChangeKind extends Enum {
        readonly isSuggestion: boolean;
        readonly isMandatory: boolean;
        readonly asMandatory: {
            readonly deadline: u128;
        } & Struct;
        readonly type: "Suggestion" | "Mandatory";
    }

    /** @name PalletStreamPaymentDepositChange (480) */
    interface PalletStreamPaymentDepositChange extends Enum {
        readonly isIncrease: boolean;
        readonly asIncrease: u128;
        readonly isDecrease: boolean;
        readonly asDecrease: u128;
        readonly isAbsolute: boolean;
        readonly asAbsolute: u128;
        readonly type: "Increase" | "Decrease" | "Absolute";
    }

    /** @name PalletMigrationsCall (481) */
    interface PalletMigrationsCall extends Enum {
        readonly isForceSetCursor: boolean;
        readonly asForceSetCursor: {
            readonly cursor: Option<PalletMigrationsMigrationCursor>;
        } & Struct;
        readonly isForceSetActiveCursor: boolean;
        readonly asForceSetActiveCursor: {
            readonly index: u32;
            readonly innerCursor: Option<Bytes>;
            readonly startedAt: Option<u32>;
        } & Struct;
        readonly isForceOnboardMbms: boolean;
        readonly isClearHistoric: boolean;
        readonly asClearHistoric: {
            readonly selector: PalletMigrationsHistoricCleanupSelector;
        } & Struct;
        readonly type: "ForceSetCursor" | "ForceSetActiveCursor" | "ForceOnboardMbms" | "ClearHistoric";
    }

    /** @name PalletMigrationsMigrationCursor (483) */
    interface PalletMigrationsMigrationCursor extends Enum {
        readonly isActive: boolean;
        readonly asActive: PalletMigrationsActiveCursor;
        readonly isStuck: boolean;
        readonly type: "Active" | "Stuck";
    }

    /** @name PalletMigrationsActiveCursor (485) */
    interface PalletMigrationsActiveCursor extends Struct {
        readonly index: u32;
        readonly innerCursor: Option<Bytes>;
        readonly startedAt: u32;
    }

    /** @name PalletMigrationsHistoricCleanupSelector (487) */
    interface PalletMigrationsHistoricCleanupSelector extends Enum {
        readonly isSpecific: boolean;
        readonly asSpecific: Vec<Bytes>;
        readonly isWildcard: boolean;
        readonly asWildcard: {
            readonly limit: Option<u32>;
            readonly previousCursor: Option<Bytes>;
        } & Struct;
        readonly type: "Specific" | "Wildcard";
    }

    /** @name PalletMaintenanceModeCall (491) */
    interface PalletMaintenanceModeCall extends Enum {
        readonly isEnterMaintenanceMode: boolean;
        readonly isResumeNormalOperation: boolean;
        readonly type: "EnterMaintenanceMode" | "ResumeNormalOperation";
    }

    /** @name PalletBeefyCall (492) */
    interface PalletBeefyCall extends Enum {
        readonly isReportDoubleVoting: boolean;
        readonly asReportDoubleVoting: {
            readonly equivocationProof: SpConsensusBeefyDoubleVotingProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly isReportDoubleVotingUnsigned: boolean;
        readonly asReportDoubleVotingUnsigned: {
            readonly equivocationProof: SpConsensusBeefyDoubleVotingProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly isSetNewGenesis: boolean;
        readonly asSetNewGenesis: {
            readonly delayInBlocks: u32;
        } & Struct;
        readonly isReportForkVoting: boolean;
        readonly asReportForkVoting: {
            readonly equivocationProof: SpConsensusBeefyForkVotingProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly isReportForkVotingUnsigned: boolean;
        readonly asReportForkVotingUnsigned: {
            readonly equivocationProof: SpConsensusBeefyForkVotingProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly isReportFutureBlockVoting: boolean;
        readonly asReportFutureBlockVoting: {
            readonly equivocationProof: SpConsensusBeefyFutureBlockVotingProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly isReportFutureBlockVotingUnsigned: boolean;
        readonly asReportFutureBlockVotingUnsigned: {
            readonly equivocationProof: SpConsensusBeefyFutureBlockVotingProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly type:
            | "ReportDoubleVoting"
            | "ReportDoubleVotingUnsigned"
            | "SetNewGenesis"
            | "ReportForkVoting"
            | "ReportForkVotingUnsigned"
            | "ReportFutureBlockVoting"
            | "ReportFutureBlockVotingUnsigned";
    }

    /** @name SpConsensusBeefyDoubleVotingProof (493) */
    interface SpConsensusBeefyDoubleVotingProof extends Struct {
        readonly first: SpConsensusBeefyVoteMessage;
        readonly second: SpConsensusBeefyVoteMessage;
    }

    /** @name SpConsensusBeefyEcdsaCryptoSignature (494) */
    interface SpConsensusBeefyEcdsaCryptoSignature extends U8aFixed {}

    /** @name SpConsensusBeefyVoteMessage (495) */
    interface SpConsensusBeefyVoteMessage extends Struct {
        readonly commitment: SpConsensusBeefyCommitment;
        readonly id: SpConsensusBeefyEcdsaCryptoPublic;
        readonly signature: SpConsensusBeefyEcdsaCryptoSignature;
    }

    /** @name SpConsensusBeefyCommitment (496) */
    interface SpConsensusBeefyCommitment extends Struct {
        readonly payload: SpConsensusBeefyPayload;
        readonly blockNumber: u32;
        readonly validatorSetId: u64;
    }

    /** @name SpConsensusBeefyPayload (497) */
    interface SpConsensusBeefyPayload extends Vec<ITuple<[U8aFixed, Bytes]>> {}

    /** @name SpConsensusBeefyForkVotingProof (500) */
    interface SpConsensusBeefyForkVotingProof extends Struct {
        readonly vote: SpConsensusBeefyVoteMessage;
        readonly ancestryProof: SpMmrPrimitivesAncestryProof;
        readonly header: SpRuntimeHeader;
    }

    /** @name SpMmrPrimitivesAncestryProof (501) */
    interface SpMmrPrimitivesAncestryProof extends Struct {
        readonly prevPeaks: Vec<H256>;
        readonly prevLeafCount: u64;
        readonly leafCount: u64;
        readonly items: Vec<ITuple<[u64, H256]>>;
    }

    /** @name SpConsensusBeefyFutureBlockVotingProof (504) */
    interface SpConsensusBeefyFutureBlockVotingProof extends Struct {
        readonly vote: SpConsensusBeefyVoteMessage;
    }

    /** @name SnowbridgePalletEthereumClientCall (505) */
    interface SnowbridgePalletEthereumClientCall extends Enum {
        readonly isForceCheckpoint: boolean;
        readonly asForceCheckpoint: {
            readonly update: SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate;
        } & Struct;
        readonly isSubmit: boolean;
        readonly asSubmit: {
            readonly update: SnowbridgeBeaconPrimitivesUpdatesUpdate;
        } & Struct;
        readonly isSetOperatingMode: boolean;
        readonly asSetOperatingMode: {
            readonly mode: SnowbridgeCoreOperatingModeBasicOperatingMode;
        } & Struct;
        readonly type: "ForceCheckpoint" | "Submit" | "SetOperatingMode";
    }

    /** @name SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate (506) */
    interface SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate extends Struct {
        readonly header: SnowbridgeBeaconPrimitivesBeaconHeader;
        readonly currentSyncCommittee: SnowbridgeBeaconPrimitivesSyncCommittee;
        readonly currentSyncCommitteeBranch: Vec<H256>;
        readonly validatorsRoot: H256;
        readonly blockRootsRoot: H256;
        readonly blockRootsBranch: Vec<H256>;
    }

    /** @name SnowbridgeBeaconPrimitivesSyncCommittee (507) */
    interface SnowbridgeBeaconPrimitivesSyncCommittee extends Struct {
        readonly pubkeys: Vec<SnowbridgeBeaconPrimitivesPublicKey>;
        readonly aggregatePubkey: SnowbridgeBeaconPrimitivesPublicKey;
    }

    /** @name SnowbridgeBeaconPrimitivesPublicKey (509) */
    interface SnowbridgeBeaconPrimitivesPublicKey extends U8aFixed {}

    /** @name SnowbridgeBeaconPrimitivesUpdatesUpdate (511) */
    interface SnowbridgeBeaconPrimitivesUpdatesUpdate extends Struct {
        readonly attestedHeader: SnowbridgeBeaconPrimitivesBeaconHeader;
        readonly syncAggregate: SnowbridgeBeaconPrimitivesSyncAggregate;
        readonly signatureSlot: u64;
        readonly nextSyncCommitteeUpdate: Option<SnowbridgeBeaconPrimitivesUpdatesNextSyncCommitteeUpdate>;
        readonly finalizedHeader: SnowbridgeBeaconPrimitivesBeaconHeader;
        readonly finalityBranch: Vec<H256>;
        readonly blockRootsRoot: H256;
        readonly blockRootsBranch: Vec<H256>;
    }

    /** @name SnowbridgeBeaconPrimitivesSyncAggregate (512) */
    interface SnowbridgeBeaconPrimitivesSyncAggregate extends Struct {
        readonly syncCommitteeBits: U8aFixed;
        readonly syncCommitteeSignature: SnowbridgeBeaconPrimitivesSignature;
    }

    /** @name SnowbridgeBeaconPrimitivesSignature (513) */
    interface SnowbridgeBeaconPrimitivesSignature extends U8aFixed {}

    /** @name SnowbridgeBeaconPrimitivesUpdatesNextSyncCommitteeUpdate (516) */
    interface SnowbridgeBeaconPrimitivesUpdatesNextSyncCommitteeUpdate extends Struct {
        readonly nextSyncCommittee: SnowbridgeBeaconPrimitivesSyncCommittee;
        readonly nextSyncCommitteeBranch: Vec<H256>;
    }

    /** @name PolkadotRuntimeCommonParasSudoWrapperPalletCall (517) */
    interface PolkadotRuntimeCommonParasSudoWrapperPalletCall extends Enum {
        readonly isSudoScheduleParaInitialize: boolean;
        readonly asSudoScheduleParaInitialize: {
            readonly id: u32;
            readonly genesis: PolkadotRuntimeParachainsParasParaGenesisArgs;
        } & Struct;
        readonly isSudoScheduleParaCleanup: boolean;
        readonly asSudoScheduleParaCleanup: {
            readonly id: u32;
        } & Struct;
        readonly isSudoScheduleParathreadUpgrade: boolean;
        readonly asSudoScheduleParathreadUpgrade: {
            readonly id: u32;
        } & Struct;
        readonly isSudoScheduleParachainDowngrade: boolean;
        readonly asSudoScheduleParachainDowngrade: {
            readonly id: u32;
        } & Struct;
        readonly isSudoQueueDownwardXcm: boolean;
        readonly asSudoQueueDownwardXcm: {
            readonly id: u32;
            readonly xcm: XcmVersionedXcm;
        } & Struct;
        readonly isSudoEstablishHrmpChannel: boolean;
        readonly asSudoEstablishHrmpChannel: {
            readonly sender: u32;
            readonly recipient: u32;
            readonly maxCapacity: u32;
            readonly maxMessageSize: u32;
        } & Struct;
        readonly type:
            | "SudoScheduleParaInitialize"
            | "SudoScheduleParaCleanup"
            | "SudoScheduleParathreadUpgrade"
            | "SudoScheduleParachainDowngrade"
            | "SudoQueueDownwardXcm"
            | "SudoEstablishHrmpChannel";
    }

    /** @name PolkadotRuntimeParachainsParasParaGenesisArgs (518) */
    interface PolkadotRuntimeParachainsParasParaGenesisArgs extends Struct {
        readonly genesisHead: Bytes;
        readonly validationCode: Bytes;
        readonly paraKind: bool;
    }

    /** @name PalletRootTestingCall (519) */
    interface PalletRootTestingCall extends Enum {
        readonly isFillBlock: boolean;
        readonly asFillBlock: {
            readonly ratio: Perbill;
        } & Struct;
        readonly isTriggerDefensive: boolean;
        readonly type: "FillBlock" | "TriggerDefensive";
    }

    /** @name PalletSudoCall (520) */
    interface PalletSudoCall extends Enum {
        readonly isSudo: boolean;
        readonly asSudo: {
            readonly call: Call;
        } & Struct;
        readonly isSudoUncheckedWeight: boolean;
        readonly asSudoUncheckedWeight: {
            readonly call: Call;
            readonly weight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isSetKey: boolean;
        readonly asSetKey: {
            readonly new_: MultiAddress;
        } & Struct;
        readonly isSudoAs: boolean;
        readonly asSudoAs: {
            readonly who: MultiAddress;
            readonly call: Call;
        } & Struct;
        readonly isRemoveKey: boolean;
        readonly type: "Sudo" | "SudoUncheckedWeight" | "SetKey" | "SudoAs" | "RemoveKey";
    }

    /** @name SpRuntimeBlakeTwo256 (521) */
    type SpRuntimeBlakeTwo256 = Null;

    /** @name PalletConvictionVotingTally (523) */
    interface PalletConvictionVotingTally extends Struct {
        readonly ayes: u128;
        readonly nays: u128;
        readonly support: u128;
    }

    /** @name PalletRankedCollectiveEvent (524) */
    interface PalletRankedCollectiveEvent extends Enum {
        readonly isMemberAdded: boolean;
        readonly asMemberAdded: {
            readonly who: AccountId32;
        } & Struct;
        readonly isRankChanged: boolean;
        readonly asRankChanged: {
            readonly who: AccountId32;
            readonly rank: u16;
        } & Struct;
        readonly isMemberRemoved: boolean;
        readonly asMemberRemoved: {
            readonly who: AccountId32;
            readonly rank: u16;
        } & Struct;
        readonly isVoted: boolean;
        readonly asVoted: {
            readonly who: AccountId32;
            readonly poll: u32;
            readonly vote: PalletRankedCollectiveVoteRecord;
            readonly tally: PalletRankedCollectiveTally;
        } & Struct;
        readonly isMemberExchanged: boolean;
        readonly asMemberExchanged: {
            readonly who: AccountId32;
            readonly newWho: AccountId32;
        } & Struct;
        readonly type: "MemberAdded" | "RankChanged" | "MemberRemoved" | "Voted" | "MemberExchanged";
    }

    /** @name PalletRankedCollectiveVoteRecord (525) */
    interface PalletRankedCollectiveVoteRecord extends Enum {
        readonly isAye: boolean;
        readonly asAye: u32;
        readonly isNay: boolean;
        readonly asNay: u32;
        readonly type: "Aye" | "Nay";
    }

    /** @name PalletRankedCollectiveTally (526) */
    interface PalletRankedCollectiveTally extends Struct {
        readonly bareAyes: u32;
        readonly ayes: u32;
        readonly nays: u32;
    }

    /** @name PalletWhitelistEvent (528) */
    interface PalletWhitelistEvent extends Enum {
        readonly isCallWhitelisted: boolean;
        readonly asCallWhitelisted: {
            readonly callHash: H256;
        } & Struct;
        readonly isWhitelistedCallRemoved: boolean;
        readonly asWhitelistedCallRemoved: {
            readonly callHash: H256;
        } & Struct;
        readonly isWhitelistedCallDispatched: boolean;
        readonly asWhitelistedCallDispatched: {
            readonly callHash: H256;
            readonly result: Result<FrameSupportDispatchPostDispatchInfo, SpRuntimeDispatchErrorWithPostInfo>;
        } & Struct;
        readonly type: "CallWhitelisted" | "WhitelistedCallRemoved" | "WhitelistedCallDispatched";
    }

    /** @name FrameSupportDispatchPostDispatchInfo (530) */
    interface FrameSupportDispatchPostDispatchInfo extends Struct {
        readonly actualWeight: Option<SpWeightsWeightV2Weight>;
        readonly paysFee: FrameSupportDispatchPays;
    }

    /** @name SpRuntimeDispatchErrorWithPostInfo (531) */
    interface SpRuntimeDispatchErrorWithPostInfo extends Struct {
        readonly postInfo: FrameSupportDispatchPostDispatchInfo;
        readonly error: SpRuntimeDispatchError;
    }

    /** @name PalletCollectiveEvent (532) */
    interface PalletCollectiveEvent extends Enum {
        readonly isProposed: boolean;
        readonly asProposed: {
            readonly account: AccountId32;
            readonly proposalIndex: u32;
            readonly proposalHash: H256;
            readonly threshold: u32;
        } & Struct;
        readonly isVoted: boolean;
        readonly asVoted: {
            readonly account: AccountId32;
            readonly proposalHash: H256;
            readonly voted: bool;
            readonly yes: u32;
            readonly no: u32;
        } & Struct;
        readonly isApproved: boolean;
        readonly asApproved: {
            readonly proposalHash: H256;
        } & Struct;
        readonly isDisapproved: boolean;
        readonly asDisapproved: {
            readonly proposalHash: H256;
        } & Struct;
        readonly isExecuted: boolean;
        readonly asExecuted: {
            readonly proposalHash: H256;
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isMemberExecuted: boolean;
        readonly asMemberExecuted: {
            readonly proposalHash: H256;
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isClosed: boolean;
        readonly asClosed: {
            readonly proposalHash: H256;
            readonly yes: u32;
            readonly no: u32;
        } & Struct;
        readonly isKilled: boolean;
        readonly asKilled: {
            readonly proposalHash: H256;
        } & Struct;
        readonly isProposalCostBurned: boolean;
        readonly asProposalCostBurned: {
            readonly proposalHash: H256;
            readonly who: AccountId32;
        } & Struct;
        readonly isProposalCostReleased: boolean;
        readonly asProposalCostReleased: {
            readonly proposalHash: H256;
            readonly who: AccountId32;
        } & Struct;
        readonly type:
            | "Proposed"
            | "Voted"
            | "Approved"
            | "Disapproved"
            | "Executed"
            | "MemberExecuted"
            | "Closed"
            | "Killed"
            | "ProposalCostBurned"
            | "ProposalCostReleased";
    }

    /** @name PolkadotRuntimeParachainsInclusionPalletEvent (534) */
    interface PolkadotRuntimeParachainsInclusionPalletEvent extends Enum {
        readonly isCandidateBacked: boolean;
        readonly asCandidateBacked: ITuple<[PolkadotPrimitivesVstagingCandidateReceiptV2, Bytes, u32, u32]>;
        readonly isCandidateIncluded: boolean;
        readonly asCandidateIncluded: ITuple<[PolkadotPrimitivesVstagingCandidateReceiptV2, Bytes, u32, u32]>;
        readonly isCandidateTimedOut: boolean;
        readonly asCandidateTimedOut: ITuple<[PolkadotPrimitivesVstagingCandidateReceiptV2, Bytes, u32]>;
        readonly isUpwardMessagesReceived: boolean;
        readonly asUpwardMessagesReceived: {
            readonly from: u32;
            readonly count: u32;
        } & Struct;
        readonly type: "CandidateBacked" | "CandidateIncluded" | "CandidateTimedOut" | "UpwardMessagesReceived";
    }

    /** @name PolkadotPrimitivesVstagingCandidateReceiptV2 (535) */
    interface PolkadotPrimitivesVstagingCandidateReceiptV2 extends Struct {
        readonly descriptor: PolkadotPrimitivesVstagingCandidateDescriptorV2;
        readonly commitmentsHash: H256;
    }

    /** @name PolkadotRuntimeParachainsParasPalletEvent (538) */
    interface PolkadotRuntimeParachainsParasPalletEvent extends Enum {
        readonly isCurrentCodeUpdated: boolean;
        readonly asCurrentCodeUpdated: u32;
        readonly isCurrentHeadUpdated: boolean;
        readonly asCurrentHeadUpdated: u32;
        readonly isCodeUpgradeScheduled: boolean;
        readonly asCodeUpgradeScheduled: u32;
        readonly isNewHeadNoted: boolean;
        readonly asNewHeadNoted: u32;
        readonly isActionQueued: boolean;
        readonly asActionQueued: ITuple<[u32, u32]>;
        readonly isPvfCheckStarted: boolean;
        readonly asPvfCheckStarted: ITuple<[H256, u32]>;
        readonly isPvfCheckAccepted: boolean;
        readonly asPvfCheckAccepted: ITuple<[H256, u32]>;
        readonly isPvfCheckRejected: boolean;
        readonly asPvfCheckRejected: ITuple<[H256, u32]>;
        readonly isUpgradeCooldownRemoved: boolean;
        readonly asUpgradeCooldownRemoved: {
            readonly paraId: u32;
        } & Struct;
        readonly isCodeAuthorized: boolean;
        readonly asCodeAuthorized: {
            readonly paraId: u32;
            readonly codeHash: H256;
            readonly expireAt: u32;
        } & Struct;
        readonly type:
            | "CurrentCodeUpdated"
            | "CurrentHeadUpdated"
            | "CodeUpgradeScheduled"
            | "NewHeadNoted"
            | "ActionQueued"
            | "PvfCheckStarted"
            | "PvfCheckAccepted"
            | "PvfCheckRejected"
            | "UpgradeCooldownRemoved"
            | "CodeAuthorized";
    }

    /** @name PolkadotRuntimeParachainsHrmpPalletEvent (539) */
    interface PolkadotRuntimeParachainsHrmpPalletEvent extends Enum {
        readonly isOpenChannelRequested: boolean;
        readonly asOpenChannelRequested: {
            readonly sender: u32;
            readonly recipient: u32;
            readonly proposedMaxCapacity: u32;
            readonly proposedMaxMessageSize: u32;
        } & Struct;
        readonly isOpenChannelCanceled: boolean;
        readonly asOpenChannelCanceled: {
            readonly byParachain: u32;
            readonly channelId: PolkadotParachainPrimitivesPrimitivesHrmpChannelId;
        } & Struct;
        readonly isOpenChannelAccepted: boolean;
        readonly asOpenChannelAccepted: {
            readonly sender: u32;
            readonly recipient: u32;
        } & Struct;
        readonly isChannelClosed: boolean;
        readonly asChannelClosed: {
            readonly byParachain: u32;
            readonly channelId: PolkadotParachainPrimitivesPrimitivesHrmpChannelId;
        } & Struct;
        readonly isHrmpChannelForceOpened: boolean;
        readonly asHrmpChannelForceOpened: {
            readonly sender: u32;
            readonly recipient: u32;
            readonly proposedMaxCapacity: u32;
            readonly proposedMaxMessageSize: u32;
        } & Struct;
        readonly isHrmpSystemChannelOpened: boolean;
        readonly asHrmpSystemChannelOpened: {
            readonly sender: u32;
            readonly recipient: u32;
            readonly proposedMaxCapacity: u32;
            readonly proposedMaxMessageSize: u32;
        } & Struct;
        readonly isOpenChannelDepositsUpdated: boolean;
        readonly asOpenChannelDepositsUpdated: {
            readonly sender: u32;
            readonly recipient: u32;
        } & Struct;
        readonly type:
            | "OpenChannelRequested"
            | "OpenChannelCanceled"
            | "OpenChannelAccepted"
            | "ChannelClosed"
            | "HrmpChannelForceOpened"
            | "HrmpSystemChannelOpened"
            | "OpenChannelDepositsUpdated";
    }

    /** @name PolkadotRuntimeParachainsDisputesPalletEvent (540) */
    interface PolkadotRuntimeParachainsDisputesPalletEvent extends Enum {
        readonly isDisputeInitiated: boolean;
        readonly asDisputeInitiated: ITuple<[H256, PolkadotRuntimeParachainsDisputesDisputeLocation]>;
        readonly isDisputeConcluded: boolean;
        readonly asDisputeConcluded: ITuple<[H256, PolkadotRuntimeParachainsDisputesDisputeResult]>;
        readonly isRevert: boolean;
        readonly asRevert: u32;
        readonly type: "DisputeInitiated" | "DisputeConcluded" | "Revert";
    }

    /** @name PolkadotRuntimeParachainsDisputesDisputeLocation (541) */
    interface PolkadotRuntimeParachainsDisputesDisputeLocation extends Enum {
        readonly isLocal: boolean;
        readonly isRemote: boolean;
        readonly type: "Local" | "Remote";
    }

    /** @name PolkadotRuntimeParachainsDisputesDisputeResult (542) */
    interface PolkadotRuntimeParachainsDisputesDisputeResult extends Enum {
        readonly isValid: boolean;
        readonly isInvalid: boolean;
        readonly type: "Valid" | "Invalid";
    }

    /** @name PalletMessageQueueEvent (543) */
    interface PalletMessageQueueEvent extends Enum {
        readonly isProcessingFailed: boolean;
        readonly asProcessingFailed: {
            readonly id: H256;
            readonly origin: DancelightRuntimeAggregateMessageOrigin;
            readonly error: FrameSupportMessagesProcessMessageError;
        } & Struct;
        readonly isProcessed: boolean;
        readonly asProcessed: {
            readonly id: H256;
            readonly origin: DancelightRuntimeAggregateMessageOrigin;
            readonly weightUsed: SpWeightsWeightV2Weight;
            readonly success: bool;
        } & Struct;
        readonly isOverweightEnqueued: boolean;
        readonly asOverweightEnqueued: {
            readonly id: U8aFixed;
            readonly origin: DancelightRuntimeAggregateMessageOrigin;
            readonly pageIndex: u32;
            readonly messageIndex: u32;
        } & Struct;
        readonly isPageReaped: boolean;
        readonly asPageReaped: {
            readonly origin: DancelightRuntimeAggregateMessageOrigin;
            readonly index: u32;
        } & Struct;
        readonly type: "ProcessingFailed" | "Processed" | "OverweightEnqueued" | "PageReaped";
    }

    /** @name FrameSupportMessagesProcessMessageError (544) */
    interface FrameSupportMessagesProcessMessageError extends Enum {
        readonly isBadFormat: boolean;
        readonly isCorrupt: boolean;
        readonly isUnsupported: boolean;
        readonly isOverweight: boolean;
        readonly asOverweight: SpWeightsWeightV2Weight;
        readonly isYield: boolean;
        readonly isStackLimitReached: boolean;
        readonly type: "BadFormat" | "Corrupt" | "Unsupported" | "Overweight" | "Yield" | "StackLimitReached";
    }

    /** @name PolkadotRuntimeParachainsOnDemandPalletEvent (545) */
    interface PolkadotRuntimeParachainsOnDemandPalletEvent extends Enum {
        readonly isOnDemandOrderPlaced: boolean;
        readonly asOnDemandOrderPlaced: {
            readonly paraId: u32;
            readonly spotPrice: u128;
            readonly orderedBy: AccountId32;
        } & Struct;
        readonly isSpotPriceSet: boolean;
        readonly asSpotPriceSet: {
            readonly spotPrice: u128;
        } & Struct;
        readonly isAccountCredited: boolean;
        readonly asAccountCredited: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly type: "OnDemandOrderPlaced" | "SpotPriceSet" | "AccountCredited";
    }

    /** @name PolkadotRuntimeCommonParasRegistrarPalletEvent (546) */
    interface PolkadotRuntimeCommonParasRegistrarPalletEvent extends Enum {
        readonly isRegistered: boolean;
        readonly asRegistered: {
            readonly paraId: u32;
            readonly manager: AccountId32;
        } & Struct;
        readonly isDeregistered: boolean;
        readonly asDeregistered: {
            readonly paraId: u32;
        } & Struct;
        readonly isReserved: boolean;
        readonly asReserved: {
            readonly paraId: u32;
            readonly who: AccountId32;
        } & Struct;
        readonly isSwapped: boolean;
        readonly asSwapped: {
            readonly paraId: u32;
            readonly otherId: u32;
        } & Struct;
        readonly type: "Registered" | "Deregistered" | "Reserved" | "Swapped";
    }

    /** @name PalletUtilityEvent (547) */
    interface PalletUtilityEvent extends Enum {
        readonly isBatchInterrupted: boolean;
        readonly asBatchInterrupted: {
            readonly index: u32;
            readonly error: SpRuntimeDispatchError;
        } & Struct;
        readonly isBatchCompleted: boolean;
        readonly isBatchCompletedWithErrors: boolean;
        readonly isItemCompleted: boolean;
        readonly isItemFailed: boolean;
        readonly asItemFailed: {
            readonly error: SpRuntimeDispatchError;
        } & Struct;
        readonly isDispatchedAs: boolean;
        readonly asDispatchedAs: {
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isIfElseMainSuccess: boolean;
        readonly isIfElseFallbackCalled: boolean;
        readonly asIfElseFallbackCalled: {
            readonly mainError: SpRuntimeDispatchError;
        } & Struct;
        readonly type:
            | "BatchInterrupted"
            | "BatchCompleted"
            | "BatchCompletedWithErrors"
            | "ItemCompleted"
            | "ItemFailed"
            | "DispatchedAs"
            | "IfElseMainSuccess"
            | "IfElseFallbackCalled";
    }

    /** @name PalletIdentityEvent (548) */
    interface PalletIdentityEvent extends Enum {
        readonly isIdentitySet: boolean;
        readonly asIdentitySet: {
            readonly who: AccountId32;
        } & Struct;
        readonly isIdentityCleared: boolean;
        readonly asIdentityCleared: {
            readonly who: AccountId32;
            readonly deposit: u128;
        } & Struct;
        readonly isIdentityKilled: boolean;
        readonly asIdentityKilled: {
            readonly who: AccountId32;
            readonly deposit: u128;
        } & Struct;
        readonly isJudgementRequested: boolean;
        readonly asJudgementRequested: {
            readonly who: AccountId32;
            readonly registrarIndex: u32;
        } & Struct;
        readonly isJudgementUnrequested: boolean;
        readonly asJudgementUnrequested: {
            readonly who: AccountId32;
            readonly registrarIndex: u32;
        } & Struct;
        readonly isJudgementGiven: boolean;
        readonly asJudgementGiven: {
            readonly target: AccountId32;
            readonly registrarIndex: u32;
        } & Struct;
        readonly isRegistrarAdded: boolean;
        readonly asRegistrarAdded: {
            readonly registrarIndex: u32;
        } & Struct;
        readonly isSubIdentityAdded: boolean;
        readonly asSubIdentityAdded: {
            readonly sub: AccountId32;
            readonly main: AccountId32;
            readonly deposit: u128;
        } & Struct;
        readonly isSubIdentitiesSet: boolean;
        readonly asSubIdentitiesSet: {
            readonly main: AccountId32;
            readonly numberOfSubs: u32;
            readonly newDeposit: u128;
        } & Struct;
        readonly isSubIdentityRenamed: boolean;
        readonly asSubIdentityRenamed: {
            readonly sub: AccountId32;
            readonly main: AccountId32;
        } & Struct;
        readonly isSubIdentityRemoved: boolean;
        readonly asSubIdentityRemoved: {
            readonly sub: AccountId32;
            readonly main: AccountId32;
            readonly deposit: u128;
        } & Struct;
        readonly isSubIdentityRevoked: boolean;
        readonly asSubIdentityRevoked: {
            readonly sub: AccountId32;
            readonly main: AccountId32;
            readonly deposit: u128;
        } & Struct;
        readonly isAuthorityAdded: boolean;
        readonly asAuthorityAdded: {
            readonly authority: AccountId32;
        } & Struct;
        readonly isAuthorityRemoved: boolean;
        readonly asAuthorityRemoved: {
            readonly authority: AccountId32;
        } & Struct;
        readonly isUsernameSet: boolean;
        readonly asUsernameSet: {
            readonly who: AccountId32;
            readonly username: Bytes;
        } & Struct;
        readonly isUsernameQueued: boolean;
        readonly asUsernameQueued: {
            readonly who: AccountId32;
            readonly username: Bytes;
            readonly expiration: u32;
        } & Struct;
        readonly isPreapprovalExpired: boolean;
        readonly asPreapprovalExpired: {
            readonly whose: AccountId32;
        } & Struct;
        readonly isPrimaryUsernameSet: boolean;
        readonly asPrimaryUsernameSet: {
            readonly who: AccountId32;
            readonly username: Bytes;
        } & Struct;
        readonly isDanglingUsernameRemoved: boolean;
        readonly asDanglingUsernameRemoved: {
            readonly who: AccountId32;
            readonly username: Bytes;
        } & Struct;
        readonly isUsernameUnbound: boolean;
        readonly asUsernameUnbound: {
            readonly username: Bytes;
        } & Struct;
        readonly isUsernameRemoved: boolean;
        readonly asUsernameRemoved: {
            readonly username: Bytes;
        } & Struct;
        readonly isUsernameKilled: boolean;
        readonly asUsernameKilled: {
            readonly username: Bytes;
        } & Struct;
        readonly type:
            | "IdentitySet"
            | "IdentityCleared"
            | "IdentityKilled"
            | "JudgementRequested"
            | "JudgementUnrequested"
            | "JudgementGiven"
            | "RegistrarAdded"
            | "SubIdentityAdded"
            | "SubIdentitiesSet"
            | "SubIdentityRenamed"
            | "SubIdentityRemoved"
            | "SubIdentityRevoked"
            | "AuthorityAdded"
            | "AuthorityRemoved"
            | "UsernameSet"
            | "UsernameQueued"
            | "PreapprovalExpired"
            | "PrimaryUsernameSet"
            | "DanglingUsernameRemoved"
            | "UsernameUnbound"
            | "UsernameRemoved"
            | "UsernameKilled";
    }

    /** @name PalletSchedulerEvent (549) */
    interface PalletSchedulerEvent extends Enum {
        readonly isScheduled: boolean;
        readonly asScheduled: {
            readonly when: u32;
            readonly index: u32;
        } & Struct;
        readonly isCanceled: boolean;
        readonly asCanceled: {
            readonly when: u32;
            readonly index: u32;
        } & Struct;
        readonly isDispatched: boolean;
        readonly asDispatched: {
            readonly task: ITuple<[u32, u32]>;
            readonly id: Option<U8aFixed>;
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isRetrySet: boolean;
        readonly asRetrySet: {
            readonly task: ITuple<[u32, u32]>;
            readonly id: Option<U8aFixed>;
            readonly period: u32;
            readonly retries: u8;
        } & Struct;
        readonly isRetryCancelled: boolean;
        readonly asRetryCancelled: {
            readonly task: ITuple<[u32, u32]>;
            readonly id: Option<U8aFixed>;
        } & Struct;
        readonly isCallUnavailable: boolean;
        readonly asCallUnavailable: {
            readonly task: ITuple<[u32, u32]>;
            readonly id: Option<U8aFixed>;
        } & Struct;
        readonly isPeriodicFailed: boolean;
        readonly asPeriodicFailed: {
            readonly task: ITuple<[u32, u32]>;
            readonly id: Option<U8aFixed>;
        } & Struct;
        readonly isRetryFailed: boolean;
        readonly asRetryFailed: {
            readonly task: ITuple<[u32, u32]>;
            readonly id: Option<U8aFixed>;
        } & Struct;
        readonly isPermanentlyOverweight: boolean;
        readonly asPermanentlyOverweight: {
            readonly task: ITuple<[u32, u32]>;
            readonly id: Option<U8aFixed>;
        } & Struct;
        readonly isAgendaIncomplete: boolean;
        readonly asAgendaIncomplete: {
            readonly when: u32;
        } & Struct;
        readonly type:
            | "Scheduled"
            | "Canceled"
            | "Dispatched"
            | "RetrySet"
            | "RetryCancelled"
            | "CallUnavailable"
            | "PeriodicFailed"
            | "RetryFailed"
            | "PermanentlyOverweight"
            | "AgendaIncomplete";
    }

    /** @name PalletProxyEvent (551) */
    interface PalletProxyEvent extends Enum {
        readonly isProxyExecuted: boolean;
        readonly asProxyExecuted: {
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isPureCreated: boolean;
        readonly asPureCreated: {
            readonly pure: AccountId32;
            readonly who: AccountId32;
            readonly proxyType: DancelightRuntimeProxyType;
            readonly disambiguationIndex: u16;
        } & Struct;
        readonly isPureKilled: boolean;
        readonly asPureKilled: {
            readonly pure: AccountId32;
            readonly spawner: AccountId32;
            readonly proxyType: DancelightRuntimeProxyType;
            readonly disambiguationIndex: u16;
        } & Struct;
        readonly isAnnounced: boolean;
        readonly asAnnounced: {
            readonly real: AccountId32;
            readonly proxy: AccountId32;
            readonly callHash: H256;
        } & Struct;
        readonly isProxyAdded: boolean;
        readonly asProxyAdded: {
            readonly delegator: AccountId32;
            readonly delegatee: AccountId32;
            readonly proxyType: DancelightRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly isProxyRemoved: boolean;
        readonly asProxyRemoved: {
            readonly delegator: AccountId32;
            readonly delegatee: AccountId32;
            readonly proxyType: DancelightRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly isDepositPoked: boolean;
        readonly asDepositPoked: {
            readonly who: AccountId32;
            readonly kind: PalletProxyDepositKind;
            readonly oldDeposit: u128;
            readonly newDeposit: u128;
        } & Struct;
        readonly type:
            | "ProxyExecuted"
            | "PureCreated"
            | "PureKilled"
            | "Announced"
            | "ProxyAdded"
            | "ProxyRemoved"
            | "DepositPoked";
    }

    /** @name PalletProxyDepositKind (552) */
    interface PalletProxyDepositKind extends Enum {
        readonly isProxies: boolean;
        readonly isAnnouncements: boolean;
        readonly type: "Proxies" | "Announcements";
    }

    /** @name PalletMultisigEvent (553) */
    interface PalletMultisigEvent extends Enum {
        readonly isNewMultisig: boolean;
        readonly asNewMultisig: {
            readonly approving: AccountId32;
            readonly multisig: AccountId32;
            readonly callHash: U8aFixed;
        } & Struct;
        readonly isMultisigApproval: boolean;
        readonly asMultisigApproval: {
            readonly approving: AccountId32;
            readonly timepoint: PalletMultisigTimepoint;
            readonly multisig: AccountId32;
            readonly callHash: U8aFixed;
        } & Struct;
        readonly isMultisigExecuted: boolean;
        readonly asMultisigExecuted: {
            readonly approving: AccountId32;
            readonly timepoint: PalletMultisigTimepoint;
            readonly multisig: AccountId32;
            readonly callHash: U8aFixed;
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isMultisigCancelled: boolean;
        readonly asMultisigCancelled: {
            readonly cancelling: AccountId32;
            readonly timepoint: PalletMultisigTimepoint;
            readonly multisig: AccountId32;
            readonly callHash: U8aFixed;
        } & Struct;
        readonly isDepositPoked: boolean;
        readonly asDepositPoked: {
            readonly who: AccountId32;
            readonly callHash: U8aFixed;
            readonly oldDeposit: u128;
            readonly newDeposit: u128;
        } & Struct;
        readonly type: "NewMultisig" | "MultisigApproval" | "MultisigExecuted" | "MultisigCancelled" | "DepositPoked";
    }

    /** @name PalletPreimageEvent (554) */
    interface PalletPreimageEvent extends Enum {
        readonly isNoted: boolean;
        readonly asNoted: {
            readonly hash_: H256;
        } & Struct;
        readonly isRequested: boolean;
        readonly asRequested: {
            readonly hash_: H256;
        } & Struct;
        readonly isCleared: boolean;
        readonly asCleared: {
            readonly hash_: H256;
        } & Struct;
        readonly type: "Noted" | "Requested" | "Cleared";
    }

    /** @name PalletAssetRateEvent (555) */
    interface PalletAssetRateEvent extends Enum {
        readonly isAssetRateCreated: boolean;
        readonly asAssetRateCreated: {
            readonly assetKind: Null;
            readonly rate: u128;
        } & Struct;
        readonly isAssetRateRemoved: boolean;
        readonly asAssetRateRemoved: {
            readonly assetKind: Null;
        } & Struct;
        readonly isAssetRateUpdated: boolean;
        readonly asAssetRateUpdated: {
            readonly assetKind: Null;
            readonly old: u128;
            readonly new_: u128;
        } & Struct;
        readonly type: "AssetRateCreated" | "AssetRateRemoved" | "AssetRateUpdated";
    }

    /** @name PalletAssetsEvent (556) */
    interface PalletAssetsEvent extends Enum {
        readonly isCreated: boolean;
        readonly asCreated: {
            readonly assetId: u16;
            readonly creator: AccountId32;
            readonly owner: AccountId32;
        } & Struct;
        readonly isIssued: boolean;
        readonly asIssued: {
            readonly assetId: u16;
            readonly owner: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isTransferred: boolean;
        readonly asTransferred: {
            readonly assetId: u16;
            readonly from: AccountId32;
            readonly to: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isBurned: boolean;
        readonly asBurned: {
            readonly assetId: u16;
            readonly owner: AccountId32;
            readonly balance: u128;
        } & Struct;
        readonly isTeamChanged: boolean;
        readonly asTeamChanged: {
            readonly assetId: u16;
            readonly issuer: AccountId32;
            readonly admin: AccountId32;
            readonly freezer: AccountId32;
        } & Struct;
        readonly isOwnerChanged: boolean;
        readonly asOwnerChanged: {
            readonly assetId: u16;
            readonly owner: AccountId32;
        } & Struct;
        readonly isFrozen: boolean;
        readonly asFrozen: {
            readonly assetId: u16;
            readonly who: AccountId32;
        } & Struct;
        readonly isThawed: boolean;
        readonly asThawed: {
            readonly assetId: u16;
            readonly who: AccountId32;
        } & Struct;
        readonly isAssetFrozen: boolean;
        readonly asAssetFrozen: {
            readonly assetId: u16;
        } & Struct;
        readonly isAssetThawed: boolean;
        readonly asAssetThawed: {
            readonly assetId: u16;
        } & Struct;
        readonly isAccountsDestroyed: boolean;
        readonly asAccountsDestroyed: {
            readonly assetId: u16;
            readonly accountsDestroyed: u32;
            readonly accountsRemaining: u32;
        } & Struct;
        readonly isApprovalsDestroyed: boolean;
        readonly asApprovalsDestroyed: {
            readonly assetId: u16;
            readonly approvalsDestroyed: u32;
            readonly approvalsRemaining: u32;
        } & Struct;
        readonly isDestructionStarted: boolean;
        readonly asDestructionStarted: {
            readonly assetId: u16;
        } & Struct;
        readonly isDestroyed: boolean;
        readonly asDestroyed: {
            readonly assetId: u16;
        } & Struct;
        readonly isForceCreated: boolean;
        readonly asForceCreated: {
            readonly assetId: u16;
            readonly owner: AccountId32;
        } & Struct;
        readonly isMetadataSet: boolean;
        readonly asMetadataSet: {
            readonly assetId: u16;
            readonly name: Bytes;
            readonly symbol: Bytes;
            readonly decimals: u8;
            readonly isFrozen: bool;
        } & Struct;
        readonly isMetadataCleared: boolean;
        readonly asMetadataCleared: {
            readonly assetId: u16;
        } & Struct;
        readonly isApprovedTransfer: boolean;
        readonly asApprovedTransfer: {
            readonly assetId: u16;
            readonly source: AccountId32;
            readonly delegate: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isApprovalCancelled: boolean;
        readonly asApprovalCancelled: {
            readonly assetId: u16;
            readonly owner: AccountId32;
            readonly delegate: AccountId32;
        } & Struct;
        readonly isTransferredApproved: boolean;
        readonly asTransferredApproved: {
            readonly assetId: u16;
            readonly owner: AccountId32;
            readonly delegate: AccountId32;
            readonly destination: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isAssetStatusChanged: boolean;
        readonly asAssetStatusChanged: {
            readonly assetId: u16;
        } & Struct;
        readonly isAssetMinBalanceChanged: boolean;
        readonly asAssetMinBalanceChanged: {
            readonly assetId: u16;
            readonly newMinBalance: u128;
        } & Struct;
        readonly isTouched: boolean;
        readonly asTouched: {
            readonly assetId: u16;
            readonly who: AccountId32;
            readonly depositor: AccountId32;
        } & Struct;
        readonly isBlocked: boolean;
        readonly asBlocked: {
            readonly assetId: u16;
            readonly who: AccountId32;
        } & Struct;
        readonly isDeposited: boolean;
        readonly asDeposited: {
            readonly assetId: u16;
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isWithdrawn: boolean;
        readonly asWithdrawn: {
            readonly assetId: u16;
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly type:
            | "Created"
            | "Issued"
            | "Transferred"
            | "Burned"
            | "TeamChanged"
            | "OwnerChanged"
            | "Frozen"
            | "Thawed"
            | "AssetFrozen"
            | "AssetThawed"
            | "AccountsDestroyed"
            | "ApprovalsDestroyed"
            | "DestructionStarted"
            | "Destroyed"
            | "ForceCreated"
            | "MetadataSet"
            | "MetadataCleared"
            | "ApprovedTransfer"
            | "ApprovalCancelled"
            | "TransferredApproved"
            | "AssetStatusChanged"
            | "AssetMinBalanceChanged"
            | "Touched"
            | "Blocked"
            | "Deposited"
            | "Withdrawn";
    }

    /** @name PalletForeignAssetCreatorEvent (557) */
    interface PalletForeignAssetCreatorEvent extends Enum {
        readonly isForeignAssetCreated: boolean;
        readonly asForeignAssetCreated: {
            readonly assetId: u16;
            readonly foreignAsset: StagingXcmV5Location;
        } & Struct;
        readonly isForeignAssetTypeChanged: boolean;
        readonly asForeignAssetTypeChanged: {
            readonly assetId: u16;
            readonly newForeignAsset: StagingXcmV5Location;
        } & Struct;
        readonly isForeignAssetRemoved: boolean;
        readonly asForeignAssetRemoved: {
            readonly assetId: u16;
            readonly foreignAsset: StagingXcmV5Location;
        } & Struct;
        readonly isForeignAssetDestroyed: boolean;
        readonly asForeignAssetDestroyed: {
            readonly assetId: u16;
            readonly foreignAsset: StagingXcmV5Location;
        } & Struct;
        readonly type:
            | "ForeignAssetCreated"
            | "ForeignAssetTypeChanged"
            | "ForeignAssetRemoved"
            | "ForeignAssetDestroyed";
    }

    /** @name PalletXcmEvent (558) */
    interface PalletXcmEvent extends Enum {
        readonly isAttempted: boolean;
        readonly asAttempted: {
            readonly outcome: StagingXcmV5TraitsOutcome;
        } & Struct;
        readonly isSent: boolean;
        readonly asSent: {
            readonly origin: StagingXcmV5Location;
            readonly destination: StagingXcmV5Location;
            readonly message: StagingXcmV5Xcm;
            readonly messageId: U8aFixed;
        } & Struct;
        readonly isSendFailed: boolean;
        readonly asSendFailed: {
            readonly origin: StagingXcmV5Location;
            readonly destination: StagingXcmV5Location;
            readonly error: XcmV3TraitsSendError;
            readonly messageId: U8aFixed;
        } & Struct;
        readonly isProcessXcmError: boolean;
        readonly asProcessXcmError: {
            readonly origin: StagingXcmV5Location;
            readonly error: XcmV5TraitsError;
            readonly messageId: U8aFixed;
        } & Struct;
        readonly isUnexpectedResponse: boolean;
        readonly asUnexpectedResponse: {
            readonly origin: StagingXcmV5Location;
            readonly queryId: u64;
        } & Struct;
        readonly isResponseReady: boolean;
        readonly asResponseReady: {
            readonly queryId: u64;
            readonly response: StagingXcmV5Response;
        } & Struct;
        readonly isNotified: boolean;
        readonly asNotified: {
            readonly queryId: u64;
            readonly palletIndex: u8;
            readonly callIndex: u8;
        } & Struct;
        readonly isNotifyOverweight: boolean;
        readonly asNotifyOverweight: {
            readonly queryId: u64;
            readonly palletIndex: u8;
            readonly callIndex: u8;
            readonly actualWeight: SpWeightsWeightV2Weight;
            readonly maxBudgetedWeight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isNotifyDispatchError: boolean;
        readonly asNotifyDispatchError: {
            readonly queryId: u64;
            readonly palletIndex: u8;
            readonly callIndex: u8;
        } & Struct;
        readonly isNotifyDecodeFailed: boolean;
        readonly asNotifyDecodeFailed: {
            readonly queryId: u64;
            readonly palletIndex: u8;
            readonly callIndex: u8;
        } & Struct;
        readonly isInvalidResponder: boolean;
        readonly asInvalidResponder: {
            readonly origin: StagingXcmV5Location;
            readonly queryId: u64;
            readonly expectedLocation: Option<StagingXcmV5Location>;
        } & Struct;
        readonly isInvalidResponderVersion: boolean;
        readonly asInvalidResponderVersion: {
            readonly origin: StagingXcmV5Location;
            readonly queryId: u64;
        } & Struct;
        readonly isResponseTaken: boolean;
        readonly asResponseTaken: {
            readonly queryId: u64;
        } & Struct;
        readonly isAssetsTrapped: boolean;
        readonly asAssetsTrapped: {
            readonly hash_: H256;
            readonly origin: StagingXcmV5Location;
            readonly assets: XcmVersionedAssets;
        } & Struct;
        readonly isVersionChangeNotified: boolean;
        readonly asVersionChangeNotified: {
            readonly destination: StagingXcmV5Location;
            readonly result: u32;
            readonly cost: StagingXcmV5AssetAssets;
            readonly messageId: U8aFixed;
        } & Struct;
        readonly isSupportedVersionChanged: boolean;
        readonly asSupportedVersionChanged: {
            readonly location: StagingXcmV5Location;
            readonly version: u32;
        } & Struct;
        readonly isNotifyTargetSendFail: boolean;
        readonly asNotifyTargetSendFail: {
            readonly location: StagingXcmV5Location;
            readonly queryId: u64;
            readonly error: XcmV5TraitsError;
        } & Struct;
        readonly isNotifyTargetMigrationFail: boolean;
        readonly asNotifyTargetMigrationFail: {
            readonly location: XcmVersionedLocation;
            readonly queryId: u64;
        } & Struct;
        readonly isInvalidQuerierVersion: boolean;
        readonly asInvalidQuerierVersion: {
            readonly origin: StagingXcmV5Location;
            readonly queryId: u64;
        } & Struct;
        readonly isInvalidQuerier: boolean;
        readonly asInvalidQuerier: {
            readonly origin: StagingXcmV5Location;
            readonly queryId: u64;
            readonly expectedQuerier: StagingXcmV5Location;
            readonly maybeActualQuerier: Option<StagingXcmV5Location>;
        } & Struct;
        readonly isVersionNotifyStarted: boolean;
        readonly asVersionNotifyStarted: {
            readonly destination: StagingXcmV5Location;
            readonly cost: StagingXcmV5AssetAssets;
            readonly messageId: U8aFixed;
        } & Struct;
        readonly isVersionNotifyRequested: boolean;
        readonly asVersionNotifyRequested: {
            readonly destination: StagingXcmV5Location;
            readonly cost: StagingXcmV5AssetAssets;
            readonly messageId: U8aFixed;
        } & Struct;
        readonly isVersionNotifyUnrequested: boolean;
        readonly asVersionNotifyUnrequested: {
            readonly destination: StagingXcmV5Location;
            readonly cost: StagingXcmV5AssetAssets;
            readonly messageId: U8aFixed;
        } & Struct;
        readonly isFeesPaid: boolean;
        readonly asFeesPaid: {
            readonly paying: StagingXcmV5Location;
            readonly fees: StagingXcmV5AssetAssets;
        } & Struct;
        readonly isAssetsClaimed: boolean;
        readonly asAssetsClaimed: {
            readonly hash_: H256;
            readonly origin: StagingXcmV5Location;
            readonly assets: XcmVersionedAssets;
        } & Struct;
        readonly isVersionMigrationFinished: boolean;
        readonly asVersionMigrationFinished: {
            readonly version: u32;
        } & Struct;
        readonly isAliasAuthorized: boolean;
        readonly asAliasAuthorized: {
            readonly aliaser: StagingXcmV5Location;
            readonly target: StagingXcmV5Location;
            readonly expiry: Option<u64>;
        } & Struct;
        readonly isAliasAuthorizationRemoved: boolean;
        readonly asAliasAuthorizationRemoved: {
            readonly aliaser: StagingXcmV5Location;
            readonly target: StagingXcmV5Location;
        } & Struct;
        readonly isAliasesAuthorizationsRemoved: boolean;
        readonly asAliasesAuthorizationsRemoved: {
            readonly target: StagingXcmV5Location;
        } & Struct;
        readonly type:
            | "Attempted"
            | "Sent"
            | "SendFailed"
            | "ProcessXcmError"
            | "UnexpectedResponse"
            | "ResponseReady"
            | "Notified"
            | "NotifyOverweight"
            | "NotifyDispatchError"
            | "NotifyDecodeFailed"
            | "InvalidResponder"
            | "InvalidResponderVersion"
            | "ResponseTaken"
            | "AssetsTrapped"
            | "VersionChangeNotified"
            | "SupportedVersionChanged"
            | "NotifyTargetSendFail"
            | "NotifyTargetMigrationFail"
            | "InvalidQuerierVersion"
            | "InvalidQuerier"
            | "VersionNotifyStarted"
            | "VersionNotifyRequested"
            | "VersionNotifyUnrequested"
            | "FeesPaid"
            | "AssetsClaimed"
            | "VersionMigrationFinished"
            | "AliasAuthorized"
            | "AliasAuthorizationRemoved"
            | "AliasesAuthorizationsRemoved";
    }

    /** @name StagingXcmV5TraitsOutcome (559) */
    interface StagingXcmV5TraitsOutcome extends Enum {
        readonly isComplete: boolean;
        readonly asComplete: {
            readonly used: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isIncomplete: boolean;
        readonly asIncomplete: {
            readonly used: SpWeightsWeightV2Weight;
            readonly error: StagingXcmV5TraitsInstructionError;
        } & Struct;
        readonly isError: boolean;
        readonly asError: StagingXcmV5TraitsInstructionError;
        readonly type: "Complete" | "Incomplete" | "Error";
    }

    /** @name StagingXcmV5TraitsInstructionError (560) */
    interface StagingXcmV5TraitsInstructionError extends Struct {
        readonly index: u8;
        readonly error: XcmV5TraitsError;
    }

    /** @name XcmV3TraitsSendError (561) */
    interface XcmV3TraitsSendError extends Enum {
        readonly isNotApplicable: boolean;
        readonly isTransport: boolean;
        readonly isUnroutable: boolean;
        readonly isDestinationUnsupported: boolean;
        readonly isExceedsMaxMessageSize: boolean;
        readonly isMissingArgument: boolean;
        readonly isFees: boolean;
        readonly type:
            | "NotApplicable"
            | "Transport"
            | "Unroutable"
            | "DestinationUnsupported"
            | "ExceedsMaxMessageSize"
            | "MissingArgument"
            | "Fees";
    }

    /** @name PalletStreamPaymentEvent (562) */
    interface PalletStreamPaymentEvent extends Enum {
        readonly isStreamOpened: boolean;
        readonly asStreamOpened: {
            readonly streamId: u64;
        } & Struct;
        readonly isStreamClosed: boolean;
        readonly asStreamClosed: {
            readonly streamId: u64;
            readonly refunded: u128;
        } & Struct;
        readonly isStreamPayment: boolean;
        readonly asStreamPayment: {
            readonly streamId: u64;
            readonly source: AccountId32;
            readonly target: AccountId32;
            readonly amount: u128;
            readonly stalled: bool;
        } & Struct;
        readonly isStreamConfigChangeRequested: boolean;
        readonly asStreamConfigChangeRequested: {
            readonly streamId: u64;
            readonly requestNonce: u32;
            readonly requester: PalletStreamPaymentParty;
            readonly oldConfig: PalletStreamPaymentStreamConfig;
            readonly newConfig: PalletStreamPaymentStreamConfig;
        } & Struct;
        readonly isStreamConfigChanged: boolean;
        readonly asStreamConfigChanged: {
            readonly streamId: u64;
            readonly oldConfig: PalletStreamPaymentStreamConfig;
            readonly newConfig: PalletStreamPaymentStreamConfig;
            readonly depositChange: Option<PalletStreamPaymentDepositChange>;
        } & Struct;
        readonly type:
            | "StreamOpened"
            | "StreamClosed"
            | "StreamPayment"
            | "StreamConfigChangeRequested"
            | "StreamConfigChanged";
    }

    /** @name PalletStreamPaymentParty (563) */
    interface PalletStreamPaymentParty extends Enum {
        readonly isSource: boolean;
        readonly isTarget: boolean;
        readonly type: "Source" | "Target";
    }

    /** @name PalletMigrationsEvent (564) */
    interface PalletMigrationsEvent extends Enum {
        readonly isRuntimeUpgradeStarted: boolean;
        readonly isRuntimeUpgradeCompleted: boolean;
        readonly asRuntimeUpgradeCompleted: {
            readonly weight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isMigrationStarted: boolean;
        readonly asMigrationStarted: {
            readonly migrationName: Bytes;
        } & Struct;
        readonly isMigrationCompleted: boolean;
        readonly asMigrationCompleted: {
            readonly migrationName: Bytes;
            readonly consumedWeight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isFailedToSuspendIdleXcmExecution: boolean;
        readonly asFailedToSuspendIdleXcmExecution: {
            readonly error: SpRuntimeDispatchError;
        } & Struct;
        readonly isFailedToResumeIdleXcmExecution: boolean;
        readonly asFailedToResumeIdleXcmExecution: {
            readonly error: SpRuntimeDispatchError;
        } & Struct;
        readonly type:
            | "RuntimeUpgradeStarted"
            | "RuntimeUpgradeCompleted"
            | "MigrationStarted"
            | "MigrationCompleted"
            | "FailedToSuspendIdleXcmExecution"
            | "FailedToResumeIdleXcmExecution";
    }

    /** @name PalletMaintenanceModeEvent (566) */
    interface PalletMaintenanceModeEvent extends Enum {
        readonly isEnteredMaintenanceMode: boolean;
        readonly isNormalOperationResumed: boolean;
        readonly isFailedToSuspendIdleXcmExecution: boolean;
        readonly asFailedToSuspendIdleXcmExecution: {
            readonly error: SpRuntimeDispatchError;
        } & Struct;
        readonly isFailedToResumeIdleXcmExecution: boolean;
        readonly asFailedToResumeIdleXcmExecution: {
            readonly error: SpRuntimeDispatchError;
        } & Struct;
        readonly type:
            | "EnteredMaintenanceMode"
            | "NormalOperationResumed"
            | "FailedToSuspendIdleXcmExecution"
            | "FailedToResumeIdleXcmExecution";
    }

    /** @name SnowbridgePalletEthereumClientEvent (567) */
    interface SnowbridgePalletEthereumClientEvent extends Enum {
        readonly isBeaconHeaderImported: boolean;
        readonly asBeaconHeaderImported: {
            readonly blockHash: H256;
            readonly slot: u64;
        } & Struct;
        readonly isSyncCommitteeUpdated: boolean;
        readonly asSyncCommitteeUpdated: {
            readonly period: u64;
        } & Struct;
        readonly isOperatingModeChanged: boolean;
        readonly asOperatingModeChanged: {
            readonly mode: SnowbridgeCoreOperatingModeBasicOperatingMode;
        } & Struct;
        readonly type: "BeaconHeaderImported" | "SyncCommitteeUpdated" | "OperatingModeChanged";
    }

    /** @name PalletRootTestingEvent (568) */
    interface PalletRootTestingEvent extends Enum {
        readonly isDefensiveTestCall: boolean;
        readonly type: "DefensiveTestCall";
    }

    /** @name PalletSudoEvent (569) */
    interface PalletSudoEvent extends Enum {
        readonly isSudid: boolean;
        readonly asSudid: {
            readonly sudoResult: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isKeyChanged: boolean;
        readonly asKeyChanged: {
            readonly old: Option<AccountId32>;
            readonly new_: AccountId32;
        } & Struct;
        readonly isKeyRemoved: boolean;
        readonly isSudoAsDone: boolean;
        readonly asSudoAsDone: {
            readonly sudoResult: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly type: "Sudid" | "KeyChanged" | "KeyRemoved" | "SudoAsDone";
    }

    /** @name FrameSystemPhase (570) */
    interface FrameSystemPhase extends Enum {
        readonly isApplyExtrinsic: boolean;
        readonly asApplyExtrinsic: u32;
        readonly isFinalization: boolean;
        readonly isInitialization: boolean;
        readonly type: "ApplyExtrinsic" | "Finalization" | "Initialization";
    }

    /** @name FrameSystemLastRuntimeUpgradeInfo (572) */
    interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
        readonly specVersion: Compact<u32>;
        readonly specName: Text;
    }

    /** @name FrameSystemCodeUpgradeAuthorization (575) */
    interface FrameSystemCodeUpgradeAuthorization extends Struct {
        readonly codeHash: H256;
        readonly checkVersion: bool;
    }

    /** @name FrameSystemLimitsBlockWeights (576) */
    interface FrameSystemLimitsBlockWeights extends Struct {
        readonly baseBlock: SpWeightsWeightV2Weight;
        readonly maxBlock: SpWeightsWeightV2Weight;
        readonly perClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
    }

    /** @name FrameSupportDispatchPerDispatchClassWeightsPerClass (577) */
    interface FrameSupportDispatchPerDispatchClassWeightsPerClass extends Struct {
        readonly normal: FrameSystemLimitsWeightsPerClass;
        readonly operational: FrameSystemLimitsWeightsPerClass;
        readonly mandatory: FrameSystemLimitsWeightsPerClass;
    }

    /** @name FrameSystemLimitsWeightsPerClass (578) */
    interface FrameSystemLimitsWeightsPerClass extends Struct {
        readonly baseExtrinsic: SpWeightsWeightV2Weight;
        readonly maxExtrinsic: Option<SpWeightsWeightV2Weight>;
        readonly maxTotal: Option<SpWeightsWeightV2Weight>;
        readonly reserved: Option<SpWeightsWeightV2Weight>;
    }

    /** @name FrameSystemLimitsBlockLength (579) */
    interface FrameSystemLimitsBlockLength extends Struct {
        readonly max: FrameSupportDispatchPerDispatchClassU32;
    }

    /** @name FrameSupportDispatchPerDispatchClassU32 (580) */
    interface FrameSupportDispatchPerDispatchClassU32 extends Struct {
        readonly normal: u32;
        readonly operational: u32;
        readonly mandatory: u32;
    }

    /** @name SpWeightsRuntimeDbWeight (581) */
    interface SpWeightsRuntimeDbWeight extends Struct {
        readonly read: u64;
        readonly write: u64;
    }

    /** @name SpVersionRuntimeVersion (582) */
    interface SpVersionRuntimeVersion extends Struct {
        readonly specName: Text;
        readonly implName: Text;
        readonly authoringVersion: u32;
        readonly specVersion: u32;
        readonly implVersion: u32;
        readonly apis: Vec<ITuple<[U8aFixed, u32]>>;
        readonly transactionVersion: u32;
        readonly systemVersion: u8;
    }

    /** @name FrameSystemError (586) */
    interface FrameSystemError extends Enum {
        readonly isInvalidSpecName: boolean;
        readonly isSpecVersionNeedsToIncrease: boolean;
        readonly isFailedToExtractRuntimeVersion: boolean;
        readonly isNonDefaultComposite: boolean;
        readonly isNonZeroRefCount: boolean;
        readonly isCallFiltered: boolean;
        readonly isMultiBlockMigrationsOngoing: boolean;
        readonly isNothingAuthorized: boolean;
        readonly isUnauthorized: boolean;
        readonly type:
            | "InvalidSpecName"
            | "SpecVersionNeedsToIncrease"
            | "FailedToExtractRuntimeVersion"
            | "NonDefaultComposite"
            | "NonZeroRefCount"
            | "CallFiltered"
            | "MultiBlockMigrationsOngoing"
            | "NothingAuthorized"
            | "Unauthorized";
    }

    /** @name SpConsensusBabeDigestsPreDigest (593) */
    interface SpConsensusBabeDigestsPreDigest extends Enum {
        readonly isPrimary: boolean;
        readonly asPrimary: SpConsensusBabeDigestsPrimaryPreDigest;
        readonly isSecondaryPlain: boolean;
        readonly asSecondaryPlain: SpConsensusBabeDigestsSecondaryPlainPreDigest;
        readonly isSecondaryVRF: boolean;
        readonly asSecondaryVRF: SpConsensusBabeDigestsSecondaryVRFPreDigest;
        readonly type: "Primary" | "SecondaryPlain" | "SecondaryVRF";
    }

    /** @name SpConsensusBabeDigestsPrimaryPreDigest (594) */
    interface SpConsensusBabeDigestsPrimaryPreDigest extends Struct {
        readonly authorityIndex: u32;
        readonly slot: u64;
        readonly vrfSignature: SpCoreSr25519VrfVrfSignature;
    }

    /** @name SpCoreSr25519VrfVrfSignature (595) */
    interface SpCoreSr25519VrfVrfSignature extends Struct {
        readonly preOutput: U8aFixed;
        readonly proof: U8aFixed;
    }

    /** @name SpConsensusBabeDigestsSecondaryPlainPreDigest (596) */
    interface SpConsensusBabeDigestsSecondaryPlainPreDigest extends Struct {
        readonly authorityIndex: u32;
        readonly slot: u64;
    }

    /** @name SpConsensusBabeDigestsSecondaryVRFPreDigest (597) */
    interface SpConsensusBabeDigestsSecondaryVRFPreDigest extends Struct {
        readonly authorityIndex: u32;
        readonly slot: u64;
        readonly vrfSignature: SpCoreSr25519VrfVrfSignature;
    }

    /** @name SpConsensusBabeBabeEpochConfiguration (598) */
    interface SpConsensusBabeBabeEpochConfiguration extends Struct {
        readonly c: ITuple<[u64, u64]>;
        readonly allowedSlots: SpConsensusBabeAllowedSlots;
    }

    /** @name PalletBabeError (602) */
    interface PalletBabeError extends Enum {
        readonly isInvalidEquivocationProof: boolean;
        readonly isInvalidKeyOwnershipProof: boolean;
        readonly isDuplicateOffenceReport: boolean;
        readonly isInvalidConfiguration: boolean;
        readonly type:
            | "InvalidEquivocationProof"
            | "InvalidKeyOwnershipProof"
            | "DuplicateOffenceReport"
            | "InvalidConfiguration";
    }

    /** @name PalletBalancesBalanceLock (604) */
    interface PalletBalancesBalanceLock extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
        readonly reasons: PalletBalancesReasons;
    }

    /** @name PalletBalancesReasons (605) */
    interface PalletBalancesReasons extends Enum {
        readonly isFee: boolean;
        readonly isMisc: boolean;
        readonly isAll: boolean;
        readonly type: "Fee" | "Misc" | "All";
    }

    /** @name PalletBalancesReserveData (608) */
    interface PalletBalancesReserveData extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
    }

    /** @name DancelightRuntimeRuntimeHoldReason (612) */
    interface DancelightRuntimeRuntimeHoldReason extends Enum {
        readonly isContainerRegistrar: boolean;
        readonly asContainerRegistrar: PalletRegistrarHoldReason;
        readonly isDataPreservers: boolean;
        readonly asDataPreservers: PalletDataPreserversHoldReason;
        readonly isPooledStaking: boolean;
        readonly asPooledStaking: PalletPooledStakingHoldReason;
        readonly isOpenTechCommitteeCollective: boolean;
        readonly asOpenTechCommitteeCollective: PalletCollectiveHoldReason;
        readonly isPreimage: boolean;
        readonly asPreimage: PalletPreimageHoldReason;
        readonly isXcmPallet: boolean;
        readonly asXcmPallet: PalletXcmHoldReason;
        readonly isStreamPayment: boolean;
        readonly asStreamPayment: PalletStreamPaymentHoldReason;
        readonly type:
            | "ContainerRegistrar"
            | "DataPreservers"
            | "PooledStaking"
            | "OpenTechCommitteeCollective"
            | "Preimage"
            | "XcmPallet"
            | "StreamPayment";
    }

    /** @name PalletRegistrarHoldReason (613) */
    interface PalletRegistrarHoldReason extends Enum {
        readonly isRegistrarDeposit: boolean;
        readonly type: "RegistrarDeposit";
    }

    /** @name PalletDataPreserversHoldReason (614) */
    interface PalletDataPreserversHoldReason extends Enum {
        readonly isProfileDeposit: boolean;
        readonly type: "ProfileDeposit";
    }

    /** @name PalletPooledStakingHoldReason (615) */
    interface PalletPooledStakingHoldReason extends Enum {
        readonly isPooledStake: boolean;
        readonly type: "PooledStake";
    }

    /** @name PalletCollectiveHoldReason (616) */
    interface PalletCollectiveHoldReason extends Enum {
        readonly isProposalSubmission: boolean;
        readonly type: "ProposalSubmission";
    }

    /** @name PalletPreimageHoldReason (617) */
    interface PalletPreimageHoldReason extends Enum {
        readonly isPreimage: boolean;
        readonly type: "Preimage";
    }

    /** @name PalletXcmHoldReason (618) */
    interface PalletXcmHoldReason extends Enum {
        readonly isAuthorizeAlias: boolean;
        readonly type: "AuthorizeAlias";
    }

    /** @name PalletStreamPaymentHoldReason (619) */
    interface PalletStreamPaymentHoldReason extends Enum {
        readonly isStreamPayment: boolean;
        readonly isStreamOpened: boolean;
        readonly type: "StreamPayment" | "StreamOpened";
    }

    /** @name FrameSupportTokensMiscIdAmount (622) */
    interface FrameSupportTokensMiscIdAmount extends Struct {
        readonly id: Null;
        readonly amount: u128;
    }

    /** @name PalletBalancesError (624) */
    interface PalletBalancesError extends Enum {
        readonly isVestingBalance: boolean;
        readonly isLiquidityRestrictions: boolean;
        readonly isInsufficientBalance: boolean;
        readonly isExistentialDeposit: boolean;
        readonly isExpendability: boolean;
        readonly isExistingVestingSchedule: boolean;
        readonly isDeadAccount: boolean;
        readonly isTooManyReserves: boolean;
        readonly isTooManyHolds: boolean;
        readonly isTooManyFreezes: boolean;
        readonly isIssuanceDeactivated: boolean;
        readonly isDeltaZero: boolean;
        readonly type:
            | "VestingBalance"
            | "LiquidityRestrictions"
            | "InsufficientBalance"
            | "ExistentialDeposit"
            | "Expendability"
            | "ExistingVestingSchedule"
            | "DeadAccount"
            | "TooManyReserves"
            | "TooManyHolds"
            | "TooManyFreezes"
            | "IssuanceDeactivated"
            | "DeltaZero";
    }

    /** @name PalletTransactionPaymentReleases (625) */
    interface PalletTransactionPaymentReleases extends Enum {
        readonly isV1Ancient: boolean;
        readonly isV2: boolean;
        readonly type: "V1Ancient" | "V2";
    }

    /** @name SpStakingOffenceOffenceDetails (626) */
    interface SpStakingOffenceOffenceDetails extends Struct {
        readonly offender: ITuple<[AccountId32, Null]>;
        readonly reporters: Vec<AccountId32>;
    }

    /** @name PalletRegistrarDepositInfo (638) */
    interface PalletRegistrarDepositInfo extends Struct {
        readonly creator: AccountId32;
        readonly deposit: u128;
    }

    /** @name PalletRegistrarError (639) */
    interface PalletRegistrarError extends Enum {
        readonly isParaIdAlreadyRegistered: boolean;
        readonly isParaIdNotRegistered: boolean;
        readonly isParaIdAlreadyDeregistered: boolean;
        readonly isParaIdAlreadyPaused: boolean;
        readonly isParaIdNotPaused: boolean;
        readonly isParaIdListFull: boolean;
        readonly isGenesisDataTooBig: boolean;
        readonly isParaIdNotInPendingVerification: boolean;
        readonly isNotSufficientDeposit: boolean;
        readonly isNotAParathread: boolean;
        readonly isNotParaCreator: boolean;
        readonly isRelayStorageRootNotFound: boolean;
        readonly isInvalidRelayStorageProof: boolean;
        readonly isInvalidRelayManagerSignature: boolean;
        readonly isParaStillExistsInRelay: boolean;
        readonly isHeadDataNecessary: boolean;
        readonly isWasmCodeNecessary: boolean;
        readonly type:
            | "ParaIdAlreadyRegistered"
            | "ParaIdNotRegistered"
            | "ParaIdAlreadyDeregistered"
            | "ParaIdAlreadyPaused"
            | "ParaIdNotPaused"
            | "ParaIdListFull"
            | "GenesisDataTooBig"
            | "ParaIdNotInPendingVerification"
            | "NotSufficientDeposit"
            | "NotAParathread"
            | "NotParaCreator"
            | "RelayStorageRootNotFound"
            | "InvalidRelayStorageProof"
            | "InvalidRelayManagerSignature"
            | "ParaStillExistsInRelay"
            | "HeadDataNecessary"
            | "WasmCodeNecessary";
    }

    /** @name PalletConfigurationHostConfiguration (640) */
    interface PalletConfigurationHostConfiguration extends Struct {
        readonly maxCollators: u32;
        readonly minOrchestratorCollators: u32;
        readonly maxOrchestratorCollators: u32;
        readonly collatorsPerContainer: u32;
        readonly fullRotationPeriod: u32;
        readonly collatorsPerParathread: u32;
        readonly parathreadsPerCollator: u32;
        readonly targetContainerChainFullness: Perbill;
        readonly maxParachainCoresPercentage: Option<Perbill>;
        readonly fullRotationMode: TpTraitsFullRotationModes;
    }

    /** @name PalletConfigurationError (643) */
    interface PalletConfigurationError extends Enum {
        readonly isInvalidNewValue: boolean;
        readonly type: "InvalidNewValue";
    }

    /** @name PalletInvulnerablesError (645) */
    interface PalletInvulnerablesError extends Enum {
        readonly isTooManyInvulnerables: boolean;
        readonly isAlreadyInvulnerable: boolean;
        readonly isNotInvulnerable: boolean;
        readonly isNoKeysRegistered: boolean;
        readonly isUnableToDeriveCollatorId: boolean;
        readonly type:
            | "TooManyInvulnerables"
            | "AlreadyInvulnerable"
            | "NotInvulnerable"
            | "NoKeysRegistered"
            | "UnableToDeriveCollatorId";
    }

    /** @name DpCollatorAssignmentAssignedCollatorsAccountId32 (646) */
    interface DpCollatorAssignmentAssignedCollatorsAccountId32 extends Struct {
        readonly orchestratorChain: Vec<AccountId32>;
        readonly containerChains: BTreeMap<u32, Vec<AccountId32>>;
    }

    /** @name DpCollatorAssignmentAssignedCollatorsPublic (651) */
    interface DpCollatorAssignmentAssignedCollatorsPublic extends Struct {
        readonly orchestratorChain: Vec<NimbusPrimitivesNimbusCryptoPublic>;
        readonly containerChains: BTreeMap<u32, Vec<NimbusPrimitivesNimbusCryptoPublic>>;
    }

    /** @name TpTraitsContainerChainBlockInfo (659) */
    interface TpTraitsContainerChainBlockInfo extends Struct {
        readonly blockNumber: u32;
        readonly author: AccountId32;
        readonly latestSlotNumber: u64;
    }

    /** @name PalletAuthorNotingError (660) */
    interface PalletAuthorNotingError extends Enum {
        readonly isFailedReading: boolean;
        readonly isFailedDecodingHeader: boolean;
        readonly isAuraDigestFirstItem: boolean;
        readonly isAsPreRuntimeError: boolean;
        readonly isNonDecodableSlot: boolean;
        readonly isAuthorNotFound: boolean;
        readonly isNonAuraDigest: boolean;
        readonly type:
            | "FailedReading"
            | "FailedDecodingHeader"
            | "AuraDigestFirstItem"
            | "AsPreRuntimeError"
            | "NonDecodableSlot"
            | "AuthorNotFound"
            | "NonAuraDigest";
    }

    /** @name PalletServicesPaymentError (661) */
    interface PalletServicesPaymentError extends Enum {
        readonly isInsufficientFundsToPurchaseCredits: boolean;
        readonly isInsufficientCredits: boolean;
        readonly isCreditPriceTooExpensive: boolean;
        readonly type: "InsufficientFundsToPurchaseCredits" | "InsufficientCredits" | "CreditPriceTooExpensive";
    }

    /** @name PalletDataPreserversRegisteredProfile (662) */
    interface PalletDataPreserversRegisteredProfile extends Struct {
        readonly account: AccountId32;
        readonly deposit: u128;
        readonly profile: PalletDataPreserversProfile;
        readonly assignment: Option<ITuple<[u32, TpDataPreserversCommonAssignmentWitness]>>;
    }

    /** @name PalletDataPreserversError (668) */
    interface PalletDataPreserversError extends Enum {
        readonly isNoBootNodes: boolean;
        readonly isUnknownProfileId: boolean;
        readonly isNextProfileIdShouldBeAvailable: boolean;
        readonly isAssignmentPaymentRequestParameterMismatch: boolean;
        readonly isProfileAlreadyAssigned: boolean;
        readonly isProfileNotAssigned: boolean;
        readonly isProfileIsNotElligibleForParaId: boolean;
        readonly isWrongParaId: boolean;
        readonly isMaxAssignmentsPerParaIdReached: boolean;
        readonly isCantDeleteAssignedProfile: boolean;
        readonly type:
            | "NoBootNodes"
            | "UnknownProfileId"
            | "NextProfileIdShouldBeAvailable"
            | "AssignmentPaymentRequestParameterMismatch"
            | "ProfileAlreadyAssigned"
            | "ProfileNotAssigned"
            | "ProfileIsNotElligibleForParaId"
            | "WrongParaId"
            | "MaxAssignmentsPerParaIdReached"
            | "CantDeleteAssignedProfile";
    }

    /** @name TpTraitsActiveEraInfo (671) */
    interface TpTraitsActiveEraInfo extends Struct {
        readonly index: u32;
        readonly start: Option<u64>;
    }

    /** @name PalletExternalValidatorsError (672) */
    interface PalletExternalValidatorsError extends Enum {
        readonly isTooManyWhitelisted: boolean;
        readonly isAlreadyWhitelisted: boolean;
        readonly isNotWhitelisted: boolean;
        readonly isNoKeysRegistered: boolean;
        readonly isUnableToDeriveValidatorId: boolean;
        readonly type:
            | "TooManyWhitelisted"
            | "AlreadyWhitelisted"
            | "NotWhitelisted"
            | "NoKeysRegistered"
            | "UnableToDeriveValidatorId";
    }

    /** @name PalletExternalValidatorSlashesSlash (677) */
    interface PalletExternalValidatorSlashesSlash extends Struct {
        readonly externalIdx: u64;
        readonly validator: AccountId32;
        readonly reporters: Vec<AccountId32>;
        readonly slashId: u32;
        readonly percentage: Perbill;
        readonly confirmed: bool;
    }

    /** @name PalletExternalValidatorSlashesError (678) */
    interface PalletExternalValidatorSlashesError extends Enum {
        readonly isEmptyTargets: boolean;
        readonly isInvalidSlashIndex: boolean;
        readonly isNotSortedAndUnique: boolean;
        readonly isProvidedFutureEra: boolean;
        readonly isProvidedNonSlashableEra: boolean;
        readonly isDeferPeriodIsOver: boolean;
        readonly isErrorComputingSlash: boolean;
        readonly isEthereumValidateFail: boolean;
        readonly isEthereumDeliverFail: boolean;
        readonly isRootTestInvalidParams: boolean;
        readonly type:
            | "EmptyTargets"
            | "InvalidSlashIndex"
            | "NotSortedAndUnique"
            | "ProvidedFutureEra"
            | "ProvidedNonSlashableEra"
            | "DeferPeriodIsOver"
            | "ErrorComputingSlash"
            | "EthereumValidateFail"
            | "EthereumDeliverFail"
            | "RootTestInvalidParams";
    }

    /** @name PalletExternalValidatorsRewardsEraRewardPoints (679) */
    interface PalletExternalValidatorsRewardsEraRewardPoints extends Struct {
        readonly total: u32;
        readonly individual: BTreeMap<AccountId32, u32>;
    }

    /** @name SnowbridgePalletOutboundQueueCommittedMessage (684) */
    interface SnowbridgePalletOutboundQueueCommittedMessage extends Struct {
        readonly channelId: SnowbridgeCoreChannelId;
        readonly nonce: Compact<u64>;
        readonly command: u8;
        readonly params: Bytes;
        readonly maxDispatchGas: Compact<u64>;
        readonly maxFeePerGas: Compact<u128>;
        readonly reward: Compact<u128>;
        readonly id: H256;
    }

    /** @name SnowbridgePalletOutboundQueueError (685) */
    interface SnowbridgePalletOutboundQueueError extends Enum {
        readonly isMessageTooLarge: boolean;
        readonly isHalted: boolean;
        readonly isInvalidChannel: boolean;
        readonly type: "MessageTooLarge" | "Halted" | "InvalidChannel";
    }

    /** @name SnowbridgePalletInboundQueueError (686) */
    interface SnowbridgePalletInboundQueueError extends Enum {
        readonly isInvalidGateway: boolean;
        readonly isInvalidEnvelope: boolean;
        readonly isInvalidNonce: boolean;
        readonly isInvalidPayload: boolean;
        readonly isInvalidChannel: boolean;
        readonly isMaxNonceReached: boolean;
        readonly isInvalidAccountConversion: boolean;
        readonly isHalted: boolean;
        readonly isVerification: boolean;
        readonly asVerification: SnowbridgeVerificationPrimitivesVerificationError;
        readonly isSend: boolean;
        readonly asSend: SnowbridgePalletInboundQueueSendError;
        readonly isConvertMessage: boolean;
        readonly asConvertMessage: SnowbridgeInboundQueuePrimitivesV1ConvertMessageError;
        readonly type:
            | "InvalidGateway"
            | "InvalidEnvelope"
            | "InvalidNonce"
            | "InvalidPayload"
            | "InvalidChannel"
            | "MaxNonceReached"
            | "InvalidAccountConversion"
            | "Halted"
            | "Verification"
            | "Send"
            | "ConvertMessage";
    }

    /** @name SnowbridgeVerificationPrimitivesVerificationError (687) */
    interface SnowbridgeVerificationPrimitivesVerificationError extends Enum {
        readonly isHeaderNotFound: boolean;
        readonly isLogNotFound: boolean;
        readonly isInvalidLog: boolean;
        readonly isInvalidProof: boolean;
        readonly isInvalidExecutionProof: boolean;
        readonly type: "HeaderNotFound" | "LogNotFound" | "InvalidLog" | "InvalidProof" | "InvalidExecutionProof";
    }

    /** @name SnowbridgePalletInboundQueueSendError (688) */
    interface SnowbridgePalletInboundQueueSendError extends Enum {
        readonly isNotApplicable: boolean;
        readonly isNotRoutable: boolean;
        readonly isTransport: boolean;
        readonly isDestinationUnsupported: boolean;
        readonly isExceedsMaxMessageSize: boolean;
        readonly isMissingArgument: boolean;
        readonly isFees: boolean;
        readonly type:
            | "NotApplicable"
            | "NotRoutable"
            | "Transport"
            | "DestinationUnsupported"
            | "ExceedsMaxMessageSize"
            | "MissingArgument"
            | "Fees";
    }

    /** @name SnowbridgeInboundQueuePrimitivesV1ConvertMessageError (689) */
    interface SnowbridgeInboundQueuePrimitivesV1ConvertMessageError extends Enum {
        readonly isUnsupportedVersion: boolean;
        readonly isInvalidDestination: boolean;
        readonly isInvalidToken: boolean;
        readonly isUnsupportedFeeAsset: boolean;
        readonly isCannotReanchor: boolean;
        readonly type:
            | "UnsupportedVersion"
            | "InvalidDestination"
            | "InvalidToken"
            | "UnsupportedFeeAsset"
            | "CannotReanchor";
    }

    /** @name SnowbridgeCoreChannel (690) */
    interface SnowbridgeCoreChannel extends Struct {
        readonly agentId: H256;
        readonly paraId: u32;
    }

    /** @name SnowbridgePalletSystemError (691) */
    interface SnowbridgePalletSystemError extends Enum {
        readonly isLocationConversionFailed: boolean;
        readonly isAgentAlreadyCreated: boolean;
        readonly isNoAgent: boolean;
        readonly isChannelAlreadyCreated: boolean;
        readonly isNoChannel: boolean;
        readonly isUnsupportedLocationVersion: boolean;
        readonly isInvalidLocation: boolean;
        readonly isSend: boolean;
        readonly asSend: SnowbridgeOutboundQueuePrimitivesSendError;
        readonly isInvalidTokenTransferFees: boolean;
        readonly isInvalidPricingParameters: boolean;
        readonly isInvalidUpgradeParameters: boolean;
        readonly type:
            | "LocationConversionFailed"
            | "AgentAlreadyCreated"
            | "NoAgent"
            | "ChannelAlreadyCreated"
            | "NoChannel"
            | "UnsupportedLocationVersion"
            | "InvalidLocation"
            | "Send"
            | "InvalidTokenTransferFees"
            | "InvalidPricingParameters"
            | "InvalidUpgradeParameters";
    }

    /** @name SnowbridgeOutboundQueuePrimitivesSendError (692) */
    interface SnowbridgeOutboundQueuePrimitivesSendError extends Enum {
        readonly isMessageTooLarge: boolean;
        readonly isHalted: boolean;
        readonly isInvalidChannel: boolean;
        readonly isInvalidOrigin: boolean;
        readonly type: "MessageTooLarge" | "Halted" | "InvalidChannel" | "InvalidOrigin";
    }

    /** @name PalletEthereumTokenTransfersError (693) */
    interface PalletEthereumTokenTransfersError extends Enum {
        readonly isChannelInfoNotSet: boolean;
        readonly isUnknownLocationForToken: boolean;
        readonly isInvalidMessage: boolean;
        readonly asInvalidMessage: SnowbridgeOutboundQueuePrimitivesSendError;
        readonly isTransferMessageNotSent: boolean;
        readonly asTransferMessageNotSent: SnowbridgeOutboundQueuePrimitivesSendError;
        readonly type: "ChannelInfoNotSet" | "UnknownLocationForToken" | "InvalidMessage" | "TransferMessageNotSent";
    }

    /** @name SpCoreCryptoKeyTypeId (700) */
    interface SpCoreCryptoKeyTypeId extends U8aFixed {}

    /** @name PalletSessionError (701) */
    interface PalletSessionError extends Enum {
        readonly isInvalidProof: boolean;
        readonly isNoAssociatedValidatorId: boolean;
        readonly isDuplicatedKey: boolean;
        readonly isNoKeys: boolean;
        readonly isNoAccount: boolean;
        readonly type: "InvalidProof" | "NoAssociatedValidatorId" | "DuplicatedKey" | "NoKeys" | "NoAccount";
    }

    /** @name PalletGrandpaStoredState (702) */
    interface PalletGrandpaStoredState extends Enum {
        readonly isLive: boolean;
        readonly isPendingPause: boolean;
        readonly asPendingPause: {
            readonly scheduledAt: u32;
            readonly delay: u32;
        } & Struct;
        readonly isPaused: boolean;
        readonly isPendingResume: boolean;
        readonly asPendingResume: {
            readonly scheduledAt: u32;
            readonly delay: u32;
        } & Struct;
        readonly type: "Live" | "PendingPause" | "Paused" | "PendingResume";
    }

    /** @name PalletGrandpaStoredPendingChange (703) */
    interface PalletGrandpaStoredPendingChange extends Struct {
        readonly scheduledAt: u32;
        readonly delay: u32;
        readonly nextAuthorities: Vec<ITuple<[SpConsensusGrandpaAppPublic, u64]>>;
        readonly forced: Option<u32>;
    }

    /** @name PalletGrandpaError (705) */
    interface PalletGrandpaError extends Enum {
        readonly isPauseFailed: boolean;
        readonly isResumeFailed: boolean;
        readonly isChangePending: boolean;
        readonly isTooSoon: boolean;
        readonly isInvalidKeyOwnershipProof: boolean;
        readonly isInvalidEquivocationProof: boolean;
        readonly isDuplicateOffenceReport: boolean;
        readonly type:
            | "PauseFailed"
            | "ResumeFailed"
            | "ChangePending"
            | "TooSoon"
            | "InvalidKeyOwnershipProof"
            | "InvalidEquivocationProof"
            | "DuplicateOffenceReport";
    }

    /** @name PalletInflationRewardsChainsToRewardValue (708) */
    interface PalletInflationRewardsChainsToRewardValue extends Struct {
        readonly paraIds: Vec<u32>;
        readonly rewardsPerChain: u128;
    }

    /** @name PalletPooledStakingCandidateEligibleCandidate (710) */
    interface PalletPooledStakingCandidateEligibleCandidate extends Struct {
        readonly candidate: AccountId32;
        readonly stake: u128;
    }

    /** @name PalletPooledStakingPoolsKey (713) */
    interface PalletPooledStakingPoolsKey extends Enum {
        readonly isCandidateTotalStake: boolean;
        readonly isJoiningShares: boolean;
        readonly asJoiningShares: {
            readonly delegator: AccountId32;
        } & Struct;
        readonly isJoiningSharesSupply: boolean;
        readonly isJoiningSharesTotalStaked: boolean;
        readonly isJoiningSharesHeldStake: boolean;
        readonly asJoiningSharesHeldStake: {
            readonly delegator: AccountId32;
        } & Struct;
        readonly isAutoCompoundingShares: boolean;
        readonly asAutoCompoundingShares: {
            readonly delegator: AccountId32;
        } & Struct;
        readonly isAutoCompoundingSharesSupply: boolean;
        readonly isAutoCompoundingSharesTotalStaked: boolean;
        readonly isAutoCompoundingSharesHeldStake: boolean;
        readonly asAutoCompoundingSharesHeldStake: {
            readonly delegator: AccountId32;
        } & Struct;
        readonly isManualRewardsShares: boolean;
        readonly asManualRewardsShares: {
            readonly delegator: AccountId32;
        } & Struct;
        readonly isManualRewardsSharesSupply: boolean;
        readonly isManualRewardsSharesTotalStaked: boolean;
        readonly isManualRewardsSharesHeldStake: boolean;
        readonly asManualRewardsSharesHeldStake: {
            readonly delegator: AccountId32;
        } & Struct;
        readonly isManualRewardsCounter: boolean;
        readonly isManualRewardsCheckpoint: boolean;
        readonly asManualRewardsCheckpoint: {
            readonly delegator: AccountId32;
        } & Struct;
        readonly isLeavingShares: boolean;
        readonly asLeavingShares: {
            readonly delegator: AccountId32;
        } & Struct;
        readonly isLeavingSharesSupply: boolean;
        readonly isLeavingSharesTotalStaked: boolean;
        readonly isLeavingSharesHeldStake: boolean;
        readonly asLeavingSharesHeldStake: {
            readonly delegator: AccountId32;
        } & Struct;
        readonly type:
            | "CandidateTotalStake"
            | "JoiningShares"
            | "JoiningSharesSupply"
            | "JoiningSharesTotalStaked"
            | "JoiningSharesHeldStake"
            | "AutoCompoundingShares"
            | "AutoCompoundingSharesSupply"
            | "AutoCompoundingSharesTotalStaked"
            | "AutoCompoundingSharesHeldStake"
            | "ManualRewardsShares"
            | "ManualRewardsSharesSupply"
            | "ManualRewardsSharesTotalStaked"
            | "ManualRewardsSharesHeldStake"
            | "ManualRewardsCounter"
            | "ManualRewardsCheckpoint"
            | "LeavingShares"
            | "LeavingSharesSupply"
            | "LeavingSharesTotalStaked"
            | "LeavingSharesHeldStake";
    }

    /** @name PalletPooledStakingPoolsCandidateSummary (716) */
    interface PalletPooledStakingPoolsCandidateSummary extends Struct {
        readonly delegators: u32;
        readonly joiningDelegators: u32;
        readonly autoCompoundingDelegators: u32;
        readonly manualRewardsDelegators: u32;
        readonly leavingDelegators: u32;
    }

    /** @name PalletPooledStakingError (717) */
    interface PalletPooledStakingError extends Enum {
        readonly isInvalidPalletSetting: boolean;
        readonly isDisabledFeature: boolean;
        readonly isNoOneIsStaking: boolean;
        readonly isStakeMustBeNonZero: boolean;
        readonly isRewardsMustBeNonZero: boolean;
        readonly isMathUnderflow: boolean;
        readonly isMathOverflow: boolean;
        readonly isNotEnoughShares: boolean;
        readonly isTryingToLeaveTooSoon: boolean;
        readonly isInconsistentState: boolean;
        readonly isUnsufficientSharesForTransfer: boolean;
        readonly isCandidateTransferingOwnSharesForbidden: boolean;
        readonly isRequestCannotBeExecuted: boolean;
        readonly asRequestCannotBeExecuted: u16;
        readonly isSwapResultsInZeroShares: boolean;
        readonly isPoolsExtrinsicsArePaused: boolean;
        readonly type:
            | "InvalidPalletSetting"
            | "DisabledFeature"
            | "NoOneIsStaking"
            | "StakeMustBeNonZero"
            | "RewardsMustBeNonZero"
            | "MathUnderflow"
            | "MathOverflow"
            | "NotEnoughShares"
            | "TryingToLeaveTooSoon"
            | "InconsistentState"
            | "UnsufficientSharesForTransfer"
            | "CandidateTransferingOwnSharesForbidden"
            | "RequestCannotBeExecuted"
            | "SwapResultsInZeroShares"
            | "PoolsExtrinsicsArePaused";
    }

    /** @name PalletInactivityTrackingOfflineStatus (720) */
    interface PalletInactivityTrackingOfflineStatus extends Enum {
        readonly isDisabled: boolean;
        readonly isNotified: boolean;
        readonly asNotified: {
            readonly cooldownEnd: u32;
        } & Struct;
        readonly type: "Disabled" | "Notified";
    }

    /** @name PalletInactivityTrackingError (721) */
    interface PalletInactivityTrackingError extends Enum {
        readonly isMaxCollatorsPerSessionReached: boolean;
        readonly isActivityTrackingStatusUpdateSuspended: boolean;
        readonly isActivityTrackingStatusAlreadyEnabled: boolean;
        readonly isActivityTrackingStatusAlreadyDisabled: boolean;
        readonly isMarkingOfflineNotEnabled: boolean;
        readonly isCollatorNotEligibleCandidate: boolean;
        readonly isCollatorNotOnline: boolean;
        readonly isCollatorAlreadyNotifiedOffline: boolean;
        readonly isCollatorNotReadyToBeOnline: boolean;
        readonly isCollatorNotOffline: boolean;
        readonly isMarkingInvulnerableOfflineInvalid: boolean;
        readonly isCollatorCannotBeNotifiedAsInactive: boolean;
        readonly type:
            | "MaxCollatorsPerSessionReached"
            | "ActivityTrackingStatusUpdateSuspended"
            | "ActivityTrackingStatusAlreadyEnabled"
            | "ActivityTrackingStatusAlreadyDisabled"
            | "MarkingOfflineNotEnabled"
            | "CollatorNotEligibleCandidate"
            | "CollatorNotOnline"
            | "CollatorAlreadyNotifiedOffline"
            | "CollatorNotReadyToBeOnline"
            | "CollatorNotOffline"
            | "MarkingInvulnerableOfflineInvalid"
            | "CollatorCannotBeNotifiedAsInactive";
    }

    /** @name PalletTreasuryProposal (722) */
    interface PalletTreasuryProposal extends Struct {
        readonly proposer: AccountId32;
        readonly value: u128;
        readonly beneficiary: AccountId32;
        readonly bond: u128;
    }

    /** @name PalletTreasurySpendStatus (724) */
    interface PalletTreasurySpendStatus extends Struct {
        readonly assetKind: Null;
        readonly amount: u128;
        readonly beneficiary: AccountId32;
        readonly validFrom: u32;
        readonly expireAt: u32;
        readonly status: PalletTreasuryPaymentState;
    }

    /** @name PalletTreasuryPaymentState (725) */
    interface PalletTreasuryPaymentState extends Enum {
        readonly isPending: boolean;
        readonly isAttempted: boolean;
        readonly asAttempted: {
            readonly id: Null;
        } & Struct;
        readonly isFailed: boolean;
        readonly type: "Pending" | "Attempted" | "Failed";
    }

    /** @name FrameSupportPalletId (727) */
    interface FrameSupportPalletId extends U8aFixed {}

    /** @name PalletTreasuryError (728) */
    interface PalletTreasuryError extends Enum {
        readonly isInvalidIndex: boolean;
        readonly isTooManyApprovals: boolean;
        readonly isInsufficientPermission: boolean;
        readonly isProposalNotApproved: boolean;
        readonly isFailedToConvertBalance: boolean;
        readonly isSpendExpired: boolean;
        readonly isEarlyPayout: boolean;
        readonly isAlreadyAttempted: boolean;
        readonly isPayoutError: boolean;
        readonly isNotAttempted: boolean;
        readonly isInconclusive: boolean;
        readonly type:
            | "InvalidIndex"
            | "TooManyApprovals"
            | "InsufficientPermission"
            | "ProposalNotApproved"
            | "FailedToConvertBalance"
            | "SpendExpired"
            | "EarlyPayout"
            | "AlreadyAttempted"
            | "PayoutError"
            | "NotAttempted"
            | "Inconclusive";
    }

    /** @name PalletConvictionVotingVoteVoting (730) */
    interface PalletConvictionVotingVoteVoting extends Enum {
        readonly isCasting: boolean;
        readonly asCasting: PalletConvictionVotingVoteCasting;
        readonly isDelegating: boolean;
        readonly asDelegating: PalletConvictionVotingVoteDelegating;
        readonly type: "Casting" | "Delegating";
    }

    /** @name PalletConvictionVotingVoteCasting (731) */
    interface PalletConvictionVotingVoteCasting extends Struct {
        readonly votes: Vec<ITuple<[u32, PalletConvictionVotingVoteAccountVote]>>;
        readonly delegations: PalletConvictionVotingDelegations;
        readonly prior: PalletConvictionVotingVotePriorLock;
    }

    /** @name PalletConvictionVotingDelegations (735) */
    interface PalletConvictionVotingDelegations extends Struct {
        readonly votes: u128;
        readonly capital: u128;
    }

    /** @name PalletConvictionVotingVotePriorLock (736) */
    interface PalletConvictionVotingVotePriorLock extends ITuple<[u32, u128]> {}

    /** @name PalletConvictionVotingVoteDelegating (737) */
    interface PalletConvictionVotingVoteDelegating extends Struct {
        readonly balance: u128;
        readonly target: AccountId32;
        readonly conviction: PalletConvictionVotingConviction;
        readonly delegations: PalletConvictionVotingDelegations;
        readonly prior: PalletConvictionVotingVotePriorLock;
    }

    /** @name PalletConvictionVotingError (741) */
    interface PalletConvictionVotingError extends Enum {
        readonly isNotOngoing: boolean;
        readonly isNotVoter: boolean;
        readonly isNoPermission: boolean;
        readonly isNoPermissionYet: boolean;
        readonly isAlreadyDelegating: boolean;
        readonly isAlreadyVoting: boolean;
        readonly isInsufficientFunds: boolean;
        readonly isNotDelegating: boolean;
        readonly isNonsense: boolean;
        readonly isMaxVotesReached: boolean;
        readonly isClassNeeded: boolean;
        readonly isBadClass: boolean;
        readonly type:
            | "NotOngoing"
            | "NotVoter"
            | "NoPermission"
            | "NoPermissionYet"
            | "AlreadyDelegating"
            | "AlreadyVoting"
            | "InsufficientFunds"
            | "NotDelegating"
            | "Nonsense"
            | "MaxVotesReached"
            | "ClassNeeded"
            | "BadClass";
    }

    /** @name PalletReferendaReferendumInfoConvictionVotingTally (742) */
    interface PalletReferendaReferendumInfoConvictionVotingTally extends Enum {
        readonly isOngoing: boolean;
        readonly asOngoing: PalletReferendaReferendumStatusConvictionVotingTally;
        readonly isApproved: boolean;
        readonly asApproved: ITuple<[u32, Option<PalletReferendaDeposit>, Option<PalletReferendaDeposit>]>;
        readonly isRejected: boolean;
        readonly asRejected: ITuple<[u32, Option<PalletReferendaDeposit>, Option<PalletReferendaDeposit>]>;
        readonly isCancelled: boolean;
        readonly asCancelled: ITuple<[u32, Option<PalletReferendaDeposit>, Option<PalletReferendaDeposit>]>;
        readonly isTimedOut: boolean;
        readonly asTimedOut: ITuple<[u32, Option<PalletReferendaDeposit>, Option<PalletReferendaDeposit>]>;
        readonly isKilled: boolean;
        readonly asKilled: u32;
        readonly type: "Ongoing" | "Approved" | "Rejected" | "Cancelled" | "TimedOut" | "Killed";
    }

    /** @name PalletReferendaReferendumStatusConvictionVotingTally (743) */
    interface PalletReferendaReferendumStatusConvictionVotingTally extends Struct {
        readonly track: u16;
        readonly origin: DancelightRuntimeOriginCaller;
        readonly proposal: FrameSupportPreimagesBounded;
        readonly enactment: FrameSupportScheduleDispatchTime;
        readonly submitted: u32;
        readonly submissionDeposit: PalletReferendaDeposit;
        readonly decisionDeposit: Option<PalletReferendaDeposit>;
        readonly deciding: Option<PalletReferendaDecidingStatus>;
        readonly tally: PalletConvictionVotingTally;
        readonly inQueue: bool;
        readonly alarm: Option<ITuple<[u32, ITuple<[u32, u32]>]>>;
    }

    /** @name PalletReferendaDeposit (744) */
    interface PalletReferendaDeposit extends Struct {
        readonly who: AccountId32;
        readonly amount: u128;
    }

    /** @name PalletReferendaDecidingStatus (747) */
    interface PalletReferendaDecidingStatus extends Struct {
        readonly since: u32;
        readonly confirming: Option<u32>;
    }

    /** @name PalletReferendaTrackDetails (755) */
    interface PalletReferendaTrackDetails extends Struct {
        readonly name: Text;
        readonly maxDeciding: u32;
        readonly decisionDeposit: u128;
        readonly preparePeriod: u32;
        readonly decisionPeriod: u32;
        readonly confirmPeriod: u32;
        readonly minEnactmentPeriod: u32;
        readonly minApproval: PalletReferendaCurve;
        readonly minSupport: PalletReferendaCurve;
    }

    /** @name PalletReferendaCurve (756) */
    interface PalletReferendaCurve extends Enum {
        readonly isLinearDecreasing: boolean;
        readonly asLinearDecreasing: {
            readonly length: Perbill;
            readonly floor: Perbill;
            readonly ceil: Perbill;
        } & Struct;
        readonly isSteppedDecreasing: boolean;
        readonly asSteppedDecreasing: {
            readonly begin: Perbill;
            readonly end: Perbill;
            readonly step: Perbill;
            readonly period: Perbill;
        } & Struct;
        readonly isReciprocal: boolean;
        readonly asReciprocal: {
            readonly factor: i64;
            readonly xOffset: i64;
            readonly yOffset: i64;
        } & Struct;
        readonly type: "LinearDecreasing" | "SteppedDecreasing" | "Reciprocal";
    }

    /** @name PalletReferendaError (759) */
    interface PalletReferendaError extends Enum {
        readonly isNotOngoing: boolean;
        readonly isHasDeposit: boolean;
        readonly isBadTrack: boolean;
        readonly isFull: boolean;
        readonly isQueueEmpty: boolean;
        readonly isBadReferendum: boolean;
        readonly isNothingToDo: boolean;
        readonly isNoTrack: boolean;
        readonly isUnfinished: boolean;
        readonly isNoPermission: boolean;
        readonly isNoDeposit: boolean;
        readonly isBadStatus: boolean;
        readonly isPreimageNotExist: boolean;
        readonly isPreimageStoredWithDifferentLength: boolean;
        readonly type:
            | "NotOngoing"
            | "HasDeposit"
            | "BadTrack"
            | "Full"
            | "QueueEmpty"
            | "BadReferendum"
            | "NothingToDo"
            | "NoTrack"
            | "Unfinished"
            | "NoPermission"
            | "NoDeposit"
            | "BadStatus"
            | "PreimageNotExist"
            | "PreimageStoredWithDifferentLength";
    }

    /** @name PalletRankedCollectiveMemberRecord (760) */
    interface PalletRankedCollectiveMemberRecord extends Struct {
        readonly rank: u16;
    }

    /** @name PalletRankedCollectiveError (764) */
    interface PalletRankedCollectiveError extends Enum {
        readonly isAlreadyMember: boolean;
        readonly isNotMember: boolean;
        readonly isNotPolling: boolean;
        readonly isOngoing: boolean;
        readonly isNoneRemaining: boolean;
        readonly isCorruption: boolean;
        readonly isRankTooLow: boolean;
        readonly isInvalidWitness: boolean;
        readonly isNoPermission: boolean;
        readonly isSameMember: boolean;
        readonly isTooManyMembers: boolean;
        readonly type:
            | "AlreadyMember"
            | "NotMember"
            | "NotPolling"
            | "Ongoing"
            | "NoneRemaining"
            | "Corruption"
            | "RankTooLow"
            | "InvalidWitness"
            | "NoPermission"
            | "SameMember"
            | "TooManyMembers";
    }

    /** @name PalletReferendaReferendumInfoRankedCollectiveTally (765) */
    interface PalletReferendaReferendumInfoRankedCollectiveTally extends Enum {
        readonly isOngoing: boolean;
        readonly asOngoing: PalletReferendaReferendumStatusRankedCollectiveTally;
        readonly isApproved: boolean;
        readonly asApproved: ITuple<[u32, Option<PalletReferendaDeposit>, Option<PalletReferendaDeposit>]>;
        readonly isRejected: boolean;
        readonly asRejected: ITuple<[u32, Option<PalletReferendaDeposit>, Option<PalletReferendaDeposit>]>;
        readonly isCancelled: boolean;
        readonly asCancelled: ITuple<[u32, Option<PalletReferendaDeposit>, Option<PalletReferendaDeposit>]>;
        readonly isTimedOut: boolean;
        readonly asTimedOut: ITuple<[u32, Option<PalletReferendaDeposit>, Option<PalletReferendaDeposit>]>;
        readonly isKilled: boolean;
        readonly asKilled: u32;
        readonly type: "Ongoing" | "Approved" | "Rejected" | "Cancelled" | "TimedOut" | "Killed";
    }

    /** @name PalletReferendaReferendumStatusRankedCollectiveTally (766) */
    interface PalletReferendaReferendumStatusRankedCollectiveTally extends Struct {
        readonly track: u16;
        readonly origin: DancelightRuntimeOriginCaller;
        readonly proposal: FrameSupportPreimagesBounded;
        readonly enactment: FrameSupportScheduleDispatchTime;
        readonly submitted: u32;
        readonly submissionDeposit: PalletReferendaDeposit;
        readonly decisionDeposit: Option<PalletReferendaDeposit>;
        readonly deciding: Option<PalletReferendaDecidingStatus>;
        readonly tally: PalletRankedCollectiveTally;
        readonly inQueue: bool;
        readonly alarm: Option<ITuple<[u32, ITuple<[u32, u32]>]>>;
    }

    /** @name PalletWhitelistError (769) */
    interface PalletWhitelistError extends Enum {
        readonly isUnavailablePreImage: boolean;
        readonly isUndecodableCall: boolean;
        readonly isInvalidCallWeightWitness: boolean;
        readonly isCallIsNotWhitelisted: boolean;
        readonly isCallAlreadyWhitelisted: boolean;
        readonly type:
            | "UnavailablePreImage"
            | "UndecodableCall"
            | "InvalidCallWeightWitness"
            | "CallIsNotWhitelisted"
            | "CallAlreadyWhitelisted";
    }

    /** @name PalletCollectiveVotes (771) */
    interface PalletCollectiveVotes extends Struct {
        readonly index: u32;
        readonly threshold: u32;
        readonly ayes: Vec<AccountId32>;
        readonly nays: Vec<AccountId32>;
        readonly end: u32;
    }

    /** @name PalletCollectiveError (772) */
    interface PalletCollectiveError extends Enum {
        readonly isNotMember: boolean;
        readonly isDuplicateProposal: boolean;
        readonly isProposalMissing: boolean;
        readonly isWrongIndex: boolean;
        readonly isDuplicateVote: boolean;
        readonly isAlreadyInitialized: boolean;
        readonly isTooEarly: boolean;
        readonly isTooManyProposals: boolean;
        readonly isWrongProposalWeight: boolean;
        readonly isWrongProposalLength: boolean;
        readonly isPrimeAccountNotMember: boolean;
        readonly isProposalActive: boolean;
        readonly type:
            | "NotMember"
            | "DuplicateProposal"
            | "ProposalMissing"
            | "WrongIndex"
            | "DuplicateVote"
            | "AlreadyInitialized"
            | "TooEarly"
            | "TooManyProposals"
            | "WrongProposalWeight"
            | "WrongProposalLength"
            | "PrimeAccountNotMember"
            | "ProposalActive";
    }

    /** @name PolkadotRuntimeParachainsConfigurationHostConfiguration (773) */
    interface PolkadotRuntimeParachainsConfigurationHostConfiguration extends Struct {
        readonly maxCodeSize: u32;
        readonly maxHeadDataSize: u32;
        readonly maxUpwardQueueCount: u32;
        readonly maxUpwardQueueSize: u32;
        readonly maxUpwardMessageSize: u32;
        readonly maxUpwardMessageNumPerCandidate: u32;
        readonly hrmpMaxMessageNumPerCandidate: u32;
        readonly validationUpgradeCooldown: u32;
        readonly validationUpgradeDelay: u32;
        readonly asyncBackingParams: PolkadotPrimitivesV8AsyncBackingAsyncBackingParams;
        readonly maxPovSize: u32;
        readonly maxDownwardMessageSize: u32;
        readonly hrmpMaxParachainOutboundChannels: u32;
        readonly hrmpSenderDeposit: u128;
        readonly hrmpRecipientDeposit: u128;
        readonly hrmpChannelMaxCapacity: u32;
        readonly hrmpChannelMaxTotalSize: u32;
        readonly hrmpMaxParachainInboundChannels: u32;
        readonly hrmpChannelMaxMessageSize: u32;
        readonly executorParams: PolkadotPrimitivesV8ExecutorParams;
        readonly codeRetentionPeriod: u32;
        readonly maxValidators: Option<u32>;
        readonly disputePeriod: u32;
        readonly disputePostConclusionAcceptancePeriod: u32;
        readonly noShowSlots: u32;
        readonly nDelayTranches: u32;
        readonly zerothDelayTrancheWidth: u32;
        readonly neededApprovals: u32;
        readonly relayVrfModuloSamples: u32;
        readonly pvfVotingTtl: u32;
        readonly minimumValidationUpgradeDelay: u32;
        readonly minimumBackingVotes: u32;
        readonly nodeFeatures: BitVec;
        readonly approvalVotingParams: PolkadotPrimitivesV8ApprovalVotingParams;
        readonly schedulerParams: PolkadotPrimitivesV8SchedulerParams;
    }

    /** @name PolkadotRuntimeParachainsConfigurationPalletError (776) */
    interface PolkadotRuntimeParachainsConfigurationPalletError extends Enum {
        readonly isInvalidNewValue: boolean;
        readonly type: "InvalidNewValue";
    }

    /** @name PolkadotRuntimeParachainsSharedAllowedRelayParentsTracker (779) */
    interface PolkadotRuntimeParachainsSharedAllowedRelayParentsTracker extends Struct {
        readonly buffer: Vec<PolkadotRuntimeParachainsSharedRelayParentInfo>;
        readonly latestNumber: u32;
    }

    /** @name PolkadotRuntimeParachainsSharedRelayParentInfo (781) */
    interface PolkadotRuntimeParachainsSharedRelayParentInfo extends Struct {
        readonly relayParent: H256;
        readonly stateRoot: H256;
        readonly claimQueue: BTreeMap<u32, BTreeMap<u8, BTreeSet<u32>>>;
    }

    /** @name PolkadotRuntimeParachainsInclusionCandidatePendingAvailability (791) */
    interface PolkadotRuntimeParachainsInclusionCandidatePendingAvailability extends Struct {
        readonly core: u32;
        readonly hash_: H256;
        readonly descriptor: PolkadotPrimitivesVstagingCandidateDescriptorV2;
        readonly commitments: PolkadotPrimitivesV8CandidateCommitments;
        readonly availabilityVotes: BitVec;
        readonly backers: BitVec;
        readonly relayParentNumber: u32;
        readonly backedInNumber: u32;
        readonly backingGroup: u32;
    }

    /** @name PolkadotRuntimeParachainsInclusionPalletError (792) */
    interface PolkadotRuntimeParachainsInclusionPalletError extends Enum {
        readonly isValidatorIndexOutOfBounds: boolean;
        readonly isUnscheduledCandidate: boolean;
        readonly isHeadDataTooLarge: boolean;
        readonly isPrematureCodeUpgrade: boolean;
        readonly isNewCodeTooLarge: boolean;
        readonly isDisallowedRelayParent: boolean;
        readonly isInvalidAssignment: boolean;
        readonly isInvalidGroupIndex: boolean;
        readonly isInsufficientBacking: boolean;
        readonly isInvalidBacking: boolean;
        readonly isValidationDataHashMismatch: boolean;
        readonly isIncorrectDownwardMessageHandling: boolean;
        readonly isInvalidUpwardMessages: boolean;
        readonly isHrmpWatermarkMishandling: boolean;
        readonly isInvalidOutboundHrmp: boolean;
        readonly isInvalidValidationCodeHash: boolean;
        readonly isParaHeadMismatch: boolean;
        readonly type:
            | "ValidatorIndexOutOfBounds"
            | "UnscheduledCandidate"
            | "HeadDataTooLarge"
            | "PrematureCodeUpgrade"
            | "NewCodeTooLarge"
            | "DisallowedRelayParent"
            | "InvalidAssignment"
            | "InvalidGroupIndex"
            | "InsufficientBacking"
            | "InvalidBacking"
            | "ValidationDataHashMismatch"
            | "IncorrectDownwardMessageHandling"
            | "InvalidUpwardMessages"
            | "HrmpWatermarkMishandling"
            | "InvalidOutboundHrmp"
            | "InvalidValidationCodeHash"
            | "ParaHeadMismatch";
    }

    /** @name PolkadotPrimitivesVstagingScrapedOnChainVotes (793) */
    interface PolkadotPrimitivesVstagingScrapedOnChainVotes extends Struct {
        readonly session: u32;
        readonly backingValidatorsPerCandidate: Vec<
            ITuple<
                [
                    PolkadotPrimitivesVstagingCandidateReceiptV2,
                    Vec<ITuple<[u32, PolkadotPrimitivesV8ValidityAttestation]>>,
                ]
            >
        >;
        readonly disputes: Vec<PolkadotPrimitivesV8DisputeStatementSet>;
    }

    /** @name PolkadotRuntimeParachainsParasInherentPalletError (798) */
    interface PolkadotRuntimeParachainsParasInherentPalletError extends Enum {
        readonly isTooManyInclusionInherents: boolean;
        readonly isInvalidParentHeader: boolean;
        readonly isInherentDataFilteredDuringExecution: boolean;
        readonly isUnscheduledCandidate: boolean;
        readonly type:
            | "TooManyInclusionInherents"
            | "InvalidParentHeader"
            | "InherentDataFilteredDuringExecution"
            | "UnscheduledCandidate";
    }

    /** @name PolkadotRuntimeParachainsSchedulerCommonAssignment (802) */
    interface PolkadotRuntimeParachainsSchedulerCommonAssignment extends Enum {
        readonly isPool: boolean;
        readonly asPool: {
            readonly paraId: u32;
            readonly coreIndex: u32;
        } & Struct;
        readonly isBulk: boolean;
        readonly asBulk: u32;
        readonly type: "Pool" | "Bulk";
    }

    /** @name PolkadotRuntimeParachainsParasPvfCheckActiveVoteState (805) */
    interface PolkadotRuntimeParachainsParasPvfCheckActiveVoteState extends Struct {
        readonly votesAccept: BitVec;
        readonly votesReject: BitVec;
        readonly age: u32;
        readonly createdAt: u32;
        readonly causes: Vec<PolkadotRuntimeParachainsParasPvfCheckCause>;
    }

    /** @name PolkadotRuntimeParachainsParasPvfCheckCause (807) */
    interface PolkadotRuntimeParachainsParasPvfCheckCause extends Enum {
        readonly isOnboarding: boolean;
        readonly asOnboarding: u32;
        readonly isUpgrade: boolean;
        readonly asUpgrade: {
            readonly id: u32;
            readonly includedAt: u32;
            readonly upgradeStrategy: PolkadotRuntimeParachainsParasUpgradeStrategy;
        } & Struct;
        readonly type: "Onboarding" | "Upgrade";
    }

    /** @name PolkadotRuntimeParachainsParasUpgradeStrategy (808) */
    interface PolkadotRuntimeParachainsParasUpgradeStrategy extends Enum {
        readonly isSetGoAheadSignal: boolean;
        readonly isApplyAtExpectedBlock: boolean;
        readonly type: "SetGoAheadSignal" | "ApplyAtExpectedBlock";
    }

    /** @name PolkadotRuntimeParachainsParasParaLifecycle (810) */
    interface PolkadotRuntimeParachainsParasParaLifecycle extends Enum {
        readonly isOnboarding: boolean;
        readonly isParathread: boolean;
        readonly isParachain: boolean;
        readonly isUpgradingParathread: boolean;
        readonly isDowngradingParachain: boolean;
        readonly isOffboardingParathread: boolean;
        readonly isOffboardingParachain: boolean;
        readonly type:
            | "Onboarding"
            | "Parathread"
            | "Parachain"
            | "UpgradingParathread"
            | "DowngradingParachain"
            | "OffboardingParathread"
            | "OffboardingParachain";
    }

    /** @name PolkadotRuntimeParachainsParasParaPastCodeMeta (812) */
    interface PolkadotRuntimeParachainsParasParaPastCodeMeta extends Struct {
        readonly upgradeTimes: Vec<PolkadotRuntimeParachainsParasReplacementTimes>;
        readonly lastPruned: Option<u32>;
    }

    /** @name PolkadotRuntimeParachainsParasReplacementTimes (814) */
    interface PolkadotRuntimeParachainsParasReplacementTimes extends Struct {
        readonly expectedAt: u32;
        readonly activatedAt: u32;
    }

    /** @name PolkadotRuntimeParachainsParasAuthorizedCodeHashAndExpiry (816) */
    interface PolkadotRuntimeParachainsParasAuthorizedCodeHashAndExpiry extends Struct {
        readonly codeHash: H256;
        readonly expireAt: u32;
    }

    /** @name PolkadotPrimitivesV8UpgradeGoAhead (817) */
    interface PolkadotPrimitivesV8UpgradeGoAhead extends Enum {
        readonly isAbort: boolean;
        readonly isGoAhead: boolean;
        readonly type: "Abort" | "GoAhead";
    }

    /** @name PolkadotPrimitivesV8UpgradeRestriction (818) */
    interface PolkadotPrimitivesV8UpgradeRestriction extends Enum {
        readonly isPresent: boolean;
        readonly type: "Present";
    }

    /** @name PolkadotRuntimeParachainsParasPalletError (819) */
    interface PolkadotRuntimeParachainsParasPalletError extends Enum {
        readonly isNotRegistered: boolean;
        readonly isCannotOnboard: boolean;
        readonly isCannotOffboard: boolean;
        readonly isCannotUpgrade: boolean;
        readonly isCannotDowngrade: boolean;
        readonly isPvfCheckStatementStale: boolean;
        readonly isPvfCheckStatementFuture: boolean;
        readonly isPvfCheckValidatorIndexOutOfBounds: boolean;
        readonly isPvfCheckInvalidSignature: boolean;
        readonly isPvfCheckDoubleVote: boolean;
        readonly isPvfCheckSubjectInvalid: boolean;
        readonly isCannotUpgradeCode: boolean;
        readonly isInvalidCode: boolean;
        readonly isNothingAuthorized: boolean;
        readonly isUnauthorized: boolean;
        readonly isInvalidBlockNumber: boolean;
        readonly type:
            | "NotRegistered"
            | "CannotOnboard"
            | "CannotOffboard"
            | "CannotUpgrade"
            | "CannotDowngrade"
            | "PvfCheckStatementStale"
            | "PvfCheckStatementFuture"
            | "PvfCheckValidatorIndexOutOfBounds"
            | "PvfCheckInvalidSignature"
            | "PvfCheckDoubleVote"
            | "PvfCheckSubjectInvalid"
            | "CannotUpgradeCode"
            | "InvalidCode"
            | "NothingAuthorized"
            | "Unauthorized"
            | "InvalidBlockNumber";
    }

    /** @name PolkadotRuntimeParachainsInitializerBufferedSessionChange (821) */
    interface PolkadotRuntimeParachainsInitializerBufferedSessionChange extends Struct {
        readonly validators: Vec<PolkadotPrimitivesV8ValidatorAppPublic>;
        readonly queued: Vec<PolkadotPrimitivesV8ValidatorAppPublic>;
        readonly sessionIndex: u32;
    }

    /** @name PolkadotCorePrimitivesInboundDownwardMessage (823) */
    interface PolkadotCorePrimitivesInboundDownwardMessage extends Struct {
        readonly sentAt: u32;
        readonly msg: Bytes;
    }

    /** @name PolkadotRuntimeParachainsHrmpHrmpOpenChannelRequest (824) */
    interface PolkadotRuntimeParachainsHrmpHrmpOpenChannelRequest extends Struct {
        readonly confirmed: bool;
        readonly age: u32;
        readonly senderDeposit: u128;
        readonly maxMessageSize: u32;
        readonly maxCapacity: u32;
        readonly maxTotalSize: u32;
    }

    /** @name PolkadotRuntimeParachainsHrmpHrmpChannel (826) */
    interface PolkadotRuntimeParachainsHrmpHrmpChannel extends Struct {
        readonly maxCapacity: u32;
        readonly maxTotalSize: u32;
        readonly maxMessageSize: u32;
        readonly msgCount: u32;
        readonly totalSize: u32;
        readonly mqcHead: Option<H256>;
        readonly senderDeposit: u128;
        readonly recipientDeposit: u128;
    }

    /** @name PolkadotCorePrimitivesInboundHrmpMessage (828) */
    interface PolkadotCorePrimitivesInboundHrmpMessage extends Struct {
        readonly sentAt: u32;
        readonly data: Bytes;
    }

    /** @name PolkadotRuntimeParachainsHrmpPalletError (831) */
    interface PolkadotRuntimeParachainsHrmpPalletError extends Enum {
        readonly isOpenHrmpChannelToSelf: boolean;
        readonly isOpenHrmpChannelInvalidRecipient: boolean;
        readonly isOpenHrmpChannelZeroCapacity: boolean;
        readonly isOpenHrmpChannelCapacityExceedsLimit: boolean;
        readonly isOpenHrmpChannelZeroMessageSize: boolean;
        readonly isOpenHrmpChannelMessageSizeExceedsLimit: boolean;
        readonly isOpenHrmpChannelAlreadyExists: boolean;
        readonly isOpenHrmpChannelAlreadyRequested: boolean;
        readonly isOpenHrmpChannelLimitExceeded: boolean;
        readonly isAcceptHrmpChannelDoesntExist: boolean;
        readonly isAcceptHrmpChannelAlreadyConfirmed: boolean;
        readonly isAcceptHrmpChannelLimitExceeded: boolean;
        readonly isCloseHrmpChannelUnauthorized: boolean;
        readonly isCloseHrmpChannelDoesntExist: boolean;
        readonly isCloseHrmpChannelAlreadyUnderway: boolean;
        readonly isCancelHrmpOpenChannelUnauthorized: boolean;
        readonly isOpenHrmpChannelDoesntExist: boolean;
        readonly isOpenHrmpChannelAlreadyConfirmed: boolean;
        readonly isWrongWitness: boolean;
        readonly isChannelCreationNotAuthorized: boolean;
        readonly type:
            | "OpenHrmpChannelToSelf"
            | "OpenHrmpChannelInvalidRecipient"
            | "OpenHrmpChannelZeroCapacity"
            | "OpenHrmpChannelCapacityExceedsLimit"
            | "OpenHrmpChannelZeroMessageSize"
            | "OpenHrmpChannelMessageSizeExceedsLimit"
            | "OpenHrmpChannelAlreadyExists"
            | "OpenHrmpChannelAlreadyRequested"
            | "OpenHrmpChannelLimitExceeded"
            | "AcceptHrmpChannelDoesntExist"
            | "AcceptHrmpChannelAlreadyConfirmed"
            | "AcceptHrmpChannelLimitExceeded"
            | "CloseHrmpChannelUnauthorized"
            | "CloseHrmpChannelDoesntExist"
            | "CloseHrmpChannelAlreadyUnderway"
            | "CancelHrmpOpenChannelUnauthorized"
            | "OpenHrmpChannelDoesntExist"
            | "OpenHrmpChannelAlreadyConfirmed"
            | "WrongWitness"
            | "ChannelCreationNotAuthorized";
    }

    /** @name PolkadotPrimitivesV8SessionInfo (833) */
    interface PolkadotPrimitivesV8SessionInfo extends Struct {
        readonly activeValidatorIndices: Vec<u32>;
        readonly randomSeed: U8aFixed;
        readonly disputePeriod: u32;
        readonly validators: PolkadotPrimitivesV8IndexedVecValidatorIndex;
        readonly discoveryKeys: Vec<SpAuthorityDiscoveryAppPublic>;
        readonly assignmentKeys: Vec<PolkadotPrimitivesV8AssignmentAppPublic>;
        readonly validatorGroups: PolkadotPrimitivesV8IndexedVecGroupIndex;
        readonly nCores: u32;
        readonly zerothDelayTrancheWidth: u32;
        readonly relayVrfModuloSamples: u32;
        readonly nDelayTranches: u32;
        readonly noShowSlots: u32;
        readonly neededApprovals: u32;
    }

    /** @name PolkadotPrimitivesV8IndexedVecValidatorIndex (834) */
    interface PolkadotPrimitivesV8IndexedVecValidatorIndex extends Vec<PolkadotPrimitivesV8ValidatorAppPublic> {}

    /** @name PolkadotPrimitivesV8IndexedVecGroupIndex (835) */
    interface PolkadotPrimitivesV8IndexedVecGroupIndex extends Vec<Vec<u32>> {}

    /** @name PolkadotPrimitivesV8DisputeState (837) */
    interface PolkadotPrimitivesV8DisputeState extends Struct {
        readonly validatorsFor: BitVec;
        readonly validatorsAgainst: BitVec;
        readonly start: u32;
        readonly concludedAt: Option<u32>;
    }

    /** @name PolkadotRuntimeParachainsDisputesPalletError (839) */
    interface PolkadotRuntimeParachainsDisputesPalletError extends Enum {
        readonly isDuplicateDisputeStatementSets: boolean;
        readonly isAncientDisputeStatement: boolean;
        readonly isValidatorIndexOutOfBounds: boolean;
        readonly isInvalidSignature: boolean;
        readonly isDuplicateStatement: boolean;
        readonly isSingleSidedDispute: boolean;
        readonly isMaliciousBacker: boolean;
        readonly isMissingBackingVotes: boolean;
        readonly isUnconfirmedDispute: boolean;
        readonly type:
            | "DuplicateDisputeStatementSets"
            | "AncientDisputeStatement"
            | "ValidatorIndexOutOfBounds"
            | "InvalidSignature"
            | "DuplicateStatement"
            | "SingleSidedDispute"
            | "MaliciousBacker"
            | "MissingBackingVotes"
            | "UnconfirmedDispute";
    }

    /** @name PolkadotPrimitivesVstagingPendingSlashes (840) */
    interface PolkadotPrimitivesVstagingPendingSlashes extends Struct {
        readonly keys_: BTreeMap<u32, PolkadotPrimitivesV8ValidatorAppPublic>;
        readonly kind: PolkadotPrimitivesVstagingDisputeOffenceKind;
    }

    /** @name PolkadotRuntimeParachainsDisputesSlashingPalletError (844) */
    interface PolkadotRuntimeParachainsDisputesSlashingPalletError extends Enum {
        readonly isInvalidKeyOwnershipProof: boolean;
        readonly isInvalidSessionIndex: boolean;
        readonly isInvalidCandidateHash: boolean;
        readonly isInvalidValidatorIndex: boolean;
        readonly isValidatorIndexIdMismatch: boolean;
        readonly isDuplicateSlashingReport: boolean;
        readonly type:
            | "InvalidKeyOwnershipProof"
            | "InvalidSessionIndex"
            | "InvalidCandidateHash"
            | "InvalidValidatorIndex"
            | "ValidatorIndexIdMismatch"
            | "DuplicateSlashingReport";
    }

    /** @name PalletMessageQueueBookState (845) */
    interface PalletMessageQueueBookState extends Struct {
        readonly begin: u32;
        readonly end: u32;
        readonly count: u32;
        readonly readyNeighbours: Option<PalletMessageQueueNeighbours>;
        readonly messageCount: u64;
        readonly size_: u64;
    }

    /** @name PalletMessageQueueNeighbours (847) */
    interface PalletMessageQueueNeighbours extends Struct {
        readonly prev: DancelightRuntimeAggregateMessageOrigin;
        readonly next: DancelightRuntimeAggregateMessageOrigin;
    }

    /** @name PalletMessageQueuePage (849) */
    interface PalletMessageQueuePage extends Struct {
        readonly remaining: u32;
        readonly remainingSize: u32;
        readonly firstIndex: u32;
        readonly first: u32;
        readonly last: u32;
        readonly heap: Bytes;
    }

    /** @name PalletMessageQueueError (851) */
    interface PalletMessageQueueError extends Enum {
        readonly isNotReapable: boolean;
        readonly isNoPage: boolean;
        readonly isNoMessage: boolean;
        readonly isAlreadyProcessed: boolean;
        readonly isQueued: boolean;
        readonly isInsufficientWeight: boolean;
        readonly isTemporarilyUnprocessable: boolean;
        readonly isQueuePaused: boolean;
        readonly isRecursiveDisallowed: boolean;
        readonly type:
            | "NotReapable"
            | "NoPage"
            | "NoMessage"
            | "AlreadyProcessed"
            | "Queued"
            | "InsufficientWeight"
            | "TemporarilyUnprocessable"
            | "QueuePaused"
            | "RecursiveDisallowed";
    }

    /** @name PolkadotRuntimeParachainsOnDemandTypesCoreAffinityCount (852) */
    interface PolkadotRuntimeParachainsOnDemandTypesCoreAffinityCount extends Struct {
        readonly coreIndex: u32;
        readonly count: u32;
    }

    /** @name PolkadotRuntimeParachainsOnDemandTypesQueueStatusType (853) */
    interface PolkadotRuntimeParachainsOnDemandTypesQueueStatusType extends Struct {
        readonly traffic: u128;
        readonly nextIndex: u32;
        readonly smallestIndex: u32;
        readonly freedIndices: BinaryHeapReverseQueueIndex;
    }

    /** @name BinaryHeapReverseQueueIndex (855) */
    interface BinaryHeapReverseQueueIndex extends Vec<u32> {}

    /** @name BinaryHeapEnqueuedOrder (858) */
    interface BinaryHeapEnqueuedOrder extends Vec<PolkadotRuntimeParachainsOnDemandTypesEnqueuedOrder> {}

    /** @name PolkadotRuntimeParachainsOnDemandTypesEnqueuedOrder (859) */
    interface PolkadotRuntimeParachainsOnDemandTypesEnqueuedOrder extends Struct {
        readonly paraId: u32;
        readonly idx: u32;
    }

    /** @name PolkadotRuntimeParachainsOnDemandPalletError (863) */
    interface PolkadotRuntimeParachainsOnDemandPalletError extends Enum {
        readonly isQueueFull: boolean;
        readonly isSpotPriceHigherThanMaxAmount: boolean;
        readonly isInsufficientCredits: boolean;
        readonly type: "QueueFull" | "SpotPriceHigherThanMaxAmount" | "InsufficientCredits";
    }

    /** @name PolkadotRuntimeCommonParasRegistrarParaInfo (864) */
    interface PolkadotRuntimeCommonParasRegistrarParaInfo extends Struct {
        readonly manager: AccountId32;
        readonly deposit: u128;
        readonly locked: Option<bool>;
    }

    /** @name PolkadotRuntimeCommonParasRegistrarPalletError (866) */
    interface PolkadotRuntimeCommonParasRegistrarPalletError extends Enum {
        readonly isNotRegistered: boolean;
        readonly isAlreadyRegistered: boolean;
        readonly isNotOwner: boolean;
        readonly isCodeTooLarge: boolean;
        readonly isHeadDataTooLarge: boolean;
        readonly isNotParachain: boolean;
        readonly isNotParathread: boolean;
        readonly isCannotDeregister: boolean;
        readonly isCannotDowngrade: boolean;
        readonly isCannotUpgrade: boolean;
        readonly isParaLocked: boolean;
        readonly isNotReserved: boolean;
        readonly isInvalidCode: boolean;
        readonly isCannotSwap: boolean;
        readonly type:
            | "NotRegistered"
            | "AlreadyRegistered"
            | "NotOwner"
            | "CodeTooLarge"
            | "HeadDataTooLarge"
            | "NotParachain"
            | "NotParathread"
            | "CannotDeregister"
            | "CannotDowngrade"
            | "CannotUpgrade"
            | "ParaLocked"
            | "NotReserved"
            | "InvalidCode"
            | "CannotSwap";
    }

    /** @name PalletUtilityError (867) */
    interface PalletUtilityError extends Enum {
        readonly isTooManyCalls: boolean;
        readonly type: "TooManyCalls";
    }

    /** @name PalletIdentityRegistration (868) */
    interface PalletIdentityRegistration extends Struct {
        readonly judgements: Vec<ITuple<[u32, PalletIdentityJudgement]>>;
        readonly deposit: u128;
        readonly info: PalletIdentityLegacyIdentityInfo;
    }

    /** @name PalletIdentityRegistrarInfo (876) */
    interface PalletIdentityRegistrarInfo extends Struct {
        readonly account: AccountId32;
        readonly fee: u128;
        readonly fields: u64;
    }

    /** @name PalletIdentityAuthorityProperties (879) */
    interface PalletIdentityAuthorityProperties extends Struct {
        readonly accountId: AccountId32;
        readonly allocation: u32;
    }

    /** @name PalletIdentityUsernameInformation (880) */
    interface PalletIdentityUsernameInformation extends Struct {
        readonly owner: AccountId32;
        readonly provider: PalletIdentityProvider;
    }

    /** @name PalletIdentityProvider (881) */
    interface PalletIdentityProvider extends Enum {
        readonly isAllocation: boolean;
        readonly isAuthorityDeposit: boolean;
        readonly asAuthorityDeposit: u128;
        readonly isSystem: boolean;
        readonly type: "Allocation" | "AuthorityDeposit" | "System";
    }

    /** @name PalletIdentityError (883) */
    interface PalletIdentityError extends Enum {
        readonly isTooManySubAccounts: boolean;
        readonly isNotFound: boolean;
        readonly isNotNamed: boolean;
        readonly isEmptyIndex: boolean;
        readonly isFeeChanged: boolean;
        readonly isNoIdentity: boolean;
        readonly isStickyJudgement: boolean;
        readonly isJudgementGiven: boolean;
        readonly isInvalidJudgement: boolean;
        readonly isInvalidIndex: boolean;
        readonly isInvalidTarget: boolean;
        readonly isTooManyRegistrars: boolean;
        readonly isAlreadyClaimed: boolean;
        readonly isNotSub: boolean;
        readonly isNotOwned: boolean;
        readonly isJudgementForDifferentIdentity: boolean;
        readonly isJudgementPaymentFailed: boolean;
        readonly isInvalidSuffix: boolean;
        readonly isNotUsernameAuthority: boolean;
        readonly isNoAllocation: boolean;
        readonly isInvalidSignature: boolean;
        readonly isRequiresSignature: boolean;
        readonly isInvalidUsername: boolean;
        readonly isUsernameTaken: boolean;
        readonly isNoUsername: boolean;
        readonly isNotExpired: boolean;
        readonly isTooEarly: boolean;
        readonly isNotUnbinding: boolean;
        readonly isAlreadyUnbinding: boolean;
        readonly isInsufficientPrivileges: boolean;
        readonly type:
            | "TooManySubAccounts"
            | "NotFound"
            | "NotNamed"
            | "EmptyIndex"
            | "FeeChanged"
            | "NoIdentity"
            | "StickyJudgement"
            | "JudgementGiven"
            | "InvalidJudgement"
            | "InvalidIndex"
            | "InvalidTarget"
            | "TooManyRegistrars"
            | "AlreadyClaimed"
            | "NotSub"
            | "NotOwned"
            | "JudgementForDifferentIdentity"
            | "JudgementPaymentFailed"
            | "InvalidSuffix"
            | "NotUsernameAuthority"
            | "NoAllocation"
            | "InvalidSignature"
            | "RequiresSignature"
            | "InvalidUsername"
            | "UsernameTaken"
            | "NoUsername"
            | "NotExpired"
            | "TooEarly"
            | "NotUnbinding"
            | "AlreadyUnbinding"
            | "InsufficientPrivileges";
    }

    /** @name PalletSchedulerScheduled (886) */
    interface PalletSchedulerScheduled extends Struct {
        readonly maybeId: Option<U8aFixed>;
        readonly priority: u8;
        readonly call: FrameSupportPreimagesBounded;
        readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
        readonly origin: DancelightRuntimeOriginCaller;
    }

    /** @name PalletSchedulerRetryConfig (888) */
    interface PalletSchedulerRetryConfig extends Struct {
        readonly totalRetries: u8;
        readonly remaining: u8;
        readonly period: u32;
    }

    /** @name PalletSchedulerError (889) */
    interface PalletSchedulerError extends Enum {
        readonly isFailedToSchedule: boolean;
        readonly isNotFound: boolean;
        readonly isTargetBlockNumberInPast: boolean;
        readonly isRescheduleNoChange: boolean;
        readonly isNamed: boolean;
        readonly type: "FailedToSchedule" | "NotFound" | "TargetBlockNumberInPast" | "RescheduleNoChange" | "Named";
    }

    /** @name PalletProxyProxyDefinition (892) */
    interface PalletProxyProxyDefinition extends Struct {
        readonly delegate: AccountId32;
        readonly proxyType: DancelightRuntimeProxyType;
        readonly delay: u32;
    }

    /** @name PalletProxyAnnouncement (896) */
    interface PalletProxyAnnouncement extends Struct {
        readonly real: AccountId32;
        readonly callHash: H256;
        readonly height: u32;
    }

    /** @name PalletProxyError (898) */
    interface PalletProxyError extends Enum {
        readonly isTooMany: boolean;
        readonly isNotFound: boolean;
        readonly isNotProxy: boolean;
        readonly isUnproxyable: boolean;
        readonly isDuplicate: boolean;
        readonly isNoPermission: boolean;
        readonly isUnannounced: boolean;
        readonly isNoSelfProxy: boolean;
        readonly type:
            | "TooMany"
            | "NotFound"
            | "NotProxy"
            | "Unproxyable"
            | "Duplicate"
            | "NoPermission"
            | "Unannounced"
            | "NoSelfProxy";
    }

    /** @name PalletMultisigMultisig (900) */
    interface PalletMultisigMultisig extends Struct {
        readonly when: PalletMultisigTimepoint;
        readonly deposit: u128;
        readonly depositor: AccountId32;
        readonly approvals: Vec<AccountId32>;
    }

    /** @name PalletMultisigError (902) */
    interface PalletMultisigError extends Enum {
        readonly isMinimumThreshold: boolean;
        readonly isAlreadyApproved: boolean;
        readonly isNoApprovalsNeeded: boolean;
        readonly isTooFewSignatories: boolean;
        readonly isTooManySignatories: boolean;
        readonly isSignatoriesOutOfOrder: boolean;
        readonly isSenderInSignatories: boolean;
        readonly isNotFound: boolean;
        readonly isNotOwner: boolean;
        readonly isNoTimepoint: boolean;
        readonly isWrongTimepoint: boolean;
        readonly isUnexpectedTimepoint: boolean;
        readonly isMaxWeightTooLow: boolean;
        readonly isAlreadyStored: boolean;
        readonly type:
            | "MinimumThreshold"
            | "AlreadyApproved"
            | "NoApprovalsNeeded"
            | "TooFewSignatories"
            | "TooManySignatories"
            | "SignatoriesOutOfOrder"
            | "SenderInSignatories"
            | "NotFound"
            | "NotOwner"
            | "NoTimepoint"
            | "WrongTimepoint"
            | "UnexpectedTimepoint"
            | "MaxWeightTooLow"
            | "AlreadyStored";
    }

    /** @name PalletPreimageOldRequestStatus (903) */
    interface PalletPreimageOldRequestStatus extends Enum {
        readonly isUnrequested: boolean;
        readonly asUnrequested: {
            readonly deposit: ITuple<[AccountId32, u128]>;
            readonly len: u32;
        } & Struct;
        readonly isRequested: boolean;
        readonly asRequested: {
            readonly deposit: Option<ITuple<[AccountId32, u128]>>;
            readonly count: u32;
            readonly len: Option<u32>;
        } & Struct;
        readonly type: "Unrequested" | "Requested";
    }

    /** @name PalletPreimageRequestStatus (906) */
    interface PalletPreimageRequestStatus extends Enum {
        readonly isUnrequested: boolean;
        readonly asUnrequested: {
            readonly ticket: ITuple<[AccountId32, u128]>;
            readonly len: u32;
        } & Struct;
        readonly isRequested: boolean;
        readonly asRequested: {
            readonly maybeTicket: Option<ITuple<[AccountId32, u128]>>;
            readonly count: u32;
            readonly maybeLen: Option<u32>;
        } & Struct;
        readonly type: "Unrequested" | "Requested";
    }

    /** @name PalletPreimageError (911) */
    interface PalletPreimageError extends Enum {
        readonly isTooBig: boolean;
        readonly isAlreadyNoted: boolean;
        readonly isNotAuthorized: boolean;
        readonly isNotNoted: boolean;
        readonly isRequested: boolean;
        readonly isNotRequested: boolean;
        readonly isTooMany: boolean;
        readonly isTooFew: boolean;
        readonly type:
            | "TooBig"
            | "AlreadyNoted"
            | "NotAuthorized"
            | "NotNoted"
            | "Requested"
            | "NotRequested"
            | "TooMany"
            | "TooFew";
    }

    /** @name PalletAssetRateError (912) */
    interface PalletAssetRateError extends Enum {
        readonly isUnknownAssetKind: boolean;
        readonly isAlreadyExists: boolean;
        readonly isOverflow: boolean;
        readonly type: "UnknownAssetKind" | "AlreadyExists" | "Overflow";
    }

    /** @name PalletAssetsAssetDetails (913) */
    interface PalletAssetsAssetDetails extends Struct {
        readonly owner: AccountId32;
        readonly issuer: AccountId32;
        readonly admin: AccountId32;
        readonly freezer: AccountId32;
        readonly supply: u128;
        readonly deposit: u128;
        readonly minBalance: u128;
        readonly isSufficient: bool;
        readonly accounts: u32;
        readonly sufficients: u32;
        readonly approvals: u32;
        readonly status: PalletAssetsAssetStatus;
    }

    /** @name PalletAssetsAssetStatus (914) */
    interface PalletAssetsAssetStatus extends Enum {
        readonly isLive: boolean;
        readonly isFrozen: boolean;
        readonly isDestroying: boolean;
        readonly type: "Live" | "Frozen" | "Destroying";
    }

    /** @name PalletAssetsAssetAccount (915) */
    interface PalletAssetsAssetAccount extends Struct {
        readonly balance: u128;
        readonly status: PalletAssetsAccountStatus;
        readonly reason: PalletAssetsExistenceReason;
        readonly extra: Null;
    }

    /** @name PalletAssetsAccountStatus (916) */
    interface PalletAssetsAccountStatus extends Enum {
        readonly isLiquid: boolean;
        readonly isFrozen: boolean;
        readonly isBlocked: boolean;
        readonly type: "Liquid" | "Frozen" | "Blocked";
    }

    /** @name PalletAssetsExistenceReason (917) */
    interface PalletAssetsExistenceReason extends Enum {
        readonly isConsumer: boolean;
        readonly isSufficient: boolean;
        readonly isDepositHeld: boolean;
        readonly asDepositHeld: u128;
        readonly isDepositRefunded: boolean;
        readonly isDepositFrom: boolean;
        readonly asDepositFrom: ITuple<[AccountId32, u128]>;
        readonly type: "Consumer" | "Sufficient" | "DepositHeld" | "DepositRefunded" | "DepositFrom";
    }

    /** @name PalletAssetsApproval (919) */
    interface PalletAssetsApproval extends Struct {
        readonly amount: u128;
        readonly deposit: u128;
    }

    /** @name PalletAssetsAssetMetadata (920) */
    interface PalletAssetsAssetMetadata extends Struct {
        readonly deposit: u128;
        readonly name: Bytes;
        readonly symbol: Bytes;
        readonly decimals: u8;
        readonly isFrozen: bool;
    }

    /** @name PalletAssetsError (922) */
    interface PalletAssetsError extends Enum {
        readonly isBalanceLow: boolean;
        readonly isNoAccount: boolean;
        readonly isNoPermission: boolean;
        readonly isUnknown: boolean;
        readonly isFrozen: boolean;
        readonly isInUse: boolean;
        readonly isBadWitness: boolean;
        readonly isMinBalanceZero: boolean;
        readonly isUnavailableConsumer: boolean;
        readonly isBadMetadata: boolean;
        readonly isUnapproved: boolean;
        readonly isWouldDie: boolean;
        readonly isAlreadyExists: boolean;
        readonly isNoDeposit: boolean;
        readonly isWouldBurn: boolean;
        readonly isLiveAsset: boolean;
        readonly isAssetNotLive: boolean;
        readonly isIncorrectStatus: boolean;
        readonly isNotFrozen: boolean;
        readonly isCallbackFailed: boolean;
        readonly isBadAssetId: boolean;
        readonly isContainsFreezes: boolean;
        readonly isContainsHolds: boolean;
        readonly type:
            | "BalanceLow"
            | "NoAccount"
            | "NoPermission"
            | "Unknown"
            | "Frozen"
            | "InUse"
            | "BadWitness"
            | "MinBalanceZero"
            | "UnavailableConsumer"
            | "BadMetadata"
            | "Unapproved"
            | "WouldDie"
            | "AlreadyExists"
            | "NoDeposit"
            | "WouldBurn"
            | "LiveAsset"
            | "AssetNotLive"
            | "IncorrectStatus"
            | "NotFrozen"
            | "CallbackFailed"
            | "BadAssetId"
            | "ContainsFreezes"
            | "ContainsHolds";
    }

    /** @name PalletForeignAssetCreatorError (923) */
    interface PalletForeignAssetCreatorError extends Enum {
        readonly isAssetAlreadyExists: boolean;
        readonly isAssetDoesNotExist: boolean;
        readonly type: "AssetAlreadyExists" | "AssetDoesNotExist";
    }

    /** @name PalletXcmQueryStatus (924) */
    interface PalletXcmQueryStatus extends Enum {
        readonly isPending: boolean;
        readonly asPending: {
            readonly responder: XcmVersionedLocation;
            readonly maybeMatchQuerier: Option<XcmVersionedLocation>;
            readonly maybeNotify: Option<ITuple<[u8, u8]>>;
            readonly timeout: u32;
        } & Struct;
        readonly isVersionNotifier: boolean;
        readonly asVersionNotifier: {
            readonly origin: XcmVersionedLocation;
            readonly isActive: bool;
        } & Struct;
        readonly isReady: boolean;
        readonly asReady: {
            readonly response: XcmVersionedResponse;
            readonly at: u32;
        } & Struct;
        readonly type: "Pending" | "VersionNotifier" | "Ready";
    }

    /** @name XcmVersionedResponse (928) */
    interface XcmVersionedResponse extends Enum {
        readonly isV3: boolean;
        readonly asV3: XcmV3Response;
        readonly isV4: boolean;
        readonly asV4: StagingXcmV4Response;
        readonly isV5: boolean;
        readonly asV5: StagingXcmV5Response;
        readonly type: "V3" | "V4" | "V5";
    }

    /** @name PalletXcmVersionMigrationStage (934) */
    interface PalletXcmVersionMigrationStage extends Enum {
        readonly isMigrateSupportedVersion: boolean;
        readonly isMigrateVersionNotifiers: boolean;
        readonly isNotifyCurrentTargets: boolean;
        readonly asNotifyCurrentTargets: Option<Bytes>;
        readonly isMigrateAndNotifyOldTargets: boolean;
        readonly type:
            | "MigrateSupportedVersion"
            | "MigrateVersionNotifiers"
            | "NotifyCurrentTargets"
            | "MigrateAndNotifyOldTargets";
    }

    /** @name PalletXcmRemoteLockedFungibleRecord (936) */
    interface PalletXcmRemoteLockedFungibleRecord extends Struct {
        readonly amount: u128;
        readonly owner: XcmVersionedLocation;
        readonly locker: XcmVersionedLocation;
        readonly consumers: Vec<ITuple<[Null, u128]>>;
    }

    /** @name PalletXcmAuthorizedAliasesEntry (943) */
    interface PalletXcmAuthorizedAliasesEntry extends Struct {
        readonly aliasers: Vec<XcmRuntimeApisAuthorizedAliasesOriginAliaser>;
        readonly ticket: FrameSupportStorageDisabled;
    }

    /** @name FrameSupportStorageDisabled (944) */
    type FrameSupportStorageDisabled = Null;

    /** @name PalletXcmMaxAuthorizedAliases (945) */
    type PalletXcmMaxAuthorizedAliases = Null;

    /** @name XcmRuntimeApisAuthorizedAliasesOriginAliaser (947) */
    interface XcmRuntimeApisAuthorizedAliasesOriginAliaser extends Struct {
        readonly location: XcmVersionedLocation;
        readonly expiry: Option<u64>;
    }

    /** @name PalletXcmError (949) */
    interface PalletXcmError extends Enum {
        readonly isUnreachable: boolean;
        readonly isSendFailure: boolean;
        readonly isFiltered: boolean;
        readonly isUnweighableMessage: boolean;
        readonly isDestinationNotInvertible: boolean;
        readonly isEmpty: boolean;
        readonly isCannotReanchor: boolean;
        readonly isTooManyAssets: boolean;
        readonly isInvalidOrigin: boolean;
        readonly isBadVersion: boolean;
        readonly isBadLocation: boolean;
        readonly isNoSubscription: boolean;
        readonly isAlreadySubscribed: boolean;
        readonly isCannotCheckOutTeleport: boolean;
        readonly isLowBalance: boolean;
        readonly isTooManyLocks: boolean;
        readonly isAccountNotSovereign: boolean;
        readonly isFeesNotMet: boolean;
        readonly isLockNotFound: boolean;
        readonly isInUse: boolean;
        readonly isInvalidAssetUnknownReserve: boolean;
        readonly isInvalidAssetUnsupportedReserve: boolean;
        readonly isTooManyReserves: boolean;
        readonly isLocalExecutionIncomplete: boolean;
        readonly isTooManyAuthorizedAliases: boolean;
        readonly isExpiresInPast: boolean;
        readonly isAliasNotFound: boolean;
        readonly isLocalExecutionIncompleteWithError: boolean;
        readonly asLocalExecutionIncompleteWithError: {
            readonly index: u8;
            readonly error: PalletXcmErrorsExecutionError;
        } & Struct;
        readonly type:
            | "Unreachable"
            | "SendFailure"
            | "Filtered"
            | "UnweighableMessage"
            | "DestinationNotInvertible"
            | "Empty"
            | "CannotReanchor"
            | "TooManyAssets"
            | "InvalidOrigin"
            | "BadVersion"
            | "BadLocation"
            | "NoSubscription"
            | "AlreadySubscribed"
            | "CannotCheckOutTeleport"
            | "LowBalance"
            | "TooManyLocks"
            | "AccountNotSovereign"
            | "FeesNotMet"
            | "LockNotFound"
            | "InUse"
            | "InvalidAssetUnknownReserve"
            | "InvalidAssetUnsupportedReserve"
            | "TooManyReserves"
            | "LocalExecutionIncomplete"
            | "TooManyAuthorizedAliases"
            | "ExpiresInPast"
            | "AliasNotFound"
            | "LocalExecutionIncompleteWithError";
    }

    /** @name PalletXcmErrorsExecutionError (950) */
    interface PalletXcmErrorsExecutionError extends Enum {
        readonly isOverflow: boolean;
        readonly isUnimplemented: boolean;
        readonly isUntrustedReserveLocation: boolean;
        readonly isUntrustedTeleportLocation: boolean;
        readonly isLocationFull: boolean;
        readonly isLocationNotInvertible: boolean;
        readonly isBadOrigin: boolean;
        readonly isInvalidLocation: boolean;
        readonly isAssetNotFound: boolean;
        readonly isFailedToTransactAsset: boolean;
        readonly isNotWithdrawable: boolean;
        readonly isLocationCannotHold: boolean;
        readonly isExceedsMaxMessageSize: boolean;
        readonly isDestinationUnsupported: boolean;
        readonly isTransport: boolean;
        readonly isUnroutable: boolean;
        readonly isUnknownClaim: boolean;
        readonly isFailedToDecode: boolean;
        readonly isMaxWeightInvalid: boolean;
        readonly isNotHoldingFees: boolean;
        readonly isTooExpensive: boolean;
        readonly isTrap: boolean;
        readonly isExpectationFalse: boolean;
        readonly isPalletNotFound: boolean;
        readonly isNameMismatch: boolean;
        readonly isVersionIncompatible: boolean;
        readonly isHoldingWouldOverflow: boolean;
        readonly isExportError: boolean;
        readonly isReanchorFailed: boolean;
        readonly isNoDeal: boolean;
        readonly isFeesNotMet: boolean;
        readonly isLockError: boolean;
        readonly isNoPermission: boolean;
        readonly isUnanchored: boolean;
        readonly isNotDepositable: boolean;
        readonly isTooManyAssets: boolean;
        readonly isUnhandledXcmVersion: boolean;
        readonly isWeightLimitReached: boolean;
        readonly isBarrier: boolean;
        readonly isWeightNotComputable: boolean;
        readonly isExceedsStackLimit: boolean;
        readonly type:
            | "Overflow"
            | "Unimplemented"
            | "UntrustedReserveLocation"
            | "UntrustedTeleportLocation"
            | "LocationFull"
            | "LocationNotInvertible"
            | "BadOrigin"
            | "InvalidLocation"
            | "AssetNotFound"
            | "FailedToTransactAsset"
            | "NotWithdrawable"
            | "LocationCannotHold"
            | "ExceedsMaxMessageSize"
            | "DestinationUnsupported"
            | "Transport"
            | "Unroutable"
            | "UnknownClaim"
            | "FailedToDecode"
            | "MaxWeightInvalid"
            | "NotHoldingFees"
            | "TooExpensive"
            | "Trap"
            | "ExpectationFalse"
            | "PalletNotFound"
            | "NameMismatch"
            | "VersionIncompatible"
            | "HoldingWouldOverflow"
            | "ExportError"
            | "ReanchorFailed"
            | "NoDeal"
            | "FeesNotMet"
            | "LockError"
            | "NoPermission"
            | "Unanchored"
            | "NotDepositable"
            | "TooManyAssets"
            | "UnhandledXcmVersion"
            | "WeightLimitReached"
            | "Barrier"
            | "WeightNotComputable"
            | "ExceedsStackLimit";
    }

    /** @name PalletStreamPaymentStream (951) */
    interface PalletStreamPaymentStream extends Struct {
        readonly source: AccountId32;
        readonly target: AccountId32;
        readonly config: PalletStreamPaymentStreamConfig;
        readonly deposit: u128;
        readonly lastTimeUpdated: u128;
        readonly requestNonce: u32;
        readonly pendingRequest: Option<PalletStreamPaymentChangeRequest>;
        readonly openingDeposit: u128;
    }

    /** @name PalletStreamPaymentChangeRequest (953) */
    interface PalletStreamPaymentChangeRequest extends Struct {
        readonly requester: PalletStreamPaymentParty;
        readonly kind: PalletStreamPaymentChangeKind;
        readonly newConfig: PalletStreamPaymentStreamConfig;
        readonly depositChange: Option<PalletStreamPaymentDepositChange>;
    }

    /** @name PalletStreamPaymentError (955) */
    interface PalletStreamPaymentError extends Enum {
        readonly isUnknownStreamId: boolean;
        readonly isStreamIdOverflow: boolean;
        readonly isUnauthorizedOrigin: boolean;
        readonly isCantBeBothSourceAndTarget: boolean;
        readonly isCantFetchCurrentTime: boolean;
        readonly isSourceCantDecreaseRate: boolean;
        readonly isTargetCantIncreaseRate: boolean;
        readonly isCantOverrideMandatoryChange: boolean;
        readonly isNoPendingRequest: boolean;
        readonly isCantAcceptOwnRequest: boolean;
        readonly isCanOnlyCancelOwnRequest: boolean;
        readonly isWrongRequestNonce: boolean;
        readonly isChangingAssetRequiresAbsoluteDepositChange: boolean;
        readonly isTargetCantChangeDeposit: boolean;
        readonly isImmediateDepositChangeRequiresSameAssetId: boolean;
        readonly isDeadlineCantBeInPast: boolean;
        readonly isCantFetchStatusBeforeLastTimeUpdated: boolean;
        readonly isDeadlineDelayIsBelowMinium: boolean;
        readonly isCantDecreaseDepositUnderSoftDepositMinimum: boolean;
        readonly isSourceCantCloseActiveStreamWithSoftDepositMinimum: boolean;
        readonly isCantCreateStreamWithDepositUnderSoftMinimum: boolean;
        readonly type:
            | "UnknownStreamId"
            | "StreamIdOverflow"
            | "UnauthorizedOrigin"
            | "CantBeBothSourceAndTarget"
            | "CantFetchCurrentTime"
            | "SourceCantDecreaseRate"
            | "TargetCantIncreaseRate"
            | "CantOverrideMandatoryChange"
            | "NoPendingRequest"
            | "CantAcceptOwnRequest"
            | "CanOnlyCancelOwnRequest"
            | "WrongRequestNonce"
            | "ChangingAssetRequiresAbsoluteDepositChange"
            | "TargetCantChangeDeposit"
            | "ImmediateDepositChangeRequiresSameAssetId"
            | "DeadlineCantBeInPast"
            | "CantFetchStatusBeforeLastTimeUpdated"
            | "DeadlineDelayIsBelowMinium"
            | "CantDecreaseDepositUnderSoftDepositMinimum"
            | "SourceCantCloseActiveStreamWithSoftDepositMinimum"
            | "CantCreateStreamWithDepositUnderSoftMinimum";
    }

    /** @name PalletMigrationsError (956) */
    interface PalletMigrationsError extends Enum {
        readonly isPreimageMissing: boolean;
        readonly isWrongUpperBound: boolean;
        readonly isPreimageIsTooBig: boolean;
        readonly isPreimageAlreadyExists: boolean;
        readonly type: "PreimageMissing" | "WrongUpperBound" | "PreimageIsTooBig" | "PreimageAlreadyExists";
    }

    /** @name PalletMaintenanceModeError (958) */
    interface PalletMaintenanceModeError extends Enum {
        readonly isAlreadyInMaintenanceMode: boolean;
        readonly isNotInMaintenanceMode: boolean;
        readonly type: "AlreadyInMaintenanceMode" | "NotInMaintenanceMode";
    }

    /** @name PalletBeefyError (961) */
    interface PalletBeefyError extends Enum {
        readonly isInvalidKeyOwnershipProof: boolean;
        readonly isInvalidDoubleVotingProof: boolean;
        readonly isInvalidForkVotingProof: boolean;
        readonly isInvalidFutureBlockVotingProof: boolean;
        readonly isInvalidEquivocationProofSession: boolean;
        readonly isDuplicateOffenceReport: boolean;
        readonly isInvalidConfiguration: boolean;
        readonly type:
            | "InvalidKeyOwnershipProof"
            | "InvalidDoubleVotingProof"
            | "InvalidForkVotingProof"
            | "InvalidFutureBlockVotingProof"
            | "InvalidEquivocationProofSession"
            | "DuplicateOffenceReport"
            | "InvalidConfiguration";
    }

    /** @name SpConsensusBeefyMmrBeefyAuthoritySet (962) */
    interface SpConsensusBeefyMmrBeefyAuthoritySet extends Struct {
        readonly id: u64;
        readonly len: u32;
        readonly keysetCommitment: H256;
    }

    /** @name SnowbridgeBeaconPrimitivesCompactBeaconState (963) */
    interface SnowbridgeBeaconPrimitivesCompactBeaconState extends Struct {
        readonly slot: Compact<u64>;
        readonly blockRootsRoot: H256;
    }

    /** @name SnowbridgeBeaconPrimitivesSyncCommitteePrepared (964) */
    interface SnowbridgeBeaconPrimitivesSyncCommitteePrepared extends Struct {
        readonly root: H256;
        readonly pubkeys: Vec<SnowbridgeMilagroBlsKeysPublicKey>;
        readonly aggregatePubkey: SnowbridgeMilagroBlsKeysPublicKey;
    }

    /** @name SnowbridgeMilagroBlsKeysPublicKey (966) */
    interface SnowbridgeMilagroBlsKeysPublicKey extends Struct {
        readonly point: SnowbridgeAmclBls381Ecp;
    }

    /** @name SnowbridgeAmclBls381Ecp (967) */
    interface SnowbridgeAmclBls381Ecp extends Struct {
        readonly x: SnowbridgeAmclBls381Fp;
        readonly y: SnowbridgeAmclBls381Fp;
        readonly z: SnowbridgeAmclBls381Fp;
    }

    /** @name SnowbridgeAmclBls381Fp (968) */
    interface SnowbridgeAmclBls381Fp extends Struct {
        readonly x: SnowbridgeAmclBls381Big;
        readonly xes: i32;
    }

    /** @name SnowbridgeAmclBls381Big (969) */
    interface SnowbridgeAmclBls381Big extends Struct {
        readonly w: Vec<i32>;
    }

    /** @name SnowbridgeBeaconPrimitivesForkVersions (972) */
    interface SnowbridgeBeaconPrimitivesForkVersions extends Struct {
        readonly genesis: SnowbridgeBeaconPrimitivesFork;
        readonly altair: SnowbridgeBeaconPrimitivesFork;
        readonly bellatrix: SnowbridgeBeaconPrimitivesFork;
        readonly capella: SnowbridgeBeaconPrimitivesFork;
        readonly deneb: SnowbridgeBeaconPrimitivesFork;
        readonly electra: SnowbridgeBeaconPrimitivesFork;
    }

    /** @name SnowbridgeBeaconPrimitivesFork (973) */
    interface SnowbridgeBeaconPrimitivesFork extends Struct {
        readonly version: U8aFixed;
        readonly epoch: u64;
    }

    /** @name SnowbridgePalletEthereumClientError (974) */
    interface SnowbridgePalletEthereumClientError extends Enum {
        readonly isSkippedSyncCommitteePeriod: boolean;
        readonly isSyncCommitteeUpdateRequired: boolean;
        readonly isIrrelevantUpdate: boolean;
        readonly isNotBootstrapped: boolean;
        readonly isSyncCommitteeParticipantsNotSupermajority: boolean;
        readonly isInvalidHeaderMerkleProof: boolean;
        readonly isInvalidSyncCommitteeMerkleProof: boolean;
        readonly isInvalidExecutionHeaderProof: boolean;
        readonly isInvalidAncestryMerkleProof: boolean;
        readonly isInvalidBlockRootsRootMerkleProof: boolean;
        readonly isInvalidFinalizedHeaderGap: boolean;
        readonly isHeaderNotFinalized: boolean;
        readonly isBlockBodyHashTreeRootFailed: boolean;
        readonly isHeaderHashTreeRootFailed: boolean;
        readonly isSyncCommitteeHashTreeRootFailed: boolean;
        readonly isSigningRootHashTreeRootFailed: boolean;
        readonly isForkDataHashTreeRootFailed: boolean;
        readonly isExpectedFinalizedHeaderNotStored: boolean;
        readonly isBlsPreparePublicKeysFailed: boolean;
        readonly isBlsVerificationFailed: boolean;
        readonly asBlsVerificationFailed: SnowbridgeBeaconPrimitivesBlsBlsError;
        readonly isInvalidUpdateSlot: boolean;
        readonly isInvalidSyncCommitteeUpdate: boolean;
        readonly isExecutionHeaderTooFarBehind: boolean;
        readonly isExecutionHeaderSkippedBlock: boolean;
        readonly isHalted: boolean;
        readonly type:
            | "SkippedSyncCommitteePeriod"
            | "SyncCommitteeUpdateRequired"
            | "IrrelevantUpdate"
            | "NotBootstrapped"
            | "SyncCommitteeParticipantsNotSupermajority"
            | "InvalidHeaderMerkleProof"
            | "InvalidSyncCommitteeMerkleProof"
            | "InvalidExecutionHeaderProof"
            | "InvalidAncestryMerkleProof"
            | "InvalidBlockRootsRootMerkleProof"
            | "InvalidFinalizedHeaderGap"
            | "HeaderNotFinalized"
            | "BlockBodyHashTreeRootFailed"
            | "HeaderHashTreeRootFailed"
            | "SyncCommitteeHashTreeRootFailed"
            | "SigningRootHashTreeRootFailed"
            | "ForkDataHashTreeRootFailed"
            | "ExpectedFinalizedHeaderNotStored"
            | "BlsPreparePublicKeysFailed"
            | "BlsVerificationFailed"
            | "InvalidUpdateSlot"
            | "InvalidSyncCommitteeUpdate"
            | "ExecutionHeaderTooFarBehind"
            | "ExecutionHeaderSkippedBlock"
            | "Halted";
    }

    /** @name SnowbridgeBeaconPrimitivesBlsBlsError (975) */
    interface SnowbridgeBeaconPrimitivesBlsBlsError extends Enum {
        readonly isInvalidSignature: boolean;
        readonly isInvalidPublicKey: boolean;
        readonly isInvalidAggregatePublicKeys: boolean;
        readonly isSignatureVerificationFailed: boolean;
        readonly type:
            | "InvalidSignature"
            | "InvalidPublicKey"
            | "InvalidAggregatePublicKeys"
            | "SignatureVerificationFailed";
    }

    /** @name PolkadotRuntimeCommonParasSudoWrapperPalletError (976) */
    interface PolkadotRuntimeCommonParasSudoWrapperPalletError extends Enum {
        readonly isParaDoesntExist: boolean;
        readonly isParaAlreadyExists: boolean;
        readonly isExceedsMaxMessageSize: boolean;
        readonly isUnroutable: boolean;
        readonly isCouldntCleanup: boolean;
        readonly isNotParathread: boolean;
        readonly isNotParachain: boolean;
        readonly isCannotUpgrade: boolean;
        readonly isCannotDowngrade: boolean;
        readonly isTooManyCores: boolean;
        readonly type:
            | "ParaDoesntExist"
            | "ParaAlreadyExists"
            | "ExceedsMaxMessageSize"
            | "Unroutable"
            | "CouldntCleanup"
            | "NotParathread"
            | "NotParachain"
            | "CannotUpgrade"
            | "CannotDowngrade"
            | "TooManyCores";
    }

    /** @name PalletSudoError (977) */
    interface PalletSudoError extends Enum {
        readonly isRequireSudo: boolean;
        readonly type: "RequireSudo";
    }

    /** @name FrameSystemExtensionsCheckNonZeroSender (980) */
    type FrameSystemExtensionsCheckNonZeroSender = Null;

    /** @name FrameSystemExtensionsCheckSpecVersion (981) */
    type FrameSystemExtensionsCheckSpecVersion = Null;

    /** @name FrameSystemExtensionsCheckTxVersion (982) */
    type FrameSystemExtensionsCheckTxVersion = Null;

    /** @name FrameSystemExtensionsCheckGenesis (983) */
    type FrameSystemExtensionsCheckGenesis = Null;

    /** @name FrameSystemExtensionsCheckNonce (986) */
    interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

    /** @name FrameSystemExtensionsCheckWeight (987) */
    type FrameSystemExtensionsCheckWeight = Null;

    /** @name PalletTransactionPaymentChargeTransactionPayment (988) */
    interface PalletTransactionPaymentChargeTransactionPayment extends Compact<u128> {}

    /** @name FrameMetadataHashExtensionCheckMetadataHash (989) */
    interface FrameMetadataHashExtensionCheckMetadataHash extends Struct {
        readonly mode: FrameMetadataHashExtensionMode;
    }

    /** @name FrameMetadataHashExtensionMode (990) */
    interface FrameMetadataHashExtensionMode extends Enum {
        readonly isDisabled: boolean;
        readonly isEnabled: boolean;
        readonly type: "Disabled" | "Enabled";
    }

    /** @name FrameSystemExtensionsWeightReclaim (991) */
    type FrameSystemExtensionsWeightReclaim = Null;

    /** @name DancelightRuntimeRuntime (992) */
    type DancelightRuntimeRuntime = Null;
} // declare module
