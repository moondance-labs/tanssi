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
            readonly dispatchInfo: FrameSupportDispatchDispatchInfo;
        } & Struct;
        readonly isExtrinsicFailed: boolean;
        readonly asExtrinsicFailed: {
            readonly dispatchError: SpRuntimeDispatchError;
            readonly dispatchInfo: FrameSupportDispatchDispatchInfo;
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

    /** @name FrameSupportDispatchDispatchInfo (23) */
    interface FrameSupportDispatchDispatchInfo extends Struct {
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
            | "RootNotAllowed";
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

    /** @name CumulusPalletParachainSystemEvent (31) */
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

    /** @name PalletSudoEvent (33) */
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

    /** @name PalletUtilityEvent (37) */
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

    /** @name PalletProxyEvent (38) */
    interface PalletProxyEvent extends Enum {
        readonly isProxyExecuted: boolean;
        readonly asProxyExecuted: {
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isPureCreated: boolean;
        readonly asPureCreated: {
            readonly pure: AccountId32;
            readonly who: AccountId32;
            readonly proxyType: DanceboxRuntimeProxyType;
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
            readonly proxyType: DanceboxRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly isProxyRemoved: boolean;
        readonly asProxyRemoved: {
            readonly delegator: AccountId32;
            readonly delegatee: AccountId32;
            readonly proxyType: DanceboxRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly type: "ProxyExecuted" | "PureCreated" | "Announced" | "ProxyAdded" | "ProxyRemoved";
    }

    /** @name DanceboxRuntimeProxyType (39) */
    interface DanceboxRuntimeProxyType extends Enum {
        readonly isAny: boolean;
        readonly isNonTransfer: boolean;
        readonly isGovernance: boolean;
        readonly isStaking: boolean;
        readonly isCancelProxy: boolean;
        readonly isBalances: boolean;
        readonly isRegistrar: boolean;
        readonly isSudoRegistrar: boolean;
        readonly isSessionKeyManagement: boolean;
        readonly type:
            | "Any"
            | "NonTransfer"
            | "Governance"
            | "Staking"
            | "CancelProxy"
            | "Balances"
            | "Registrar"
            | "SudoRegistrar"
            | "SessionKeyManagement";
    }

    /** @name PalletMigrationsEvent (41) */
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

    /** @name PalletMaintenanceModeEvent (42) */
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

    /** @name PalletTxPauseEvent (43) */
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

    /** @name PalletBalancesEvent (46) */
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

    /** @name FrameSupportTokensMiscBalanceStatus (47) */
    interface FrameSupportTokensMiscBalanceStatus extends Enum {
        readonly isFree: boolean;
        readonly isReserved: boolean;
        readonly type: "Free" | "Reserved";
    }

    /** @name PalletTransactionPaymentEvent (48) */
    interface PalletTransactionPaymentEvent extends Enum {
        readonly isTransactionFeePaid: boolean;
        readonly asTransactionFeePaid: {
            readonly who: AccountId32;
            readonly actualFee: u128;
            readonly tip: u128;
        } & Struct;
        readonly type: "TransactionFeePaid";
    }

    /** @name PalletStreamPaymentEvent (49) */
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

    /** @name PalletStreamPaymentParty (50) */
    interface PalletStreamPaymentParty extends Enum {
        readonly isSource: boolean;
        readonly isTarget: boolean;
        readonly type: "Source" | "Target";
    }

    /** @name PalletStreamPaymentStreamConfig (51) */
    interface PalletStreamPaymentStreamConfig extends Struct {
        readonly timeUnit: DanceboxRuntimeTimeUnit;
        readonly assetId: DanceboxRuntimeStreamPaymentAssetId;
        readonly rate: u128;
    }

    /** @name DanceboxRuntimeTimeUnit (52) */
    interface DanceboxRuntimeTimeUnit extends Enum {
        readonly isBlockNumber: boolean;
        readonly isTimestamp: boolean;
        readonly type: "BlockNumber" | "Timestamp";
    }

    /** @name DanceboxRuntimeStreamPaymentAssetId (53) */
    interface DanceboxRuntimeStreamPaymentAssetId extends Enum {
        readonly isNative: boolean;
        readonly type: "Native";
    }

    /** @name PalletStreamPaymentDepositChange (55) */
    interface PalletStreamPaymentDepositChange extends Enum {
        readonly isIncrease: boolean;
        readonly asIncrease: u128;
        readonly isDecrease: boolean;
        readonly asDecrease: u128;
        readonly isAbsolute: boolean;
        readonly asAbsolute: u128;
        readonly type: "Increase" | "Decrease" | "Absolute";
    }

    /** @name PalletIdentityEvent (56) */
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
        readonly type:
            | "IdentitySet"
            | "IdentityCleared"
            | "IdentityKilled"
            | "JudgementRequested"
            | "JudgementUnrequested"
            | "JudgementGiven"
            | "RegistrarAdded"
            | "SubIdentityAdded"
            | "SubIdentityRemoved"
            | "SubIdentityRevoked"
            | "AuthorityAdded"
            | "AuthorityRemoved"
            | "UsernameSet"
            | "UsernameQueued"
            | "PreapprovalExpired"
            | "PrimaryUsernameSet"
            | "DanglingUsernameRemoved";
    }

    /** @name PalletMultisigEvent (58) */
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

    /** @name PalletMultisigTimepoint (59) */
    interface PalletMultisigTimepoint extends Struct {
        readonly height: u32;
        readonly index: u32;
    }

    /** @name PalletRegistrarEvent (60) */
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

    /** @name PalletCollatorAssignmentEvent (62) */
    interface PalletCollatorAssignmentEvent extends Enum {
        readonly isNewPendingAssignment: boolean;
        readonly asNewPendingAssignment: {
            readonly randomSeed: U8aFixed;
            readonly fullRotation: bool;
            readonly targetSession: u32;
        } & Struct;
        readonly type: "NewPendingAssignment";
    }

    /** @name PalletAuthorNotingEvent (63) */
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

    /** @name PalletServicesPaymentEvent (65) */
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

    /** @name PalletDataPreserversEvent (67) */
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

    /** @name PalletInvulnerablesEvent (68) */
    interface PalletInvulnerablesEvent extends Enum {
        readonly isNewInvulnerables: boolean;
        readonly asNewInvulnerables: {
            readonly invulnerables: Vec<AccountId32>;
        } & Struct;
        readonly isInvulnerableAdded: boolean;
        readonly asInvulnerableAdded: {
            readonly accountId: AccountId32;
        } & Struct;
        readonly isInvulnerableRemoved: boolean;
        readonly asInvulnerableRemoved: {
            readonly accountId: AccountId32;
        } & Struct;
        readonly isInvalidInvulnerableSkipped: boolean;
        readonly asInvalidInvulnerableSkipped: {
            readonly accountId: AccountId32;
        } & Struct;
        readonly type: "NewInvulnerables" | "InvulnerableAdded" | "InvulnerableRemoved" | "InvalidInvulnerableSkipped";
    }

    /** @name PalletSessionEvent (70) */
    interface PalletSessionEvent extends Enum {
        readonly isNewSession: boolean;
        readonly asNewSession: {
            readonly sessionIndex: u32;
        } & Struct;
        readonly type: "NewSession";
    }

    /** @name PalletPooledStakingEvent (71) */
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
            readonly pool: PalletPooledStakingTargetPool;
            readonly pending: u128;
        } & Struct;
        readonly isExecutedDelegate: boolean;
        readonly asExecutedDelegate: {
            readonly candidate: AccountId32;
            readonly delegator: AccountId32;
            readonly pool: PalletPooledStakingTargetPool;
            readonly staked: u128;
            readonly released: u128;
        } & Struct;
        readonly isRequestedUndelegate: boolean;
        readonly asRequestedUndelegate: {
            readonly candidate: AccountId32;
            readonly delegator: AccountId32;
            readonly from: PalletPooledStakingTargetPool;
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
            readonly sourcePool: PalletPooledStakingTargetPool;
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

    /** @name PalletPooledStakingTargetPool (73) */
    interface PalletPooledStakingTargetPool extends Enum {
        readonly isAutoCompounding: boolean;
        readonly isManualRewards: boolean;
        readonly type: "AutoCompounding" | "ManualRewards";
    }

    /** @name PalletInflationRewardsEvent (74) */
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

    /** @name PalletTreasuryEvent (75) */
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

    /** @name CumulusPalletXcmpQueueEvent (76) */
    interface CumulusPalletXcmpQueueEvent extends Enum {
        readonly isXcmpMessageSent: boolean;
        readonly asXcmpMessageSent: {
            readonly messageHash: U8aFixed;
        } & Struct;
        readonly type: "XcmpMessageSent";
    }

    /** @name CumulusPalletXcmEvent (77) */
    interface CumulusPalletXcmEvent extends Enum {
        readonly isInvalidFormat: boolean;
        readonly asInvalidFormat: U8aFixed;
        readonly isUnsupportedVersion: boolean;
        readonly asUnsupportedVersion: U8aFixed;
        readonly isExecutedDownward: boolean;
        readonly asExecutedDownward: ITuple<[U8aFixed, StagingXcmV4TraitsOutcome]>;
        readonly type: "InvalidFormat" | "UnsupportedVersion" | "ExecutedDownward";
    }

    /** @name StagingXcmV4TraitsOutcome (78) */
    interface StagingXcmV4TraitsOutcome extends Enum {
        readonly isComplete: boolean;
        readonly asComplete: {
            readonly used: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isIncomplete: boolean;
        readonly asIncomplete: {
            readonly used: SpWeightsWeightV2Weight;
            readonly error: XcmV3TraitsError;
        } & Struct;
        readonly isError: boolean;
        readonly asError: {
            readonly error: XcmV3TraitsError;
        } & Struct;
        readonly type: "Complete" | "Incomplete" | "Error";
    }

    /** @name XcmV3TraitsError (79) */
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

    /** @name PalletXcmEvent (80) */
    interface PalletXcmEvent extends Enum {
        readonly isAttempted: boolean;
        readonly asAttempted: {
            readonly outcome: StagingXcmV4TraitsOutcome;
        } & Struct;
        readonly isSent: boolean;
        readonly asSent: {
            readonly origin: StagingXcmV4Location;
            readonly destination: StagingXcmV4Location;
            readonly message: StagingXcmV4Xcm;
            readonly messageId: U8aFixed;
        } & Struct;
        readonly isUnexpectedResponse: boolean;
        readonly asUnexpectedResponse: {
            readonly origin: StagingXcmV4Location;
            readonly queryId: u64;
        } & Struct;
        readonly isResponseReady: boolean;
        readonly asResponseReady: {
            readonly queryId: u64;
            readonly response: StagingXcmV4Response;
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
            readonly origin: StagingXcmV4Location;
            readonly queryId: u64;
            readonly expectedLocation: Option<StagingXcmV4Location>;
        } & Struct;
        readonly isInvalidResponderVersion: boolean;
        readonly asInvalidResponderVersion: {
            readonly origin: StagingXcmV4Location;
            readonly queryId: u64;
        } & Struct;
        readonly isResponseTaken: boolean;
        readonly asResponseTaken: {
            readonly queryId: u64;
        } & Struct;
        readonly isAssetsTrapped: boolean;
        readonly asAssetsTrapped: {
            readonly hash_: H256;
            readonly origin: StagingXcmV4Location;
            readonly assets: XcmVersionedAssets;
        } & Struct;
        readonly isVersionChangeNotified: boolean;
        readonly asVersionChangeNotified: {
            readonly destination: StagingXcmV4Location;
            readonly result: u32;
            readonly cost: StagingXcmV4AssetAssets;
            readonly messageId: U8aFixed;
        } & Struct;
        readonly isSupportedVersionChanged: boolean;
        readonly asSupportedVersionChanged: {
            readonly location: StagingXcmV4Location;
            readonly version: u32;
        } & Struct;
        readonly isNotifyTargetSendFail: boolean;
        readonly asNotifyTargetSendFail: {
            readonly location: StagingXcmV4Location;
            readonly queryId: u64;
            readonly error: XcmV3TraitsError;
        } & Struct;
        readonly isNotifyTargetMigrationFail: boolean;
        readonly asNotifyTargetMigrationFail: {
            readonly location: XcmVersionedLocation;
            readonly queryId: u64;
        } & Struct;
        readonly isInvalidQuerierVersion: boolean;
        readonly asInvalidQuerierVersion: {
            readonly origin: StagingXcmV4Location;
            readonly queryId: u64;
        } & Struct;
        readonly isInvalidQuerier: boolean;
        readonly asInvalidQuerier: {
            readonly origin: StagingXcmV4Location;
            readonly queryId: u64;
            readonly expectedQuerier: StagingXcmV4Location;
            readonly maybeActualQuerier: Option<StagingXcmV4Location>;
        } & Struct;
        readonly isVersionNotifyStarted: boolean;
        readonly asVersionNotifyStarted: {
            readonly destination: StagingXcmV4Location;
            readonly cost: StagingXcmV4AssetAssets;
            readonly messageId: U8aFixed;
        } & Struct;
        readonly isVersionNotifyRequested: boolean;
        readonly asVersionNotifyRequested: {
            readonly destination: StagingXcmV4Location;
            readonly cost: StagingXcmV4AssetAssets;
            readonly messageId: U8aFixed;
        } & Struct;
        readonly isVersionNotifyUnrequested: boolean;
        readonly asVersionNotifyUnrequested: {
            readonly destination: StagingXcmV4Location;
            readonly cost: StagingXcmV4AssetAssets;
            readonly messageId: U8aFixed;
        } & Struct;
        readonly isFeesPaid: boolean;
        readonly asFeesPaid: {
            readonly paying: StagingXcmV4Location;
            readonly fees: StagingXcmV4AssetAssets;
        } & Struct;
        readonly isAssetsClaimed: boolean;
        readonly asAssetsClaimed: {
            readonly hash_: H256;
            readonly origin: StagingXcmV4Location;
            readonly assets: XcmVersionedAssets;
        } & Struct;
        readonly isVersionMigrationFinished: boolean;
        readonly asVersionMigrationFinished: {
            readonly version: u32;
        } & Struct;
        readonly type:
            | "Attempted"
            | "Sent"
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
            | "VersionMigrationFinished";
    }

    /** @name StagingXcmV4Location (81) */
    interface StagingXcmV4Location extends Struct {
        readonly parents: u8;
        readonly interior: StagingXcmV4Junctions;
    }

    /** @name StagingXcmV4Junctions (82) */
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

    /** @name StagingXcmV4Junction (84) */
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

    /** @name StagingXcmV4JunctionNetworkId (87) */
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

    /** @name XcmV3JunctionBodyId (90) */
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

    /** @name XcmV3JunctionBodyPart (91) */
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

    /** @name StagingXcmV4Xcm (99) */
    interface StagingXcmV4Xcm extends Vec<StagingXcmV4Instruction> {}

    /** @name StagingXcmV4Instruction (101) */
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

    /** @name StagingXcmV4AssetAssets (102) */
    interface StagingXcmV4AssetAssets extends Vec<StagingXcmV4Asset> {}

    /** @name StagingXcmV4Asset (104) */
    interface StagingXcmV4Asset extends Struct {
        readonly id: StagingXcmV4AssetAssetId;
        readonly fun: StagingXcmV4AssetFungibility;
    }

    /** @name StagingXcmV4AssetAssetId (105) */
    interface StagingXcmV4AssetAssetId extends StagingXcmV4Location {}

    /** @name StagingXcmV4AssetFungibility (106) */
    interface StagingXcmV4AssetFungibility extends Enum {
        readonly isFungible: boolean;
        readonly asFungible: Compact<u128>;
        readonly isNonFungible: boolean;
        readonly asNonFungible: StagingXcmV4AssetAssetInstance;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name StagingXcmV4AssetAssetInstance (107) */
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

    /** @name StagingXcmV4Response (110) */
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

    /** @name StagingXcmV4PalletInfo (114) */
    interface StagingXcmV4PalletInfo extends Struct {
        readonly index: Compact<u32>;
        readonly name: Bytes;
        readonly moduleName: Bytes;
        readonly major: Compact<u32>;
        readonly minor: Compact<u32>;
        readonly patch: Compact<u32>;
    }

    /** @name XcmV3MaybeErrorCode (117) */
    interface XcmV3MaybeErrorCode extends Enum {
        readonly isSuccess: boolean;
        readonly isError: boolean;
        readonly asError: Bytes;
        readonly isTruncatedError: boolean;
        readonly asTruncatedError: Bytes;
        readonly type: "Success" | "Error" | "TruncatedError";
    }

    /** @name XcmV3OriginKind (120) */
    interface XcmV3OriginKind extends Enum {
        readonly isNative: boolean;
        readonly isSovereignAccount: boolean;
        readonly isSuperuser: boolean;
        readonly isXcm: boolean;
        readonly type: "Native" | "SovereignAccount" | "Superuser" | "Xcm";
    }

    /** @name XcmDoubleEncoded (121) */
    interface XcmDoubleEncoded extends Struct {
        readonly encoded: Bytes;
    }

    /** @name StagingXcmV4QueryResponseInfo (122) */
    interface StagingXcmV4QueryResponseInfo extends Struct {
        readonly destination: StagingXcmV4Location;
        readonly queryId: Compact<u64>;
        readonly maxWeight: SpWeightsWeightV2Weight;
    }

    /** @name StagingXcmV4AssetAssetFilter (123) */
    interface StagingXcmV4AssetAssetFilter extends Enum {
        readonly isDefinite: boolean;
        readonly asDefinite: StagingXcmV4AssetAssets;
        readonly isWild: boolean;
        readonly asWild: StagingXcmV4AssetWildAsset;
        readonly type: "Definite" | "Wild";
    }

    /** @name StagingXcmV4AssetWildAsset (124) */
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

    /** @name StagingXcmV4AssetWildFungibility (125) */
    interface StagingXcmV4AssetWildFungibility extends Enum {
        readonly isFungible: boolean;
        readonly isNonFungible: boolean;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name XcmV3WeightLimit (126) */
    interface XcmV3WeightLimit extends Enum {
        readonly isUnlimited: boolean;
        readonly isLimited: boolean;
        readonly asLimited: SpWeightsWeightV2Weight;
        readonly type: "Unlimited" | "Limited";
    }

    /** @name XcmVersionedAssets (127) */
    interface XcmVersionedAssets extends Enum {
        readonly isV2: boolean;
        readonly asV2: XcmV2MultiassetMultiAssets;
        readonly isV3: boolean;
        readonly asV3: XcmV3MultiassetMultiAssets;
        readonly isV4: boolean;
        readonly asV4: StagingXcmV4AssetAssets;
        readonly type: "V2" | "V3" | "V4";
    }

    /** @name XcmV2MultiassetMultiAssets (128) */
    interface XcmV2MultiassetMultiAssets extends Vec<XcmV2MultiAsset> {}

    /** @name XcmV2MultiAsset (130) */
    interface XcmV2MultiAsset extends Struct {
        readonly id: XcmV2MultiassetAssetId;
        readonly fun: XcmV2MultiassetFungibility;
    }

    /** @name XcmV2MultiassetAssetId (131) */
    interface XcmV2MultiassetAssetId extends Enum {
        readonly isConcrete: boolean;
        readonly asConcrete: XcmV2MultiLocation;
        readonly isAbstract: boolean;
        readonly asAbstract: Bytes;
        readonly type: "Concrete" | "Abstract";
    }

    /** @name XcmV2MultiLocation (132) */
    interface XcmV2MultiLocation extends Struct {
        readonly parents: u8;
        readonly interior: XcmV2MultilocationJunctions;
    }

    /** @name XcmV2MultilocationJunctions (133) */
    interface XcmV2MultilocationJunctions extends Enum {
        readonly isHere: boolean;
        readonly isX1: boolean;
        readonly asX1: XcmV2Junction;
        readonly isX2: boolean;
        readonly asX2: ITuple<[XcmV2Junction, XcmV2Junction]>;
        readonly isX3: boolean;
        readonly asX3: ITuple<[XcmV2Junction, XcmV2Junction, XcmV2Junction]>;
        readonly isX4: boolean;
        readonly asX4: ITuple<[XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction]>;
        readonly isX5: boolean;
        readonly asX5: ITuple<[XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction]>;
        readonly isX6: boolean;
        readonly asX6: ITuple<
            [XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction]
        >;
        readonly isX7: boolean;
        readonly asX7: ITuple<
            [XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction, XcmV2Junction]
        >;
        readonly isX8: boolean;
        readonly asX8: ITuple<
            [
                XcmV2Junction,
                XcmV2Junction,
                XcmV2Junction,
                XcmV2Junction,
                XcmV2Junction,
                XcmV2Junction,
                XcmV2Junction,
                XcmV2Junction,
            ]
        >;
        readonly type: "Here" | "X1" | "X2" | "X3" | "X4" | "X5" | "X6" | "X7" | "X8";
    }

    /** @name XcmV2Junction (134) */
    interface XcmV2Junction extends Enum {
        readonly isParachain: boolean;
        readonly asParachain: Compact<u32>;
        readonly isAccountId32: boolean;
        readonly asAccountId32: {
            readonly network: XcmV2NetworkId;
            readonly id: U8aFixed;
        } & Struct;
        readonly isAccountIndex64: boolean;
        readonly asAccountIndex64: {
            readonly network: XcmV2NetworkId;
            readonly index: Compact<u64>;
        } & Struct;
        readonly isAccountKey20: boolean;
        readonly asAccountKey20: {
            readonly network: XcmV2NetworkId;
            readonly key: U8aFixed;
        } & Struct;
        readonly isPalletInstance: boolean;
        readonly asPalletInstance: u8;
        readonly isGeneralIndex: boolean;
        readonly asGeneralIndex: Compact<u128>;
        readonly isGeneralKey: boolean;
        readonly asGeneralKey: Bytes;
        readonly isOnlyChild: boolean;
        readonly isPlurality: boolean;
        readonly asPlurality: {
            readonly id: XcmV2BodyId;
            readonly part: XcmV2BodyPart;
        } & Struct;
        readonly type:
            | "Parachain"
            | "AccountId32"
            | "AccountIndex64"
            | "AccountKey20"
            | "PalletInstance"
            | "GeneralIndex"
            | "GeneralKey"
            | "OnlyChild"
            | "Plurality";
    }

    /** @name XcmV2NetworkId (135) */
    interface XcmV2NetworkId extends Enum {
        readonly isAny: boolean;
        readonly isNamed: boolean;
        readonly asNamed: Bytes;
        readonly isPolkadot: boolean;
        readonly isKusama: boolean;
        readonly type: "Any" | "Named" | "Polkadot" | "Kusama";
    }

    /** @name XcmV2BodyId (137) */
    interface XcmV2BodyId extends Enum {
        readonly isUnit: boolean;
        readonly isNamed: boolean;
        readonly asNamed: Bytes;
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
            | "Named"
            | "Index"
            | "Executive"
            | "Technical"
            | "Legislative"
            | "Judicial"
            | "Defense"
            | "Administration"
            | "Treasury";
    }

    /** @name XcmV2BodyPart (138) */
    interface XcmV2BodyPart extends Enum {
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

    /** @name XcmV2MultiassetFungibility (139) */
    interface XcmV2MultiassetFungibility extends Enum {
        readonly isFungible: boolean;
        readonly asFungible: Compact<u128>;
        readonly isNonFungible: boolean;
        readonly asNonFungible: XcmV2MultiassetAssetInstance;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name XcmV2MultiassetAssetInstance (140) */
    interface XcmV2MultiassetAssetInstance extends Enum {
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
        readonly isBlob: boolean;
        readonly asBlob: Bytes;
        readonly type: "Undefined" | "Index" | "Array4" | "Array8" | "Array16" | "Array32" | "Blob";
    }

    /** @name XcmV3MultiassetMultiAssets (141) */
    interface XcmV3MultiassetMultiAssets extends Vec<XcmV3MultiAsset> {}

    /** @name XcmV3MultiAsset (143) */
    interface XcmV3MultiAsset extends Struct {
        readonly id: XcmV3MultiassetAssetId;
        readonly fun: XcmV3MultiassetFungibility;
    }

    /** @name XcmV3MultiassetAssetId (144) */
    interface XcmV3MultiassetAssetId extends Enum {
        readonly isConcrete: boolean;
        readonly asConcrete: StagingXcmV3MultiLocation;
        readonly isAbstract: boolean;
        readonly asAbstract: U8aFixed;
        readonly type: "Concrete" | "Abstract";
    }

    /** @name StagingXcmV3MultiLocation (145) */
    interface StagingXcmV3MultiLocation extends Struct {
        readonly parents: u8;
        readonly interior: XcmV3Junctions;
    }

    /** @name XcmV3Junctions (146) */
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

    /** @name XcmV3Junction (147) */
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

    /** @name XcmV3JunctionNetworkId (149) */
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

    /** @name XcmV3MultiassetFungibility (150) */
    interface XcmV3MultiassetFungibility extends Enum {
        readonly isFungible: boolean;
        readonly asFungible: Compact<u128>;
        readonly isNonFungible: boolean;
        readonly asNonFungible: XcmV3MultiassetAssetInstance;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name XcmV3MultiassetAssetInstance (151) */
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

    /** @name XcmVersionedLocation (152) */
    interface XcmVersionedLocation extends Enum {
        readonly isV2: boolean;
        readonly asV2: XcmV2MultiLocation;
        readonly isV3: boolean;
        readonly asV3: StagingXcmV3MultiLocation;
        readonly isV4: boolean;
        readonly asV4: StagingXcmV4Location;
        readonly type: "V2" | "V3" | "V4";
    }

    /** @name PalletAssetsEvent (153) */
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

    /** @name PalletForeignAssetCreatorEvent (154) */
    interface PalletForeignAssetCreatorEvent extends Enum {
        readonly isForeignAssetCreated: boolean;
        readonly asForeignAssetCreated: {
            readonly assetId: u16;
            readonly foreignAsset: StagingXcmV4Location;
        } & Struct;
        readonly isForeignAssetTypeChanged: boolean;
        readonly asForeignAssetTypeChanged: {
            readonly assetId: u16;
            readonly newForeignAsset: StagingXcmV4Location;
        } & Struct;
        readonly isForeignAssetRemoved: boolean;
        readonly asForeignAssetRemoved: {
            readonly assetId: u16;
            readonly foreignAsset: StagingXcmV4Location;
        } & Struct;
        readonly isForeignAssetDestroyed: boolean;
        readonly asForeignAssetDestroyed: {
            readonly assetId: u16;
            readonly foreignAsset: StagingXcmV4Location;
        } & Struct;
        readonly type:
            | "ForeignAssetCreated"
            | "ForeignAssetTypeChanged"
            | "ForeignAssetRemoved"
            | "ForeignAssetDestroyed";
    }

    /** @name PalletAssetRateEvent (155) */
    interface PalletAssetRateEvent extends Enum {
        readonly isAssetRateCreated: boolean;
        readonly asAssetRateCreated: {
            readonly assetKind: u16;
            readonly rate: u128;
        } & Struct;
        readonly isAssetRateRemoved: boolean;
        readonly asAssetRateRemoved: {
            readonly assetKind: u16;
        } & Struct;
        readonly isAssetRateUpdated: boolean;
        readonly asAssetRateUpdated: {
            readonly assetKind: u16;
            readonly old: u128;
            readonly new_: u128;
        } & Struct;
        readonly type: "AssetRateCreated" | "AssetRateRemoved" | "AssetRateUpdated";
    }

    /** @name PalletMessageQueueEvent (157) */
    interface PalletMessageQueueEvent extends Enum {
        readonly isProcessingFailed: boolean;
        readonly asProcessingFailed: {
            readonly id: H256;
            readonly origin: CumulusPrimitivesCoreAggregateMessageOrigin;
            readonly error: FrameSupportMessagesProcessMessageError;
        } & Struct;
        readonly isProcessed: boolean;
        readonly asProcessed: {
            readonly id: H256;
            readonly origin: CumulusPrimitivesCoreAggregateMessageOrigin;
            readonly weightUsed: SpWeightsWeightV2Weight;
            readonly success: bool;
        } & Struct;
        readonly isOverweightEnqueued: boolean;
        readonly asOverweightEnqueued: {
            readonly id: U8aFixed;
            readonly origin: CumulusPrimitivesCoreAggregateMessageOrigin;
            readonly pageIndex: u32;
            readonly messageIndex: u32;
        } & Struct;
        readonly isPageReaped: boolean;
        readonly asPageReaped: {
            readonly origin: CumulusPrimitivesCoreAggregateMessageOrigin;
            readonly index: u32;
        } & Struct;
        readonly type: "ProcessingFailed" | "Processed" | "OverweightEnqueued" | "PageReaped";
    }

    /** @name CumulusPrimitivesCoreAggregateMessageOrigin (158) */
    interface CumulusPrimitivesCoreAggregateMessageOrigin extends Enum {
        readonly isHere: boolean;
        readonly isParent: boolean;
        readonly isSibling: boolean;
        readonly asSibling: u32;
        readonly type: "Here" | "Parent" | "Sibling";
    }

    /** @name FrameSupportMessagesProcessMessageError (159) */
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

    /** @name PalletXcmCoreBuyerEvent (160) */
    interface PalletXcmCoreBuyerEvent extends Enum {
        readonly isBuyCoreXcmSent: boolean;
        readonly asBuyCoreXcmSent: {
            readonly paraId: u32;
            readonly transactionStatusQueryId: u64;
        } & Struct;
        readonly isReceivedBuyCoreXCMResult: boolean;
        readonly asReceivedBuyCoreXCMResult: {
            readonly paraId: u32;
            readonly response: StagingXcmV4Response;
        } & Struct;
        readonly isCleanedUpExpiredPendingBlocksEntries: boolean;
        readonly asCleanedUpExpiredPendingBlocksEntries: {
            readonly paraIds: Vec<u32>;
        } & Struct;
        readonly isCleanedUpExpiredInFlightOrderEntries: boolean;
        readonly asCleanedUpExpiredInFlightOrderEntries: {
            readonly paraIds: Vec<u32>;
        } & Struct;
        readonly type:
            | "BuyCoreXcmSent"
            | "ReceivedBuyCoreXCMResult"
            | "CleanedUpExpiredPendingBlocksEntries"
            | "CleanedUpExpiredInFlightOrderEntries";
    }

    /** @name PalletRootTestingEvent (162) */
    interface PalletRootTestingEvent extends Enum {
        readonly isDefensiveTestCall: boolean;
        readonly type: "DefensiveTestCall";
    }

    /** @name FrameSystemPhase (163) */
    interface FrameSystemPhase extends Enum {
        readonly isApplyExtrinsic: boolean;
        readonly asApplyExtrinsic: u32;
        readonly isFinalization: boolean;
        readonly isInitialization: boolean;
        readonly type: "ApplyExtrinsic" | "Finalization" | "Initialization";
    }

    /** @name FrameSystemLastRuntimeUpgradeInfo (167) */
    interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
        readonly specVersion: Compact<u32>;
        readonly specName: Text;
    }

    /** @name FrameSystemCodeUpgradeAuthorization (169) */
    interface FrameSystemCodeUpgradeAuthorization extends Struct {
        readonly codeHash: H256;
        readonly checkVersion: bool;
    }

    /** @name FrameSystemCall (170) */
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

    /** @name FrameSystemLimitsBlockWeights (174) */
    interface FrameSystemLimitsBlockWeights extends Struct {
        readonly baseBlock: SpWeightsWeightV2Weight;
        readonly maxBlock: SpWeightsWeightV2Weight;
        readonly perClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
    }

    /** @name FrameSupportDispatchPerDispatchClassWeightsPerClass (175) */
    interface FrameSupportDispatchPerDispatchClassWeightsPerClass extends Struct {
        readonly normal: FrameSystemLimitsWeightsPerClass;
        readonly operational: FrameSystemLimitsWeightsPerClass;
        readonly mandatory: FrameSystemLimitsWeightsPerClass;
    }

    /** @name FrameSystemLimitsWeightsPerClass (176) */
    interface FrameSystemLimitsWeightsPerClass extends Struct {
        readonly baseExtrinsic: SpWeightsWeightV2Weight;
        readonly maxExtrinsic: Option<SpWeightsWeightV2Weight>;
        readonly maxTotal: Option<SpWeightsWeightV2Weight>;
        readonly reserved: Option<SpWeightsWeightV2Weight>;
    }

    /** @name FrameSystemLimitsBlockLength (178) */
    interface FrameSystemLimitsBlockLength extends Struct {
        readonly max: FrameSupportDispatchPerDispatchClassU32;
    }

    /** @name FrameSupportDispatchPerDispatchClassU32 (179) */
    interface FrameSupportDispatchPerDispatchClassU32 extends Struct {
        readonly normal: u32;
        readonly operational: u32;
        readonly mandatory: u32;
    }

    /** @name SpWeightsRuntimeDbWeight (180) */
    interface SpWeightsRuntimeDbWeight extends Struct {
        readonly read: u64;
        readonly write: u64;
    }

    /** @name SpVersionRuntimeVersion (181) */
    interface SpVersionRuntimeVersion extends Struct {
        readonly specName: Text;
        readonly implName: Text;
        readonly authoringVersion: u32;
        readonly specVersion: u32;
        readonly implVersion: u32;
        readonly apis: Vec<ITuple<[U8aFixed, u32]>>;
        readonly transactionVersion: u32;
        readonly stateVersion: u8;
    }

    /** @name FrameSystemError (185) */
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

    /** @name CumulusPalletParachainSystemUnincludedSegmentAncestor (187) */
    interface CumulusPalletParachainSystemUnincludedSegmentAncestor extends Struct {
        readonly usedBandwidth: CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth;
        readonly paraHeadHash: Option<H256>;
        readonly consumedGoAheadSignal: Option<PolkadotPrimitivesV7UpgradeGoAhead>;
    }

    /** @name CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth (188) */
    interface CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth extends Struct {
        readonly umpMsgCount: u32;
        readonly umpTotalBytes: u32;
        readonly hrmpOutgoing: BTreeMap<u32, CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate>;
    }

    /** @name CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate (190) */
    interface CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate extends Struct {
        readonly msgCount: u32;
        readonly totalBytes: u32;
    }

    /** @name PolkadotPrimitivesV7UpgradeGoAhead (195) */
    interface PolkadotPrimitivesV7UpgradeGoAhead extends Enum {
        readonly isAbort: boolean;
        readonly isGoAhead: boolean;
        readonly type: "Abort" | "GoAhead";
    }

    /** @name CumulusPalletParachainSystemUnincludedSegmentSegmentTracker (196) */
    interface CumulusPalletParachainSystemUnincludedSegmentSegmentTracker extends Struct {
        readonly usedBandwidth: CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth;
        readonly hrmpWatermark: Option<u32>;
        readonly consumedGoAheadSignal: Option<PolkadotPrimitivesV7UpgradeGoAhead>;
    }

    /** @name PolkadotPrimitivesV7PersistedValidationData (197) */
    interface PolkadotPrimitivesV7PersistedValidationData extends Struct {
        readonly parentHead: Bytes;
        readonly relayParentNumber: u32;
        readonly relayParentStorageRoot: H256;
        readonly maxPovSize: u32;
    }

    /** @name PolkadotPrimitivesV7UpgradeRestriction (200) */
    interface PolkadotPrimitivesV7UpgradeRestriction extends Enum {
        readonly isPresent: boolean;
        readonly type: "Present";
    }

    /** @name SpTrieStorageProof (201) */
    interface SpTrieStorageProof extends Struct {
        readonly trieNodes: BTreeSet<Bytes>;
    }

    /** @name CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot (203) */
    interface CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot extends Struct {
        readonly dmqMqcHead: H256;
        readonly relayDispatchQueueRemainingCapacity: CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity;
        readonly ingressChannels: Vec<ITuple<[u32, PolkadotPrimitivesV7AbridgedHrmpChannel]>>;
        readonly egressChannels: Vec<ITuple<[u32, PolkadotPrimitivesV7AbridgedHrmpChannel]>>;
    }

    /** @name CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity (204) */
    interface CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity extends Struct {
        readonly remainingCount: u32;
        readonly remainingSize: u32;
    }

    /** @name PolkadotPrimitivesV7AbridgedHrmpChannel (207) */
    interface PolkadotPrimitivesV7AbridgedHrmpChannel extends Struct {
        readonly maxCapacity: u32;
        readonly maxTotalSize: u32;
        readonly maxMessageSize: u32;
        readonly msgCount: u32;
        readonly totalSize: u32;
        readonly mqcHead: Option<H256>;
    }

    /** @name PolkadotPrimitivesV7AbridgedHostConfiguration (208) */
    interface PolkadotPrimitivesV7AbridgedHostConfiguration extends Struct {
        readonly maxCodeSize: u32;
        readonly maxHeadDataSize: u32;
        readonly maxUpwardQueueCount: u32;
        readonly maxUpwardQueueSize: u32;
        readonly maxUpwardMessageSize: u32;
        readonly maxUpwardMessageNumPerCandidate: u32;
        readonly hrmpMaxMessageNumPerCandidate: u32;
        readonly validationUpgradeCooldown: u32;
        readonly validationUpgradeDelay: u32;
        readonly asyncBackingParams: PolkadotPrimitivesV7AsyncBackingAsyncBackingParams;
    }

    /** @name PolkadotPrimitivesV7AsyncBackingAsyncBackingParams (209) */
    interface PolkadotPrimitivesV7AsyncBackingAsyncBackingParams extends Struct {
        readonly maxCandidateDepth: u32;
        readonly allowedAncestryLen: u32;
    }

    /** @name PolkadotCorePrimitivesOutboundHrmpMessage (215) */
    interface PolkadotCorePrimitivesOutboundHrmpMessage extends Struct {
        readonly recipient: u32;
        readonly data: Bytes;
    }

    /** @name CumulusPalletParachainSystemCall (216) */
    interface CumulusPalletParachainSystemCall extends Enum {
        readonly isSetValidationData: boolean;
        readonly asSetValidationData: {
            readonly data: CumulusPrimitivesParachainInherentParachainInherentData;
        } & Struct;
        readonly isSudoSendUpwardMessage: boolean;
        readonly asSudoSendUpwardMessage: {
            readonly message: Bytes;
        } & Struct;
        readonly isAuthorizeUpgrade: boolean;
        readonly asAuthorizeUpgrade: {
            readonly codeHash: H256;
            readonly checkVersion: bool;
        } & Struct;
        readonly isEnactAuthorizedUpgrade: boolean;
        readonly asEnactAuthorizedUpgrade: {
            readonly code: Bytes;
        } & Struct;
        readonly type: "SetValidationData" | "SudoSendUpwardMessage" | "AuthorizeUpgrade" | "EnactAuthorizedUpgrade";
    }

    /** @name CumulusPrimitivesParachainInherentParachainInherentData (217) */
    interface CumulusPrimitivesParachainInherentParachainInherentData extends Struct {
        readonly validationData: PolkadotPrimitivesV7PersistedValidationData;
        readonly relayChainState: SpTrieStorageProof;
        readonly downwardMessages: Vec<PolkadotCorePrimitivesInboundDownwardMessage>;
        readonly horizontalMessages: BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>;
    }

    /** @name PolkadotCorePrimitivesInboundDownwardMessage (219) */
    interface PolkadotCorePrimitivesInboundDownwardMessage extends Struct {
        readonly sentAt: u32;
        readonly msg: Bytes;
    }

    /** @name PolkadotCorePrimitivesInboundHrmpMessage (222) */
    interface PolkadotCorePrimitivesInboundHrmpMessage extends Struct {
        readonly sentAt: u32;
        readonly data: Bytes;
    }

    /** @name CumulusPalletParachainSystemError (225) */
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

    /** @name PalletTimestampCall (226) */
    interface PalletTimestampCall extends Enum {
        readonly isSet: boolean;
        readonly asSet: {
            readonly now: Compact<u64>;
        } & Struct;
        readonly type: "Set";
    }

    /** @name StagingParachainInfoCall (227) */
    type StagingParachainInfoCall = Null;

    /** @name PalletSudoCall (228) */
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

    /** @name PalletUtilityCall (230) */
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
            readonly asOrigin: DanceboxRuntimeOriginCaller;
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

    /** @name DanceboxRuntimeOriginCaller (232) */
    interface DanceboxRuntimeOriginCaller extends Enum {
        readonly isSystem: boolean;
        readonly asSystem: FrameSupportDispatchRawOrigin;
        readonly isVoid: boolean;
        readonly isCumulusXcm: boolean;
        readonly asCumulusXcm: CumulusPalletXcmOrigin;
        readonly isPolkadotXcm: boolean;
        readonly asPolkadotXcm: PalletXcmOrigin;
        readonly type: "System" | "Void" | "CumulusXcm" | "PolkadotXcm";
    }

    /** @name FrameSupportDispatchRawOrigin (233) */
    interface FrameSupportDispatchRawOrigin extends Enum {
        readonly isRoot: boolean;
        readonly isSigned: boolean;
        readonly asSigned: AccountId32;
        readonly isNone: boolean;
        readonly type: "Root" | "Signed" | "None";
    }

    /** @name CumulusPalletXcmOrigin (234) */
    interface CumulusPalletXcmOrigin extends Enum {
        readonly isRelay: boolean;
        readonly isSiblingParachain: boolean;
        readonly asSiblingParachain: u32;
        readonly type: "Relay" | "SiblingParachain";
    }

    /** @name PalletXcmOrigin (235) */
    interface PalletXcmOrigin extends Enum {
        readonly isXcm: boolean;
        readonly asXcm: StagingXcmV4Location;
        readonly isResponse: boolean;
        readonly asResponse: StagingXcmV4Location;
        readonly type: "Xcm" | "Response";
    }

    /** @name SpCoreVoid (236) */
    type SpCoreVoid = Null;

    /** @name PalletProxyCall (237) */
    interface PalletProxyCall extends Enum {
        readonly isProxy: boolean;
        readonly asProxy: {
            readonly real: MultiAddress;
            readonly forceProxyType: Option<DanceboxRuntimeProxyType>;
            readonly call: Call;
        } & Struct;
        readonly isAddProxy: boolean;
        readonly asAddProxy: {
            readonly delegate: MultiAddress;
            readonly proxyType: DanceboxRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly isRemoveProxy: boolean;
        readonly asRemoveProxy: {
            readonly delegate: MultiAddress;
            readonly proxyType: DanceboxRuntimeProxyType;
            readonly delay: u32;
        } & Struct;
        readonly isRemoveProxies: boolean;
        readonly isCreatePure: boolean;
        readonly asCreatePure: {
            readonly proxyType: DanceboxRuntimeProxyType;
            readonly delay: u32;
            readonly index: u16;
        } & Struct;
        readonly isKillPure: boolean;
        readonly asKillPure: {
            readonly spawner: MultiAddress;
            readonly proxyType: DanceboxRuntimeProxyType;
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
            readonly forceProxyType: Option<DanceboxRuntimeProxyType>;
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

    /** @name PalletMaintenanceModeCall (241) */
    interface PalletMaintenanceModeCall extends Enum {
        readonly isEnterMaintenanceMode: boolean;
        readonly isResumeNormalOperation: boolean;
        readonly type: "EnterMaintenanceMode" | "ResumeNormalOperation";
    }

    /** @name PalletTxPauseCall (242) */
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

    /** @name PalletBalancesCall (243) */
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

    /** @name PalletBalancesAdjustmentDirection (244) */
    interface PalletBalancesAdjustmentDirection extends Enum {
        readonly isIncrease: boolean;
        readonly isDecrease: boolean;
        readonly type: "Increase" | "Decrease";
    }

    /** @name PalletStreamPaymentCall (245) */
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
            readonly assetId: DanceboxRuntimeStreamPaymentAssetId;
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

    /** @name PalletStreamPaymentChangeKind (246) */
    interface PalletStreamPaymentChangeKind extends Enum {
        readonly isSuggestion: boolean;
        readonly isMandatory: boolean;
        readonly asMandatory: {
            readonly deadline: u128;
        } & Struct;
        readonly type: "Suggestion" | "Mandatory";
    }

    /** @name PalletIdentityCall (247) */
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
            readonly authority: MultiAddress;
        } & Struct;
        readonly isSetUsernameFor: boolean;
        readonly asSetUsernameFor: {
            readonly who: MultiAddress;
            readonly username: Bytes;
            readonly signature: Option<SpRuntimeMultiSignature>;
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
        readonly isRemoveDanglingUsername: boolean;
        readonly asRemoveDanglingUsername: {
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
            | "RemoveDanglingUsername";
    }

    /** @name PalletIdentityLegacyIdentityInfo (248) */
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

    /** @name PalletIdentityJudgement (284) */
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

    /** @name SpRuntimeMultiSignature (286) */
    interface SpRuntimeMultiSignature extends Enum {
        readonly isEd25519: boolean;
        readonly asEd25519: U8aFixed;
        readonly isSr25519: boolean;
        readonly asSr25519: U8aFixed;
        readonly isEcdsa: boolean;
        readonly asEcdsa: U8aFixed;
        readonly type: "Ed25519" | "Sr25519" | "Ecdsa";
    }

    /** @name PalletMultisigCall (289) */
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

    /** @name PalletRegistrarCall (291) */
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

    /** @name DpContainerChainGenesisDataContainerChainGenesisData (292) */
    interface DpContainerChainGenesisDataContainerChainGenesisData extends Struct {
        readonly storage: Vec<DpContainerChainGenesisDataContainerChainGenesisDataItem>;
        readonly name: Bytes;
        readonly id: Bytes;
        readonly forkId: Option<Bytes>;
        readonly extensions: Bytes;
        readonly properties: DpContainerChainGenesisDataProperties;
    }

    /** @name DpContainerChainGenesisDataContainerChainGenesisDataItem (294) */
    interface DpContainerChainGenesisDataContainerChainGenesisDataItem extends Struct {
        readonly key: Bytes;
        readonly value: Bytes;
    }

    /** @name DpContainerChainGenesisDataProperties (296) */
    interface DpContainerChainGenesisDataProperties extends Struct {
        readonly tokenMetadata: DpContainerChainGenesisDataTokenMetadata;
        readonly isEthereum: bool;
    }

    /** @name DpContainerChainGenesisDataTokenMetadata (297) */
    interface DpContainerChainGenesisDataTokenMetadata extends Struct {
        readonly tokenSymbol: Bytes;
        readonly ss58Format: u32;
        readonly tokenDecimals: u32;
    }

    /** @name TpTraitsSlotFrequency (300) */
    interface TpTraitsSlotFrequency extends Struct {
        readonly min: u32;
        readonly max: u32;
    }

    /** @name TpTraitsParathreadParams (302) */
    interface TpTraitsParathreadParams extends Struct {
        readonly slotFrequency: TpTraitsSlotFrequency;
    }

    /** @name PalletConfigurationCall (303) */
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
            | "SetBypassConsistencyCheck";
    }

    /** @name PalletCollatorAssignmentCall (305) */
    type PalletCollatorAssignmentCall = Null;

    /** @name PalletAuthorNotingCall (306) */
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

    /** @name TpAuthorNotingInherentOwnParachainInherentData (307) */
    interface TpAuthorNotingInherentOwnParachainInherentData extends Struct {
        readonly relayStorageProof: SpTrieStorageProof;
    }

    /** @name PalletAuthorityAssignmentCall (308) */
    type PalletAuthorityAssignmentCall = Null;

    /** @name PalletServicesPaymentCall (309) */
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

    /** @name PalletDataPreserversCall (310) */
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
            readonly assignerParam: DanceboxRuntimePreserversAssignementPaymentExtra;
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
            readonly assignmentWitness: DanceboxRuntimePreserversAssignementPaymentWitness;
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

    /** @name PalletDataPreserversProfile (311) */
    interface PalletDataPreserversProfile extends Struct {
        readonly url: Bytes;
        readonly paraIds: PalletDataPreserversParaIdsFilter;
        readonly mode: PalletDataPreserversProfileMode;
        readonly assignmentRequest: DanceboxRuntimePreserversAssignementPaymentRequest;
    }

    /** @name PalletDataPreserversParaIdsFilter (313) */
    interface PalletDataPreserversParaIdsFilter extends Enum {
        readonly isAnyParaId: boolean;
        readonly isWhitelist: boolean;
        readonly asWhitelist: BTreeSet<u32>;
        readonly isBlacklist: boolean;
        readonly asBlacklist: BTreeSet<u32>;
        readonly type: "AnyParaId" | "Whitelist" | "Blacklist";
    }

    /** @name PalletDataPreserversProfileMode (316) */
    interface PalletDataPreserversProfileMode extends Enum {
        readonly isBootnode: boolean;
        readonly isRpc: boolean;
        readonly asRpc: {
            readonly supportsEthereumRpcs: bool;
        } & Struct;
        readonly type: "Bootnode" | "Rpc";
    }

    /** @name DanceboxRuntimePreserversAssignementPaymentRequest (317) */
    interface DanceboxRuntimePreserversAssignementPaymentRequest extends Enum {
        readonly isFree: boolean;
        readonly type: "Free";
    }

    /** @name DanceboxRuntimePreserversAssignementPaymentExtra (318) */
    interface DanceboxRuntimePreserversAssignementPaymentExtra extends Enum {
        readonly isFree: boolean;
        readonly type: "Free";
    }

    /** @name DanceboxRuntimePreserversAssignementPaymentWitness (319) */
    interface DanceboxRuntimePreserversAssignementPaymentWitness extends Enum {
        readonly isFree: boolean;
        readonly type: "Free";
    }

    /** @name PalletInvulnerablesCall (320) */
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

    /** @name PalletSessionCall (321) */
    interface PalletSessionCall extends Enum {
        readonly isSetKeys: boolean;
        readonly asSetKeys: {
            readonly keys_: DanceboxRuntimeSessionKeys;
            readonly proof: Bytes;
        } & Struct;
        readonly isPurgeKeys: boolean;
        readonly type: "SetKeys" | "PurgeKeys";
    }

    /** @name DanceboxRuntimeSessionKeys (322) */
    interface DanceboxRuntimeSessionKeys extends Struct {
        readonly nimbus: NimbusPrimitivesNimbusCryptoPublic;
    }

    /** @name NimbusPrimitivesNimbusCryptoPublic (323) */
    interface NimbusPrimitivesNimbusCryptoPublic extends U8aFixed {}

    /** @name PalletAuthorInherentCall (324) */
    interface PalletAuthorInherentCall extends Enum {
        readonly isKickOffAuthorshipValidation: boolean;
        readonly type: "KickOffAuthorshipValidation";
    }

    /** @name PalletPooledStakingCall (325) */
    interface PalletPooledStakingCall extends Enum {
        readonly isRebalanceHold: boolean;
        readonly asRebalanceHold: {
            readonly candidate: AccountId32;
            readonly delegator: AccountId32;
            readonly pool: PalletPooledStakingAllTargetPool;
        } & Struct;
        readonly isRequestDelegate: boolean;
        readonly asRequestDelegate: {
            readonly candidate: AccountId32;
            readonly pool: PalletPooledStakingTargetPool;
            readonly stake: u128;
        } & Struct;
        readonly isExecutePendingOperations: boolean;
        readonly asExecutePendingOperations: {
            readonly operations: Vec<PalletPooledStakingPendingOperationQuery>;
        } & Struct;
        readonly isRequestUndelegate: boolean;
        readonly asRequestUndelegate: {
            readonly candidate: AccountId32;
            readonly pool: PalletPooledStakingTargetPool;
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
            readonly sourcePool: PalletPooledStakingTargetPool;
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

    /** @name PalletPooledStakingAllTargetPool (326) */
    interface PalletPooledStakingAllTargetPool extends Enum {
        readonly isJoining: boolean;
        readonly isAutoCompounding: boolean;
        readonly isManualRewards: boolean;
        readonly isLeaving: boolean;
        readonly type: "Joining" | "AutoCompounding" | "ManualRewards" | "Leaving";
    }

    /** @name PalletPooledStakingPendingOperationQuery (328) */
    interface PalletPooledStakingPendingOperationQuery extends Struct {
        readonly delegator: AccountId32;
        readonly operation: PalletPooledStakingPendingOperationKey;
    }

    /** @name PalletPooledStakingPendingOperationKey (329) */
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

    /** @name PalletPooledStakingSharesOrStake (330) */
    interface PalletPooledStakingSharesOrStake extends Enum {
        readonly isShares: boolean;
        readonly asShares: u128;
        readonly isStake: boolean;
        readonly asStake: u128;
        readonly type: "Shares" | "Stake";
    }

    /** @name PalletTreasuryCall (333) */
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

    /** @name CumulusPalletXcmpQueueCall (334) */
    interface CumulusPalletXcmpQueueCall extends Enum {
        readonly isSuspendXcmExecution: boolean;
        readonly isResumeXcmExecution: boolean;
        readonly isUpdateSuspendThreshold: boolean;
        readonly asUpdateSuspendThreshold: {
            readonly new_: u32;
        } & Struct;
        readonly isUpdateDropThreshold: boolean;
        readonly asUpdateDropThreshold: {
            readonly new_: u32;
        } & Struct;
        readonly isUpdateResumeThreshold: boolean;
        readonly asUpdateResumeThreshold: {
            readonly new_: u32;
        } & Struct;
        readonly type:
            | "SuspendXcmExecution"
            | "ResumeXcmExecution"
            | "UpdateSuspendThreshold"
            | "UpdateDropThreshold"
            | "UpdateResumeThreshold";
    }

    /** @name PalletXcmCall (335) */
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
            readonly location: StagingXcmV4Location;
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
            | "TransferAssetsUsingTypeAndThen";
    }

    /** @name XcmVersionedXcm (336) */
    interface XcmVersionedXcm extends Enum {
        readonly isV2: boolean;
        readonly asV2: XcmV2Xcm;
        readonly isV3: boolean;
        readonly asV3: XcmV3Xcm;
        readonly isV4: boolean;
        readonly asV4: StagingXcmV4Xcm;
        readonly type: "V2" | "V3" | "V4";
    }

    /** @name XcmV2Xcm (337) */
    interface XcmV2Xcm extends Vec<XcmV2Instruction> {}

    /** @name XcmV2Instruction (339) */
    interface XcmV2Instruction extends Enum {
        readonly isWithdrawAsset: boolean;
        readonly asWithdrawAsset: XcmV2MultiassetMultiAssets;
        readonly isReserveAssetDeposited: boolean;
        readonly asReserveAssetDeposited: XcmV2MultiassetMultiAssets;
        readonly isReceiveTeleportedAsset: boolean;
        readonly asReceiveTeleportedAsset: XcmV2MultiassetMultiAssets;
        readonly isQueryResponse: boolean;
        readonly asQueryResponse: {
            readonly queryId: Compact<u64>;
            readonly response: XcmV2Response;
            readonly maxWeight: Compact<u64>;
        } & Struct;
        readonly isTransferAsset: boolean;
        readonly asTransferAsset: {
            readonly assets: XcmV2MultiassetMultiAssets;
            readonly beneficiary: XcmV2MultiLocation;
        } & Struct;
        readonly isTransferReserveAsset: boolean;
        readonly asTransferReserveAsset: {
            readonly assets: XcmV2MultiassetMultiAssets;
            readonly dest: XcmV2MultiLocation;
            readonly xcm: XcmV2Xcm;
        } & Struct;
        readonly isTransact: boolean;
        readonly asTransact: {
            readonly originType: XcmV2OriginKind;
            readonly requireWeightAtMost: Compact<u64>;
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
        readonly asDescendOrigin: XcmV2MultilocationJunctions;
        readonly isReportError: boolean;
        readonly asReportError: {
            readonly queryId: Compact<u64>;
            readonly dest: XcmV2MultiLocation;
            readonly maxResponseWeight: Compact<u64>;
        } & Struct;
        readonly isDepositAsset: boolean;
        readonly asDepositAsset: {
            readonly assets: XcmV2MultiassetMultiAssetFilter;
            readonly maxAssets: Compact<u32>;
            readonly beneficiary: XcmV2MultiLocation;
        } & Struct;
        readonly isDepositReserveAsset: boolean;
        readonly asDepositReserveAsset: {
            readonly assets: XcmV2MultiassetMultiAssetFilter;
            readonly maxAssets: Compact<u32>;
            readonly dest: XcmV2MultiLocation;
            readonly xcm: XcmV2Xcm;
        } & Struct;
        readonly isExchangeAsset: boolean;
        readonly asExchangeAsset: {
            readonly give: XcmV2MultiassetMultiAssetFilter;
            readonly receive: XcmV2MultiassetMultiAssets;
        } & Struct;
        readonly isInitiateReserveWithdraw: boolean;
        readonly asInitiateReserveWithdraw: {
            readonly assets: XcmV2MultiassetMultiAssetFilter;
            readonly reserve: XcmV2MultiLocation;
            readonly xcm: XcmV2Xcm;
        } & Struct;
        readonly isInitiateTeleport: boolean;
        readonly asInitiateTeleport: {
            readonly assets: XcmV2MultiassetMultiAssetFilter;
            readonly dest: XcmV2MultiLocation;
            readonly xcm: XcmV2Xcm;
        } & Struct;
        readonly isQueryHolding: boolean;
        readonly asQueryHolding: {
            readonly queryId: Compact<u64>;
            readonly dest: XcmV2MultiLocation;
            readonly assets: XcmV2MultiassetMultiAssetFilter;
            readonly maxResponseWeight: Compact<u64>;
        } & Struct;
        readonly isBuyExecution: boolean;
        readonly asBuyExecution: {
            readonly fees: XcmV2MultiAsset;
            readonly weightLimit: XcmV2WeightLimit;
        } & Struct;
        readonly isRefundSurplus: boolean;
        readonly isSetErrorHandler: boolean;
        readonly asSetErrorHandler: XcmV2Xcm;
        readonly isSetAppendix: boolean;
        readonly asSetAppendix: XcmV2Xcm;
        readonly isClearError: boolean;
        readonly isClaimAsset: boolean;
        readonly asClaimAsset: {
            readonly assets: XcmV2MultiassetMultiAssets;
            readonly ticket: XcmV2MultiLocation;
        } & Struct;
        readonly isTrap: boolean;
        readonly asTrap: Compact<u64>;
        readonly isSubscribeVersion: boolean;
        readonly asSubscribeVersion: {
            readonly queryId: Compact<u64>;
            readonly maxResponseWeight: Compact<u64>;
        } & Struct;
        readonly isUnsubscribeVersion: boolean;
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
            | "QueryHolding"
            | "BuyExecution"
            | "RefundSurplus"
            | "SetErrorHandler"
            | "SetAppendix"
            | "ClearError"
            | "ClaimAsset"
            | "Trap"
            | "SubscribeVersion"
            | "UnsubscribeVersion";
    }

    /** @name XcmV2Response (340) */
    interface XcmV2Response extends Enum {
        readonly isNull: boolean;
        readonly isAssets: boolean;
        readonly asAssets: XcmV2MultiassetMultiAssets;
        readonly isExecutionResult: boolean;
        readonly asExecutionResult: Option<ITuple<[u32, XcmV2TraitsError]>>;
        readonly isVersion: boolean;
        readonly asVersion: u32;
        readonly type: "Null" | "Assets" | "ExecutionResult" | "Version";
    }

    /** @name XcmV2TraitsError (343) */
    interface XcmV2TraitsError extends Enum {
        readonly isOverflow: boolean;
        readonly isUnimplemented: boolean;
        readonly isUntrustedReserveLocation: boolean;
        readonly isUntrustedTeleportLocation: boolean;
        readonly isMultiLocationFull: boolean;
        readonly isMultiLocationNotInvertible: boolean;
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
        readonly isUnhandledXcmVersion: boolean;
        readonly isWeightLimitReached: boolean;
        readonly asWeightLimitReached: u64;
        readonly isBarrier: boolean;
        readonly isWeightNotComputable: boolean;
        readonly type:
            | "Overflow"
            | "Unimplemented"
            | "UntrustedReserveLocation"
            | "UntrustedTeleportLocation"
            | "MultiLocationFull"
            | "MultiLocationNotInvertible"
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
            | "UnhandledXcmVersion"
            | "WeightLimitReached"
            | "Barrier"
            | "WeightNotComputable";
    }

    /** @name XcmV2OriginKind (344) */
    interface XcmV2OriginKind extends Enum {
        readonly isNative: boolean;
        readonly isSovereignAccount: boolean;
        readonly isSuperuser: boolean;
        readonly isXcm: boolean;
        readonly type: "Native" | "SovereignAccount" | "Superuser" | "Xcm";
    }

    /** @name XcmV2MultiassetMultiAssetFilter (345) */
    interface XcmV2MultiassetMultiAssetFilter extends Enum {
        readonly isDefinite: boolean;
        readonly asDefinite: XcmV2MultiassetMultiAssets;
        readonly isWild: boolean;
        readonly asWild: XcmV2MultiassetWildMultiAsset;
        readonly type: "Definite" | "Wild";
    }

    /** @name XcmV2MultiassetWildMultiAsset (346) */
    interface XcmV2MultiassetWildMultiAsset extends Enum {
        readonly isAll: boolean;
        readonly isAllOf: boolean;
        readonly asAllOf: {
            readonly id: XcmV2MultiassetAssetId;
            readonly fun: XcmV2MultiassetWildFungibility;
        } & Struct;
        readonly type: "All" | "AllOf";
    }

    /** @name XcmV2MultiassetWildFungibility (347) */
    interface XcmV2MultiassetWildFungibility extends Enum {
        readonly isFungible: boolean;
        readonly isNonFungible: boolean;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name XcmV2WeightLimit (348) */
    interface XcmV2WeightLimit extends Enum {
        readonly isUnlimited: boolean;
        readonly isLimited: boolean;
        readonly asLimited: Compact<u64>;
        readonly type: "Unlimited" | "Limited";
    }

    /** @name XcmV3Xcm (349) */
    interface XcmV3Xcm extends Vec<XcmV3Instruction> {}

    /** @name XcmV3Instruction (351) */
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

    /** @name XcmV3Response (352) */
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

    /** @name XcmV3PalletInfo (354) */
    interface XcmV3PalletInfo extends Struct {
        readonly index: Compact<u32>;
        readonly name: Bytes;
        readonly moduleName: Bytes;
        readonly major: Compact<u32>;
        readonly minor: Compact<u32>;
        readonly patch: Compact<u32>;
    }

    /** @name XcmV3QueryResponseInfo (358) */
    interface XcmV3QueryResponseInfo extends Struct {
        readonly destination: StagingXcmV3MultiLocation;
        readonly queryId: Compact<u64>;
        readonly maxWeight: SpWeightsWeightV2Weight;
    }

    /** @name XcmV3MultiassetMultiAssetFilter (359) */
    interface XcmV3MultiassetMultiAssetFilter extends Enum {
        readonly isDefinite: boolean;
        readonly asDefinite: XcmV3MultiassetMultiAssets;
        readonly isWild: boolean;
        readonly asWild: XcmV3MultiassetWildMultiAsset;
        readonly type: "Definite" | "Wild";
    }

    /** @name XcmV3MultiassetWildMultiAsset (360) */
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

    /** @name XcmV3MultiassetWildFungibility (361) */
    interface XcmV3MultiassetWildFungibility extends Enum {
        readonly isFungible: boolean;
        readonly isNonFungible: boolean;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name StagingXcmExecutorAssetTransferTransferType (373) */
    interface StagingXcmExecutorAssetTransferTransferType extends Enum {
        readonly isTeleport: boolean;
        readonly isLocalReserve: boolean;
        readonly isDestinationReserve: boolean;
        readonly isRemoteReserve: boolean;
        readonly asRemoteReserve: XcmVersionedLocation;
        readonly type: "Teleport" | "LocalReserve" | "DestinationReserve" | "RemoteReserve";
    }

    /** @name XcmVersionedAssetId (374) */
    interface XcmVersionedAssetId extends Enum {
        readonly isV3: boolean;
        readonly asV3: XcmV3MultiassetAssetId;
        readonly isV4: boolean;
        readonly asV4: StagingXcmV4AssetAssetId;
        readonly type: "V3" | "V4";
    }

    /** @name PalletAssetsCall (375) */
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
            | "Block";
    }

    /** @name PalletForeignAssetCreatorCall (376) */
    interface PalletForeignAssetCreatorCall extends Enum {
        readonly isCreateForeignAsset: boolean;
        readonly asCreateForeignAsset: {
            readonly foreignAsset: StagingXcmV4Location;
            readonly assetId: u16;
            readonly admin: AccountId32;
            readonly isSufficient: bool;
            readonly minBalance: u128;
        } & Struct;
        readonly isChangeExistingAssetType: boolean;
        readonly asChangeExistingAssetType: {
            readonly assetId: u16;
            readonly newForeignAsset: StagingXcmV4Location;
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

    /** @name PalletAssetRateCall (377) */
    interface PalletAssetRateCall extends Enum {
        readonly isCreate: boolean;
        readonly asCreate: {
            readonly assetKind: u16;
            readonly rate: u128;
        } & Struct;
        readonly isUpdate: boolean;
        readonly asUpdate: {
            readonly assetKind: u16;
            readonly rate: u128;
        } & Struct;
        readonly isRemove: boolean;
        readonly asRemove: {
            readonly assetKind: u16;
        } & Struct;
        readonly type: "Create" | "Update" | "Remove";
    }

    /** @name PalletMessageQueueCall (378) */
    interface PalletMessageQueueCall extends Enum {
        readonly isReapPage: boolean;
        readonly asReapPage: {
            readonly messageOrigin: CumulusPrimitivesCoreAggregateMessageOrigin;
            readonly pageIndex: u32;
        } & Struct;
        readonly isExecuteOverweight: boolean;
        readonly asExecuteOverweight: {
            readonly messageOrigin: CumulusPrimitivesCoreAggregateMessageOrigin;
            readonly page: u32;
            readonly index: u32;
            readonly weightLimit: SpWeightsWeightV2Weight;
        } & Struct;
        readonly type: "ReapPage" | "ExecuteOverweight";
    }

    /** @name PalletXcmCoreBuyerCall (379) */
    interface PalletXcmCoreBuyerCall extends Enum {
        readonly isBuyCore: boolean;
        readonly asBuyCore: {
            readonly paraId: u32;
            readonly proof: TpXcmCoreBuyerBuyCoreCollatorProof;
        } & Struct;
        readonly isForceBuyCore: boolean;
        readonly asForceBuyCore: {
            readonly paraId: u32;
        } & Struct;
        readonly isSetRelayXcmWeightConfig: boolean;
        readonly asSetRelayXcmWeightConfig: {
            readonly xcmWeights: Option<PalletXcmCoreBuyerRelayXcmWeightConfigInner>;
        } & Struct;
        readonly isSetRelayChain: boolean;
        readonly asSetRelayChain: {
            readonly relayChain: Option<DanceboxRuntimeXcmConfigRelayChain>;
        } & Struct;
        readonly isQueryResponse: boolean;
        readonly asQueryResponse: {
            readonly queryId: u64;
            readonly response: StagingXcmV4Response;
        } & Struct;
        readonly isCleanUpExpiredPendingBlocks: boolean;
        readonly asCleanUpExpiredPendingBlocks: {
            readonly expiredPendingBlocksParaId: Vec<u32>;
        } & Struct;
        readonly isCleanUpExpiredInFlightOrders: boolean;
        readonly asCleanUpExpiredInFlightOrders: {
            readonly expiredInFlightOrders: Vec<u32>;
        } & Struct;
        readonly type:
            | "BuyCore"
            | "ForceBuyCore"
            | "SetRelayXcmWeightConfig"
            | "SetRelayChain"
            | "QueryResponse"
            | "CleanUpExpiredPendingBlocks"
            | "CleanUpExpiredInFlightOrders";
    }

    /** @name TpXcmCoreBuyerBuyCoreCollatorProof (380) */
    interface TpXcmCoreBuyerBuyCoreCollatorProof extends Struct {
        readonly nonce: u64;
        readonly publicKey: NimbusPrimitivesNimbusCryptoPublic;
        readonly signature: NimbusPrimitivesNimbusCryptoSignature;
    }

    /** @name NimbusPrimitivesNimbusCryptoSignature (381) */
    interface NimbusPrimitivesNimbusCryptoSignature extends U8aFixed {}

    /** @name PalletXcmCoreBuyerRelayXcmWeightConfigInner (383) */
    interface PalletXcmCoreBuyerRelayXcmWeightConfigInner extends Struct {
        readonly buyExecutionCost: u128;
        readonly weightAtMost: SpWeightsWeightV2Weight;
    }

    /** @name DanceboxRuntimeXcmConfigRelayChain (385) */
    interface DanceboxRuntimeXcmConfigRelayChain extends Enum {
        readonly isWestend: boolean;
        readonly isRococo: boolean;
        readonly type: "Westend" | "Rococo";
    }

    /** @name PalletRootTestingCall (386) */
    interface PalletRootTestingCall extends Enum {
        readonly isFillBlock: boolean;
        readonly asFillBlock: {
            readonly ratio: Perbill;
        } & Struct;
        readonly isTriggerDefensive: boolean;
        readonly type: "FillBlock" | "TriggerDefensive";
    }

    /** @name PalletSudoError (387) */
    interface PalletSudoError extends Enum {
        readonly isRequireSudo: boolean;
        readonly type: "RequireSudo";
    }

    /** @name PalletUtilityError (388) */
    interface PalletUtilityError extends Enum {
        readonly isTooManyCalls: boolean;
        readonly type: "TooManyCalls";
    }

    /** @name PalletProxyProxyDefinition (391) */
    interface PalletProxyProxyDefinition extends Struct {
        readonly delegate: AccountId32;
        readonly proxyType: DanceboxRuntimeProxyType;
        readonly delay: u32;
    }

    /** @name PalletProxyAnnouncement (395) */
    interface PalletProxyAnnouncement extends Struct {
        readonly real: AccountId32;
        readonly callHash: H256;
        readonly height: u32;
    }

    /** @name PalletProxyError (397) */
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

    /** @name PalletMigrationsError (398) */
    interface PalletMigrationsError extends Enum {
        readonly isPreimageMissing: boolean;
        readonly isWrongUpperBound: boolean;
        readonly isPreimageIsTooBig: boolean;
        readonly isPreimageAlreadyExists: boolean;
        readonly type: "PreimageMissing" | "WrongUpperBound" | "PreimageIsTooBig" | "PreimageAlreadyExists";
    }

    /** @name PalletMaintenanceModeError (399) */
    interface PalletMaintenanceModeError extends Enum {
        readonly isAlreadyInMaintenanceMode: boolean;
        readonly isNotInMaintenanceMode: boolean;
        readonly type: "AlreadyInMaintenanceMode" | "NotInMaintenanceMode";
    }

    /** @name PalletTxPauseError (400) */
    interface PalletTxPauseError extends Enum {
        readonly isIsPaused: boolean;
        readonly isIsUnpaused: boolean;
        readonly isUnpausable: boolean;
        readonly isNotFound: boolean;
        readonly type: "IsPaused" | "IsUnpaused" | "Unpausable" | "NotFound";
    }

    /** @name PalletBalancesBalanceLock (402) */
    interface PalletBalancesBalanceLock extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
        readonly reasons: PalletBalancesReasons;
    }

    /** @name PalletBalancesReasons (403) */
    interface PalletBalancesReasons extends Enum {
        readonly isFee: boolean;
        readonly isMisc: boolean;
        readonly isAll: boolean;
        readonly type: "Fee" | "Misc" | "All";
    }

    /** @name PalletBalancesReserveData (406) */
    interface PalletBalancesReserveData extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
    }

    /** @name FrameSupportTokensMiscIdAmountRuntimeHoldReason (409) */
    interface FrameSupportTokensMiscIdAmountRuntimeHoldReason extends Struct {
        readonly id: DanceboxRuntimeRuntimeHoldReason;
        readonly amount: u128;
    }

    /** @name DanceboxRuntimeRuntimeHoldReason (410) */
    interface DanceboxRuntimeRuntimeHoldReason extends Enum {
        readonly isStreamPayment: boolean;
        readonly asStreamPayment: PalletStreamPaymentHoldReason;
        readonly isRegistrar: boolean;
        readonly asRegistrar: PalletRegistrarHoldReason;
        readonly isDataPreservers: boolean;
        readonly asDataPreservers: PalletDataPreserversHoldReason;
        readonly isPooledStaking: boolean;
        readonly asPooledStaking: PalletPooledStakingHoldReason;
        readonly type: "StreamPayment" | "Registrar" | "DataPreservers" | "PooledStaking";
    }

    /** @name PalletStreamPaymentHoldReason (411) */
    interface PalletStreamPaymentHoldReason extends Enum {
        readonly isStreamPayment: boolean;
        readonly isStreamOpened: boolean;
        readonly type: "StreamPayment" | "StreamOpened";
    }

    /** @name PalletRegistrarHoldReason (412) */
    interface PalletRegistrarHoldReason extends Enum {
        readonly isRegistrarDeposit: boolean;
        readonly type: "RegistrarDeposit";
    }

    /** @name PalletDataPreserversHoldReason (413) */
    interface PalletDataPreserversHoldReason extends Enum {
        readonly isProfileDeposit: boolean;
        readonly type: "ProfileDeposit";
    }

    /** @name PalletPooledStakingHoldReason (414) */
    interface PalletPooledStakingHoldReason extends Enum {
        readonly isPooledStake: boolean;
        readonly type: "PooledStake";
    }

    /** @name FrameSupportTokensMiscIdAmountRuntimeFreezeReason (417) */
    interface FrameSupportTokensMiscIdAmountRuntimeFreezeReason extends Struct {
        readonly id: DanceboxRuntimeRuntimeFreezeReason;
        readonly amount: u128;
    }

    /** @name DanceboxRuntimeRuntimeFreezeReason (418) */
    interface DanceboxRuntimeRuntimeFreezeReason extends Enum {
        readonly isStreamPayment: boolean;
        readonly asStreamPayment: PalletStreamPaymentFreezeReason;
        readonly type: "StreamPayment";
    }

    /** @name PalletStreamPaymentFreezeReason (419) */
    interface PalletStreamPaymentFreezeReason extends Enum {
        readonly isStreamPayment: boolean;
        readonly type: "StreamPayment";
    }

    /** @name PalletBalancesError (421) */
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

    /** @name PalletTransactionPaymentReleases (422) */
    interface PalletTransactionPaymentReleases extends Enum {
        readonly isV1Ancient: boolean;
        readonly isV2: boolean;
        readonly type: "V1Ancient" | "V2";
    }

    /** @name PalletStreamPaymentStream (423) */
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

    /** @name PalletStreamPaymentChangeRequest (425) */
    interface PalletStreamPaymentChangeRequest extends Struct {
        readonly requester: PalletStreamPaymentParty;
        readonly kind: PalletStreamPaymentChangeKind;
        readonly newConfig: PalletStreamPaymentStreamConfig;
        readonly depositChange: Option<PalletStreamPaymentDepositChange>;
    }

    /** @name PalletStreamPaymentError (427) */
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

    /** @name PalletIdentityRegistration (429) */
    interface PalletIdentityRegistration extends Struct {
        readonly judgements: Vec<ITuple<[u32, PalletIdentityJudgement]>>;
        readonly deposit: u128;
        readonly info: PalletIdentityLegacyIdentityInfo;
    }

    /** @name PalletIdentityRegistrarInfo (438) */
    interface PalletIdentityRegistrarInfo extends Struct {
        readonly account: AccountId32;
        readonly fee: u128;
        readonly fields: u64;
    }

    /** @name PalletIdentityAuthorityProperties (440) */
    interface PalletIdentityAuthorityProperties extends Struct {
        readonly suffix: Bytes;
        readonly allocation: u32;
    }

    /** @name PalletIdentityError (443) */
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
            | "NotExpired";
    }

    /** @name PalletMultisigMultisig (445) */
    interface PalletMultisigMultisig extends Struct {
        readonly when: PalletMultisigTimepoint;
        readonly deposit: u128;
        readonly depositor: AccountId32;
        readonly approvals: Vec<AccountId32>;
    }

    /** @name PalletMultisigError (447) */
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

    /** @name PalletRegistrarDepositInfo (456) */
    interface PalletRegistrarDepositInfo extends Struct {
        readonly creator: AccountId32;
        readonly deposit: u128;
    }

    /** @name PalletRegistrarError (457) */
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

    /** @name PalletConfigurationHostConfiguration (458) */
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
    }

    /** @name PalletConfigurationError (462) */
    interface PalletConfigurationError extends Enum {
        readonly isInvalidNewValue: boolean;
        readonly type: "InvalidNewValue";
    }

    /** @name DpCollatorAssignmentAssignedCollatorsAccountId32 (463) */
    interface DpCollatorAssignmentAssignedCollatorsAccountId32 extends Struct {
        readonly orchestratorChain: Vec<AccountId32>;
        readonly containerChains: BTreeMap<u32, Vec<AccountId32>>;
    }

    /** @name TpTraitsContainerChainBlockInfo (468) */
    interface TpTraitsContainerChainBlockInfo extends Struct {
        readonly blockNumber: u32;
        readonly author: AccountId32;
        readonly latestSlotNumber: u64;
    }

    /** @name PalletAuthorNotingError (469) */
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

    /** @name DpCollatorAssignmentAssignedCollatorsPublic (470) */
    interface DpCollatorAssignmentAssignedCollatorsPublic extends Struct {
        readonly orchestratorChain: Vec<NimbusPrimitivesNimbusCryptoPublic>;
        readonly containerChains: BTreeMap<u32, Vec<NimbusPrimitivesNimbusCryptoPublic>>;
    }

    /** @name PalletServicesPaymentError (475) */
    interface PalletServicesPaymentError extends Enum {
        readonly isInsufficientFundsToPurchaseCredits: boolean;
        readonly isInsufficientCredits: boolean;
        readonly isCreditPriceTooExpensive: boolean;
        readonly type: "InsufficientFundsToPurchaseCredits" | "InsufficientCredits" | "CreditPriceTooExpensive";
    }

    /** @name PalletDataPreserversRegisteredProfile (476) */
    interface PalletDataPreserversRegisteredProfile extends Struct {
        readonly account: AccountId32;
        readonly deposit: u128;
        readonly profile: PalletDataPreserversProfile;
        readonly assignment: Option<ITuple<[u32, DanceboxRuntimePreserversAssignementPaymentWitness]>>;
    }

    /** @name PalletDataPreserversError (482) */
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

    /** @name PalletInvulnerablesError (484) */
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

    /** @name SpCoreCryptoKeyTypeId (489) */
    interface SpCoreCryptoKeyTypeId extends U8aFixed {}

    /** @name PalletSessionError (490) */
    interface PalletSessionError extends Enum {
        readonly isInvalidProof: boolean;
        readonly isNoAssociatedValidatorId: boolean;
        readonly isDuplicatedKey: boolean;
        readonly isNoKeys: boolean;
        readonly isNoAccount: boolean;
        readonly type: "InvalidProof" | "NoAssociatedValidatorId" | "DuplicatedKey" | "NoKeys" | "NoAccount";
    }

    /** @name PalletAuthorInherentError (494) */
    interface PalletAuthorInherentError extends Enum {
        readonly isAuthorAlreadySet: boolean;
        readonly isNoAccountId: boolean;
        readonly isCannotBeAuthor: boolean;
        readonly type: "AuthorAlreadySet" | "NoAccountId" | "CannotBeAuthor";
    }

    /** @name PalletPooledStakingCandidateEligibleCandidate (496) */
    interface PalletPooledStakingCandidateEligibleCandidate extends Struct {
        readonly candidate: AccountId32;
        readonly stake: u128;
    }

    /** @name PalletPooledStakingPoolsKey (499) */
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

    /** @name PalletPooledStakingError (501) */
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
            | "SwapResultsInZeroShares";
    }

    /** @name PalletInflationRewardsChainsToRewardValue (502) */
    interface PalletInflationRewardsChainsToRewardValue extends Struct {
        readonly paraIds: Vec<u32>;
        readonly rewardsPerChain: u128;
    }

    /** @name PalletTreasuryProposal (503) */
    interface PalletTreasuryProposal extends Struct {
        readonly proposer: AccountId32;
        readonly value: u128;
        readonly beneficiary: AccountId32;
        readonly bond: u128;
    }

    /** @name PalletTreasurySpendStatus (505) */
    interface PalletTreasurySpendStatus extends Struct {
        readonly assetKind: Null;
        readonly amount: u128;
        readonly beneficiary: AccountId32;
        readonly validFrom: u32;
        readonly expireAt: u32;
        readonly status: PalletTreasuryPaymentState;
    }

    /** @name PalletTreasuryPaymentState (506) */
    interface PalletTreasuryPaymentState extends Enum {
        readonly isPending: boolean;
        readonly isAttempted: boolean;
        readonly asAttempted: {
            readonly id: Null;
        } & Struct;
        readonly isFailed: boolean;
        readonly type: "Pending" | "Attempted" | "Failed";
    }

    /** @name FrameSupportPalletId (508) */
    interface FrameSupportPalletId extends U8aFixed {}

    /** @name PalletTreasuryError (509) */
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

    /** @name CumulusPalletXcmpQueueOutboundChannelDetails (512) */
    interface CumulusPalletXcmpQueueOutboundChannelDetails extends Struct {
        readonly recipient: u32;
        readonly state: CumulusPalletXcmpQueueOutboundState;
        readonly signalsExist: bool;
        readonly firstIndex: u16;
        readonly lastIndex: u16;
    }

    /** @name CumulusPalletXcmpQueueOutboundState (513) */
    interface CumulusPalletXcmpQueueOutboundState extends Enum {
        readonly isOk: boolean;
        readonly isSuspended: boolean;
        readonly type: "Ok" | "Suspended";
    }

    /** @name CumulusPalletXcmpQueueQueueConfigData (517) */
    interface CumulusPalletXcmpQueueQueueConfigData extends Struct {
        readonly suspendThreshold: u32;
        readonly dropThreshold: u32;
        readonly resumeThreshold: u32;
    }

    /** @name CumulusPalletXcmpQueueError (518) */
    interface CumulusPalletXcmpQueueError extends Enum {
        readonly isBadQueueConfig: boolean;
        readonly isAlreadySuspended: boolean;
        readonly isAlreadyResumed: boolean;
        readonly isTooManyActiveOutboundChannels: boolean;
        readonly isTooBig: boolean;
        readonly type:
            | "BadQueueConfig"
            | "AlreadySuspended"
            | "AlreadyResumed"
            | "TooManyActiveOutboundChannels"
            | "TooBig";
    }

    /** @name PalletXcmQueryStatus (519) */
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

    /** @name XcmVersionedResponse (523) */
    interface XcmVersionedResponse extends Enum {
        readonly isV2: boolean;
        readonly asV2: XcmV2Response;
        readonly isV3: boolean;
        readonly asV3: XcmV3Response;
        readonly isV4: boolean;
        readonly asV4: StagingXcmV4Response;
        readonly type: "V2" | "V3" | "V4";
    }

    /** @name PalletXcmVersionMigrationStage (529) */
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

    /** @name PalletXcmRemoteLockedFungibleRecord (531) */
    interface PalletXcmRemoteLockedFungibleRecord extends Struct {
        readonly amount: u128;
        readonly owner: XcmVersionedLocation;
        readonly locker: XcmVersionedLocation;
        readonly consumers: Vec<ITuple<[Null, u128]>>;
    }

    /** @name PalletXcmError (538) */
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
            | "LocalExecutionIncomplete";
    }

    /** @name PalletAssetsAssetDetails (539) */
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

    /** @name PalletAssetsAssetStatus (540) */
    interface PalletAssetsAssetStatus extends Enum {
        readonly isLive: boolean;
        readonly isFrozen: boolean;
        readonly isDestroying: boolean;
        readonly type: "Live" | "Frozen" | "Destroying";
    }

    /** @name PalletAssetsAssetAccount (542) */
    interface PalletAssetsAssetAccount extends Struct {
        readonly balance: u128;
        readonly status: PalletAssetsAccountStatus;
        readonly reason: PalletAssetsExistenceReason;
        readonly extra: Null;
    }

    /** @name PalletAssetsAccountStatus (543) */
    interface PalletAssetsAccountStatus extends Enum {
        readonly isLiquid: boolean;
        readonly isFrozen: boolean;
        readonly isBlocked: boolean;
        readonly type: "Liquid" | "Frozen" | "Blocked";
    }

    /** @name PalletAssetsExistenceReason (544) */
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

    /** @name PalletAssetsApproval (546) */
    interface PalletAssetsApproval extends Struct {
        readonly amount: u128;
        readonly deposit: u128;
    }

    /** @name PalletAssetsAssetMetadata (547) */
    interface PalletAssetsAssetMetadata extends Struct {
        readonly deposit: u128;
        readonly name: Bytes;
        readonly symbol: Bytes;
        readonly decimals: u8;
        readonly isFrozen: bool;
    }

    /** @name PalletAssetsError (549) */
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
            | "BadAssetId";
    }

    /** @name PalletForeignAssetCreatorError (550) */
    interface PalletForeignAssetCreatorError extends Enum {
        readonly isAssetAlreadyExists: boolean;
        readonly isAssetDoesNotExist: boolean;
        readonly type: "AssetAlreadyExists" | "AssetDoesNotExist";
    }

    /** @name PalletAssetRateError (551) */
    interface PalletAssetRateError extends Enum {
        readonly isUnknownAssetKind: boolean;
        readonly isAlreadyExists: boolean;
        readonly isOverflow: boolean;
        readonly type: "UnknownAssetKind" | "AlreadyExists" | "Overflow";
    }

    /** @name PalletMessageQueueBookState (552) */
    interface PalletMessageQueueBookState extends Struct {
        readonly begin: u32;
        readonly end: u32;
        readonly count: u32;
        readonly readyNeighbours: Option<PalletMessageQueueNeighbours>;
        readonly messageCount: u64;
        readonly size_: u64;
    }

    /** @name PalletMessageQueueNeighbours (554) */
    interface PalletMessageQueueNeighbours extends Struct {
        readonly prev: CumulusPrimitivesCoreAggregateMessageOrigin;
        readonly next: CumulusPrimitivesCoreAggregateMessageOrigin;
    }

    /** @name PalletMessageQueuePage (556) */
    interface PalletMessageQueuePage extends Struct {
        readonly remaining: u32;
        readonly remainingSize: u32;
        readonly firstIndex: u32;
        readonly first: u32;
        readonly last: u32;
        readonly heap: Bytes;
    }

    /** @name PalletMessageQueueError (558) */
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

    /** @name PalletXcmCoreBuyerInFlightCoreBuyingOrder (559) */
    interface PalletXcmCoreBuyerInFlightCoreBuyingOrder extends Struct {
        readonly paraId: u32;
        readonly queryId: u64;
        readonly ttl: u32;
    }

    /** @name PalletXcmCoreBuyerError (560) */
    interface PalletXcmCoreBuyerError extends Enum {
        readonly isInvalidProof: boolean;
        readonly isErrorValidatingXCM: boolean;
        readonly isErrorDeliveringXCM: boolean;
        readonly isOrderAlreadyExists: boolean;
        readonly isNotAParathread: boolean;
        readonly isInFlightLimitReached: boolean;
        readonly isNoAssignedCollators: boolean;
        readonly isCollatorNotAssigned: boolean;
        readonly isXcmWeightStorageNotSet: boolean;
        readonly isReanchorFailed: boolean;
        readonly isLocationInversionFailed: boolean;
        readonly isReportNotifyingSetupFailed: boolean;
        readonly isUnexpectedXCMResponse: boolean;
        readonly isBlockProductionPending: boolean;
        readonly isNotAllowedToProduceBlockRightNow: boolean;
        readonly isIncorrectCollatorSignatureNonce: boolean;
        readonly isInvalidCollatorSignature: boolean;
        readonly type:
            | "InvalidProof"
            | "ErrorValidatingXCM"
            | "ErrorDeliveringXCM"
            | "OrderAlreadyExists"
            | "NotAParathread"
            | "InFlightLimitReached"
            | "NoAssignedCollators"
            | "CollatorNotAssigned"
            | "XcmWeightStorageNotSet"
            | "ReanchorFailed"
            | "LocationInversionFailed"
            | "ReportNotifyingSetupFailed"
            | "UnexpectedXCMResponse"
            | "BlockProductionPending"
            | "NotAllowedToProduceBlockRightNow"
            | "IncorrectCollatorSignatureNonce"
            | "InvalidCollatorSignature";
    }

    /** @name FrameSystemExtensionsCheckNonZeroSender (565) */
    type FrameSystemExtensionsCheckNonZeroSender = Null;

    /** @name FrameSystemExtensionsCheckSpecVersion (566) */
    type FrameSystemExtensionsCheckSpecVersion = Null;

    /** @name FrameSystemExtensionsCheckTxVersion (567) */
    type FrameSystemExtensionsCheckTxVersion = Null;

    /** @name FrameSystemExtensionsCheckGenesis (568) */
    type FrameSystemExtensionsCheckGenesis = Null;

    /** @name FrameSystemExtensionsCheckNonce (571) */
    interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

    /** @name FrameSystemExtensionsCheckWeight (572) */
    type FrameSystemExtensionsCheckWeight = Null;

    /** @name PalletTransactionPaymentChargeTransactionPayment (573) */
    interface PalletTransactionPaymentChargeTransactionPayment extends Compact<u128> {}

    /** @name CumulusPrimitivesStorageWeightReclaimStorageWeightReclaim (574) */
    type CumulusPrimitivesStorageWeightReclaimStorageWeightReclaim = Null;

    /** @name DanceboxRuntimeRuntime (575) */
    type DanceboxRuntimeRuntime = Null;
} // declare module
