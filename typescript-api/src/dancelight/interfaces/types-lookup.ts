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

    /** @name PalletBalancesEvent (31) */
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

    /** @name FrameSupportTokensMiscBalanceStatus (32) */
    interface FrameSupportTokensMiscBalanceStatus extends Enum {
        readonly isFree: boolean;
        readonly isReserved: boolean;
        readonly type: "Free" | "Reserved";
    }

    /** @name PalletParametersEvent (33) */
    interface PalletParametersEvent extends Enum {
        readonly isUpdated: boolean;
        readonly asUpdated: {
            readonly key: DancelightRuntimeRuntimeParametersKey;
            readonly oldValue: Option<DancelightRuntimeRuntimeParametersValue>;
            readonly newValue: Option<DancelightRuntimeRuntimeParametersValue>;
        } & Struct;
        readonly type: "Updated";
    }

    /** @name DancelightRuntimeRuntimeParametersKey (34) */
    interface DancelightRuntimeRuntimeParametersKey extends Enum {
        readonly isPreimage: boolean;
        readonly asPreimage: DancelightRuntimeDynamicParamsPreimageParametersKey;
        readonly type: "Preimage";
    }

    /** @name DancelightRuntimeDynamicParamsPreimageParametersKey (35) */
    interface DancelightRuntimeDynamicParamsPreimageParametersKey extends Enum {
        readonly isBaseDeposit: boolean;
        readonly isByteDeposit: boolean;
        readonly type: "BaseDeposit" | "ByteDeposit";
    }

    /** @name DancelightRuntimeDynamicParamsPreimageBaseDeposit (36) */
    type DancelightRuntimeDynamicParamsPreimageBaseDeposit = Null;

    /** @name DancelightRuntimeDynamicParamsPreimageByteDeposit (37) */
    type DancelightRuntimeDynamicParamsPreimageByteDeposit = Null;

    /** @name DancelightRuntimeRuntimeParametersValue (39) */
    interface DancelightRuntimeRuntimeParametersValue extends Enum {
        readonly isPreimage: boolean;
        readonly asPreimage: DancelightRuntimeDynamicParamsPreimageParametersValue;
        readonly type: "Preimage";
    }

    /** @name DancelightRuntimeDynamicParamsPreimageParametersValue (40) */
    interface DancelightRuntimeDynamicParamsPreimageParametersValue extends Enum {
        readonly isBaseDeposit: boolean;
        readonly asBaseDeposit: u128;
        readonly isByteDeposit: boolean;
        readonly asByteDeposit: u128;
        readonly type: "BaseDeposit" | "ByteDeposit";
    }

    /** @name PalletTransactionPaymentEvent (41) */
    interface PalletTransactionPaymentEvent extends Enum {
        readonly isTransactionFeePaid: boolean;
        readonly asTransactionFeePaid: {
            readonly who: AccountId32;
            readonly actualFee: u128;
            readonly tip: u128;
        } & Struct;
        readonly type: "TransactionFeePaid";
    }

    /** @name PalletOffencesEvent (42) */
    interface PalletOffencesEvent extends Enum {
        readonly isOffence: boolean;
        readonly asOffence: {
            readonly kind: U8aFixed;
            readonly timeslot: Bytes;
        } & Struct;
        readonly type: "Offence";
    }

    /** @name PalletRegistrarEvent (44) */
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

    /** @name PalletInvulnerablesEvent (46) */
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

    /** @name PalletCollatorAssignmentEvent (47) */
    interface PalletCollatorAssignmentEvent extends Enum {
        readonly isNewPendingAssignment: boolean;
        readonly asNewPendingAssignment: {
            readonly randomSeed: U8aFixed;
            readonly fullRotation: bool;
            readonly targetSession: u32;
        } & Struct;
        readonly type: "NewPendingAssignment";
    }

    /** @name PalletAuthorNotingEvent (48) */
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

    /** @name PalletServicesPaymentEvent (50) */
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

    /** @name PalletDataPreserversEvent (53) */
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

    /** @name PalletExternalValidatorsEvent (54) */
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
        readonly type: "WhitelistedValidatorAdded" | "WhitelistedValidatorRemoved" | "NewEra" | "ForceEra";
    }

    /** @name PalletExternalValidatorsForcing (55) */
    interface PalletExternalValidatorsForcing extends Enum {
        readonly isNotForcing: boolean;
        readonly isForceNew: boolean;
        readonly isForceNone: boolean;
        readonly isForceAlways: boolean;
        readonly type: "NotForcing" | "ForceNew" | "ForceNone" | "ForceAlways";
    }

    /** @name PalletSessionEvent (56) */
    interface PalletSessionEvent extends Enum {
        readonly isNewSession: boolean;
        readonly asNewSession: {
            readonly sessionIndex: u32;
        } & Struct;
        readonly type: "NewSession";
    }

    /** @name PalletGrandpaEvent (57) */
    interface PalletGrandpaEvent extends Enum {
        readonly isNewAuthorities: boolean;
        readonly asNewAuthorities: {
            readonly authoritySet: Vec<ITuple<[SpConsensusGrandpaAppPublic, u64]>>;
        } & Struct;
        readonly isPaused: boolean;
        readonly isResumed: boolean;
        readonly type: "NewAuthorities" | "Paused" | "Resumed";
    }

    /** @name SpConsensusGrandpaAppPublic (60) */
    interface SpConsensusGrandpaAppPublic extends U8aFixed {}

    /** @name PalletInflationRewardsEvent (61) */
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

    /** @name PalletTreasuryEvent (62) */
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

    /** @name PalletConvictionVotingEvent (64) */
    interface PalletConvictionVotingEvent extends Enum {
        readonly isDelegated: boolean;
        readonly asDelegated: ITuple<[AccountId32, AccountId32]>;
        readonly isUndelegated: boolean;
        readonly asUndelegated: AccountId32;
        readonly type: "Delegated" | "Undelegated";
    }

    /** @name PalletReferendaEvent (65) */
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

    /** @name FrameSupportPreimagesBounded (67) */
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

    /** @name FrameSystemCall (69) */
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

    /** @name PalletBabeCall (73) */
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

    /** @name SpConsensusSlotsEquivocationProof (74) */
    interface SpConsensusSlotsEquivocationProof extends Struct {
        readonly offender: SpConsensusBabeAppPublic;
        readonly slot: u64;
        readonly firstHeader: SpRuntimeHeader;
        readonly secondHeader: SpRuntimeHeader;
    }

    /** @name SpRuntimeHeader (75) */
    interface SpRuntimeHeader extends Struct {
        readonly parentHash: H256;
        readonly number: Compact<u32>;
        readonly stateRoot: H256;
        readonly extrinsicsRoot: H256;
        readonly digest: SpRuntimeDigest;
    }

    /** @name SpConsensusBabeAppPublic (77) */
    interface SpConsensusBabeAppPublic extends U8aFixed {}

    /** @name SpSessionMembershipProof (78) */
    interface SpSessionMembershipProof extends Struct {
        readonly session: u32;
        readonly trieNodes: Vec<Bytes>;
        readonly validatorCount: u32;
    }

    /** @name SpConsensusBabeDigestsNextConfigDescriptor (79) */
    interface SpConsensusBabeDigestsNextConfigDescriptor extends Enum {
        readonly isV1: boolean;
        readonly asV1: {
            readonly c: ITuple<[u64, u64]>;
            readonly allowedSlots: SpConsensusBabeAllowedSlots;
        } & Struct;
        readonly type: "V1";
    }

    /** @name SpConsensusBabeAllowedSlots (81) */
    interface SpConsensusBabeAllowedSlots extends Enum {
        readonly isPrimarySlots: boolean;
        readonly isPrimaryAndSecondaryPlainSlots: boolean;
        readonly isPrimaryAndSecondaryVRFSlots: boolean;
        readonly type: "PrimarySlots" | "PrimaryAndSecondaryPlainSlots" | "PrimaryAndSecondaryVRFSlots";
    }

    /** @name PalletTimestampCall (82) */
    interface PalletTimestampCall extends Enum {
        readonly isSet: boolean;
        readonly asSet: {
            readonly now: Compact<u64>;
        } & Struct;
        readonly type: "Set";
    }

    /** @name PalletBalancesCall (83) */
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

    /** @name PalletBalancesAdjustmentDirection (89) */
    interface PalletBalancesAdjustmentDirection extends Enum {
        readonly isIncrease: boolean;
        readonly isDecrease: boolean;
        readonly type: "Increase" | "Decrease";
    }

    /** @name PalletParametersCall (90) */
    interface PalletParametersCall extends Enum {
        readonly isSetParameter: boolean;
        readonly asSetParameter: {
            readonly keyValue: DancelightRuntimeRuntimeParameters;
        } & Struct;
        readonly type: "SetParameter";
    }

    /** @name DancelightRuntimeRuntimeParameters (91) */
    interface DancelightRuntimeRuntimeParameters extends Enum {
        readonly isPreimage: boolean;
        readonly asPreimage: DancelightRuntimeDynamicParamsPreimageParameters;
        readonly type: "Preimage";
    }

    /** @name DancelightRuntimeDynamicParamsPreimageParameters (92) */
    interface DancelightRuntimeDynamicParamsPreimageParameters extends Enum {
        readonly isBaseDeposit: boolean;
        readonly asBaseDeposit: ITuple<[DancelightRuntimeDynamicParamsPreimageBaseDeposit, Option<u128>]>;
        readonly isByteDeposit: boolean;
        readonly asByteDeposit: ITuple<[DancelightRuntimeDynamicParamsPreimageByteDeposit, Option<u128>]>;
        readonly type: "BaseDeposit" | "ByteDeposit";
    }

    /** @name PalletRegistrarCall (93) */
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

    /** @name DpContainerChainGenesisDataContainerChainGenesisData (94) */
    interface DpContainerChainGenesisDataContainerChainGenesisData extends Struct {
        readonly storage: Vec<DpContainerChainGenesisDataContainerChainGenesisDataItem>;
        readonly name: Bytes;
        readonly id: Bytes;
        readonly forkId: Option<Bytes>;
        readonly extensions: Bytes;
        readonly properties: DpContainerChainGenesisDataProperties;
    }

    /** @name DpContainerChainGenesisDataContainerChainGenesisDataItem (96) */
    interface DpContainerChainGenesisDataContainerChainGenesisDataItem extends Struct {
        readonly key: Bytes;
        readonly value: Bytes;
    }

    /** @name DpContainerChainGenesisDataProperties (98) */
    interface DpContainerChainGenesisDataProperties extends Struct {
        readonly tokenMetadata: DpContainerChainGenesisDataTokenMetadata;
        readonly isEthereum: bool;
    }

    /** @name DpContainerChainGenesisDataTokenMetadata (99) */
    interface DpContainerChainGenesisDataTokenMetadata extends Struct {
        readonly tokenSymbol: Bytes;
        readonly ss58Format: u32;
        readonly tokenDecimals: u32;
    }

    /** @name TpTraitsSlotFrequency (103) */
    interface TpTraitsSlotFrequency extends Struct {
        readonly min: u32;
        readonly max: u32;
    }

    /** @name TpTraitsParathreadParams (105) */
    interface TpTraitsParathreadParams extends Struct {
        readonly slotFrequency: TpTraitsSlotFrequency;
    }

    /** @name SpTrieStorageProof (106) */
    interface SpTrieStorageProof extends Struct {
        readonly trieNodes: BTreeSet<Bytes>;
    }

    /** @name SpRuntimeMultiSignature (108) */
    interface SpRuntimeMultiSignature extends Enum {
        readonly isEd25519: boolean;
        readonly asEd25519: U8aFixed;
        readonly isSr25519: boolean;
        readonly asSr25519: U8aFixed;
        readonly isEcdsa: boolean;
        readonly asEcdsa: U8aFixed;
        readonly type: "Ed25519" | "Sr25519" | "Ecdsa";
    }

    /** @name PalletConfigurationCall (111) */
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
            | "SetBypassConsistencyCheck";
    }

    /** @name PalletInvulnerablesCall (114) */
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

    /** @name PalletCollatorAssignmentCall (115) */
    type PalletCollatorAssignmentCall = Null;

    /** @name PalletAuthorityAssignmentCall (116) */
    type PalletAuthorityAssignmentCall = Null;

    /** @name PalletAuthorNotingCall (117) */
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

    /** @name PalletServicesPaymentCall (118) */
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

    /** @name PalletDataPreserversCall (119) */
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
            readonly assignerParam: DancelightRuntimePreserversAssignmentPaymentExtra;
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
            readonly assignmentWitness: DancelightRuntimePreserversAssignmentPaymentWitness;
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

    /** @name PalletDataPreserversProfile (120) */
    interface PalletDataPreserversProfile extends Struct {
        readonly url: Bytes;
        readonly paraIds: PalletDataPreserversParaIdsFilter;
        readonly mode: PalletDataPreserversProfileMode;
        readonly assignmentRequest: DancelightRuntimePreserversAssignmentPaymentRequest;
    }

    /** @name PalletDataPreserversParaIdsFilter (122) */
    interface PalletDataPreserversParaIdsFilter extends Enum {
        readonly isAnyParaId: boolean;
        readonly isWhitelist: boolean;
        readonly asWhitelist: BTreeSet<u32>;
        readonly isBlacklist: boolean;
        readonly asBlacklist: BTreeSet<u32>;
        readonly type: "AnyParaId" | "Whitelist" | "Blacklist";
    }

    /** @name PalletDataPreserversProfileMode (126) */
    interface PalletDataPreserversProfileMode extends Enum {
        readonly isBootnode: boolean;
        readonly isRpc: boolean;
        readonly asRpc: {
            readonly supportsEthereumRpcs: bool;
        } & Struct;
        readonly type: "Bootnode" | "Rpc";
    }

    /** @name DancelightRuntimePreserversAssignmentPaymentRequest (127) */
    interface DancelightRuntimePreserversAssignmentPaymentRequest extends Enum {
        readonly isFree: boolean;
        readonly type: "Free";
    }

    /** @name DancelightRuntimePreserversAssignmentPaymentExtra (128) */
    interface DancelightRuntimePreserversAssignmentPaymentExtra extends Enum {
        readonly isFree: boolean;
        readonly type: "Free";
    }

    /** @name DancelightRuntimePreserversAssignmentPaymentWitness (129) */
    interface DancelightRuntimePreserversAssignmentPaymentWitness extends Enum {
        readonly isFree: boolean;
        readonly type: "Free";
    }

    /** @name PalletExternalValidatorsCall (130) */
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
        readonly type: "SkipExternalValidators" | "AddWhitelisted" | "RemoveWhitelisted" | "ForceEra";
    }

    /** @name PalletSessionCall (131) */
    interface PalletSessionCall extends Enum {
        readonly isSetKeys: boolean;
        readonly asSetKeys: {
            readonly keys_: DancelightRuntimeSessionKeys;
            readonly proof: Bytes;
        } & Struct;
        readonly isPurgeKeys: boolean;
        readonly type: "SetKeys" | "PurgeKeys";
    }

    /** @name DancelightRuntimeSessionKeys (132) */
    interface DancelightRuntimeSessionKeys extends Struct {
        readonly grandpa: SpConsensusGrandpaAppPublic;
        readonly babe: SpConsensusBabeAppPublic;
        readonly paraValidator: PolkadotPrimitivesV7ValidatorAppPublic;
        readonly paraAssignment: PolkadotPrimitivesV7AssignmentAppPublic;
        readonly authorityDiscovery: SpAuthorityDiscoveryAppPublic;
        readonly beefy: SpConsensusBeefyEcdsaCryptoPublic;
        readonly nimbus: NimbusPrimitivesNimbusCryptoPublic;
    }

    /** @name PolkadotPrimitivesV7ValidatorAppPublic (133) */
    interface PolkadotPrimitivesV7ValidatorAppPublic extends U8aFixed {}

    /** @name PolkadotPrimitivesV7AssignmentAppPublic (134) */
    interface PolkadotPrimitivesV7AssignmentAppPublic extends U8aFixed {}

    /** @name SpAuthorityDiscoveryAppPublic (135) */
    interface SpAuthorityDiscoveryAppPublic extends U8aFixed {}

    /** @name SpConsensusBeefyEcdsaCryptoPublic (136) */
    interface SpConsensusBeefyEcdsaCryptoPublic extends U8aFixed {}

    /** @name NimbusPrimitivesNimbusCryptoPublic (138) */
    interface NimbusPrimitivesNimbusCryptoPublic extends U8aFixed {}

    /** @name PalletGrandpaCall (139) */
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

    /** @name SpConsensusGrandpaEquivocationProof (140) */
    interface SpConsensusGrandpaEquivocationProof extends Struct {
        readonly setId: u64;
        readonly equivocation: SpConsensusGrandpaEquivocation;
    }

    /** @name SpConsensusGrandpaEquivocation (141) */
    interface SpConsensusGrandpaEquivocation extends Enum {
        readonly isPrevote: boolean;
        readonly asPrevote: FinalityGrandpaEquivocationPrevote;
        readonly isPrecommit: boolean;
        readonly asPrecommit: FinalityGrandpaEquivocationPrecommit;
        readonly type: "Prevote" | "Precommit";
    }

    /** @name FinalityGrandpaEquivocationPrevote (142) */
    interface FinalityGrandpaEquivocationPrevote extends Struct {
        readonly roundNumber: u64;
        readonly identity: SpConsensusGrandpaAppPublic;
        readonly first: ITuple<[FinalityGrandpaPrevote, SpConsensusGrandpaAppSignature]>;
        readonly second: ITuple<[FinalityGrandpaPrevote, SpConsensusGrandpaAppSignature]>;
    }

    /** @name FinalityGrandpaPrevote (143) */
    interface FinalityGrandpaPrevote extends Struct {
        readonly targetHash: H256;
        readonly targetNumber: u32;
    }

    /** @name SpConsensusGrandpaAppSignature (144) */
    interface SpConsensusGrandpaAppSignature extends U8aFixed {}

    /** @name FinalityGrandpaEquivocationPrecommit (146) */
    interface FinalityGrandpaEquivocationPrecommit extends Struct {
        readonly roundNumber: u64;
        readonly identity: SpConsensusGrandpaAppPublic;
        readonly first: ITuple<[FinalityGrandpaPrecommit, SpConsensusGrandpaAppSignature]>;
        readonly second: ITuple<[FinalityGrandpaPrecommit, SpConsensusGrandpaAppSignature]>;
    }

    /** @name FinalityGrandpaPrecommit (147) */
    interface FinalityGrandpaPrecommit extends Struct {
        readonly targetHash: H256;
        readonly targetNumber: u32;
    }

    /** @name PalletTreasuryCall (149) */
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

    /** @name PalletConvictionVotingCall (151) */
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

    /** @name PalletConvictionVotingVoteAccountVote (152) */
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

    /** @name PalletConvictionVotingConviction (154) */
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

    /** @name PalletReferendaCall (156) */
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

    /** @name DancelightRuntimeOriginCaller (157) */
    interface DancelightRuntimeOriginCaller extends Enum {
        readonly isSystem: boolean;
        readonly asSystem: FrameSupportDispatchRawOrigin;
        readonly isVoid: boolean;
        readonly isOrigins: boolean;
        readonly asOrigins: DancelightRuntimeGovernanceOriginsPalletCustomOriginsOrigin;
        readonly isParachainsOrigin: boolean;
        readonly asParachainsOrigin: PolkadotRuntimeParachainsOriginPalletOrigin;
        readonly isXcmPallet: boolean;
        readonly asXcmPallet: PalletXcmOrigin;
        readonly type: "System" | "Void" | "Origins" | "ParachainsOrigin" | "XcmPallet";
    }

    /** @name FrameSupportDispatchRawOrigin (158) */
    interface FrameSupportDispatchRawOrigin extends Enum {
        readonly isRoot: boolean;
        readonly isSigned: boolean;
        readonly asSigned: AccountId32;
        readonly isNone: boolean;
        readonly type: "Root" | "Signed" | "None";
    }

    /** @name DancelightRuntimeGovernanceOriginsPalletCustomOriginsOrigin (159) */
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

    /** @name PolkadotRuntimeParachainsOriginPalletOrigin (160) */
    interface PolkadotRuntimeParachainsOriginPalletOrigin extends Enum {
        readonly isParachain: boolean;
        readonly asParachain: u32;
        readonly type: "Parachain";
    }

    /** @name PalletXcmOrigin (161) */
    interface PalletXcmOrigin extends Enum {
        readonly isXcm: boolean;
        readonly asXcm: StagingXcmV4Location;
        readonly isResponse: boolean;
        readonly asResponse: StagingXcmV4Location;
        readonly type: "Xcm" | "Response";
    }

    /** @name StagingXcmV4Location (162) */
    interface StagingXcmV4Location extends Struct {
        readonly parents: u8;
        readonly interior: StagingXcmV4Junctions;
    }

    /** @name StagingXcmV4Junctions (163) */
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

    /** @name StagingXcmV4Junction (165) */
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

    /** @name StagingXcmV4JunctionNetworkId (167) */
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

    /** @name XcmV3JunctionBodyId (168) */
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

    /** @name XcmV3JunctionBodyPart (169) */
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

    /** @name SpCoreVoid (177) */
    type SpCoreVoid = Null;

    /** @name FrameSupportScheduleDispatchTime (178) */
    interface FrameSupportScheduleDispatchTime extends Enum {
        readonly isAt: boolean;
        readonly asAt: u32;
        readonly isAfter: boolean;
        readonly asAfter: u32;
        readonly type: "At" | "After";
    }

    /** @name PalletRankedCollectiveCall (180) */
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

    /** @name PalletWhitelistCall (182) */
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

    /** @name PolkadotRuntimeParachainsConfigurationPalletCall (183) */
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
        readonly isSetMaxAvailabilityTimeouts: boolean;
        readonly asSetMaxAvailabilityTimeouts: {
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
            readonly new_: PolkadotPrimitivesV7AsyncBackingAsyncBackingParams;
        } & Struct;
        readonly isSetExecutorParams: boolean;
        readonly asSetExecutorParams: {
            readonly new_: PolkadotPrimitivesV7ExecutorParams;
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
        readonly isSetOnDemandTtl: boolean;
        readonly asSetOnDemandTtl: {
            readonly new_: u32;
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
            readonly new_: PolkadotPrimitivesV7ApprovalVotingParams;
        } & Struct;
        readonly isSetSchedulerParams: boolean;
        readonly asSetSchedulerParams: {
            readonly new_: PolkadotPrimitivesVstagingSchedulerParams;
        } & Struct;
        readonly type:
            | "SetValidationUpgradeCooldown"
            | "SetValidationUpgradeDelay"
            | "SetCodeRetentionPeriod"
            | "SetMaxCodeSize"
            | "SetMaxPovSize"
            | "SetMaxHeadDataSize"
            | "SetCoretimeCores"
            | "SetMaxAvailabilityTimeouts"
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
            | "SetOnDemandTtl"
            | "SetMinimumBackingVotes"
            | "SetNodeFeature"
            | "SetApprovalVotingParams"
            | "SetSchedulerParams";
    }

    /** @name PolkadotPrimitivesV7AsyncBackingAsyncBackingParams (184) */
    interface PolkadotPrimitivesV7AsyncBackingAsyncBackingParams extends Struct {
        readonly maxCandidateDepth: u32;
        readonly allowedAncestryLen: u32;
    }

    /** @name PolkadotPrimitivesV7ExecutorParams (185) */
    interface PolkadotPrimitivesV7ExecutorParams extends Vec<PolkadotPrimitivesV7ExecutorParamsExecutorParam> {}

    /** @name PolkadotPrimitivesV7ExecutorParamsExecutorParam (187) */
    interface PolkadotPrimitivesV7ExecutorParamsExecutorParam extends Enum {
        readonly isMaxMemoryPages: boolean;
        readonly asMaxMemoryPages: u32;
        readonly isStackLogicalMax: boolean;
        readonly asStackLogicalMax: u32;
        readonly isStackNativeMax: boolean;
        readonly asStackNativeMax: u32;
        readonly isPrecheckingMaxMemory: boolean;
        readonly asPrecheckingMaxMemory: u64;
        readonly isPvfPrepTimeout: boolean;
        readonly asPvfPrepTimeout: ITuple<[PolkadotPrimitivesV7PvfPrepKind, u64]>;
        readonly isPvfExecTimeout: boolean;
        readonly asPvfExecTimeout: ITuple<[PolkadotPrimitivesV7PvfExecKind, u64]>;
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

    /** @name PolkadotPrimitivesV7PvfPrepKind (188) */
    interface PolkadotPrimitivesV7PvfPrepKind extends Enum {
        readonly isPrecheck: boolean;
        readonly isPrepare: boolean;
        readonly type: "Precheck" | "Prepare";
    }

    /** @name PolkadotPrimitivesV7PvfExecKind (189) */
    interface PolkadotPrimitivesV7PvfExecKind extends Enum {
        readonly isBacking: boolean;
        readonly isApproval: boolean;
        readonly type: "Backing" | "Approval";
    }

    /** @name PolkadotPrimitivesV7ApprovalVotingParams (190) */
    interface PolkadotPrimitivesV7ApprovalVotingParams extends Struct {
        readonly maxApprovalCoalesceCount: u32;
    }

    /** @name PolkadotPrimitivesVstagingSchedulerParams (191) */
    interface PolkadotPrimitivesVstagingSchedulerParams extends Struct {
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

    /** @name PolkadotRuntimeParachainsSharedPalletCall (192) */
    type PolkadotRuntimeParachainsSharedPalletCall = Null;

    /** @name PolkadotRuntimeParachainsInclusionPalletCall (193) */
    type PolkadotRuntimeParachainsInclusionPalletCall = Null;

    /** @name PolkadotRuntimeParachainsParasInherentPalletCall (194) */
    interface PolkadotRuntimeParachainsParasInherentPalletCall extends Enum {
        readonly isEnter: boolean;
        readonly asEnter: {
            readonly data: PolkadotPrimitivesV7InherentData;
        } & Struct;
        readonly type: "Enter";
    }

    /** @name PolkadotPrimitivesV7InherentData (195) */
    interface PolkadotPrimitivesV7InherentData extends Struct {
        readonly bitfields: Vec<PolkadotPrimitivesV7SignedUncheckedSigned>;
        readonly backedCandidates: Vec<PolkadotPrimitivesV7BackedCandidate>;
        readonly disputes: Vec<PolkadotPrimitivesV7DisputeStatementSet>;
        readonly parentHeader: SpRuntimeHeader;
    }

    /** @name PolkadotPrimitivesV7SignedUncheckedSigned (197) */
    interface PolkadotPrimitivesV7SignedUncheckedSigned extends Struct {
        readonly payload: BitVec;
        readonly validatorIndex: u32;
        readonly signature: PolkadotPrimitivesV7ValidatorAppSignature;
    }

    /** @name BitvecOrderLsb0 (200) */
    type BitvecOrderLsb0 = Null;

    /** @name PolkadotPrimitivesV7ValidatorAppSignature (202) */
    interface PolkadotPrimitivesV7ValidatorAppSignature extends U8aFixed {}

    /** @name PolkadotPrimitivesV7BackedCandidate (204) */
    interface PolkadotPrimitivesV7BackedCandidate extends Struct {
        readonly candidate: PolkadotPrimitivesV7CommittedCandidateReceipt;
        readonly validityVotes: Vec<PolkadotPrimitivesV7ValidityAttestation>;
        readonly validatorIndices: BitVec;
    }

    /** @name PolkadotPrimitivesV7CommittedCandidateReceipt (205) */
    interface PolkadotPrimitivesV7CommittedCandidateReceipt extends Struct {
        readonly descriptor: PolkadotPrimitivesV7CandidateDescriptor;
        readonly commitments: PolkadotPrimitivesV7CandidateCommitments;
    }

    /** @name PolkadotPrimitivesV7CandidateDescriptor (206) */
    interface PolkadotPrimitivesV7CandidateDescriptor extends Struct {
        readonly paraId: u32;
        readonly relayParent: H256;
        readonly collator: PolkadotPrimitivesV7CollatorAppPublic;
        readonly persistedValidationDataHash: H256;
        readonly povHash: H256;
        readonly erasureRoot: H256;
        readonly signature: PolkadotPrimitivesV7CollatorAppSignature;
        readonly paraHead: H256;
        readonly validationCodeHash: H256;
    }

    /** @name PolkadotPrimitivesV7CollatorAppPublic (207) */
    interface PolkadotPrimitivesV7CollatorAppPublic extends U8aFixed {}

    /** @name PolkadotPrimitivesV7CollatorAppSignature (208) */
    interface PolkadotPrimitivesV7CollatorAppSignature extends U8aFixed {}

    /** @name PolkadotPrimitivesV7CandidateCommitments (210) */
    interface PolkadotPrimitivesV7CandidateCommitments extends Struct {
        readonly upwardMessages: Vec<Bytes>;
        readonly horizontalMessages: Vec<PolkadotCorePrimitivesOutboundHrmpMessage>;
        readonly newValidationCode: Option<Bytes>;
        readonly headData: Bytes;
        readonly processedDownwardMessages: u32;
        readonly hrmpWatermark: u32;
    }

    /** @name PolkadotCorePrimitivesOutboundHrmpMessage (213) */
    interface PolkadotCorePrimitivesOutboundHrmpMessage extends Struct {
        readonly recipient: u32;
        readonly data: Bytes;
    }

    /** @name PolkadotPrimitivesV7ValidityAttestation (218) */
    interface PolkadotPrimitivesV7ValidityAttestation extends Enum {
        readonly isImplicit: boolean;
        readonly asImplicit: PolkadotPrimitivesV7ValidatorAppSignature;
        readonly isExplicit: boolean;
        readonly asExplicit: PolkadotPrimitivesV7ValidatorAppSignature;
        readonly type: "Implicit" | "Explicit";
    }

    /** @name PolkadotPrimitivesV7DisputeStatementSet (220) */
    interface PolkadotPrimitivesV7DisputeStatementSet extends Struct {
        readonly candidateHash: H256;
        readonly session: u32;
        readonly statements: Vec<
            ITuple<[PolkadotPrimitivesV7DisputeStatement, u32, PolkadotPrimitivesV7ValidatorAppSignature]>
        >;
    }

    /** @name PolkadotPrimitivesV7DisputeStatement (224) */
    interface PolkadotPrimitivesV7DisputeStatement extends Enum {
        readonly isValid: boolean;
        readonly asValid: PolkadotPrimitivesV7ValidDisputeStatementKind;
        readonly isInvalid: boolean;
        readonly asInvalid: PolkadotPrimitivesV7InvalidDisputeStatementKind;
        readonly type: "Valid" | "Invalid";
    }

    /** @name PolkadotPrimitivesV7ValidDisputeStatementKind (225) */
    interface PolkadotPrimitivesV7ValidDisputeStatementKind extends Enum {
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

    /** @name PolkadotPrimitivesV7InvalidDisputeStatementKind (227) */
    interface PolkadotPrimitivesV7InvalidDisputeStatementKind extends Enum {
        readonly isExplicit: boolean;
        readonly type: "Explicit";
    }

    /** @name PolkadotRuntimeParachainsParasPalletCall (228) */
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
            readonly stmt: PolkadotPrimitivesV7PvfCheckStatement;
            readonly signature: PolkadotPrimitivesV7ValidatorAppSignature;
        } & Struct;
        readonly isForceSetMostRecentContext: boolean;
        readonly asForceSetMostRecentContext: {
            readonly para: u32;
            readonly context: u32;
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
            | "ForceSetMostRecentContext";
    }

    /** @name PolkadotPrimitivesV7PvfCheckStatement (229) */
    interface PolkadotPrimitivesV7PvfCheckStatement extends Struct {
        readonly accept: bool;
        readonly subject: H256;
        readonly sessionIndex: u32;
        readonly validatorIndex: u32;
    }

    /** @name PolkadotRuntimeParachainsInitializerPalletCall (230) */
    interface PolkadotRuntimeParachainsInitializerPalletCall extends Enum {
        readonly isForceApprove: boolean;
        readonly asForceApprove: {
            readonly upTo: u32;
        } & Struct;
        readonly type: "ForceApprove";
    }

    /** @name PolkadotRuntimeParachainsHrmpPalletCall (231) */
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

    /** @name PolkadotParachainPrimitivesPrimitivesHrmpChannelId (232) */
    interface PolkadotParachainPrimitivesPrimitivesHrmpChannelId extends Struct {
        readonly sender: u32;
        readonly recipient: u32;
    }

    /** @name PolkadotRuntimeParachainsDisputesPalletCall (233) */
    interface PolkadotRuntimeParachainsDisputesPalletCall extends Enum {
        readonly isForceUnfreeze: boolean;
        readonly type: "ForceUnfreeze";
    }

    /** @name PolkadotRuntimeParachainsDisputesSlashingPalletCall (234) */
    interface PolkadotRuntimeParachainsDisputesSlashingPalletCall extends Enum {
        readonly isReportDisputeLostUnsigned: boolean;
        readonly asReportDisputeLostUnsigned: {
            readonly disputeProof: PolkadotPrimitivesV7SlashingDisputeProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly type: "ReportDisputeLostUnsigned";
    }

    /** @name PolkadotPrimitivesV7SlashingDisputeProof (235) */
    interface PolkadotPrimitivesV7SlashingDisputeProof extends Struct {
        readonly timeSlot: PolkadotPrimitivesV7SlashingDisputesTimeSlot;
        readonly kind: PolkadotPrimitivesV7SlashingSlashingOffenceKind;
        readonly validatorIndex: u32;
        readonly validatorId: PolkadotPrimitivesV7ValidatorAppPublic;
    }

    /** @name PolkadotPrimitivesV7SlashingDisputesTimeSlot (236) */
    interface PolkadotPrimitivesV7SlashingDisputesTimeSlot extends Struct {
        readonly sessionIndex: u32;
        readonly candidateHash: H256;
    }

    /** @name PolkadotPrimitivesV7SlashingSlashingOffenceKind (237) */
    interface PolkadotPrimitivesV7SlashingSlashingOffenceKind extends Enum {
        readonly isForInvalid: boolean;
        readonly isAgainstValid: boolean;
        readonly type: "ForInvalid" | "AgainstValid";
    }

    /** @name PalletMessageQueueCall (238) */
    interface PalletMessageQueueCall extends Enum {
        readonly isReapPage: boolean;
        readonly asReapPage: {
            readonly messageOrigin: PolkadotRuntimeParachainsInclusionAggregateMessageOrigin;
            readonly pageIndex: u32;
        } & Struct;
        readonly isExecuteOverweight: boolean;
        readonly asExecuteOverweight: {
            readonly messageOrigin: PolkadotRuntimeParachainsInclusionAggregateMessageOrigin;
            readonly page: u32;
            readonly index: u32;
            readonly weightLimit: SpWeightsWeightV2Weight;
        } & Struct;
        readonly type: "ReapPage" | "ExecuteOverweight";
    }

    /** @name PolkadotRuntimeParachainsInclusionAggregateMessageOrigin (239) */
    interface PolkadotRuntimeParachainsInclusionAggregateMessageOrigin extends Enum {
        readonly isUmp: boolean;
        readonly asUmp: PolkadotRuntimeParachainsInclusionUmpQueueId;
        readonly type: "Ump";
    }

    /** @name PolkadotRuntimeParachainsInclusionUmpQueueId (240) */
    interface PolkadotRuntimeParachainsInclusionUmpQueueId extends Enum {
        readonly isPara: boolean;
        readonly asPara: u32;
        readonly type: "Para";
    }

    /** @name PolkadotRuntimeParachainsAssignerOnDemandPalletCall (241) */
    interface PolkadotRuntimeParachainsAssignerOnDemandPalletCall extends Enum {
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
        readonly type: "PlaceOrderAllowDeath" | "PlaceOrderKeepAlive";
    }

    /** @name PolkadotRuntimeCommonParasRegistrarPalletCall (242) */
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

    /** @name PalletUtilityCall (243) */
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
        readonly type: "Batch" | "AsDerivative" | "BatchAll" | "DispatchAs" | "ForceBatch" | "WithWeight";
    }

    /** @name PalletIdentityCall (245) */
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

    /** @name PalletIdentityLegacyIdentityInfo (246) */
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

    /** @name PalletIdentityJudgement (283) */
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

    /** @name PalletSchedulerCall (286) */
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

    /** @name PalletProxyCall (289) */
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

    /** @name DancelightRuntimeProxyType (291) */
    interface DancelightRuntimeProxyType extends Enum {
        readonly isAny: boolean;
        readonly isNonTransfer: boolean;
        readonly isGovernance: boolean;
        readonly isIdentityJudgement: boolean;
        readonly isCancelProxy: boolean;
        readonly isAuction: boolean;
        readonly isOnDemandOrdering: boolean;
        readonly type:
            | "Any"
            | "NonTransfer"
            | "Governance"
            | "IdentityJudgement"
            | "CancelProxy"
            | "Auction"
            | "OnDemandOrdering";
    }

    /** @name PalletMultisigCall (292) */
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

    /** @name PalletMultisigTimepoint (294) */
    interface PalletMultisigTimepoint extends Struct {
        readonly height: u32;
        readonly index: u32;
    }

    /** @name PalletPreimageCall (295) */
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

    /** @name PalletAssetRateCall (297) */
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

    /** @name PalletXcmCall (299) */
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

    /** @name XcmVersionedLocation (300) */
    interface XcmVersionedLocation extends Enum {
        readonly isV2: boolean;
        readonly asV2: XcmV2MultiLocation;
        readonly isV3: boolean;
        readonly asV3: StagingXcmV3MultiLocation;
        readonly isV4: boolean;
        readonly asV4: StagingXcmV4Location;
        readonly type: "V2" | "V3" | "V4";
    }

    /** @name XcmV2MultiLocation (301) */
    interface XcmV2MultiLocation extends Struct {
        readonly parents: u8;
        readonly interior: XcmV2MultilocationJunctions;
    }

    /** @name XcmV2MultilocationJunctions (302) */
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

    /** @name XcmV2Junction (303) */
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

    /** @name XcmV2NetworkId (304) */
    interface XcmV2NetworkId extends Enum {
        readonly isAny: boolean;
        readonly isNamed: boolean;
        readonly asNamed: Bytes;
        readonly isPolkadot: boolean;
        readonly isKusama: boolean;
        readonly type: "Any" | "Named" | "Polkadot" | "Kusama";
    }

    /** @name XcmV2BodyId (306) */
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

    /** @name XcmV2BodyPart (307) */
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

    /** @name StagingXcmV3MultiLocation (308) */
    interface StagingXcmV3MultiLocation extends Struct {
        readonly parents: u8;
        readonly interior: XcmV3Junctions;
    }

    /** @name XcmV3Junctions (309) */
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

    /** @name XcmV3Junction (310) */
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

    /** @name XcmV3JunctionNetworkId (312) */
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

    /** @name XcmVersionedXcm (313) */
    interface XcmVersionedXcm extends Enum {
        readonly isV2: boolean;
        readonly asV2: XcmV2Xcm;
        readonly isV3: boolean;
        readonly asV3: XcmV3Xcm;
        readonly isV4: boolean;
        readonly asV4: StagingXcmV4Xcm;
        readonly type: "V2" | "V3" | "V4";
    }

    /** @name XcmV2Xcm (314) */
    interface XcmV2Xcm extends Vec<XcmV2Instruction> {}

    /** @name XcmV2Instruction (316) */
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

    /** @name XcmV2MultiassetMultiAssets (317) */
    interface XcmV2MultiassetMultiAssets extends Vec<XcmV2MultiAsset> {}

    /** @name XcmV2MultiAsset (319) */
    interface XcmV2MultiAsset extends Struct {
        readonly id: XcmV2MultiassetAssetId;
        readonly fun: XcmV2MultiassetFungibility;
    }

    /** @name XcmV2MultiassetAssetId (320) */
    interface XcmV2MultiassetAssetId extends Enum {
        readonly isConcrete: boolean;
        readonly asConcrete: XcmV2MultiLocation;
        readonly isAbstract: boolean;
        readonly asAbstract: Bytes;
        readonly type: "Concrete" | "Abstract";
    }

    /** @name XcmV2MultiassetFungibility (321) */
    interface XcmV2MultiassetFungibility extends Enum {
        readonly isFungible: boolean;
        readonly asFungible: Compact<u128>;
        readonly isNonFungible: boolean;
        readonly asNonFungible: XcmV2MultiassetAssetInstance;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name XcmV2MultiassetAssetInstance (322) */
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

    /** @name XcmV2Response (323) */
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

    /** @name XcmV2TraitsError (326) */
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

    /** @name XcmV2OriginKind (327) */
    interface XcmV2OriginKind extends Enum {
        readonly isNative: boolean;
        readonly isSovereignAccount: boolean;
        readonly isSuperuser: boolean;
        readonly isXcm: boolean;
        readonly type: "Native" | "SovereignAccount" | "Superuser" | "Xcm";
    }

    /** @name XcmDoubleEncoded (328) */
    interface XcmDoubleEncoded extends Struct {
        readonly encoded: Bytes;
    }

    /** @name XcmV2MultiassetMultiAssetFilter (329) */
    interface XcmV2MultiassetMultiAssetFilter extends Enum {
        readonly isDefinite: boolean;
        readonly asDefinite: XcmV2MultiassetMultiAssets;
        readonly isWild: boolean;
        readonly asWild: XcmV2MultiassetWildMultiAsset;
        readonly type: "Definite" | "Wild";
    }

    /** @name XcmV2MultiassetWildMultiAsset (330) */
    interface XcmV2MultiassetWildMultiAsset extends Enum {
        readonly isAll: boolean;
        readonly isAllOf: boolean;
        readonly asAllOf: {
            readonly id: XcmV2MultiassetAssetId;
            readonly fun: XcmV2MultiassetWildFungibility;
        } & Struct;
        readonly type: "All" | "AllOf";
    }

    /** @name XcmV2MultiassetWildFungibility (331) */
    interface XcmV2MultiassetWildFungibility extends Enum {
        readonly isFungible: boolean;
        readonly isNonFungible: boolean;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name XcmV2WeightLimit (332) */
    interface XcmV2WeightLimit extends Enum {
        readonly isUnlimited: boolean;
        readonly isLimited: boolean;
        readonly asLimited: Compact<u64>;
        readonly type: "Unlimited" | "Limited";
    }

    /** @name XcmV3Xcm (333) */
    interface XcmV3Xcm extends Vec<XcmV3Instruction> {}

    /** @name XcmV3Instruction (335) */
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

    /** @name XcmV3MultiassetMultiAssets (336) */
    interface XcmV3MultiassetMultiAssets extends Vec<XcmV3MultiAsset> {}

    /** @name XcmV3MultiAsset (338) */
    interface XcmV3MultiAsset extends Struct {
        readonly id: XcmV3MultiassetAssetId;
        readonly fun: XcmV3MultiassetFungibility;
    }

    /** @name XcmV3MultiassetAssetId (339) */
    interface XcmV3MultiassetAssetId extends Enum {
        readonly isConcrete: boolean;
        readonly asConcrete: StagingXcmV3MultiLocation;
        readonly isAbstract: boolean;
        readonly asAbstract: U8aFixed;
        readonly type: "Concrete" | "Abstract";
    }

    /** @name XcmV3MultiassetFungibility (340) */
    interface XcmV3MultiassetFungibility extends Enum {
        readonly isFungible: boolean;
        readonly asFungible: Compact<u128>;
        readonly isNonFungible: boolean;
        readonly asNonFungible: XcmV3MultiassetAssetInstance;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name XcmV3MultiassetAssetInstance (341) */
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

    /** @name XcmV3Response (342) */
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

    /** @name XcmV3TraitsError (345) */
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

    /** @name XcmV3PalletInfo (347) */
    interface XcmV3PalletInfo extends Struct {
        readonly index: Compact<u32>;
        readonly name: Bytes;
        readonly moduleName: Bytes;
        readonly major: Compact<u32>;
        readonly minor: Compact<u32>;
        readonly patch: Compact<u32>;
    }

    /** @name XcmV3MaybeErrorCode (350) */
    interface XcmV3MaybeErrorCode extends Enum {
        readonly isSuccess: boolean;
        readonly isError: boolean;
        readonly asError: Bytes;
        readonly isTruncatedError: boolean;
        readonly asTruncatedError: Bytes;
        readonly type: "Success" | "Error" | "TruncatedError";
    }

    /** @name XcmV3OriginKind (353) */
    interface XcmV3OriginKind extends Enum {
        readonly isNative: boolean;
        readonly isSovereignAccount: boolean;
        readonly isSuperuser: boolean;
        readonly isXcm: boolean;
        readonly type: "Native" | "SovereignAccount" | "Superuser" | "Xcm";
    }

    /** @name XcmV3QueryResponseInfo (354) */
    interface XcmV3QueryResponseInfo extends Struct {
        readonly destination: StagingXcmV3MultiLocation;
        readonly queryId: Compact<u64>;
        readonly maxWeight: SpWeightsWeightV2Weight;
    }

    /** @name XcmV3MultiassetMultiAssetFilter (355) */
    interface XcmV3MultiassetMultiAssetFilter extends Enum {
        readonly isDefinite: boolean;
        readonly asDefinite: XcmV3MultiassetMultiAssets;
        readonly isWild: boolean;
        readonly asWild: XcmV3MultiassetWildMultiAsset;
        readonly type: "Definite" | "Wild";
    }

    /** @name XcmV3MultiassetWildMultiAsset (356) */
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

    /** @name XcmV3MultiassetWildFungibility (357) */
    interface XcmV3MultiassetWildFungibility extends Enum {
        readonly isFungible: boolean;
        readonly isNonFungible: boolean;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name XcmV3WeightLimit (358) */
    interface XcmV3WeightLimit extends Enum {
        readonly isUnlimited: boolean;
        readonly isLimited: boolean;
        readonly asLimited: SpWeightsWeightV2Weight;
        readonly type: "Unlimited" | "Limited";
    }

    /** @name StagingXcmV4Xcm (359) */
    interface StagingXcmV4Xcm extends Vec<StagingXcmV4Instruction> {}

    /** @name StagingXcmV4Instruction (361) */
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

    /** @name StagingXcmV4AssetAssets (362) */
    interface StagingXcmV4AssetAssets extends Vec<StagingXcmV4Asset> {}

    /** @name StagingXcmV4Asset (364) */
    interface StagingXcmV4Asset extends Struct {
        readonly id: StagingXcmV4AssetAssetId;
        readonly fun: StagingXcmV4AssetFungibility;
    }

    /** @name StagingXcmV4AssetAssetId (365) */
    interface StagingXcmV4AssetAssetId extends StagingXcmV4Location {}

    /** @name StagingXcmV4AssetFungibility (366) */
    interface StagingXcmV4AssetFungibility extends Enum {
        readonly isFungible: boolean;
        readonly asFungible: Compact<u128>;
        readonly isNonFungible: boolean;
        readonly asNonFungible: StagingXcmV4AssetAssetInstance;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name StagingXcmV4AssetAssetInstance (367) */
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

    /** @name StagingXcmV4Response (368) */
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

    /** @name StagingXcmV4PalletInfo (370) */
    interface StagingXcmV4PalletInfo extends Struct {
        readonly index: Compact<u32>;
        readonly name: Bytes;
        readonly moduleName: Bytes;
        readonly major: Compact<u32>;
        readonly minor: Compact<u32>;
        readonly patch: Compact<u32>;
    }

    /** @name StagingXcmV4QueryResponseInfo (374) */
    interface StagingXcmV4QueryResponseInfo extends Struct {
        readonly destination: StagingXcmV4Location;
        readonly queryId: Compact<u64>;
        readonly maxWeight: SpWeightsWeightV2Weight;
    }

    /** @name StagingXcmV4AssetAssetFilter (375) */
    interface StagingXcmV4AssetAssetFilter extends Enum {
        readonly isDefinite: boolean;
        readonly asDefinite: StagingXcmV4AssetAssets;
        readonly isWild: boolean;
        readonly asWild: StagingXcmV4AssetWildAsset;
        readonly type: "Definite" | "Wild";
    }

    /** @name StagingXcmV4AssetWildAsset (376) */
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

    /** @name StagingXcmV4AssetWildFungibility (377) */
    interface StagingXcmV4AssetWildFungibility extends Enum {
        readonly isFungible: boolean;
        readonly isNonFungible: boolean;
        readonly type: "Fungible" | "NonFungible";
    }

    /** @name XcmVersionedAssets (378) */
    interface XcmVersionedAssets extends Enum {
        readonly isV2: boolean;
        readonly asV2: XcmV2MultiassetMultiAssets;
        readonly isV3: boolean;
        readonly asV3: XcmV3MultiassetMultiAssets;
        readonly isV4: boolean;
        readonly asV4: StagingXcmV4AssetAssets;
        readonly type: "V2" | "V3" | "V4";
    }

    /** @name StagingXcmExecutorAssetTransferTransferType (390) */
    interface StagingXcmExecutorAssetTransferTransferType extends Enum {
        readonly isTeleport: boolean;
        readonly isLocalReserve: boolean;
        readonly isDestinationReserve: boolean;
        readonly isRemoteReserve: boolean;
        readonly asRemoteReserve: XcmVersionedLocation;
        readonly type: "Teleport" | "LocalReserve" | "DestinationReserve" | "RemoteReserve";
    }

    /** @name XcmVersionedAssetId (391) */
    interface XcmVersionedAssetId extends Enum {
        readonly isV3: boolean;
        readonly asV3: XcmV3MultiassetAssetId;
        readonly isV4: boolean;
        readonly asV4: StagingXcmV4AssetAssetId;
        readonly type: "V3" | "V4";
    }

    /** @name PalletMigrationsCall (392) */
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

    /** @name PalletMigrationsMigrationCursor (394) */
    interface PalletMigrationsMigrationCursor extends Enum {
        readonly isActive: boolean;
        readonly asActive: PalletMigrationsActiveCursor;
        readonly isStuck: boolean;
        readonly type: "Active" | "Stuck";
    }

    /** @name PalletMigrationsActiveCursor (396) */
    interface PalletMigrationsActiveCursor extends Struct {
        readonly index: u32;
        readonly innerCursor: Option<Bytes>;
        readonly startedAt: u32;
    }

    /** @name PalletMigrationsHistoricCleanupSelector (398) */
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

    /** @name PalletBeefyCall (401) */
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

    /** @name SpConsensusBeefyDoubleVotingProof (402) */
    interface SpConsensusBeefyDoubleVotingProof extends Struct {
        readonly first: SpConsensusBeefyVoteMessage;
        readonly second: SpConsensusBeefyVoteMessage;
    }

    /** @name SpConsensusBeefyEcdsaCryptoSignature (403) */
    interface SpConsensusBeefyEcdsaCryptoSignature extends U8aFixed {}

    /** @name SpConsensusBeefyVoteMessage (404) */
    interface SpConsensusBeefyVoteMessage extends Struct {
        readonly commitment: SpConsensusBeefyCommitment;
        readonly id: SpConsensusBeefyEcdsaCryptoPublic;
        readonly signature: SpConsensusBeefyEcdsaCryptoSignature;
    }

    /** @name SpConsensusBeefyCommitment (405) */
    interface SpConsensusBeefyCommitment extends Struct {
        readonly payload: SpConsensusBeefyPayload;
        readonly blockNumber: u32;
        readonly validatorSetId: u64;
    }

    /** @name SpConsensusBeefyPayload (406) */
    interface SpConsensusBeefyPayload extends Vec<ITuple<[U8aFixed, Bytes]>> {}

    /** @name SpConsensusBeefyForkVotingProof (409) */
    interface SpConsensusBeefyForkVotingProof extends Struct {
        readonly vote: SpConsensusBeefyVoteMessage;
        readonly ancestryProof: SpMmrPrimitivesAncestryProof;
        readonly header: SpRuntimeHeader;
    }

    /** @name SpMmrPrimitivesAncestryProof (410) */
    interface SpMmrPrimitivesAncestryProof extends Struct {
        readonly prevPeaks: Vec<H256>;
        readonly prevLeafCount: u64;
        readonly leafCount: u64;
        readonly items: Vec<ITuple<[u64, H256]>>;
    }

    /** @name SpConsensusBeefyFutureBlockVotingProof (413) */
    interface SpConsensusBeefyFutureBlockVotingProof extends Struct {
        readonly vote: SpConsensusBeefyVoteMessage;
    }

    /** @name SnowbridgePalletEthereumClientCall (414) */
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

    /** @name SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate (415) */
    interface SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate extends Struct {
        readonly header: SnowbridgeBeaconPrimitivesBeaconHeader;
        readonly currentSyncCommittee: SnowbridgeBeaconPrimitivesSyncCommittee;
        readonly currentSyncCommitteeBranch: Vec<H256>;
        readonly validatorsRoot: H256;
        readonly blockRootsRoot: H256;
        readonly blockRootsBranch: Vec<H256>;
    }

    /** @name SnowbridgeBeaconPrimitivesBeaconHeader (416) */
    interface SnowbridgeBeaconPrimitivesBeaconHeader extends Struct {
        readonly slot: u64;
        readonly proposerIndex: u64;
        readonly parentRoot: H256;
        readonly stateRoot: H256;
        readonly bodyRoot: H256;
    }

    /** @name SnowbridgeBeaconPrimitivesSyncCommittee (417) */
    interface SnowbridgeBeaconPrimitivesSyncCommittee extends Struct {
        readonly pubkeys: Vec<SnowbridgeBeaconPrimitivesPublicKey>;
        readonly aggregatePubkey: SnowbridgeBeaconPrimitivesPublicKey;
    }

    /** @name SnowbridgeBeaconPrimitivesPublicKey (419) */
    interface SnowbridgeBeaconPrimitivesPublicKey extends U8aFixed {}

    /** @name SnowbridgeBeaconPrimitivesUpdatesUpdate (421) */
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

    /** @name SnowbridgeBeaconPrimitivesSyncAggregate (422) */
    interface SnowbridgeBeaconPrimitivesSyncAggregate extends Struct {
        readonly syncCommitteeBits: U8aFixed;
        readonly syncCommitteeSignature: SnowbridgeBeaconPrimitivesSignature;
    }

    /** @name SnowbridgeBeaconPrimitivesSignature (423) */
    interface SnowbridgeBeaconPrimitivesSignature extends U8aFixed {}

    /** @name SnowbridgeBeaconPrimitivesUpdatesNextSyncCommitteeUpdate (426) */
    interface SnowbridgeBeaconPrimitivesUpdatesNextSyncCommitteeUpdate extends Struct {
        readonly nextSyncCommittee: SnowbridgeBeaconPrimitivesSyncCommittee;
        readonly nextSyncCommitteeBranch: Vec<H256>;
    }

    /** @name SnowbridgeCoreOperatingModeBasicOperatingMode (427) */
    interface SnowbridgeCoreOperatingModeBasicOperatingMode extends Enum {
        readonly isNormal: boolean;
        readonly isHalted: boolean;
        readonly type: "Normal" | "Halted";
    }

    /** @name PolkadotRuntimeCommonParasSudoWrapperPalletCall (428) */
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

    /** @name PolkadotRuntimeParachainsParasParaGenesisArgs (429) */
    interface PolkadotRuntimeParachainsParasParaGenesisArgs extends Struct {
        readonly genesisHead: Bytes;
        readonly validationCode: Bytes;
        readonly paraKind: bool;
    }

    /** @name PalletRootTestingCall (430) */
    interface PalletRootTestingCall extends Enum {
        readonly isFillBlock: boolean;
        readonly asFillBlock: {
            readonly ratio: Perbill;
        } & Struct;
        readonly isTriggerDefensive: boolean;
        readonly type: "FillBlock" | "TriggerDefensive";
    }

    /** @name PalletSudoCall (431) */
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

    /** @name SpRuntimeBlakeTwo256 (432) */
    type SpRuntimeBlakeTwo256 = Null;

    /** @name PalletConvictionVotingTally (434) */
    interface PalletConvictionVotingTally extends Struct {
        readonly ayes: u128;
        readonly nays: u128;
        readonly support: u128;
    }

    /** @name PalletRankedCollectiveEvent (435) */
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

    /** @name PalletRankedCollectiveVoteRecord (436) */
    interface PalletRankedCollectiveVoteRecord extends Enum {
        readonly isAye: boolean;
        readonly asAye: u32;
        readonly isNay: boolean;
        readonly asNay: u32;
        readonly type: "Aye" | "Nay";
    }

    /** @name PalletRankedCollectiveTally (437) */
    interface PalletRankedCollectiveTally extends Struct {
        readonly bareAyes: u32;
        readonly ayes: u32;
        readonly nays: u32;
    }

    /** @name PalletWhitelistEvent (439) */
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

    /** @name FrameSupportDispatchPostDispatchInfo (441) */
    interface FrameSupportDispatchPostDispatchInfo extends Struct {
        readonly actualWeight: Option<SpWeightsWeightV2Weight>;
        readonly paysFee: FrameSupportDispatchPays;
    }

    /** @name SpRuntimeDispatchErrorWithPostInfo (443) */
    interface SpRuntimeDispatchErrorWithPostInfo extends Struct {
        readonly postInfo: FrameSupportDispatchPostDispatchInfo;
        readonly error: SpRuntimeDispatchError;
    }

    /** @name PolkadotRuntimeParachainsInclusionPalletEvent (444) */
    interface PolkadotRuntimeParachainsInclusionPalletEvent extends Enum {
        readonly isCandidateBacked: boolean;
        readonly asCandidateBacked: ITuple<[PolkadotPrimitivesV7CandidateReceipt, Bytes, u32, u32]>;
        readonly isCandidateIncluded: boolean;
        readonly asCandidateIncluded: ITuple<[PolkadotPrimitivesV7CandidateReceipt, Bytes, u32, u32]>;
        readonly isCandidateTimedOut: boolean;
        readonly asCandidateTimedOut: ITuple<[PolkadotPrimitivesV7CandidateReceipt, Bytes, u32]>;
        readonly isUpwardMessagesReceived: boolean;
        readonly asUpwardMessagesReceived: {
            readonly from: u32;
            readonly count: u32;
        } & Struct;
        readonly type: "CandidateBacked" | "CandidateIncluded" | "CandidateTimedOut" | "UpwardMessagesReceived";
    }

    /** @name PolkadotPrimitivesV7CandidateReceipt (445) */
    interface PolkadotPrimitivesV7CandidateReceipt extends Struct {
        readonly descriptor: PolkadotPrimitivesV7CandidateDescriptor;
        readonly commitmentsHash: H256;
    }

    /** @name PolkadotRuntimeParachainsParasPalletEvent (448) */
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
        readonly type:
            | "CurrentCodeUpdated"
            | "CurrentHeadUpdated"
            | "CodeUpgradeScheduled"
            | "NewHeadNoted"
            | "ActionQueued"
            | "PvfCheckStarted"
            | "PvfCheckAccepted"
            | "PvfCheckRejected";
    }

    /** @name PolkadotRuntimeParachainsHrmpPalletEvent (449) */
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

    /** @name PolkadotRuntimeParachainsDisputesPalletEvent (450) */
    interface PolkadotRuntimeParachainsDisputesPalletEvent extends Enum {
        readonly isDisputeInitiated: boolean;
        readonly asDisputeInitiated: ITuple<[H256, PolkadotRuntimeParachainsDisputesDisputeLocation]>;
        readonly isDisputeConcluded: boolean;
        readonly asDisputeConcluded: ITuple<[H256, PolkadotRuntimeParachainsDisputesDisputeResult]>;
        readonly isRevert: boolean;
        readonly asRevert: u32;
        readonly type: "DisputeInitiated" | "DisputeConcluded" | "Revert";
    }

    /** @name PolkadotRuntimeParachainsDisputesDisputeLocation (451) */
    interface PolkadotRuntimeParachainsDisputesDisputeLocation extends Enum {
        readonly isLocal: boolean;
        readonly isRemote: boolean;
        readonly type: "Local" | "Remote";
    }

    /** @name PolkadotRuntimeParachainsDisputesDisputeResult (452) */
    interface PolkadotRuntimeParachainsDisputesDisputeResult extends Enum {
        readonly isValid: boolean;
        readonly isInvalid: boolean;
        readonly type: "Valid" | "Invalid";
    }

    /** @name PalletMessageQueueEvent (453) */
    interface PalletMessageQueueEvent extends Enum {
        readonly isProcessingFailed: boolean;
        readonly asProcessingFailed: {
            readonly id: H256;
            readonly origin: PolkadotRuntimeParachainsInclusionAggregateMessageOrigin;
            readonly error: FrameSupportMessagesProcessMessageError;
        } & Struct;
        readonly isProcessed: boolean;
        readonly asProcessed: {
            readonly id: H256;
            readonly origin: PolkadotRuntimeParachainsInclusionAggregateMessageOrigin;
            readonly weightUsed: SpWeightsWeightV2Weight;
            readonly success: bool;
        } & Struct;
        readonly isOverweightEnqueued: boolean;
        readonly asOverweightEnqueued: {
            readonly id: U8aFixed;
            readonly origin: PolkadotRuntimeParachainsInclusionAggregateMessageOrigin;
            readonly pageIndex: u32;
            readonly messageIndex: u32;
        } & Struct;
        readonly isPageReaped: boolean;
        readonly asPageReaped: {
            readonly origin: PolkadotRuntimeParachainsInclusionAggregateMessageOrigin;
            readonly index: u32;
        } & Struct;
        readonly type: "ProcessingFailed" | "Processed" | "OverweightEnqueued" | "PageReaped";
    }

    /** @name FrameSupportMessagesProcessMessageError (454) */
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

    /** @name PolkadotRuntimeParachainsAssignerOnDemandPalletEvent (455) */
    interface PolkadotRuntimeParachainsAssignerOnDemandPalletEvent extends Enum {
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
        readonly type: "OnDemandOrderPlaced" | "SpotPriceSet";
    }

    /** @name PolkadotRuntimeCommonParasRegistrarPalletEvent (456) */
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

    /** @name PalletUtilityEvent (457) */
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

    /** @name PalletIdentityEvent (459) */
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

    /** @name PalletSchedulerEvent (460) */
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
        readonly type:
            | "Scheduled"
            | "Canceled"
            | "Dispatched"
            | "RetrySet"
            | "RetryCancelled"
            | "CallUnavailable"
            | "PeriodicFailed"
            | "RetryFailed"
            | "PermanentlyOverweight";
    }

    /** @name PalletProxyEvent (462) */
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
        readonly type: "ProxyExecuted" | "PureCreated" | "Announced" | "ProxyAdded" | "ProxyRemoved";
    }

    /** @name PalletMultisigEvent (463) */
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

    /** @name PalletPreimageEvent (464) */
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

    /** @name PalletAssetRateEvent (465) */
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

    /** @name PalletXcmEvent (466) */
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

    /** @name StagingXcmV4TraitsOutcome (467) */
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

    /** @name PalletMigrationsEvent (468) */
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

    /** @name SnowbridgePalletEthereumClientEvent (470) */
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

    /** @name PalletRootTestingEvent (471) */
    interface PalletRootTestingEvent extends Enum {
        readonly isDefensiveTestCall: boolean;
        readonly type: "DefensiveTestCall";
    }

    /** @name PalletSudoEvent (472) */
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

    /** @name FrameSystemPhase (473) */
    interface FrameSystemPhase extends Enum {
        readonly isApplyExtrinsic: boolean;
        readonly asApplyExtrinsic: u32;
        readonly isFinalization: boolean;
        readonly isInitialization: boolean;
        readonly type: "ApplyExtrinsic" | "Finalization" | "Initialization";
    }

    /** @name FrameSystemLastRuntimeUpgradeInfo (475) */
    interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
        readonly specVersion: Compact<u32>;
        readonly specName: Text;
    }

    /** @name FrameSystemCodeUpgradeAuthorization (477) */
    interface FrameSystemCodeUpgradeAuthorization extends Struct {
        readonly codeHash: H256;
        readonly checkVersion: bool;
    }

    /** @name FrameSystemLimitsBlockWeights (478) */
    interface FrameSystemLimitsBlockWeights extends Struct {
        readonly baseBlock: SpWeightsWeightV2Weight;
        readonly maxBlock: SpWeightsWeightV2Weight;
        readonly perClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
    }

    /** @name FrameSupportDispatchPerDispatchClassWeightsPerClass (479) */
    interface FrameSupportDispatchPerDispatchClassWeightsPerClass extends Struct {
        readonly normal: FrameSystemLimitsWeightsPerClass;
        readonly operational: FrameSystemLimitsWeightsPerClass;
        readonly mandatory: FrameSystemLimitsWeightsPerClass;
    }

    /** @name FrameSystemLimitsWeightsPerClass (480) */
    interface FrameSystemLimitsWeightsPerClass extends Struct {
        readonly baseExtrinsic: SpWeightsWeightV2Weight;
        readonly maxExtrinsic: Option<SpWeightsWeightV2Weight>;
        readonly maxTotal: Option<SpWeightsWeightV2Weight>;
        readonly reserved: Option<SpWeightsWeightV2Weight>;
    }

    /** @name FrameSystemLimitsBlockLength (481) */
    interface FrameSystemLimitsBlockLength extends Struct {
        readonly max: FrameSupportDispatchPerDispatchClassU32;
    }

    /** @name FrameSupportDispatchPerDispatchClassU32 (482) */
    interface FrameSupportDispatchPerDispatchClassU32 extends Struct {
        readonly normal: u32;
        readonly operational: u32;
        readonly mandatory: u32;
    }

    /** @name SpWeightsRuntimeDbWeight (483) */
    interface SpWeightsRuntimeDbWeight extends Struct {
        readonly read: u64;
        readonly write: u64;
    }

    /** @name SpVersionRuntimeVersion (484) */
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

    /** @name FrameSystemError (488) */
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

    /** @name SpConsensusBabeDigestsPreDigest (495) */
    interface SpConsensusBabeDigestsPreDigest extends Enum {
        readonly isPrimary: boolean;
        readonly asPrimary: SpConsensusBabeDigestsPrimaryPreDigest;
        readonly isSecondaryPlain: boolean;
        readonly asSecondaryPlain: SpConsensusBabeDigestsSecondaryPlainPreDigest;
        readonly isSecondaryVRF: boolean;
        readonly asSecondaryVRF: SpConsensusBabeDigestsSecondaryVRFPreDigest;
        readonly type: "Primary" | "SecondaryPlain" | "SecondaryVRF";
    }

    /** @name SpConsensusBabeDigestsPrimaryPreDigest (496) */
    interface SpConsensusBabeDigestsPrimaryPreDigest extends Struct {
        readonly authorityIndex: u32;
        readonly slot: u64;
        readonly vrfSignature: SpCoreSr25519VrfVrfSignature;
    }

    /** @name SpCoreSr25519VrfVrfSignature (497) */
    interface SpCoreSr25519VrfVrfSignature extends Struct {
        readonly preOutput: U8aFixed;
        readonly proof: U8aFixed;
    }

    /** @name SpConsensusBabeDigestsSecondaryPlainPreDigest (498) */
    interface SpConsensusBabeDigestsSecondaryPlainPreDigest extends Struct {
        readonly authorityIndex: u32;
        readonly slot: u64;
    }

    /** @name SpConsensusBabeDigestsSecondaryVRFPreDigest (499) */
    interface SpConsensusBabeDigestsSecondaryVRFPreDigest extends Struct {
        readonly authorityIndex: u32;
        readonly slot: u64;
        readonly vrfSignature: SpCoreSr25519VrfVrfSignature;
    }

    /** @name SpConsensusBabeBabeEpochConfiguration (500) */
    interface SpConsensusBabeBabeEpochConfiguration extends Struct {
        readonly c: ITuple<[u64, u64]>;
        readonly allowedSlots: SpConsensusBabeAllowedSlots;
    }

    /** @name PalletBabeError (504) */
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

    /** @name PalletBalancesBalanceLock (506) */
    interface PalletBalancesBalanceLock extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
        readonly reasons: PalletBalancesReasons;
    }

    /** @name PalletBalancesReasons (507) */
    interface PalletBalancesReasons extends Enum {
        readonly isFee: boolean;
        readonly isMisc: boolean;
        readonly isAll: boolean;
        readonly type: "Fee" | "Misc" | "All";
    }

    /** @name PalletBalancesReserveData (510) */
    interface PalletBalancesReserveData extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
    }

    /** @name DancelightRuntimeRuntimeHoldReason (514) */
    interface DancelightRuntimeRuntimeHoldReason extends Enum {
        readonly isContainerRegistrar: boolean;
        readonly asContainerRegistrar: PalletRegistrarHoldReason;
        readonly isDataPreservers: boolean;
        readonly asDataPreservers: PalletDataPreserversHoldReason;
        readonly isPreimage: boolean;
        readonly asPreimage: PalletPreimageHoldReason;
        readonly type: "ContainerRegistrar" | "DataPreservers" | "Preimage";
    }

    /** @name PalletRegistrarHoldReason (515) */
    interface PalletRegistrarHoldReason extends Enum {
        readonly isRegistrarDeposit: boolean;
        readonly type: "RegistrarDeposit";
    }

    /** @name PalletDataPreserversHoldReason (516) */
    interface PalletDataPreserversHoldReason extends Enum {
        readonly isProfileDeposit: boolean;
        readonly type: "ProfileDeposit";
    }

    /** @name PalletPreimageHoldReason (517) */
    interface PalletPreimageHoldReason extends Enum {
        readonly isPreimage: boolean;
        readonly type: "Preimage";
    }

    /** @name FrameSupportTokensMiscIdAmount (520) */
    interface FrameSupportTokensMiscIdAmount extends Struct {
        readonly id: Null;
        readonly amount: u128;
    }

    /** @name PalletBalancesError (522) */
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

    /** @name PalletTransactionPaymentReleases (523) */
    interface PalletTransactionPaymentReleases extends Enum {
        readonly isV1Ancient: boolean;
        readonly isV2: boolean;
        readonly type: "V1Ancient" | "V2";
    }

    /** @name SpStakingOffenceOffenceDetails (524) */
    interface SpStakingOffenceOffenceDetails extends Struct {
        readonly offender: ITuple<[AccountId32, Null]>;
        readonly reporters: Vec<AccountId32>;
    }

    /** @name PalletRegistrarDepositInfo (536) */
    interface PalletRegistrarDepositInfo extends Struct {
        readonly creator: AccountId32;
        readonly deposit: u128;
    }

    /** @name PalletRegistrarError (537) */
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

    /** @name PalletConfigurationHostConfiguration (538) */
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

    /** @name PalletConfigurationError (541) */
    interface PalletConfigurationError extends Enum {
        readonly isInvalidNewValue: boolean;
        readonly type: "InvalidNewValue";
    }

    /** @name PalletInvulnerablesError (543) */
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

    /** @name DpCollatorAssignmentAssignedCollatorsAccountId32 (544) */
    interface DpCollatorAssignmentAssignedCollatorsAccountId32 extends Struct {
        readonly orchestratorChain: Vec<AccountId32>;
        readonly containerChains: BTreeMap<u32, Vec<AccountId32>>;
    }

    /** @name DpCollatorAssignmentAssignedCollatorsPublic (549) */
    interface DpCollatorAssignmentAssignedCollatorsPublic extends Struct {
        readonly orchestratorChain: Vec<NimbusPrimitivesNimbusCryptoPublic>;
        readonly containerChains: BTreeMap<u32, Vec<NimbusPrimitivesNimbusCryptoPublic>>;
    }

    /** @name TpTraitsContainerChainBlockInfo (557) */
    interface TpTraitsContainerChainBlockInfo extends Struct {
        readonly blockNumber: u32;
        readonly author: AccountId32;
        readonly latestSlotNumber: u64;
    }

    /** @name PalletAuthorNotingError (558) */
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

    /** @name PalletServicesPaymentError (559) */
    interface PalletServicesPaymentError extends Enum {
        readonly isInsufficientFundsToPurchaseCredits: boolean;
        readonly isInsufficientCredits: boolean;
        readonly isCreditPriceTooExpensive: boolean;
        readonly type: "InsufficientFundsToPurchaseCredits" | "InsufficientCredits" | "CreditPriceTooExpensive";
    }

    /** @name PalletDataPreserversRegisteredProfile (560) */
    interface PalletDataPreserversRegisteredProfile extends Struct {
        readonly account: AccountId32;
        readonly deposit: u128;
        readonly profile: PalletDataPreserversProfile;
        readonly assignment: Option<ITuple<[u32, DancelightRuntimePreserversAssignmentPaymentWitness]>>;
    }

    /** @name PalletDataPreserversError (566) */
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

    /** @name TpTraitsActiveEraInfo (569) */
    interface TpTraitsActiveEraInfo extends Struct {
        readonly index: u32;
        readonly start: Option<u64>;
    }

    /** @name PalletExternalValidatorsError (571) */
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

    /** @name SpCoreCryptoKeyTypeId (576) */
    interface SpCoreCryptoKeyTypeId extends U8aFixed {}

    /** @name PalletSessionError (577) */
    interface PalletSessionError extends Enum {
        readonly isInvalidProof: boolean;
        readonly isNoAssociatedValidatorId: boolean;
        readonly isDuplicatedKey: boolean;
        readonly isNoKeys: boolean;
        readonly isNoAccount: boolean;
        readonly type: "InvalidProof" | "NoAssociatedValidatorId" | "DuplicatedKey" | "NoKeys" | "NoAccount";
    }

    /** @name PalletGrandpaStoredState (578) */
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

    /** @name PalletGrandpaStoredPendingChange (579) */
    interface PalletGrandpaStoredPendingChange extends Struct {
        readonly scheduledAt: u32;
        readonly delay: u32;
        readonly nextAuthorities: Vec<ITuple<[SpConsensusGrandpaAppPublic, u64]>>;
        readonly forced: Option<u32>;
    }

    /** @name PalletGrandpaError (581) */
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

    /** @name PalletInflationRewardsChainsToRewardValue (584) */
    interface PalletInflationRewardsChainsToRewardValue extends Struct {
        readonly paraIds: Vec<u32>;
        readonly rewardsPerChain: u128;
    }

    /** @name PalletTreasuryProposal (585) */
    interface PalletTreasuryProposal extends Struct {
        readonly proposer: AccountId32;
        readonly value: u128;
        readonly beneficiary: AccountId32;
        readonly bond: u128;
    }

    /** @name PalletTreasurySpendStatus (587) */
    interface PalletTreasurySpendStatus extends Struct {
        readonly assetKind: Null;
        readonly amount: u128;
        readonly beneficiary: AccountId32;
        readonly validFrom: u32;
        readonly expireAt: u32;
        readonly status: PalletTreasuryPaymentState;
    }

    /** @name PalletTreasuryPaymentState (588) */
    interface PalletTreasuryPaymentState extends Enum {
        readonly isPending: boolean;
        readonly isAttempted: boolean;
        readonly asAttempted: {
            readonly id: Null;
        } & Struct;
        readonly isFailed: boolean;
        readonly type: "Pending" | "Attempted" | "Failed";
    }

    /** @name FrameSupportPalletId (590) */
    interface FrameSupportPalletId extends U8aFixed {}

    /** @name PalletTreasuryError (591) */
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

    /** @name PalletConvictionVotingVoteVoting (593) */
    interface PalletConvictionVotingVoteVoting extends Enum {
        readonly isCasting: boolean;
        readonly asCasting: PalletConvictionVotingVoteCasting;
        readonly isDelegating: boolean;
        readonly asDelegating: PalletConvictionVotingVoteDelegating;
        readonly type: "Casting" | "Delegating";
    }

    /** @name PalletConvictionVotingVoteCasting (594) */
    interface PalletConvictionVotingVoteCasting extends Struct {
        readonly votes: Vec<ITuple<[u32, PalletConvictionVotingVoteAccountVote]>>;
        readonly delegations: PalletConvictionVotingDelegations;
        readonly prior: PalletConvictionVotingVotePriorLock;
    }

    /** @name PalletConvictionVotingDelegations (598) */
    interface PalletConvictionVotingDelegations extends Struct {
        readonly votes: u128;
        readonly capital: u128;
    }

    /** @name PalletConvictionVotingVotePriorLock (599) */
    interface PalletConvictionVotingVotePriorLock extends ITuple<[u32, u128]> {}

    /** @name PalletConvictionVotingVoteDelegating (600) */
    interface PalletConvictionVotingVoteDelegating extends Struct {
        readonly balance: u128;
        readonly target: AccountId32;
        readonly conviction: PalletConvictionVotingConviction;
        readonly delegations: PalletConvictionVotingDelegations;
        readonly prior: PalletConvictionVotingVotePriorLock;
    }

    /** @name PalletConvictionVotingError (604) */
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

    /** @name PalletReferendaReferendumInfoConvictionVotingTally (605) */
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

    /** @name PalletReferendaReferendumStatusConvictionVotingTally (606) */
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

    /** @name PalletReferendaDeposit (607) */
    interface PalletReferendaDeposit extends Struct {
        readonly who: AccountId32;
        readonly amount: u128;
    }

    /** @name PalletReferendaDecidingStatus (610) */
    interface PalletReferendaDecidingStatus extends Struct {
        readonly since: u32;
        readonly confirming: Option<u32>;
    }

    /** @name PalletReferendaTrackInfo (618) */
    interface PalletReferendaTrackInfo extends Struct {
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

    /** @name PalletReferendaCurve (619) */
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

    /** @name PalletReferendaError (622) */
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

    /** @name PalletRankedCollectiveMemberRecord (623) */
    interface PalletRankedCollectiveMemberRecord extends Struct {
        readonly rank: u16;
    }

    /** @name PalletRankedCollectiveError (628) */
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

    /** @name PalletReferendaReferendumInfoRankedCollectiveTally (629) */
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

    /** @name PalletReferendaReferendumStatusRankedCollectiveTally (630) */
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

    /** @name PalletWhitelistError (633) */
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

    /** @name PolkadotRuntimeParachainsConfigurationHostConfiguration (634) */
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
        readonly asyncBackingParams: PolkadotPrimitivesV7AsyncBackingAsyncBackingParams;
        readonly maxPovSize: u32;
        readonly maxDownwardMessageSize: u32;
        readonly hrmpMaxParachainOutboundChannels: u32;
        readonly hrmpSenderDeposit: u128;
        readonly hrmpRecipientDeposit: u128;
        readonly hrmpChannelMaxCapacity: u32;
        readonly hrmpChannelMaxTotalSize: u32;
        readonly hrmpMaxParachainInboundChannels: u32;
        readonly hrmpChannelMaxMessageSize: u32;
        readonly executorParams: PolkadotPrimitivesV7ExecutorParams;
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
        readonly approvalVotingParams: PolkadotPrimitivesV7ApprovalVotingParams;
        readonly schedulerParams: PolkadotPrimitivesVstagingSchedulerParams;
    }

    /** @name PolkadotRuntimeParachainsConfigurationPalletError (637) */
    interface PolkadotRuntimeParachainsConfigurationPalletError extends Enum {
        readonly isInvalidNewValue: boolean;
        readonly type: "InvalidNewValue";
    }

    /** @name PolkadotRuntimeParachainsSharedAllowedRelayParentsTracker (640) */
    interface PolkadotRuntimeParachainsSharedAllowedRelayParentsTracker extends Struct {
        readonly buffer: Vec<ITuple<[H256, H256]>>;
        readonly latestNumber: u32;
    }

    /** @name PolkadotRuntimeParachainsInclusionCandidatePendingAvailability (644) */
    interface PolkadotRuntimeParachainsInclusionCandidatePendingAvailability extends Struct {
        readonly core: u32;
        readonly hash_: H256;
        readonly descriptor: PolkadotPrimitivesV7CandidateDescriptor;
        readonly commitments: PolkadotPrimitivesV7CandidateCommitments;
        readonly availabilityVotes: BitVec;
        readonly backers: BitVec;
        readonly relayParentNumber: u32;
        readonly backedInNumber: u32;
        readonly backingGroup: u32;
    }

    /** @name PolkadotRuntimeParachainsInclusionPalletError (645) */
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
        readonly isNotCollatorSigned: boolean;
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
            | "NotCollatorSigned"
            | "ValidationDataHashMismatch"
            | "IncorrectDownwardMessageHandling"
            | "InvalidUpwardMessages"
            | "HrmpWatermarkMishandling"
            | "InvalidOutboundHrmp"
            | "InvalidValidationCodeHash"
            | "ParaHeadMismatch";
    }

    /** @name PolkadotPrimitivesV7ScrapedOnChainVotes (646) */
    interface PolkadotPrimitivesV7ScrapedOnChainVotes extends Struct {
        readonly session: u32;
        readonly backingValidatorsPerCandidate: Vec<
            ITuple<[PolkadotPrimitivesV7CandidateReceipt, Vec<ITuple<[u32, PolkadotPrimitivesV7ValidityAttestation]>>]>
        >;
        readonly disputes: Vec<PolkadotPrimitivesV7DisputeStatementSet>;
    }

    /** @name PolkadotRuntimeParachainsParasInherentPalletError (651) */
    interface PolkadotRuntimeParachainsParasInherentPalletError extends Enum {
        readonly isTooManyInclusionInherents: boolean;
        readonly isInvalidParentHeader: boolean;
        readonly isInherentOverweight: boolean;
        readonly isCandidatesFilteredDuringExecution: boolean;
        readonly isUnscheduledCandidate: boolean;
        readonly type:
            | "TooManyInclusionInherents"
            | "InvalidParentHeader"
            | "InherentOverweight"
            | "CandidatesFilteredDuringExecution"
            | "UnscheduledCandidate";
    }

    /** @name PolkadotRuntimeParachainsSchedulerPalletCoreOccupied (654) */
    interface PolkadotRuntimeParachainsSchedulerPalletCoreOccupied extends Enum {
        readonly isFree: boolean;
        readonly isParas: boolean;
        readonly asParas: PolkadotRuntimeParachainsSchedulerPalletParasEntry;
        readonly type: "Free" | "Paras";
    }

    /** @name PolkadotRuntimeParachainsSchedulerPalletParasEntry (655) */
    interface PolkadotRuntimeParachainsSchedulerPalletParasEntry extends Struct {
        readonly assignment: PolkadotRuntimeParachainsSchedulerCommonAssignment;
        readonly availabilityTimeouts: u32;
        readonly ttl: u32;
    }

    /** @name PolkadotRuntimeParachainsSchedulerCommonAssignment (656) */
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

    /** @name PolkadotRuntimeParachainsParasPvfCheckActiveVoteState (661) */
    interface PolkadotRuntimeParachainsParasPvfCheckActiveVoteState extends Struct {
        readonly votesAccept: BitVec;
        readonly votesReject: BitVec;
        readonly age: u32;
        readonly createdAt: u32;
        readonly causes: Vec<PolkadotRuntimeParachainsParasPvfCheckCause>;
    }

    /** @name PolkadotRuntimeParachainsParasPvfCheckCause (663) */
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

    /** @name PolkadotRuntimeParachainsParasUpgradeStrategy (664) */
    interface PolkadotRuntimeParachainsParasUpgradeStrategy extends Enum {
        readonly isSetGoAheadSignal: boolean;
        readonly isApplyAtExpectedBlock: boolean;
        readonly type: "SetGoAheadSignal" | "ApplyAtExpectedBlock";
    }

    /** @name PolkadotRuntimeParachainsParasParaLifecycle (666) */
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

    /** @name PolkadotRuntimeParachainsParasParaPastCodeMeta (668) */
    interface PolkadotRuntimeParachainsParasParaPastCodeMeta extends Struct {
        readonly upgradeTimes: Vec<PolkadotRuntimeParachainsParasReplacementTimes>;
        readonly lastPruned: Option<u32>;
    }

    /** @name PolkadotRuntimeParachainsParasReplacementTimes (670) */
    interface PolkadotRuntimeParachainsParasReplacementTimes extends Struct {
        readonly expectedAt: u32;
        readonly activatedAt: u32;
    }

    /** @name PolkadotPrimitivesV7UpgradeGoAhead (672) */
    interface PolkadotPrimitivesV7UpgradeGoAhead extends Enum {
        readonly isAbort: boolean;
        readonly isGoAhead: boolean;
        readonly type: "Abort" | "GoAhead";
    }

    /** @name PolkadotPrimitivesV7UpgradeRestriction (673) */
    interface PolkadotPrimitivesV7UpgradeRestriction extends Enum {
        readonly isPresent: boolean;
        readonly type: "Present";
    }

    /** @name PolkadotRuntimeParachainsParasPalletError (674) */
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
            | "InvalidCode";
    }

    /** @name PolkadotRuntimeParachainsInitializerBufferedSessionChange (676) */
    interface PolkadotRuntimeParachainsInitializerBufferedSessionChange extends Struct {
        readonly validators: Vec<PolkadotPrimitivesV7ValidatorAppPublic>;
        readonly queued: Vec<PolkadotPrimitivesV7ValidatorAppPublic>;
        readonly sessionIndex: u32;
    }

    /** @name PolkadotCorePrimitivesInboundDownwardMessage (678) */
    interface PolkadotCorePrimitivesInboundDownwardMessage extends Struct {
        readonly sentAt: u32;
        readonly msg: Bytes;
    }

    /** @name PolkadotRuntimeParachainsHrmpHrmpOpenChannelRequest (679) */
    interface PolkadotRuntimeParachainsHrmpHrmpOpenChannelRequest extends Struct {
        readonly confirmed: bool;
        readonly age: u32;
        readonly senderDeposit: u128;
        readonly maxMessageSize: u32;
        readonly maxCapacity: u32;
        readonly maxTotalSize: u32;
    }

    /** @name PolkadotRuntimeParachainsHrmpHrmpChannel (681) */
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

    /** @name PolkadotCorePrimitivesInboundHrmpMessage (683) */
    interface PolkadotCorePrimitivesInboundHrmpMessage extends Struct {
        readonly sentAt: u32;
        readonly data: Bytes;
    }

    /** @name PolkadotRuntimeParachainsHrmpPalletError (686) */
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

    /** @name PolkadotPrimitivesV7SessionInfo (688) */
    interface PolkadotPrimitivesV7SessionInfo extends Struct {
        readonly activeValidatorIndices: Vec<u32>;
        readonly randomSeed: U8aFixed;
        readonly disputePeriod: u32;
        readonly validators: PolkadotPrimitivesV7IndexedVecValidatorIndex;
        readonly discoveryKeys: Vec<SpAuthorityDiscoveryAppPublic>;
        readonly assignmentKeys: Vec<PolkadotPrimitivesV7AssignmentAppPublic>;
        readonly validatorGroups: PolkadotPrimitivesV7IndexedVecGroupIndex;
        readonly nCores: u32;
        readonly zerothDelayTrancheWidth: u32;
        readonly relayVrfModuloSamples: u32;
        readonly nDelayTranches: u32;
        readonly noShowSlots: u32;
        readonly neededApprovals: u32;
    }

    /** @name PolkadotPrimitivesV7IndexedVecValidatorIndex (689) */
    interface PolkadotPrimitivesV7IndexedVecValidatorIndex extends Vec<PolkadotPrimitivesV7ValidatorAppPublic> {}

    /** @name PolkadotPrimitivesV7IndexedVecGroupIndex (690) */
    interface PolkadotPrimitivesV7IndexedVecGroupIndex extends Vec<Vec<u32>> {}

    /** @name PolkadotPrimitivesV7DisputeState (692) */
    interface PolkadotPrimitivesV7DisputeState extends Struct {
        readonly validatorsFor: BitVec;
        readonly validatorsAgainst: BitVec;
        readonly start: u32;
        readonly concludedAt: Option<u32>;
    }

    /** @name PolkadotRuntimeParachainsDisputesPalletError (694) */
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

    /** @name PolkadotPrimitivesV7SlashingPendingSlashes (695) */
    interface PolkadotPrimitivesV7SlashingPendingSlashes extends Struct {
        readonly keys_: BTreeMap<u32, PolkadotPrimitivesV7ValidatorAppPublic>;
        readonly kind: PolkadotPrimitivesV7SlashingSlashingOffenceKind;
    }

    /** @name PolkadotRuntimeParachainsDisputesSlashingPalletError (699) */
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

    /** @name PalletMessageQueueBookState (700) */
    interface PalletMessageQueueBookState extends Struct {
        readonly begin: u32;
        readonly end: u32;
        readonly count: u32;
        readonly readyNeighbours: Option<PalletMessageQueueNeighbours>;
        readonly messageCount: u64;
        readonly size_: u64;
    }

    /** @name PalletMessageQueueNeighbours (702) */
    interface PalletMessageQueueNeighbours extends Struct {
        readonly prev: PolkadotRuntimeParachainsInclusionAggregateMessageOrigin;
        readonly next: PolkadotRuntimeParachainsInclusionAggregateMessageOrigin;
    }

    /** @name PalletMessageQueuePage (704) */
    interface PalletMessageQueuePage extends Struct {
        readonly remaining: u32;
        readonly remainingSize: u32;
        readonly firstIndex: u32;
        readonly first: u32;
        readonly last: u32;
        readonly heap: Bytes;
    }

    /** @name PalletMessageQueueError (706) */
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

    /** @name PolkadotRuntimeParachainsAssignerOnDemandTypesCoreAffinityCount (707) */
    interface PolkadotRuntimeParachainsAssignerOnDemandTypesCoreAffinityCount extends Struct {
        readonly coreIndex: u32;
        readonly count: u32;
    }

    /** @name PolkadotRuntimeParachainsAssignerOnDemandTypesQueueStatusType (708) */
    interface PolkadotRuntimeParachainsAssignerOnDemandTypesQueueStatusType extends Struct {
        readonly traffic: u128;
        readonly nextIndex: u32;
        readonly smallestIndex: u32;
        readonly freedIndices: BinaryHeapReverseQueueIndex;
    }

    /** @name BinaryHeapReverseQueueIndex (710) */
    interface BinaryHeapReverseQueueIndex extends Vec<u32> {}

    /** @name BinaryHeapEnqueuedOrder (713) */
    interface BinaryHeapEnqueuedOrder extends Vec<PolkadotRuntimeParachainsAssignerOnDemandTypesEnqueuedOrder> {}

    /** @name PolkadotRuntimeParachainsAssignerOnDemandTypesEnqueuedOrder (714) */
    interface PolkadotRuntimeParachainsAssignerOnDemandTypesEnqueuedOrder extends Struct {
        readonly paraId: u32;
        readonly idx: u32;
    }

    /** @name PolkadotRuntimeParachainsAssignerOnDemandPalletError (718) */
    interface PolkadotRuntimeParachainsAssignerOnDemandPalletError extends Enum {
        readonly isQueueFull: boolean;
        readonly isSpotPriceHigherThanMaxAmount: boolean;
        readonly type: "QueueFull" | "SpotPriceHigherThanMaxAmount";
    }

    /** @name PolkadotRuntimeCommonParasRegistrarParaInfo (719) */
    interface PolkadotRuntimeCommonParasRegistrarParaInfo extends Struct {
        readonly manager: AccountId32;
        readonly deposit: u128;
        readonly locked: Option<bool>;
    }

    /** @name PolkadotRuntimeCommonParasRegistrarPalletError (721) */
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

    /** @name PalletUtilityError (722) */
    interface PalletUtilityError extends Enum {
        readonly isTooManyCalls: boolean;
        readonly type: "TooManyCalls";
    }

    /** @name PalletIdentityRegistration (724) */
    interface PalletIdentityRegistration extends Struct {
        readonly judgements: Vec<ITuple<[u32, PalletIdentityJudgement]>>;
        readonly deposit: u128;
        readonly info: PalletIdentityLegacyIdentityInfo;
    }

    /** @name PalletIdentityRegistrarInfo (733) */
    interface PalletIdentityRegistrarInfo extends Struct {
        readonly account: AccountId32;
        readonly fee: u128;
        readonly fields: u64;
    }

    /** @name PalletIdentityAuthorityProperties (735) */
    interface PalletIdentityAuthorityProperties extends Struct {
        readonly suffix: Bytes;
        readonly allocation: u32;
    }

    /** @name PalletIdentityError (738) */
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

    /** @name PalletSchedulerScheduled (741) */
    interface PalletSchedulerScheduled extends Struct {
        readonly maybeId: Option<U8aFixed>;
        readonly priority: u8;
        readonly call: FrameSupportPreimagesBounded;
        readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
        readonly origin: DancelightRuntimeOriginCaller;
    }

    /** @name PalletSchedulerRetryConfig (743) */
    interface PalletSchedulerRetryConfig extends Struct {
        readonly totalRetries: u8;
        readonly remaining: u8;
        readonly period: u32;
    }

    /** @name PalletSchedulerError (744) */
    interface PalletSchedulerError extends Enum {
        readonly isFailedToSchedule: boolean;
        readonly isNotFound: boolean;
        readonly isTargetBlockNumberInPast: boolean;
        readonly isRescheduleNoChange: boolean;
        readonly isNamed: boolean;
        readonly type: "FailedToSchedule" | "NotFound" | "TargetBlockNumberInPast" | "RescheduleNoChange" | "Named";
    }

    /** @name PalletProxyProxyDefinition (747) */
    interface PalletProxyProxyDefinition extends Struct {
        readonly delegate: AccountId32;
        readonly proxyType: DancelightRuntimeProxyType;
        readonly delay: u32;
    }

    /** @name PalletProxyAnnouncement (751) */
    interface PalletProxyAnnouncement extends Struct {
        readonly real: AccountId32;
        readonly callHash: H256;
        readonly height: u32;
    }

    /** @name PalletProxyError (753) */
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

    /** @name PalletMultisigMultisig (755) */
    interface PalletMultisigMultisig extends Struct {
        readonly when: PalletMultisigTimepoint;
        readonly deposit: u128;
        readonly depositor: AccountId32;
        readonly approvals: Vec<AccountId32>;
    }

    /** @name PalletMultisigError (757) */
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

    /** @name PalletPreimageOldRequestStatus (758) */
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

    /** @name PalletPreimageRequestStatus (761) */
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

    /** @name PalletPreimageError (766) */
    interface PalletPreimageError extends Enum {
        readonly isTooBig: boolean;
        readonly isAlreadyNoted: boolean;
        readonly isNotAuthorized: boolean;
        readonly isNotNoted: boolean;
        readonly isRequested: boolean;
        readonly isNotRequested: boolean;
        readonly isTooMany: boolean;
        readonly isTooFew: boolean;
        readonly isNoCost: boolean;
        readonly type:
            | "TooBig"
            | "AlreadyNoted"
            | "NotAuthorized"
            | "NotNoted"
            | "Requested"
            | "NotRequested"
            | "TooMany"
            | "TooFew"
            | "NoCost";
    }

    /** @name PalletAssetRateError (767) */
    interface PalletAssetRateError extends Enum {
        readonly isUnknownAssetKind: boolean;
        readonly isAlreadyExists: boolean;
        readonly isOverflow: boolean;
        readonly type: "UnknownAssetKind" | "AlreadyExists" | "Overflow";
    }

    /** @name PalletXcmQueryStatus (768) */
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

    /** @name XcmVersionedResponse (772) */
    interface XcmVersionedResponse extends Enum {
        readonly isV2: boolean;
        readonly asV2: XcmV2Response;
        readonly isV3: boolean;
        readonly asV3: XcmV3Response;
        readonly isV4: boolean;
        readonly asV4: StagingXcmV4Response;
        readonly type: "V2" | "V3" | "V4";
    }

    /** @name PalletXcmVersionMigrationStage (778) */
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

    /** @name PalletXcmRemoteLockedFungibleRecord (780) */
    interface PalletXcmRemoteLockedFungibleRecord extends Struct {
        readonly amount: u128;
        readonly owner: XcmVersionedLocation;
        readonly locker: XcmVersionedLocation;
        readonly consumers: Vec<ITuple<[Null, u128]>>;
    }

    /** @name PalletXcmError (787) */
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

    /** @name PalletMigrationsError (788) */
    interface PalletMigrationsError extends Enum {
        readonly isPreimageMissing: boolean;
        readonly isWrongUpperBound: boolean;
        readonly isPreimageIsTooBig: boolean;
        readonly isPreimageAlreadyExists: boolean;
        readonly type: "PreimageMissing" | "WrongUpperBound" | "PreimageIsTooBig" | "PreimageAlreadyExists";
    }

    /** @name PalletBeefyError (792) */
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

    /** @name SpConsensusBeefyMmrBeefyAuthoritySet (793) */
    interface SpConsensusBeefyMmrBeefyAuthoritySet extends Struct {
        readonly id: u64;
        readonly len: u32;
        readonly keysetCommitment: H256;
    }

    /** @name SnowbridgeBeaconPrimitivesCompactBeaconState (794) */
    interface SnowbridgeBeaconPrimitivesCompactBeaconState extends Struct {
        readonly slot: Compact<u64>;
        readonly blockRootsRoot: H256;
    }

    /** @name SnowbridgeBeaconPrimitivesSyncCommitteePrepared (795) */
    interface SnowbridgeBeaconPrimitivesSyncCommitteePrepared extends Struct {
        readonly root: H256;
        readonly pubkeys: Vec<SnowbridgeMilagroBlsKeysPublicKey>;
        readonly aggregatePubkey: SnowbridgeMilagroBlsKeysPublicKey;
    }

    /** @name SnowbridgeMilagroBlsKeysPublicKey (797) */
    interface SnowbridgeMilagroBlsKeysPublicKey extends Struct {
        readonly point: SnowbridgeAmclBls381Ecp;
    }

    /** @name SnowbridgeAmclBls381Ecp (798) */
    interface SnowbridgeAmclBls381Ecp extends Struct {
        readonly x: SnowbridgeAmclBls381Fp;
        readonly y: SnowbridgeAmclBls381Fp;
        readonly z: SnowbridgeAmclBls381Fp;
    }

    /** @name SnowbridgeAmclBls381Fp (799) */
    interface SnowbridgeAmclBls381Fp extends Struct {
        readonly x: SnowbridgeAmclBls381Big;
        readonly xes: i32;
    }

    /** @name SnowbridgeAmclBls381Big (800) */
    interface SnowbridgeAmclBls381Big extends Struct {
        readonly w: Vec<i32>;
    }

    /** @name SnowbridgeBeaconPrimitivesForkVersions (803) */
    interface SnowbridgeBeaconPrimitivesForkVersions extends Struct {
        readonly genesis: SnowbridgeBeaconPrimitivesFork;
        readonly altair: SnowbridgeBeaconPrimitivesFork;
        readonly bellatrix: SnowbridgeBeaconPrimitivesFork;
        readonly capella: SnowbridgeBeaconPrimitivesFork;
        readonly deneb: SnowbridgeBeaconPrimitivesFork;
    }

    /** @name SnowbridgeBeaconPrimitivesFork (804) */
    interface SnowbridgeBeaconPrimitivesFork extends Struct {
        readonly version: U8aFixed;
        readonly epoch: u64;
    }

    /** @name SnowbridgePalletEthereumClientError (805) */
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

    /** @name SnowbridgeBeaconPrimitivesBlsBlsError (806) */
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

    /** @name PolkadotRuntimeCommonParasSudoWrapperPalletError (807) */
    interface PolkadotRuntimeCommonParasSudoWrapperPalletError extends Enum {
        readonly isParaDoesntExist: boolean;
        readonly isParaAlreadyExists: boolean;
        readonly isExceedsMaxMessageSize: boolean;
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
            | "CouldntCleanup"
            | "NotParathread"
            | "NotParachain"
            | "CannotUpgrade"
            | "CannotDowngrade"
            | "TooManyCores";
    }

    /** @name PalletSudoError (808) */
    interface PalletSudoError extends Enum {
        readonly isRequireSudo: boolean;
        readonly type: "RequireSudo";
    }

    /** @name FrameSystemExtensionsCheckNonZeroSender (811) */
    type FrameSystemExtensionsCheckNonZeroSender = Null;

    /** @name FrameSystemExtensionsCheckSpecVersion (812) */
    type FrameSystemExtensionsCheckSpecVersion = Null;

    /** @name FrameSystemExtensionsCheckTxVersion (813) */
    type FrameSystemExtensionsCheckTxVersion = Null;

    /** @name FrameSystemExtensionsCheckGenesis (814) */
    type FrameSystemExtensionsCheckGenesis = Null;

    /** @name FrameSystemExtensionsCheckNonce (817) */
    interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

    /** @name FrameSystemExtensionsCheckWeight (818) */
    type FrameSystemExtensionsCheckWeight = Null;

    /** @name PalletTransactionPaymentChargeTransactionPayment (819) */
    interface PalletTransactionPaymentChargeTransactionPayment extends Compact<u128> {}

    /** @name DancelightRuntimeRuntime (820) */
    type DancelightRuntimeRuntime = Null;
} // declare module
