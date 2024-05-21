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
    DanceboxRuntimeProxyType,
    DanceboxRuntimeSessionKeys,
    DanceboxRuntimeStreamPaymentAssetId,
    DanceboxRuntimeXcmConfigRelayChain,
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
    PalletXcmCoreBuyerBuyCoreCollatorProof,
    PalletXcmCoreBuyerRelayXcmWeightConfigInner,
    SpRuntimeMultiSignature,
    SpWeightsWeightV2Weight,
    StagingXcmV3MultiLocation,
    TpAuthorNotingInherentOwnParachainInherentData,
    TpContainerChainGenesisDataContainerChainGenesisData,
    TpTraitsSlotFrequency,
    XcmV3Response,
    XcmV3WeightLimit,
    XcmVersionedMultiAssets,
    XcmVersionedMultiLocation,
    XcmVersionedXcm,
} from "@polkadot/types/lookup";

export type __AugmentedSubmittable = AugmentedSubmittable<() => unknown>;
export type __SubmittableExtrinsic<ApiType extends ApiTypes> = SubmittableExtrinsic<ApiType>;
export type __SubmittableExtrinsicFunction<ApiType extends ApiTypes> = SubmittableExtrinsicFunction<ApiType>;

declare module "@polkadot/api-base/types/submittable" {
    interface AugmentedSubmittables<ApiType extends ApiTypes> {
        assetRate: {
            /** See [`Pallet::create`]. */
            create: AugmentedSubmittable<
                (
                    assetKind: u16 | AnyNumber | Uint8Array,
                    rate: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, u128]
            >;
            /** See [`Pallet::remove`]. */
            remove: AugmentedSubmittable<
                (assetKind: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /** See [`Pallet::update`]. */
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
            /** See [`Pallet::kick_off_authorship_validation`]. */
            kickOffAuthorshipValidation: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        authorityAssignment: {
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        authorNoting: {
            /** See [`Pallet::kill_author_data`]. */
            killAuthorData: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::set_author`]. */
            setAuthor: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    blockNumber: u32 | AnyNumber | Uint8Array,
                    author: AccountId32 | string | Uint8Array,
                    latestSlotNumber: u64 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32, AccountId32, u64]
            >;
            /** See [`Pallet::set_latest_author_data`]. */
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
            /** See [`Pallet::force_set_balance`]. */
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
            /** See [`Pallet::force_transfer`]. */
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
            /** See [`Pallet::force_unreserve`]. */
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
            /** See [`Pallet::transfer_all`]. */
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
            /** See [`Pallet::transfer_allow_death`]. */
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
            /** See [`Pallet::transfer_keep_alive`]. */
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
            /** See [`Pallet::upgrade_accounts`]. */
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
            /** See [`Pallet::set_bypass_consistency_check`]. */
            setBypassConsistencyCheck: AugmentedSubmittable<
                (updated: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [bool]
            >;
            /** See [`Pallet::set_collators_per_container`]. */
            setCollatorsPerContainer: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::set_collators_per_parathread`]. */
            setCollatorsPerParathread: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::set_full_rotation_period`]. */
            setFullRotationPeriod: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::set_max_collators`]. */
            setMaxCollators: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::set_max_orchestrator_collators`]. */
            setMaxOrchestratorCollators: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::set_min_orchestrator_collators`]. */
            setMinOrchestratorCollators: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::set_parathreads_per_collator`]. */
            setParathreadsPerCollator: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::set_target_container_chain_fullness`]. */
            setTargetContainerChainFullness: AugmentedSubmittable<
                (updated: Perbill | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Perbill]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        dataPreservers: {
            /** See [`Pallet::set_boot_nodes`]. */
            setBootNodes: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    bootNodes: Vec<Bytes> | (Bytes | string | Uint8Array)[]
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Vec<Bytes>]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        dmpQueue: {
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        foreignAssets: {
            /** See [`Pallet::approve_transfer`]. */
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
            /** See [`Pallet::block`]. */
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
            /** See [`Pallet::burn`]. */
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
            /** See [`Pallet::cancel_approval`]. */
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
            /** See [`Pallet::clear_metadata`]. */
            clearMetadata: AugmentedSubmittable<
                (id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /** See [`Pallet::create`]. */
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
            /** See [`Pallet::destroy_accounts`]. */
            destroyAccounts: AugmentedSubmittable<
                (id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /** See [`Pallet::destroy_approvals`]. */
            destroyApprovals: AugmentedSubmittable<
                (id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /** See [`Pallet::finish_destroy`]. */
            finishDestroy: AugmentedSubmittable<
                (id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /** See [`Pallet::force_asset_status`]. */
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
            /** See [`Pallet::force_cancel_approval`]. */
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
            /** See [`Pallet::force_clear_metadata`]. */
            forceClearMetadata: AugmentedSubmittable<
                (id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /** See [`Pallet::force_create`]. */
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
            /** See [`Pallet::force_set_metadata`]. */
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
            /** See [`Pallet::force_transfer`]. */
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
            /** See [`Pallet::freeze`]. */
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
            /** See [`Pallet::freeze_asset`]. */
            freezeAsset: AugmentedSubmittable<
                (id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /** See [`Pallet::mint`]. */
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
            /** See [`Pallet::refund`]. */
            refund: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    allowBurn: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, bool]
            >;
            /** See [`Pallet::refund_other`]. */
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
            /** See [`Pallet::set_metadata`]. */
            setMetadata: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    name: Bytes | string | Uint8Array,
                    symbol: Bytes | string | Uint8Array,
                    decimals: u8 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, Bytes, Bytes, u8]
            >;
            /** See [`Pallet::set_min_balance`]. */
            setMinBalance: AugmentedSubmittable<
                (
                    id: u16 | AnyNumber | Uint8Array,
                    minBalance: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, u128]
            >;
            /** See [`Pallet::set_team`]. */
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
            /** See [`Pallet::start_destroy`]. */
            startDestroy: AugmentedSubmittable<
                (id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /** See [`Pallet::thaw`]. */
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
            /** See [`Pallet::thaw_asset`]. */
            thawAsset: AugmentedSubmittable<(id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u16]>;
            /** See [`Pallet::touch`]. */
            touch: AugmentedSubmittable<(id: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u16]>;
            /** See [`Pallet::touch_other`]. */
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
            /** See [`Pallet::transfer`]. */
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
            /** See [`Pallet::transfer_approved`]. */
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
            /** See [`Pallet::transfer_keep_alive`]. */
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
            /** See [`Pallet::transfer_ownership`]. */
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
            /** See [`Pallet::change_existing_asset_type`]. */
            changeExistingAssetType: AugmentedSubmittable<
                (
                    assetId: u16 | AnyNumber | Uint8Array,
                    newForeignAsset: StagingXcmV3MultiLocation | { parents?: any; interior?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, StagingXcmV3MultiLocation]
            >;
            /** See [`Pallet::create_foreign_asset`]. */
            createForeignAsset: AugmentedSubmittable<
                (
                    foreignAsset: StagingXcmV3MultiLocation | { parents?: any; interior?: any } | string | Uint8Array,
                    assetId: u16 | AnyNumber | Uint8Array,
                    admin: AccountId32 | string | Uint8Array,
                    isSufficient: bool | boolean | Uint8Array,
                    minBalance: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [StagingXcmV3MultiLocation, u16, AccountId32, bool, u128]
            >;
            /** See [`Pallet::destroy_foreign_asset`]. */
            destroyForeignAsset: AugmentedSubmittable<
                (assetId: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /** See [`Pallet::remove_existing_asset_type`]. */
            removeExistingAssetType: AugmentedSubmittable<
                (assetId: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        identity: {
            /** See [`Pallet::accept_username`]. */
            acceptUsername: AugmentedSubmittable<
                (username: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /** See [`Pallet::add_registrar`]. */
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
            /** See [`Pallet::add_sub`]. */
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
            /** See [`Pallet::add_username_authority`]. */
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
            /** See [`Pallet::cancel_request`]. */
            cancelRequest: AugmentedSubmittable<
                (regIndex: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::clear_identity`]. */
            clearIdentity: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** See [`Pallet::kill_identity`]. */
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
            /** See [`Pallet::provide_judgement`]. */
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
            /** See [`Pallet::quit_sub`]. */
            quitSub: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** See [`Pallet::remove_dangling_username`]. */
            removeDanglingUsername: AugmentedSubmittable<
                (username: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /** See [`Pallet::remove_expired_approval`]. */
            removeExpiredApproval: AugmentedSubmittable<
                (username: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /** See [`Pallet::remove_sub`]. */
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
            /** See [`Pallet::remove_username_authority`]. */
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
            /** See [`Pallet::rename_sub`]. */
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
            /** See [`Pallet::request_judgement`]. */
            requestJudgement: AugmentedSubmittable<
                (
                    regIndex: Compact<u32> | AnyNumber | Uint8Array,
                    maxFee: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, Compact<u128>]
            >;
            /** See [`Pallet::set_account_id`]. */
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
            /** See [`Pallet::set_fee`]. */
            setFee: AugmentedSubmittable<
                (
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    fee: Compact<u128> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, Compact<u128>]
            >;
            /** See [`Pallet::set_fields`]. */
            setFields: AugmentedSubmittable<
                (
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    fields: u64 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, u64]
            >;
            /** See [`Pallet::set_identity`]. */
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
            /** See [`Pallet::set_primary_username`]. */
            setPrimaryUsername: AugmentedSubmittable<
                (username: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /** See [`Pallet::set_subs`]. */
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
                              )
                          ][]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<ITuple<[AccountId32, Data]>>]
            >;
            /** See [`Pallet::set_username_for`]. */
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
            /** See [`Pallet::add_invulnerable`]. */
            addInvulnerable: AugmentedSubmittable<
                (who: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /** See [`Pallet::remove_invulnerable`]. */
            removeInvulnerable: AugmentedSubmittable<
                (who: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        maintenanceMode: {
            /** See [`Pallet::enter_maintenance_mode`]. */
            enterMaintenanceMode: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** See [`Pallet::resume_normal_operation`]. */
            resumeNormalOperation: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        messageQueue: {
            /** See [`Pallet::execute_overweight`]. */
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
            /** See [`Pallet::reap_page`]. */
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
            /** See [`Pallet::approve_as_multi`]. */
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
            /** See [`Pallet::as_multi`]. */
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
            /** See [`Pallet::as_multi_threshold_1`]. */
            asMultiThreshold1: AugmentedSubmittable<
                (
                    otherSignatories: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>, Call]
            >;
            /** See [`Pallet::cancel_as_multi`]. */
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
            /** See [`Pallet::authorize_upgrade`]. */
            authorizeUpgrade: AugmentedSubmittable<
                (
                    codeHash: H256 | string | Uint8Array,
                    checkVersion: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, bool]
            >;
            /** See [`Pallet::enact_authorized_upgrade`]. */
            enactAuthorizedUpgrade: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /** See [`Pallet::set_validation_data`]. */
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
            /** See [`Pallet::sudo_send_upward_message`]. */
            sudoSendUpwardMessage: AugmentedSubmittable<
                (message: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        polkadotXcm: {
            /** See [`Pallet::execute`]. */
            execute: AugmentedSubmittable<
                (
                    message: XcmVersionedXcm | { V2: any } | { V3: any } | string | Uint8Array,
                    maxWeight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedXcm, SpWeightsWeightV2Weight]
            >;
            /** See [`Pallet::force_default_xcm_version`]. */
            forceDefaultXcmVersion: AugmentedSubmittable<
                (maybeXcmVersion: Option<u32> | null | Uint8Array | u32 | AnyNumber) => SubmittableExtrinsic<ApiType>,
                [Option<u32>]
            >;
            /** See [`Pallet::force_subscribe_version_notify`]. */
            forceSubscribeVersionNotify: AugmentedSubmittable<
                (
                    location: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedMultiLocation]
            >;
            /** See [`Pallet::force_suspension`]. */
            forceSuspension: AugmentedSubmittable<
                (suspended: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [bool]
            >;
            /** See [`Pallet::force_unsubscribe_version_notify`]. */
            forceUnsubscribeVersionNotify: AugmentedSubmittable<
                (
                    location: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedMultiLocation]
            >;
            /** See [`Pallet::force_xcm_version`]. */
            forceXcmVersion: AugmentedSubmittable<
                (
                    location: StagingXcmV3MultiLocation | { parents?: any; interior?: any } | string | Uint8Array,
                    version: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [StagingXcmV3MultiLocation, u32]
            >;
            /** See [`Pallet::limited_reserve_transfer_assets`]. */
            limitedReserveTransferAssets: AugmentedSubmittable<
                (
                    dest: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array,
                    beneficiary: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array,
                    assets: XcmVersionedMultiAssets | { V2: any } | { V3: any } | string | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array,
                    weightLimit: XcmV3WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32, XcmV3WeightLimit]
            >;
            /** See [`Pallet::limited_teleport_assets`]. */
            limitedTeleportAssets: AugmentedSubmittable<
                (
                    dest: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array,
                    beneficiary: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array,
                    assets: XcmVersionedMultiAssets | { V2: any } | { V3: any } | string | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array,
                    weightLimit: XcmV3WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32, XcmV3WeightLimit]
            >;
            /** See [`Pallet::reserve_transfer_assets`]. */
            reserveTransferAssets: AugmentedSubmittable<
                (
                    dest: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array,
                    beneficiary: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array,
                    assets: XcmVersionedMultiAssets | { V2: any } | { V3: any } | string | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32]
            >;
            /** See [`Pallet::send`]. */
            send: AugmentedSubmittable<
                (
                    dest: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array,
                    message: XcmVersionedXcm | { V2: any } | { V3: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedMultiLocation, XcmVersionedXcm]
            >;
            /** See [`Pallet::teleport_assets`]. */
            teleportAssets: AugmentedSubmittable<
                (
                    dest: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array,
                    beneficiary: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array,
                    assets: XcmVersionedMultiAssets | { V2: any } | { V3: any } | string | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32]
            >;
            /** See [`Pallet::transfer_assets`]. */
            transferAssets: AugmentedSubmittable<
                (
                    dest: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array,
                    beneficiary: XcmVersionedMultiLocation | { V2: any } | { V3: any } | string | Uint8Array,
                    assets: XcmVersionedMultiAssets | { V2: any } | { V3: any } | string | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array,
                    weightLimit: XcmV3WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedMultiLocation, XcmVersionedMultiLocation, XcmVersionedMultiAssets, u32, XcmV3WeightLimit]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        pooledStaking: {
            /** See [`Pallet::claim_manual_rewards`]. */
            claimManualRewards: AugmentedSubmittable<
                (
                    pairs:
                        | Vec<ITuple<[AccountId32, AccountId32]>>
                        | [AccountId32 | string | Uint8Array, AccountId32 | string | Uint8Array][]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<ITuple<[AccountId32, AccountId32]>>]
            >;
            /** See [`Pallet::execute_pending_operations`]. */
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
            /** See [`Pallet::rebalance_hold`]. */
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
            /** See [`Pallet::request_delegate`]. */
            requestDelegate: AugmentedSubmittable<
                (
                    candidate: AccountId32 | string | Uint8Array,
                    pool: PalletPooledStakingTargetPool | "AutoCompounding" | "ManualRewards" | number | Uint8Array,
                    stake: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, PalletPooledStakingTargetPool, u128]
            >;
            /** See [`Pallet::request_undelegate`]. */
            requestUndelegate: AugmentedSubmittable<
                (
                    candidate: AccountId32 | string | Uint8Array,
                    pool: PalletPooledStakingTargetPool | "AutoCompounding" | "ManualRewards" | number | Uint8Array,
                    amount: PalletPooledStakingSharesOrStake | { Shares: any } | { Stake: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, PalletPooledStakingTargetPool, PalletPooledStakingSharesOrStake]
            >;
            /** See [`Pallet::swap_pool`]. */
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
            /** See [`Pallet::update_candidate_position`]. */
            updateCandidatePosition: AugmentedSubmittable<
                (candidates: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        proxy: {
            /** See [`Pallet::add_proxy`]. */
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
            /** See [`Pallet::announce`]. */
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
            /** See [`Pallet::create_pure`]. */
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
            /** See [`Pallet::kill_pure`]. */
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
            /** See [`Pallet::proxy`]. */
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
            /** See [`Pallet::proxy_announced`]. */
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
            /** See [`Pallet::reject_announcement`]. */
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
            /** See [`Pallet::remove_announcement`]. */
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
            /** See [`Pallet::remove_proxies`]. */
            removeProxies: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** See [`Pallet::remove_proxy`]. */
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
            /** See [`Pallet::deregister`]. */
            deregister: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::mark_valid_for_collating`]. */
            markValidForCollating: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::pause_container_chain`]. */
            pauseContainerChain: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::register`]. */
            register: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    genesisData:
                        | TpContainerChainGenesisDataContainerChainGenesisData
                        | { storage?: any; name?: any; id?: any; forkId?: any; extensions?: any; properties?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, TpContainerChainGenesisDataContainerChainGenesisData]
            >;
            /** See [`Pallet::register_parathread`]. */
            registerParathread: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    slotFrequency: TpTraitsSlotFrequency | { min?: any; max?: any } | string | Uint8Array,
                    genesisData:
                        | TpContainerChainGenesisDataContainerChainGenesisData
                        | { storage?: any; name?: any; id?: any; forkId?: any; extensions?: any; properties?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, TpTraitsSlotFrequency, TpContainerChainGenesisDataContainerChainGenesisData]
            >;
            /** See [`Pallet::set_parathread_params`]. */
            setParathreadParams: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    slotFrequency: TpTraitsSlotFrequency | { min?: any; max?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, TpTraitsSlotFrequency]
            >;
            /** See [`Pallet::unpause_container_chain`]. */
            unpauseContainerChain: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        rootTesting: {
            /** See `Pallet::fill_block`. */
            fillBlock: AugmentedSubmittable<
                (ratio: Perbill | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Perbill]
            >;
            /** See `Pallet::trigger_defensive`. */
            triggerDefensive: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        servicesPayment: {
            /** See [`Pallet::purchase_credits`]. */
            purchaseCredits: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    credit: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u128]
            >;
            /** See [`Pallet::set_block_production_credits`]. */
            setBlockProductionCredits: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    freeBlockCredits: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /** See [`Pallet::set_collator_assignment_credits`]. */
            setCollatorAssignmentCredits: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    freeCollatorAssignmentCredits: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /** See [`Pallet::set_given_free_credits`]. */
            setGivenFreeCredits: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    givenFreeCredits: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, bool]
            >;
            /** See [`Pallet::set_max_core_price`]. */
            setMaxCorePrice: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    maxCorePrice: Option<u128> | null | Uint8Array | u128 | AnyNumber
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<u128>]
            >;
            /** See [`Pallet::set_max_tip`]. */
            setMaxTip: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    maxTip: Option<u128> | null | Uint8Array | u128 | AnyNumber
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<u128>]
            >;
            /** See [`Pallet::set_refund_address`]. */
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
            /** See [`Pallet::purge_keys`]. */
            purgeKeys: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** See [`Pallet::set_keys`]. */
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
            /** See [`Pallet::accept_requested_change`]. */
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
            /** See [`Pallet::cancel_change_request`]. */
            cancelChangeRequest: AugmentedSubmittable<
                (streamId: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /** See [`Pallet::close_stream`]. */
            closeStream: AugmentedSubmittable<
                (streamId: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /** See [`Pallet::immediately_change_deposit`]. */
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
            /** See [`Pallet::open_stream`]. */
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
            /** See [`Pallet::perform_payment`]. */
            performPayment: AugmentedSubmittable<
                (streamId: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /** See [`Pallet::request_change`]. */
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
                    Option<PalletStreamPaymentDepositChange>
                ]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        sudo: {
            /** See [`Pallet::remove_key`]. */
            removeKey: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** See [`Pallet::set_key`]. */
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
            /** See [`Pallet::sudo`]. */
            sudo: AugmentedSubmittable<
                (call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Call]
            >;
            /** See [`Pallet::sudo_as`]. */
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
            /** See [`Pallet::sudo_unchecked_weight`]. */
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
            /** See [`Pallet::apply_authorized_upgrade`]. */
            applyAuthorizedUpgrade: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /** See [`Pallet::authorize_upgrade`]. */
            authorizeUpgrade: AugmentedSubmittable<
                (codeHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /** See [`Pallet::authorize_upgrade_without_checks`]. */
            authorizeUpgradeWithoutChecks: AugmentedSubmittable<
                (codeHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /** See [`Pallet::kill_prefix`]. */
            killPrefix: AugmentedSubmittable<
                (
                    prefix: Bytes | string | Uint8Array,
                    subkeys: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, u32]
            >;
            /** See [`Pallet::kill_storage`]. */
            killStorage: AugmentedSubmittable<
                (keys: Vec<Bytes> | (Bytes | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<Bytes>]
            >;
            /** See [`Pallet::remark`]. */
            remark: AugmentedSubmittable<
                (remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /** See [`Pallet::remark_with_event`]. */
            remarkWithEvent: AugmentedSubmittable<
                (remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /** See [`Pallet::set_code`]. */
            setCode: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /** See [`Pallet::set_code_without_checks`]. */
            setCodeWithoutChecks: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /** See [`Pallet::set_heap_pages`]. */
            setHeapPages: AugmentedSubmittable<
                (pages: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /** See [`Pallet::set_storage`]. */
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
            /** See [`Pallet::set`]. */
            set: AugmentedSubmittable<
                (now: Compact<u64> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u64>]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        treasury: {
            /** See [`Pallet::approve_proposal`]. */
            approveProposal: AugmentedSubmittable<
                (proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /** See [`Pallet::check_status`]. */
            checkStatus: AugmentedSubmittable<
                (index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::payout`]. */
            payout: AugmentedSubmittable<(index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
            /** See [`Pallet::propose_spend`]. */
            proposeSpend: AugmentedSubmittable<
                (
                    value: Compact<u128> | AnyNumber | Uint8Array,
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
            /** See [`Pallet::reject_proposal`]. */
            rejectProposal: AugmentedSubmittable<
                (proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /** See [`Pallet::remove_approval`]. */
            removeApproval: AugmentedSubmittable<
                (proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /** See [`Pallet::spend`]. */
            spend: AugmentedSubmittable<
                (
                    assetKind: Null | null,
                    amount: Compact<u128> | AnyNumber | Uint8Array,
                    beneficiary: AccountId32 | string | Uint8Array,
                    validFrom: Option<u32> | null | Uint8Array | u32 | AnyNumber
                ) => SubmittableExtrinsic<ApiType>,
                [Null, Compact<u128>, AccountId32, Option<u32>]
            >;
            /** See [`Pallet::spend_local`]. */
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
            /** See [`Pallet::void_spend`]. */
            voidSpend: AugmentedSubmittable<
                (index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        txPause: {
            /** See [`Pallet::pause`]. */
            pause: AugmentedSubmittable<
                (
                    fullName: ITuple<[Bytes, Bytes]> | [Bytes | string | Uint8Array, Bytes | string | Uint8Array]
                ) => SubmittableExtrinsic<ApiType>,
                [ITuple<[Bytes, Bytes]>]
            >;
            /** See [`Pallet::unpause`]. */
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
            /** See [`Pallet::as_derivative`]. */
            asDerivative: AugmentedSubmittable<
                (
                    index: u16 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, Call]
            >;
            /** See [`Pallet::batch`]. */
            batch: AugmentedSubmittable<
                (calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<Call>]
            >;
            /** See [`Pallet::batch_all`]. */
            batchAll: AugmentedSubmittable<
                (calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<Call>]
            >;
            /** See [`Pallet::dispatch_as`]. */
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
            /** See [`Pallet::force_batch`]. */
            forceBatch: AugmentedSubmittable<
                (calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<Call>]
            >;
            /** See [`Pallet::with_weight`]. */
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
            /** See [`Pallet::buy_core`]. */
            buyCore: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    proof:
                        | PalletXcmCoreBuyerBuyCoreCollatorProof
                        | { account?: any; signature?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, PalletXcmCoreBuyerBuyCoreCollatorProof]
            >;
            /** See [`Pallet::clean_up_expired_in_flight_orders`]. */
            cleanUpExpiredInFlightOrders: AugmentedSubmittable<
                (expiredInFlightOrders: Vec<u32> | (u32 | AnyNumber | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<u32>]
            >;
            /** See [`Pallet::clean_up_expired_pending_blocks`]. */
            cleanUpExpiredPendingBlocks: AugmentedSubmittable<
                (
                    expiredPendingBlocksParaId: Vec<u32> | (u32 | AnyNumber | Uint8Array)[]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<u32>]
            >;
            /** See [`Pallet::force_buy_core`]. */
            forceBuyCore: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::query_response`]. */
            queryResponse: AugmentedSubmittable<
                (
                    queryId: u64 | AnyNumber | Uint8Array,
                    response:
                        | XcmV3Response
                        | { Null: any }
                        | { Assets: any }
                        | { ExecutionResult: any }
                        | { Version: any }
                        | { PalletsInfo: any }
                        | { DispatchResult: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, XcmV3Response]
            >;
            /** See [`Pallet::set_relay_chain`]. */
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
            /** See [`Pallet::set_relay_xcm_weight_config`]. */
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
            /** See [`Pallet::resume_xcm_execution`]. */
            resumeXcmExecution: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** See [`Pallet::suspend_xcm_execution`]. */
            suspendXcmExecution: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** See [`Pallet::update_drop_threshold`]. */
            updateDropThreshold: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::update_resume_threshold`]. */
            updateResumeThreshold: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** See [`Pallet::update_suspend_threshold`]. */
            updateSuspendThreshold: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
    } // AugmentedSubmittables
} // declare module
