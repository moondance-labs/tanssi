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
    Set,
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

    /** @name FrameSupportDispatchPerDispatchClassWeight (8) */
    interface FrameSupportDispatchPerDispatchClassWeight extends Struct {
        readonly normal: SpWeightsWeightV2Weight;
        readonly operational: SpWeightsWeightV2Weight;
        readonly mandatory: SpWeightsWeightV2Weight;
    }

    /** @name SpWeightsWeightV2Weight (9) */
    interface SpWeightsWeightV2Weight extends Struct {
        readonly refTime: Compact<u64>;
        readonly proofSize: Compact<u64>;
    }

    /** @name SpRuntimeDigest (14) */
    interface SpRuntimeDigest extends Struct {
        readonly logs: Vec<SpRuntimeDigestDigestItem>;
    }

    /** @name SpRuntimeDigestDigestItem (16) */
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

    /** @name FrameSystemEventRecord (19) */
    interface FrameSystemEventRecord extends Struct {
        readonly phase: FrameSystemPhase;
        readonly event: Event;
        readonly topics: Vec<H256>;
    }

    /** @name FrameSystemEvent (21) */
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
        readonly type:
            | "ExtrinsicSuccess"
            | "ExtrinsicFailed"
            | "CodeUpdated"
            | "NewAccount"
            | "KilledAccount"
            | "Remarked";
    }

    /** @name FrameSupportDispatchDispatchInfo (22) */
    interface FrameSupportDispatchDispatchInfo extends Struct {
        readonly weight: SpWeightsWeightV2Weight;
        readonly class: FrameSupportDispatchDispatchClass;
        readonly paysFee: FrameSupportDispatchPays;
    }

    /** @name FrameSupportDispatchDispatchClass (23) */
    interface FrameSupportDispatchDispatchClass extends Enum {
        readonly isNormal: boolean;
        readonly isOperational: boolean;
        readonly isMandatory: boolean;
        readonly type: "Normal" | "Operational" | "Mandatory";
    }

    /** @name FrameSupportDispatchPays (24) */
    interface FrameSupportDispatchPays extends Enum {
        readonly isYes: boolean;
        readonly isNo: boolean;
        readonly type: "Yes" | "No";
    }

    /** @name SpRuntimeDispatchError (25) */
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

    /** @name SpRuntimeModuleError (26) */
    interface SpRuntimeModuleError extends Struct {
        readonly index: u8;
        readonly error: U8aFixed;
    }

    /** @name SpRuntimeTokenError (27) */
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

    /** @name SpArithmeticArithmeticError (28) */
    interface SpArithmeticArithmeticError extends Enum {
        readonly isUnderflow: boolean;
        readonly isOverflow: boolean;
        readonly isDivisionByZero: boolean;
        readonly type: "Underflow" | "Overflow" | "DivisionByZero";
    }

    /** @name SpRuntimeTransactionalError (29) */
    interface SpRuntimeTransactionalError extends Enum {
        readonly isLimitReached: boolean;
        readonly isNoLayer: boolean;
        readonly type: "LimitReached" | "NoLayer";
    }

    /** @name CumulusPalletParachainSystemEvent (30) */
    interface CumulusPalletParachainSystemEvent extends Enum {
        readonly isValidationFunctionStored: boolean;
        readonly isValidationFunctionApplied: boolean;
        readonly asValidationFunctionApplied: {
            readonly relayChainBlockNum: u32;
        } & Struct;
        readonly isValidationFunctionDiscarded: boolean;
        readonly isUpgradeAuthorized: boolean;
        readonly asUpgradeAuthorized: {
            readonly codeHash: H256;
        } & Struct;
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
            | "UpgradeAuthorized"
            | "DownwardMessagesReceived"
            | "DownwardMessagesProcessed"
            | "UpwardMessageSent";
    }

    /** @name PalletSudoEvent (32) */
    interface PalletSudoEvent extends Enum {
        readonly isSudid: boolean;
        readonly asSudid: {
            readonly sudoResult: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isKeyChanged: boolean;
        readonly asKeyChanged: {
            readonly oldSudoer: Option<AccountId32>;
        } & Struct;
        readonly isSudoAsDone: boolean;
        readonly asSudoAsDone: {
            readonly sudoResult: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly type: "Sudid" | "KeyChanged" | "SudoAsDone";
    }

    /** @name PalletUtilityEvent (36) */
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

    /** @name PalletProxyEvent (37) */
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

    /** @name FlashboxRuntimeProxyType (38) */
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

    /** @name PalletMigrationsEvent (40) */
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

    /** @name PalletMaintenanceModeEvent (41) */
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

    /** @name PalletTxPauseEvent (42) */
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

    /** @name PalletBalancesEvent (45) */
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
            | "Thawed";
    }

    /** @name FrameSupportTokensMiscBalanceStatus (46) */
    interface FrameSupportTokensMiscBalanceStatus extends Enum {
        readonly isFree: boolean;
        readonly isReserved: boolean;
        readonly type: "Free" | "Reserved";
    }

    /** @name PalletTransactionPaymentEvent (47) */
    interface PalletTransactionPaymentEvent extends Enum {
        readonly isTransactionFeePaid: boolean;
        readonly asTransactionFeePaid: {
            readonly who: AccountId32;
            readonly actualFee: u128;
            readonly tip: u128;
        } & Struct;
        readonly type: "TransactionFeePaid";
    }

    /** @name PalletIdentityEvent (48) */
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
            | "SubIdentityRevoked";
    }

    /** @name PalletRegistrarEvent (49) */
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
        readonly type:
            | "ParaIdRegistered"
            | "ParaIdDeregistered"
            | "ParaIdValidForCollating"
            | "ParaIdPaused"
            | "ParaIdUnpaused";
    }

    /** @name PalletCollatorAssignmentEvent (51) */
    interface PalletCollatorAssignmentEvent extends Enum {
        readonly isNewPendingAssignment: boolean;
        readonly asNewPendingAssignment: {
            readonly randomSeed: U8aFixed;
            readonly fullRotation: bool;
            readonly targetSession: u32;
        } & Struct;
        readonly type: "NewPendingAssignment";
    }

    /** @name PalletAuthorNotingEvent (53) */
    interface PalletAuthorNotingEvent extends Enum {
        readonly isLatestAuthorChanged: boolean;
        readonly asLatestAuthorChanged: {
            readonly paraId: u32;
            readonly blockNumber: u32;
            readonly newAuthor: AccountId32;
        } & Struct;
        readonly isRemovedAuthorData: boolean;
        readonly asRemovedAuthorData: {
            readonly paraId: u32;
        } & Struct;
        readonly type: "LatestAuthorChanged" | "RemovedAuthorData";
    }

    /** @name PalletServicesPaymentEvent (54) */
    interface PalletServicesPaymentEvent extends Enum {
        readonly isCreditsPurchased: boolean;
        readonly asCreditsPurchased: {
            readonly paraId: u32;
            readonly payer: AccountId32;
            readonly fee: u128;
            readonly creditsPurchased: u32;
            readonly creditsRemaining: u32;
        } & Struct;
        readonly isCreditBurned: boolean;
        readonly asCreditBurned: {
            readonly paraId: u32;
            readonly creditsRemaining: u32;
        } & Struct;
        readonly isCreditsSet: boolean;
        readonly asCreditsSet: {
            readonly paraId: u32;
            readonly credits: u32;
        } & Struct;
        readonly type: "CreditsPurchased" | "CreditBurned" | "CreditsSet";
    }

    /** @name PalletDataPreserversEvent (55) */
    interface PalletDataPreserversEvent extends Enum {
        readonly isBootNodesChanged: boolean;
        readonly asBootNodesChanged: {
            readonly paraId: u32;
        } & Struct;
        readonly type: "BootNodesChanged";
    }

    /** @name PalletInvulnerablesEvent (56) */
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

    /** @name PalletSessionEvent (58) */
    interface PalletSessionEvent extends Enum {
        readonly isNewSession: boolean;
        readonly asNewSession: {
            readonly sessionIndex: u32;
        } & Struct;
        readonly type: "NewSession";
    }

    /** @name PalletInflationRewardsEvent (59) */
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

    /** @name PalletTreasuryEvent (60) */
    interface PalletTreasuryEvent extends Enum {
        readonly isProposed: boolean;
        readonly asProposed: {
            readonly proposalIndex: u32;
        } & Struct;
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
        readonly isRejected: boolean;
        readonly asRejected: {
            readonly proposalIndex: u32;
            readonly slashed: u128;
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
            | "Proposed"
            | "Spending"
            | "Awarded"
            | "Rejected"
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

    /** @name FrameSystemPhase (61) */
    interface FrameSystemPhase extends Enum {
        readonly isApplyExtrinsic: boolean;
        readonly asApplyExtrinsic: u32;
        readonly isFinalization: boolean;
        readonly isInitialization: boolean;
        readonly type: "ApplyExtrinsic" | "Finalization" | "Initialization";
    }

    /** @name FrameSystemLastRuntimeUpgradeInfo (65) */
    interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
        readonly specVersion: Compact<u32>;
        readonly specName: Text;
    }

    /** @name FrameSystemCall (68) */
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
        readonly type:
            | "Remark"
            | "SetHeapPages"
            | "SetCode"
            | "SetCodeWithoutChecks"
            | "SetStorage"
            | "KillStorage"
            | "KillPrefix"
            | "RemarkWithEvent";
    }

    /** @name FrameSystemLimitsBlockWeights (72) */
    interface FrameSystemLimitsBlockWeights extends Struct {
        readonly baseBlock: SpWeightsWeightV2Weight;
        readonly maxBlock: SpWeightsWeightV2Weight;
        readonly perClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
    }

    /** @name FrameSupportDispatchPerDispatchClassWeightsPerClass (73) */
    interface FrameSupportDispatchPerDispatchClassWeightsPerClass extends Struct {
        readonly normal: FrameSystemLimitsWeightsPerClass;
        readonly operational: FrameSystemLimitsWeightsPerClass;
        readonly mandatory: FrameSystemLimitsWeightsPerClass;
    }

    /** @name FrameSystemLimitsWeightsPerClass (74) */
    interface FrameSystemLimitsWeightsPerClass extends Struct {
        readonly baseExtrinsic: SpWeightsWeightV2Weight;
        readonly maxExtrinsic: Option<SpWeightsWeightV2Weight>;
        readonly maxTotal: Option<SpWeightsWeightV2Weight>;
        readonly reserved: Option<SpWeightsWeightV2Weight>;
    }

    /** @name FrameSystemLimitsBlockLength (76) */
    interface FrameSystemLimitsBlockLength extends Struct {
        readonly max: FrameSupportDispatchPerDispatchClassU32;
    }

    /** @name FrameSupportDispatchPerDispatchClassU32 (77) */
    interface FrameSupportDispatchPerDispatchClassU32 extends Struct {
        readonly normal: u32;
        readonly operational: u32;
        readonly mandatory: u32;
    }

    /** @name SpWeightsRuntimeDbWeight (78) */
    interface SpWeightsRuntimeDbWeight extends Struct {
        readonly read: u64;
        readonly write: u64;
    }

    /** @name SpVersionRuntimeVersion (79) */
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

    /** @name FrameSystemError (84) */
    interface FrameSystemError extends Enum {
        readonly isInvalidSpecName: boolean;
        readonly isSpecVersionNeedsToIncrease: boolean;
        readonly isFailedToExtractRuntimeVersion: boolean;
        readonly isNonDefaultComposite: boolean;
        readonly isNonZeroRefCount: boolean;
        readonly isCallFiltered: boolean;
        readonly type:
            | "InvalidSpecName"
            | "SpecVersionNeedsToIncrease"
            | "FailedToExtractRuntimeVersion"
            | "NonDefaultComposite"
            | "NonZeroRefCount"
            | "CallFiltered";
    }

    /** @name CumulusPalletParachainSystemUnincludedSegmentAncestor (86) */
    interface CumulusPalletParachainSystemUnincludedSegmentAncestor extends Struct {
        readonly usedBandwidth: CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth;
        readonly paraHeadHash: Option<H256>;
        readonly consumedGoAheadSignal: Option<PolkadotPrimitivesV6UpgradeGoAhead>;
    }

    /** @name CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth (87) */
    interface CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth extends Struct {
        readonly umpMsgCount: u32;
        readonly umpTotalBytes: u32;
        readonly hrmpOutgoing: BTreeMap<u32, CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate>;
    }

    /** @name CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate (89) */
    interface CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate extends Struct {
        readonly msgCount: u32;
        readonly totalBytes: u32;
    }

    /** @name PolkadotPrimitivesV6UpgradeGoAhead (94) */
    interface PolkadotPrimitivesV6UpgradeGoAhead extends Enum {
        readonly isAbort: boolean;
        readonly isGoAhead: boolean;
        readonly type: "Abort" | "GoAhead";
    }

    /** @name CumulusPalletParachainSystemUnincludedSegmentSegmentTracker (95) */
    interface CumulusPalletParachainSystemUnincludedSegmentSegmentTracker extends Struct {
        readonly usedBandwidth: CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth;
        readonly hrmpWatermark: Option<u32>;
        readonly consumedGoAheadSignal: Option<PolkadotPrimitivesV6UpgradeGoAhead>;
    }

    /** @name PolkadotPrimitivesV6PersistedValidationData (97) */
    interface PolkadotPrimitivesV6PersistedValidationData extends Struct {
        readonly parentHead: Bytes;
        readonly relayParentNumber: u32;
        readonly relayParentStorageRoot: H256;
        readonly maxPovSize: u32;
    }

    /** @name PolkadotPrimitivesV6UpgradeRestriction (100) */
    interface PolkadotPrimitivesV6UpgradeRestriction extends Enum {
        readonly isPresent: boolean;
        readonly type: "Present";
    }

    /** @name SpTrieStorageProof (101) */
    interface SpTrieStorageProof extends Struct {
        readonly trieNodes: BTreeSet<Bytes>;
    }

    /** @name CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot (103) */
    interface CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot extends Struct {
        readonly dmqMqcHead: H256;
        readonly relayDispatchQueueRemainingCapacity: CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity;
        readonly ingressChannels: Vec<ITuple<[u32, PolkadotPrimitivesV6AbridgedHrmpChannel]>>;
        readonly egressChannels: Vec<ITuple<[u32, PolkadotPrimitivesV6AbridgedHrmpChannel]>>;
    }

    /** @name CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity (104) */
    interface CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity extends Struct {
        readonly remainingCount: u32;
        readonly remainingSize: u32;
    }

    /** @name PolkadotPrimitivesV6AbridgedHrmpChannel (107) */
    interface PolkadotPrimitivesV6AbridgedHrmpChannel extends Struct {
        readonly maxCapacity: u32;
        readonly maxTotalSize: u32;
        readonly maxMessageSize: u32;
        readonly msgCount: u32;
        readonly totalSize: u32;
        readonly mqcHead: Option<H256>;
    }

    /** @name PolkadotPrimitivesV6AbridgedHostConfiguration (108) */
    interface PolkadotPrimitivesV6AbridgedHostConfiguration extends Struct {
        readonly maxCodeSize: u32;
        readonly maxHeadDataSize: u32;
        readonly maxUpwardQueueCount: u32;
        readonly maxUpwardQueueSize: u32;
        readonly maxUpwardMessageSize: u32;
        readonly maxUpwardMessageNumPerCandidate: u32;
        readonly hrmpMaxMessageNumPerCandidate: u32;
        readonly validationUpgradeCooldown: u32;
        readonly validationUpgradeDelay: u32;
        readonly asyncBackingParams: PolkadotPrimitivesV6AsyncBackingAsyncBackingParams;
    }

    /** @name PolkadotPrimitivesV6AsyncBackingAsyncBackingParams (109) */
    interface PolkadotPrimitivesV6AsyncBackingAsyncBackingParams extends Struct {
        readonly maxCandidateDepth: u32;
        readonly allowedAncestryLen: u32;
    }

    /** @name PolkadotCorePrimitivesOutboundHrmpMessage (115) */
    interface PolkadotCorePrimitivesOutboundHrmpMessage extends Struct {
        readonly recipient: u32;
        readonly data: Bytes;
    }

    /** @name CumulusPalletParachainSystemCodeUpgradeAuthorization (117) */
    interface CumulusPalletParachainSystemCodeUpgradeAuthorization extends Struct {
        readonly codeHash: H256;
        readonly checkVersion: bool;
    }

    /** @name CumulusPalletParachainSystemCall (118) */
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

    /** @name CumulusPrimitivesParachainInherentParachainInherentData (119) */
    interface CumulusPrimitivesParachainInherentParachainInherentData extends Struct {
        readonly validationData: PolkadotPrimitivesV6PersistedValidationData;
        readonly relayChainState: SpTrieStorageProof;
        readonly downwardMessages: Vec<PolkadotCorePrimitivesInboundDownwardMessage>;
        readonly horizontalMessages: BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>;
    }

    /** @name PolkadotCorePrimitivesInboundDownwardMessage (121) */
    interface PolkadotCorePrimitivesInboundDownwardMessage extends Struct {
        readonly sentAt: u32;
        readonly msg: Bytes;
    }

    /** @name PolkadotCorePrimitivesInboundHrmpMessage (124) */
    interface PolkadotCorePrimitivesInboundHrmpMessage extends Struct {
        readonly sentAt: u32;
        readonly data: Bytes;
    }

    /** @name CumulusPalletParachainSystemError (127) */
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

    /** @name PalletTimestampCall (128) */
    interface PalletTimestampCall extends Enum {
        readonly isSet: boolean;
        readonly asSet: {
            readonly now: Compact<u64>;
        } & Struct;
        readonly type: "Set";
    }

    /** @name ParachainInfoCall (129) */
    type ParachainInfoCall = Null;

    /** @name PalletSudoCall (130) */
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
        readonly type: "Sudo" | "SudoUncheckedWeight" | "SetKey" | "SudoAs";
    }

    /** @name PalletUtilityCall (132) */
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

    /** @name FlashboxRuntimeOriginCaller (134) */
    interface FlashboxRuntimeOriginCaller extends Enum {
        readonly isSystem: boolean;
        readonly asSystem: FrameSupportDispatchRawOrigin;
        readonly isVoid: boolean;
        readonly type: "System" | "Void";
    }

    /** @name FrameSupportDispatchRawOrigin (135) */
    interface FrameSupportDispatchRawOrigin extends Enum {
        readonly isRoot: boolean;
        readonly isSigned: boolean;
        readonly asSigned: AccountId32;
        readonly isNone: boolean;
        readonly type: "Root" | "Signed" | "None";
    }

    /** @name SpCoreVoid (136) */
    type SpCoreVoid = Null;

    /** @name PalletProxyCall (137) */
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

    /** @name PalletMaintenanceModeCall (142) */
    interface PalletMaintenanceModeCall extends Enum {
        readonly isEnterMaintenanceMode: boolean;
        readonly isResumeNormalOperation: boolean;
        readonly type: "EnterMaintenanceMode" | "ResumeNormalOperation";
    }

    /** @name PalletTxPauseCall (143) */
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

    /** @name PalletBalancesCall (144) */
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
        readonly type:
            | "TransferAllowDeath"
            | "ForceTransfer"
            | "TransferKeepAlive"
            | "TransferAll"
            | "ForceUnreserve"
            | "UpgradeAccounts"
            | "ForceSetBalance";
    }

    /** @name PalletIdentityCall (146) */
    interface PalletIdentityCall extends Enum {
        readonly isAddRegistrar: boolean;
        readonly asAddRegistrar: {
            readonly account: MultiAddress;
        } & Struct;
        readonly isSetIdentity: boolean;
        readonly asSetIdentity: {
            readonly info: PalletIdentitySimpleIdentityInfo;
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
            readonly fields: PalletIdentityBitFlags;
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
            | "QuitSub";
    }

    /** @name PalletIdentitySimpleIdentityInfo (147) */
    interface PalletIdentitySimpleIdentityInfo extends Struct {
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

    /** @name PalletIdentityBitFlags (184) */
    interface PalletIdentityBitFlags extends Set {
        readonly isDisplay: boolean;
        readonly isLegal: boolean;
        readonly isWeb: boolean;
        readonly isRiot: boolean;
        readonly isEmail: boolean;
        readonly isPgpFingerprint: boolean;
        readonly isImage: boolean;
        readonly isTwitter: boolean;
    }

    /** @name PalletIdentitySimpleIdentityField (185) */
    interface PalletIdentitySimpleIdentityField extends Enum {
        readonly isDisplay: boolean;
        readonly isLegal: boolean;
        readonly isWeb: boolean;
        readonly isRiot: boolean;
        readonly isEmail: boolean;
        readonly isPgpFingerprint: boolean;
        readonly isImage: boolean;
        readonly isTwitter: boolean;
        readonly type: "Display" | "Legal" | "Web" | "Riot" | "Email" | "PgpFingerprint" | "Image" | "Twitter";
    }

    /** @name PalletIdentityJudgement (186) */
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

    /** @name PalletRegistrarCall (187) */
    interface PalletRegistrarCall extends Enum {
        readonly isRegister: boolean;
        readonly asRegister: {
            readonly paraId: u32;
            readonly genesisData: TpContainerChainGenesisDataContainerChainGenesisData;
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
        readonly type:
            | "Register"
            | "Deregister"
            | "MarkValidForCollating"
            | "PauseContainerChain"
            | "UnpauseContainerChain";
    }

    /** @name TpContainerChainGenesisDataContainerChainGenesisData (188) */
    interface TpContainerChainGenesisDataContainerChainGenesisData extends Struct {
        readonly storage: Vec<TpContainerChainGenesisDataContainerChainGenesisDataItem>;
        readonly name: Bytes;
        readonly id: Bytes;
        readonly forkId: Option<Bytes>;
        readonly extensions: Bytes;
        readonly properties: TpContainerChainGenesisDataProperties;
    }

    /** @name TpContainerChainGenesisDataContainerChainGenesisDataItem (190) */
    interface TpContainerChainGenesisDataContainerChainGenesisDataItem extends Struct {
        readonly key: Bytes;
        readonly value: Bytes;
    }

    /** @name TpContainerChainGenesisDataProperties (192) */
    interface TpContainerChainGenesisDataProperties extends Struct {
        readonly tokenMetadata: TpContainerChainGenesisDataTokenMetadata;
        readonly isEthereum: bool;
    }

    /** @name TpContainerChainGenesisDataTokenMetadata (193) */
    interface TpContainerChainGenesisDataTokenMetadata extends Struct {
        readonly tokenSymbol: Bytes;
        readonly ss58Format: u32;
        readonly tokenDecimals: u32;
    }

    /** @name PalletConfigurationCall (195) */
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

    /** @name PalletCollatorAssignmentCall (197) */
    type PalletCollatorAssignmentCall = Null;

    /** @name PalletAuthorNotingCall (198) */
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
        } & Struct;
        readonly isKillAuthorData: boolean;
        readonly asKillAuthorData: {
            readonly paraId: u32;
        } & Struct;
        readonly type: "SetLatestAuthorData" | "SetAuthor" | "KillAuthorData";
    }

    /** @name TpAuthorNotingInherentOwnParachainInherentData (199) */
    interface TpAuthorNotingInherentOwnParachainInherentData extends Struct {
        readonly relayStorageProof: SpTrieStorageProof;
    }

    /** @name PalletAuthorityAssignmentCall (200) */
    type PalletAuthorityAssignmentCall = Null;

    /** @name PalletServicesPaymentCall (201) */
    interface PalletServicesPaymentCall extends Enum {
        readonly isPurchaseCredits: boolean;
        readonly asPurchaseCredits: {
            readonly paraId: u32;
            readonly credits: u32;
            readonly maxPricePerCredit: Option<u128>;
        } & Struct;
        readonly isSetCredits: boolean;
        readonly asSetCredits: {
            readonly paraId: u32;
            readonly credits: u32;
        } & Struct;
        readonly isSetGivenFreeCredits: boolean;
        readonly asSetGivenFreeCredits: {
            readonly paraId: u32;
            readonly givenFreeCredits: bool;
        } & Struct;
        readonly type: "PurchaseCredits" | "SetCredits" | "SetGivenFreeCredits";
    }

    /** @name PalletDataPreserversCall (203) */
    interface PalletDataPreserversCall extends Enum {
        readonly isSetBootNodes: boolean;
        readonly asSetBootNodes: {
            readonly paraId: u32;
            readonly bootNodes: Vec<Bytes>;
        } & Struct;
        readonly type: "SetBootNodes";
    }

    /** @name PalletInvulnerablesCall (207) */
    interface PalletInvulnerablesCall extends Enum {
        readonly isSetInvulnerables: boolean;
        readonly asSetInvulnerables: {
            readonly new_: Vec<AccountId32>;
        } & Struct;
        readonly isAddInvulnerable: boolean;
        readonly asAddInvulnerable: {
            readonly who: AccountId32;
        } & Struct;
        readonly isRemoveInvulnerable: boolean;
        readonly asRemoveInvulnerable: {
            readonly who: AccountId32;
        } & Struct;
        readonly type: "SetInvulnerables" | "AddInvulnerable" | "RemoveInvulnerable";
    }

    /** @name PalletSessionCall (208) */
    interface PalletSessionCall extends Enum {
        readonly isSetKeys: boolean;
        readonly asSetKeys: {
            readonly keys_: FlashboxRuntimeSessionKeys;
            readonly proof: Bytes;
        } & Struct;
        readonly isPurgeKeys: boolean;
        readonly type: "SetKeys" | "PurgeKeys";
    }

    /** @name FlashboxRuntimeSessionKeys (209) */
    interface FlashboxRuntimeSessionKeys extends Struct {
        readonly nimbus: NimbusPrimitivesNimbusCryptoPublic;
    }

    /** @name NimbusPrimitivesNimbusCryptoPublic (210) */
    interface NimbusPrimitivesNimbusCryptoPublic extends SpCoreSr25519Public {}

    /** @name SpCoreSr25519Public (211) */
    interface SpCoreSr25519Public extends U8aFixed {}

    /** @name PalletAuthorInherentCall (212) */
    interface PalletAuthorInherentCall extends Enum {
        readonly isKickOffAuthorshipValidation: boolean;
        readonly type: "KickOffAuthorshipValidation";
    }

    /** @name PalletTreasuryCall (213) */
    interface PalletTreasuryCall extends Enum {
        readonly isProposeSpend: boolean;
        readonly asProposeSpend: {
            readonly value: Compact<u128>;
            readonly beneficiary: MultiAddress;
        } & Struct;
        readonly isRejectProposal: boolean;
        readonly asRejectProposal: {
            readonly proposalId: Compact<u32>;
        } & Struct;
        readonly isApproveProposal: boolean;
        readonly asApproveProposal: {
            readonly proposalId: Compact<u32>;
        } & Struct;
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
        readonly type:
            | "ProposeSpend"
            | "RejectProposal"
            | "ApproveProposal"
            | "SpendLocal"
            | "RemoveApproval"
            | "Spend"
            | "Payout"
            | "CheckStatus"
            | "VoidSpend";
    }

    /** @name PalletRootTestingCall (214) */
    interface PalletRootTestingCall extends Enum {
        readonly isFillBlock: boolean;
        readonly asFillBlock: {
            readonly ratio: Perbill;
        } & Struct;
        readonly type: "FillBlock";
    }

    /** @name PalletSudoError (215) */
    interface PalletSudoError extends Enum {
        readonly isRequireSudo: boolean;
        readonly type: "RequireSudo";
    }

    /** @name PalletUtilityError (216) */
    interface PalletUtilityError extends Enum {
        readonly isTooManyCalls: boolean;
        readonly type: "TooManyCalls";
    }

    /** @name PalletProxyProxyDefinition (219) */
    interface PalletProxyProxyDefinition extends Struct {
        readonly delegate: AccountId32;
        readonly proxyType: FlashboxRuntimeProxyType;
        readonly delay: u32;
    }

    /** @name PalletProxyAnnouncement (223) */
    interface PalletProxyAnnouncement extends Struct {
        readonly real: AccountId32;
        readonly callHash: H256;
        readonly height: u32;
    }

    /** @name PalletProxyError (225) */
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

    /** @name PalletMigrationsError (226) */
    interface PalletMigrationsError extends Enum {
        readonly isPreimageMissing: boolean;
        readonly isWrongUpperBound: boolean;
        readonly isPreimageIsTooBig: boolean;
        readonly isPreimageAlreadyExists: boolean;
        readonly type: "PreimageMissing" | "WrongUpperBound" | "PreimageIsTooBig" | "PreimageAlreadyExists";
    }

    /** @name PalletMaintenanceModeError (227) */
    interface PalletMaintenanceModeError extends Enum {
        readonly isAlreadyInMaintenanceMode: boolean;
        readonly isNotInMaintenanceMode: boolean;
        readonly type: "AlreadyInMaintenanceMode" | "NotInMaintenanceMode";
    }

    /** @name PalletTxPauseError (228) */
    interface PalletTxPauseError extends Enum {
        readonly isIsPaused: boolean;
        readonly isIsUnpaused: boolean;
        readonly isUnpausable: boolean;
        readonly isNotFound: boolean;
        readonly type: "IsPaused" | "IsUnpaused" | "Unpausable" | "NotFound";
    }

    /** @name PalletBalancesBalanceLock (230) */
    interface PalletBalancesBalanceLock extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
        readonly reasons: PalletBalancesReasons;
    }

    /** @name PalletBalancesReasons (231) */
    interface PalletBalancesReasons extends Enum {
        readonly isFee: boolean;
        readonly isMisc: boolean;
        readonly isAll: boolean;
        readonly type: "Fee" | "Misc" | "All";
    }

    /** @name PalletBalancesReserveData (234) */
    interface PalletBalancesReserveData extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
    }

    /** @name FlashboxRuntimeRuntimeHoldReason (238) */
    type FlashboxRuntimeRuntimeHoldReason = Null;

    /** @name PalletBalancesIdAmount (241) */
    interface PalletBalancesIdAmount extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
    }

    /** @name PalletBalancesError (243) */
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
            | "TooManyFreezes";
    }

    /** @name PalletTransactionPaymentReleases (244) */
    interface PalletTransactionPaymentReleases extends Enum {
        readonly isV1Ancient: boolean;
        readonly isV2: boolean;
        readonly type: "V1Ancient" | "V2";
    }

    /** @name PalletIdentityRegistration (245) */
    interface PalletIdentityRegistration extends Struct {
        readonly judgements: Vec<ITuple<[u32, PalletIdentityJudgement]>>;
        readonly deposit: u128;
        readonly info: PalletIdentitySimpleIdentityInfo;
    }

    /** @name PalletIdentityRegistrarInfo (253) */
    interface PalletIdentityRegistrarInfo extends Struct {
        readonly account: AccountId32;
        readonly fee: u128;
        readonly fields: PalletIdentityBitFlags;
    }

    /** @name PalletIdentityError (255) */
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
        readonly isTooManyFields: boolean;
        readonly isTooManyRegistrars: boolean;
        readonly isAlreadyClaimed: boolean;
        readonly isNotSub: boolean;
        readonly isNotOwned: boolean;
        readonly isJudgementForDifferentIdentity: boolean;
        readonly isJudgementPaymentFailed: boolean;
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
            | "TooManyFields"
            | "TooManyRegistrars"
            | "AlreadyClaimed"
            | "NotSub"
            | "NotOwned"
            | "JudgementForDifferentIdentity"
            | "JudgementPaymentFailed";
    }

    /** @name PalletRegistrarDepositInfo (260) */
    interface PalletRegistrarDepositInfo extends Struct {
        readonly creator: AccountId32;
        readonly deposit: u128;
    }

    /** @name PalletRegistrarError (261) */
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
        readonly type:
            | "ParaIdAlreadyRegistered"
            | "ParaIdNotRegistered"
            | "ParaIdAlreadyDeregistered"
            | "ParaIdAlreadyPaused"
            | "ParaIdNotPaused"
            | "ParaIdListFull"
            | "GenesisDataTooBig"
            | "ParaIdNotInPendingVerification"
            | "NotSufficientDeposit";
    }

    /** @name PalletConfigurationHostConfiguration (262) */
    interface PalletConfigurationHostConfiguration extends Struct {
        readonly maxCollators: u32;
        readonly minOrchestratorCollators: u32;
        readonly maxOrchestratorCollators: u32;
        readonly collatorsPerContainer: u32;
        readonly fullRotationPeriod: u32;
        readonly collatorsPerParathread: u32;
        readonly parathreadsPerCollator: u32;
        readonly targetContainerChainFullness: Perbill;
    }

    /** @name PalletConfigurationError (265) */
    interface PalletConfigurationError extends Enum {
        readonly isInvalidNewValue: boolean;
        readonly type: "InvalidNewValue";
    }

    /** @name DpCollatorAssignmentAssignedCollatorsAccountId32 (266) */
    interface DpCollatorAssignmentAssignedCollatorsAccountId32 extends Struct {
        readonly orchestratorChain: Vec<AccountId32>;
        readonly containerChains: BTreeMap<u32, Vec<AccountId32>>;
    }

    /** @name PalletAuthorNotingContainerChainBlockInfo (271) */
    interface PalletAuthorNotingContainerChainBlockInfo extends Struct {
        readonly blockNumber: u32;
        readonly author: AccountId32;
    }

    /** @name PalletAuthorNotingError (272) */
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

    /** @name DpCollatorAssignmentAssignedCollatorsPublic (273) */
    interface DpCollatorAssignmentAssignedCollatorsPublic extends Struct {
        readonly orchestratorChain: Vec<NimbusPrimitivesNimbusCryptoPublic>;
        readonly containerChains: BTreeMap<u32, Vec<NimbusPrimitivesNimbusCryptoPublic>>;
    }

    /** @name PalletServicesPaymentError (278) */
    interface PalletServicesPaymentError extends Enum {
        readonly isInsufficientFundsToPurchaseCredits: boolean;
        readonly isInsufficientCredits: boolean;
        readonly isCreditPriceTooExpensive: boolean;
        readonly type: "InsufficientFundsToPurchaseCredits" | "InsufficientCredits" | "CreditPriceTooExpensive";
    }

    /** @name PalletDataPreserversError (279) */
    interface PalletDataPreserversError extends Enum {
        readonly isNoBootNodes: boolean;
        readonly type: "NoBootNodes";
    }

    /** @name PalletInvulnerablesError (281) */
    interface PalletInvulnerablesError extends Enum {
        readonly isTooManyInvulnerables: boolean;
        readonly isAlreadyInvulnerable: boolean;
        readonly isNotInvulnerable: boolean;
        readonly type: "TooManyInvulnerables" | "AlreadyInvulnerable" | "NotInvulnerable";
    }

    /** @name SpCoreCryptoKeyTypeId (286) */
    interface SpCoreCryptoKeyTypeId extends U8aFixed {}

    /** @name PalletSessionError (287) */
    interface PalletSessionError extends Enum {
        readonly isInvalidProof: boolean;
        readonly isNoAssociatedValidatorId: boolean;
        readonly isDuplicatedKey: boolean;
        readonly isNoKeys: boolean;
        readonly isNoAccount: boolean;
        readonly type: "InvalidProof" | "NoAssociatedValidatorId" | "DuplicatedKey" | "NoKeys" | "NoAccount";
    }

    /** @name PalletAuthorInherentError (291) */
    interface PalletAuthorInherentError extends Enum {
        readonly isAuthorAlreadySet: boolean;
        readonly isNoAccountId: boolean;
        readonly isCannotBeAuthor: boolean;
        readonly type: "AuthorAlreadySet" | "NoAccountId" | "CannotBeAuthor";
    }

    /** @name PalletInflationRewardsChainsToRewardValue (292) */
    interface PalletInflationRewardsChainsToRewardValue extends Struct {
        readonly paraIds: Vec<u32>;
        readonly rewardsPerChain: u128;
    }

    /** @name PalletTreasuryProposal (293) */
    interface PalletTreasuryProposal extends Struct {
        readonly proposer: AccountId32;
        readonly value: u128;
        readonly beneficiary: AccountId32;
        readonly bond: u128;
    }

    /** @name PalletTreasurySpendStatus (295) */
    interface PalletTreasurySpendStatus extends Struct {
        readonly assetKind: Null;
        readonly amount: u128;
        readonly beneficiary: AccountId32;
        readonly validFrom: u32;
        readonly expireAt: u32;
        readonly status: PalletTreasuryPaymentState;
    }

    /** @name PalletTreasuryPaymentState (296) */
    interface PalletTreasuryPaymentState extends Enum {
        readonly isPending: boolean;
        readonly isAttempted: boolean;
        readonly asAttempted: {
            readonly id: Null;
        } & Struct;
        readonly isFailed: boolean;
        readonly type: "Pending" | "Attempted" | "Failed";
    }

    /** @name FrameSupportPalletId (298) */
    interface FrameSupportPalletId extends U8aFixed {}

    /** @name PalletTreasuryError (299) */
    interface PalletTreasuryError extends Enum {
        readonly isInsufficientProposersBalance: boolean;
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
            | "InsufficientProposersBalance"
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

    /** @name SpRuntimeMultiSignature (304) */
    interface SpRuntimeMultiSignature extends Enum {
        readonly isEd25519: boolean;
        readonly asEd25519: SpCoreEd25519Signature;
        readonly isSr25519: boolean;
        readonly asSr25519: SpCoreSr25519Signature;
        readonly isEcdsa: boolean;
        readonly asEcdsa: SpCoreEcdsaSignature;
        readonly type: "Ed25519" | "Sr25519" | "Ecdsa";
    }

    /** @name SpCoreEd25519Signature (305) */
    interface SpCoreEd25519Signature extends U8aFixed {}

    /** @name SpCoreSr25519Signature (307) */
    interface SpCoreSr25519Signature extends U8aFixed {}

    /** @name SpCoreEcdsaSignature (308) */
    interface SpCoreEcdsaSignature extends U8aFixed {}

    /** @name FrameSystemExtensionsCheckNonZeroSender (311) */
    type FrameSystemExtensionsCheckNonZeroSender = Null;

    /** @name FrameSystemExtensionsCheckSpecVersion (312) */
    type FrameSystemExtensionsCheckSpecVersion = Null;

    /** @name FrameSystemExtensionsCheckTxVersion (313) */
    type FrameSystemExtensionsCheckTxVersion = Null;

    /** @name FrameSystemExtensionsCheckGenesis (314) */
    type FrameSystemExtensionsCheckGenesis = Null;

    /** @name FrameSystemExtensionsCheckNonce (317) */
    interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

    /** @name FrameSystemExtensionsCheckWeight (318) */
    type FrameSystemExtensionsCheckWeight = Null;

    /** @name PalletTransactionPaymentChargeTransactionPayment (319) */
    interface PalletTransactionPaymentChargeTransactionPayment extends Compact<u128> {}

    /** @name FlashboxRuntimeRuntime (320) */
    type FlashboxRuntimeRuntime = Null;
} // declare module
