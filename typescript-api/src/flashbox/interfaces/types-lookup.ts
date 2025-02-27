// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/types/lookup";

import type { Data } from "@polkadot/types";
import type {
    BTreeMap,
    BTreeSet,
    Bytes,
    Compact,
    Enum,
    Null,
    Option,
    Result,
    Struct,
    Text,
    U8aFixed,
    Vec,
    bool,
    u128,
    u16,
    u32,
    u64,
    u8,
} from "@polkadot/types-codec";
import type { ITuple } from "@polkadot/types-codec/types";
import type { AccountId32, Call, H256, MultiAddress, Perbill } from "@polkadot/types/interfaces/runtime";
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
        readonly type:
            | "ExtrinsicSuccess"
            | "ExtrinsicFailed"
            | "CodeUpdated"
            | "NewAccount"
            | "KilledAccount"
            | "Remarked"
            | "UpgradeAuthorized";
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

    /** @name CumulusPalletParachainSystemEvent (32) */
    interface CumulusPalletParachainSystemEvent extends Enum {
        readonly isValidationFunctionStored: boolean;
        readonly isValidationFunctionApplied: boolean;
        readonly asValidationFunctionApplied: {
            readonly relayChainBlockNum: u32;
        } & Struct;
        readonly isValidationFunctionDiscarded: boolean;
        readonly isDownwardMessagesReceived: boolean;
        readonly asDownwardMessagesReceived: {
            readonly count: u32;
        } & Struct;
        readonly isDownwardMessagesProcessed: boolean;
        readonly asDownwardMessagesProcessed: {
            readonly weightUsed: SpWeightsWeightV2Weight;
            readonly dmqHead: H256;
        } & Struct;
        readonly isUpwardMessageSent: boolean;
        readonly asUpwardMessageSent: {
            readonly messageHash: Option<U8aFixed>;
        } & Struct;
        readonly type:
            | "ValidationFunctionStored"
            | "ValidationFunctionApplied"
            | "ValidationFunctionDiscarded"
            | "DownwardMessagesReceived"
            | "DownwardMessagesProcessed"
            | "UpwardMessageSent";
    }

    /** @name PalletSudoEvent (34) */
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

    /** @name PalletUtilityEvent (38) */
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
        readonly type:
            | "BatchInterrupted"
            | "BatchCompleted"
            | "BatchCompletedWithErrors"
            | "ItemCompleted"
            | "ItemFailed"
            | "DispatchedAs";
    }

    /** @name PalletProxyEvent (39) */
    interface PalletProxyEvent extends Enum {
        readonly isProxyExecuted: boolean;
        readonly asProxyExecuted: {
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isPureCreated: boolean;
        readonly asPureCreated: {
            readonly pure: AccountId32;
            readonly who: AccountId32;
            readonly proxyType: FlashboxRuntimeProxyType;
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
            readonly proxyType: FlashboxRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly isProxyRemoved: boolean;
        readonly asProxyRemoved: {
            readonly delegator: AccountId32;
            readonly delegatee: AccountId32;
            readonly proxyType: FlashboxRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly type: "ProxyExecuted" | "PureCreated" | "Announced" | "ProxyAdded" | "ProxyRemoved";
    }

    /** @name FlashboxRuntimeProxyType (40) */
    interface FlashboxRuntimeProxyType extends Enum {
        readonly isAny: boolean;
        readonly isNonTransfer: boolean;
        readonly isGovernance: boolean;
        readonly isStaking: boolean;
        readonly isCancelProxy: boolean;
        readonly isBalances: boolean;
        readonly isRegistrar: boolean;
        readonly isSudoRegistrar: boolean;
        readonly type:
            | "Any"
            | "NonTransfer"
            | "Governance"
            | "Staking"
            | "CancelProxy"
            | "Balances"
            | "Registrar"
            | "SudoRegistrar";
    }

    /** @name PalletMigrationsEvent (42) */
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

    /** @name PalletMaintenanceModeEvent (45) */
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

    /** @name PalletTxPauseEvent (46) */
    interface PalletTxPauseEvent extends Enum {
        readonly isCallPaused: boolean;
        readonly asCallPaused: {
            readonly fullName: ITuple<[Bytes, Bytes]>;
        } & Struct;
        readonly isCallUnpaused: boolean;
        readonly asCallUnpaused: {
            readonly fullName: ITuple<[Bytes, Bytes]>;
        } & Struct;
        readonly type: "CallPaused" | "CallUnpaused";
    }

    /** @name PalletBalancesEvent (49) */
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

    /** @name FrameSupportTokensMiscBalanceStatus (50) */
    interface FrameSupportTokensMiscBalanceStatus extends Enum {
        readonly isFree: boolean;
        readonly isReserved: boolean;
        readonly type: "Free" | "Reserved";
    }

    /** @name PalletTransactionPaymentEvent (51) */
    interface PalletTransactionPaymentEvent extends Enum {
        readonly isTransactionFeePaid: boolean;
        readonly asTransactionFeePaid: {
            readonly who: AccountId32;
            readonly actualFee: u128;
            readonly tip: u128;
        } & Struct;
        readonly type: "TransactionFeePaid";
    }

    /** @name PalletStreamPaymentEvent (52) */
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

    /** @name PalletStreamPaymentParty (53) */
    interface PalletStreamPaymentParty extends Enum {
        readonly isSource: boolean;
        readonly isTarget: boolean;
        readonly type: "Source" | "Target";
    }

    /** @name PalletStreamPaymentStreamConfig (54) */
    interface PalletStreamPaymentStreamConfig extends Struct {
        readonly timeUnit: FlashboxRuntimeTimeUnit;
        readonly assetId: FlashboxRuntimeStreamPaymentAssetId;
        readonly rate: u128;
    }

    /** @name FlashboxRuntimeTimeUnit (55) */
    interface FlashboxRuntimeTimeUnit extends Enum {
        readonly isBlockNumber: boolean;
        readonly isTimestamp: boolean;
        readonly type: "BlockNumber" | "Timestamp";
    }

    /** @name FlashboxRuntimeStreamPaymentAssetId (56) */
    interface FlashboxRuntimeStreamPaymentAssetId extends Enum {
        readonly isNative: boolean;
        readonly type: "Native";
    }

    /** @name PalletStreamPaymentDepositChange (58) */
    interface PalletStreamPaymentDepositChange extends Enum {
        readonly isIncrease: boolean;
        readonly asIncrease: u128;
        readonly isDecrease: boolean;
        readonly asDecrease: u128;
        readonly isAbsolute: boolean;
        readonly asAbsolute: u128;
        readonly type: "Increase" | "Decrease" | "Absolute";
    }

    /** @name PalletIdentityEvent (59) */
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

    /** @name PalletMultisigEvent (61) */
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
        readonly type: "NewMultisig" | "MultisigApproval" | "MultisigExecuted" | "MultisigCancelled";
    }

    /** @name PalletMultisigTimepoint (62) */
    interface PalletMultisigTimepoint extends Struct {
        readonly height: u32;
        readonly index: u32;
    }

    /** @name PalletRegistrarEvent (63) */
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

    /** @name PalletCollatorAssignmentEvent (65) */
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

    /** @name TpTraitsFullRotationModes (66) */
    interface TpTraitsFullRotationModes extends Struct {
        readonly orchestrator: TpTraitsFullRotationMode;
        readonly parachain: TpTraitsFullRotationMode;
        readonly parathread: TpTraitsFullRotationMode;
    }

    /** @name TpTraitsFullRotationMode (67) */
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

    /** @name PalletAuthorNotingEvent (69) */
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

    /** @name PalletServicesPaymentEvent (71) */
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
            readonly maxCorePrice: Option<u128>;
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

    /** @name PalletDataPreserversEvent (73) */
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

    /** @name PalletInvulnerablesEvent (74) */
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

    /** @name PalletSessionEvent (75) */
    interface PalletSessionEvent extends Enum {
        readonly isNewSession: boolean;
        readonly asNewSession: {
            readonly sessionIndex: u32;
        } & Struct;
        readonly type: "NewSession";
    }

    /** @name PalletInflationRewardsEvent (76) */
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

    /** @name PalletTreasuryEvent (77) */
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

    /** @name PalletRootTestingEvent (78) */
    interface PalletRootTestingEvent extends Enum {
        readonly isDefensiveTestCall: boolean;
        readonly type: "DefensiveTestCall";
    }

    /** @name FrameSystemPhase (79) */
    interface FrameSystemPhase extends Enum {
        readonly isApplyExtrinsic: boolean;
        readonly asApplyExtrinsic: u32;
        readonly isFinalization: boolean;
        readonly isInitialization: boolean;
        readonly type: "ApplyExtrinsic" | "Finalization" | "Initialization";
    }

    /** @name FrameSystemLastRuntimeUpgradeInfo (83) */
    interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
        readonly specVersion: Compact<u32>;
        readonly specName: Text;
    }

    /** @name FrameSystemCodeUpgradeAuthorization (87) */
    interface FrameSystemCodeUpgradeAuthorization extends Struct {
        readonly codeHash: H256;
        readonly checkVersion: bool;
    }

    /** @name FrameSystemCall (88) */
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

    /** @name FrameSystemLimitsBlockWeights (92) */
    interface FrameSystemLimitsBlockWeights extends Struct {
        readonly baseBlock: SpWeightsWeightV2Weight;
        readonly maxBlock: SpWeightsWeightV2Weight;
        readonly perClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
    }

    /** @name FrameSupportDispatchPerDispatchClassWeightsPerClass (93) */
    interface FrameSupportDispatchPerDispatchClassWeightsPerClass extends Struct {
        readonly normal: FrameSystemLimitsWeightsPerClass;
        readonly operational: FrameSystemLimitsWeightsPerClass;
        readonly mandatory: FrameSystemLimitsWeightsPerClass;
    }

    /** @name FrameSystemLimitsWeightsPerClass (94) */
    interface FrameSystemLimitsWeightsPerClass extends Struct {
        readonly baseExtrinsic: SpWeightsWeightV2Weight;
        readonly maxExtrinsic: Option<SpWeightsWeightV2Weight>;
        readonly maxTotal: Option<SpWeightsWeightV2Weight>;
        readonly reserved: Option<SpWeightsWeightV2Weight>;
    }

    /** @name FrameSystemLimitsBlockLength (96) */
    interface FrameSystemLimitsBlockLength extends Struct {
        readonly max: FrameSupportDispatchPerDispatchClassU32;
    }

    /** @name FrameSupportDispatchPerDispatchClassU32 (97) */
    interface FrameSupportDispatchPerDispatchClassU32 extends Struct {
        readonly normal: u32;
        readonly operational: u32;
        readonly mandatory: u32;
    }

    /** @name SpWeightsRuntimeDbWeight (98) */
    interface SpWeightsRuntimeDbWeight extends Struct {
        readonly read: u64;
        readonly write: u64;
    }

    /** @name SpVersionRuntimeVersion (99) */
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

    /** @name FrameSystemError (104) */
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

    /** @name CumulusPalletParachainSystemUnincludedSegmentAncestor (106) */
    interface CumulusPalletParachainSystemUnincludedSegmentAncestor extends Struct {
        readonly usedBandwidth: CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth;
        readonly paraHeadHash: Option<H256>;
        readonly consumedGoAheadSignal: Option<PolkadotPrimitivesV8UpgradeGoAhead>;
    }

    /** @name CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth (107) */
    interface CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth extends Struct {
        readonly umpMsgCount: u32;
        readonly umpTotalBytes: u32;
        readonly hrmpOutgoing: BTreeMap<u32, CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate>;
    }

    /** @name CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate (109) */
    interface CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate extends Struct {
        readonly msgCount: u32;
        readonly totalBytes: u32;
    }

    /** @name PolkadotPrimitivesV8UpgradeGoAhead (114) */
    interface PolkadotPrimitivesV8UpgradeGoAhead extends Enum {
        readonly isAbort: boolean;
        readonly isGoAhead: boolean;
        readonly type: "Abort" | "GoAhead";
    }

    /** @name CumulusPalletParachainSystemUnincludedSegmentSegmentTracker (115) */
    interface CumulusPalletParachainSystemUnincludedSegmentSegmentTracker extends Struct {
        readonly usedBandwidth: CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth;
        readonly hrmpWatermark: Option<u32>;
        readonly consumedGoAheadSignal: Option<PolkadotPrimitivesV8UpgradeGoAhead>;
    }

    /** @name PolkadotPrimitivesV8PersistedValidationData (117) */
    interface PolkadotPrimitivesV8PersistedValidationData extends Struct {
        readonly parentHead: Bytes;
        readonly relayParentNumber: u32;
        readonly relayParentStorageRoot: H256;
        readonly maxPovSize: u32;
    }

    /** @name PolkadotPrimitivesV8UpgradeRestriction (120) */
    interface PolkadotPrimitivesV8UpgradeRestriction extends Enum {
        readonly isPresent: boolean;
        readonly type: "Present";
    }

    /** @name SpTrieStorageProof (121) */
    interface SpTrieStorageProof extends Struct {
        readonly trieNodes: BTreeSet<Bytes>;
    }

    /** @name CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot (123) */
    interface CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot extends Struct {
        readonly dmqMqcHead: H256;
        readonly relayDispatchQueueRemainingCapacity: CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity;
        readonly ingressChannels: Vec<ITuple<[u32, PolkadotPrimitivesV8AbridgedHrmpChannel]>>;
        readonly egressChannels: Vec<ITuple<[u32, PolkadotPrimitivesV8AbridgedHrmpChannel]>>;
    }

    /** @name CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity (124) */
    interface CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity extends Struct {
        readonly remainingCount: u32;
        readonly remainingSize: u32;
    }

    /** @name PolkadotPrimitivesV8AbridgedHrmpChannel (127) */
    interface PolkadotPrimitivesV8AbridgedHrmpChannel extends Struct {
        readonly maxCapacity: u32;
        readonly maxTotalSize: u32;
        readonly maxMessageSize: u32;
        readonly msgCount: u32;
        readonly totalSize: u32;
        readonly mqcHead: Option<H256>;
    }

    /** @name PolkadotPrimitivesV8AbridgedHostConfiguration (128) */
    interface PolkadotPrimitivesV8AbridgedHostConfiguration extends Struct {
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
    }

    /** @name PolkadotPrimitivesV8AsyncBackingAsyncBackingParams (129) */
    interface PolkadotPrimitivesV8AsyncBackingAsyncBackingParams extends Struct {
        readonly maxCandidateDepth: u32;
        readonly allowedAncestryLen: u32;
    }

    /** @name PolkadotCorePrimitivesOutboundHrmpMessage (135) */
    interface PolkadotCorePrimitivesOutboundHrmpMessage extends Struct {
        readonly recipient: u32;
        readonly data: Bytes;
    }

    /** @name CumulusPalletParachainSystemCall (137) */
    interface CumulusPalletParachainSystemCall extends Enum {
        readonly isSetValidationData: boolean;
        readonly asSetValidationData: {
            readonly data: CumulusPrimitivesParachainInherentParachainInherentData;
        } & Struct;
        readonly isSudoSendUpwardMessage: boolean;
        readonly asSudoSendUpwardMessage: {
            readonly message: Bytes;
        } & Struct;
        readonly type: "SetValidationData" | "SudoSendUpwardMessage";
    }

    /** @name CumulusPrimitivesParachainInherentParachainInherentData (138) */
    interface CumulusPrimitivesParachainInherentParachainInherentData extends Struct {
        readonly validationData: PolkadotPrimitivesV8PersistedValidationData;
        readonly relayChainState: SpTrieStorageProof;
        readonly downwardMessages: Vec<PolkadotCorePrimitivesInboundDownwardMessage>;
        readonly horizontalMessages: BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>;
    }

    /** @name PolkadotCorePrimitivesInboundDownwardMessage (140) */
    interface PolkadotCorePrimitivesInboundDownwardMessage extends Struct {
        readonly sentAt: u32;
        readonly msg: Bytes;
    }

    /** @name PolkadotCorePrimitivesInboundHrmpMessage (143) */
    interface PolkadotCorePrimitivesInboundHrmpMessage extends Struct {
        readonly sentAt: u32;
        readonly data: Bytes;
    }

    /** @name CumulusPalletParachainSystemError (146) */
    interface CumulusPalletParachainSystemError extends Enum {
        readonly isOverlappingUpgrades: boolean;
        readonly isProhibitedByPolkadot: boolean;
        readonly isTooBig: boolean;
        readonly isValidationDataNotAvailable: boolean;
        readonly isHostConfigurationNotAvailable: boolean;
        readonly isNotScheduled: boolean;
        readonly isNothingAuthorized: boolean;
        readonly isUnauthorized: boolean;
        readonly type:
            | "OverlappingUpgrades"
            | "ProhibitedByPolkadot"
            | "TooBig"
            | "ValidationDataNotAvailable"
            | "HostConfigurationNotAvailable"
            | "NotScheduled"
            | "NothingAuthorized"
            | "Unauthorized";
    }

    /** @name PalletTimestampCall (147) */
    interface PalletTimestampCall extends Enum {
        readonly isSet: boolean;
        readonly asSet: {
            readonly now: Compact<u64>;
        } & Struct;
        readonly type: "Set";
    }

    /** @name StagingParachainInfoCall (148) */
    type StagingParachainInfoCall = Null;

    /** @name PalletSudoCall (149) */
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

    /** @name PalletUtilityCall (151) */
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
            readonly asOrigin: FlashboxRuntimeOriginCaller;
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
        readonly type: "Batch" | "AsDerivative" | "BatchAll" | "DispatchAs" | "ForceBatch" | "WithWeight";
    }

    /** @name FlashboxRuntimeOriginCaller (153) */
    interface FlashboxRuntimeOriginCaller extends Enum {
        readonly isSystem: boolean;
        readonly asSystem: FrameSupportDispatchRawOrigin;
        readonly isVoid: boolean;
        readonly type: "System" | "Void";
    }

    /** @name FrameSupportDispatchRawOrigin (154) */
    interface FrameSupportDispatchRawOrigin extends Enum {
        readonly isRoot: boolean;
        readonly isSigned: boolean;
        readonly asSigned: AccountId32;
        readonly isNone: boolean;
        readonly type: "Root" | "Signed" | "None";
    }

    /** @name SpCoreVoid (155) */
    type SpCoreVoid = Null;

    /** @name PalletProxyCall (156) */
    interface PalletProxyCall extends Enum {
        readonly isProxy: boolean;
        readonly asProxy: {
            readonly real: MultiAddress;
            readonly forceProxyType: Option<FlashboxRuntimeProxyType>;
            readonly call: Call;
        } & Struct;
        readonly isAddProxy: boolean;
        readonly asAddProxy: {
            readonly delegate: MultiAddress;
            readonly proxyType: FlashboxRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly isRemoveProxy: boolean;
        readonly asRemoveProxy: {
            readonly delegate: MultiAddress;
            readonly proxyType: FlashboxRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly isRemoveProxies: boolean;
        readonly isCreatePure: boolean;
        readonly asCreatePure: {
            readonly proxyType: FlashboxRuntimeProxyType;
            readonly delay: u32;
            readonly index: u16;
        } & Struct;
        readonly isKillPure: boolean;
        readonly asKillPure: {
            readonly spawner: MultiAddress;
            readonly proxyType: FlashboxRuntimeProxyType;
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
            readonly forceProxyType: Option<FlashboxRuntimeProxyType>;
            readonly call: Call;
        } & Struct;
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
            | "ProxyAnnounced";
    }

    /** @name PalletMigrationsCall (161) */
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

    /** @name PalletMigrationsMigrationCursor (163) */
    interface PalletMigrationsMigrationCursor extends Enum {
        readonly isActive: boolean;
        readonly asActive: PalletMigrationsActiveCursor;
        readonly isStuck: boolean;
        readonly type: "Active" | "Stuck";
    }

    /** @name PalletMigrationsActiveCursor (165) */
    interface PalletMigrationsActiveCursor extends Struct {
        readonly index: u32;
        readonly innerCursor: Option<Bytes>;
        readonly startedAt: u32;
    }

    /** @name PalletMigrationsHistoricCleanupSelector (167) */
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

    /** @name PalletMaintenanceModeCall (169) */
    interface PalletMaintenanceModeCall extends Enum {
        readonly isEnterMaintenanceMode: boolean;
        readonly isResumeNormalOperation: boolean;
        readonly type: "EnterMaintenanceMode" | "ResumeNormalOperation";
    }

    /** @name PalletTxPauseCall (170) */
    interface PalletTxPauseCall extends Enum {
        readonly isPause: boolean;
        readonly asPause: {
            readonly fullName: ITuple<[Bytes, Bytes]>;
        } & Struct;
        readonly isUnpause: boolean;
        readonly asUnpause: {
            readonly ident: ITuple<[Bytes, Bytes]>;
        } & Struct;
        readonly type: "Pause" | "Unpause";
    }

    /** @name PalletBalancesCall (171) */
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

    /** @name PalletBalancesAdjustmentDirection (174) */
    interface PalletBalancesAdjustmentDirection extends Enum {
        readonly isIncrease: boolean;
        readonly isDecrease: boolean;
        readonly type: "Increase" | "Decrease";
    }

    /** @name PalletStreamPaymentCall (175) */
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
            readonly assetId: FlashboxRuntimeStreamPaymentAssetId;
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

    /** @name PalletStreamPaymentChangeKind (176) */
    interface PalletStreamPaymentChangeKind extends Enum {
        readonly isSuggestion: boolean;
        readonly isMandatory: boolean;
        readonly asMandatory: {
            readonly deadline: u128;
        } & Struct;
        readonly type: "Suggestion" | "Mandatory";
    }

    /** @name PalletIdentityCall (177) */
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

    /** @name PalletIdentityLegacyIdentityInfo (178) */
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

    /** @name PalletIdentityJudgement (215) */
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

    /** @name SpRuntimeMultiSignature (217) */
    interface SpRuntimeMultiSignature extends Enum {
        readonly isEd25519: boolean;
        readonly asEd25519: U8aFixed;
        readonly isSr25519: boolean;
        readonly asSr25519: U8aFixed;
        readonly isEcdsa: boolean;
        readonly asEcdsa: U8aFixed;
        readonly type: "Ed25519" | "Sr25519" | "Ecdsa";
    }

    /** @name PalletMultisigCall (220) */
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
        readonly type: "AsMultiThreshold1" | "AsMulti" | "ApproveAsMulti" | "CancelAsMulti";
    }

    /** @name PalletRegistrarCall (222) */
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

    /** @name DpContainerChainGenesisDataContainerChainGenesisData (223) */
    interface DpContainerChainGenesisDataContainerChainGenesisData extends Struct {
        readonly storage: Vec<DpContainerChainGenesisDataContainerChainGenesisDataItem>;
        readonly name: Bytes;
        readonly id: Bytes;
        readonly forkId: Option<Bytes>;
        readonly extensions: Bytes;
        readonly properties: DpContainerChainGenesisDataProperties;
    }

    /** @name DpContainerChainGenesisDataContainerChainGenesisDataItem (225) */
    interface DpContainerChainGenesisDataContainerChainGenesisDataItem extends Struct {
        readonly key: Bytes;
        readonly value: Bytes;
    }

    /** @name DpContainerChainGenesisDataProperties (226) */
    interface DpContainerChainGenesisDataProperties extends Struct {
        readonly tokenMetadata: DpContainerChainGenesisDataTokenMetadata;
        readonly isEthereum: bool;
    }

    /** @name DpContainerChainGenesisDataTokenMetadata (227) */
    interface DpContainerChainGenesisDataTokenMetadata extends Struct {
        readonly tokenSymbol: Bytes;
        readonly ss58Format: u32;
        readonly tokenDecimals: u32;
    }

    /** @name TpTraitsSlotFrequency (230) */
    interface TpTraitsSlotFrequency extends Struct {
        readonly min: u32;
        readonly max: u32;
    }

    /** @name TpTraitsParathreadParams (232) */
    interface TpTraitsParathreadParams extends Struct {
        readonly slotFrequency: TpTraitsSlotFrequency;
    }

    /** @name PalletConfigurationCall (233) */
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

    /** @name PalletCollatorAssignmentCall (236) */
    type PalletCollatorAssignmentCall = Null;

    /** @name PalletAuthorNotingCall (237) */
    interface PalletAuthorNotingCall extends Enum {
        readonly isSetLatestAuthorData: boolean;
        readonly asSetLatestAuthorData: {
            readonly data: TpAuthorNotingInherentOwnParachainInherentData;
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

    /** @name TpAuthorNotingInherentOwnParachainInherentData (238) */
    interface TpAuthorNotingInherentOwnParachainInherentData extends Struct {
        readonly relayStorageProof: SpTrieStorageProof;
    }

    /** @name PalletAuthorityAssignmentCall (239) */
    type PalletAuthorityAssignmentCall = Null;

    /** @name PalletServicesPaymentCall (240) */
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
            readonly maxCorePrice: Option<u128>;
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

    /** @name PalletDataPreserversCall (241) */
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
            readonly assignerParam: FlashboxRuntimePreserversAssignementPaymentExtra;
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
            readonly assignmentWitness: FlashboxRuntimePreserversAssignementPaymentWitness;
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

    /** @name PalletDataPreserversProfile (242) */
    interface PalletDataPreserversProfile extends Struct {
        readonly url: Bytes;
        readonly paraIds: PalletDataPreserversParaIdsFilter;
        readonly mode: PalletDataPreserversProfileMode;
        readonly assignmentRequest: FlashboxRuntimePreserversAssignementPaymentRequest;
    }

    /** @name PalletDataPreserversParaIdsFilter (244) */
    interface PalletDataPreserversParaIdsFilter extends Enum {
        readonly isAnyParaId: boolean;
        readonly isWhitelist: boolean;
        readonly asWhitelist: BTreeSet<u32>;
        readonly isBlacklist: boolean;
        readonly asBlacklist: BTreeSet<u32>;
        readonly type: "AnyParaId" | "Whitelist" | "Blacklist";
    }

    /** @name PalletDataPreserversProfileMode (248) */
    interface PalletDataPreserversProfileMode extends Enum {
        readonly isBootnode: boolean;
        readonly isRpc: boolean;
        readonly asRpc: {
            readonly supportsEthereumRpcs: bool;
        } & Struct;
        readonly type: "Bootnode" | "Rpc";
    }

    /** @name FlashboxRuntimePreserversAssignementPaymentRequest (249) */
    interface FlashboxRuntimePreserversAssignementPaymentRequest extends Enum {
        readonly isFree: boolean;
        readonly isStreamPayment: boolean;
        readonly asStreamPayment: {
            readonly config: PalletStreamPaymentStreamConfig;
        } & Struct;
        readonly type: "Free" | "StreamPayment";
    }

    /** @name FlashboxRuntimePreserversAssignementPaymentExtra (250) */
    interface FlashboxRuntimePreserversAssignementPaymentExtra extends Enum {
        readonly isFree: boolean;
        readonly isStreamPayment: boolean;
        readonly asStreamPayment: {
            readonly initialDeposit: u128;
        } & Struct;
        readonly type: "Free" | "StreamPayment";
    }

    /** @name FlashboxRuntimePreserversAssignementPaymentWitness (251) */
    interface FlashboxRuntimePreserversAssignementPaymentWitness extends Enum {
        readonly isFree: boolean;
        readonly isStreamPayment: boolean;
        readonly asStreamPayment: {
            readonly streamId: u64;
        } & Struct;
        readonly type: "Free" | "StreamPayment";
    }

    /** @name PalletInvulnerablesCall (252) */
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

    /** @name PalletSessionCall (253) */
    interface PalletSessionCall extends Enum {
        readonly isSetKeys: boolean;
        readonly asSetKeys: {
            readonly keys_: FlashboxRuntimeSessionKeys;
            readonly proof: Bytes;
        } & Struct;
        readonly isPurgeKeys: boolean;
        readonly type: "SetKeys" | "PurgeKeys";
    }

    /** @name FlashboxRuntimeSessionKeys (254) */
    interface FlashboxRuntimeSessionKeys extends Struct {
        readonly nimbus: NimbusPrimitivesNimbusCryptoPublic;
    }

    /** @name NimbusPrimitivesNimbusCryptoPublic (255) */
    interface NimbusPrimitivesNimbusCryptoPublic extends U8aFixed {}

    /** @name PalletAuthorInherentCall (256) */
    interface PalletAuthorInherentCall extends Enum {
        readonly isKickOffAuthorshipValidation: boolean;
        readonly type: "KickOffAuthorshipValidation";
    }

    /** @name PalletTreasuryCall (257) */
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

    /** @name PalletRootTestingCall (258) */
    interface PalletRootTestingCall extends Enum {
        readonly isFillBlock: boolean;
        readonly asFillBlock: {
            readonly ratio: Perbill;
        } & Struct;
        readonly isTriggerDefensive: boolean;
        readonly type: "FillBlock" | "TriggerDefensive";
    }

    /** @name PalletSudoError (259) */
    interface PalletSudoError extends Enum {
        readonly isRequireSudo: boolean;
        readonly type: "RequireSudo";
    }

    /** @name PalletUtilityError (260) */
    interface PalletUtilityError extends Enum {
        readonly isTooManyCalls: boolean;
        readonly type: "TooManyCalls";
    }

    /** @name PalletProxyProxyDefinition (263) */
    interface PalletProxyProxyDefinition extends Struct {
        readonly delegate: AccountId32;
        readonly proxyType: FlashboxRuntimeProxyType;
        readonly delay: u32;
    }

    /** @name PalletProxyAnnouncement (267) */
    interface PalletProxyAnnouncement extends Struct {
        readonly real: AccountId32;
        readonly callHash: H256;
        readonly height: u32;
    }

    /** @name PalletProxyError (269) */
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

    /** @name PalletMigrationsError (270) */
    interface PalletMigrationsError extends Enum {
        readonly isPreimageMissing: boolean;
        readonly isWrongUpperBound: boolean;
        readonly isPreimageIsTooBig: boolean;
        readonly isPreimageAlreadyExists: boolean;
        readonly type: "PreimageMissing" | "WrongUpperBound" | "PreimageIsTooBig" | "PreimageAlreadyExists";
    }

    /** @name PalletMaintenanceModeError (272) */
    interface PalletMaintenanceModeError extends Enum {
        readonly isAlreadyInMaintenanceMode: boolean;
        readonly isNotInMaintenanceMode: boolean;
        readonly type: "AlreadyInMaintenanceMode" | "NotInMaintenanceMode";
    }

    /** @name PalletTxPauseError (273) */
    interface PalletTxPauseError extends Enum {
        readonly isIsPaused: boolean;
        readonly isIsUnpaused: boolean;
        readonly isUnpausable: boolean;
        readonly isNotFound: boolean;
        readonly type: "IsPaused" | "IsUnpaused" | "Unpausable" | "NotFound";
    }

    /** @name PalletBalancesBalanceLock (275) */
    interface PalletBalancesBalanceLock extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
        readonly reasons: PalletBalancesReasons;
    }

    /** @name PalletBalancesReasons (276) */
    interface PalletBalancesReasons extends Enum {
        readonly isFee: boolean;
        readonly isMisc: boolean;
        readonly isAll: boolean;
        readonly type: "Fee" | "Misc" | "All";
    }

    /** @name PalletBalancesReserveData (279) */
    interface PalletBalancesReserveData extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
    }

    /** @name FrameSupportTokensMiscIdAmountRuntimeHoldReason (282) */
    interface FrameSupportTokensMiscIdAmountRuntimeHoldReason extends Struct {
        readonly id: FlashboxRuntimeRuntimeHoldReason;
        readonly amount: u128;
    }

    /** @name FlashboxRuntimeRuntimeHoldReason (283) */
    interface FlashboxRuntimeRuntimeHoldReason extends Enum {
        readonly isStreamPayment: boolean;
        readonly asStreamPayment: PalletStreamPaymentHoldReason;
        readonly isRegistrar: boolean;
        readonly asRegistrar: PalletRegistrarHoldReason;
        readonly isDataPreservers: boolean;
        readonly asDataPreservers: PalletDataPreserversHoldReason;
        readonly type: "StreamPayment" | "Registrar" | "DataPreservers";
    }

    /** @name PalletStreamPaymentHoldReason (284) */
    interface PalletStreamPaymentHoldReason extends Enum {
        readonly isStreamPayment: boolean;
        readonly isStreamOpened: boolean;
        readonly type: "StreamPayment" | "StreamOpened";
    }

    /** @name PalletRegistrarHoldReason (285) */
    interface PalletRegistrarHoldReason extends Enum {
        readonly isRegistrarDeposit: boolean;
        readonly type: "RegistrarDeposit";
    }

    /** @name PalletDataPreserversHoldReason (286) */
    interface PalletDataPreserversHoldReason extends Enum {
        readonly isProfileDeposit: boolean;
        readonly type: "ProfileDeposit";
    }

    /** @name FrameSupportTokensMiscIdAmountRuntimeFreezeReason (289) */
    interface FrameSupportTokensMiscIdAmountRuntimeFreezeReason extends Struct {
        readonly id: FlashboxRuntimeRuntimeFreezeReason;
        readonly amount: u128;
    }

    /** @name FlashboxRuntimeRuntimeFreezeReason (290) */
    interface FlashboxRuntimeRuntimeFreezeReason extends Enum {
        readonly isStreamPayment: boolean;
        readonly asStreamPayment: PalletStreamPaymentFreezeReason;
        readonly type: "StreamPayment";
    }

    /** @name PalletStreamPaymentFreezeReason (291) */
    interface PalletStreamPaymentFreezeReason extends Enum {
        readonly isStreamPayment: boolean;
        readonly type: "StreamPayment";
    }

    /** @name PalletBalancesError (293) */
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

    /** @name PalletTransactionPaymentReleases (294) */
    interface PalletTransactionPaymentReleases extends Enum {
        readonly isV1Ancient: boolean;
        readonly isV2: boolean;
        readonly type: "V1Ancient" | "V2";
    }

    /** @name PalletStreamPaymentStream (295) */
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

    /** @name PalletStreamPaymentChangeRequest (297) */
    interface PalletStreamPaymentChangeRequest extends Struct {
        readonly requester: PalletStreamPaymentParty;
        readonly kind: PalletStreamPaymentChangeKind;
        readonly newConfig: PalletStreamPaymentStreamConfig;
        readonly depositChange: Option<PalletStreamPaymentDepositChange>;
    }

    /** @name PalletStreamPaymentError (299) */
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
            | "CantFetchStatusBeforeLastTimeUpdated";
    }

    /** @name PalletIdentityRegistration (300) */
    interface PalletIdentityRegistration extends Struct {
        readonly judgements: Vec<ITuple<[u32, PalletIdentityJudgement]>>;
        readonly deposit: u128;
        readonly info: PalletIdentityLegacyIdentityInfo;
    }

    /** @name PalletIdentityRegistrarInfo (308) */
    interface PalletIdentityRegistrarInfo extends Struct {
        readonly account: AccountId32;
        readonly fee: u128;
        readonly fields: u64;
    }

    /** @name PalletIdentityAuthorityProperties (311) */
    interface PalletIdentityAuthorityProperties extends Struct {
        readonly accountId: AccountId32;
        readonly allocation: u32;
    }

    /** @name PalletIdentityUsernameInformation (312) */
    interface PalletIdentityUsernameInformation extends Struct {
        readonly owner: AccountId32;
        readonly provider: PalletIdentityProvider;
    }

    /** @name PalletIdentityProvider (313) */
    interface PalletIdentityProvider extends Enum {
        readonly isAllocation: boolean;
        readonly isAuthorityDeposit: boolean;
        readonly asAuthorityDeposit: u128;
        readonly isSystem: boolean;
        readonly type: "Allocation" | "AuthorityDeposit" | "System";
    }

    /** @name PalletIdentityError (315) */
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

    /** @name PalletMultisigMultisig (317) */
    interface PalletMultisigMultisig extends Struct {
        readonly when: PalletMultisigTimepoint;
        readonly deposit: u128;
        readonly depositor: AccountId32;
        readonly approvals: Vec<AccountId32>;
    }

    /** @name PalletMultisigError (319) */
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

    /** @name PalletRegistrarDepositInfo (328) */
    interface PalletRegistrarDepositInfo extends Struct {
        readonly creator: AccountId32;
        readonly deposit: u128;
    }

    /** @name PalletRegistrarError (329) */
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

    /** @name PalletConfigurationHostConfiguration (330) */
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

    /** @name PalletConfigurationError (333) */
    interface PalletConfigurationError extends Enum {
        readonly isInvalidNewValue: boolean;
        readonly type: "InvalidNewValue";
    }

    /** @name DpCollatorAssignmentAssignedCollatorsAccountId32 (334) */
    interface DpCollatorAssignmentAssignedCollatorsAccountId32 extends Struct {
        readonly orchestratorChain: Vec<AccountId32>;
        readonly containerChains: BTreeMap<u32, Vec<AccountId32>>;
    }

    /** @name TpTraitsContainerChainBlockInfo (339) */
    interface TpTraitsContainerChainBlockInfo extends Struct {
        readonly blockNumber: u32;
        readonly author: AccountId32;
        readonly latestSlotNumber: u64;
    }

    /** @name PalletAuthorNotingError (340) */
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

    /** @name DpCollatorAssignmentAssignedCollatorsPublic (341) */
    interface DpCollatorAssignmentAssignedCollatorsPublic extends Struct {
        readonly orchestratorChain: Vec<NimbusPrimitivesNimbusCryptoPublic>;
        readonly containerChains: BTreeMap<u32, Vec<NimbusPrimitivesNimbusCryptoPublic>>;
    }

    /** @name PalletServicesPaymentError (346) */
    interface PalletServicesPaymentError extends Enum {
        readonly isInsufficientFundsToPurchaseCredits: boolean;
        readonly isInsufficientCredits: boolean;
        readonly isCreditPriceTooExpensive: boolean;
        readonly type: "InsufficientFundsToPurchaseCredits" | "InsufficientCredits" | "CreditPriceTooExpensive";
    }

    /** @name PalletDataPreserversRegisteredProfile (347) */
    interface PalletDataPreserversRegisteredProfile extends Struct {
        readonly account: AccountId32;
        readonly deposit: u128;
        readonly profile: PalletDataPreserversProfile;
        readonly assignment: Option<ITuple<[u32, FlashboxRuntimePreserversAssignementPaymentWitness]>>;
    }

    /** @name PalletDataPreserversError (353) */
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

    /** @name PalletInvulnerablesError (355) */
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

    /** @name SpCoreCryptoKeyTypeId (360) */
    interface SpCoreCryptoKeyTypeId extends U8aFixed {}

    /** @name PalletSessionError (361) */
    interface PalletSessionError extends Enum {
        readonly isInvalidProof: boolean;
        readonly isNoAssociatedValidatorId: boolean;
        readonly isDuplicatedKey: boolean;
        readonly isNoKeys: boolean;
        readonly isNoAccount: boolean;
        readonly type: "InvalidProof" | "NoAssociatedValidatorId" | "DuplicatedKey" | "NoKeys" | "NoAccount";
    }

    /** @name PalletAuthorInherentError (365) */
    interface PalletAuthorInherentError extends Enum {
        readonly isAuthorAlreadySet: boolean;
        readonly isNoAccountId: boolean;
        readonly isCannotBeAuthor: boolean;
        readonly type: "AuthorAlreadySet" | "NoAccountId" | "CannotBeAuthor";
    }

    /** @name PalletInflationRewardsChainsToRewardValue (366) */
    interface PalletInflationRewardsChainsToRewardValue extends Struct {
        readonly paraIds: Vec<u32>;
        readonly rewardsPerChain: u128;
    }

    /** @name PalletTreasuryProposal (367) */
    interface PalletTreasuryProposal extends Struct {
        readonly proposer: AccountId32;
        readonly value: u128;
        readonly beneficiary: AccountId32;
        readonly bond: u128;
    }

    /** @name PalletTreasurySpendStatus (369) */
    interface PalletTreasurySpendStatus extends Struct {
        readonly assetKind: Null;
        readonly amount: u128;
        readonly beneficiary: AccountId32;
        readonly validFrom: u32;
        readonly expireAt: u32;
        readonly status: PalletTreasuryPaymentState;
    }

    /** @name PalletTreasuryPaymentState (370) */
    interface PalletTreasuryPaymentState extends Enum {
        readonly isPending: boolean;
        readonly isAttempted: boolean;
        readonly asAttempted: {
            readonly id: Null;
        } & Struct;
        readonly isFailed: boolean;
        readonly type: "Pending" | "Attempted" | "Failed";
    }

    /** @name FrameSupportPalletId (372) */
    interface FrameSupportPalletId extends U8aFixed {}

    /** @name PalletTreasuryError (373) */
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

    /** @name FrameSystemExtensionsCheckNonZeroSender (378) */
    type FrameSystemExtensionsCheckNonZeroSender = Null;

    /** @name FrameSystemExtensionsCheckSpecVersion (379) */
    type FrameSystemExtensionsCheckSpecVersion = Null;

    /** @name FrameSystemExtensionsCheckTxVersion (380) */
    type FrameSystemExtensionsCheckTxVersion = Null;

    /** @name FrameSystemExtensionsCheckGenesis (381) */
    type FrameSystemExtensionsCheckGenesis = Null;

    /** @name FrameSystemExtensionsCheckNonce (384) */
    interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

    /** @name FrameSystemExtensionsCheckWeight (385) */
    type FrameSystemExtensionsCheckWeight = Null;

    /** @name PalletTransactionPaymentChargeTransactionPayment (386) */
    interface PalletTransactionPaymentChargeTransactionPayment extends Compact<u128> {}

    /** @name CumulusPrimitivesStorageWeightReclaimStorageWeightReclaim (387) */
    type CumulusPrimitivesStorageWeightReclaimStorageWeightReclaim = Null;

    /** @name FlashboxRuntimeRuntime (388) */
    type FlashboxRuntimeRuntime = Null;
} // declare module
