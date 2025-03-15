// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/events";

import type { ApiTypes, AugmentedEvent } from "@polkadot/api-base/types";
import type { Bytes, Null, Option, Result, U8aFixed, Vec, bool, u128, u16, u32, u64, u8 } from "@polkadot/types-codec";
import type { ITuple } from "@polkadot/types-codec/types";
import type { AccountId32, H256 } from "@polkadot/types/interfaces/runtime";
import type {
    CumulusPrimitivesCoreAggregateMessageOrigin,
    DanceboxRuntimeProxyType,
    FrameSupportMessagesProcessMessageError,
    FrameSupportTokensMiscBalanceStatus,
    FrameSystemDispatchEventInfo,
    PalletMultisigTimepoint,
    PalletPooledStakingTargetPool,
    PalletStreamPaymentDepositChange,
    PalletStreamPaymentParty,
    PalletStreamPaymentStreamConfig,
    SpRuntimeDispatchError,
    SpWeightsWeightV2Weight,
    StagingXcmV5AssetAssets,
    StagingXcmV5Location,
    StagingXcmV5Response,
    StagingXcmV5TraitsOutcome,
    StagingXcmV5Xcm,
    TpTraitsFullRotationModes,
    XcmV5TraitsError,
    XcmVersionedAssets,
    XcmVersionedLocation,
} from "@polkadot/types/lookup";

export type __AugmentedEvent<ApiType extends ApiTypes> = AugmentedEvent<ApiType>;

