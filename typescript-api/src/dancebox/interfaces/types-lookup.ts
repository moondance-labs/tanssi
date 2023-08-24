// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/types/lookup";

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
import type {
  AccountId32,
  Call,
  H256,
  MultiAddress,
  Perbill,
} from "@polkadot/types/interfaces/runtime";
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
    readonly type:
      | "Other"
      | "Consensus"
      | "Seal"
      | "PreRuntime"
      | "RuntimeEnvironmentUpdated";
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
    readonly type:
      | "ProxyExecuted"
      | "PureCreated"
      | "Announced"
      | "ProxyAdded"
      | "ProxyRemoved";
  }

  /** @name DanceboxRuntimeProxyType (38) */
  interface DanceboxRuntimeProxyType extends Enum {
    readonly isAny: boolean;
    readonly isNonTransfer: boolean;
    readonly isGovernance: boolean;
    readonly isStaking: boolean;
    readonly isCancelProxy: boolean;
    readonly isBalances: boolean;
    readonly type:
      | "Any"
      | "NonTransfer"
      | "Governance"
      | "Staking"
      | "CancelProxy"
      | "Balances";
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

  /** @name PalletBalancesEvent (42) */
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

  /** @name FrameSupportTokensMiscBalanceStatus (43) */
  interface FrameSupportTokensMiscBalanceStatus extends Enum {
    readonly isFree: boolean;
    readonly isReserved: boolean;
    readonly type: "Free" | "Reserved";
  }

  /** @name PalletTransactionPaymentEvent (44) */
  interface PalletTransactionPaymentEvent extends Enum {
    readonly isTransactionFeePaid: boolean;
    readonly asTransactionFeePaid: {
      readonly who: AccountId32;
      readonly actualFee: u128;
      readonly tip: u128;
    } & Struct;
    readonly type: "TransactionFeePaid";
  }

  /** @name PalletRegistrarEvent (45) */
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
    readonly isBootNodesChanged: boolean;
    readonly asBootNodesChanged: {
      readonly paraId: u32;
    } & Struct;
    readonly type:
      | "ParaIdRegistered"
      | "ParaIdDeregistered"
      | "ParaIdValidForCollating"
      | "BootNodesChanged";
  }

  /** @name PalletAuthorNotingEvent (47) */
  interface PalletAuthorNotingEvent extends Enum {
    readonly isLatestAuthorChanged: boolean;
    readonly asLatestAuthorChanged: {
      readonly paraId: u32;
      readonly blockNumber: u32;
      readonly newAuthor: AccountId32;
    } & Struct;
    readonly type: "LatestAuthorChanged";
  }

  /** @name PalletCollatorSelectionEvent (48) */
  interface PalletCollatorSelectionEvent extends Enum {
    readonly isNewInvulnerables: boolean;
    readonly asNewInvulnerables: {
      readonly invulnerables: Vec<AccountId32>;
    } & Struct;
    readonly isNewDesiredCandidates: boolean;
    readonly asNewDesiredCandidates: {
      readonly desiredCandidates: u32;
    } & Struct;
    readonly isNewCandidacyBond: boolean;
    readonly asNewCandidacyBond: {
      readonly bondAmount: u128;
    } & Struct;
    readonly isCandidateAdded: boolean;
    readonly asCandidateAdded: {
      readonly accountId: AccountId32;
      readonly deposit: u128;
    } & Struct;
    readonly isCandidateRemoved: boolean;
    readonly asCandidateRemoved: {
      readonly accountId: AccountId32;
    } & Struct;
    readonly type:
      | "NewInvulnerables"
      | "NewDesiredCandidates"
      | "NewCandidacyBond"
      | "CandidateAdded"
      | "CandidateRemoved";
  }

  /** @name PalletSessionEvent (50) */
  interface PalletSessionEvent extends Enum {
    readonly isNewSession: boolean;
    readonly asNewSession: {
      readonly sessionIndex: u32;
    } & Struct;
    readonly type: "NewSession";
  }

  /** @name PalletInvulnerablesEvent (51) */
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
    readonly type:
      | "NewInvulnerables"
      | "InvulnerableAdded"
      | "InvulnerableRemoved";
  }

  /** @name FrameSystemPhase (52) */
  interface FrameSystemPhase extends Enum {
    readonly isApplyExtrinsic: boolean;
    readonly asApplyExtrinsic: u32;
    readonly isFinalization: boolean;
    readonly isInitialization: boolean;
    readonly type: "ApplyExtrinsic" | "Finalization" | "Initialization";
  }

  /** @name FrameSystemLastRuntimeUpgradeInfo (56) */
  interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
    readonly specVersion: Compact<u32>;
    readonly specName: Text;
  }

  /** @name FrameSystemCall (60) */
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

  /** @name FrameSystemLimitsBlockWeights (64) */
  interface FrameSystemLimitsBlockWeights extends Struct {
    readonly baseBlock: SpWeightsWeightV2Weight;
    readonly maxBlock: SpWeightsWeightV2Weight;
    readonly perClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
  }

  /** @name FrameSupportDispatchPerDispatchClassWeightsPerClass (65) */
  interface FrameSupportDispatchPerDispatchClassWeightsPerClass extends Struct {
    readonly normal: FrameSystemLimitsWeightsPerClass;
    readonly operational: FrameSystemLimitsWeightsPerClass;
    readonly mandatory: FrameSystemLimitsWeightsPerClass;
  }

  /** @name FrameSystemLimitsWeightsPerClass (66) */
  interface FrameSystemLimitsWeightsPerClass extends Struct {
    readonly baseExtrinsic: SpWeightsWeightV2Weight;
    readonly maxExtrinsic: Option<SpWeightsWeightV2Weight>;
    readonly maxTotal: Option<SpWeightsWeightV2Weight>;
    readonly reserved: Option<SpWeightsWeightV2Weight>;
  }

  /** @name FrameSystemLimitsBlockLength (68) */
  interface FrameSystemLimitsBlockLength extends Struct {
    readonly max: FrameSupportDispatchPerDispatchClassU32;
  }

  /** @name FrameSupportDispatchPerDispatchClassU32 (69) */
  interface FrameSupportDispatchPerDispatchClassU32 extends Struct {
    readonly normal: u32;
    readonly operational: u32;
    readonly mandatory: u32;
  }

  /** @name SpWeightsRuntimeDbWeight (70) */
  interface SpWeightsRuntimeDbWeight extends Struct {
    readonly read: u64;
    readonly write: u64;
  }

  /** @name SpVersionRuntimeVersion (71) */
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

  /** @name FrameSystemError (76) */
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

  /** @name PolkadotPrimitivesV4PersistedValidationData (77) */
  interface PolkadotPrimitivesV4PersistedValidationData extends Struct {
    readonly parentHead: Bytes;
    readonly relayParentNumber: u32;
    readonly relayParentStorageRoot: H256;
    readonly maxPovSize: u32;
  }

  /** @name PolkadotPrimitivesV4UpgradeRestriction (80) */
  interface PolkadotPrimitivesV4UpgradeRestriction extends Enum {
    readonly isPresent: boolean;
    readonly type: "Present";
  }

  /** @name SpTrieStorageProof (81) */
  interface SpTrieStorageProof extends Struct {
    readonly trieNodes: BTreeSet<Bytes>;
  }

  /** @name CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot (83) */
  interface CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot
    extends Struct {
    readonly dmqMqcHead: H256;
    readonly relayDispatchQueueSize: CumulusPalletParachainSystemRelayStateSnapshotRelayDispachQueueSize;
    readonly ingressChannels: Vec<
      ITuple<[u32, PolkadotPrimitivesV4AbridgedHrmpChannel]>
    >;
    readonly egressChannels: Vec<
      ITuple<[u32, PolkadotPrimitivesV4AbridgedHrmpChannel]>
    >;
  }

  /** @name CumulusPalletParachainSystemRelayStateSnapshotRelayDispachQueueSize (84) */
  interface CumulusPalletParachainSystemRelayStateSnapshotRelayDispachQueueSize
    extends Struct {
    readonly remainingCount: u32;
    readonly remainingSize: u32;
  }

  /** @name PolkadotPrimitivesV4AbridgedHrmpChannel (87) */
  interface PolkadotPrimitivesV4AbridgedHrmpChannel extends Struct {
    readonly maxCapacity: u32;
    readonly maxTotalSize: u32;
    readonly maxMessageSize: u32;
    readonly msgCount: u32;
    readonly totalSize: u32;
    readonly mqcHead: Option<H256>;
  }

  /** @name PolkadotPrimitivesV4AbridgedHostConfiguration (89) */
  interface PolkadotPrimitivesV4AbridgedHostConfiguration extends Struct {
    readonly maxCodeSize: u32;
    readonly maxHeadDataSize: u32;
    readonly maxUpwardQueueCount: u32;
    readonly maxUpwardQueueSize: u32;
    readonly maxUpwardMessageSize: u32;
    readonly maxUpwardMessageNumPerCandidate: u32;
    readonly hrmpMaxMessageNumPerCandidate: u32;
    readonly validationUpgradeCooldown: u32;
    readonly validationUpgradeDelay: u32;
  }

  /** @name PolkadotCorePrimitivesOutboundHrmpMessage (95) */
  interface PolkadotCorePrimitivesOutboundHrmpMessage extends Struct {
    readonly recipient: u32;
    readonly data: Bytes;
  }

  /** @name CumulusPalletParachainSystemCodeUpgradeAuthorization (96) */
  interface CumulusPalletParachainSystemCodeUpgradeAuthorization
    extends Struct {
    readonly codeHash: H256;
    readonly checkVersion: bool;
  }

  /** @name CumulusPalletParachainSystemCall (97) */
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
    readonly type:
      | "SetValidationData"
      | "SudoSendUpwardMessage"
      | "AuthorizeUpgrade"
      | "EnactAuthorizedUpgrade";
  }

  /** @name CumulusPrimitivesParachainInherentParachainInherentData (98) */
  interface CumulusPrimitivesParachainInherentParachainInherentData
    extends Struct {
    readonly validationData: PolkadotPrimitivesV4PersistedValidationData;
    readonly relayChainState: SpTrieStorageProof;
    readonly downwardMessages: Vec<PolkadotCorePrimitivesInboundDownwardMessage>;
    readonly horizontalMessages: BTreeMap<
      u32,
      Vec<PolkadotCorePrimitivesInboundHrmpMessage>
    >;
  }

  /** @name PolkadotCorePrimitivesInboundDownwardMessage (100) */
  interface PolkadotCorePrimitivesInboundDownwardMessage extends Struct {
    readonly sentAt: u32;
    readonly msg: Bytes;
  }

  /** @name PolkadotCorePrimitivesInboundHrmpMessage (103) */
  interface PolkadotCorePrimitivesInboundHrmpMessage extends Struct {
    readonly sentAt: u32;
    readonly data: Bytes;
  }

  /** @name CumulusPalletParachainSystemError (106) */
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

  /** @name PalletTimestampCall (107) */
  interface PalletTimestampCall extends Enum {
    readonly isSet: boolean;
    readonly asSet: {
      readonly now: Compact<u64>;
    } & Struct;
    readonly type: "Set";
  }

  /** @name ParachainInfoCall (108) */
  type ParachainInfoCall = Null;

  /** @name PalletSudoCall (109) */
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

  /** @name PalletUtilityCall (111) */
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
    readonly type:
      | "Batch"
      | "AsDerivative"
      | "BatchAll"
      | "DispatchAs"
      | "ForceBatch"
      | "WithWeight";
  }

  /** @name DanceboxRuntimeOriginCaller (113) */
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

  /** @name FrameSupportDispatchRawOrigin (114) */
  interface FrameSupportDispatchRawOrigin extends Enum {
    readonly isRoot: boolean;
    readonly isSigned: boolean;
    readonly asSigned: AccountId32;
    readonly isNone: boolean;
    readonly type: "Root" | "Signed" | "None";
  }

  /** @name SpCoreVoid (115) */
  type SpCoreVoid = Null;

  /** @name PalletProxyCall (116) */
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

  /** @name PalletMaintenanceModeCall (121) */
  interface PalletMaintenanceModeCall extends Enum {
    readonly isEnterMaintenanceMode: boolean;
    readonly isResumeNormalOperation: boolean;
    readonly type: "EnterMaintenanceMode" | "ResumeNormalOperation";
  }

  /** @name PalletBalancesCall (122) */
  interface PalletBalancesCall extends Enum {
    readonly isTransferAllowDeath: boolean;
    readonly asTransferAllowDeath: {
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
    } & Struct;
    readonly isSetBalanceDeprecated: boolean;
    readonly asSetBalanceDeprecated: {
      readonly who: MultiAddress;
      readonly newFree: Compact<u128>;
      readonly oldReserved: Compact<u128>;
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
    readonly isTransfer: boolean;
    readonly asTransfer: {
      readonly dest: MultiAddress;
      readonly value: Compact<u128>;
    } & Struct;
    readonly isForceSetBalance: boolean;
    readonly asForceSetBalance: {
      readonly who: MultiAddress;
      readonly newFree: Compact<u128>;
    } & Struct;
    readonly type:
      | "TransferAllowDeath"
      | "SetBalanceDeprecated"
      | "ForceTransfer"
      | "TransferKeepAlive"
      | "TransferAll"
      | "ForceUnreserve"
      | "UpgradeAccounts"
      | "Transfer"
      | "ForceSetBalance";
  }

  /** @name PalletRegistrarCall (124) */
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
    readonly isSetBootNodes: boolean;
    readonly asSetBootNodes: {
      readonly paraId: u32;
      readonly bootNodes: Vec<Bytes>;
    } & Struct;
    readonly type:
      | "Register"
      | "Deregister"
      | "MarkValidForCollating"
      | "SetBootNodes";
  }

  /** @name TpContainerChainGenesisDataContainerChainGenesisData (125) */
  interface TpContainerChainGenesisDataContainerChainGenesisData
    extends Struct {
    readonly storage: Vec<TpContainerChainGenesisDataContainerChainGenesisDataItem>;
    readonly name: Bytes;
    readonly id: Bytes;
    readonly forkId: Option<Bytes>;
    readonly extensions: Bytes;
    readonly properties: TpContainerChainGenesisDataProperties;
  }

  /** @name TpContainerChainGenesisDataContainerChainGenesisDataItem (127) */
  interface TpContainerChainGenesisDataContainerChainGenesisDataItem
    extends Struct {
    readonly key: Bytes;
    readonly value: Bytes;
  }

  /** @name TpContainerChainGenesisDataProperties (129) */
  interface TpContainerChainGenesisDataProperties extends Struct {
    readonly tokenMetadata: TpContainerChainGenesisDataTokenMetadata;
    readonly isEthereum: bool;
  }

  /** @name TpContainerChainGenesisDataTokenMetadata (130) */
  interface TpContainerChainGenesisDataTokenMetadata extends Struct {
    readonly tokenSymbol: Bytes;
    readonly ss58Format: u32;
    readonly tokenDecimals: u32;
  }

  /** @name PalletConfigurationCall (135) */
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
    readonly isSetBypassConsistencyCheck: boolean;
    readonly asSetBypassConsistencyCheck: {
      readonly new_: bool;
    } & Struct;
    readonly type:
      | "SetMaxCollators"
      | "SetMinOrchestratorCollators"
      | "SetMaxOrchestratorCollators"
      | "SetCollatorsPerContainer"
      | "SetBypassConsistencyCheck";
  }

  /** @name PalletCollatorAssignmentCall (136) */
  type PalletCollatorAssignmentCall = Null;

  /** @name PalletAuthorNotingCall (137) */
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
    readonly type: "SetLatestAuthorData" | "SetAuthor";
  }

  /** @name TpAuthorNotingInherentOwnParachainInherentData (138) */
  interface TpAuthorNotingInherentOwnParachainInherentData extends Struct {
    readonly relayStorageProof: SpTrieStorageProof;
  }

  /** @name PalletAuthorityAssignmentCall (139) */
  type PalletAuthorityAssignmentCall = Null;

  /** @name PalletCollatorSelectionCall (140) */
  interface PalletCollatorSelectionCall extends Enum {
    readonly isSetInvulnerables: boolean;
    readonly asSetInvulnerables: {
      readonly new_: Vec<AccountId32>;
    } & Struct;
    readonly isSetDesiredCandidates: boolean;
    readonly asSetDesiredCandidates: {
      readonly max: u32;
    } & Struct;
    readonly isSetCandidacyBond: boolean;
    readonly asSetCandidacyBond: {
      readonly bond: u128;
    } & Struct;
    readonly isRegisterAsCandidate: boolean;
    readonly isLeaveIntent: boolean;
    readonly type:
      | "SetInvulnerables"
      | "SetDesiredCandidates"
      | "SetCandidacyBond"
      | "RegisterAsCandidate"
      | "LeaveIntent";
  }

  /** @name PalletSessionCall (141) */
  interface PalletSessionCall extends Enum {
    readonly isSetKeys: boolean;
    readonly asSetKeys: {
      readonly keys_: DanceboxRuntimeSessionKeys;
      readonly proof: Bytes;
    } & Struct;
    readonly isPurgeKeys: boolean;
    readonly type: "SetKeys" | "PurgeKeys";
  }

  /** @name DanceboxRuntimeSessionKeys (142) */
  interface DanceboxRuntimeSessionKeys extends Struct {
    readonly nimbus: NimbusPrimitivesNimbusCryptoPublic;
  }

  /** @name NimbusPrimitivesNimbusCryptoPublic (143) */
  interface NimbusPrimitivesNimbusCryptoPublic extends SpCoreSr25519Public {}

  /** @name SpCoreSr25519Public (144) */
  interface SpCoreSr25519Public extends U8aFixed {}

  /** @name PalletAuthorInherentCall (145) */
  interface PalletAuthorInherentCall extends Enum {
    readonly isKickOffAuthorshipValidation: boolean;
    readonly type: "KickOffAuthorshipValidation";
  }

  /** @name PalletInvulnerablesCall (146) */
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
    readonly type:
      | "SetInvulnerables"
      | "AddInvulnerable"
      | "RemoveInvulnerable";
  }

  /** @name PalletRootTestingCall (147) */
  interface PalletRootTestingCall extends Enum {
    readonly isFillBlock: boolean;
    readonly asFillBlock: {
      readonly ratio: Perbill;
    } & Struct;
    readonly type: "FillBlock";
  }

  /** @name PalletSudoError (149) */
  interface PalletSudoError extends Enum {
    readonly isRequireSudo: boolean;
    readonly type: "RequireSudo";
  }

  /** @name PalletUtilityError (150) */
  interface PalletUtilityError extends Enum {
    readonly isTooManyCalls: boolean;
    readonly type: "TooManyCalls";
  }

  /** @name PalletProxyProxyDefinition (153) */
  interface PalletProxyProxyDefinition extends Struct {
    readonly delegate: AccountId32;
    readonly proxyType: DanceboxRuntimeProxyType;
    readonly delay: u32;
  }

  /** @name PalletProxyAnnouncement (157) */
  interface PalletProxyAnnouncement extends Struct {
    readonly real: AccountId32;
    readonly callHash: H256;
    readonly height: u32;
  }

  /** @name PalletProxyError (159) */
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

  /** @name PalletMigrationsError (160) */
  interface PalletMigrationsError extends Enum {
    readonly isPreimageMissing: boolean;
    readonly isWrongUpperBound: boolean;
    readonly isPreimageIsTooBig: boolean;
    readonly isPreimageAlreadyExists: boolean;
    readonly type:
      | "PreimageMissing"
      | "WrongUpperBound"
      | "PreimageIsTooBig"
      | "PreimageAlreadyExists";
  }

  /** @name PalletMaintenanceModeError (161) */
  interface PalletMaintenanceModeError extends Enum {
    readonly isAlreadyInMaintenanceMode: boolean;
    readonly isNotInMaintenanceMode: boolean;
    readonly type: "AlreadyInMaintenanceMode" | "NotInMaintenanceMode";
  }

  /** @name PalletBalancesBalanceLock (163) */
  interface PalletBalancesBalanceLock extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
    readonly reasons: PalletBalancesReasons;
  }

  /** @name PalletBalancesReasons (164) */
  interface PalletBalancesReasons extends Enum {
    readonly isFee: boolean;
    readonly isMisc: boolean;
    readonly isAll: boolean;
    readonly type: "Fee" | "Misc" | "All";
  }

  /** @name PalletBalancesReserveData (167) */
  interface PalletBalancesReserveData extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name PalletBalancesIdAmount (170) */
  interface PalletBalancesIdAmount extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name PalletBalancesError (172) */
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

  /** @name PalletTransactionPaymentReleases (174) */
  interface PalletTransactionPaymentReleases extends Enum {
    readonly isV1Ancient: boolean;
    readonly isV2: boolean;
    readonly type: "V1Ancient" | "V2";
  }

  /** @name PalletRegistrarDepositInfo (179) */
  interface PalletRegistrarDepositInfo extends Struct {
    readonly creator: AccountId32;
    readonly deposit: u128;
  }

  /** @name PalletRegistrarError (180) */
  interface PalletRegistrarError extends Enum {
    readonly isParaIdAlreadyRegistered: boolean;
    readonly isParaIdNotRegistered: boolean;
    readonly isParaIdListFull: boolean;
    readonly isGenesisDataTooBig: boolean;
    readonly isParaIdNotInPendingVerification: boolean;
    readonly isNotSufficientDeposit: boolean;
    readonly type:
      | "ParaIdAlreadyRegistered"
      | "ParaIdNotRegistered"
      | "ParaIdListFull"
      | "GenesisDataTooBig"
      | "ParaIdNotInPendingVerification"
      | "NotSufficientDeposit";
  }

  /** @name PalletConfigurationHostConfiguration (181) */
  interface PalletConfigurationHostConfiguration extends Struct {
    readonly maxCollators: u32;
    readonly minOrchestratorCollators: u32;
    readonly maxOrchestratorCollators: u32;
    readonly collatorsPerContainer: u32;
  }

  /** @name PalletConfigurationError (184) */
  interface PalletConfigurationError extends Enum {
    readonly isInvalidNewValue: boolean;
    readonly type: "InvalidNewValue";
  }

  /** @name TpCollatorAssignmentAssignedCollatorsAccountId32 (185) */
  interface TpCollatorAssignmentAssignedCollatorsAccountId32 extends Struct {
    readonly orchestratorChain: Vec<AccountId32>;
    readonly containerChains: BTreeMap<u32, Vec<AccountId32>>;
  }

  /** @name PalletInitializerBufferedSessionChange (190) */
  interface PalletInitializerBufferedSessionChange extends Struct {
    readonly changed: bool;
    readonly validators: Vec<
      ITuple<[AccountId32, NimbusPrimitivesNimbusCryptoPublic]>
    >;
    readonly queued: Vec<
      ITuple<[AccountId32, NimbusPrimitivesNimbusCryptoPublic]>
    >;
    readonly sessionIndex: u32;
  }

  /** @name PalletAuthorNotingContainerChainBlockInfo (193) */
  interface PalletAuthorNotingContainerChainBlockInfo extends Struct {
    readonly blockNumber: u32;
    readonly author: AccountId32;
  }

  /** @name PalletAuthorNotingError (194) */
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

  /** @name TpCollatorAssignmentAssignedCollatorsPublic (195) */
  interface TpCollatorAssignmentAssignedCollatorsPublic extends Struct {
    readonly orchestratorChain: Vec<NimbusPrimitivesNimbusCryptoPublic>;
    readonly containerChains: BTreeMap<
      u32,
      Vec<NimbusPrimitivesNimbusCryptoPublic>
    >;
  }

  /** @name PalletCollatorSelectionCandidateInfo (202) */
  interface PalletCollatorSelectionCandidateInfo extends Struct {
    readonly who: AccountId32;
    readonly deposit: u128;
  }

  /** @name PalletCollatorSelectionError (204) */
  interface PalletCollatorSelectionError extends Enum {
    readonly isTooManyCandidates: boolean;
    readonly isTooFewCandidates: boolean;
    readonly isUnknown: boolean;
    readonly isPermission: boolean;
    readonly isAlreadyCandidate: boolean;
    readonly isNotCandidate: boolean;
    readonly isTooManyInvulnerables: boolean;
    readonly isAlreadyInvulnerable: boolean;
    readonly isNoAssociatedValidatorId: boolean;
    readonly isValidatorNotRegistered: boolean;
    readonly type:
      | "TooManyCandidates"
      | "TooFewCandidates"
      | "Unknown"
      | "Permission"
      | "AlreadyCandidate"
      | "NotCandidate"
      | "TooManyInvulnerables"
      | "AlreadyInvulnerable"
      | "NoAssociatedValidatorId"
      | "ValidatorNotRegistered";
  }

  /** @name SpCoreCryptoKeyTypeId (209) */
  interface SpCoreCryptoKeyTypeId extends U8aFixed {}

  /** @name PalletSessionError (210) */
  interface PalletSessionError extends Enum {
    readonly isInvalidProof: boolean;
    readonly isNoAssociatedValidatorId: boolean;
    readonly isDuplicatedKey: boolean;
    readonly isNoKeys: boolean;
    readonly isNoAccount: boolean;
    readonly type:
      | "InvalidProof"
      | "NoAssociatedValidatorId"
      | "DuplicatedKey"
      | "NoKeys"
      | "NoAccount";
  }

  /** @name PalletAuthorInherentError (214) */
  interface PalletAuthorInherentError extends Enum {
    readonly isAuthorAlreadySet: boolean;
    readonly isNoAccountId: boolean;
    readonly isCannotBeAuthor: boolean;
    readonly type: "AuthorAlreadySet" | "NoAccountId" | "CannotBeAuthor";
  }

  /** @name PalletInvulnerablesError (215) */
  interface PalletInvulnerablesError extends Enum {
    readonly isTooManyInvulnerables: boolean;
    readonly isAlreadyInvulnerable: boolean;
    readonly isNotInvulnerable: boolean;
    readonly type:
      | "TooManyInvulnerables"
      | "AlreadyInvulnerable"
      | "NotInvulnerable";
  }

  /** @name SpRuntimeMultiSignature (217) */
  interface SpRuntimeMultiSignature extends Enum {
    readonly isEd25519: boolean;
    readonly asEd25519: SpCoreEd25519Signature;
    readonly isSr25519: boolean;
    readonly asSr25519: SpCoreSr25519Signature;
    readonly isEcdsa: boolean;
    readonly asEcdsa: SpCoreEcdsaSignature;
    readonly type: "Ed25519" | "Sr25519" | "Ecdsa";
  }

  /** @name SpCoreEd25519Signature (218) */
  interface SpCoreEd25519Signature extends U8aFixed {}

  /** @name SpCoreSr25519Signature (220) */
  interface SpCoreSr25519Signature extends U8aFixed {}

  /** @name SpCoreEcdsaSignature (221) */
  interface SpCoreEcdsaSignature extends U8aFixed {}

  /** @name FrameSystemExtensionsCheckNonZeroSender (224) */
  type FrameSystemExtensionsCheckNonZeroSender = Null;

  /** @name FrameSystemExtensionsCheckSpecVersion (225) */
  type FrameSystemExtensionsCheckSpecVersion = Null;

  /** @name FrameSystemExtensionsCheckTxVersion (226) */
  type FrameSystemExtensionsCheckTxVersion = Null;

  /** @name FrameSystemExtensionsCheckGenesis (227) */
  type FrameSystemExtensionsCheckGenesis = Null;

  /** @name FrameSystemExtensionsCheckNonce (230) */
  interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

  /** @name FrameSystemExtensionsCheckWeight (231) */
  type FrameSystemExtensionsCheckWeight = Null;

  /** @name PalletTransactionPaymentChargeTransactionPayment (232) */
  interface PalletTransactionPaymentChargeTransactionPayment
    extends Compact<u128> {}

  /** @name DanceboxRuntimeRuntime (233) */
  type DanceboxRuntimeRuntime = Null;
} // declare module
