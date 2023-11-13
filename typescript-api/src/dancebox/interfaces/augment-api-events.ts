// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/events";

import type { ApiTypes, AugmentedEvent } from "@polkadot/api-base/types";
import type { Bytes, Null, Option, Result, U8aFixed, Vec, bool, u128, u16, u32, u64, u8 } from "@polkadot/types-codec";
import type { AccountId32, H256 } from "@polkadot/types/interfaces/runtime";
import type {
    DanceboxRuntimeProxyType,
    FrameSupportDispatchDispatchInfo,
    FrameSupportTokensMiscBalanceStatus,
    PalletPooledStakingTargetPool,
    SpRuntimeDispatchError,
    SpWeightsWeightV2Weight,
    StagingXcmV3MultiLocation,
    StagingXcmV3MultiassetMultiAssets,
    StagingXcmV3Response,
    StagingXcmV3TraitsError,
    StagingXcmV3TraitsOutcome,
    StagingXcmV3Xcm,
    StagingXcmVersionedMultiAssets,
    StagingXcmVersionedMultiLocation,
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
        cumulusXcm: {
            /** Downward message executed with the given outcome. [ id, outcome ] */
            ExecutedDownward: AugmentedEvent<ApiType, [U8aFixed, StagingXcmV3TraitsOutcome]>;
            /** Downward message is invalid XCM. [ id ] */
            InvalidFormat: AugmentedEvent<ApiType, [U8aFixed]>;
            /** Downward message is unsupported version of XCM. [ id ] */
            UnsupportedVersion: AugmentedEvent<ApiType, [U8aFixed]>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        dmpQueue: {
            /** Downward message executed with the given outcome. */
            ExecutedDownward: AugmentedEvent<
                ApiType,
                [messageHash: U8aFixed, messageId: U8aFixed, outcome: StagingXcmV3TraitsOutcome],
                { messageHash: U8aFixed; messageId: U8aFixed; outcome: StagingXcmV3TraitsOutcome }
            >;
            /** Downward message is invalid XCM. */
            InvalidFormat: AugmentedEvent<ApiType, [messageHash: U8aFixed], { messageHash: U8aFixed }>;
            /** The maximum number of downward messages was reached. */
            MaxMessagesExhausted: AugmentedEvent<ApiType, [messageHash: U8aFixed], { messageHash: U8aFixed }>;
            /** Downward message is overweight and was placed in the overweight queue. */
            OverweightEnqueued: AugmentedEvent<
                ApiType,
                [
                    messageHash: U8aFixed,
                    messageId: U8aFixed,
                    overweightIndex: u64,
                    requiredWeight: SpWeightsWeightV2Weight
                ],
                {
                    messageHash: U8aFixed;
                    messageId: U8aFixed;
                    overweightIndex: u64;
                    requiredWeight: SpWeightsWeightV2Weight;
                }
            >;
            /** Downward message from the overweight queue was executed. */
            OverweightServiced: AugmentedEvent<
                ApiType,
                [overweightIndex: u64, weightUsed: SpWeightsWeightV2Weight],
                { overweightIndex: u64; weightUsed: SpWeightsWeightV2Weight }
            >;
            /** Downward message is unsupported version of XCM. */
            UnsupportedVersion: AugmentedEvent<ApiType, [messageHash: U8aFixed], { messageHash: U8aFixed }>;
            /** The weight limit for handling downward messages was reached. */
            WeightExhausted: AugmentedEvent<
                ApiType,
                [
                    messageHash: U8aFixed,
                    messageId: U8aFixed,
                    remainingWeight: SpWeightsWeightV2Weight,
                    requiredWeight: SpWeightsWeightV2Weight
                ],
                {
                    messageHash: U8aFixed;
                    messageId: U8aFixed;
                    remainingWeight: SpWeightsWeightV2Weight;
                    requiredWeight: SpWeightsWeightV2Weight;
                }
            >;
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
        polkadotXcm: {
            /** Some assets have been claimed from an asset trap */
            AssetsClaimed: AugmentedEvent<
                ApiType,
                [hash_: H256, origin: StagingXcmV3MultiLocation, assets: StagingXcmVersionedMultiAssets],
                { hash_: H256; origin: StagingXcmV3MultiLocation; assets: StagingXcmVersionedMultiAssets }
            >;
            /** Some assets have been placed in an asset trap. */
            AssetsTrapped: AugmentedEvent<
                ApiType,
                [hash_: H256, origin: StagingXcmV3MultiLocation, assets: StagingXcmVersionedMultiAssets],
                { hash_: H256; origin: StagingXcmV3MultiLocation; assets: StagingXcmVersionedMultiAssets }
            >;
            /** Execution of an XCM message was attempted. */
            Attempted: AugmentedEvent<
                ApiType,
                [outcome: StagingXcmV3TraitsOutcome],
                { outcome: StagingXcmV3TraitsOutcome }
            >;
            /** Fees were paid from a location for an operation (often for using `SendXcm`). */
            FeesPaid: AugmentedEvent<
                ApiType,
                [paying: StagingXcmV3MultiLocation, fees: StagingXcmV3MultiassetMultiAssets],
                { paying: StagingXcmV3MultiLocation; fees: StagingXcmV3MultiassetMultiAssets }
            >;
            /**
             * Expected query response has been received but the querier location of the response does not match the expected.
             * The query remains registered for a later, valid, response to be received and acted upon.
             */
            InvalidQuerier: AugmentedEvent<
                ApiType,
                [
                    origin: StagingXcmV3MultiLocation,
                    queryId: u64,
                    expectedQuerier: StagingXcmV3MultiLocation,
                    maybeActualQuerier: Option<StagingXcmV3MultiLocation>
                ],
                {
                    origin: StagingXcmV3MultiLocation;
                    queryId: u64;
                    expectedQuerier: StagingXcmV3MultiLocation;
                    maybeActualQuerier: Option<StagingXcmV3MultiLocation>;
                }
            >;
            /**
             * Expected query response has been received but the expected querier location placed in storage by this runtime
             * previously cannot be decoded. The query remains registered.
             *
             * This is unexpected (since a location placed in storage in a previously executing runtime should be readable
             * prior to query timeout) and dangerous since the possibly valid response will be dropped. Manual governance
             * intervention is probably going to be needed.
             */
            InvalidQuerierVersion: AugmentedEvent<
                ApiType,
                [origin: StagingXcmV3MultiLocation, queryId: u64],
                { origin: StagingXcmV3MultiLocation; queryId: u64 }
            >;
            /**
             * Expected query response has been received but the origin location of the response does not match that expected.
             * The query remains registered for a later, valid, response to be received and acted upon.
             */
            InvalidResponder: AugmentedEvent<
                ApiType,
                [origin: StagingXcmV3MultiLocation, queryId: u64, expectedLocation: Option<StagingXcmV3MultiLocation>],
                { origin: StagingXcmV3MultiLocation; queryId: u64; expectedLocation: Option<StagingXcmV3MultiLocation> }
            >;
            /**
             * Expected query response has been received but the expected origin location placed in storage by this runtime
             * previously cannot be decoded. The query remains registered.
             *
             * This is unexpected (since a location placed in storage in a previously executing runtime should be readable
             * prior to query timeout) and dangerous since the possibly valid response will be dropped. Manual governance
             * intervention is probably going to be needed.
             */
            InvalidResponderVersion: AugmentedEvent<
                ApiType,
                [origin: StagingXcmV3MultiLocation, queryId: u64],
                { origin: StagingXcmV3MultiLocation; queryId: u64 }
            >;
            /**
             * Query response has been received and query is removed. The registered notification has been dispatched and
             * executed successfully.
             */
            Notified: AugmentedEvent<
                ApiType,
                [queryId: u64, palletIndex: u8, callIndex: u8],
                { queryId: u64; palletIndex: u8; callIndex: u8 }
            >;
            /**
             * Query response has been received and query is removed. The dispatch was unable to be decoded into a `Call`;
             * this might be due to dispatch function having a signature which is not `(origin, QueryId, Response)`.
             */
            NotifyDecodeFailed: AugmentedEvent<
                ApiType,
                [queryId: u64, palletIndex: u8, callIndex: u8],
                { queryId: u64; palletIndex: u8; callIndex: u8 }
            >;
            /** Query response has been received and query is removed. There was a general error with dispatching the notification call. */
            NotifyDispatchError: AugmentedEvent<
                ApiType,
                [queryId: u64, palletIndex: u8, callIndex: u8],
                { queryId: u64; palletIndex: u8; callIndex: u8 }
            >;
            /**
             * Query response has been received and query is removed. The registered notification could not be dispatched
             * because the dispatch weight is greater than the maximum weight originally budgeted by this runtime for the query result.
             */
            NotifyOverweight: AugmentedEvent<
                ApiType,
                [
                    queryId: u64,
                    palletIndex: u8,
                    callIndex: u8,
                    actualWeight: SpWeightsWeightV2Weight,
                    maxBudgetedWeight: SpWeightsWeightV2Weight
                ],
                {
                    queryId: u64;
                    palletIndex: u8;
                    callIndex: u8;
                    actualWeight: SpWeightsWeightV2Weight;
                    maxBudgetedWeight: SpWeightsWeightV2Weight;
                }
            >;
            /**
             * A given location which had a version change subscription was dropped owing to an error migrating the location
             * to our new XCM format.
             */
            NotifyTargetMigrationFail: AugmentedEvent<
                ApiType,
                [location: StagingXcmVersionedMultiLocation, queryId: u64],
                { location: StagingXcmVersionedMultiLocation; queryId: u64 }
            >;
            /** A given location which had a version change subscription was dropped owing to an error sending the notification to it. */
            NotifyTargetSendFail: AugmentedEvent<
                ApiType,
                [location: StagingXcmV3MultiLocation, queryId: u64, error: StagingXcmV3TraitsError],
                { location: StagingXcmV3MultiLocation; queryId: u64; error: StagingXcmV3TraitsError }
            >;
            /** Query response has been received and is ready for taking with `take_response`. There is no registered notification call. */
            ResponseReady: AugmentedEvent<
                ApiType,
                [queryId: u64, response: StagingXcmV3Response],
                { queryId: u64; response: StagingXcmV3Response }
            >;
            /** Received query response has been read and removed. */
            ResponseTaken: AugmentedEvent<ApiType, [queryId: u64], { queryId: u64 }>;
            /** A XCM message was sent. */
            Sent: AugmentedEvent<
                ApiType,
                [
                    origin: StagingXcmV3MultiLocation,
                    destination: StagingXcmV3MultiLocation,
                    message: StagingXcmV3Xcm,
                    messageId: U8aFixed
                ],
                {
                    origin: StagingXcmV3MultiLocation;
                    destination: StagingXcmV3MultiLocation;
                    message: StagingXcmV3Xcm;
                    messageId: U8aFixed;
                }
            >;
            /**
             * The supported version of a location has been changed. This might be through an automatic notification or a
             * manual intervention.
             */
            SupportedVersionChanged: AugmentedEvent<
                ApiType,
                [location: StagingXcmV3MultiLocation, version: u32],
                { location: StagingXcmV3MultiLocation; version: u32 }
            >;
            /**
             * Query response received which does not match a registered query. This may be because a matching query was never
             * registered, it may be because it is a duplicate response, or because the query timed out.
             */
            UnexpectedResponse: AugmentedEvent<
                ApiType,
                [origin: StagingXcmV3MultiLocation, queryId: u64],
                { origin: StagingXcmV3MultiLocation; queryId: u64 }
            >;
            /**
             * An XCM version change notification message has been attempted to be sent.
             *
             * The cost of sending it (borne by the chain) is included.
             */
            VersionChangeNotified: AugmentedEvent<
                ApiType,
                [
                    destination: StagingXcmV3MultiLocation,
                    result: u32,
                    cost: StagingXcmV3MultiassetMultiAssets,
                    messageId: U8aFixed
                ],
                {
                    destination: StagingXcmV3MultiLocation;
                    result: u32;
                    cost: StagingXcmV3MultiassetMultiAssets;
                    messageId: U8aFixed;
                }
            >;
            /** We have requested that a remote chain send us XCM version change notifications. */
            VersionNotifyRequested: AugmentedEvent<
                ApiType,
                [destination: StagingXcmV3MultiLocation, cost: StagingXcmV3MultiassetMultiAssets, messageId: U8aFixed],
                { destination: StagingXcmV3MultiLocation; cost: StagingXcmV3MultiassetMultiAssets; messageId: U8aFixed }
            >;
            /**
             * A remote has requested XCM version change notification from us and we have honored it. A version information
             * message is sent to them and its cost is included.
             */
            VersionNotifyStarted: AugmentedEvent<
                ApiType,
                [destination: StagingXcmV3MultiLocation, cost: StagingXcmV3MultiassetMultiAssets, messageId: U8aFixed],
                { destination: StagingXcmV3MultiLocation; cost: StagingXcmV3MultiassetMultiAssets; messageId: U8aFixed }
            >;
            /** We have requested that a remote chain stops sending us XCM version change notifications. */
            VersionNotifyUnrequested: AugmentedEvent<
                ApiType,
                [destination: StagingXcmV3MultiLocation, cost: StagingXcmV3MultiassetMultiAssets, messageId: U8aFixed],
                { destination: StagingXcmV3MultiLocation; cost: StagingXcmV3MultiassetMultiAssets; messageId: U8aFixed }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        pooledStaking: {
            /** Rewards manually claimed. */
            ClaimedManualRewards: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, rewards: u128],
                { candidate: AccountId32; delegator: AccountId32; rewards: u128 }
            >;
            /** Stake of that Candidate decreased. */
            DecreasedStake: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, stakeDiff: u128],
                { candidate: AccountId32; stakeDiff: u128 }
            >;
            /**
             * Delegation request was executed. `staked` has been properly staked in `pool`, while the rounding when
             * converting to shares has been `released`.
             */
            ExecutedDelegate: AugmentedEvent<
                ApiType,
                [
                    candidate: AccountId32,
                    delegator: AccountId32,
                    pool: PalletPooledStakingTargetPool,
                    staked: u128,
                    released: u128
                ],
                {
                    candidate: AccountId32;
                    delegator: AccountId32;
                    pool: PalletPooledStakingTargetPool;
                    staked: u128;
                    released: u128;
                }
            >;
            /** Undelegation request was executed. */
            ExecutedUndelegate: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, released: u128],
                { candidate: AccountId32; delegator: AccountId32; released: u128 }
            >;
            /** Stake of that Candidate increased. */
            IncreasedStake: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, stakeDiff: u128],
                { candidate: AccountId32; stakeDiff: u128 }
            >;
            /** User requested to delegate towards a candidate. */
            RequestedDelegate: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, pool: PalletPooledStakingTargetPool, pending: u128],
                { candidate: AccountId32; delegator: AccountId32; pool: PalletPooledStakingTargetPool; pending: u128 }
            >;
            /**
             * User requested to undelegate from a candidate. Stake was removed from a `pool` and is `pending` for the request
             * to be executed. The rounding when converting to leaving shares has been `released` immediately.
             */
            RequestedUndelegate: AugmentedEvent<
                ApiType,
                [
                    candidate: AccountId32,
                    delegator: AccountId32,
                    from: PalletPooledStakingTargetPool,
                    pending: u128,
                    released: u128
                ],
                {
                    candidate: AccountId32;
                    delegator: AccountId32;
                    from: PalletPooledStakingTargetPool;
                    pending: u128;
                    released: u128;
                }
            >;
            /** Collator has been rewarded. */
            RewardedCollator: AugmentedEvent<
                ApiType,
                [collator: AccountId32, autoCompoundingRewards: u128, manualClaimRewards: u128],
                { collator: AccountId32; autoCompoundingRewards: u128; manualClaimRewards: u128 }
            >;
            /** Delegators have been rewarded. */
            RewardedDelegators: AugmentedEvent<
                ApiType,
                [collator: AccountId32, autoCompoundingRewards: u128, manualClaimRewards: u128],
                { collator: AccountId32; autoCompoundingRewards: u128; manualClaimRewards: u128 }
            >;
            /** Delegator staked towards a Candidate for AutoCompounding Shares. */
            StakedAutoCompounding: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, shares: u128, stake: u128],
                { candidate: AccountId32; delegator: AccountId32; shares: u128; stake: u128 }
            >;
            /** Delegator staked towards a candidate for ManualRewards Shares. */
            StakedManualRewards: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, shares: u128, stake: u128],
                { candidate: AccountId32; delegator: AccountId32; shares: u128; stake: u128 }
            >;
            /** Swapped between AutoCompounding and ManualReward shares */
            SwappedPool: AugmentedEvent<
                ApiType,
                [
                    candidate: AccountId32,
                    delegator: AccountId32,
                    sourcePool: PalletPooledStakingTargetPool,
                    sourceShares: u128,
                    sourceStake: u128,
                    targetShares: u128,
                    targetStake: u128,
                    pendingLeaving: u128,
                    released: u128
                ],
                {
                    candidate: AccountId32;
                    delegator: AccountId32;
                    sourcePool: PalletPooledStakingTargetPool;
                    sourceShares: u128;
                    sourceStake: u128;
                    targetShares: u128;
                    targetStake: u128;
                    pendingLeaving: u128;
                    released: u128;
                }
            >;
            /** Delegator unstaked towards a candidate with AutoCompounding Shares. */
            UnstakedAutoCompounding: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, shares: u128, stake: u128],
                { candidate: AccountId32; delegator: AccountId32; shares: u128; stake: u128 }
            >;
            /** Delegator unstaked towards a candidate with ManualRewards Shares. */
            UnstakedManualRewards: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, shares: u128, stake: u128],
                { candidate: AccountId32; delegator: AccountId32; shares: u128; stake: u128 }
            >;
            /** Stake of the candidate has changed, which may have modified its position in the eligible candidates list. */
            UpdatedCandidatePosition: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, stake: u128, selfDelegation: u128, before: Option<u32>, after: Option<u32>],
                { candidate: AccountId32; stake: u128; selfDelegation: u128; before: Option<u32>; after: Option<u32> }
            >;
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
                [delegator: AccountId32, delegatee: AccountId32, proxyType: DanceboxRuntimeProxyType, delay: u32],
                { delegator: AccountId32; delegatee: AccountId32; proxyType: DanceboxRuntimeProxyType; delay: u32 }
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
                [delegator: AccountId32, delegatee: AccountId32, proxyType: DanceboxRuntimeProxyType, delay: u32],
                { delegator: AccountId32; delegatee: AccountId32; proxyType: DanceboxRuntimeProxyType; delay: u32 }
            >;
            /** A pure account has been created by new proxy with given disambiguation index and proxy type. */
            PureCreated: AugmentedEvent<
                ApiType,
                [pure: AccountId32, who: AccountId32, proxyType: DanceboxRuntimeProxyType, disambiguationIndex: u16],
                { pure: AccountId32; who: AccountId32; proxyType: DanceboxRuntimeProxyType; disambiguationIndex: u16 }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        registrar: {
            /** The list of boot_nodes */
            BootNodesChanged: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /** A para id has been deregistered. [para_id] */
            ParaIdDeregistered: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /** A para id has been paused from collating. */
            ParaIdPaused: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /** A new para id has been registered. [para_id] */
            ParaIdRegistered: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
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
            /** The [sudoer] just switched identity; the old key is supplied if one existed. */
            KeyChanged: AugmentedEvent<ApiType, [oldSudoer: Option<AccountId32>], { oldSudoer: Option<AccountId32> }>;
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
        xcmpQueue: {
            /** Bad XCM format used. */
            BadFormat: AugmentedEvent<ApiType, [messageHash: U8aFixed], { messageHash: U8aFixed }>;
            /** Bad XCM version used. */
            BadVersion: AugmentedEvent<ApiType, [messageHash: U8aFixed], { messageHash: U8aFixed }>;
            /** Some XCM failed. */
            Fail: AugmentedEvent<
                ApiType,
                [
                    messageHash: U8aFixed,
                    messageId: U8aFixed,
                    error: StagingXcmV3TraitsError,
                    weight: SpWeightsWeightV2Weight
                ],
                {
                    messageHash: U8aFixed;
                    messageId: U8aFixed;
                    error: StagingXcmV3TraitsError;
                    weight: SpWeightsWeightV2Weight;
                }
            >;
            /** An XCM exceeded the individual message weight budget. */
            OverweightEnqueued: AugmentedEvent<
                ApiType,
                [sender: u32, sentAt: u32, index: u64, required: SpWeightsWeightV2Weight],
                { sender: u32; sentAt: u32; index: u64; required: SpWeightsWeightV2Weight }
            >;
            /** An XCM from the overweight queue was executed with the given actual weight used. */
            OverweightServiced: AugmentedEvent<
                ApiType,
                [index: u64, used: SpWeightsWeightV2Weight],
                { index: u64; used: SpWeightsWeightV2Weight }
            >;
            /** Some XCM was executed ok. */
            Success: AugmentedEvent<
                ApiType,
                [messageHash: U8aFixed, messageId: U8aFixed, weight: SpWeightsWeightV2Weight],
                { messageHash: U8aFixed; messageId: U8aFixed; weight: SpWeightsWeightV2Weight }
            >;
            /** An HRMP message was sent to a sibling parachain. */
            XcmpMessageSent: AugmentedEvent<ApiType, [messageHash: U8aFixed], { messageHash: U8aFixed }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
    } // AugmentedEvents
} // declare module
