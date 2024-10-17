// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/submittable";

import type {
    ApiTypes,
    AugmentedSubmittable,
    SubmittableExtrinsic,
    SubmittableExtrinsicFunction,
} from "@polkadot/api-base/types";
import type { Data } from "@polkadot/types";
import type { Bytes, Compact, Null, Option, U8aFixed, Vec, bool, u128, u16, u32, u64, u8 } from "@polkadot/types-codec";
import type { AnyNumber, IMethod, ITuple } from "@polkadot/types-codec/types";
import type { AccountId32, Call, H256, MultiAddress, Perbill } from "@polkadot/types/interfaces/runtime";
import type {
    CumulusPrimitivesCoreAggregateMessageOrigin,
    CumulusPrimitivesParachainInherentParachainInherentData,
    DanceboxRuntimeOriginCaller,
    DanceboxRuntimePreserversAssignementPaymentExtra,
    DanceboxRuntimePreserversAssignementPaymentWitness,
    DanceboxRuntimeProxyType,
    DanceboxRuntimeSessionKeys,
    DanceboxRuntimeStreamPaymentAssetId,
    DanceboxRuntimeXcmConfigRelayChain,
    DpContainerChainGenesisDataContainerChainGenesisData,
    PalletBalancesAdjustmentDirection,
    PalletDataPreserversProfile,
    PalletIdentityJudgement,
    PalletIdentityLegacyIdentityInfo,
    PalletMultisigTimepoint,
    PalletPooledStakingAllTargetPool,
    PalletPooledStakingPendingOperationQuery,
    PalletPooledStakingSharesOrStake,
    PalletPooledStakingTargetPool,
    PalletStreamPaymentChangeKind,
    PalletStreamPaymentDepositChange,
    PalletStreamPaymentStreamConfig,
    PalletXcmCoreBuyerRelayXcmWeightConfigInner,
    SpRuntimeMultiSignature,
    SpTrieStorageProof,
    SpWeightsWeightV2Weight,
    StagingXcmExecutorAssetTransferTransferType,
    StagingXcmV4Location,
    StagingXcmV4Response,
    TpAuthorNotingInherentOwnParachainInherentData,
    TpTraitsParathreadParams,
    TpTraitsSlotFrequency,
    TpXcmCoreBuyerBuyCoreCollatorProof,
    XcmV3WeightLimit,
    XcmVersionedAssetId,
    XcmVersionedAssets,
    XcmVersionedLocation,
    XcmVersionedXcm,
} from "@polkadot/types/lookup";

export type __AugmentedSubmittable = AugmentedSubmittable<() => unknown>;
export type __SubmittableExtrinsic<ApiType extends ApiTypes> = SubmittableExtrinsic<ApiType>;
export type __SubmittableExtrinsicFunction<ApiType extends ApiTypes> = SubmittableExtrinsicFunction<ApiType>;

