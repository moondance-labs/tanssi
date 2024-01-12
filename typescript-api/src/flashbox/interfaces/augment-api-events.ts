// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/events";

import type { ApiTypes, AugmentedEvent } from "@polkadot/api-base/types";
import type { Bytes, Null, Option, Result, U8aFixed, Vec, bool, u128, u16, u32 } from "@polkadot/types-codec";
import type { ITuple } from "@polkadot/types-codec/types";
import type { AccountId32, H256 } from "@polkadot/types/interfaces/runtime";
import type {
    FlashboxRuntimeProxyType,
    FrameSupportDispatchDispatchInfo,
    FrameSupportTokensMiscBalanceStatus,
    SpRuntimeDispatchError,
    SpWeightsWeightV2Weight,
} from "@polkadot/types/lookup";

export type __AugmentedEvent<ApiType extends ApiTypes> = AugmentedEvent<ApiType>;

declare module "@polkadot/api-base/types/events" {
    interface AugmentedEvents<ApiType extends ApiTypes> {
        authorNoting: {
            /** Latest author changed */
            LatestAuthorChanged: AugmentedEvent<
                ApiType,
                [paraId: u32, blockNumber: u32, newAuthor: AccountId32],
                { paraId: u32; blockNumber: u32; newAuthor: AccountId32 }
            >;
            /** Removed author data */
            RemovedAuthorData: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        balances: {
            /** A balance was set by root. */
            BalanceSet: AugmentedEvent<ApiType, [who: AccountId32, free: u128], { who: AccountId32; free: u128 }>;
            /** Some amount was burned from an account. */
            Burned: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /** Some amount was deposited (e.g. for transaction fees). */
            Deposit: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /** An account was removed whose balance was non-zero but below ExistentialDeposit, resulting in an outright loss. */
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
            Frozen: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /** Total issuance was increased by `amount`, creating a credit to be balanced. */
            Issued: AugmentedEvent<ApiType, [amount: u128], { amount: u128 }>;
            /** Some balance was locked. */
            Locked: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /** Some amount was minted into an account. */
            Minted: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /** Total issuance was decreased by `amount`, creating a debt to be balanced. */
            Rescinded: AugmentedEvent<ApiType, [amount: u128], { amount: u128 }>;
            /** Some balance was reserved (moved from free to reserved). */
            Reserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Some balance was moved from the reserve of the first account to the second account. Final argument indicates
             * the destination balance type.
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
            Restored: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /** Some amount was removed from the account (e.g. for misbehavior). */
            Slashed: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /** Some amount was suspended from an account (it can be restored later). */
            Suspended: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /** Some balance was thawed. */
            Thawed: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /** Transfer succeeded. */
            Transfer: AugmentedEvent<
                ApiType,
                [from: AccountId32, to: AccountId32, amount: u128],
                { from: AccountId32; to: AccountId32; amount: u128 }
            >;
            /** Some balance was unlocked. */
            Unlocked: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /** Some balance was unreserved (moved from reserved to free). */
            Unreserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /** An account was upgraded. */
            Upgraded: AugmentedEvent<ApiType, [who: AccountId32], { who: AccountId32 }>;
            /** Some amount was withdrawn from the account (e.g. for transaction fees). */
            Withdraw: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        collatorAssignment: {
            NewPendingAssignment: AugmentedEvent<
                ApiType,
                [randomSeed: U8aFixed, fullRotation: bool, targetSession: u32],
                { randomSeed: U8aFixed; fullRotation: bool; targetSession: u32 }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        dataPreservers: {
            /** The list of boot_nodes changed. */
            BootNodesChanged: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        inflationRewards: {
            /** Rewarding container author */
            RewardedContainer: AugmentedEvent<
                ApiType,
                [accountId: AccountId32, paraId: u32, balance: u128],
                { accountId: AccountId32; paraId: u32; balance: u128 }
            >;
            /** Rewarding orchestrator author */
            RewardedOrchestrator: AugmentedEvent<
                ApiType,
                [accountId: AccountId32, balance: u128],
                { accountId: AccountId32; balance: u128 }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        invulnerables: {
            /**
             * An account was unable to be added to the Invulnerables because they did not have keys registered. Other
             * Invulnerables may have been set.
             */
            InvalidInvulnerableSkipped: AugmentedEvent<ApiType, [accountId: AccountId32], { accountId: AccountId32 }>;
            /** A new Invulnerable was added. */
            InvulnerableAdded: AugmentedEvent<ApiType, [accountId: AccountId32], { accountId: AccountId32 }>;
            /** An Invulnerable was removed. */
            InvulnerableRemoved: AugmentedEvent<ApiType, [accountId: AccountId32], { accountId: AccountId32 }>;
            /** New Invulnerables were set. */
            NewInvulnerables: AugmentedEvent<
                ApiType,
                [invulnerables: Vec<AccountId32>],
                { invulnerables: Vec<AccountId32> }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        maintenanceMode: {
            /** The chain was put into Maintenance Mode */
            EnteredMaintenanceMode: AugmentedEvent<ApiType, []>;
            /** The call to resume on_idle XCM execution failed with inner error */
            FailedToResumeIdleXcmExecution: AugmentedEvent<
                ApiType,
                [error: SpRuntimeDispatchError],
                { error: SpRuntimeDispatchError }
            >;
            /** The call to suspend on_idle XCM execution failed with inner error */
            FailedToSuspendIdleXcmExecution: AugmentedEvent<
                ApiType,
                [error: SpRuntimeDispatchError],
                { error: SpRuntimeDispatchError }
            >;
            /** The chain returned to its normal operating state */
            NormalOperationResumed: AugmentedEvent<ApiType, []>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        migrations: {
            /** XCM execution resume failed with inner error */
            FailedToResumeIdleXcmExecution: AugmentedEvent<
                ApiType,
                [error: SpRuntimeDispatchError],
                { error: SpRuntimeDispatchError }
            >;
            /** XCM execution suspension failed with inner error */
            FailedToSuspendIdleXcmExecution: AugmentedEvent<
                ApiType,
                [error: SpRuntimeDispatchError],
                { error: SpRuntimeDispatchError }
            >;
            /** Migration completed */
            MigrationCompleted: AugmentedEvent<
                ApiType,
                [migrationName: Bytes, consumedWeight: SpWeightsWeightV2Weight],
                { migrationName: Bytes; consumedWeight: SpWeightsWeightV2Weight }
            >;
            /** Migration started */
            MigrationStarted: AugmentedEvent<ApiType, [migrationName: Bytes], { migrationName: Bytes }>;
            /** Runtime upgrade completed */
            RuntimeUpgradeCompleted: AugmentedEvent<
                ApiType,
                [weight: SpWeightsWeightV2Weight],
                { weight: SpWeightsWeightV2Weight }
            >;
            /** Runtime upgrade started */
            RuntimeUpgradeStarted: AugmentedEvent<ApiType, []>;
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
            DownwardMessagesReceived: AugmentedEvent<ApiType, [count: u32], { count: u32 }>;
            /** An upgrade has been authorized. */
            UpgradeAuthorized: AugmentedEvent<ApiType, [codeHash: H256], { codeHash: H256 }>;
            /** An upward message was sent to the relay chain. */
            UpwardMessageSent: AugmentedEvent<
                ApiType,
                [messageHash: Option<U8aFixed>],
                { messageHash: Option<U8aFixed> }
            >;
            /** The validation function was applied as of the contained relay chain block number. */
            ValidationFunctionApplied: AugmentedEvent<ApiType, [relayChainBlockNum: u32], { relayChainBlockNum: u32 }>;
            /** The relay-chain aborted the upgrade process. */
            ValidationFunctionDiscarded: AugmentedEvent<ApiType, []>;
            /** The validation function has been scheduled to apply. */
            ValidationFunctionStored: AugmentedEvent<ApiType, []>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        proxy: {
            /** An announcement was placed to make a call in the future. */
            Announced: AugmentedEvent<
                ApiType,
                [real: AccountId32, proxy: AccountId32, callHash: H256],
                { real: AccountId32; proxy: AccountId32; callHash: H256 }
            >;
            /** A proxy was added. */
            ProxyAdded: AugmentedEvent<
                ApiType,
                [delegator: AccountId32, delegatee: AccountId32, proxyType: FlashboxRuntimeProxyType, delay: u32],
                { delegator: AccountId32; delegatee: AccountId32; proxyType: FlashboxRuntimeProxyType; delay: u32 }
            >;
            /** A proxy was executed correctly, with the given. */
            ProxyExecuted: AugmentedEvent<
                ApiType,
                [result: Result<Null, SpRuntimeDispatchError>],
                { result: Result<Null, SpRuntimeDispatchError> }
            >;
            /** A proxy was removed. */
            ProxyRemoved: AugmentedEvent<
                ApiType,
                [delegator: AccountId32, delegatee: AccountId32, proxyType: FlashboxRuntimeProxyType, delay: u32],
                { delegator: AccountId32; delegatee: AccountId32; proxyType: FlashboxRuntimeProxyType; delay: u32 }
            >;
            /** A pure account has been created by new proxy with given disambiguation index and proxy type. */
            PureCreated: AugmentedEvent<
                ApiType,
                [pure: AccountId32, who: AccountId32, proxyType: FlashboxRuntimeProxyType, disambiguationIndex: u16],
                { pure: AccountId32; who: AccountId32; proxyType: FlashboxRuntimeProxyType; disambiguationIndex: u16 }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        registrar: {
            /** A para id has been deregistered. [para_id] */
            ParaIdDeregistered: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /** A para id has been paused from collating. */
            ParaIdPaused: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /** A new para id has been registered. [para_id] */
            ParaIdRegistered: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /** A para id has been unpaused. */
            ParaIdUnpaused: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /** A new para id is now valid for collating. [para_id] */
            ParaIdValidForCollating: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        servicesPayment: {
            CreditBurned: AugmentedEvent<
                ApiType,
                [paraId: u32, creditsRemaining: u32],
                { paraId: u32; creditsRemaining: u32 }
            >;
            CreditsPurchased: AugmentedEvent<
                ApiType,
                [paraId: u32, payer: AccountId32, fee: u128, creditsPurchased: u32, creditsRemaining: u32],
                { paraId: u32; payer: AccountId32; fee: u128; creditsPurchased: u32; creditsRemaining: u32 }
            >;
            CreditsSet: AugmentedEvent<ApiType, [paraId: u32, credits: u32], { paraId: u32; credits: u32 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        session: {
            /** New session has happened. Note that the argument is the session index, not the block number as the type might suggest. */
            NewSession: AugmentedEvent<ApiType, [sessionIndex: u32], { sessionIndex: u32 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        sudo: {
            /** The sudo key has been updated. */
            KeyChanged: AugmentedEvent<ApiType, [oldSudoer: Option<AccountId32>], { oldSudoer: Option<AccountId32> }>;
            /** A sudo call just took place. */
            Sudid: AugmentedEvent<
                ApiType,
                [sudoResult: Result<Null, SpRuntimeDispatchError>],
                { sudoResult: Result<Null, SpRuntimeDispatchError> }
            >;
            /** A [sudo_as](Pallet::sudo_as) call just took place. */
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
                [dispatchError: SpRuntimeDispatchError, dispatchInfo: FrameSupportDispatchDispatchInfo],
                { dispatchError: SpRuntimeDispatchError; dispatchInfo: FrameSupportDispatchDispatchInfo }
            >;
            /** An extrinsic completed successfully. */
            ExtrinsicSuccess: AugmentedEvent<
                ApiType,
                [dispatchInfo: FrameSupportDispatchDispatchInfo],
                { dispatchInfo: FrameSupportDispatchDispatchInfo }
            >;
            /** An account was reaped. */
            KilledAccount: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            /** A new account was created. */
            NewAccount: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            /** On on-chain remark happened. */
            Remarked: AugmentedEvent<ApiType, [sender: AccountId32, hash_: H256], { sender: AccountId32; hash_: H256 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        transactionPayment: {
            /** A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee, has been paid by `who`. */
            TransactionFeePaid: AugmentedEvent<
                ApiType,
                [who: AccountId32, actualFee: u128, tip: u128],
                { who: AccountId32; actualFee: u128; tip: u128 }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        txPause: {
            /** This pallet, or a specific call is now paused. */
            CallPaused: AugmentedEvent<
                ApiType,
                [fullName: ITuple<[Bytes, Bytes]>],
                { fullName: ITuple<[Bytes, Bytes]> }
            >;
            /** This pallet, or a specific call is now unpaused. */
            CallUnpaused: AugmentedEvent<
                ApiType,
                [fullName: ITuple<[Bytes, Bytes]>],
                { fullName: ITuple<[Bytes, Bytes]> }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        utility: {
            /** Batch of dispatches completed fully with no error. */
            BatchCompleted: AugmentedEvent<ApiType, []>;
            /** Batch of dispatches completed but has errors. */
            BatchCompletedWithErrors: AugmentedEvent<ApiType, []>;
            /** Batch of dispatches did not complete fully. Index of first failing dispatch given, as well as the error. */
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
            ItemFailed: AugmentedEvent<ApiType, [error: SpRuntimeDispatchError], { error: SpRuntimeDispatchError }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
    } // AugmentedEvents
} // declare module