declare module "@polkadot/api-base/types/events" {
    interface AugmentedEvents<ApiType extends ApiTypes> {
        assetRate: {
            AssetRateCreated: AugmentedEvent<ApiType, [assetKind: u16, rate: u128], { assetKind: u16; rate: u128 }>;
            AssetRateRemoved: AugmentedEvent<ApiType, [assetKind: u16], { assetKind: u16 }>;
            AssetRateUpdated: AugmentedEvent<
                ApiType,
                [assetKind: u16, old: u128, new_: u128],
                { assetKind: u16; old: u128; new_: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        authorNoting: {
            /**
             * Latest author changed
             **/
            LatestAuthorChanged: AugmentedEvent<
                ApiType,
                [paraId: u32, blockNumber: u32, newAuthor: AccountId32, latestSlotNumber: u64],
                { paraId: u32; blockNumber: u32; newAuthor: AccountId32; latestSlotNumber: u64 }
            >;
            /**
             * Removed author data
             **/
            RemovedAuthorData: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        balances: {
            /**
             * A balance was set by root.
             **/
            BalanceSet: AugmentedEvent<ApiType, [who: AccountId32, free: u128], { who: AccountId32; free: u128 }>;
            /**
             * Some amount was burned from an account.
             **/
            Burned: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Some amount was deposited (e.g. for transaction fees).
             **/
            Deposit: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * An account was removed whose balance was non-zero but below ExistentialDeposit,
             * resulting in an outright loss.
             **/
            DustLost: AugmentedEvent<
                ApiType,
                [account: AccountId32, amount: u128],
                { account: AccountId32; amount: u128 }
            >;
            /**
             * An account was created with some free balance.
             **/
            Endowed: AugmentedEvent<
                ApiType,
                [account: AccountId32, freeBalance: u128],
                { account: AccountId32; freeBalance: u128 }
            >;
            /**
             * Some balance was frozen.
             **/
            Frozen: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Total issuance was increased by `amount`, creating a credit to be balanced.
             **/
            Issued: AugmentedEvent<ApiType, [amount: u128], { amount: u128 }>;
            /**
             * Some balance was locked.
             **/
            Locked: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Some amount was minted into an account.
             **/
            Minted: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Total issuance was decreased by `amount`, creating a debt to be balanced.
             **/
            Rescinded: AugmentedEvent<ApiType, [amount: u128], { amount: u128 }>;
            /**
             * Some balance was reserved (moved from free to reserved).
             **/
            Reserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Some balance was moved from the reserve of the first account to the second account.
             * Final argument indicates the destination balance type.
             **/
            ReserveRepatriated: AugmentedEvent<
                ApiType,
                [
                    from: AccountId32,
                    to: AccountId32,
                    amount: u128,
                    destinationStatus: FrameSupportTokensMiscBalanceStatus,
                ],
                {
                    from: AccountId32;
                    to: AccountId32;
                    amount: u128;
                    destinationStatus: FrameSupportTokensMiscBalanceStatus;
                }
            >;
            /**
             * Some amount was restored into an account.
             **/
            Restored: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Some amount was removed from the account (e.g. for misbehavior).
             **/
            Slashed: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Some amount was suspended from an account (it can be restored later).
             **/
            Suspended: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Some balance was thawed.
             **/
            Thawed: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * The `TotalIssuance` was forcefully changed.
             **/
            TotalIssuanceForced: AugmentedEvent<ApiType, [old: u128, new_: u128], { old: u128; new_: u128 }>;
            /**
             * Transfer succeeded.
             **/
            Transfer: AugmentedEvent<
                ApiType,
                [from: AccountId32, to: AccountId32, amount: u128],
                { from: AccountId32; to: AccountId32; amount: u128 }
            >;
            /**
             * Some balance was unlocked.
             **/
            Unlocked: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Some balance was unreserved (moved from reserved to free).
             **/
            Unreserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * An account was upgraded.
             **/
            Upgraded: AugmentedEvent<ApiType, [who: AccountId32], { who: AccountId32 }>;
            /**
             * Some amount was withdrawn from the account (e.g. for transaction fees).
             **/
            Withdraw: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        collatorAssignment: {
            NewPendingAssignment: AugmentedEvent<
                ApiType,
                [
                    randomSeed: U8aFixed,
                    fullRotation: bool,
                    targetSession: u32,
                    fullRotationMode: TpTraitsFullRotationModes,
                ],
                {
                    randomSeed: U8aFixed;
                    fullRotation: bool;
                    targetSession: u32;
                    fullRotationMode: TpTraitsFullRotationModes;
                }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        cumulusXcm: {
            /**
             * Downward message executed with the given outcome.
             * \[ id, outcome \]
             **/
            ExecutedDownward: AugmentedEvent<ApiType, [U8aFixed, StagingXcmV5TraitsOutcome]>;
            /**
             * Downward message is invalid XCM.
             * \[ id \]
             **/
            InvalidFormat: AugmentedEvent<ApiType, [U8aFixed]>;
            /**
             * Downward message is unsupported version of XCM.
             * \[ id \]
             **/
            UnsupportedVersion: AugmentedEvent<ApiType, [U8aFixed]>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        dataPreservers: {
            AssignmentStarted: AugmentedEvent<ApiType, [profileId: u64, paraId: u32], { profileId: u64; paraId: u32 }>;
            AssignmentStopped: AugmentedEvent<ApiType, [profileId: u64, paraId: u32], { profileId: u64; paraId: u32 }>;
            /**
             * The list of boot_nodes changed.
             **/
            BootNodesChanged: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            ProfileCreated: AugmentedEvent<
                ApiType,
                [account: AccountId32, profileId: u64, deposit: u128],
                { account: AccountId32; profileId: u64; deposit: u128 }
            >;
            ProfileDeleted: AugmentedEvent<
                ApiType,
                [profileId: u64, releasedDeposit: u128],
                { profileId: u64; releasedDeposit: u128 }
            >;
            ProfileUpdated: AugmentedEvent<
                ApiType,
                [profileId: u64, oldDeposit: u128, newDeposit: u128],
                { profileId: u64; oldDeposit: u128; newDeposit: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        foreignAssets: {
            /**
             * Accounts were destroyed for given asset.
             **/
            AccountsDestroyed: AugmentedEvent<
                ApiType,
                [assetId: u16, accountsDestroyed: u32, accountsRemaining: u32],
                { assetId: u16; accountsDestroyed: u32; accountsRemaining: u32 }
            >;
            /**
             * An approval for account `delegate` was cancelled by `owner`.
             **/
            ApprovalCancelled: AugmentedEvent<
                ApiType,
                [assetId: u16, owner: AccountId32, delegate: AccountId32],
                { assetId: u16; owner: AccountId32; delegate: AccountId32 }
            >;
            /**
             * Approvals were destroyed for given asset.
             **/
            ApprovalsDestroyed: AugmentedEvent<
                ApiType,
                [assetId: u16, approvalsDestroyed: u32, approvalsRemaining: u32],
                { assetId: u16; approvalsDestroyed: u32; approvalsRemaining: u32 }
            >;
            /**
             * (Additional) funds have been approved for transfer to a destination account.
             **/
            ApprovedTransfer: AugmentedEvent<
                ApiType,
                [assetId: u16, source: AccountId32, delegate: AccountId32, amount: u128],
                { assetId: u16; source: AccountId32; delegate: AccountId32; amount: u128 }
            >;
            /**
             * Some asset `asset_id` was frozen.
             **/
            AssetFrozen: AugmentedEvent<ApiType, [assetId: u16], { assetId: u16 }>;
            /**
             * The min_balance of an asset has been updated by the asset owner.
             **/
            AssetMinBalanceChanged: AugmentedEvent<
                ApiType,
                [assetId: u16, newMinBalance: u128],
                { assetId: u16; newMinBalance: u128 }
            >;
            /**
             * An asset has had its attributes changed by the `Force` origin.
             **/
            AssetStatusChanged: AugmentedEvent<ApiType, [assetId: u16], { assetId: u16 }>;
            /**
             * Some asset `asset_id` was thawed.
             **/
            AssetThawed: AugmentedEvent<ApiType, [assetId: u16], { assetId: u16 }>;
            /**
             * Some account `who` was blocked.
             **/
            Blocked: AugmentedEvent<ApiType, [assetId: u16, who: AccountId32], { assetId: u16; who: AccountId32 }>;
            /**
             * Some assets were destroyed.
             **/
            Burned: AugmentedEvent<
                ApiType,
                [assetId: u16, owner: AccountId32, balance: u128],
                { assetId: u16; owner: AccountId32; balance: u128 }
            >;
            /**
             * Some asset class was created.
             **/
            Created: AugmentedEvent<
                ApiType,
                [assetId: u16, creator: AccountId32, owner: AccountId32],
                { assetId: u16; creator: AccountId32; owner: AccountId32 }
            >;
            /**
             * Some assets were deposited (e.g. for transaction fees).
             **/
            Deposited: AugmentedEvent<
                ApiType,
                [assetId: u16, who: AccountId32, amount: u128],
                { assetId: u16; who: AccountId32; amount: u128 }
            >;
            /**
             * An asset class was destroyed.
             **/
            Destroyed: AugmentedEvent<ApiType, [assetId: u16], { assetId: u16 }>;
            /**
             * An asset class is in the process of being destroyed.
             **/
            DestructionStarted: AugmentedEvent<ApiType, [assetId: u16], { assetId: u16 }>;
            /**
             * Some asset class was force-created.
             **/
            ForceCreated: AugmentedEvent<
                ApiType,
                [assetId: u16, owner: AccountId32],
                { assetId: u16; owner: AccountId32 }
            >;
            /**
             * Some account `who` was frozen.
             **/
            Frozen: AugmentedEvent<ApiType, [assetId: u16, who: AccountId32], { assetId: u16; who: AccountId32 }>;
            /**
             * Some assets were issued.
             **/
            Issued: AugmentedEvent<
                ApiType,
                [assetId: u16, owner: AccountId32, amount: u128],
                { assetId: u16; owner: AccountId32; amount: u128 }
            >;
            /**
             * Metadata has been cleared for an asset.
             **/
            MetadataCleared: AugmentedEvent<ApiType, [assetId: u16], { assetId: u16 }>;
            /**
             * New metadata has been set for an asset.
             **/
            MetadataSet: AugmentedEvent<
                ApiType,
                [assetId: u16, name: Bytes, symbol_: Bytes, decimals: u8, isFrozen: bool],
                { assetId: u16; name: Bytes; symbol: Bytes; decimals: u8; isFrozen: bool }
            >;
            /**
             * The owner changed.
             **/
            OwnerChanged: AugmentedEvent<
                ApiType,
                [assetId: u16, owner: AccountId32],
                { assetId: u16; owner: AccountId32 }
            >;
            /**
             * The management team changed.
             **/
            TeamChanged: AugmentedEvent<
                ApiType,
                [assetId: u16, issuer: AccountId32, admin: AccountId32, freezer: AccountId32],
                { assetId: u16; issuer: AccountId32; admin: AccountId32; freezer: AccountId32 }
            >;
            /**
             * Some account `who` was thawed.
             **/
            Thawed: AugmentedEvent<ApiType, [assetId: u16, who: AccountId32], { assetId: u16; who: AccountId32 }>;
            /**
             * Some account `who` was created with a deposit from `depositor`.
             **/
            Touched: AugmentedEvent<
                ApiType,
                [assetId: u16, who: AccountId32, depositor: AccountId32],
                { assetId: u16; who: AccountId32; depositor: AccountId32 }
            >;
            /**
             * Some assets were transferred.
             **/
            Transferred: AugmentedEvent<
                ApiType,
                [assetId: u16, from: AccountId32, to: AccountId32, amount: u128],
                { assetId: u16; from: AccountId32; to: AccountId32; amount: u128 }
            >;
            /**
             * An `amount` was transferred in its entirety from `owner` to `destination` by
             * the approved `delegate`.
             **/
            TransferredApproved: AugmentedEvent<
                ApiType,
                [assetId: u16, owner: AccountId32, delegate: AccountId32, destination: AccountId32, amount: u128],
                { assetId: u16; owner: AccountId32; delegate: AccountId32; destination: AccountId32; amount: u128 }
            >;
            /**
             * Some assets were withdrawn from the account (e.g. for transaction fees).
             **/
            Withdrawn: AugmentedEvent<
                ApiType,
                [assetId: u16, who: AccountId32, amount: u128],
                { assetId: u16; who: AccountId32; amount: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        foreignAssetsCreator: {
            /**
             * New asset with the asset manager is registered
             **/
            ForeignAssetCreated: AugmentedEvent<
                ApiType,
                [assetId: u16, foreignAsset: StagingXcmV5Location],
                { assetId: u16; foreignAsset: StagingXcmV5Location }
            >;
            /**
             * Removed all information related to an assetId and destroyed asset
             **/
            ForeignAssetDestroyed: AugmentedEvent<
                ApiType,
                [assetId: u16, foreignAsset: StagingXcmV5Location],
                { assetId: u16; foreignAsset: StagingXcmV5Location }
            >;
            /**
             * Removed all information related to an assetId
             **/
            ForeignAssetRemoved: AugmentedEvent<
                ApiType,
                [assetId: u16, foreignAsset: StagingXcmV5Location],
                { assetId: u16; foreignAsset: StagingXcmV5Location }
            >;
            /**
             * Changed the xcm type mapping for a given asset id
             **/
            ForeignAssetTypeChanged: AugmentedEvent<
                ApiType,
                [assetId: u16, newForeignAsset: StagingXcmV5Location],
                { assetId: u16; newForeignAsset: StagingXcmV5Location }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        identity: {
            /**
             * A username authority was added.
             **/
            AuthorityAdded: AugmentedEvent<ApiType, [authority: AccountId32], { authority: AccountId32 }>;
            /**
             * A username authority was removed.
             **/
            AuthorityRemoved: AugmentedEvent<ApiType, [authority: AccountId32], { authority: AccountId32 }>;
            /**
             * A dangling username (as in, a username corresponding to an account that has removed its
             * identity) has been removed.
             **/
            DanglingUsernameRemoved: AugmentedEvent<
                ApiType,
                [who: AccountId32, username: Bytes],
                { who: AccountId32; username: Bytes }
            >;
            /**
             * A name was cleared, and the given balance returned.
             **/
            IdentityCleared: AugmentedEvent<
                ApiType,
                [who: AccountId32, deposit: u128],
                { who: AccountId32; deposit: u128 }
            >;
            /**
             * A name was removed and the given balance slashed.
             **/
            IdentityKilled: AugmentedEvent<
                ApiType,
                [who: AccountId32, deposit: u128],
                { who: AccountId32; deposit: u128 }
            >;
            /**
             * A name was set or reset (which will remove all judgements).
             **/
            IdentitySet: AugmentedEvent<ApiType, [who: AccountId32], { who: AccountId32 }>;
            /**
             * A judgement was given by a registrar.
             **/
            JudgementGiven: AugmentedEvent<
                ApiType,
                [target: AccountId32, registrarIndex: u32],
                { target: AccountId32; registrarIndex: u32 }
            >;
            /**
             * A judgement was asked from a registrar.
             **/
            JudgementRequested: AugmentedEvent<
                ApiType,
                [who: AccountId32, registrarIndex: u32],
                { who: AccountId32; registrarIndex: u32 }
            >;
            /**
             * A judgement request was retracted.
             **/
            JudgementUnrequested: AugmentedEvent<
                ApiType,
                [who: AccountId32, registrarIndex: u32],
                { who: AccountId32; registrarIndex: u32 }
            >;
            /**
             * A queued username passed its expiration without being claimed and was removed.
             **/
            PreapprovalExpired: AugmentedEvent<ApiType, [whose: AccountId32], { whose: AccountId32 }>;
            /**
             * A username was set as a primary and can be looked up from `who`.
             **/
            PrimaryUsernameSet: AugmentedEvent<
                ApiType,
                [who: AccountId32, username: Bytes],
                { who: AccountId32; username: Bytes }
            >;
            /**
             * A registrar was added.
             **/
            RegistrarAdded: AugmentedEvent<ApiType, [registrarIndex: u32], { registrarIndex: u32 }>;
            /**
             * An account's sub-identities were set (in bulk).
             **/
            SubIdentitiesSet: AugmentedEvent<
                ApiType,
                [main: AccountId32, numberOfSubs: u32, newDeposit: u128],
                { main: AccountId32; numberOfSubs: u32; newDeposit: u128 }
            >;
            /**
             * A sub-identity was added to an identity and the deposit paid.
             **/
            SubIdentityAdded: AugmentedEvent<
                ApiType,
                [sub: AccountId32, main: AccountId32, deposit: u128],
                { sub: AccountId32; main: AccountId32; deposit: u128 }
            >;
            /**
             * A sub-identity was removed from an identity and the deposit freed.
             **/
            SubIdentityRemoved: AugmentedEvent<
                ApiType,
                [sub: AccountId32, main: AccountId32, deposit: u128],
                { sub: AccountId32; main: AccountId32; deposit: u128 }
            >;
            /**
             * A given sub-account's associated name was changed by its super-identity.
             **/
            SubIdentityRenamed: AugmentedEvent<
                ApiType,
                [sub: AccountId32, main: AccountId32],
                { sub: AccountId32; main: AccountId32 }
            >;
            /**
             * A sub-identity was cleared, and the given deposit repatriated from the
             * main identity account to the sub-identity account.
             **/
            SubIdentityRevoked: AugmentedEvent<
                ApiType,
                [sub: AccountId32, main: AccountId32, deposit: u128],
                { sub: AccountId32; main: AccountId32; deposit: u128 }
            >;
            /**
             * A username has been killed.
             **/
            UsernameKilled: AugmentedEvent<ApiType, [username: Bytes], { username: Bytes }>;
            /**
             * A username was queued, but `who` must accept it prior to `expiration`.
             **/
            UsernameQueued: AugmentedEvent<
                ApiType,
                [who: AccountId32, username: Bytes, expiration: u32],
                { who: AccountId32; username: Bytes; expiration: u32 }
            >;
            /**
             * A username has been removed.
             **/
            UsernameRemoved: AugmentedEvent<ApiType, [username: Bytes], { username: Bytes }>;
            /**
             * A username was set for `who`.
             **/
            UsernameSet: AugmentedEvent<
                ApiType,
                [who: AccountId32, username: Bytes],
                { who: AccountId32; username: Bytes }
            >;
            /**
             * A username has been unbound.
             **/
            UsernameUnbound: AugmentedEvent<ApiType, [username: Bytes], { username: Bytes }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        inactivityTracking: {
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        inflationRewards: {
            /**
             * Rewarding container author
             **/
            RewardedContainer: AugmentedEvent<
                ApiType,
                [accountId: AccountId32, paraId: u32, balance: u128],
                { accountId: AccountId32; paraId: u32; balance: u128 }
            >;
            /**
             * Rewarding orchestrator author
             **/
            RewardedOrchestrator: AugmentedEvent<
                ApiType,
                [accountId: AccountId32, balance: u128],
                { accountId: AccountId32; balance: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        invulnerables: {
            /**
             * A new Invulnerable was added.
             **/
            InvulnerableAdded: AugmentedEvent<ApiType, [accountId: AccountId32], { accountId: AccountId32 }>;
            /**
             * An Invulnerable was removed.
             **/
            InvulnerableRemoved: AugmentedEvent<ApiType, [accountId: AccountId32], { accountId: AccountId32 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        maintenanceMode: {
            /**
             * The chain was put into Maintenance Mode
             **/
            EnteredMaintenanceMode: AugmentedEvent<ApiType, []>;
            /**
             * The call to resume on_idle XCM execution failed with inner error
             **/
            FailedToResumeIdleXcmExecution: AugmentedEvent<
                ApiType,
                [error: SpRuntimeDispatchError],
                { error: SpRuntimeDispatchError }
            >;
            /**
             * The call to suspend on_idle XCM execution failed with inner error
             **/
            FailedToSuspendIdleXcmExecution: AugmentedEvent<
                ApiType,
                [error: SpRuntimeDispatchError],
                { error: SpRuntimeDispatchError }
            >;
            /**
             * The chain returned to its normal operating state
             **/
            NormalOperationResumed: AugmentedEvent<ApiType, []>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        messageQueue: {
            /**
             * Message placed in overweight queue.
             **/
            OverweightEnqueued: AugmentedEvent<
                ApiType,
                [id: U8aFixed, origin: CumulusPrimitivesCoreAggregateMessageOrigin, pageIndex: u32, messageIndex: u32],
                { id: U8aFixed; origin: CumulusPrimitivesCoreAggregateMessageOrigin; pageIndex: u32; messageIndex: u32 }
            >;
            /**
             * This page was reaped.
             **/
            PageReaped: AugmentedEvent<
                ApiType,
                [origin: CumulusPrimitivesCoreAggregateMessageOrigin, index: u32],
                { origin: CumulusPrimitivesCoreAggregateMessageOrigin; index: u32 }
            >;
            /**
             * Message is processed.
             **/
            Processed: AugmentedEvent<
                ApiType,
                [
                    id: H256,
                    origin: CumulusPrimitivesCoreAggregateMessageOrigin,
                    weightUsed: SpWeightsWeightV2Weight,
                    success: bool,
                ],
                {
                    id: H256;
                    origin: CumulusPrimitivesCoreAggregateMessageOrigin;
                    weightUsed: SpWeightsWeightV2Weight;
                    success: bool;
                }
            >;
            /**
             * Message discarded due to an error in the `MessageProcessor` (usually a format error).
             **/
            ProcessingFailed: AugmentedEvent<
                ApiType,
                [
                    id: H256,
                    origin: CumulusPrimitivesCoreAggregateMessageOrigin,
                    error: FrameSupportMessagesProcessMessageError,
                ],
                {
                    id: H256;
                    origin: CumulusPrimitivesCoreAggregateMessageOrigin;
                    error: FrameSupportMessagesProcessMessageError;
                }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        migrations: {
            /**
             * XCM execution resume failed with inner error
             **/
            FailedToResumeIdleXcmExecution: AugmentedEvent<
                ApiType,
                [error: SpRuntimeDispatchError],
                { error: SpRuntimeDispatchError }
            >;
            /**
             * XCM execution suspension failed with inner error
             **/
            FailedToSuspendIdleXcmExecution: AugmentedEvent<
                ApiType,
                [error: SpRuntimeDispatchError],
                { error: SpRuntimeDispatchError }
            >;
            /**
             * Migration completed
             **/
            MigrationCompleted: AugmentedEvent<
                ApiType,
                [migrationName: Bytes, consumedWeight: SpWeightsWeightV2Weight],
                { migrationName: Bytes; consumedWeight: SpWeightsWeightV2Weight }
            >;
            /**
             * Migration started
             **/
            MigrationStarted: AugmentedEvent<ApiType, [migrationName: Bytes], { migrationName: Bytes }>;
            /**
             * Runtime upgrade completed
             **/
            RuntimeUpgradeCompleted: AugmentedEvent<
                ApiType,
                [weight: SpWeightsWeightV2Weight],
                { weight: SpWeightsWeightV2Weight }
            >;
            /**
             * Runtime upgrade started
             **/
            RuntimeUpgradeStarted: AugmentedEvent<ApiType, []>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        multiBlockMigrations: {
            /**
             * The set of historical migrations has been cleared.
             **/
            HistoricCleared: AugmentedEvent<ApiType, [nextCursor: Option<Bytes>], { nextCursor: Option<Bytes> }>;
            /**
             * A migration progressed.
             **/
            MigrationAdvanced: AugmentedEvent<ApiType, [index: u32, took: u32], { index: u32; took: u32 }>;
            /**
             * A Migration completed.
             **/
            MigrationCompleted: AugmentedEvent<ApiType, [index: u32, took: u32], { index: u32; took: u32 }>;
            /**
             * A Migration failed.
             *
             * This implies that the whole upgrade failed and governance intervention is required.
             **/
            MigrationFailed: AugmentedEvent<ApiType, [index: u32, took: u32], { index: u32; took: u32 }>;
            /**
             * A migration was skipped since it was already executed in the past.
             **/
            MigrationSkipped: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /**
             * The current runtime upgrade completed.
             *
             * This implies that all of its migrations completed successfully as well.
             **/
            UpgradeCompleted: AugmentedEvent<ApiType, []>;
            /**
             * Runtime upgrade failed.
             *
             * This is very bad and will require governance intervention.
             **/
            UpgradeFailed: AugmentedEvent<ApiType, []>;
            /**
             * A Runtime upgrade started.
             *
             * Its end is indicated by `UpgradeCompleted` or `UpgradeFailed`.
             **/
            UpgradeStarted: AugmentedEvent<ApiType, [migrations: u32], { migrations: u32 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        multisig: {
            /**
             * A multisig operation has been approved by someone.
             **/
            MultisigApproval: AugmentedEvent<
                ApiType,
                [approving: AccountId32, timepoint: PalletMultisigTimepoint, multisig: AccountId32, callHash: U8aFixed],
                {
                    approving: AccountId32;
                    timepoint: PalletMultisigTimepoint;
                    multisig: AccountId32;
                    callHash: U8aFixed;
                }
            >;
            /**
             * A multisig operation has been cancelled.
             **/
            MultisigCancelled: AugmentedEvent<
                ApiType,
                [
                    cancelling: AccountId32,
                    timepoint: PalletMultisigTimepoint,
                    multisig: AccountId32,
                    callHash: U8aFixed,
                ],
                {
                    cancelling: AccountId32;
                    timepoint: PalletMultisigTimepoint;
                    multisig: AccountId32;
                    callHash: U8aFixed;
                }
            >;
            /**
             * A multisig operation has been executed.
             **/
            MultisigExecuted: AugmentedEvent<
                ApiType,
                [
                    approving: AccountId32,
                    timepoint: PalletMultisigTimepoint,
                    multisig: AccountId32,
                    callHash: U8aFixed,
                    result: Result<Null, SpRuntimeDispatchError>,
                ],
                {
                    approving: AccountId32;
                    timepoint: PalletMultisigTimepoint;
                    multisig: AccountId32;
                    callHash: U8aFixed;
                    result: Result<Null, SpRuntimeDispatchError>;
                }
            >;
            /**
             * A new multisig operation has begun.
             **/
            NewMultisig: AugmentedEvent<
                ApiType,
                [approving: AccountId32, multisig: AccountId32, callHash: U8aFixed],
                { approving: AccountId32; multisig: AccountId32; callHash: U8aFixed }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        parachainSystem: {
            /**
             * Downward messages were processed using the given weight.
             **/
            DownwardMessagesProcessed: AugmentedEvent<
                ApiType,
                [weightUsed: SpWeightsWeightV2Weight, dmqHead: H256],
                { weightUsed: SpWeightsWeightV2Weight; dmqHead: H256 }
            >;
            /**
             * Some downward messages have been received and will be processed.
             **/
            DownwardMessagesReceived: AugmentedEvent<ApiType, [count: u32], { count: u32 }>;
            /**
             * An upward message was sent to the relay chain.
             **/
            UpwardMessageSent: AugmentedEvent<
                ApiType,
                [messageHash: Option<U8aFixed>],
                { messageHash: Option<U8aFixed> }
            >;
            /**
             * The validation function was applied as of the contained relay chain block number.
             **/
            ValidationFunctionApplied: AugmentedEvent<ApiType, [relayChainBlockNum: u32], { relayChainBlockNum: u32 }>;
            /**
             * The relay-chain aborted the upgrade process.
             **/
            ValidationFunctionDiscarded: AugmentedEvent<ApiType, []>;
            /**
             * The validation function has been scheduled to apply.
             **/
            ValidationFunctionStored: AugmentedEvent<ApiType, []>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        polkadotXcm: {
            /**
             * Some assets have been claimed from an asset trap
             **/
            AssetsClaimed: AugmentedEvent<
                ApiType,
                [hash_: H256, origin: StagingXcmV5Location, assets: XcmVersionedAssets],
                { hash_: H256; origin: StagingXcmV5Location; assets: XcmVersionedAssets }
            >;
            /**
             * Some assets have been placed in an asset trap.
             **/
            AssetsTrapped: AugmentedEvent<
                ApiType,
                [hash_: H256, origin: StagingXcmV5Location, assets: XcmVersionedAssets],
                { hash_: H256; origin: StagingXcmV5Location; assets: XcmVersionedAssets }
            >;
            /**
             * Execution of an XCM message was attempted.
             **/
            Attempted: AugmentedEvent<
                ApiType,
                [outcome: StagingXcmV5TraitsOutcome],
                { outcome: StagingXcmV5TraitsOutcome }
            >;
            /**
             * Fees were paid from a location for an operation (often for using `SendXcm`).
             **/
            FeesPaid: AugmentedEvent<
                ApiType,
                [paying: StagingXcmV5Location, fees: StagingXcmV5AssetAssets],
                { paying: StagingXcmV5Location; fees: StagingXcmV5AssetAssets }
            >;
            /**
             * Expected query response has been received but the querier location of the response does
             * not match the expected. The query remains registered for a later, valid, response to
             * be received and acted upon.
             **/
            InvalidQuerier: AugmentedEvent<
                ApiType,
                [
                    origin: StagingXcmV5Location,
                    queryId: u64,
                    expectedQuerier: StagingXcmV5Location,
                    maybeActualQuerier: Option<StagingXcmV5Location>,
                ],
                {
                    origin: StagingXcmV5Location;
                    queryId: u64;
                    expectedQuerier: StagingXcmV5Location;
                    maybeActualQuerier: Option<StagingXcmV5Location>;
                }
            >;
            /**
             * Expected query response has been received but the expected querier location placed in
             * storage by this runtime previously cannot be decoded. The query remains registered.
             *
             * This is unexpected (since a location placed in storage in a previously executing
             * runtime should be readable prior to query timeout) and dangerous since the possibly
             * valid response will be dropped. Manual governance intervention is probably going to be
             * needed.
             **/
            InvalidQuerierVersion: AugmentedEvent<
                ApiType,
                [origin: StagingXcmV5Location, queryId: u64],
                { origin: StagingXcmV5Location; queryId: u64 }
            >;
            /**
             * Expected query response has been received but the origin location of the response does
             * not match that expected. The query remains registered for a later, valid, response to
             * be received and acted upon.
             **/
            InvalidResponder: AugmentedEvent<
                ApiType,
                [origin: StagingXcmV5Location, queryId: u64, expectedLocation: Option<StagingXcmV5Location>],
                { origin: StagingXcmV5Location; queryId: u64; expectedLocation: Option<StagingXcmV5Location> }
            >;
            /**
             * Expected query response has been received but the expected origin location placed in
             * storage by this runtime previously cannot be decoded. The query remains registered.
             *
             * This is unexpected (since a location placed in storage in a previously executing
             * runtime should be readable prior to query timeout) and dangerous since the possibly
             * valid response will be dropped. Manual governance intervention is probably going to be
             * needed.
             **/
            InvalidResponderVersion: AugmentedEvent<
                ApiType,
                [origin: StagingXcmV5Location, queryId: u64],
                { origin: StagingXcmV5Location; queryId: u64 }
            >;
            /**
             * Query response has been received and query is removed. The registered notification has
             * been dispatched and executed successfully.
             **/
            Notified: AugmentedEvent<
                ApiType,
                [queryId: u64, palletIndex: u8, callIndex: u8],
                { queryId: u64; palletIndex: u8; callIndex: u8 }
            >;
            /**
             * Query response has been received and query is removed. The dispatch was unable to be
             * decoded into a `Call`; this might be due to dispatch function having a signature which
             * is not `(origin, QueryId, Response)`.
             **/
            NotifyDecodeFailed: AugmentedEvent<
                ApiType,
                [queryId: u64, palletIndex: u8, callIndex: u8],
                { queryId: u64; palletIndex: u8; callIndex: u8 }
            >;
            /**
             * Query response has been received and query is removed. There was a general error with
             * dispatching the notification call.
             **/
            NotifyDispatchError: AugmentedEvent<
                ApiType,
                [queryId: u64, palletIndex: u8, callIndex: u8],
                { queryId: u64; palletIndex: u8; callIndex: u8 }
            >;
            /**
             * Query response has been received and query is removed. The registered notification
             * could not be dispatched because the dispatch weight is greater than the maximum weight
             * originally budgeted by this runtime for the query result.
             **/
            NotifyOverweight: AugmentedEvent<
                ApiType,
                [
                    queryId: u64,
                    palletIndex: u8,
                    callIndex: u8,
                    actualWeight: SpWeightsWeightV2Weight,
                    maxBudgetedWeight: SpWeightsWeightV2Weight,
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
             * A given location which had a version change subscription was dropped owing to an error
             * migrating the location to our new XCM format.
             **/
            NotifyTargetMigrationFail: AugmentedEvent<
                ApiType,
                [location: XcmVersionedLocation, queryId: u64],
                { location: XcmVersionedLocation; queryId: u64 }
            >;
            /**
             * A given location which had a version change subscription was dropped owing to an error
             * sending the notification to it.
             **/
            NotifyTargetSendFail: AugmentedEvent<
                ApiType,
                [location: StagingXcmV5Location, queryId: u64, error: XcmV5TraitsError],
                { location: StagingXcmV5Location; queryId: u64; error: XcmV5TraitsError }
            >;
            /**
             * Query response has been received and is ready for taking with `take_response`. There is
             * no registered notification call.
             **/
            ResponseReady: AugmentedEvent<
                ApiType,
                [queryId: u64, response: StagingXcmV5Response],
                { queryId: u64; response: StagingXcmV5Response }
            >;
            /**
             * Received query response has been read and removed.
             **/
            ResponseTaken: AugmentedEvent<ApiType, [queryId: u64], { queryId: u64 }>;
            /**
             * A XCM message was sent.
             **/
            Sent: AugmentedEvent<
                ApiType,
                [
                    origin: StagingXcmV5Location,
                    destination: StagingXcmV5Location,
                    message: StagingXcmV5Xcm,
                    messageId: U8aFixed,
                ],
                {
                    origin: StagingXcmV5Location;
                    destination: StagingXcmV5Location;
                    message: StagingXcmV5Xcm;
                    messageId: U8aFixed;
                }
            >;
            /**
             * The supported version of a location has been changed. This might be through an
             * automatic notification or a manual intervention.
             **/
            SupportedVersionChanged: AugmentedEvent<
                ApiType,
                [location: StagingXcmV5Location, version: u32],
                { location: StagingXcmV5Location; version: u32 }
            >;
            /**
             * Query response received which does not match a registered query. This may be because a
             * matching query was never registered, it may be because it is a duplicate response, or
             * because the query timed out.
             **/
            UnexpectedResponse: AugmentedEvent<
                ApiType,
                [origin: StagingXcmV5Location, queryId: u64],
                { origin: StagingXcmV5Location; queryId: u64 }
            >;
            /**
             * An XCM version change notification message has been attempted to be sent.
             *
             * The cost of sending it (borne by the chain) is included.
             **/
            VersionChangeNotified: AugmentedEvent<
                ApiType,
                [destination: StagingXcmV5Location, result: u32, cost: StagingXcmV5AssetAssets, messageId: U8aFixed],
                { destination: StagingXcmV5Location; result: u32; cost: StagingXcmV5AssetAssets; messageId: U8aFixed }
            >;
            /**
             * A XCM version migration finished.
             **/
            VersionMigrationFinished: AugmentedEvent<ApiType, [version: u32], { version: u32 }>;
            /**
             * We have requested that a remote chain send us XCM version change notifications.
             **/
            VersionNotifyRequested: AugmentedEvent<
                ApiType,
                [destination: StagingXcmV5Location, cost: StagingXcmV5AssetAssets, messageId: U8aFixed],
                { destination: StagingXcmV5Location; cost: StagingXcmV5AssetAssets; messageId: U8aFixed }
            >;
            /**
             * A remote has requested XCM version change notification from us and we have honored it.
             * A version information message is sent to them and its cost is included.
             **/
            VersionNotifyStarted: AugmentedEvent<
                ApiType,
                [destination: StagingXcmV5Location, cost: StagingXcmV5AssetAssets, messageId: U8aFixed],
                { destination: StagingXcmV5Location; cost: StagingXcmV5AssetAssets; messageId: U8aFixed }
            >;
            /**
             * We have requested that a remote chain stops sending us XCM version change
             * notifications.
             **/
            VersionNotifyUnrequested: AugmentedEvent<
                ApiType,
                [destination: StagingXcmV5Location, cost: StagingXcmV5AssetAssets, messageId: U8aFixed],
                { destination: StagingXcmV5Location; cost: StagingXcmV5AssetAssets; messageId: U8aFixed }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        pooledStaking: {
            /**
             * Rewards manually claimed.
             **/
            ClaimedManualRewards: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, rewards: u128],
                { candidate: AccountId32; delegator: AccountId32; rewards: u128 }
            >;
            /**
             * Stake of that Candidate decreased.
             **/
            DecreasedStake: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, stakeDiff: u128],
                { candidate: AccountId32; stakeDiff: u128 }
            >;
            /**
             * Delegation request was executed. `staked` has been properly staked
             * in `pool`, while the rounding when converting to shares has been
             * `released`.
             **/
            ExecutedDelegate: AugmentedEvent<
                ApiType,
                [
                    candidate: AccountId32,
                    delegator: AccountId32,
                    pool: PalletPooledStakingTargetPool,
                    staked: u128,
                    released: u128,
                ],
                {
                    candidate: AccountId32;
                    delegator: AccountId32;
                    pool: PalletPooledStakingTargetPool;
                    staked: u128;
                    released: u128;
                }
            >;
            /**
             * Undelegation request was executed.
             **/
            ExecutedUndelegate: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, released: u128],
                { candidate: AccountId32; delegator: AccountId32; released: u128 }
            >;
            /**
             * Stake of that Candidate increased.
             **/
            IncreasedStake: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, stakeDiff: u128],
                { candidate: AccountId32; stakeDiff: u128 }
            >;
            /**
             * User requested to delegate towards a candidate.
             **/
            RequestedDelegate: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, pool: PalletPooledStakingTargetPool, pending: u128],
                { candidate: AccountId32; delegator: AccountId32; pool: PalletPooledStakingTargetPool; pending: u128 }
            >;
            /**
             * User requested to undelegate from a candidate.
             * Stake was removed from a `pool` and is `pending` for the request
             * to be executed. The rounding when converting to leaving shares has
             * been `released` immediately.
             **/
            RequestedUndelegate: AugmentedEvent<
                ApiType,
                [
                    candidate: AccountId32,
                    delegator: AccountId32,
                    from: PalletPooledStakingTargetPool,
                    pending: u128,
                    released: u128,
                ],
                {
                    candidate: AccountId32;
                    delegator: AccountId32;
                    from: PalletPooledStakingTargetPool;
                    pending: u128;
                    released: u128;
                }
            >;
            /**
             * Collator has been rewarded.
             **/
            RewardedCollator: AugmentedEvent<
                ApiType,
                [collator: AccountId32, autoCompoundingRewards: u128, manualClaimRewards: u128],
                { collator: AccountId32; autoCompoundingRewards: u128; manualClaimRewards: u128 }
            >;
            /**
             * Delegators have been rewarded.
             **/
            RewardedDelegators: AugmentedEvent<
                ApiType,
                [collator: AccountId32, autoCompoundingRewards: u128, manualClaimRewards: u128],
                { collator: AccountId32; autoCompoundingRewards: u128; manualClaimRewards: u128 }
            >;
            /**
             * Delegator staked towards a Candidate for AutoCompounding Shares.
             **/
            StakedAutoCompounding: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, shares: u128, stake: u128],
                { candidate: AccountId32; delegator: AccountId32; shares: u128; stake: u128 }
            >;
            /**
             * Delegator staked towards a candidate for ManualRewards Shares.
             **/
            StakedManualRewards: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, shares: u128, stake: u128],
                { candidate: AccountId32; delegator: AccountId32; shares: u128; stake: u128 }
            >;
            /**
             * Swapped between AutoCompounding and ManualReward shares
             **/
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
                    released: u128,
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
            /**
             * Delegator unstaked towards a candidate with AutoCompounding Shares.
             **/
            UnstakedAutoCompounding: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, shares: u128, stake: u128],
                { candidate: AccountId32; delegator: AccountId32; shares: u128; stake: u128 }
            >;
            /**
             * Delegator unstaked towards a candidate with ManualRewards Shares.
             **/
            UnstakedManualRewards: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, delegator: AccountId32, shares: u128, stake: u128],
                { candidate: AccountId32; delegator: AccountId32; shares: u128; stake: u128 }
            >;
            /**
             * Stake of the candidate has changed, which may have modified its
             * position in the eligible candidates list.
             **/
            UpdatedCandidatePosition: AugmentedEvent<
                ApiType,
                [candidate: AccountId32, stake: u128, selfDelegation: u128, before: Option<u32>, after: Option<u32>],
                { candidate: AccountId32; stake: u128; selfDelegation: u128; before: Option<u32>; after: Option<u32> }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        proxy: {
            /**
             * An announcement was placed to make a call in the future.
             **/
            Announced: AugmentedEvent<
                ApiType,
                [real: AccountId32, proxy: AccountId32, callHash: H256],
                { real: AccountId32; proxy: AccountId32; callHash: H256 }
            >;
            /**
             * A proxy was added.
             **/
            ProxyAdded: AugmentedEvent<
                ApiType,
                [delegator: AccountId32, delegatee: AccountId32, proxyType: DanceboxRuntimeProxyType, delay: u32],
                { delegator: AccountId32; delegatee: AccountId32; proxyType: DanceboxRuntimeProxyType; delay: u32 }
            >;
            /**
             * A proxy was executed correctly, with the given.
             **/
            ProxyExecuted: AugmentedEvent<
                ApiType,
                [result: Result<Null, SpRuntimeDispatchError>],
                { result: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * A proxy was removed.
             **/
            ProxyRemoved: AugmentedEvent<
                ApiType,
                [delegator: AccountId32, delegatee: AccountId32, proxyType: DanceboxRuntimeProxyType, delay: u32],
                { delegator: AccountId32; delegatee: AccountId32; proxyType: DanceboxRuntimeProxyType; delay: u32 }
            >;
            /**
             * A pure account has been created by new proxy with given
             * disambiguation index and proxy type.
             **/
            PureCreated: AugmentedEvent<
                ApiType,
                [pure: AccountId32, who: AccountId32, proxyType: DanceboxRuntimeProxyType, disambiguationIndex: u16],
                { pure: AccountId32; who: AccountId32; proxyType: DanceboxRuntimeProxyType; disambiguationIndex: u16 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        registrar: {
            /**
             * A para id has been deregistered. [para_id]
             **/
            ParaIdDeregistered: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /**
             * A para id has been paused from collating.
             **/
            ParaIdPaused: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /**
             * A new para id has been registered. [para_id]
             **/
            ParaIdRegistered: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /**
             * A para id has been unpaused.
             **/
            ParaIdUnpaused: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /**
             * A new para id is now valid for collating. [para_id]
             **/
            ParaIdValidForCollating: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /**
             * Para manager has changed
             **/
            ParaManagerChanged: AugmentedEvent<
                ApiType,
                [paraId: u32, managerAddress: AccountId32],
                { paraId: u32; managerAddress: AccountId32 }
            >;
            /**
             * Parathread params changed
             **/
            ParathreadParamsChanged: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        rootTesting: {
            /**
             * Event dispatched when the trigger_defensive extrinsic is called.
             **/
            DefensiveTestCall: AugmentedEvent<ApiType, []>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        servicesPayment: {
            BlockProductionCreditBurned: AugmentedEvent<
                ApiType,
                [paraId: u32, creditsRemaining: u32],
                { paraId: u32; creditsRemaining: u32 }
            >;
            BlockProductionCreditsSet: AugmentedEvent<
                ApiType,
                [paraId: u32, credits: u32],
                { paraId: u32; credits: u32 }
            >;
            CollatorAssignmentCreditBurned: AugmentedEvent<
                ApiType,
                [paraId: u32, creditsRemaining: u32],
                { paraId: u32; creditsRemaining: u32 }
            >;
            CollatorAssignmentCreditsSet: AugmentedEvent<
                ApiType,
                [paraId: u32, credits: u32],
                { paraId: u32; credits: u32 }
            >;
            CollatorAssignmentTipCollected: AugmentedEvent<
                ApiType,
                [paraId: u32, payer: AccountId32, tip: u128],
                { paraId: u32; payer: AccountId32; tip: u128 }
            >;
            CreditsPurchased: AugmentedEvent<
                ApiType,
                [paraId: u32, payer: AccountId32, credit: u128],
                { paraId: u32; payer: AccountId32; credit: u128 }
            >;
            MaxCorePriceUpdated: AugmentedEvent<
                ApiType,
                [paraId: u32, maxCorePrice: Option<u128>],
                { paraId: u32; maxCorePrice: Option<u128> }
            >;
            RefundAddressUpdated: AugmentedEvent<
                ApiType,
                [paraId: u32, refundAddress: Option<AccountId32>],
                { paraId: u32; refundAddress: Option<AccountId32> }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        session: {
            /**
             * New session has happened. Note that the argument is the session index, not the
             * block number as the type might suggest.
             **/
            NewSession: AugmentedEvent<ApiType, [sessionIndex: u32], { sessionIndex: u32 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        streamPayment: {
            StreamClosed: AugmentedEvent<ApiType, [streamId: u64, refunded: u128], { streamId: u64; refunded: u128 }>;
            StreamConfigChanged: AugmentedEvent<
                ApiType,
                [
                    streamId: u64,
                    oldConfig: PalletStreamPaymentStreamConfig,
                    newConfig: PalletStreamPaymentStreamConfig,
                    depositChange: Option<PalletStreamPaymentDepositChange>,
                ],
                {
                    streamId: u64;
                    oldConfig: PalletStreamPaymentStreamConfig;
                    newConfig: PalletStreamPaymentStreamConfig;
                    depositChange: Option<PalletStreamPaymentDepositChange>;
                }
            >;
            StreamConfigChangeRequested: AugmentedEvent<
                ApiType,
                [
                    streamId: u64,
                    requestNonce: u32,
                    requester: PalletStreamPaymentParty,
                    oldConfig: PalletStreamPaymentStreamConfig,
                    newConfig: PalletStreamPaymentStreamConfig,
                ],
                {
                    streamId: u64;
                    requestNonce: u32;
                    requester: PalletStreamPaymentParty;
                    oldConfig: PalletStreamPaymentStreamConfig;
                    newConfig: PalletStreamPaymentStreamConfig;
                }
            >;
            StreamOpened: AugmentedEvent<ApiType, [streamId: u64], { streamId: u64 }>;
            StreamPayment: AugmentedEvent<
                ApiType,
                [streamId: u64, source: AccountId32, target: AccountId32, amount: u128, stalled: bool],
                { streamId: u64; source: AccountId32; target: AccountId32; amount: u128; stalled: bool }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        sudo: {
            /**
             * The sudo key has been updated.
             **/
            KeyChanged: AugmentedEvent<
                ApiType,
                [old: Option<AccountId32>, new_: AccountId32],
                { old: Option<AccountId32>; new_: AccountId32 }
            >;
            /**
             * The key was permanently removed.
             **/
            KeyRemoved: AugmentedEvent<ApiType, []>;
            /**
             * A sudo call just took place.
             **/
            Sudid: AugmentedEvent<
                ApiType,
                [sudoResult: Result<Null, SpRuntimeDispatchError>],
                { sudoResult: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * A [sudo_as](Pallet::sudo_as) call just took place.
             **/
            SudoAsDone: AugmentedEvent<
                ApiType,
                [sudoResult: Result<Null, SpRuntimeDispatchError>],
                { sudoResult: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        system: {
            /**
             * `:code` was updated.
             **/
            CodeUpdated: AugmentedEvent<ApiType, []>;
            /**
             * An extrinsic failed.
             **/
            ExtrinsicFailed: AugmentedEvent<
                ApiType,
                [dispatchError: SpRuntimeDispatchError, dispatchInfo: FrameSystemDispatchEventInfo],
                { dispatchError: SpRuntimeDispatchError; dispatchInfo: FrameSystemDispatchEventInfo }
            >;
            /**
             * An extrinsic completed successfully.
             **/
            ExtrinsicSuccess: AugmentedEvent<
                ApiType,
                [dispatchInfo: FrameSystemDispatchEventInfo],
                { dispatchInfo: FrameSystemDispatchEventInfo }
            >;
            /**
             * An account was reaped.
             **/
            KilledAccount: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            /**
             * A new account was created.
             **/
            NewAccount: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            /**
             * On on-chain remark happened.
             **/
            Remarked: AugmentedEvent<ApiType, [sender: AccountId32, hash_: H256], { sender: AccountId32; hash_: H256 }>;
            /**
             * An upgrade was authorized.
             **/
            UpgradeAuthorized: AugmentedEvent<
                ApiType,
                [codeHash: H256, checkVersion: bool],
                { codeHash: H256; checkVersion: bool }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        transactionPayment: {
            /**
             * A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,
             * has been paid by `who`.
             **/
            TransactionFeePaid: AugmentedEvent<
                ApiType,
                [who: AccountId32, actualFee: u128, tip: u128],
                { who: AccountId32; actualFee: u128; tip: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        treasury: {
            /**
             * A new asset spend proposal has been approved.
             **/
            AssetSpendApproved: AugmentedEvent<
                ApiType,
                [index: u32, assetKind: Null, amount: u128, beneficiary: AccountId32, validFrom: u32, expireAt: u32],
                { index: u32; assetKind: Null; amount: u128; beneficiary: AccountId32; validFrom: u32; expireAt: u32 }
            >;
            /**
             * An approved spend was voided.
             **/
            AssetSpendVoided: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /**
             * Some funds have been allocated.
             **/
            Awarded: AugmentedEvent<
                ApiType,
                [proposalIndex: u32, award: u128, account: AccountId32],
                { proposalIndex: u32; award: u128; account: AccountId32 }
            >;
            /**
             * Some of our funds have been burnt.
             **/
            Burnt: AugmentedEvent<ApiType, [burntFunds: u128], { burntFunds: u128 }>;
            /**
             * Some funds have been deposited.
             **/
            Deposit: AugmentedEvent<ApiType, [value: u128], { value: u128 }>;
            /**
             * A payment happened.
             **/
            Paid: AugmentedEvent<ApiType, [index: u32, paymentId: Null], { index: u32; paymentId: Null }>;
            /**
             * A payment failed and can be retried.
             **/
            PaymentFailed: AugmentedEvent<ApiType, [index: u32, paymentId: Null], { index: u32; paymentId: Null }>;
            /**
             * Spending has finished; this is the amount that rolls over until next spend.
             **/
            Rollover: AugmentedEvent<ApiType, [rolloverBalance: u128], { rolloverBalance: u128 }>;
            /**
             * A new spend proposal has been approved.
             **/
            SpendApproved: AugmentedEvent<
                ApiType,
                [proposalIndex: u32, amount: u128, beneficiary: AccountId32],
                { proposalIndex: u32; amount: u128; beneficiary: AccountId32 }
            >;
            /**
             * We have ended a spend period and will now allocate funds.
             **/
            Spending: AugmentedEvent<ApiType, [budgetRemaining: u128], { budgetRemaining: u128 }>;
            /**
             * A spend was processed and removed from the storage. It might have been successfully
             * paid or it may have expired.
             **/
            SpendProcessed: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /**
             * The inactive funds of the pallet have been updated.
             **/
            UpdatedInactive: AugmentedEvent<
                ApiType,
                [reactivated: u128, deactivated: u128],
                { reactivated: u128; deactivated: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        txPause: {
            /**
             * This pallet, or a specific call is now paused.
             **/
            CallPaused: AugmentedEvent<
                ApiType,
                [fullName: ITuple<[Bytes, Bytes]>],
                { fullName: ITuple<[Bytes, Bytes]> }
            >;
            /**
             * This pallet, or a specific call is now unpaused.
             **/
            CallUnpaused: AugmentedEvent<
                ApiType,
                [fullName: ITuple<[Bytes, Bytes]>],
                { fullName: ITuple<[Bytes, Bytes]> }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        utility: {
            /**
             * Batch of dispatches completed fully with no error.
             **/
            BatchCompleted: AugmentedEvent<ApiType, []>;
            /**
             * Batch of dispatches completed but has errors.
             **/
            BatchCompletedWithErrors: AugmentedEvent<ApiType, []>;
            /**
             * Batch of dispatches did not complete fully. Index of first failing dispatch given, as
             * well as the error.
             **/
            BatchInterrupted: AugmentedEvent<
                ApiType,
                [index: u32, error: SpRuntimeDispatchError],
                { index: u32; error: SpRuntimeDispatchError }
            >;
            /**
             * A call was dispatched.
             **/
            DispatchedAs: AugmentedEvent<
                ApiType,
                [result: Result<Null, SpRuntimeDispatchError>],
                { result: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * A single item within a Batch of dispatches has completed with no error.
             **/
            ItemCompleted: AugmentedEvent<ApiType, []>;
            /**
             * A single item within a Batch of dispatches has completed with error.
             **/
            ItemFailed: AugmentedEvent<ApiType, [error: SpRuntimeDispatchError], { error: SpRuntimeDispatchError }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        xcmCoreBuyer: {
            /**
             * An XCM message to buy a core for this parathread has been sent to the relay chain.
             **/
            BuyCoreXcmSent: AugmentedEvent<
                ApiType,
                [paraId: u32, transactionStatusQueryId: u64],
                { paraId: u32; transactionStatusQueryId: u64 }
            >;
            /**
             * We cleaned up expired in flight orders entries.
             **/
            CleanedUpExpiredInFlightOrderEntries: AugmentedEvent<ApiType, [paraIds: Vec<u32>], { paraIds: Vec<u32> }>;
            /**
             * We cleaned up expired pending blocks entries.
             **/
            CleanedUpExpiredPendingBlocksEntries: AugmentedEvent<ApiType, [paraIds: Vec<u32>], { paraIds: Vec<u32> }>;
            /**
             * We received response for xcm
             **/
            ReceivedBuyCoreXCMResult: AugmentedEvent<
                ApiType,
                [paraId: u32, response: StagingXcmV5Response],
                { paraId: u32; response: StagingXcmV5Response }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        xcmpQueue: {
            /**
             * An HRMP message was sent to a sibling parachain.
             **/
            XcmpMessageSent: AugmentedEvent<ApiType, [messageHash: U8aFixed], { messageHash: U8aFixed }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
    } // AugmentedEvents
} // declare module