declare module "@polkadot/api-base/types/submittable" {
    interface AugmentedSubmittables<ApiType extends ApiTypes> {
        assetRate: {
            /**
             * Initialize a conversion rate to native balance for the given asset.
             *
             * ## Complexity
             *
             * - O(1)
             */
            create: AugmentedSubmittable<
                (
                    assetKind: u16 | AnyNumber | Uint8Array,
                    rate: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, u128]
            >;
            /**
             * Remove an existing conversion rate to native balance for the given asset.
             *
             * ## Complexity
             *
             * - O(1)
             */
            remove: AugmentedSubmittable<
                (assetKind: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /**
             * Update the conversion rate to native balance for the given asset.
             *
             * ## Complexity
             *
             * - O(1)
             */
            update: AugmentedSubmittable<
                (
                    assetKind: u16 | AnyNumber | Uint8Array,
                    rate: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, u128]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        authorInherent: {
            /**
             * This inherent is a workaround to run code after the "real" inherents have executed, but before transactions are
             * executed.
             */
            kickOffAuthorshipValidation: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        authorityAssignment: {
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        authorNoting: {
            killAuthorData: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            setAuthor: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    blockNumber: u32 | AnyNumber | Uint8Array,
                    author: AccountId32 | string | Uint8Array,
                    latestSlotNumber: u64 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32, AccountId32, u64]
            >;
            setLatestAuthorData: AugmentedSubmittable<
                (
                    data:
                        | TpAuthorNotingInherentOwnParachainInherentData
                        | { relayStorageProof?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [TpAuthorNotingInherentOwnParachainInherentData]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        balances: {
            /**
             * Burn the specified liquid free balance from the origin account.
             *
             * If the origin's account ends up below the existential deposit as a result of the burn and `keep_alive` is
             * false, the account will be reaped.
             *
             * Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible, this `burn` operation will
             * reduce total issuance by the amount _burned_.
             */
            burn: AugmentedSubmittable<
                (
                    value: Compact<u128> | AnyNumber | Uint8Array,
                    keepAlive: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u128>, bool]
            >;
            /**
             * Adjust the total issuance in a saturating way.
             *
             * Can only be called by root and always needs a positive `delta`.
             *
             * # Example
             */
            forceAdjustTotalIssuance: AugmentedSubmittable<
                (
                    direction: PalletBalancesAdjustmentDirection | "Increase" | "Decrease" | number | Uint8Array,
                    delta: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PalletBalancesAdjustmentDirection, Compact<u128>]
            >;
            /**
             * Set the regular balance of a given account.
             *
             * The dispatch origin for this call is `root`.
             */
            forceSetBalance: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    newFree: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Compact<u128>]
            >;
            /** Exactly as `transfer_allow_death`, except the origin must be root and the source account may be specified. */
            forceTransfer: AugmentedSubmittable<
                (
                    source:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    value: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, MultiAddress, Compact<u128>]
            >;
            /**
             * Unreserve some balance from a user by force.
             *
             * Can only be called by ROOT.
             */
            forceUnreserve: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    amount: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, u128]
            >;
            /**
             * Transfer the entire transferable balance from the caller account.
             *
             * NOTE: This function only attempts to transfer _transferable_ balances. This means that any locked, reserved, or
             * existential deposits (when `keep_alive` is `true`), will not be transferred by this function. To ensure that
             * this function results in a killed account, you might need to prepare the account by removing any reference
             * counters, storage deposits, etc...
             *
             * The dispatch origin of this call must be Signed.
             *
             * - `dest`: The recipient of the transfer.
             * - `keep_alive`: A boolean to determine if the `transfer_all` operation should send all of the funds the account
             *   has, causing the sender account to be killed (false), or transfer everything except at least the existential
             *   deposit, which will guarantee to keep the sender account alive (true).
             */
            transferAll: AugmentedSubmittable<
                (
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    keepAlive: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, bool]
            >;
            /**
             * Transfer some liquid free balance to another account.
             *
             * `transfer_allow_death` will set the `FreeBalance` of the sender and receiver. If the sender's account is below
             * the existential deposit as a result of the transfer, the account will be reaped.
             *
             * The dispatch origin for this call must be `Signed` by the transactor.
             */
            transferAllowDeath: AugmentedSubmittable<
                (
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    value: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Compact<u128>]
            >;
            /**
             * Same as the [`transfer_allow_death`][`transfer_allow_death`] call, but with a check that the transfer will not
             * kill the origin account.
             *
             * 99% of the time you want [`transfer_allow_death`][`transfer_allow_death`] instead.
             *
             * [`transfer_allow_death`]: struct.Pallet.html#method.transfer
             */
            transferKeepAlive: AugmentedSubmittable<
                (
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    value: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Compact<u128>]
            >;
            /**
             * Upgrade a specified account.
             *
             * - `origin`: Must be `Signed`.
             * - `who`: The account to be upgraded.
             *
             * This will waive the transaction fee if at least all but 10% of the accounts needed to be upgraded. (We let some
             * not have to be upgraded just in order to allow for the possibility of churn).
             */
            upgradeAccounts: AugmentedSubmittable<
                (who: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        collatorAssignment: {
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        configuration: {
            /** Setting this to true will disable consistency checks for the configuration setters. Use with caution. */
            setBypassConsistencyCheck: AugmentedSubmittable<
                (updated: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [bool]
            >;
            setCollatorsPerContainer: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            setCollatorsPerParathread: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            setFullRotationPeriod: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            setMaxCollators: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            setMaxOrchestratorCollators: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            setMaxParachainCoresPercentage: AugmentedSubmittable<
                (updated: Option<Perbill> | null | Uint8Array | Perbill | AnyNumber) => SubmittableExtrinsic<ApiType>,
                [Option<Perbill>]
            >;
            setMinOrchestratorCollators: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            setParathreadsPerCollator: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            setTargetContainerChainFullness: AugmentedSubmittable<
                (updated: Perbill | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Perbill]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        dataPreservers: {
            createProfile: AugmentedSubmittable<
                (
                    profile:
                        | PalletDataPreserversProfile
                        | { url?: any; paraIds?: any; mode?: any; assignmentRequest?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PalletDataPreserversProfile]
            >;
            deleteProfile: AugmentedSubmittable<
                (profileId: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            forceCreateProfile: AugmentedSubmittable<
                (
                    profile:
                        | PalletDataPreserversProfile
                        | { url?: any; paraIds?: any; mode?: any; assignmentRequest?: any }
                        | string
                        | Uint8Array,
                    forAccount: AccountId32 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PalletDataPreserversProfile, AccountId32]
            >;
            forceDeleteProfile: AugmentedSubmittable<
                (profileId: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            forceStartAssignment: AugmentedSubmittable<
                (
                    profileId: u64 | AnyNumber | Uint8Array,
                    paraId: u32 | AnyNumber | Uint8Array,
                    assignmentWitness: DanceboxRuntimePreserversAssignementPaymentWitness | "Free" | number | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, u32, DanceboxRuntimePreserversAssignementPaymentWitness]
            >;
            forceUpdateProfile: AugmentedSubmittable<
                (
                    profileId: u64 | AnyNumber | Uint8Array,
                    profile:
                        | PalletDataPreserversProfile
                        | { url?: any; paraIds?: any; mode?: any; assignmentRequest?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, PalletDataPreserversProfile]
            >;
            startAssignment: AugmentedSubmittable<
                (
                    profileId: u64 | AnyNumber | Uint8Array,
                    paraId: u32 | AnyNumber | Uint8Array,
                    assignerParam: DanceboxRuntimePreserversAssignementPaymentExtra | "Free" | number | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, u32, DanceboxRuntimePreserversAssignementPaymentExtra]
            >;
            stopAssignment: AugmentedSubmittable<
                (
                    profileId: u64 | AnyNumber | Uint8Array,
                    paraId: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, u32]
            >;
            updateProfile: AugmentedSubmittable<
                (
                    profileId: u64 | AnyNumber | Uint8Array,
                    profile:
                        | PalletDataPreserversProfile
                        | { url?: any; paraIds?: any; mode?: any; assignmentRequest?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, PalletDataPreserversProfile]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        foreignAssets: {
            /**
             * Approve an amount of asset for transfer by a delegated third-party account.
             *
             * Origin must be Signed.
             *
             * Ensures that `ApprovalDeposit` worth of `Currency` is reserved from signing account for the purpose of holding
             * the approval. If some non-zero amount of assets is already approved from signing account to `delegate`, then it
             * is topped up or unreserved to meet the right value.
             *
             * NOTE: The signing account does not need to own `amount` of assets at the point of making this call.
             *
             * - `id`: The identifier of the asset.
             * - `delegate`: The account to delegate permission to transfer asset.
             * - `amount`: The amount of asset that may be transferred by `delegate`. If there is already an approval in place,
             *   then this acts additively.
             *
             * Emits `ApprovedTransfer` on success.
             *
             * Weight: `O(1)`
             */
            approveTransfer: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    delegate:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    amount: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress, Compact<u128>]
            >;
            /**
             * Disallow further unprivileged transfers of an asset `id` to and from an account `who`.
             *
             * Origin must be Signed and the sender should be the Freezer of the asset `id`.
             *
             * - `id`: The identifier of the account's asset.
             * - `who`: The account to be unblocked.
             *
             * Emits `Blocked`.
             *
             * Weight: `O(1)`
             */
            block: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress]
            >;
            /**
             * Reduce the balance of `who` by as much as possible up to `amount` assets of `id`.
             *
             * Origin must be Signed and the sender should be the Manager of the asset `id`.
             *
             * Bails with `NoAccount` if the `who` is already dead.
             *
             * - `id`: The identifier of the asset to have some amount burned.
             * - `who`: The account to be debited from.
             * - `amount`: The maximum amount by which `who`'s balance should be reduced.
             *
             * Emits `Burned` with the actual amount burned. If this takes the balance to below the minimum for the asset,
             * then the amount burned is increased to take it to zero.
             *
             * Weight: `O(1)` Modes: Post-existence of `who`; Pre & post Zombie-status of `who`.
             */
            burn: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    amount: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress, Compact<u128>]
            >;
            /**
             * Cancel all of some asset approved for delegated transfer by a third-party account.
             *
             * Origin must be Signed and there must be an approval in place between signer and `delegate`.
             *
             * Unreserves any deposit previously reserved by `approve_transfer` for the approval.
             *
             * - `id`: The identifier of the asset.
             * - `delegate`: The account delegated permission to transfer asset.
             *
             * Emits `ApprovalCancelled` on success.
             *
             * Weight: `O(1)`
             */
            cancelApproval: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    delegate:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress]
            >;
            /**
             * Clear the metadata for an asset.
             *
             * Origin must be Signed and the sender should be the Owner of the asset `id`.
             *
             * Any deposit is freed for the asset owner.
             *
             * - `id`: The identifier of the asset to clear.
             *
             * Emits `MetadataCleared`.
             *
             * Weight: `O(1)`
             */
            clearMetadata: AugmentedSubmittable<
                (id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /**
             * Issue a new class of fungible assets from a public origin.
             *
             * This new asset class has no assets initially and its owner is the origin.
             *
             * The origin must conform to the configured `CreateOrigin` and have sufficient funds free.
             *
             * Funds of sender are reserved by `AssetDeposit`.
             *
             * Parameters:
             *
             * - `id`: The identifier of the new asset. This must not be currently in use to identify an existing asset. If
             *   [`NextAssetId`] is set, then this must be equal to it.
             * - `admin`: The admin of this class of assets. The admin is the initial address of each member of the asset
             *   class's admin team.
             * - `min_balance`: The minimum balance of this new asset that any single account must have. If an account's balance
             *   is reduced below this, then it collapses to zero.
             *
             * Emits `Created` event when successful.
             *
             * Weight: `O(1)`
             */
            create: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    admin:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    minBalance: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress, u128]
            >;
            /**
             * Destroy all accounts associated with a given asset.
             *
             * `destroy_accounts` should only be called after `start_destroy` has been called, and the asset is in a
             * `Destroying` state.
             *
             * Due to weight restrictions, this function may need to be called multiple times to fully destroy all accounts.
             * It will destroy `RemoveItemsLimit` accounts at a time.
             *
             * - `id`: The identifier of the asset to be destroyed. This must identify an existing asset.
             *
             * Each call emits the `Event::DestroyedAccounts` event.
             */
            destroyAccounts: AugmentedSubmittable<
                (id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /**
             * Destroy all approvals associated with a given asset up to the max (T::RemoveItemsLimit).
             *
             * `destroy_approvals` should only be called after `start_destroy` has been called, and the asset is in a
             * `Destroying` state.
             *
             * Due to weight restrictions, this function may need to be called multiple times to fully destroy all approvals.
             * It will destroy `RemoveItemsLimit` approvals at a time.
             *
             * - `id`: The identifier of the asset to be destroyed. This must identify an existing asset.
             *
             * Each call emits the `Event::DestroyedApprovals` event.
             */
            destroyApprovals: AugmentedSubmittable<
                (id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /**
             * Complete destroying asset and unreserve currency.
             *
             * `finish_destroy` should only be called after `start_destroy` has been called, and the asset is in a
             * `Destroying` state. All accounts or approvals should be destroyed before hand.
             *
             * - `id`: The identifier of the asset to be destroyed. This must identify an existing asset.
             *
             * Each successful call emits the `Event::Destroyed` event.
             */
            finishDestroy: AugmentedSubmittable<
                (id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /**
             * Alter the attributes of a given asset.
             *
             * Origin must be `ForceOrigin`.
             *
             * - `id`: The identifier of the asset.
             * - `owner`: The new Owner of this asset.
             * - `issuer`: The new Issuer of this asset.
             * - `admin`: The new Admin of this asset.
             * - `freezer`: The new Freezer of this asset.
             * - `min_balance`: The minimum balance of this new asset that any single account must have. If an account's balance
             *   is reduced below this, then it collapses to zero.
             * - `is_sufficient`: Whether a non-zero balance of this asset is deposit of sufficient value to account for the
             *   state bloat associated with its balance storage. If set to `true`, then non-zero balances may be stored
             *   without a `consumer` reference (and thus an ED in the Balances pallet or whatever else is used to control
             *   user-account state growth).
             * - `is_frozen`: Whether this asset class is frozen except for permissioned/admin instructions.
             *
             * Emits `AssetStatusChanged` with the identity of the asset.
             *
             * Weight: `O(1)`
             */
            forceAssetStatus: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    owner:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    issuer:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    admin:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    freezer:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    minBalance: Compact<u128> | AnyNumber | Uint8Array,
                    isSufficient: bool | boolean | Uint8Array,
                    isFrozen: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress, MultiAddress, MultiAddress, MultiAddress, Compact<u128>, bool, bool]
            >;
            /**
             * Cancel all of some asset approved for delegated transfer by a third-party account.
             *
             * Origin must be either ForceOrigin or Signed origin with the signer being the Admin account of the asset `id`.
             *
             * Unreserves any deposit previously reserved by `approve_transfer` for the approval.
             *
             * - `id`: The identifier of the asset.
             * - `delegate`: The account delegated permission to transfer asset.
             *
             * Emits `ApprovalCancelled` on success.
             *
             * Weight: `O(1)`
             */
            forceCancelApproval: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    owner:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    delegate:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress, MultiAddress]
            >;
            /**
             * Clear the metadata for an asset.
             *
             * Origin must be ForceOrigin.
             *
             * Any deposit is returned.
             *
             * - `id`: The identifier of the asset to clear.
             *
             * Emits `MetadataCleared`.
             *
             * Weight: `O(1)`
             */
            forceClearMetadata: AugmentedSubmittable<
                (id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /**
             * Issue a new class of fungible assets from a privileged origin.
             *
             * This new asset class has no assets initially.
             *
             * The origin must conform to `ForceOrigin`.
             *
             * Unlike `create`, no funds are reserved.
             *
             * - `id`: The identifier of the new asset. This must not be currently in use to identify an existing asset. If
             *   [`NextAssetId`] is set, then this must be equal to it.
             * - `owner`: The owner of this class of assets. The owner has full superuser permissions over this asset, but may
             *   later change and configure the permissions using `transfer_ownership` and `set_team`.
             * - `min_balance`: The minimum balance of this new asset that any single account must have. If an account's balance
             *   is reduced below this, then it collapses to zero.
             *
             * Emits `ForceCreated` event when successful.
             *
             * Weight: `O(1)`
             */
            forceCreate: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    owner:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    isSufficient: bool | boolean | Uint8Array,
                    minBalance: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress, bool, Compact<u128>]
            >;
            /**
             * Force the metadata for an asset to some value.
             *
             * Origin must be ForceOrigin.
             *
             * Any deposit is left alone.
             *
             * - `id`: The identifier of the asset to update.
             * - `name`: The user friendly name of this asset. Limited in length by `StringLimit`.
             * - `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`.
             * - `decimals`: The number of decimals this asset uses to represent one unit.
             *
             * Emits `MetadataSet`.
             *
             * Weight: `O(N + S)` where N and S are the length of the name and symbol respectively.
             */
            forceSetMetadata: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    name: Bytes | string | Uint8Array,
                    symbol: Bytes | string | Uint8Array,
                    decimals: u8 | AnyNumber | Uint8Array,
                    isFrozen: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, Bytes, Bytes, u8, bool]
            >;
            /**
             * Move some assets from one account to another.
             *
             * Origin must be Signed and the sender should be the Admin of the asset `id`.
             *
             * - `id`: The identifier of the asset to have some amount transferred.
             * - `source`: The account to be debited.
             * - `dest`: The account to be credited.
             * - `amount`: The amount by which the `source`'s balance of assets should be reduced and `dest`'s balance
             *   increased. The amount actually transferred may be slightly greater in the case that the transfer would
             *   otherwise take the `source` balance above zero but below the minimum balance. Must be greater than zero.
             *
             * Emits `Transferred` with the actual amount transferred. If this takes the source balance to below the minimum
             * for the asset, then the amount transferred is increased to take it to zero.
             *
             * Weight: `O(1)` Modes: Pre-existence of `dest`; Post-existence of `source`; Account pre-existence of `dest`.
             */
            forceTransfer: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    source:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    amount: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress, MultiAddress, Compact<u128>]
            >;
            /**
             * Disallow further unprivileged transfers of an asset `id` from an account `who`. `who` must already exist as an
             * entry in `Account`s of the asset. If you want to freeze an account that does not have an entry, use
             * `touch_other` first.
             *
             * Origin must be Signed and the sender should be the Freezer of the asset `id`.
             *
             * - `id`: The identifier of the asset to be frozen.
             * - `who`: The account to be frozen.
             *
             * Emits `Frozen`.
             *
             * Weight: `O(1)`
             */
            freeze: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress]
            >;
            /**
             * Disallow further unprivileged transfers for the asset class.
             *
             * Origin must be Signed and the sender should be the Freezer of the asset `id`.
             *
             * - `id`: The identifier of the asset to be frozen.
             *
             * Emits `Frozen`.
             *
             * Weight: `O(1)`
             */
            freezeAsset: AugmentedSubmittable<
                (id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /**
             * Mint assets of a particular class.
             *
             * The origin must be Signed and the sender must be the Issuer of the asset `id`.
             *
             * - `id`: The identifier of the asset to have some amount minted.
             * - `beneficiary`: The account to be credited with the minted assets.
             * - `amount`: The amount of the asset to be minted.
             *
             * Emits `Issued` event when successful.
             *
             * Weight: `O(1)` Modes: Pre-existing balance of `beneficiary`; Account pre-existence of `beneficiary`.
             */
            mint: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    beneficiary:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    amount: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress, Compact<u128>]
            >;
            /**
             * Return the deposit (if any) of an asset account or a consumer reference (if any) of an account.
             *
             * The origin must be Signed.
             *
             * - `id`: The identifier of the asset for which the caller would like the deposit refunded.
             * - `allow_burn`: If `true` then assets may be destroyed in order to complete the refund.
             *
             * Emits `Refunded` event when successful.
             */
            refund: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    allowBurn: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, bool]
            >;
            /**
             * Return the deposit (if any) of a target asset account. Useful if you are the depositor.
             *
             * The origin must be Signed and either the account owner, depositor, or asset `Admin`. In order to burn a
             * non-zero balance of the asset, the caller must be the account and should use `refund`.
             *
             * - `id`: The identifier of the asset for the account holding a deposit.
             * - `who`: The account to refund.
             *
             * Emits `Refunded` event when successful.
             */
            refundOther: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress]
            >;
            /**
             * Set the metadata for an asset.
             *
             * Origin must be Signed and the sender should be the Owner of the asset `id`.
             *
             * Funds of sender are reserved according to the formula: `MetadataDepositBase + MetadataDepositPerByte *
             * (name.len + symbol.len)` taking into account any already reserved funds.
             *
             * - `id`: The identifier of the asset to update.
             * - `name`: The user friendly name of this asset. Limited in length by `StringLimit`.
             * - `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`.
             * - `decimals`: The number of decimals this asset uses to represent one unit.
             *
             * Emits `MetadataSet`.
             *
             * Weight: `O(1)`
             */
            setMetadata: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    name: Bytes | string | Uint8Array,
                    symbol: Bytes | string | Uint8Array,
                    decimals: u8 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, Bytes, Bytes, u8]
            >;
            /**
             * Sets the minimum balance of an asset.
             *
             * Only works if there aren't any accounts that are holding the asset or if the new value of `min_balance` is less
             * than the old one.
             *
             * Origin must be Signed and the sender has to be the Owner of the asset `id`.
             *
             * - `id`: The identifier of the asset.
             * - `min_balance`: The new value of `min_balance`.
             *
             * Emits `AssetMinBalanceChanged` event when successful.
             */
            setMinBalance: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    minBalance: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, u128]
            >;
            /**
             * Change the Issuer, Admin and Freezer of an asset.
             *
             * Origin must be Signed and the sender should be the Owner of the asset `id`.
             *
             * - `id`: The identifier of the asset to be frozen.
             * - `issuer`: The new Issuer of this asset.
             * - `admin`: The new Admin of this asset.
             * - `freezer`: The new Freezer of this asset.
             *
             * Emits `TeamChanged`.
             *
             * Weight: `O(1)`
             */
            setTeam: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    issuer:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    admin:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    freezer:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress, MultiAddress, MultiAddress]
            >;
            /**
             * Start the process of destroying a fungible asset class.
             *
             * `start_destroy` is the first in a series of extrinsics that should be called, to allow destruction of an asset
             * class.
             *
             * The origin must conform to `ForceOrigin` or must be `Signed` by the asset's `owner`.
             *
             * - `id`: The identifier of the asset to be destroyed. This must identify an existing asset.
             *
             * The asset class must be frozen before calling `start_destroy`.
             */
            startDestroy: AugmentedSubmittable<
                (id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /**
             * Allow unprivileged transfers to and from an account again.
             *
             * Origin must be Signed and the sender should be the Admin of the asset `id`.
             *
             * - `id`: The identifier of the asset to be frozen.
             * - `who`: The account to be unfrozen.
             *
             * Emits `Thawed`.
             *
             * Weight: `O(1)`
             */
            thaw: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress]
            >;
            /**
             * Allow unprivileged transfers for the asset again.
             *
             * Origin must be Signed and the sender should be the Admin of the asset `id`.
             *
             * - `id`: The identifier of the asset to be thawed.
             *
             * Emits `Thawed`.
             *
             * Weight: `O(1)`
             */
            thawAsset: AugmentedSubmittable<(id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u16]>;
            /**
             * Create an asset account for non-provider assets.
             *
             * A deposit will be taken from the signer account.
             *
             * - `origin`: Must be Signed; the signer account must have sufficient funds for a deposit to be taken.
             * - `id`: The identifier of the asset for the account to be created.
             *
             * Emits `Touched` event when successful.
             */
            touch: AugmentedSubmittable<(id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u16]>;
            /**
             * Create an asset account for `who`.
             *
             * A deposit will be taken from the signer account.
             *
             * - `origin`: Must be Signed by `Freezer` or `Admin` of the asset `id`; the signer account must have sufficient
             *   funds for a deposit to be taken.
             * - `id`: The identifier of the asset for the account to be created.
             * - `who`: The account to be created.
             *
             * Emits `Touched` event when successful.
             */
            touchOther: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress]
            >;
            /**
             * Move some assets from the sender account to another.
             *
             * Origin must be Signed.
             *
             * - `id`: The identifier of the asset to have some amount transferred.
             * - `target`: The account to be credited.
             * - `amount`: The amount by which the sender's balance of assets should be reduced and `target`'s balance
             *   increased. The amount actually transferred may be slightly greater in the case that the transfer would
             *   otherwise take the sender balance above zero but below the minimum balance. Must be greater than zero.
             *
             * Emits `Transferred` with the actual amount transferred. If this takes the source balance to below the minimum
             * for the asset, then the amount transferred is increased to take it to zero.
             *
             * Weight: `O(1)` Modes: Pre-existence of `target`; Post-existence of sender; Account pre-existence of `target`.
             */
            transfer: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    target:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    amount: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress, Compact<u128>]
            >;
            /**
             * Transfer some asset balance from a previously delegated account to some third-party account.
             *
             * Origin must be Signed and there must be an approval in place by the `owner` to the signer.
             *
             * If the entire amount approved for transfer is transferred, then any deposit previously reserved by
             * `approve_transfer` is unreserved.
             *
             * - `id`: The identifier of the asset.
             * - `owner`: The account which previously approved for a transfer of at least `amount` and from which the asset
             *   balance will be withdrawn.
             * - `destination`: The account to which the asset balance of `amount` will be transferred.
             * - `amount`: The amount of assets to transfer.
             *
             * Emits `TransferredApproved` on success.
             *
             * Weight: `O(1)`
             */
            transferApproved: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    owner:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    destination:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    amount: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress, MultiAddress, Compact<u128>]
            >;
            /**
             * Move some assets from the sender account to another, keeping the sender account alive.
             *
             * Origin must be Signed.
             *
             * - `id`: The identifier of the asset to have some amount transferred.
             * - `target`: The account to be credited.
             * - `amount`: The amount by which the sender's balance of assets should be reduced and `target`'s balance
             *   increased. The amount actually transferred may be slightly greater in the case that the transfer would
             *   otherwise take the sender balance above zero but below the minimum balance. Must be greater than zero.
             *
             * Emits `Transferred` with the actual amount transferred. If this takes the source balance to below the minimum
             * for the asset, then the amount transferred is increased to take it to zero.
             *
             * Weight: `O(1)` Modes: Pre-existence of `target`; Post-existence of sender; Account pre-existence of `target`.
             */
            transferKeepAlive: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    target:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    amount: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress, Compact<u128>]
            >;
            /**
             * Change the Owner of an asset.
             *
             * Origin must be Signed and the sender should be the Owner of the asset `id`.
             *
             * - `id`: The identifier of the asset.
             * - `owner`: The new Owner of this asset.
             *
             * Emits `OwnerChanged`.
             *
             * Weight: `O(1)`
             */
            transferOwnership: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    owner:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        foreignAssetsCreator: {
            /**
             * Change the xcm type mapping for a given assetId We also change this if the previous units per second where
             * pointing at the old assetType
             */
            changeExistingAssetType: AugmentedSubmittable<
                (
                    assetId: u16 | AnyNumber | Uint8Array,
                    newForeignAsset: StagingXcmV4Location | { parents?: any; interior?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, StagingXcmV4Location]
            >;
            /** Create new asset with the ForeignAssetCreator */
            createForeignAsset: AugmentedSubmittable<
                (
                    foreignAsset: StagingXcmV4Location | { parents?: any; interior?: any } | string | Uint8Array,
                    assetId: u16 | AnyNumber | Uint8Array,
                    admin: AccountId32 | string | Uint8Array,
                    isSufficient: bool | boolean | Uint8Array,
                    minBalance: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [StagingXcmV4Location, u16, AccountId32, bool, u128]
            >;
            /**
             * Destroy a given foreign assetId The weight in this case is the one returned by the trait plus the db writes and
             * reads from removing all the associated data
             */
            destroyForeignAsset: AugmentedSubmittable<
                (assetId: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /** Remove a given assetId -> foreignAsset association */
            removeExistingAssetType: AugmentedSubmittable<
                (assetId: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        identity: {
            /**
             * Accept a given username that an `authority` granted. The call must include the full username, as in
             * `username.suffix`.
             */
            acceptUsername: AugmentedSubmittable<
                (username: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Add a registrar to the system.
             *
             * The dispatch origin for this call must be `T::RegistrarOrigin`.
             *
             * - `account`: the account of the registrar.
             *
             * Emits `RegistrarAdded` if successful.
             */
            addRegistrar: AugmentedSubmittable<
                (
                    account:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Add the given account to the sender's subs.
             *
             * Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated to the sender.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered sub identity of `sub`.
             */
            addSub: AugmentedSubmittable<
                (
                    sub:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    data:
                        | Data
                        | { None: any }
                        | { Raw: any }
                        | { BlakeTwo256: any }
                        | { Sha256: any }
                        | { Keccak256: any }
                        | { ShaThree256: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Data]
            >;
            /**
             * Add an `AccountId` with permission to grant usernames with a given `suffix` appended.
             *
             * The authority can grant up to `allocation` usernames. To top up their allocation, they should just issue (or
             * request via governance) a new `add_username_authority` call.
             */
            addUsernameAuthority: AugmentedSubmittable<
                (
                    authority:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    suffix: Bytes | string | Uint8Array,
                    allocation: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Bytes, u32]
            >;
            /**
             * Cancel a previous request.
             *
             * Payment: A previously reserved deposit is returned on success.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered identity.
             *
             * - `reg_index`: The index of the registrar whose judgement is no longer requested.
             *
             * Emits `JudgementUnrequested` if successful.
             */
            cancelRequest: AugmentedSubmittable<
                (regIndex: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Clear an account's identity info and all sub-accounts and return all deposits.
             *
             * Payment: All reserved balances on the account are returned.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered identity.
             *
             * Emits `IdentityCleared` if successful.
             */
            clearIdentity: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Remove an account's identity and sub-account information and slash the deposits.
             *
             * Payment: Reserved balances from `set_subs` and `set_identity` are slashed and handled by `Slash`. Verification
             * request deposits are not returned; they should be cancelled manually using `cancel_request`.
             *
             * The dispatch origin for this call must match `T::ForceOrigin`.
             *
             * - `target`: the account whose identity the judgement is upon. This must be an account with a registered identity.
             *
             * Emits `IdentityKilled` if successful.
             */
            killIdentity: AugmentedSubmittable<
                (
                    target:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Provide a judgement for an account's identity.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must be the account of the registrar whose
             * index is `reg_index`.
             *
             * - `reg_index`: the index of the registrar whose judgement is being made.
             * - `target`: the account whose identity the judgement is upon. This must be an account with a registered identity.
             * - `judgement`: the judgement of the registrar of index `reg_index` about `target`.
             * - `identity`: The hash of the [`IdentityInformationProvider`] for that the judgement is provided.
             *
             * Note: Judgements do not apply to a username.
             *
             * Emits `JudgementGiven` if successful.
             */
            provideJudgement: AugmentedSubmittable<
                (
                    regIndex: Compact<u32> | AnyNumber | Uint8Array,
                    target:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    judgement:
                        | PalletIdentityJudgement
                        | { Unknown: any }
                        | { FeePaid: any }
                        | { Reasonable: any }
                        | { KnownGood: any }
                        | { OutOfDate: any }
                        | { LowQuality: any }
                        | { Erroneous: any }
                        | string
                        | Uint8Array,
                    identity: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, MultiAddress, PalletIdentityJudgement, H256]
            >;
            /**
             * Remove the sender as a sub-account.
             *
             * Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated to the sender (_not_
             * the original depositor).
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered super-identity.
             *
             * NOTE: This should not normally be used, but is provided in the case that the non- controller of an account is
             * maliciously registered as a sub-account.
             */
            quitSub: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Remove a username that corresponds to an account with no identity. Exists when a user gets a username but then
             * calls `clear_identity`.
             */
            removeDanglingUsername: AugmentedSubmittable<
                (username: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Remove an expired username approval. The username was approved by an authority but never accepted by the user
             * and must now be beyond its expiration. The call must include the full username, as in `username.suffix`.
             */
            removeExpiredApproval: AugmentedSubmittable<
                (username: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Remove the given account from the sender's subs.
             *
             * Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated to the sender.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered sub identity of `sub`.
             */
            removeSub: AugmentedSubmittable<
                (
                    sub:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /** Remove `authority` from the username authorities. */
            removeUsernameAuthority: AugmentedSubmittable<
                (
                    authority:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Alter the associated name of the given sub-account.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered sub identity of `sub`.
             */
            renameSub: AugmentedSubmittable<
                (
                    sub:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    data:
                        | Data
                        | { None: any }
                        | { Raw: any }
                        | { BlakeTwo256: any }
                        | { Sha256: any }
                        | { Keccak256: any }
                        | { ShaThree256: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Data]
            >;
            /**
             * Request a judgement from a registrar.
             *
             * Payment: At most `max_fee` will be reserved for payment to the registrar if judgement given.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered identity.
             *
             * - `reg_index`: The index of the registrar whose judgement is requested.
             * - `max_fee`: The maximum fee that may be paid. This should just be auto-populated as:
             *
             * ```nocompile
             * Self::registrars().get(reg_index).unwrap().fee;
             * ```
             *
             * Emits `JudgementRequested` if successful.
             */
            requestJudgement: AugmentedSubmittable<
                (
                    regIndex: Compact<u32> | AnyNumber | Uint8Array,
                    maxFee: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, Compact<u128>]
            >;
            /**
             * Change the account associated with a registrar.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must be the account of the registrar whose
             * index is `index`.
             *
             * - `index`: the index of the registrar whose fee is to be set.
             * - `new`: the new account ID.
             */
            setAccountId: AugmentedSubmittable<
                (
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    updated:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, MultiAddress]
            >;
            /**
             * Set the fee required for a judgement to be requested from a registrar.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must be the account of the registrar whose
             * index is `index`.
             *
             * - `index`: the index of the registrar whose fee is to be set.
             * - `fee`: the new fee.
             */
            setFee: AugmentedSubmittable<
                (
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    fee: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, Compact<u128>]
            >;
            /**
             * Set the field information for a registrar.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must be the account of the registrar whose
             * index is `index`.
             *
             * - `index`: the index of the registrar whose fee is to be set.
             * - `fields`: the fields that the registrar concerns themselves with.
             */
            setFields: AugmentedSubmittable<
                (
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    fields: u64 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, u64]
            >;
            /**
             * Set an account's identity information and reserve the appropriate deposit.
             *
             * If the account already has identity information, the deposit is taken as part payment for the new deposit.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `info`: The identity information.
             *
             * Emits `IdentitySet` if successful.
             */
            setIdentity: AugmentedSubmittable<
                (
                    info:
                        | PalletIdentityLegacyIdentityInfo
                        | {
                              additional?: any;
                              display?: any;
                              legal?: any;
                              web?: any;
                              riot?: any;
                              email?: any;
                              pgpFingerprint?: any;
                              image?: any;
                              twitter?: any;
                          }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PalletIdentityLegacyIdentityInfo]
            >;
            /** Set a given username as the primary. The username should include the suffix. */
            setPrimaryUsername: AugmentedSubmittable<
                (username: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Set the sub-accounts of the sender.
             *
             * Payment: Any aggregate balance reserved by previous `set_subs` calls will be returned and an amount
             * `SubAccountDeposit` will be reserved for each item in `subs`.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered identity.
             *
             * - `subs`: The identity's (new) sub-accounts.
             */
            setSubs: AugmentedSubmittable<
                (
                    subs:
                        | Vec<ITuple<[AccountId32, Data]>>
                        | [
                              AccountId32 | string | Uint8Array,
                              (
                                  | Data
                                  | { None: any }
                                  | { Raw: any }
                                  | { BlakeTwo256: any }
                                  | { Sha256: any }
                                  | { Keccak256: any }
                                  | { ShaThree256: any }
                                  | string
                                  | Uint8Array
                              ),
                          ][]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<ITuple<[AccountId32, Data]>>]
            >;
            /**
             * Set the username for `who`. Must be called by a username authority.
             *
             * The authority must have an `allocation`. Users can either pre-sign their usernames or accept them later.
             *
             * Usernames must:
             *
             * - Only contain lowercase ASCII characters or digits.
             * - When combined with the suffix of the issuing authority be _less than_ the `MaxUsernameLength`.
             */
            setUsernameFor: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    username: Bytes | string | Uint8Array,
                    signature:
                        | Option<SpRuntimeMultiSignature>
                        | null
                        | Uint8Array
                        | SpRuntimeMultiSignature
                        | { Ed25519: any }
                        | { Sr25519: any }
                        | { Ecdsa: any }
                        | string
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Bytes, Option<SpRuntimeMultiSignature>]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        invulnerables: {
            /**
             * Add a new account `who` to the list of `Invulnerables` collators.
             *
             * The origin for this call must be the `UpdateOrigin`.
             */
            addInvulnerable: AugmentedSubmittable<
                (who: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Remove an account `who` from the list of `Invulnerables` collators.
             *
             * The origin for this call must be the `UpdateOrigin`.
             */
            removeInvulnerable: AugmentedSubmittable<
                (who: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        maintenanceMode: {
            /**
             * Place the chain in maintenance mode
             *
             * Weight cost is:
             *
             * - One DB read to ensure we're not already in maintenance mode
             * - Three DB writes - 1 for the mode, 1 for suspending xcm execution, 1 for the event
             */
            enterMaintenanceMode: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Return the chain to normal operating mode
             *
             * Weight cost is:
             *
             * - One DB read to ensure we're in maintenance mode
             * - Three DB writes - 1 for the mode, 1 for resuming xcm execution, 1 for the event
             */
            resumeNormalOperation: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        messageQueue: {
            /**
             * Execute an overweight message.
             *
             * Temporary processing errors will be propagated whereas permanent errors are treated as success condition.
             *
             * - `origin`: Must be `Signed`.
             * - `message_origin`: The origin from which the message to be executed arrived.
             * - `page`: The page in the queue in which the message to be executed is sitting.
             * - `index`: The index into the queue of the message to be executed.
             * - `weight_limit`: The maximum amount of weight allowed to be consumed in the execution of the message.
             *
             * Benchmark complexity considerations: O(index + weight_limit).
             */
            executeOverweight: AugmentedSubmittable<
                (
                    messageOrigin:
                        | CumulusPrimitivesCoreAggregateMessageOrigin
                        | { Here: any }
                        | { Parent: any }
                        | { Sibling: any }
                        | string
                        | Uint8Array,
                    page: u32 | AnyNumber | Uint8Array,
                    index: u32 | AnyNumber | Uint8Array,
                    weightLimit: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [CumulusPrimitivesCoreAggregateMessageOrigin, u32, u32, SpWeightsWeightV2Weight]
            >;
            /** Remove a page which has no more messages remaining to be processed or is stale. */
            reapPage: AugmentedSubmittable<
                (
                    messageOrigin:
                        | CumulusPrimitivesCoreAggregateMessageOrigin
                        | { Here: any }
                        | { Parent: any }
                        | { Sibling: any }
                        | string
                        | Uint8Array,
                    pageIndex: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [CumulusPrimitivesCoreAggregateMessageOrigin, u32]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        multisig: {
            /**
             * Register approval for a dispatch to be made from a deterministic composite account if approved by a total of
             * `threshold - 1` of `other_signatories`.
             *
             * Payment: `DepositBase` will be reserved if this is the first approval, plus `threshold` times `DepositFactor`.
             * It is returned once this dispatch happens or is cancelled.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `threshold`: The total number of approvals for this dispatch before it is executed.
             * - `other_signatories`: The accounts (other than the sender) who can approve this dispatch. May not be empty.
             * - `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is not the first approval,
             *   then it must be `Some`, with the timepoint (block number and transaction index) of the first approval
             *   transaction.
             * - `call_hash`: The hash of the call to be executed.
             *
             * NOTE: If this is the final approval, you will want to use `as_multi` instead.
             *
             * ## Complexity
             *
             * - `O(S)`.
             * - Up to one balance-reserve or unreserve operation.
             * - One passthrough operation, one insert, both `O(S)` where `S` is the number of signatories. `S` is capped by
             *   `MaxSignatories`, with weight being proportional.
             * - One encode & hash, both of complexity `O(S)`.
             * - Up to one binary search and insert (`O(logS + S)`).
             * - I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove.
             * - One event.
             * - Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit taken for its lifetime of
             *   `DepositBase + threshold * DepositFactor`.
             */
            approveAsMulti: AugmentedSubmittable<
                (
                    threshold: u16 | AnyNumber | Uint8Array,
                    otherSignatories: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                    maybeTimepoint:
                        | Option<PalletMultisigTimepoint>
                        | null
                        | Uint8Array
                        | PalletMultisigTimepoint
                        | { height?: any; index?: any }
                        | string,
                    callHash: U8aFixed | string | Uint8Array,
                    maxWeight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, Vec<AccountId32>, Option<PalletMultisigTimepoint>, U8aFixed, SpWeightsWeightV2Weight]
            >;
            /**
             * Register approval for a dispatch to be made from a deterministic composite account if approved by a total of
             * `threshold - 1` of `other_signatories`.
             *
             * If there are enough, then dispatch the call.
             *
             * Payment: `DepositBase` will be reserved if this is the first approval, plus `threshold` times `DepositFactor`.
             * It is returned once this dispatch happens or is cancelled.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `threshold`: The total number of approvals for this dispatch before it is executed.
             * - `other_signatories`: The accounts (other than the sender) who can approve this dispatch. May not be empty.
             * - `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is not the first approval,
             *   then it must be `Some`, with the timepoint (block number and transaction index) of the first approval
             *   transaction.
             * - `call`: The call to be executed.
             *
             * NOTE: Unless this is the final approval, you will generally want to use `approve_as_multi` instead, since it
             * only requires a hash of the call.
             *
             * Result is equivalent to the dispatched result if `threshold` is exactly `1`. Otherwise on success, result is
             * `Ok` and the result from the interior call, if it was executed, may be found in the deposited
             * `MultisigExecuted` event.
             *
             * ## Complexity
             *
             * - `O(S + Z + Call)`.
             * - Up to one balance-reserve or unreserve operation.
             * - One passthrough operation, one insert, both `O(S)` where `S` is the number of signatories. `S` is capped by
             *   `MaxSignatories`, with weight being proportional.
             * - One call encode & hash, both of complexity `O(Z)` where `Z` is tx-len.
             * - One encode & hash, both of complexity `O(S)`.
             * - Up to one binary search and insert (`O(logS + S)`).
             * - I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove.
             * - One event.
             * - The weight of the `call`.
             * - Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit taken for its lifetime of
             *   `DepositBase + threshold * DepositFactor`.
             */
            asMulti: AugmentedSubmittable<
                (
                    threshold: u16 | AnyNumber | Uint8Array,
                    otherSignatories: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                    maybeTimepoint:
                        | Option<PalletMultisigTimepoint>
                        | null
                        | Uint8Array
                        | PalletMultisigTimepoint
                        | { height?: any; index?: any }
                        | string,
                    call: Call | IMethod | string | Uint8Array,
                    maxWeight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, Vec<AccountId32>, Option<PalletMultisigTimepoint>, Call, SpWeightsWeightV2Weight]
            >;
            /**
             * Immediately dispatch a multi-signature call using a single approval from the caller.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `other_signatories`: The accounts (other than the sender) who are part of the multi-signature, but do not
             *   participate in the approval process.
             * - `call`: The call to be executed.
             *
             * Result is equivalent to the dispatched result.
             *
             * ## Complexity
             *
             * O(Z + C) where Z is the length of the call and C its execution weight.
             */
            asMultiThreshold1: AugmentedSubmittable<
                (
                    otherSignatories: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>, Call]
            >;
            /**
             * Cancel a pre-existing, on-going multisig transaction. Any deposit reserved previously for this operation will
             * be unreserved on success.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `threshold`: The total number of approvals for this dispatch before it is executed.
             * - `other_signatories`: The accounts (other than the sender) who can approve this dispatch. May not be empty.
             * - `timepoint`: The timepoint (block number and transaction index) of the first approval transaction for this
             *   dispatch.
             * - `call_hash`: The hash of the call to be executed.
             *
             * ## Complexity
             *
             * - `O(S)`.
             * - Up to one balance-reserve or unreserve operation.
             * - One passthrough operation, one insert, both `O(S)` where `S` is the number of signatories. `S` is capped by
             *   `MaxSignatories`, with weight being proportional.
             * - One encode & hash, both of complexity `O(S)`.
             * - One event.
             * - I/O: 1 read `O(S)`, one remove.
             * - Storage: removes one item.
             */
            cancelAsMulti: AugmentedSubmittable<
                (
                    threshold: u16 | AnyNumber | Uint8Array,
                    otherSignatories: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                    timepoint: PalletMultisigTimepoint | { height?: any; index?: any } | string | Uint8Array,
                    callHash: U8aFixed | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, Vec<AccountId32>, PalletMultisigTimepoint, U8aFixed]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        parachainInfo: {
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        parachainSystem: {
            /**
             * Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied later.
             *
             * The `check_version` parameter sets a boolean flag for whether or not the runtime's spec version and name should
             * be verified on upgrade. Since the authorization only has a hash, it cannot actually perform the verification.
             *
             * This call requires Root origin.
             */
            authorizeUpgrade: AugmentedSubmittable<
                (
                    codeHash: H256 | string | Uint8Array,
                    checkVersion: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, bool]
            >;
            /**
             * Provide the preimage (runtime binary) `code` for an upgrade that has been authorized.
             *
             * If the authorization required a version check, this call will ensure the spec name remains unchanged and that
             * the spec version has increased.
             *
             * Note that this function will not apply the new `code`, but only attempt to schedule the upgrade with the Relay
             * Chain.
             *
             * All origins are allowed.
             */
            enactAuthorizedUpgrade: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Set the current validation data.
             *
             * This should be invoked exactly once per block. It will panic at the finalization phase if the call was not
             * invoked.
             *
             * The dispatch origin for this call must be `Inherent`
             *
             * As a side effect, this function upgrades the current validation function if the appropriate time has come.
             */
            setValidationData: AugmentedSubmittable<
                (
                    data:
                        | CumulusPrimitivesParachainInherentParachainInherentData
                        | {
                              validationData?: any;
                              relayChainState?: any;
                              downwardMessages?: any;
                              horizontalMessages?: any;
                          }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [CumulusPrimitivesParachainInherentParachainInherentData]
            >;
            sudoSendUpwardMessage: AugmentedSubmittable<
                (message: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        polkadotXcm: {
            /**
             * Claims assets trapped on this pallet because of leftover assets during XCM execution.
             *
             * - `origin`: Anyone can call this extrinsic.
             * - `assets`: The exact assets that were trapped. Use the version to specify what version was the latest when they
             *   were trapped.
             * - `beneficiary`: The location/account where the claimed assets will be deposited.
             */
            claimAssets: AugmentedSubmittable<
                (
                    assets: XcmVersionedAssets | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    beneficiary: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedAssets, XcmVersionedLocation]
            >;
            /**
             * Execute an XCM message from a local, signed, origin.
             *
             * An event is deposited indicating whether `msg` could be executed completely or only partially.
             *
             * No more than `max_weight` will be used in its attempted execution. If this is less than the maximum amount of
             * weight that the message could take to be executed, then no execution attempt will be made.
             */
            execute: AugmentedSubmittable<
                (
                    message: XcmVersionedXcm | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    maxWeight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedXcm, SpWeightsWeightV2Weight]
            >;
            /**
             * Set a safe XCM version (the version that XCM should be encoded with if the most recent version a destination
             * can accept is unknown).
             *
             * - `origin`: Must be an origin specified by AdminOrigin.
             * - `maybe_xcm_version`: The default XCM encoding version, or `None` to disable.
             */
            forceDefaultXcmVersion: AugmentedSubmittable<
                (maybeXcmVersion: Option<u32> | null | Uint8Array | u32 | AnyNumber) => SubmittableExtrinsic<ApiType>,
                [Option<u32>]
            >;
            /**
             * Ask a location to notify us regarding their XCM version and any changes to it.
             *
             * - `origin`: Must be an origin specified by AdminOrigin.
             * - `location`: The location to which we should subscribe for XCM version notifications.
             */
            forceSubscribeVersionNotify: AugmentedSubmittable<
                (
                    location: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation]
            >;
            /**
             * Set or unset the global suspension state of the XCM executor.
             *
             * - `origin`: Must be an origin specified by AdminOrigin.
             * - `suspended`: `true` to suspend, `false` to resume.
             */
            forceSuspension: AugmentedSubmittable<
                (suspended: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [bool]
            >;
            /**
             * Require that a particular destination should no longer notify us regarding any XCM version changes.
             *
             * - `origin`: Must be an origin specified by AdminOrigin.
             * - `location`: The location to which we are currently subscribed for XCM version notifications which we no longer
             *   desire.
             */
            forceUnsubscribeVersionNotify: AugmentedSubmittable<
                (
                    location: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation]
            >;
            /**
             * Extoll that a particular destination can be communicated with through a particular version of XCM.
             *
             * - `origin`: Must be an origin specified by AdminOrigin.
             * - `location`: The destination that is being described.
             * - `xcm_version`: The latest version of XCM that `location` supports.
             */
            forceXcmVersion: AugmentedSubmittable<
                (
                    location: StagingXcmV4Location | { parents?: any; interior?: any } | string | Uint8Array,
                    version: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [StagingXcmV4Location, u32]
            >;
            /**
             * Transfer some assets from the local chain to the destination chain through their local, destination or remote
             * reserve.
             *
             * `assets` must have same reserve location and may not be teleportable to `dest`.
             *
             * - `assets` have local reserve: transfer assets to sovereign account of destination chain and forward a
             *   notification XCM to `dest` to mint and deposit reserve-based assets to `beneficiary`.
             * - `assets` have destination reserve: burn local assets and forward a notification to `dest` chain to withdraw the
             *   reserve assets from this chain's sovereign account and deposit them to `beneficiary`.
             * - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move reserves from this
             *   chain's SA to `dest` chain's SA, and forward another XCM to `dest` to mint and deposit reserve-based assets
             *   to `beneficiary`.
             *
             * Fee payment on the destination side is made from the asset in the `assets` vector of index `fee_asset_item`, up
             * to enough to pay for `weight_limit` of weight. If more weight is needed than `weight_limit`, then the operation
             * will fail and the sent assets may be at risk.
             *
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `[Parent, Parachain(..)]` to send from parachain
             *   to parachain, or `[Parachain(..)]` to send from relay to parachain.
             * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be an
             *   `AccountId32` value.
             * - `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the `dest` (and
             *   possibly reserve) chains.
             * - `fee_asset_item`: The index into `assets` of the item which should be used to pay fees.
             * - `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase.
             */
            limitedReserveTransferAssets: AugmentedSubmittable<
                (
                    dest: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    beneficiary: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    assets: XcmVersionedAssets | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array,
                    weightLimit: XcmV3WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation, XcmVersionedLocation, XcmVersionedAssets, u32, XcmV3WeightLimit]
            >;
            /**
             * Teleport some assets from the local chain to some destination chain.
             *
             * Fee payment on the destination side is made from the asset in the `assets` vector of index `fee_asset_item`, up
             * to enough to pay for `weight_limit` of weight. If more weight is needed than `weight_limit`, then the operation
             * will fail and the sent assets may be at risk.
             *
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `[Parent, Parachain(..)]` to send from parachain
             *   to parachain, or `[Parachain(..)]` to send from relay to parachain.
             * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be an
             *   `AccountId32` value.
             * - `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the `dest` chain.
             * - `fee_asset_item`: The index into `assets` of the item which should be used to pay fees.
             * - `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase.
             */
            limitedTeleportAssets: AugmentedSubmittable<
                (
                    dest: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    beneficiary: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    assets: XcmVersionedAssets | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array,
                    weightLimit: XcmV3WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation, XcmVersionedLocation, XcmVersionedAssets, u32, XcmV3WeightLimit]
            >;
            /**
             * Transfer some assets from the local chain to the destination chain through their local, destination or remote
             * reserve.
             *
             * `assets` must have same reserve location and may not be teleportable to `dest`.
             *
             * - `assets` have local reserve: transfer assets to sovereign account of destination chain and forward a
             *   notification XCM to `dest` to mint and deposit reserve-based assets to `beneficiary`.
             * - `assets` have destination reserve: burn local assets and forward a notification to `dest` chain to withdraw the
             *   reserve assets from this chain's sovereign account and deposit them to `beneficiary`.
             * - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move reserves from this
             *   chain's SA to `dest` chain's SA, and forward another XCM to `dest` to mint and deposit reserve-based assets
             *   to `beneficiary`.
             *
             * **This function is deprecated: Use `limited_reserve_transfer_assets` instead.**
             *
             * Fee payment on the destination side is made from the asset in the `assets` vector of index `fee_asset_item`.
             * The weight limit for fees is not provided and thus is unlimited, with all fees taken as needed from the asset.
             *
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `[Parent, Parachain(..)]` to send from parachain
             *   to parachain, or `[Parachain(..)]` to send from relay to parachain.
             * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be an
             *   `AccountId32` value.
             * - `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the `dest` (and
             *   possibly reserve) chains.
             * - `fee_asset_item`: The index into `assets` of the item which should be used to pay fees.
             */
            reserveTransferAssets: AugmentedSubmittable<
                (
                    dest: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    beneficiary: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    assets: XcmVersionedAssets | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation, XcmVersionedLocation, XcmVersionedAssets, u32]
            >;
            send: AugmentedSubmittable<
                (
                    dest: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    message: XcmVersionedXcm | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation, XcmVersionedXcm]
            >;
            /**
             * Teleport some assets from the local chain to some destination chain.
             *
             * **This function is deprecated: Use `limited_teleport_assets` instead.**
             *
             * Fee payment on the destination side is made from the asset in the `assets` vector of index `fee_asset_item`.
             * The weight limit for fees is not provided and thus is unlimited, with all fees taken as needed from the asset.
             *
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `[Parent, Parachain(..)]` to send from parachain
             *   to parachain, or `[Parachain(..)]` to send from relay to parachain.
             * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be an
             *   `AccountId32` value.
             * - `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the `dest` chain.
             * - `fee_asset_item`: The index into `assets` of the item which should be used to pay fees.
             */
            teleportAssets: AugmentedSubmittable<
                (
                    dest: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    beneficiary: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    assets: XcmVersionedAssets | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation, XcmVersionedLocation, XcmVersionedAssets, u32]
            >;
            /**
             * Transfer some assets from the local chain to the destination chain through their local, destination or remote
             * reserve, or through teleports.
             *
             * Fee payment on the destination side is made from the asset in the `assets` vector of index `fee_asset_item`
             * (hence referred to as `fees`), up to enough to pay for `weight_limit` of weight. If more weight is needed than
             * `weight_limit`, then the operation will fail and the sent assets may be at risk.
             *
             * `assets` (excluding `fees`) must have same reserve location or otherwise be teleportable to `dest`, no
             * limitations imposed on `fees`.
             *
             * - For local reserve: transfer assets to sovereign account of destination chain and forward a notification XCM to
             *   `dest` to mint and deposit reserve-based assets to `beneficiary`.
             * - For destination reserve: burn local assets and forward a notification to `dest` chain to withdraw the reserve
             *   assets from this chain's sovereign account and deposit them to `beneficiary`.
             * - For remote reserve: burn local assets, forward XCM to reserve chain to move reserves from this chain's SA to
             *   `dest` chain's SA, and forward another XCM to `dest` to mint and deposit reserve-based assets to
             *   `beneficiary`.
             * - For teleports: burn local assets and forward XCM to `dest` chain to mint/teleport assets and deposit them to
             *   `beneficiary`.
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send from
             *   parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain.
             * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be an
             *   `AccountId32` value.
             * - `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the `dest` (and
             *   possibly reserve) chains.
             * - `fee_asset_item`: The index into `assets` of the item which should be used to pay fees.
             * - `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase.
             */
            transferAssets: AugmentedSubmittable<
                (
                    dest: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    beneficiary: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    assets: XcmVersionedAssets | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array,
                    weightLimit: XcmV3WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation, XcmVersionedLocation, XcmVersionedAssets, u32, XcmV3WeightLimit]
            >;
            /**
             * Transfer assets from the local chain to the destination chain using explicit transfer types for assets and
             * fees.
             *
             * `assets` must have same reserve location or may be teleportable to `dest`. Caller must provide the
             * `assets_transfer_type` to be used for `assets`:
             *
             * - `TransferType::LocalReserve`: transfer assets to sovereign account of destination chain and forward a
             *   notification XCM to `dest` to mint and deposit reserve-based assets to `beneficiary`.
             * - `TransferType::DestinationReserve`: burn local assets and forward a notification to `dest` chain to withdraw
             *   the reserve assets from this chain's sovereign account and deposit them to `beneficiary`.
             * - `TransferType::RemoteReserve(reserve)`: burn local assets, forward XCM to `reserve` chain to move reserves from
             *   this chain's SA to `dest` chain's SA, and forward another XCM to `dest` to mint and deposit reserve-based
             *   assets to `beneficiary`. Typically the remote `reserve` is Asset Hub.
             * - `TransferType::Teleport`: burn local assets and forward XCM to `dest` chain to mint/teleport assets and deposit
             *   them to `beneficiary`.
             *
             * On the destination chain, as well as any intermediary hops, `BuyExecution` is used to buy execution using
             * transferred `assets` identified by `remote_fees_id`. Make sure enough of the specified `remote_fees_id` asset
             * is included in the given list of `assets`. `remote_fees_id` should be enough to pay for `weight_limit`. If more
             * weight is needed than `weight_limit`, then the operation will fail and the sent assets may be at risk.
             *
             * `remote_fees_id` may use different transfer type than rest of `assets` and can be specified through
             * `fees_transfer_type`.
             *
             * The caller needs to specify what should happen to the transferred assets once they reach the `dest` chain. This
             * is done through the `custom_xcm_on_dest` parameter, which contains the instructions to execute on `dest` as a
             * final step. This is usually as simple as: `Xcm(vec![DepositAsset { assets: Wild(AllCounted(assets.len())),
             * beneficiary }])`, but could be something more exotic like sending the `assets` even further.
             *
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `[Parent, Parachain(..)]` to send from parachain
             *   to parachain, or `[Parachain(..)]` to send from relay to parachain, or `(parents: 2, (GlobalConsensus(..),
             *   ..))` to send from parachain across a bridge to another ecosystem destination.
             * - `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the `dest` (and
             *   possibly reserve) chains.
             * - `assets_transfer_type`: The XCM `TransferType` used to transfer the `assets`.
             * - `remote_fees_id`: One of the included `assets` to be used to pay fees.
             * - `fees_transfer_type`: The XCM `TransferType` used to transfer the `fees` assets.
             * - `custom_xcm_on_dest`: The XCM to be executed on `dest` chain as the last step of the transfer, which also
             *   determines what happens to the assets on the destination chain.
             * - `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase.
             */
            transferAssetsUsingTypeAndThen: AugmentedSubmittable<
                (
                    dest: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    assets: XcmVersionedAssets | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    assetsTransferType:
                        | StagingXcmExecutorAssetTransferTransferType
                        | { Teleport: any }
                        | { LocalReserve: any }
                        | { DestinationReserve: any }
                        | { RemoteReserve: any }
                        | string
                        | Uint8Array,
                    remoteFeesId: XcmVersionedAssetId | { V3: any } | { V4: any } | string | Uint8Array,
                    feesTransferType:
                        | StagingXcmExecutorAssetTransferTransferType
                        | { Teleport: any }
                        | { LocalReserve: any }
                        | { DestinationReserve: any }
                        | { RemoteReserve: any }
                        | string
                        | Uint8Array,
                    customXcmOnDest: XcmVersionedXcm | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array,
                    weightLimit: XcmV3WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [
                    XcmVersionedLocation,
                    XcmVersionedAssets,
                    StagingXcmExecutorAssetTransferTransferType,
                    XcmVersionedAssetId,
                    StagingXcmExecutorAssetTransferTransferType,
                    XcmVersionedXcm,
                    XcmV3WeightLimit,
                ]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        pooledStaking: {
            claimManualRewards: AugmentedSubmittable<
                (
                    pairs:
                        | Vec<ITuple<[AccountId32, AccountId32]>>
                        | [AccountId32 | string | Uint8Array, AccountId32 | string | Uint8Array][]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<ITuple<[AccountId32, AccountId32]>>]
            >;
            /** Execute pending operations can incur in claim manual rewards per operation, we simply add the worst case */
            executePendingOperations: AugmentedSubmittable<
                (
                    operations:
                        | Vec<PalletPooledStakingPendingOperationQuery>
                        | (
                              | PalletPooledStakingPendingOperationQuery
                              | { delegator?: any; operation?: any }
                              | string
                              | Uint8Array
                          )[]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<PalletPooledStakingPendingOperationQuery>]
            >;
            rebalanceHold: AugmentedSubmittable<
                (
                    candidate: AccountId32 | string | Uint8Array,
                    delegator: AccountId32 | string | Uint8Array,
                    pool:
                        | PalletPooledStakingAllTargetPool
                        | "Joining"
                        | "AutoCompounding"
                        | "ManualRewards"
                        | "Leaving"
                        | number
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, AccountId32, PalletPooledStakingAllTargetPool]
            >;
            requestDelegate: AugmentedSubmittable<
                (
                    candidate: AccountId32 | string | Uint8Array,
                    pool: PalletPooledStakingTargetPool | "AutoCompounding" | "ManualRewards" | number | Uint8Array,
                    stake: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, PalletPooledStakingTargetPool, u128]
            >;
            /** Request undelegate can incur in either claim manual rewards or hold rebalances, we simply add the worst case */
            requestUndelegate: AugmentedSubmittable<
                (
                    candidate: AccountId32 | string | Uint8Array,
                    pool: PalletPooledStakingTargetPool | "AutoCompounding" | "ManualRewards" | number | Uint8Array,
                    amount: PalletPooledStakingSharesOrStake | { Shares: any } | { Stake: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, PalletPooledStakingTargetPool, PalletPooledStakingSharesOrStake]
            >;
            swapPool: AugmentedSubmittable<
                (
                    candidate: AccountId32 | string | Uint8Array,
                    sourcePool:
                        | PalletPooledStakingTargetPool
                        | "AutoCompounding"
                        | "ManualRewards"
                        | number
                        | Uint8Array,
                    amount: PalletPooledStakingSharesOrStake | { Shares: any } | { Stake: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, PalletPooledStakingTargetPool, PalletPooledStakingSharesOrStake]
            >;
            updateCandidatePosition: AugmentedSubmittable<
                (candidates: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        proxy: {
            /**
             * Register a proxy account for the sender that is able to make calls on its behalf.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             *
             * - `proxy`: The account that the `caller` would like to make a proxy.
             * - `proxy_type`: The permissions allowed for this proxy account.
             * - `delay`: The announcement period required of the initial proxy. Will generally be zero.
             */
            addProxy: AugmentedSubmittable<
                (
                    delegate:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    proxyType:
                        | DanceboxRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "Governance"
                        | "Staking"
                        | "CancelProxy"
                        | "Balances"
                        | "Registrar"
                        | "SudoRegistrar"
                        | "SessionKeyManagement"
                        | number
                        | Uint8Array,
                    delay: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, DanceboxRuntimeProxyType, u32]
            >;
            /**
             * Publish the hash of a proxy-call that will be made in the future.
             *
             * This must be called some number of blocks before the corresponding `proxy` is attempted if the delay associated
             * with the proxy relationship is greater than zero.
             *
             * No more than `MaxPending` announcements may be made at any one time.
             *
             * This will take a deposit of `AnnouncementDepositFactor` as well as `AnnouncementDepositBase` if there are no
             * other pending announcements.
             *
             * The dispatch origin for this call must be _Signed_ and a proxy of `real`.
             *
             * Parameters:
             *
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `call_hash`: The hash of the call to be made by the `real` account.
             */
            announce: AugmentedSubmittable<
                (
                    real:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    callHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, H256]
            >;
            /**
             * Spawn a fresh new account that is guaranteed to be otherwise inaccessible, and initialize it with a proxy of
             * `proxy_type` for `origin` sender.
             *
             * Requires a `Signed` origin.
             *
             * - `proxy_type`: The type of the proxy that the sender will be registered as over the new account. This will
             *   almost always be the most permissive `ProxyType` possible to allow for maximum flexibility.
             * - `index`: A disambiguation index, in case this is called multiple times in the same transaction (e.g. with
             *   `utility::batch`). Unless you're using `batch` you probably just want to use `0`.
             * - `delay`: The announcement period required of the initial proxy. Will generally be zero.
             *
             * Fails with `Duplicate` if this has already been called in this transaction, from the same sender, with the same
             * parameters.
             *
             * Fails if there are insufficient funds to pay for deposit.
             */
            createPure: AugmentedSubmittable<
                (
                    proxyType:
                        | DanceboxRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "Governance"
                        | "Staking"
                        | "CancelProxy"
                        | "Balances"
                        | "Registrar"
                        | "SudoRegistrar"
                        | "SessionKeyManagement"
                        | number
                        | Uint8Array,
                    delay: u32 | AnyNumber | Uint8Array,
                    index: u16 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [DanceboxRuntimeProxyType, u32, u16]
            >;
            /**
             * Removes a previously spawned pure proxy.
             *
             * WARNING: **All access to this account will be lost.** Any funds held in it will be inaccessible.
             *
             * Requires a `Signed` origin, and the sender account must have been created by a call to `pure` with
             * corresponding parameters.
             *
             * - `spawner`: The account that originally called `pure` to create this account.
             * - `index`: The disambiguation index originally passed to `pure`. Probably `0`.
             * - `proxy_type`: The proxy type originally passed to `pure`.
             * - `height`: The height of the chain when the call to `pure` was processed.
             * - `ext_index`: The extrinsic index in which the call to `pure` was processed.
             *
             * Fails with `NoPermission` in case the caller is not a previously created pure account whose `pure` call has
             * corresponding parameters.
             */
            killPure: AugmentedSubmittable<
                (
                    spawner:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    proxyType:
                        | DanceboxRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "Governance"
                        | "Staking"
                        | "CancelProxy"
                        | "Balances"
                        | "Registrar"
                        | "SudoRegistrar"
                        | "SessionKeyManagement"
                        | number
                        | Uint8Array,
                    index: u16 | AnyNumber | Uint8Array,
                    height: Compact<u32> | AnyNumber | Uint8Array,
                    extIndex: Compact<u32> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, DanceboxRuntimeProxyType, u16, Compact<u32>, Compact<u32>]
            >;
            /**
             * Dispatch the given `call` from an account that the sender is authorised for through `add_proxy`.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             *
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `force_proxy_type`: Specify the exact proxy type to be used and checked for this call.
             * - `call`: The call to be made by the `real` account.
             */
            proxy: AugmentedSubmittable<
                (
                    real:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    forceProxyType:
                        | Option<DanceboxRuntimeProxyType>
                        | null
                        | Uint8Array
                        | DanceboxRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "Governance"
                        | "Staking"
                        | "CancelProxy"
                        | "Balances"
                        | "Registrar"
                        | "SudoRegistrar"
                        | "SessionKeyManagement"
                        | number,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Option<DanceboxRuntimeProxyType>, Call]
            >;
            /**
             * Dispatch the given `call` from an account that the sender is authorized for through `add_proxy`.
             *
             * Removes any corresponding announcement(s).
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             *
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `force_proxy_type`: Specify the exact proxy type to be used and checked for this call.
             * - `call`: The call to be made by the `real` account.
             */
            proxyAnnounced: AugmentedSubmittable<
                (
                    delegate:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    real:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    forceProxyType:
                        | Option<DanceboxRuntimeProxyType>
                        | null
                        | Uint8Array
                        | DanceboxRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "Governance"
                        | "Staking"
                        | "CancelProxy"
                        | "Balances"
                        | "Registrar"
                        | "SudoRegistrar"
                        | "SessionKeyManagement"
                        | number,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, MultiAddress, Option<DanceboxRuntimeProxyType>, Call]
            >;
            /**
             * Remove the given announcement of a delegate.
             *
             * May be called by a target (proxied) account to remove a call that one of their delegates (`delegate`) has
             * announced they want to execute. The deposit is returned.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             *
             * - `delegate`: The account that previously announced the call.
             * - `call_hash`: The hash of the call to be made.
             */
            rejectAnnouncement: AugmentedSubmittable<
                (
                    delegate:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    callHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, H256]
            >;
            /**
             * Remove a given announcement.
             *
             * May be called by a proxy account to remove a call they previously announced and return the deposit.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             *
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `call_hash`: The hash of the call to be made by the `real` account.
             */
            removeAnnouncement: AugmentedSubmittable<
                (
                    real:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    callHash: H256 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, H256]
            >;
            /**
             * Unregister all proxy accounts for the sender.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * WARNING: This may be called on accounts created by `pure`, however if done, then the unreserved fees will be
             * inaccessible. **All access to this account will be lost.**
             */
            removeProxies: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Unregister a proxy account for the sender.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             *
             * - `proxy`: The account that the `caller` would like to remove as a proxy.
             * - `proxy_type`: The permissions currently enabled for the removed proxy account.
             */
            removeProxy: AugmentedSubmittable<
                (
                    delegate:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    proxyType:
                        | DanceboxRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "Governance"
                        | "Staking"
                        | "CancelProxy"
                        | "Balances"
                        | "Registrar"
                        | "SudoRegistrar"
                        | "SessionKeyManagement"
                        | number
                        | Uint8Array,
                    delay: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, DanceboxRuntimeProxyType, u32]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        registrar: {
            /**
             * Deregister container-chain.
             *
             * If a container-chain is registered but not marked as valid_for_collating, this will remove it from
             * `PendingVerification` as well.
             */
            deregister: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Deregister a parachain that no longer exists in the relay chain. The origin of this extrinsic will be rewarded
             * with the parachain deposit.
             */
            deregisterWithRelayProof: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    relayProofBlockNumber: u32 | AnyNumber | Uint8Array,
                    relayStorageProof: SpTrieStorageProof | { trieNodes?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32, SpTrieStorageProof]
            >;
            /** Mark container-chain valid for collating */
            markValidForCollating: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Pause container-chain from collating. Does not remove its boot nodes nor its genesis config. Only
             * container-chains that have been marked as valid_for_collating can be paused.
             */
            pauseContainerChain: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** Register container-chain */
            register: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    genesisData:
                        | DpContainerChainGenesisDataContainerChainGenesisData
                        | { storage?: any; name?: any; id?: any; forkId?: any; extensions?: any; properties?: any }
                        | string
                        | Uint8Array,
                    headData: Option<Bytes> | null | Uint8Array | Bytes | string
                ) => SubmittableExtrinsic<ApiType>,
                [u32, DpContainerChainGenesisDataContainerChainGenesisData, Option<Bytes>]
            >;
            /** Register parathread */
            registerParathread: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    slotFrequency: TpTraitsSlotFrequency | { min?: any; max?: any } | string | Uint8Array,
                    genesisData:
                        | DpContainerChainGenesisDataContainerChainGenesisData
                        | { storage?: any; name?: any; id?: any; forkId?: any; extensions?: any; properties?: any }
                        | string
                        | Uint8Array,
                    headData: Option<Bytes> | null | Uint8Array | Bytes | string
                ) => SubmittableExtrinsic<ApiType>,
                [u32, TpTraitsSlotFrequency, DpContainerChainGenesisDataContainerChainGenesisData, Option<Bytes>]
            >;
            /** Register parachain or parathread */
            registerWithRelayProof: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    parathreadParams:
                        | Option<TpTraitsParathreadParams>
                        | null
                        | Uint8Array
                        | TpTraitsParathreadParams
                        | { slotFrequency?: any }
                        | string,
                    relayProofBlockNumber: u32 | AnyNumber | Uint8Array,
                    relayStorageProof: SpTrieStorageProof | { trieNodes?: any } | string | Uint8Array,
                    managerSignature:
                        | SpRuntimeMultiSignature
                        | { Ed25519: any }
                        | { Sr25519: any }
                        | { Ecdsa: any }
                        | string
                        | Uint8Array,
                    genesisData:
                        | DpContainerChainGenesisDataContainerChainGenesisData
                        | { storage?: any; name?: any; id?: any; forkId?: any; extensions?: any; properties?: any }
                        | string
                        | Uint8Array,
                    headData: Option<Bytes> | null | Uint8Array | Bytes | string
                ) => SubmittableExtrinsic<ApiType>,
                [
                    u32,
                    Option<TpTraitsParathreadParams>,
                    u32,
                    SpTrieStorageProof,
                    SpRuntimeMultiSignature,
                    DpContainerChainGenesisDataContainerChainGenesisData,
                    Option<Bytes>,
                ]
            >;
            setParaManager: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    managerAddress: AccountId32 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, AccountId32]
            >;
            /** Change parathread params */
            setParathreadParams: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    slotFrequency: TpTraitsSlotFrequency | { min?: any; max?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, TpTraitsSlotFrequency]
            >;
            /** Unpause container-chain. Only container-chains that have been paused can be unpaused. */
            unpauseContainerChain: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        rootTesting: {
            /** A dispatch that will fill the block weight up to the given ratio. */
            fillBlock: AugmentedSubmittable<
                (ratio: Perbill | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Perbill]
            >;
            triggerDefensive: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        servicesPayment: {
            purchaseCredits: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    credit: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u128]
            >;
            /**
             * Set the number of block production credits for this para_id without paying for them. Can only be called by
             * root.
             */
            setBlockProductionCredits: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    freeBlockCredits: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * Set the number of block production credits for this para_id without paying for them. Can only be called by
             * root.
             */
            setCollatorAssignmentCredits: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    freeCollatorAssignmentCredits: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /** Helper to set and cleanup the `GivenFreeCredits` storage. Can only be called by root. */
            setGivenFreeCredits: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    givenFreeCredits: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, bool]
            >;
            /** Max core price for parathread in relay chain currency */
            setMaxCorePrice: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    maxCorePrice: Option<u128> | null | Uint8Array | u128 | AnyNumber
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<u128>]
            >;
            /**
             * Set the maximum tip a container chain is willing to pay to be assigned a collator on congestion. Can only be
             * called by container chain manager.
             */
            setMaxTip: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    maxTip: Option<u128> | null | Uint8Array | u128 | AnyNumber
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<u128>]
            >;
            /** Call index to set the refund address for non-spent tokens */
            setRefundAddress: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    refundAddress: Option<AccountId32> | null | Uint8Array | AccountId32 | string
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<AccountId32>]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        session: {
            /**
             * Removes any session key(s) of the function caller.
             *
             * This doesn't take effect until the next session.
             *
             * The dispatch origin of this function must be Signed and the account must be either be convertible to a
             * validator ID using the chain's typical addressing system (this usually means being a controller account) or
             * directly convertible into a validator ID (which usually means being a stash account).
             *
             * ## Complexity
             *
             * - `O(1)` in number of key types. Actual cost depends on the number of length of `T::Keys::key_ids()` which is
             *   fixed.
             */
            purgeKeys: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Sets the session key(s) of the function caller to `keys`. Allows an account to set its session key prior to
             * becoming a validator. This doesn't take effect until the next session.
             *
             * The dispatch origin of this function must be signed.
             *
             * ## Complexity
             *
             * - `O(1)`. Actual cost depends on the number of length of `T::Keys::key_ids()` which is fixed.
             */
            setKeys: AugmentedSubmittable<
                (
                    keys: DanceboxRuntimeSessionKeys | { nimbus?: any } | string | Uint8Array,
                    proof: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [DanceboxRuntimeSessionKeys, Bytes]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        streamPayment: {
            /**
             * Accepts a change requested before by the other party. Takes a nonce to prevent frontrunning attacks. If the
             * target made a request, the source is able to change their deposit.
             */
            acceptRequestedChange: AugmentedSubmittable<
                (
                    streamId: u64 | AnyNumber | Uint8Array,
                    requestNonce: u32 | AnyNumber | Uint8Array,
                    depositChange:
                        | Option<PalletStreamPaymentDepositChange>
                        | null
                        | Uint8Array
                        | PalletStreamPaymentDepositChange
                        | { Increase: any }
                        | { Decrease: any }
                        | { Absolute: any }
                        | string
                ) => SubmittableExtrinsic<ApiType>,
                [u64, u32, Option<PalletStreamPaymentDepositChange>]
            >;
            cancelChangeRequest: AugmentedSubmittable<
                (streamId: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /**
             * Close a given stream in which the origin is involved. It performs the pending payment before closing the
             * stream.
             */
            closeStream: AugmentedSubmittable<
                (streamId: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /**
             * Allows immediately changing the deposit for a stream, which is simpler than calling `request_change` with the
             * proper parameters. The call takes an asset id to ensure it has not changed (by an accepted request) before the
             * call is included in a block, in which case the unit is no longer the same and quantities will not have the same
             * scale/value.
             */
            immediatelyChangeDeposit: AugmentedSubmittable<
                (
                    streamId: u64 | AnyNumber | Uint8Array,
                    assetId: DanceboxRuntimeStreamPaymentAssetId | "Native" | number | Uint8Array,
                    change:
                        | PalletStreamPaymentDepositChange
                        | { Increase: any }
                        | { Decrease: any }
                        | { Absolute: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, DanceboxRuntimeStreamPaymentAssetId, PalletStreamPaymentDepositChange]
            >;
            /**
             * Create a payment stream from the origin to the target with provided config and initial deposit (in the asset
             * defined in the config).
             */
            openStream: AugmentedSubmittable<
                (
                    target: AccountId32 | string | Uint8Array,
                    config:
                        | PalletStreamPaymentStreamConfig
                        | { timeUnit?: any; assetId?: any; rate?: any }
                        | string
                        | Uint8Array,
                    initialDeposit: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, PalletStreamPaymentStreamConfig, u128]
            >;
            /** Perform the pending payment of a stream. Anyone can call this. */
            performPayment: AugmentedSubmittable<
                (streamId: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /**
             * Requests a change to a stream config or deposit.
             *
             * If the new config don't change the time unit and asset id, the change will be applied immediately if it is at
             * the desadvantage of the caller. Otherwise, the request is stored in the stream and will have to be approved by
             * the other party.
             *
             * This call accepts a deposit change, which can only be provided by the source of the stream. An absolute change
             * is required when changing asset id, as the current deposit will be released and a new deposit is required in
             * the new asset.
             */
            requestChange: AugmentedSubmittable<
                (
                    streamId: u64 | AnyNumber | Uint8Array,
                    kind:
                        | PalletStreamPaymentChangeKind
                        | { Suggestion: any }
                        | { Mandatory: any }
                        | string
                        | Uint8Array,
                    newConfig:
                        | PalletStreamPaymentStreamConfig
                        | { timeUnit?: any; assetId?: any; rate?: any }
                        | string
                        | Uint8Array,
                    depositChange:
                        | Option<PalletStreamPaymentDepositChange>
                        | null
                        | Uint8Array
                        | PalletStreamPaymentDepositChange
                        | { Increase: any }
                        | { Decrease: any }
                        | { Absolute: any }
                        | string
                ) => SubmittableExtrinsic<ApiType>,
                [
                    u64,
                    PalletStreamPaymentChangeKind,
                    PalletStreamPaymentStreamConfig,
                    Option<PalletStreamPaymentDepositChange>,
                ]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        sudo: {
            /**
             * Permanently removes the sudo key.
             *
             * **This cannot be un-done.**
             */
            removeKey: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo key. */
            setKey: AugmentedSubmittable<
                (
                    updated:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /** Authenticates the sudo key and dispatches a function call with `Root` origin. */
            sudo: AugmentedSubmittable<
                (call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Call]
            >;
            /**
             * Authenticates the sudo key and dispatches a function call with `Signed` origin from a given account.
             *
             * The dispatch origin for this call must be _Signed_.
             */
            sudoAs: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Call]
            >;
            /**
             * Authenticates the sudo key and dispatches a function call with `Root` origin. This function does not check the
             * weight of the call, and instead allows the Sudo user to specify the weight of the call.
             *
             * The dispatch origin for this call must be _Signed_.
             */
            sudoUncheckedWeight: AugmentedSubmittable<
                (
                    call: Call | IMethod | string | Uint8Array,
                    weight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Call, SpWeightsWeightV2Weight]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        system: {
            /**
             * Provide the preimage (runtime binary) `code` for an upgrade that has been authorized.
             *
             * If the authorization required a version check, this call will ensure the spec name remains unchanged and that
             * the spec version has increased.
             *
             * Depending on the runtime's `OnSetCode` configuration, this function may directly apply the new `code` in the
             * same block or attempt to schedule the upgrade.
             *
             * All origins are allowed.
             */
            applyAuthorizedUpgrade: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied later.
             *
             * This call requires Root origin.
             */
            authorizeUpgrade: AugmentedSubmittable<
                (codeHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied later.
             *
             * WARNING: This authorizes an upgrade that will take place without any safety checks, for example that the spec
             * name remains the same and that the version number increases. Not recommended for normal use. Use
             * `authorize_upgrade` instead.
             *
             * This call requires Root origin.
             */
            authorizeUpgradeWithoutChecks: AugmentedSubmittable<
                (codeHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Kill all storage items with a key that starts with the given prefix.
             *
             * **NOTE:** We rely on the Root origin to provide us the number of subkeys under the prefix we are removing to
             * accurately calculate the weight of this function.
             */
            killPrefix: AugmentedSubmittable<
                (
                    prefix: Bytes | string | Uint8Array,
                    subkeys: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, u32]
            >;
            /** Kill some items from storage. */
            killStorage: AugmentedSubmittable<
                (keys: Vec<Bytes> | (Bytes | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<Bytes>]
            >;
            /**
             * Make some on-chain remark.
             *
             * Can be executed by every `origin`.
             */
            remark: AugmentedSubmittable<
                (remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /** Make some on-chain remark and emit event. */
            remarkWithEvent: AugmentedSubmittable<
                (remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /** Set the new runtime code. */
            setCode: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Set the new runtime code without doing any checks of the given `code`.
             *
             * Note that runtime upgrades will not run if this is called with a not-increasing spec version!
             */
            setCodeWithoutChecks: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /** Set the number of pages in the WebAssembly environment's heap. */
            setHeapPages: AugmentedSubmittable<
                (pages: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /** Set some items of storage. */
            setStorage: AugmentedSubmittable<
                (
                    items: Vec<ITuple<[Bytes, Bytes]>> | [Bytes | string | Uint8Array, Bytes | string | Uint8Array][]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<ITuple<[Bytes, Bytes]>>]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        timestamp: {
            /**
             * Set the current time.
             *
             * This call should be invoked exactly once per block. It will panic at the finalization phase, if this call
             * hasn't been invoked by that time.
             *
             * The timestamp should be greater than the previous one by the amount specified by [`Config::MinimumPeriod`].
             *
             * The dispatch origin for this call must be _None_.
             *
             * This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware that changing the
             * complexity of this call could result exhausting the resources in a block to execute any other calls.
             *
             * ## Complexity
             *
             * - `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)
             * - 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in `on_finalize`)
             * - 1 event handler `on_timestamp_set`. Must be `O(1)`.
             */
            set: AugmentedSubmittable<
                (now: Compact<u64> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u64>]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        treasury: {
            /**
             * Check the status of the spend and remove it from the storage if processed.
             *
             * ## Dispatch Origin
             *
             * Must be signed.
             *
             * ## Details
             *
             * The status check is a prerequisite for retrying a failed payout. If a spend has either succeeded or expired, it
             * is removed from the storage by this function. In such instances, transaction fees are refunded.
             *
             * ### Parameters
             *
             * - `index`: The spend index.
             *
             * ## Events
             *
             * Emits [`Event::PaymentFailed`] if the spend payout has failed. Emits [`Event::SpendProcessed`] if the spend
             * payout has succeed.
             */
            checkStatus: AugmentedSubmittable<
                (index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Claim a spend.
             *
             * ## Dispatch Origin
             *
             * Must be signed
             *
             * ## Details
             *
             * Spends must be claimed within some temporal bounds. A spend may be claimed within one [`Config::PayoutPeriod`]
             * from the `valid_from` block. In case of a payout failure, the spend status must be updated with the
             * `check_status` dispatchable before retrying with the current function.
             *
             * ### Parameters
             *
             * - `index`: The spend index.
             *
             * ## Events
             *
             * Emits [`Event::Paid`] if successful.
             */
            payout: AugmentedSubmittable<(index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
            /**
             * Force a previously approved proposal to be removed from the approval queue.
             *
             * ## Dispatch Origin
             *
             * Must be [`Config::RejectOrigin`].
             *
             * ## Details
             *
             * The original deposit will no longer be returned.
             *
             * ### Parameters
             *
             * - `proposal_id`: The index of a proposal
             *
             * ### Complexity
             *
             * - O(A) where `A` is the number of approvals
             *
             * ### Errors
             *
             * - [`Error::ProposalNotApproved`]: The `proposal_id` supplied was not found in the approval queue, i.e., the
             *   proposal has not been approved. This could also mean the proposal does not exist altogether, thus there is no
             *   way it would have been approved in the first place.
             */
            removeApproval: AugmentedSubmittable<
                (proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /**
             * Propose and approve a spend of treasury funds.
             *
             * ## Dispatch Origin
             *
             * Must be [`Config::SpendOrigin`] with the `Success` value being at least `amount` of `asset_kind` in the native
             * asset. The amount of `asset_kind` is converted for assertion using the [`Config::BalanceConverter`].
             *
             * ## Details
             *
             * Create an approved spend for transferring a specific `amount` of `asset_kind` to a designated beneficiary. The
             * spend must be claimed using the `payout` dispatchable within the [`Config::PayoutPeriod`].
             *
             * ### Parameters
             *
             * - `asset_kind`: An indicator of the specific asset class to be spent.
             * - `amount`: The amount to be transferred from the treasury to the `beneficiary`.
             * - `beneficiary`: The beneficiary of the spend.
             * - `valid_from`: The block number from which the spend can be claimed. It can refer to the past if the resulting
             *   spend has not yet expired according to the [`Config::PayoutPeriod`]. If `None`, the spend can be claimed
             *   immediately after approval.
             *
             * ## Events
             *
             * Emits [`Event::AssetSpendApproved`] if successful.
             */
            spend: AugmentedSubmittable<
                (
                    assetKind: Null | null,
                    amount: Compact<u128> | AnyNumber | Uint8Array,
                    beneficiary: AccountId32 | string | Uint8Array,
                    validFrom: Option<u32> | null | Uint8Array | u32 | AnyNumber
                ) => SubmittableExtrinsic<ApiType>,
                [Null, Compact<u128>, AccountId32, Option<u32>]
            >;
            /**
             * Propose and approve a spend of treasury funds.
             *
             * ## Dispatch Origin
             *
             * Must be [`Config::SpendOrigin`] with the `Success` value being at least `amount`.
             *
             * ### Details
             *
             * NOTE: For record-keeping purposes, the proposer is deemed to be equivalent to the beneficiary.
             *
             * ### Parameters
             *
             * - `amount`: The amount to be transferred from the treasury to the `beneficiary`.
             * - `beneficiary`: The destination account for the transfer.
             *
             * ## Events
             *
             * Emits [`Event::SpendApproved`] if successful.
             */
            spendLocal: AugmentedSubmittable<
                (
                    amount: Compact<u128> | AnyNumber | Uint8Array,
                    beneficiary:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u128>, MultiAddress]
            >;
            /**
             * Void previously approved spend.
             *
             * ## Dispatch Origin
             *
             * Must be [`Config::RejectOrigin`].
             *
             * ## Details
             *
             * A spend void is only possible if the payout has not been attempted yet.
             *
             * ### Parameters
             *
             * - `index`: The spend index.
             *
             * ## Events
             *
             * Emits [`Event::AssetSpendVoided`] if successful.
             */
            voidSpend: AugmentedSubmittable<
                (index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        txPause: {
            /**
             * Pause a call.
             *
             * Can only be called by [`Config::PauseOrigin`]. Emits an [`Event::CallPaused`] event on success.
             */
            pause: AugmentedSubmittable<
                (
                    fullName: ITuple<[Bytes, Bytes]> | [Bytes | string | Uint8Array, Bytes | string | Uint8Array]
                ) => SubmittableExtrinsic<ApiType>,
                [ITuple<[Bytes, Bytes]>]
            >;
            /**
             * Un-pause a call.
             *
             * Can only be called by [`Config::UnpauseOrigin`]. Emits an [`Event::CallUnpaused`] event on success.
             */
            unpause: AugmentedSubmittable<
                (
                    ident: ITuple<[Bytes, Bytes]> | [Bytes | string | Uint8Array, Bytes | string | Uint8Array]
                ) => SubmittableExtrinsic<ApiType>,
                [ITuple<[Bytes, Bytes]>]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        utility: {
            /**
             * Send a call through an indexed pseudonym of the sender.
             *
             * Filter from origin are passed along. The call will be dispatched with an origin which use the same filter as
             * the origin of this call.
             *
             * NOTE: If you need to ensure that any account-based filtering is not honored (i.e. because you expect `proxy` to
             * have been used prior in the call stack and you do not want the call restrictions to apply to any sub-accounts),
             * then use `as_multi_threshold_1` in the Multisig pallet instead.
             *
             * NOTE: Prior to version *12, this was called `as_limited_sub`.
             *
             * The dispatch origin for this call must be _Signed_.
             */
            asDerivative: AugmentedSubmittable<
                (
                    index: u16 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, Call]
            >;
            /**
             * Send a batch of dispatch calls.
             *
             * May be called from any origin except `None`.
             *
             * - `calls`: The calls to be dispatched from the same origin. The number of call must not exceed the constant:
             *   `batched_calls_limit` (available in constant metadata).
             *
             * If origin is root then the calls are dispatched without checking origin filter. (This includes bypassing
             * `frame_system::Config::BaseCallFilter`).
             *
             * ## Complexity
             *
             * - O(C) where C is the number of calls to be batched.
             *
             * This will return `Ok` in all circumstances. To determine the success of the batch, an event is deposited. If a
             * call failed and the batch was interrupted, then the `BatchInterrupted` event is deposited, along with the
             * number of successful calls made and the error of the failed call. If all were successful, then the
             * `BatchCompleted` event is deposited.
             */
            batch: AugmentedSubmittable<
                (calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<Call>]
            >;
            /**
             * Send a batch of dispatch calls and atomically execute them. The whole transaction will rollback and fail if any
             * of the calls failed.
             *
             * May be called from any origin except `None`.
             *
             * - `calls`: The calls to be dispatched from the same origin. The number of call must not exceed the constant:
             *   `batched_calls_limit` (available in constant metadata).
             *
             * If origin is root then the calls are dispatched without checking origin filter. (This includes bypassing
             * `frame_system::Config::BaseCallFilter`).
             *
             * ## Complexity
             *
             * - O(C) where C is the number of calls to be batched.
             */
            batchAll: AugmentedSubmittable<
                (calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<Call>]
            >;
            /**
             * Dispatches a function call with a provided origin.
             *
             * The dispatch origin for this call must be _Root_.
             *
             * ## Complexity
             *
             * - O(1).
             */
            dispatchAs: AugmentedSubmittable<
                (
                    asOrigin:
                        | DanceboxRuntimeOriginCaller
                        | { system: any }
                        | { Void: any }
                        | { CumulusXcm: any }
                        | { PolkadotXcm: any }
                        | string
                        | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [DanceboxRuntimeOriginCaller, Call]
            >;
            /**
             * Send a batch of dispatch calls. Unlike `batch`, it allows errors and won't interrupt.
             *
             * May be called from any origin except `None`.
             *
             * - `calls`: The calls to be dispatched from the same origin. The number of call must not exceed the constant:
             *   `batched_calls_limit` (available in constant metadata).
             *
             * If origin is root then the calls are dispatch without checking origin filter. (This includes bypassing
             * `frame_system::Config::BaseCallFilter`).
             *
             * ## Complexity
             *
             * - O(C) where C is the number of calls to be batched.
             */
            forceBatch: AugmentedSubmittable<
                (calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<Call>]
            >;
            /**
             * Dispatch a function call with a specified weight.
             *
             * This function does not check the weight of the call, and instead allows the Root origin to specify the weight
             * of the call.
             *
             * The dispatch origin for this call must be _Root_.
             */
            withWeight: AugmentedSubmittable<
                (
                    call: Call | IMethod | string | Uint8Array,
                    weight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Call, SpWeightsWeightV2Weight]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        xcmCoreBuyer: {
            /**
             * Buy a core for this parathread id. Collators should call this to indicate that they intend to produce a block,
             * but they cannot do it because this para id has no available cores. The purchase is automatic using XCM, and
             * collators do not need to do anything.
             */
            buyCore: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    proof:
                        | TpXcmCoreBuyerBuyCoreCollatorProof
                        | { nonce?: any; publicKey?: any; signature?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, TpXcmCoreBuyerBuyCoreCollatorProof]
            >;
            cleanUpExpiredInFlightOrders: AugmentedSubmittable<
                (expiredInFlightOrders: Vec<u32> | (u32 | AnyNumber | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<u32>]
            >;
            cleanUpExpiredPendingBlocks: AugmentedSubmittable<
                (
                    expiredPendingBlocksParaId: Vec<u32> | (u32 | AnyNumber | Uint8Array)[]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<u32>]
            >;
            /** Buy core for para id as root. Does not require any proof, useful in tests. */
            forceBuyCore: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            queryResponse: AugmentedSubmittable<
                (
                    queryId: u64 | AnyNumber | Uint8Array,
                    response:
                        | StagingXcmV4Response
                        | { Null: any }
                        | { Assets: any }
                        | { ExecutionResult: any }
                        | { Version: any }
                        | { PalletsInfo: any }
                        | { DispatchResult: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, StagingXcmV4Response]
            >;
            setRelayChain: AugmentedSubmittable<
                (
                    relayChain:
                        | Option<DanceboxRuntimeXcmConfigRelayChain>
                        | null
                        | Uint8Array
                        | DanceboxRuntimeXcmConfigRelayChain
                        | "Westend"
                        | "Rococo"
                        | number
                ) => SubmittableExtrinsic<ApiType>,
                [Option<DanceboxRuntimeXcmConfigRelayChain>]
            >;
            setRelayXcmWeightConfig: AugmentedSubmittable<
                (
                    xcmWeights:
                        | Option<PalletXcmCoreBuyerRelayXcmWeightConfigInner>
                        | null
                        | Uint8Array
                        | PalletXcmCoreBuyerRelayXcmWeightConfigInner
                        | { buyExecutionCost?: any; weightAtMost?: any }
                        | string
                ) => SubmittableExtrinsic<ApiType>,
                [Option<PalletXcmCoreBuyerRelayXcmWeightConfigInner>]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        xcmpQueue: {
            /**
             * Resumes all XCM executions for the XCMP queue.
             *
             * Note that this function doesn't change the status of the in/out bound channels.
             *
             * - `origin`: Must pass `ControllerOrigin`.
             */
            resumeXcmExecution: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Suspends all XCM executions for the XCMP queue, regardless of the sender's origin.
             *
             * - `origin`: Must pass `ControllerOrigin`.
             */
            suspendXcmExecution: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Overwrites the number of pages which must be in the queue after which we drop any further messages from the
             * channel.
             *
             * - `origin`: Must pass `Root`.
             * - `new`: Desired value for `QueueConfigData.drop_threshold`
             */
            updateDropThreshold: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Overwrites the number of pages which the queue must be reduced to before it signals that message sending may
             * recommence after it has been suspended.
             *
             * - `origin`: Must pass `Root`.
             * - `new`: Desired value for `QueueConfigData.resume_threshold`
             */
            updateResumeThreshold: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Overwrites the number of pages which must be in the queue for the other side to be told to suspend their
             * sending.
             *
             * - `origin`: Must pass `Root`.
             * - `new`: Desired value for `QueueConfigData.suspend_value`
             */
            updateSuspendThreshold: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
    } // AugmentedSubmittables
} // declare module
