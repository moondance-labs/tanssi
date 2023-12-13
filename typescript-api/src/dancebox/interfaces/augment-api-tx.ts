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
import type { Bytes, Compact, Option, Vec, bool, u128, u16, u32, u64 } from "@polkadot/types-codec";
import type { AnyNumber, IMethod, ITuple } from "@polkadot/types-codec/types";
import type { AccountId32, Call, H256, MultiAddress, Perbill } from "@polkadot/types/interfaces/runtime";
import type {
    CumulusPrimitivesParachainInherentParachainInherentData,
    DanceboxRuntimeOriginCaller,
    DanceboxRuntimeProxyType,
    DanceboxRuntimeSessionKeys,
    PalletPooledStakingAllTargetPool,
    PalletPooledStakingPendingOperationQuery,
    PalletPooledStakingSharesOrStake,
    PalletPooledStakingTargetPool,
    SpWeightsWeightV2Weight,
    StagingXcmV3MultiLocation,
    TpAuthorNotingInherentOwnParachainInherentData,
    TpContainerChainGenesisDataContainerChainGenesisData,
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
                    author: AccountId32 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32, AccountId32]
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
            /** See [`Pallet::service_overweight`]. */
            serviceOverweight: AugmentedSubmittable<
                (
                    index: u64 | AnyNumber | Uint8Array,
                    weightLimit: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, SpWeightsWeightV2Weight]
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
            /** See [`Pallet::set_invulnerables`]. */
            setInvulnerables: AugmentedSubmittable<
                (updated: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>]
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
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        servicesPayment: {
            /** See [`Pallet::purchase_credits`]. */
            purchaseCredits: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    credits: u32 | AnyNumber | Uint8Array,
                    maxPricePerCredit: Option<u128> | null | Uint8Array | u128 | AnyNumber
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32, Option<u128>]
            >;
            /** See [`Pallet::set_credits`]. */
            setCredits: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    credits: u32 | AnyNumber | Uint8Array
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
        sudo: {
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
        xcmpQueue: {
            /** See [`Pallet::resume_xcm_execution`]. */
            resumeXcmExecution: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /** See [`Pallet::service_overweight`]. */
            serviceOverweight: AugmentedSubmittable<
                (
                    index: u64 | AnyNumber | Uint8Array,
                    weightLimit: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, SpWeightsWeightV2Weight]
            >;
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
            /** See [`Pallet::update_threshold_weight`]. */
            updateThresholdWeight: AugmentedSubmittable<
                (
                    updated: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpWeightsWeightV2Weight]
            >;
            /** See [`Pallet::update_weight_restrict_decay`]. */
            updateWeightRestrictDecay: AugmentedSubmittable<
                (
                    updated: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpWeightsWeightV2Weight]
            >;
            /** See [`Pallet::update_xcmp_max_individual_weight`]. */
            updateXcmpMaxIndividualWeight: AugmentedSubmittable<
                (
                    updated: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpWeightsWeightV2Weight]
            >;
            /** Generic tx */
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
    } // AugmentedSubmittables
} // declare module
