// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/events";

import type { ApiTypes, AugmentedEvent } from "@polkadot/api-base/types";
import type {
  Null,
  Option,
  Result,
  U8aFixed,
  Vec,
  u128,
  u32,
} from "@polkadot/types-codec";
import type { AccountId32, H256 } from "@polkadot/types/interfaces/runtime";
import type {
  FrameSupportDispatchDispatchInfo,
  FrameSupportTokensMiscBalanceStatus,
  SpRuntimeDispatchError,
  SpWeightsWeightV2Weight,
} from "@polkadot/types/lookup";

export type __AugmentedEvent<ApiType extends ApiTypes> =
  AugmentedEvent<ApiType>;

declare module "@polkadot/api-base/types/events" {
  interface AugmentedEvents<ApiType extends ApiTypes> {
    authorNoting: {
      /** Latest author changed */
      LatestAuthorChanged: AugmentedEvent<
        ApiType,
        [paraId: u32, newAuthor: AccountId32],
        { paraId: u32; newAuthor: AccountId32 }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    balances: {
      /** A balance was set by root. */
      BalanceSet: AugmentedEvent<
        ApiType,
        [who: AccountId32, free: u128],
        { who: AccountId32; free: u128 }
      >;
      /** Some amount was burned from an account. */
      Burned: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Some amount was deposited (e.g. for transaction fees). */
      Deposit: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /**
       * An account was removed whose balance was non-zero but below
       * ExistentialDeposit, resulting in an outright loss.
       */
      DustLost: AugmentedEvent<
        ApiType,
        [account: AccountId32, amount: u128],
        { account: AccountId32; amount: u128 }
      >;
      /** An account was created with some free balance. */
      Endowed: AugmentedEvent<
        ApiType,
        [account: AccountId32, freeBalance: u128],
        { account: AccountId32; freeBalance: u128 }
      >;
      /** Some balance was frozen. */
      Frozen: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Total issuance was increased by `amount`, creating a credit to be balanced. */
      Issued: AugmentedEvent<ApiType, [amount: u128], { amount: u128 }>;
      /** Some balance was locked. */
      Locked: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Some amount was minted into an account. */
      Minted: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Total issuance was decreased by `amount`, creating a debt to be balanced. */
      Rescinded: AugmentedEvent<ApiType, [amount: u128], { amount: u128 }>;
      /** Some balance was reserved (moved from free to reserved). */
      Reserved: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /**
       * Some balance was moved from the reserve of the first account to the
       * second account. Final argument indicates the destination balance type.
       */
      ReserveRepatriated: AugmentedEvent<
        ApiType,
        [
          from: AccountId32,
          to: AccountId32,
          amount: u128,
          destinationStatus: FrameSupportTokensMiscBalanceStatus
        ],
        {
          from: AccountId32;
          to: AccountId32;
          amount: u128;
          destinationStatus: FrameSupportTokensMiscBalanceStatus;
        }
      >;
      /** Some amount was restored into an account. */
      Restored: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Some amount was removed from the account (e.g. for misbehavior). */
      Slashed: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Some amount was suspended from an account (it can be restored later). */
      Suspended: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Some balance was thawed. */
      Thawed: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Transfer succeeded. */
      Transfer: AugmentedEvent<
        ApiType,
        [from: AccountId32, to: AccountId32, amount: u128],
        { from: AccountId32; to: AccountId32; amount: u128 }
      >;
      /** Some balance was unlocked. */
      Unlocked: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Some balance was unreserved (moved from reserved to free). */
      Unreserved: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** An account was upgraded. */
      Upgraded: AugmentedEvent<
        ApiType,
        [who: AccountId32],
        { who: AccountId32 }
      >;
      /** Some amount was withdrawn from the account (e.g. for transaction fees). */
      Withdraw: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    collatorSelection: {
      CandidateAdded: AugmentedEvent<
        ApiType,
        [accountId: AccountId32, deposit: u128],
        { accountId: AccountId32; deposit: u128 }
      >;
      CandidateRemoved: AugmentedEvent<
        ApiType,
        [accountId: AccountId32],
        { accountId: AccountId32 }
      >;
      NewCandidacyBond: AugmentedEvent<
        ApiType,
        [bondAmount: u128],
        { bondAmount: u128 }
      >;
      NewDesiredCandidates: AugmentedEvent<
        ApiType,
        [desiredCandidates: u32],
        { desiredCandidates: u32 }
      >;
      NewInvulnerables: AugmentedEvent<
        ApiType,
        [invulnerables: Vec<AccountId32>],
        { invulnerables: Vec<AccountId32> }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    parachainSystem: {
      /** Downward messages were processed using the given weight. */
      DownwardMessagesProcessed: AugmentedEvent<
        ApiType,
        [weightUsed: SpWeightsWeightV2Weight, dmqHead: H256],
        { weightUsed: SpWeightsWeightV2Weight; dmqHead: H256 }
      >;
      /** Some downward messages have been received and will be processed. */
      DownwardMessagesReceived: AugmentedEvent<
        ApiType,
        [count: u32],
        { count: u32 }
      >;
      /** An upgrade has been authorized. */
      UpgradeAuthorized: AugmentedEvent<
        ApiType,
        [codeHash: H256],
        { codeHash: H256 }
      >;
      /** An upward message was sent to the relay chain. */
      UpwardMessageSent: AugmentedEvent<
        ApiType,
        [messageHash: Option<U8aFixed>],
        { messageHash: Option<U8aFixed> }
      >;
      /** The validation function was applied as of the contained relay chain block number. */
      ValidationFunctionApplied: AugmentedEvent<
        ApiType,
        [relayChainBlockNum: u32],
        { relayChainBlockNum: u32 }
      >;
      /** The relay-chain aborted the upgrade process. */
      ValidationFunctionDiscarded: AugmentedEvent<ApiType, []>;
      /** The validation function has been scheduled to apply. */
      ValidationFunctionStored: AugmentedEvent<ApiType, []>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    registrar: {
      /** The list of boot_nodes */
      BootNodesChanged: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
      /** A para id has been deregistered. [para_id] */
      ParaIdDeregistered: AugmentedEvent<
        ApiType,
        [paraId: u32],
        { paraId: u32 }
      >;
      /** A new para id has been registered. [para_id] */
      ParaIdRegistered: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
      /** A new para id is now valid for collating. [para_id] */
      ParaIdValidForCollating: AugmentedEvent<
        ApiType,
        [paraId: u32],
        { paraId: u32 }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    session: {
      /**
       * New session has happened. Note that the argument is the session index,
       * not the block number as the type might suggest.
       */
      NewSession: AugmentedEvent<
        ApiType,
        [sessionIndex: u32],
        { sessionIndex: u32 }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    sudo: {
      /** The [sudoer] just switched identity; the old key is supplied if one existed. */
      KeyChanged: AugmentedEvent<
        ApiType,
        [oldSudoer: Option<AccountId32>],
        { oldSudoer: Option<AccountId32> }
      >;
      /** A sudo just took place. [result] */
      Sudid: AugmentedEvent<
        ApiType,
        [sudoResult: Result<Null, SpRuntimeDispatchError>],
        { sudoResult: Result<Null, SpRuntimeDispatchError> }
      >;
      /** A sudo just took place. [result] */
      SudoAsDone: AugmentedEvent<
        ApiType,
        [sudoResult: Result<Null, SpRuntimeDispatchError>],
        { sudoResult: Result<Null, SpRuntimeDispatchError> }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    system: {
      /** `:code` was updated. */
      CodeUpdated: AugmentedEvent<ApiType, []>;
      /** An extrinsic failed. */
      ExtrinsicFailed: AugmentedEvent<
        ApiType,
        [
          dispatchError: SpRuntimeDispatchError,
          dispatchInfo: FrameSupportDispatchDispatchInfo
        ],
        {
          dispatchError: SpRuntimeDispatchError;
          dispatchInfo: FrameSupportDispatchDispatchInfo;
        }
      >;
      /** An extrinsic completed successfully. */
      ExtrinsicSuccess: AugmentedEvent<
        ApiType,
        [dispatchInfo: FrameSupportDispatchDispatchInfo],
        { dispatchInfo: FrameSupportDispatchDispatchInfo }
      >;
      /** An account was reaped. */
      KilledAccount: AugmentedEvent<
        ApiType,
        [account: AccountId32],
        { account: AccountId32 }
      >;
      /** A new account was created. */
      NewAccount: AugmentedEvent<
        ApiType,
        [account: AccountId32],
        { account: AccountId32 }
      >;
      /** On on-chain remark happened. */
      Remarked: AugmentedEvent<
        ApiType,
        [sender: AccountId32, hash_: H256],
        { sender: AccountId32; hash_: H256 }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    utility: {
      /** Batch of dispatches completed fully with no error. */
      BatchCompleted: AugmentedEvent<ApiType, []>;
      /** Batch of dispatches completed but has errors. */
      BatchCompletedWithErrors: AugmentedEvent<ApiType, []>;
      /**
       * Batch of dispatches did not complete fully. Index of first failing
       * dispatch given, as well as the error.
       */
      BatchInterrupted: AugmentedEvent<
        ApiType,
        [index: u32, error: SpRuntimeDispatchError],
        { index: u32; error: SpRuntimeDispatchError }
      >;
      /** A call was dispatched. */
      DispatchedAs: AugmentedEvent<
        ApiType,
        [result: Result<Null, SpRuntimeDispatchError>],
        { result: Result<Null, SpRuntimeDispatchError> }
      >;
      /** A single item within a Batch of dispatches has completed with no error. */
      ItemCompleted: AugmentedEvent<ApiType, []>;
      /** A single item within a Batch of dispatches has completed with error. */
      ItemFailed: AugmentedEvent<
        ApiType,
        [error: SpRuntimeDispatchError],
        { error: SpRuntimeDispatchError }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
  } // AugmentedEvents
} // declare module
