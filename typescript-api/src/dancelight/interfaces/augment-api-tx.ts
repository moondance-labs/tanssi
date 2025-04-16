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
import type {
    Bytes,
    Compact,
    Null,
    Option,
    U256,
    U8aFixed,
    Vec,
    bool,
    u128,
    u16,
    u32,
    u64,
    u8,
} from "@polkadot/types-codec";
import type { AnyNumber, IMethod, ITuple } from "@polkadot/types-codec/types";
import type { AccountId32, Call, H160, H256, MultiAddress, Perbill } from "@polkadot/types/interfaces/runtime";
import type {
    DancelightRuntimeAggregateMessageOrigin,
    DancelightRuntimeOriginCaller,
    DancelightRuntimeProxyType,
    DancelightRuntimeRuntimeParameters,
    DancelightRuntimeSessionKeys,
    DpContainerChainGenesisDataContainerChainGenesisData,
    FrameSupportPreimagesBounded,
    FrameSupportScheduleDispatchTime,
    PalletBalancesAdjustmentDirection,
    PalletConvictionVotingConviction,
    PalletConvictionVotingVoteAccountVote,
    PalletDataPreserversProfile,
    PalletExternalValidatorSlashesSlashingModeOption,
    PalletExternalValidatorsForcing,
    PalletIdentityJudgement,
    PalletIdentityLegacyIdentityInfo,
    PalletMigrationsHistoricCleanupSelector,
    PalletMigrationsMigrationCursor,
    PalletMultisigTimepoint,
    PalletPooledStakingPendingOperationQuery,
    PalletPooledStakingPoolsActivePoolKind,
    PalletPooledStakingPoolsPoolKind,
    PalletPooledStakingSharesOrStake,
    PalletStreamPaymentChangeKind,
    PalletStreamPaymentDepositChange,
    PalletStreamPaymentStreamConfig,
    PolkadotParachainPrimitivesPrimitivesHrmpChannelId,
    PolkadotPrimitivesV8ApprovalVotingParams,
    PolkadotPrimitivesV8AsyncBackingAsyncBackingParams,
    PolkadotPrimitivesV8ExecutorParams,
    PolkadotPrimitivesV8PvfCheckStatement,
    PolkadotPrimitivesV8SchedulerParams,
    PolkadotPrimitivesV8SlashingDisputeProof,
    PolkadotPrimitivesV8ValidatorAppSignature,
    PolkadotPrimitivesVstagingInherentData,
    PolkadotRuntimeParachainsParasParaGenesisArgs,
    SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate,
    SnowbridgeBeaconPrimitivesUpdatesUpdate,
    SnowbridgeCoreAssetMetadata,
    SnowbridgeCoreChannelId,
    SnowbridgeCoreInboundMessage,
    SnowbridgeCoreOperatingModeBasicOperatingMode,
    SnowbridgeCoreOutboundV1Initializer,
    SnowbridgeCoreOutboundV1OperatingMode,
    SnowbridgeCorePricingPricingParameters,
    SpConsensusBabeDigestsNextConfigDescriptor,
    SpConsensusBeefyDoubleVotingProof,
    SpConsensusBeefyForkVotingProof,
    SpConsensusBeefyFutureBlockVotingProof,
    SpConsensusGrandpaEquivocationProof,
    SpConsensusSlotsEquivocationProof,
    SpRuntimeMultiSignature,
    SpSessionMembershipProof,
    SpTrieStorageProof,
    SpWeightsWeightV2Weight,
    StagingXcmExecutorAssetTransferTransferType,
    StagingXcmV5Location,
    TpDataPreserversCommonAssignerExtra,
    TpDataPreserversCommonAssignmentWitness,
    TpStreamPaymentCommonAssetId,
    TpTraitsFullRotationMode,
    TpTraitsParathreadParams,
    TpTraitsSlotFrequency,
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
             * - O(1)
             **/
            create: AugmentedSubmittable<
                (assetKind: Null | null, rate: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Null, u128]
            >;
            /**
             * Remove an existing conversion rate to native balance for the given asset.
             *
             * ## Complexity
             * - O(1)
             **/
            remove: AugmentedSubmittable<(assetKind: Null | null) => SubmittableExtrinsic<ApiType>, [Null]>;
            /**
             * Update the conversion rate to native balance for the given asset.
             *
             * ## Complexity
             * - O(1)
             **/
            update: AugmentedSubmittable<
                (assetKind: Null | null, rate: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Null, u128]
            >;
            /**
             * Generic tx
             **/
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
            setLatestAuthorData: AugmentedSubmittable<(data: Null | null) => SubmittableExtrinsic<ApiType>, [Null]>;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        babe: {
            /**
             * Plan an epoch config change. The epoch config change is recorded and will be enacted on
             * the next call to `enact_epoch_change`. The config will be activated one epoch after.
             * Multiple calls to this method will replace any existing planned config change that had
             * not been enacted yet.
             **/
            planConfigChange: AugmentedSubmittable<
                (
                    config: SpConsensusBabeDigestsNextConfigDescriptor | { V1: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusBabeDigestsNextConfigDescriptor]
            >;
            /**
             * Report authority equivocation/misbehavior. This method will verify
             * the equivocation proof and validate the given key ownership proof
             * against the extracted offender. If both are valid, the offence will
             * be reported.
             **/
            reportEquivocation: AugmentedSubmittable<
                (
                    equivocationProof:
                        | SpConsensusSlotsEquivocationProof
                        | { offender?: any; slot?: any; firstHeader?: any; secondHeader?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof:
                        | SpSessionMembershipProof
                        | { session?: any; trieNodes?: any; validatorCount?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusSlotsEquivocationProof, SpSessionMembershipProof]
            >;
            /**
             * Report authority equivocation/misbehavior. This method will verify
             * the equivocation proof and validate the given key ownership proof
             * against the extracted offender. If both are valid, the offence will
             * be reported.
             * This extrinsic must be called unsigned and it is expected that only
             * block authors will call it (validated in `ValidateUnsigned`), as such
             * if the block author is defined it will be defined as the equivocation
             * reporter.
             **/
            reportEquivocationUnsigned: AugmentedSubmittable<
                (
                    equivocationProof:
                        | SpConsensusSlotsEquivocationProof
                        | { offender?: any; slot?: any; firstHeader?: any; secondHeader?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof:
                        | SpSessionMembershipProof
                        | { session?: any; trieNodes?: any; validatorCount?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusSlotsEquivocationProof, SpSessionMembershipProof]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        balances: {
            /**
             * Burn the specified liquid free balance from the origin account.
             *
             * If the origin's account ends up below the existential deposit as a result
             * of the burn and `keep_alive` is false, the account will be reaped.
             *
             * Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,
             * this `burn` operation will reduce total issuance by the amount _burned_.
             **/
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
             **/
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
             **/
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
            /**
             * Exactly as `transfer_allow_death`, except the origin must be root and the source account
             * may be specified.
             **/
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
             **/
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
             * NOTE: This function only attempts to transfer _transferable_ balances. This means that
             * any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be
             * transferred by this function. To ensure that this function results in a killed account,
             * you might need to prepare the account by removing any reference counters, storage
             * deposits, etc...
             *
             * The dispatch origin of this call must be Signed.
             *
             * - `dest`: The recipient of the transfer.
             * - `keep_alive`: A boolean to determine if the `transfer_all` operation should send all
             * of the funds the account has, causing the sender account to be killed (false), or
             * transfer everything except at least the existential deposit, which will guarantee to
             * keep the sender account alive (true).
             **/
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
             * `transfer_allow_death` will set the `FreeBalance` of the sender and receiver.
             * If the sender's account is below the existential deposit as a result
             * of the transfer, the account will be reaped.
             *
             * The dispatch origin for this call must be `Signed` by the transactor.
             **/
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
             * Same as the [`transfer_allow_death`] call, but with a check that the transfer will not
             * kill the origin account.
             *
             * 99% of the time you want [`transfer_allow_death`] instead.
             *
             * [`transfer_allow_death`]: struct.Pallet.html#method.transfer
             **/
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
             * This will waive the transaction fee if at least all but 10% of the accounts needed to
             * be upgraded. (We let some not have to be upgraded just in order to allow for the
             * possibility of churn).
             **/
            upgradeAccounts: AugmentedSubmittable<
                (who: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        beefy: {
            /**
             * Report voter equivocation/misbehavior. This method will verify the
             * equivocation proof and validate the given key ownership proof
             * against the extracted offender. If both are valid, the offence
             * will be reported.
             **/
            reportDoubleVoting: AugmentedSubmittable<
                (
                    equivocationProof:
                        | SpConsensusBeefyDoubleVotingProof
                        | { first?: any; second?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof:
                        | SpSessionMembershipProof
                        | { session?: any; trieNodes?: any; validatorCount?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusBeefyDoubleVotingProof, SpSessionMembershipProof]
            >;
            /**
             * Report voter equivocation/misbehavior. This method will verify the
             * equivocation proof and validate the given key ownership proof
             * against the extracted offender. If both are valid, the offence
             * will be reported.
             *
             * This extrinsic must be called unsigned and it is expected that only
             * block authors will call it (validated in `ValidateUnsigned`), as such
             * if the block author is defined it will be defined as the equivocation
             * reporter.
             **/
            reportDoubleVotingUnsigned: AugmentedSubmittable<
                (
                    equivocationProof:
                        | SpConsensusBeefyDoubleVotingProof
                        | { first?: any; second?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof:
                        | SpSessionMembershipProof
                        | { session?: any; trieNodes?: any; validatorCount?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusBeefyDoubleVotingProof, SpSessionMembershipProof]
            >;
            /**
             * Report fork voting equivocation. This method will verify the equivocation proof
             * and validate the given key ownership proof against the extracted offender.
             * If both are valid, the offence will be reported.
             **/
            reportForkVoting: AugmentedSubmittable<
                (
                    equivocationProof:
                        | SpConsensusBeefyForkVotingProof
                        | { vote?: any; ancestryProof?: any; header?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof:
                        | SpSessionMembershipProof
                        | { session?: any; trieNodes?: any; validatorCount?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusBeefyForkVotingProof, SpSessionMembershipProof]
            >;
            /**
             * Report fork voting equivocation. This method will verify the equivocation proof
             * and validate the given key ownership proof against the extracted offender.
             * If both are valid, the offence will be reported.
             *
             * This extrinsic must be called unsigned and it is expected that only
             * block authors will call it (validated in `ValidateUnsigned`), as such
             * if the block author is defined it will be defined as the equivocation
             * reporter.
             **/
            reportForkVotingUnsigned: AugmentedSubmittable<
                (
                    equivocationProof:
                        | SpConsensusBeefyForkVotingProof
                        | { vote?: any; ancestryProof?: any; header?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof:
                        | SpSessionMembershipProof
                        | { session?: any; trieNodes?: any; validatorCount?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusBeefyForkVotingProof, SpSessionMembershipProof]
            >;
            /**
             * Report future block voting equivocation. This method will verify the equivocation proof
             * and validate the given key ownership proof against the extracted offender.
             * If both are valid, the offence will be reported.
             **/
            reportFutureBlockVoting: AugmentedSubmittable<
                (
                    equivocationProof: SpConsensusBeefyFutureBlockVotingProof | { vote?: any } | string | Uint8Array,
                    keyOwnerProof:
                        | SpSessionMembershipProof
                        | { session?: any; trieNodes?: any; validatorCount?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusBeefyFutureBlockVotingProof, SpSessionMembershipProof]
            >;
            /**
             * Report future block voting equivocation. This method will verify the equivocation proof
             * and validate the given key ownership proof against the extracted offender.
             * If both are valid, the offence will be reported.
             *
             * This extrinsic must be called unsigned and it is expected that only
             * block authors will call it (validated in `ValidateUnsigned`), as such
             * if the block author is defined it will be defined as the equivocation
             * reporter.
             **/
            reportFutureBlockVotingUnsigned: AugmentedSubmittable<
                (
                    equivocationProof: SpConsensusBeefyFutureBlockVotingProof | { vote?: any } | string | Uint8Array,
                    keyOwnerProof:
                        | SpSessionMembershipProof
                        | { session?: any; trieNodes?: any; validatorCount?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusBeefyFutureBlockVotingProof, SpSessionMembershipProof]
            >;
            /**
             * Reset BEEFY consensus by setting a new BEEFY genesis at `delay_in_blocks` blocks in the
             * future.
             *
             * Note: `delay_in_blocks` has to be at least 1.
             **/
            setNewGenesis: AugmentedSubmittable<
                (delayInBlocks: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        collatorConfiguration: {
            /**
             * Setting this to true will disable consistency checks for the configuration setters.
             * Use with caution.
             **/
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
            setFullRotationMode: AugmentedSubmittable<
                (
                    orchestrator:
                        | Option<TpTraitsFullRotationMode>
                        | null
                        | Uint8Array
                        | TpTraitsFullRotationMode
                        | { RotateAll: any }
                        | { KeepAll: any }
                        | { KeepCollators: any }
                        | { KeepPerbill: any }
                        | string,
                    parachain:
                        | Option<TpTraitsFullRotationMode>
                        | null
                        | Uint8Array
                        | TpTraitsFullRotationMode
                        | { RotateAll: any }
                        | { KeepAll: any }
                        | { KeepCollators: any }
                        | { KeepPerbill: any }
                        | string,
                    parathread:
                        | Option<TpTraitsFullRotationMode>
                        | null
                        | Uint8Array
                        | TpTraitsFullRotationMode
                        | { RotateAll: any }
                        | { KeepAll: any }
                        | { KeepCollators: any }
                        | { KeepPerbill: any }
                        | string
                ) => SubmittableExtrinsic<ApiType>,
                [Option<TpTraitsFullRotationMode>, Option<TpTraitsFullRotationMode>, Option<TpTraitsFullRotationMode>]
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        configuration: {
            /**
             * Set approval-voting-params.
             **/
            setApprovalVotingParams: AugmentedSubmittable<
                (
                    updated:
                        | PolkadotPrimitivesV8ApprovalVotingParams
                        | { maxApprovalCoalesceCount?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PolkadotPrimitivesV8ApprovalVotingParams]
            >;
            /**
             * Set the asynchronous backing parameters.
             **/
            setAsyncBackingParams: AugmentedSubmittable<
                (
                    updated:
                        | PolkadotPrimitivesV8AsyncBackingAsyncBackingParams
                        | { maxCandidateDepth?: any; allowedAncestryLen?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PolkadotPrimitivesV8AsyncBackingAsyncBackingParams]
            >;
            /**
             * Setting this to true will disable consistency checks for the configuration setters.
             * Use with caution.
             **/
            setBypassConsistencyCheck: AugmentedSubmittable<
                (updated: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [bool]
            >;
            /**
             * Set the acceptance period for an included candidate.
             **/
            setCodeRetentionPeriod: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the number of coretime execution cores.
             *
             * NOTE: that this configuration is managed by the coretime chain. Only manually change
             * this, if you really know what you are doing!
             **/
            setCoretimeCores: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the dispute period, in number of sessions to keep for disputes.
             **/
            setDisputePeriod: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the dispute post conclusion acceptance period.
             **/
            setDisputePostConclusionAcceptancePeriod: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set PVF executor parameters.
             **/
            setExecutorParams: AugmentedSubmittable<
                (updated: PolkadotPrimitivesV8ExecutorParams) => SubmittableExtrinsic<ApiType>,
                [PolkadotPrimitivesV8ExecutorParams]
            >;
            /**
             * Set the parachain validator-group rotation frequency
             **/
            setGroupRotationFrequency: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Sets the maximum number of messages allowed in an HRMP channel at once.
             **/
            setHrmpChannelMaxCapacity: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Sets the maximum size of a message that could ever be put into an HRMP channel.
             **/
            setHrmpChannelMaxMessageSize: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Sets the maximum total size of messages in bytes allowed in an HRMP channel at once.
             **/
            setHrmpChannelMaxTotalSize: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Sets the maximum number of outbound HRMP messages can be sent by a candidate.
             **/
            setHrmpMaxMessageNumPerCandidate: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Sets the maximum number of inbound HRMP channels a parachain is allowed to accept.
             **/
            setHrmpMaxParachainInboundChannels: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Sets the maximum number of outbound HRMP channels a parachain is allowed to open.
             **/
            setHrmpMaxParachainOutboundChannels: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Sets the number of sessions after which an HRMP open channel request expires.
             **/
            setHrmpOpenRequestTtl: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Sets the amount of funds that the recipient should provide for accepting opening an HRMP
             * channel.
             **/
            setHrmpRecipientDeposit: AugmentedSubmittable<
                (updated: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u128]
            >;
            /**
             * Sets the amount of funds that the sender should provide for opening an HRMP channel.
             **/
            setHrmpSenderDeposit: AugmentedSubmittable<
                (updated: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u128]
            >;
            /**
             * Set the max validation code size for incoming upgrades.
             **/
            setMaxCodeSize: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the critical downward message size.
             **/
            setMaxDownwardMessageSize: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the max head data size for paras.
             **/
            setMaxHeadDataSize: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the max POV block size for incoming upgrades.
             **/
            setMaxPovSize: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Sets the maximum number of messages that a candidate can contain.
             **/
            setMaxUpwardMessageNumPerCandidate: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Sets the maximum size of an upward message that can be sent by a candidate.
             **/
            setMaxUpwardMessageSize: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Sets the maximum items that can present in a upward dispatch queue at once.
             **/
            setMaxUpwardQueueCount: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Sets the maximum total size of items that can present in a upward dispatch queue at
             * once.
             **/
            setMaxUpwardQueueSize: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the maximum number of validators to use in parachain consensus.
             **/
            setMaxValidators: AugmentedSubmittable<
                (updated: Option<u32> | null | Uint8Array | u32 | AnyNumber) => SubmittableExtrinsic<ApiType>,
                [Option<u32>]
            >;
            /**
             * Set the maximum number of validators to assign to any core.
             **/
            setMaxValidatorsPerCore: AugmentedSubmittable<
                (updated: Option<u32> | null | Uint8Array | u32 | AnyNumber) => SubmittableExtrinsic<ApiType>,
                [Option<u32>]
            >;
            /**
             * Set the minimum backing votes threshold.
             **/
            setMinimumBackingVotes: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Sets the minimum delay between announcing the upgrade block for a parachain until the
             * upgrade taking place.
             *
             * See the field documentation for information and constraints for the new value.
             **/
            setMinimumValidationUpgradeDelay: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the total number of delay tranches.
             **/
            setNDelayTranches: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the number of validators needed to approve a block.
             **/
            setNeededApprovals: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set/Unset a node feature.
             **/
            setNodeFeature: AugmentedSubmittable<
                (
                    index: u8 | AnyNumber | Uint8Array,
                    value: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u8, bool]
            >;
            /**
             * Set the no show slots, in number of number of consensus slots.
             * Must be at least 1.
             **/
            setNoShowSlots: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the on demand (parathreads) base fee.
             **/
            setOnDemandBaseFee: AugmentedSubmittable<
                (updated: u128 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u128]
            >;
            /**
             * Set the on demand (parathreads) fee variability.
             **/
            setOnDemandFeeVariability: AugmentedSubmittable<
                (updated: Perbill | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Perbill]
            >;
            /**
             * Set the on demand (parathreads) queue max size.
             **/
            setOnDemandQueueMaxSize: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the on demand (parathreads) fee variability.
             **/
            setOnDemandTargetQueueUtilization: AugmentedSubmittable<
                (updated: Perbill | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Perbill]
            >;
            /**
             * Set the availability period for paras.
             **/
            setParasAvailabilityPeriod: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the number of session changes after which a PVF pre-checking voting is rejected.
             **/
            setPvfVotingTtl: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the number of samples to do of the `RelayVRFModulo` approval assignment criterion.
             **/
            setRelayVrfModuloSamples: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set scheduler-params.
             **/
            setSchedulerParams: AugmentedSubmittable<
                (
                    updated:
                        | PolkadotPrimitivesV8SchedulerParams
                        | {
                              groupRotationFrequency?: any;
                              parasAvailabilityPeriod?: any;
                              maxValidatorsPerCore?: any;
                              lookahead?: any;
                              numCores?: any;
                              maxAvailabilityTimeouts?: any;
                              onDemandQueueMaxSize?: any;
                              onDemandTargetQueueUtilization?: any;
                              onDemandFeeVariability?: any;
                              onDemandBaseFee?: any;
                              ttl?: any;
                          }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PolkadotPrimitivesV8SchedulerParams]
            >;
            /**
             * Set the scheduling lookahead, in expected number of blocks at peak throughput.
             **/
            setSchedulingLookahead: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the validation upgrade cooldown.
             **/
            setValidationUpgradeCooldown: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the validation upgrade delay.
             **/
            setValidationUpgradeDelay: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set the zeroth delay tranche width.
             **/
            setZerothDelayTrancheWidth: AugmentedSubmittable<
                (updated: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        containerRegistrar: {
            /**
             * Deregister container-chain.
             *
             * If a container-chain is registered but not marked as valid_for_collating, this will remove it
             * from `PendingVerification` as well.
             **/
            deregister: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Deregister a parachain that no longer exists in the relay chain. The origin of this
             * extrinsic will be rewarded with the parachain deposit.
             **/
            deregisterWithRelayProof: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    relayProofBlockNumber: u32 | AnyNumber | Uint8Array,
                    relayStorageProof: SpTrieStorageProof | { trieNodes?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32, SpTrieStorageProof]
            >;
            /**
             * Mark container-chain valid for collating
             **/
            markValidForCollating: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Pause container-chain from collating. Does not remove its boot nodes nor its genesis config.
             * Only container-chains that have been marked as valid_for_collating can be paused.
             **/
            pauseContainerChain: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Register container-chain
             **/
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
            /**
             * Register parathread
             **/
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
            /**
             * Register parachain or parathread
             **/
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
            /**
             * Change parathread params
             **/
            setParathreadParams: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    slotFrequency: TpTraitsSlotFrequency | { min?: any; max?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, TpTraitsSlotFrequency]
            >;
            /**
             * Unpause container-chain.
             * Only container-chains that have been paused can be unpaused.
             **/
            unpauseContainerChain: AugmentedSubmittable<
                (paraId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        convictionVoting: {
            /**
             * Delegate the voting power (with some given conviction) of the sending account for a
             * particular class of polls.
             *
             * The balance delegated is locked for as long as it's delegated, and thereafter for the
             * time appropriate for the conviction's lock period.
             *
             * The dispatch origin of this call must be _Signed_, and the signing account must either:
             * - be delegating already; or
             * - have no voting activity (if there is, then it will need to be removed through
             * `remove_vote`).
             *
             * - `to`: The account whose voting the `target` account's voting power will follow.
             * - `class`: The class of polls to delegate. To delegate multiple classes, multiple calls
             * to this function are required.
             * - `conviction`: The conviction that will be attached to the delegated votes. When the
             * account is undelegated, the funds will be locked for the corresponding period.
             * - `balance`: The amount of the account's balance to be used in delegating. This must not
             * be more than the account's current balance.
             *
             * Emits `Delegated`.
             *
             * Weight: `O(R)` where R is the number of polls the voter delegating to has
             * voted on. Weight is initially charged as if maximum votes, but is refunded later.
             **/
            delegate: AugmentedSubmittable<
                (
                    clazz: u16 | AnyNumber | Uint8Array,
                    to:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    conviction:
                        | PalletConvictionVotingConviction
                        | "None"
                        | "Locked1x"
                        | "Locked2x"
                        | "Locked3x"
                        | "Locked4x"
                        | "Locked5x"
                        | "Locked6x"
                        | number
                        | Uint8Array,
                    balance: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, MultiAddress, PalletConvictionVotingConviction, u128]
            >;
            /**
             * Remove a vote for a poll.
             *
             * If the `target` is equal to the signer, then this function is exactly equivalent to
             * `remove_vote`. If not equal to the signer, then the vote must have expired,
             * either because the poll was cancelled, because the voter lost the poll or
             * because the conviction period is over.
             *
             * The dispatch origin of this call must be _Signed_.
             *
             * - `target`: The account of the vote to be removed; this account must have voted for poll
             * `index`.
             * - `index`: The index of poll of the vote to be removed.
             * - `class`: The class of the poll.
             *
             * Weight: `O(R + log R)` where R is the number of polls that `target` has voted on.
             * Weight is calculated for the maximum number of vote.
             **/
            removeOtherVote: AugmentedSubmittable<
                (
                    target:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    clazz: u16 | AnyNumber | Uint8Array,
                    index: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, u16, u32]
            >;
            /**
             * Remove a vote for a poll.
             *
             * If:
             * - the poll was cancelled, or
             * - the poll is ongoing, or
             * - the poll has ended such that
             * - the vote of the account was in opposition to the result; or
             * - there was no conviction to the account's vote; or
             * - the account made a split vote
             * ...then the vote is removed cleanly and a following call to `unlock` may result in more
             * funds being available.
             *
             * If, however, the poll has ended and:
             * - it finished corresponding to the vote of the account, and
             * - the account made a standard vote with conviction, and
             * - the lock period of the conviction is not over
             * ...then the lock will be aggregated into the overall account's lock, which may involve
             * *overlocking* (where the two locks are combined into a single lock that is the maximum
             * of both the amount locked and the time is it locked for).
             *
             * The dispatch origin of this call must be _Signed_, and the signer must have a vote
             * registered for poll `index`.
             *
             * - `index`: The index of poll of the vote to be removed.
             * - `class`: Optional parameter, if given it indicates the class of the poll. For polls
             * which have finished or are cancelled, this must be `Some`.
             *
             * Weight: `O(R + log R)` where R is the number of polls that `target` has voted on.
             * Weight is calculated for the maximum number of vote.
             **/
            removeVote: AugmentedSubmittable<
                (
                    clazz: Option<u16> | null | Uint8Array | u16 | AnyNumber,
                    index: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Option<u16>, u32]
            >;
            /**
             * Undelegate the voting power of the sending account for a particular class of polls.
             *
             * Tokens may be unlocked following once an amount of time consistent with the lock period
             * of the conviction with which the delegation was issued has passed.
             *
             * The dispatch origin of this call must be _Signed_ and the signing account must be
             * currently delegating.
             *
             * - `class`: The class of polls to remove the delegation from.
             *
             * Emits `Undelegated`.
             *
             * Weight: `O(R)` where R is the number of polls the voter delegating to has
             * voted on. Weight is initially charged as if maximum votes, but is refunded later.
             **/
            undelegate: AugmentedSubmittable<
                (clazz: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /**
             * Remove the lock caused by prior voting/delegating which has expired within a particular
             * class.
             *
             * The dispatch origin of this call must be _Signed_.
             *
             * - `class`: The class of polls to unlock.
             * - `target`: The account to remove the lock on.
             *
             * Weight: `O(R)` with R number of vote of target.
             **/
            unlock: AugmentedSubmittable<
                (
                    clazz: u16 | AnyNumber | Uint8Array,
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
                [u16, MultiAddress]
            >;
            /**
             * Vote in a poll. If `vote.is_aye()`, the vote is to enact the proposal;
             * otherwise it is a vote to keep the status quo.
             *
             * The dispatch origin of this call must be _Signed_.
             *
             * - `poll_index`: The index of the poll to vote for.
             * - `vote`: The vote configuration.
             *
             * Weight: `O(R)` where R is the number of polls the voter has voted on.
             **/
            vote: AugmentedSubmittable<
                (
                    pollIndex: Compact<u32> | AnyNumber | Uint8Array,
                    vote:
                        | PalletConvictionVotingVoteAccountVote
                        | { Standard: any }
                        | { Split: any }
                        | { SplitAbstain: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, PalletConvictionVotingVoteAccountVote]
            >;
            /**
             * Generic tx
             **/
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
                    assignmentWitness:
                        | TpDataPreserversCommonAssignmentWitness
                        | { Free: any }
                        | { StreamPayment: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, u32, TpDataPreserversCommonAssignmentWitness]
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
                    assignerParam:
                        | TpDataPreserversCommonAssignerExtra
                        | { Free: any }
                        | { StreamPayment: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, u32, TpDataPreserversCommonAssignerExtra]
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        ethereumBeaconClient: {
            /**
             * Used for pallet initialization and light client resetting. Needs to be called by
             * the root origin.
             **/
            forceCheckpoint: AugmentedSubmittable<
                (
                    update:
                        | SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate
                        | {
                              header?: any;
                              currentSyncCommittee?: any;
                              currentSyncCommitteeBranch?: any;
                              validatorsRoot?: any;
                              blockRootsRoot?: any;
                              blockRootsBranch?: any;
                          }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate]
            >;
            /**
             * Halt or resume all pallet operations. May only be called by root.
             **/
            setOperatingMode: AugmentedSubmittable<
                (
                    mode: SnowbridgeCoreOperatingModeBasicOperatingMode | "Normal" | "Halted" | number | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SnowbridgeCoreOperatingModeBasicOperatingMode]
            >;
            /**
             * Submits a new finalized beacon header update. The update may contain the next
             * sync committee.
             **/
            submit: AugmentedSubmittable<
                (
                    update:
                        | SnowbridgeBeaconPrimitivesUpdatesUpdate
                        | {
                              attestedHeader?: any;
                              syncAggregate?: any;
                              signatureSlot?: any;
                              nextSyncCommitteeUpdate?: any;
                              finalizedHeader?: any;
                              finalityBranch?: any;
                              blockRootsRoot?: any;
                              blockRootsBranch?: any;
                          }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SnowbridgeBeaconPrimitivesUpdatesUpdate]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        ethereumInboundQueue: {
            /**
             * Halt or resume all pallet operations. May only be called by root.
             **/
            setOperatingMode: AugmentedSubmittable<
                (
                    mode: SnowbridgeCoreOperatingModeBasicOperatingMode | "Normal" | "Halted" | number | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SnowbridgeCoreOperatingModeBasicOperatingMode]
            >;
            /**
             * Submit an inbound message originating from the Gateway contract on Ethereum
             **/
            submit: AugmentedSubmittable<
                (
                    message: SnowbridgeCoreInboundMessage | { eventLog?: any; proof?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SnowbridgeCoreInboundMessage]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        ethereumOutboundQueue: {
            /**
             * Halt or resume all pallet operations. May only be called by root.
             **/
            setOperatingMode: AugmentedSubmittable<
                (
                    mode: SnowbridgeCoreOperatingModeBasicOperatingMode | "Normal" | "Halted" | number | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SnowbridgeCoreOperatingModeBasicOperatingMode]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        ethereumSystem: {
            /**
             * Sends a command to the Gateway contract to instantiate a new agent contract representing
             * `origin`.
             *
             * Fee required: Yes
             *
             * - `origin`: Must be `Location` of a sibling parachain
             **/
            createAgent: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Sends a message to the Gateway contract to create a new channel representing `origin`
             *
             * Fee required: Yes
             *
             * This extrinsic is permissionless, so a fee is charged to prevent spamming and pay
             * for execution costs on the remote side.
             *
             * The message is sent over the bridge on BridgeHub's own channel to the Gateway.
             *
             * - `origin`: Must be `Location`
             * - `mode`: Initial operating mode of the channel
             **/
            createChannel: AugmentedSubmittable<
                (
                    mode:
                        | SnowbridgeCoreOutboundV1OperatingMode
                        | "Normal"
                        | "RejectingOutboundMessages"
                        | number
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SnowbridgeCoreOutboundV1OperatingMode]
            >;
            /**
             * Sends a message to the Gateway contract to transfer ether from an agent to `recipient`.
             *
             * Privileged. Can only be called by root.
             *
             * Fee required: No
             *
             * - `origin`: Must be root
             * - `location`: Location used to resolve the agent
             * - `recipient`: Recipient of funds
             * - `amount`: Amount to transfer
             **/
            forceTransferNativeFromAgent: AugmentedSubmittable<
                (
                    location: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    recipient: H160 | string | Uint8Array,
                    amount: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation, H160, u128]
            >;
            /**
             * Sends a message to the Gateway contract to update an arbitrary channel
             *
             * Fee required: No
             *
             * - `origin`: Must be root
             * - `channel_id`: ID of channel
             * - `mode`: Initial operating mode of the channel
             * - `outbound_fee`: Fee charged to users for sending outbound messages to Polkadot
             **/
            forceUpdateChannel: AugmentedSubmittable<
                (
                    channelId: SnowbridgeCoreChannelId | string | Uint8Array,
                    mode:
                        | SnowbridgeCoreOutboundV1OperatingMode
                        | "Normal"
                        | "RejectingOutboundMessages"
                        | number
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SnowbridgeCoreChannelId, SnowbridgeCoreOutboundV1OperatingMode]
            >;
            /**
             * Registers a Polkadot-native token as a wrapped ERC20 token on Ethereum.
             * Privileged. Can only be called by root.
             *
             * Fee required: No
             *
             * - `origin`: Must be root
             * - `location`: Location of the asset (relative to this chain)
             * - `metadata`: Metadata to include in the instantiated ERC20 contract on Ethereum
             **/
            registerToken: AugmentedSubmittable<
                (
                    location: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    metadata:
                        | SnowbridgeCoreAssetMetadata
                        | { name?: any; symbol?: any; decimals?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation, SnowbridgeCoreAssetMetadata]
            >;
            /**
             * Sends a message to the Gateway contract to change its operating mode
             *
             * Fee required: No
             *
             * - `origin`: Must be `Location`
             **/
            setOperatingMode: AugmentedSubmittable<
                (
                    mode:
                        | SnowbridgeCoreOutboundV1OperatingMode
                        | "Normal"
                        | "RejectingOutboundMessages"
                        | number
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SnowbridgeCoreOutboundV1OperatingMode]
            >;
            /**
             * Set pricing parameters on both sides of the bridge
             *
             * Fee required: No
             *
             * - `origin`: Must be root
             **/
            setPricingParameters: AugmentedSubmittable<
                (
                    params:
                        | SnowbridgeCorePricingPricingParameters
                        | { exchangeRate?: any; rewards?: any; feePerGas?: any; multiplier?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SnowbridgeCorePricingPricingParameters]
            >;
            /**
             * Sends a message to the Gateway contract to update fee related parameters for
             * token transfers.
             *
             * Privileged. Can only be called by root.
             *
             * Fee required: No
             *
             * - `origin`: Must be root
             * - `create_asset_xcm`: The XCM execution cost for creating a new asset class on AssetHub,
             * in DOT
             * - `transfer_asset_xcm`: The XCM execution cost for performing a reserve transfer on
             * AssetHub, in DOT
             * - `register_token`: The Ether fee for registering a new token, to discourage spamming
             **/
            setTokenTransferFees: AugmentedSubmittable<
                (
                    createAssetXcm: u128 | AnyNumber | Uint8Array,
                    transferAssetXcm: u128 | AnyNumber | Uint8Array,
                    registerToken: U256 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u128, u128, U256]
            >;
            /**
             * Sends a message to the Gateway contract to transfer ether from an agent to `recipient`.
             *
             * A partial fee will be charged for local processing only.
             *
             * - `origin`: Must be `Location`
             **/
            transferNativeFromAgent: AugmentedSubmittable<
                (
                    recipient: H160 | string | Uint8Array,
                    amount: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H160, u128]
            >;
            /**
             * Sends a message to the Gateway contract to update a channel configuration
             *
             * The origin must already have a channel initialized, as this message is sent over it.
             *
             * A partial fee will be charged for local processing only.
             *
             * - `origin`: Must be `Location`
             * - `mode`: Initial operating mode of the channel
             **/
            updateChannel: AugmentedSubmittable<
                (
                    mode:
                        | SnowbridgeCoreOutboundV1OperatingMode
                        | "Normal"
                        | "RejectingOutboundMessages"
                        | number
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SnowbridgeCoreOutboundV1OperatingMode]
            >;
            /**
             * Sends command to the Gateway contract to upgrade itself with a new implementation
             * contract
             *
             * Fee required: No
             *
             * - `origin`: Must be `Root`.
             * - `impl_address`: The address of the implementation contract.
             * - `impl_code_hash`: The codehash of the implementation contract.
             * - `initializer`: Optionally call an initializer on the implementation contract.
             **/
            upgrade: AugmentedSubmittable<
                (
                    implAddress: H160 | string | Uint8Array,
                    implCodeHash: H256 | string | Uint8Array,
                    initializer:
                        | Option<SnowbridgeCoreOutboundV1Initializer>
                        | null
                        | Uint8Array
                        | SnowbridgeCoreOutboundV1Initializer
                        | { params?: any; maximumRequiredGas?: any }
                        | string
                ) => SubmittableExtrinsic<ApiType>,
                [H160, H256, Option<SnowbridgeCoreOutboundV1Initializer>]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        ethereumTokenTransfers: {
            setTokenTransferChannel: AugmentedSubmittable<
                (
                    channelId: SnowbridgeCoreChannelId | string | Uint8Array,
                    agentId: H256 | string | Uint8Array,
                    paraId: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SnowbridgeCoreChannelId, H256, u32]
            >;
            transferNativeToken: AugmentedSubmittable<
                (
                    amount: u128 | AnyNumber | Uint8Array,
                    recipient: H160 | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u128, H160]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        externalValidators: {
            /**
             * Add a new account `who` to the list of `WhitelistedValidators`.
             *
             * The origin for this call must be the `UpdateOrigin`.
             **/
            addWhitelisted: AugmentedSubmittable<
                (who: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Force when the next era will start. Possible values: next session, never, same as always.
             **/
            forceEra: AugmentedSubmittable<
                (
                    mode:
                        | PalletExternalValidatorsForcing
                        | "NotForcing"
                        | "ForceNew"
                        | "ForceNone"
                        | "ForceAlways"
                        | number
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PalletExternalValidatorsForcing]
            >;
            /**
             * Remove an account `who` from the list of `WhitelistedValidators` collators.
             *
             * The origin for this call must be the `UpdateOrigin`.
             **/
            removeWhitelisted: AugmentedSubmittable<
                (who: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Manually set external validators. Should only be needed for tests, validators are set
             * automatically by the bridge.
             **/
            setExternalValidators: AugmentedSubmittable<
                (
                    validators: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                    externalIndex: u64 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>, u64]
            >;
            /**
             * Allow to ignore external validators and use only whitelisted ones.
             *
             * The origin for this call must be the `UpdateOrigin`.
             **/
            skipExternalValidators: AugmentedSubmittable<
                (skip: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [bool]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        externalValidatorSlashes: {
            /**
             * Cancel a slash that was deferred for a later era
             **/
            cancelDeferredSlash: AugmentedSubmittable<
                (
                    era: u32 | AnyNumber | Uint8Array,
                    slashIndices: Vec<u32> | (u32 | AnyNumber | Uint8Array)[]
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Vec<u32>]
            >;
            forceInjectSlash: AugmentedSubmittable<
                (
                    era: u32 | AnyNumber | Uint8Array,
                    validator: AccountId32 | string | Uint8Array,
                    percentage: Perbill | AnyNumber | Uint8Array,
                    externalIdx: u64 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, AccountId32, Perbill, u64]
            >;
            rootTestSendMsgToEth: AugmentedSubmittable<
                (
                    nonce: H256 | string | Uint8Array,
                    numMsgs: u32 | AnyNumber | Uint8Array,
                    msgSize: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, u32, u32]
            >;
            setSlashingMode: AugmentedSubmittable<
                (
                    mode:
                        | PalletExternalValidatorSlashesSlashingModeOption
                        | "Enabled"
                        | "LogOnly"
                        | "Disabled"
                        | number
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PalletExternalValidatorSlashesSlashingModeOption]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        fellowshipCollective: {
            /**
             * Introduce a new member.
             *
             * - `origin`: Must be the `AddOrigin`.
             * - `who`: Account of non-member which will become a member.
             *
             * Weight: `O(1)`
             **/
            addMember: AugmentedSubmittable<
                (
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
                [MultiAddress]
            >;
            /**
             * Remove votes from the given poll. It must have ended.
             *
             * - `origin`: Must be `Signed` by any account.
             * - `poll_index`: Index of a poll which is completed and for which votes continue to
             * exist.
             * - `max`: Maximum number of vote items from remove in this call.
             *
             * Transaction fees are waived if the operation is successful.
             *
             * Weight `O(max)` (less if there are fewer items to remove than `max`).
             **/
            cleanupPoll: AugmentedSubmittable<
                (
                    pollIndex: u32 | AnyNumber | Uint8Array,
                    max: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * Decrement the rank of an existing member by one. If the member is already at rank zero,
             * then they are removed entirely.
             *
             * - `origin`: Must be the `DemoteOrigin`.
             * - `who`: Account of existing member of rank greater than zero.
             *
             * Weight: `O(1)`, less if the member's index is highest in its rank.
             **/
            demoteMember: AugmentedSubmittable<
                (
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
                [MultiAddress]
            >;
            /**
             * Exchanges a member with a new account and the same existing rank.
             *
             * - `origin`: Must be the `ExchangeOrigin`.
             * - `who`: Account of existing member of rank greater than zero to be exchanged.
             * - `new_who`: New Account of existing member of rank greater than zero to exchanged to.
             **/
            exchangeMember: AugmentedSubmittable<
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
                    newWho:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, MultiAddress]
            >;
            /**
             * Increment the rank of an existing member by one.
             *
             * - `origin`: Must be the `PromoteOrigin`.
             * - `who`: Account of existing member.
             *
             * Weight: `O(1)`
             **/
            promoteMember: AugmentedSubmittable<
                (
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
                [MultiAddress]
            >;
            /**
             * Remove the member entirely.
             *
             * - `origin`: Must be the `RemoveOrigin`.
             * - `who`: Account of existing member of rank greater than zero.
             * - `min_rank`: The rank of the member or greater.
             *
             * Weight: `O(min_rank)`.
             **/
            removeMember: AugmentedSubmittable<
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
                    minRank: u16 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, u16]
            >;
            /**
             * Add an aye or nay vote for the sender to the given proposal.
             *
             * - `origin`: Must be `Signed` by a member account.
             * - `poll`: Index of a poll which is ongoing.
             * - `aye`: `true` if the vote is to approve the proposal, `false` otherwise.
             *
             * Transaction fees are be waived if the member is voting on any particular proposal
             * for the first time and the call is successful. Subsequent vote changes will charge a
             * fee.
             *
             * Weight: `O(1)`, less if there was no previous vote on the poll by the member.
             **/
            vote: AugmentedSubmittable<
                (poll: u32 | AnyNumber | Uint8Array, aye: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32, bool]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        fellowshipReferenda: {
            /**
             * Cancel an ongoing referendum.
             *
             * - `origin`: must be the `CancelOrigin`.
             * - `index`: The index of the referendum to be cancelled.
             *
             * Emits `Cancelled`.
             **/
            cancel: AugmentedSubmittable<(index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
            /**
             * Cancel an ongoing referendum and slash the deposits.
             *
             * - `origin`: must be the `KillOrigin`.
             * - `index`: The index of the referendum to be cancelled.
             *
             * Emits `Killed` and `DepositSlashed`.
             **/
            kill: AugmentedSubmittable<(index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
            /**
             * Advance a referendum onto its next logical state. Only used internally.
             *
             * - `origin`: must be `Root`.
             * - `index`: the referendum to be advanced.
             **/
            nudgeReferendum: AugmentedSubmittable<
                (index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Advance a track onto its next logical state. Only used internally.
             *
             * - `origin`: must be `Root`.
             * - `track`: the track to be advanced.
             *
             * Action item for when there is now one fewer referendum in the deciding phase and the
             * `DecidingCount` is not yet updated. This means that we should either:
             * - begin deciding another referendum (and leave `DecidingCount` alone); or
             * - decrement `DecidingCount`.
             **/
            oneFewerDeciding: AugmentedSubmittable<
                (track: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /**
             * Post the Decision Deposit for a referendum.
             *
             * - `origin`: must be `Signed` and the account must have funds available for the
             * referendum's track's Decision Deposit.
             * - `index`: The index of the submitted referendum whose Decision Deposit is yet to be
             * posted.
             *
             * Emits `DecisionDepositPlaced`.
             **/
            placeDecisionDeposit: AugmentedSubmittable<
                (index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Refund the Decision Deposit for a closed referendum back to the depositor.
             *
             * - `origin`: must be `Signed` or `Root`.
             * - `index`: The index of a closed referendum whose Decision Deposit has not yet been
             * refunded.
             *
             * Emits `DecisionDepositRefunded`.
             **/
            refundDecisionDeposit: AugmentedSubmittable<
                (index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Refund the Submission Deposit for a closed referendum back to the depositor.
             *
             * - `origin`: must be `Signed` or `Root`.
             * - `index`: The index of a closed referendum whose Submission Deposit has not yet been
             * refunded.
             *
             * Emits `SubmissionDepositRefunded`.
             **/
            refundSubmissionDeposit: AugmentedSubmittable<
                (index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set or clear metadata of a referendum.
             *
             * Parameters:
             * - `origin`: Must be `Signed` by a creator of a referendum or by anyone to clear a
             * metadata of a finished referendum.
             * - `index`:  The index of a referendum to set or clear metadata for.
             * - `maybe_hash`: The hash of an on-chain stored preimage. `None` to clear a metadata.
             **/
            setMetadata: AugmentedSubmittable<
                (
                    index: u32 | AnyNumber | Uint8Array,
                    maybeHash: Option<H256> | null | Uint8Array | H256 | string
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<H256>]
            >;
            /**
             * Propose a referendum on a privileged action.
             *
             * - `origin`: must be `SubmitOrigin` and the account must have `SubmissionDeposit` funds
             * available.
             * - `proposal_origin`: The origin from which the proposal should be executed.
             * - `proposal`: The proposal.
             * - `enactment_moment`: The moment that the proposal should be enacted.
             *
             * Emits `Submitted`.
             **/
            submit: AugmentedSubmittable<
                (
                    proposalOrigin:
                        | DancelightRuntimeOriginCaller
                        | { system: any }
                        | { Void: any }
                        | { Origins: any }
                        | { ParachainsOrigin: any }
                        | { XcmPallet: any }
                        | string
                        | Uint8Array,
                    proposal:
                        | FrameSupportPreimagesBounded
                        | { Legacy: any }
                        | { Inline: any }
                        | { Lookup: any }
                        | string
                        | Uint8Array,
                    enactmentMoment:
                        | FrameSupportScheduleDispatchTime
                        | { At: any }
                        | { After: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [DancelightRuntimeOriginCaller, FrameSupportPreimagesBounded, FrameSupportScheduleDispatchTime]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        grandpa: {
            /**
             * Note that the current authority set of the GRANDPA finality gadget has stalled.
             *
             * This will trigger a forced authority set change at the beginning of the next session, to
             * be enacted `delay` blocks after that. The `delay` should be high enough to safely assume
             * that the block signalling the forced change will not be re-orged e.g. 1000 blocks.
             * The block production rate (which may be slowed down because of finality lagging) should
             * be taken into account when choosing the `delay`. The GRANDPA voters based on the new
             * authority will start voting on top of `best_finalized_block_number` for new finalized
             * blocks. `best_finalized_block_number` should be the highest of the latest finalized
             * block of all validators of the new authority set.
             *
             * Only callable by root.
             **/
            noteStalled: AugmentedSubmittable<
                (
                    delay: u32 | AnyNumber | Uint8Array,
                    bestFinalizedBlockNumber: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * Report voter equivocation/misbehavior. This method will verify the
             * equivocation proof and validate the given key ownership proof
             * against the extracted offender. If both are valid, the offence
             * will be reported.
             **/
            reportEquivocation: AugmentedSubmittable<
                (
                    equivocationProof:
                        | SpConsensusGrandpaEquivocationProof
                        | { setId?: any; equivocation?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof:
                        | SpSessionMembershipProof
                        | { session?: any; trieNodes?: any; validatorCount?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusGrandpaEquivocationProof, SpSessionMembershipProof]
            >;
            /**
             * Report voter equivocation/misbehavior. This method will verify the
             * equivocation proof and validate the given key ownership proof
             * against the extracted offender. If both are valid, the offence
             * will be reported.
             *
             * This extrinsic must be called unsigned and it is expected that only
             * block authors will call it (validated in `ValidateUnsigned`), as such
             * if the block author is defined it will be defined as the equivocation
             * reporter.
             **/
            reportEquivocationUnsigned: AugmentedSubmittable<
                (
                    equivocationProof:
                        | SpConsensusGrandpaEquivocationProof
                        | { setId?: any; equivocation?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof:
                        | SpSessionMembershipProof
                        | { session?: any; trieNodes?: any; validatorCount?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusGrandpaEquivocationProof, SpSessionMembershipProof]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        hrmp: {
            /**
             * Establish a bidirectional HRMP channel between a parachain and a system chain.
             *
             * Arguments:
             *
             * - `target_system_chain`: A system chain, `ParaId`.
             *
             * The origin needs to be the parachain origin.
             **/
            establishChannelWithSystem: AugmentedSubmittable<
                (targetSystemChain: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Establish an HRMP channel between two system chains. If the channel does not already
             * exist, the transaction fees will be refunded to the caller. The system does not take
             * deposits for channels between system chains, and automatically sets the message number
             * and size limits to the maximum allowed by the network's configuration.
             *
             * Arguments:
             *
             * - `sender`: A system chain, `ParaId`.
             * - `recipient`: A system chain, `ParaId`.
             *
             * Any signed origin can call this function, but _both_ inputs MUST be system chains. If
             * the channel does not exist yet, there is no fee.
             **/
            establishSystemChannel: AugmentedSubmittable<
                (
                    sender: u32 | AnyNumber | Uint8Array,
                    recipient: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * This extrinsic triggers the cleanup of all the HRMP storage items that a para may have.
             * Normally this happens once per session, but this allows you to trigger the cleanup
             * immediately for a specific parachain.
             *
             * Number of inbound and outbound channels for `para` must be provided as witness data.
             *
             * Origin must be the `ChannelManager`.
             **/
            forceCleanHrmp: AugmentedSubmittable<
                (
                    para: u32 | AnyNumber | Uint8Array,
                    numInbound: u32 | AnyNumber | Uint8Array,
                    numOutbound: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32, u32]
            >;
            /**
             * Open a channel from a `sender` to a `recipient` `ParaId`. Although opened by governance,
             * the `max_capacity` and `max_message_size` are still subject to the Relay Chain's
             * configured limits.
             *
             * Expected use is when one (and only one) of the `ParaId`s involved in the channel is
             * governed by the system, e.g. a system parachain.
             *
             * Origin must be the `ChannelManager`.
             **/
            forceOpenHrmpChannel: AugmentedSubmittable<
                (
                    sender: u32 | AnyNumber | Uint8Array,
                    recipient: u32 | AnyNumber | Uint8Array,
                    maxCapacity: u32 | AnyNumber | Uint8Array,
                    maxMessageSize: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32, u32, u32]
            >;
            /**
             * Force process HRMP close channel requests.
             *
             * If there are pending HRMP close channel requests, you can use this function to process
             * all of those requests immediately.
             *
             * Total number of closing channels must be provided as witness data.
             *
             * Origin must be the `ChannelManager`.
             **/
            forceProcessHrmpClose: AugmentedSubmittable<
                (channels: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Force process HRMP open channel requests.
             *
             * If there are pending HRMP open channel requests, you can use this function to process
             * all of those requests immediately.
             *
             * Total number of opening channels must be provided as witness data.
             *
             * Origin must be the `ChannelManager`.
             **/
            forceProcessHrmpOpen: AugmentedSubmittable<
                (channels: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Accept a pending open channel request from the given sender.
             *
             * The channel will be opened only on the next session boundary.
             **/
            hrmpAcceptOpenChannel: AugmentedSubmittable<
                (sender: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * This cancels a pending open channel request. It can be canceled by either of the sender
             * or the recipient for that request. The origin must be either of those.
             *
             * The cancellation happens immediately. It is not possible to cancel the request if it is
             * already accepted.
             *
             * Total number of open requests (i.e. `HrmpOpenChannelRequestsList`) must be provided as
             * witness data.
             **/
            hrmpCancelOpenRequest: AugmentedSubmittable<
                (
                    channelId:
                        | PolkadotParachainPrimitivesPrimitivesHrmpChannelId
                        | { sender?: any; recipient?: any }
                        | string
                        | Uint8Array,
                    openRequests: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PolkadotParachainPrimitivesPrimitivesHrmpChannelId, u32]
            >;
            /**
             * Initiate unilateral closing of a channel. The origin must be either the sender or the
             * recipient in the channel being closed.
             *
             * The closure can only happen on a session change.
             **/
            hrmpCloseChannel: AugmentedSubmittable<
                (
                    channelId:
                        | PolkadotParachainPrimitivesPrimitivesHrmpChannelId
                        | { sender?: any; recipient?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PolkadotParachainPrimitivesPrimitivesHrmpChannelId]
            >;
            /**
             * Initiate opening a channel from a parachain to a given recipient with given channel
             * parameters.
             *
             * - `proposed_max_capacity` - specifies how many messages can be in the channel at once.
             * - `proposed_max_message_size` - specifies the maximum size of the messages.
             *
             * These numbers are a subject to the relay-chain configuration limits.
             *
             * The channel can be opened only after the recipient confirms it and only on a session
             * change.
             **/
            hrmpInitOpenChannel: AugmentedSubmittable<
                (
                    recipient: u32 | AnyNumber | Uint8Array,
                    proposedMaxCapacity: u32 | AnyNumber | Uint8Array,
                    proposedMaxMessageSize: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32, u32]
            >;
            /**
             * Update the deposits held for an HRMP channel to the latest `Configuration`. Channels
             * with system chains do not require a deposit.
             *
             * Arguments:
             *
             * - `sender`: A chain, `ParaId`.
             * - `recipient`: A chain, `ParaId`.
             *
             * Any signed origin can call this function.
             **/
            pokeChannelDeposits: AugmentedSubmittable<
                (
                    sender: u32 | AnyNumber | Uint8Array,
                    recipient: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        identity: {
            /**
             * Accept a given username that an `authority` granted. The call must include the full
             * username, as in `username.suffix`.
             **/
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
             **/
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
             * Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated
             * to the sender.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * sub identity of `sub`.
             **/
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
             * The authority can grant up to `allocation` usernames. To top up the allocation or
             * change the account used to grant usernames, this call can be used with the updated
             * parameters to overwrite the existing configuration.
             **/
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
             * The dispatch origin for this call must be _Signed_ and the sender must have a
             * registered identity.
             *
             * - `reg_index`: The index of the registrar whose judgement is no longer requested.
             *
             * Emits `JudgementUnrequested` if successful.
             **/
            cancelRequest: AugmentedSubmittable<
                (regIndex: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Clear an account's identity info and all sub-accounts and return all deposits.
             *
             * Payment: All reserved balances on the account are returned.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * identity.
             *
             * Emits `IdentityCleared` if successful.
             **/
            clearIdentity: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Remove an account's identity and sub-account information and slash the deposits.
             *
             * Payment: Reserved balances from `set_subs` and `set_identity` are slashed and handled by
             * `Slash`. Verification request deposits are not returned; they should be cancelled
             * manually using `cancel_request`.
             *
             * The dispatch origin for this call must match `T::ForceOrigin`.
             *
             * - `target`: the account whose identity the judgement is upon. This must be an account
             * with a registered identity.
             *
             * Emits `IdentityKilled` if successful.
             **/
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
             * Call with [ForceOrigin](crate::Config::ForceOrigin) privileges which deletes a username
             * and slashes any deposit associated with it.
             **/
            killUsername: AugmentedSubmittable<
                (username: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Provide a judgement for an account's identity.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must be the account
             * of the registrar whose index is `reg_index`.
             *
             * - `reg_index`: the index of the registrar whose judgement is being made.
             * - `target`: the account whose identity the judgement is upon. This must be an account
             * with a registered identity.
             * - `judgement`: the judgement of the registrar of index `reg_index` about `target`.
             * - `identity`: The hash of the [`IdentityInformationProvider`] for that the judgement is
             * provided.
             *
             * Note: Judgements do not apply to a username.
             *
             * Emits `JudgementGiven` if successful.
             **/
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
             * Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated
             * to the sender (*not* the original depositor).
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * super-identity.
             *
             * NOTE: This should not normally be used, but is provided in the case that the non-
             * controller of an account is maliciously registered as a sub-account.
             **/
            quitSub: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Remove an expired username approval. The username was approved by an authority but never
             * accepted by the user and must now be beyond its expiration. The call must include the
             * full username, as in `username.suffix`.
             **/
            removeExpiredApproval: AugmentedSubmittable<
                (username: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Remove the given account from the sender's subs.
             *
             * Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated
             * to the sender.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * sub identity of `sub`.
             **/
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
            /**
             * Permanently delete a username which has been unbinding for longer than the grace period.
             * Caller is refunded the fee if the username expired and the removal was successful.
             **/
            removeUsername: AugmentedSubmittable<
                (username: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Remove `authority` from the username authorities.
             **/
            removeUsernameAuthority: AugmentedSubmittable<
                (
                    suffix: Bytes | string | Uint8Array,
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
                [Bytes, MultiAddress]
            >;
            /**
             * Alter the associated name of the given sub-account.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * sub identity of `sub`.
             **/
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
             * Payment: At most `max_fee` will be reserved for payment to the registrar if judgement
             * given.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a
             * registered identity.
             *
             * - `reg_index`: The index of the registrar whose judgement is requested.
             * - `max_fee`: The maximum fee that may be paid. This should just be auto-populated as:
             *
             * ```nocompile
             * Registrars::<T>::get().get(reg_index).unwrap().fee
             * ```
             *
             * Emits `JudgementRequested` if successful.
             **/
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
             * The dispatch origin for this call must be _Signed_ and the sender must be the account
             * of the registrar whose index is `index`.
             *
             * - `index`: the index of the registrar whose fee is to be set.
             * - `new`: the new account ID.
             **/
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
             * The dispatch origin for this call must be _Signed_ and the sender must be the account
             * of the registrar whose index is `index`.
             *
             * - `index`: the index of the registrar whose fee is to be set.
             * - `fee`: the new fee.
             **/
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
             * The dispatch origin for this call must be _Signed_ and the sender must be the account
             * of the registrar whose index is `index`.
             *
             * - `index`: the index of the registrar whose fee is to be set.
             * - `fields`: the fields that the registrar concerns themselves with.
             **/
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
             * If the account already has identity information, the deposit is taken as part payment
             * for the new deposit.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `info`: The identity information.
             *
             * Emits `IdentitySet` if successful.
             **/
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
            /**
             * Set a given username as the primary. The username should include the suffix.
             **/
            setPrimaryUsername: AugmentedSubmittable<
                (username: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Set the sub-accounts of the sender.
             *
             * Payment: Any aggregate balance reserved by previous `set_subs` calls will be returned
             * and an amount `SubAccountDeposit` will be reserved for each item in `subs`.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * identity.
             *
             * - `subs`: The identity's (new) sub-accounts.
             **/
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
             * If `use_allocation` is set, the authority must have a username allocation available to
             * spend. Otherwise, the authority will need to put up a deposit for registering the
             * username.
             *
             * Users can either pre-sign their usernames or
             * accept them later.
             *
             * Usernames must:
             * - Only contain lowercase ASCII characters or digits.
             * - When combined with the suffix of the issuing authority be _less than_ the
             * `MaxUsernameLength`.
             **/
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
                        | string,
                    useAllocation: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Bytes, Option<SpRuntimeMultiSignature>, bool]
            >;
            /**
             * Start the process of removing a username by placing it in the unbinding usernames map.
             * Once the grace period has passed, the username can be deleted by calling
             * [remove_username](crate::Call::remove_username).
             **/
            unbindUsername: AugmentedSubmittable<
                (username: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        inactivityTracking: {
            setInactivityTrackingStatus: AugmentedSubmittable<
                (enableInactivityTracking: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [bool]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        initializer: {
            /**
             * Issue a signal to the consensus engine to forcibly act as though all parachain
             * blocks in all relay chain blocks up to and including the given number in the current
             * chain are valid and should be finalized.
             **/
            forceApprove: AugmentedSubmittable<
                (upTo: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        messageQueue: {
            /**
             * Execute an overweight message.
             *
             * Temporary processing errors will be propagated whereas permanent errors are treated
             * as success condition.
             *
             * - `origin`: Must be `Signed`.
             * - `message_origin`: The origin from which the message to be executed arrived.
             * - `page`: The page in the queue in which the message to be executed is sitting.
             * - `index`: The index into the queue of the message to be executed.
             * - `weight_limit`: The maximum amount of weight allowed to be consumed in the execution
             * of the message.
             *
             * Benchmark complexity considerations: O(index + weight_limit).
             **/
            executeOverweight: AugmentedSubmittable<
                (
                    messageOrigin:
                        | DancelightRuntimeAggregateMessageOrigin
                        | { Ump: any }
                        | { Snowbridge: any }
                        | { SnowbridgeTanssi: any }
                        | string
                        | Uint8Array,
                    page: u32 | AnyNumber | Uint8Array,
                    index: u32 | AnyNumber | Uint8Array,
                    weightLimit: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [DancelightRuntimeAggregateMessageOrigin, u32, u32, SpWeightsWeightV2Weight]
            >;
            /**
             * Remove a page which has no more messages remaining to be processed or is stale.
             **/
            reapPage: AugmentedSubmittable<
                (
                    messageOrigin:
                        | DancelightRuntimeAggregateMessageOrigin
                        | { Ump: any }
                        | { Snowbridge: any }
                        | { SnowbridgeTanssi: any }
                        | string
                        | Uint8Array,
                    pageIndex: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [DancelightRuntimeAggregateMessageOrigin, u32]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        multiBlockMigrations: {
            /**
             * Clears the `Historic` set.
             *
             * `map_cursor` must be set to the last value that was returned by the
             * `HistoricCleared` event. The first time `None` can be used. `limit` must be chosen in a
             * way that will result in a sensible weight.
             **/
            clearHistoric: AugmentedSubmittable<
                (
                    selector:
                        | PalletMigrationsHistoricCleanupSelector
                        | { Specific: any }
                        | { Wildcard: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PalletMigrationsHistoricCleanupSelector]
            >;
            /**
             * Forces the onboarding of the migrations.
             *
             * This process happens automatically on a runtime upgrade. It is in place as an emergency
             * measurement. The cursor needs to be `None` for this to succeed.
             **/
            forceOnboardMbms: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Allows root to set an active cursor to forcefully start/forward the migration process.
             *
             * This is an edge-case version of [`Self::force_set_cursor`] that allows to set the
             * `started_at` value to the next block number. Otherwise this would not be possible, since
             * `force_set_cursor` takes an absolute block number. Setting `started_at` to `None`
             * indicates that the current block number plus one should be used.
             **/
            forceSetActiveCursor: AugmentedSubmittable<
                (
                    index: u32 | AnyNumber | Uint8Array,
                    innerCursor: Option<Bytes> | null | Uint8Array | Bytes | string,
                    startedAt: Option<u32> | null | Uint8Array | u32 | AnyNumber
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<Bytes>, Option<u32>]
            >;
            /**
             * Allows root to set a cursor to forcefully start, stop or forward the migration process.
             *
             * Should normally not be needed and is only in place as emergency measure. Note that
             * restarting the migration process in this manner will not call the
             * [`MigrationStatusHandler::started`] hook or emit an `UpgradeStarted` event.
             **/
            forceSetCursor: AugmentedSubmittable<
                (
                    cursor:
                        | Option<PalletMigrationsMigrationCursor>
                        | null
                        | Uint8Array
                        | PalletMigrationsMigrationCursor
                        | { Active: any }
                        | { Stuck: any }
                        | string
                ) => SubmittableExtrinsic<ApiType>,
                [Option<PalletMigrationsMigrationCursor>]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        multisig: {
            /**
             * Register approval for a dispatch to be made from a deterministic composite account if
             * approved by a total of `threshold - 1` of `other_signatories`.
             *
             * Payment: `DepositBase` will be reserved if this is the first approval, plus
             * `threshold` times `DepositFactor`. It is returned once this dispatch happens or
             * is cancelled.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `threshold`: The total number of approvals for this dispatch before it is executed.
             * - `other_signatories`: The accounts (other than the sender) who can approve this
             * dispatch. May not be empty.
             * - `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is
             * not the first approval, then it must be `Some`, with the timepoint (block number and
             * transaction index) of the first approval transaction.
             * - `call_hash`: The hash of the call to be executed.
             *
             * NOTE: If this is the final approval, you will want to use `as_multi` instead.
             *
             * ## Complexity
             * - `O(S)`.
             * - Up to one balance-reserve or unreserve operation.
             * - One passthrough operation, one insert, both `O(S)` where `S` is the number of
             * signatories. `S` is capped by `MaxSignatories`, with weight being proportional.
             * - One encode & hash, both of complexity `O(S)`.
             * - Up to one binary search and insert (`O(logS + S)`).
             * - I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove.
             * - One event.
             * - Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit
             * taken for its lifetime of `DepositBase + threshold * DepositFactor`.
             **/
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
             * Register approval for a dispatch to be made from a deterministic composite account if
             * approved by a total of `threshold - 1` of `other_signatories`.
             *
             * If there are enough, then dispatch the call.
             *
             * Payment: `DepositBase` will be reserved if this is the first approval, plus
             * `threshold` times `DepositFactor`. It is returned once this dispatch happens or
             * is cancelled.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `threshold`: The total number of approvals for this dispatch before it is executed.
             * - `other_signatories`: The accounts (other than the sender) who can approve this
             * dispatch. May not be empty.
             * - `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is
             * not the first approval, then it must be `Some`, with the timepoint (block number and
             * transaction index) of the first approval transaction.
             * - `call`: The call to be executed.
             *
             * NOTE: Unless this is the final approval, you will generally want to use
             * `approve_as_multi` instead, since it only requires a hash of the call.
             *
             * Result is equivalent to the dispatched result if `threshold` is exactly `1`. Otherwise
             * on success, result is `Ok` and the result from the interior call, if it was executed,
             * may be found in the deposited `MultisigExecuted` event.
             *
             * ## Complexity
             * - `O(S + Z + Call)`.
             * - Up to one balance-reserve or unreserve operation.
             * - One passthrough operation, one insert, both `O(S)` where `S` is the number of
             * signatories. `S` is capped by `MaxSignatories`, with weight being proportional.
             * - One call encode & hash, both of complexity `O(Z)` where `Z` is tx-len.
             * - One encode & hash, both of complexity `O(S)`.
             * - Up to one binary search and insert (`O(logS + S)`).
             * - I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove.
             * - One event.
             * - The weight of the `call`.
             * - Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit
             * taken for its lifetime of `DepositBase + threshold * DepositFactor`.
             **/
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
             * - `other_signatories`: The accounts (other than the sender) who are part of the
             * multi-signature, but do not participate in the approval process.
             * - `call`: The call to be executed.
             *
             * Result is equivalent to the dispatched result.
             *
             * ## Complexity
             * O(Z + C) where Z is the length of the call and C its execution weight.
             **/
            asMultiThreshold1: AugmentedSubmittable<
                (
                    otherSignatories: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>, Call]
            >;
            /**
             * Cancel a pre-existing, on-going multisig transaction. Any deposit reserved previously
             * for this operation will be unreserved on success.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `threshold`: The total number of approvals for this dispatch before it is executed.
             * - `other_signatories`: The accounts (other than the sender) who can approve this
             * dispatch. May not be empty.
             * - `timepoint`: The timepoint (block number and transaction index) of the first approval
             * transaction for this dispatch.
             * - `call_hash`: The hash of the call to be executed.
             *
             * ## Complexity
             * - `O(S)`.
             * - Up to one balance-reserve or unreserve operation.
             * - One passthrough operation, one insert, both `O(S)` where `S` is the number of
             * signatories. `S` is capped by `MaxSignatories`, with weight being proportional.
             * - One encode & hash, both of complexity `O(S)`.
             * - One event.
             * - I/O: 1 read `O(S)`, one remove.
             * - Storage: removes one item.
             **/
            cancelAsMulti: AugmentedSubmittable<
                (
                    threshold: u16 | AnyNumber | Uint8Array,
                    otherSignatories: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                    timepoint: PalletMultisigTimepoint | { height?: any; index?: any } | string | Uint8Array,
                    callHash: U8aFixed | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u16, Vec<AccountId32>, PalletMultisigTimepoint, U8aFixed]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        onDemandAssignmentProvider: {
            /**
             * Create a single on demand core order.
             * Will use the spot price for the current block and will reap the account if needed.
             *
             * Parameters:
             * - `origin`: The sender of the call, funds will be withdrawn from this account.
             * - `max_amount`: The maximum balance to withdraw from the origin to place an order.
             * - `para_id`: A `ParaId` the origin wants to provide blockspace for.
             *
             * Errors:
             * - `InsufficientBalance`: from the Currency implementation
             * - `QueueFull`
             * - `SpotPriceHigherThanMaxAmount`
             *
             * Events:
             * - `OnDemandOrderPlaced`
             **/
            placeOrderAllowDeath: AugmentedSubmittable<
                (
                    maxAmount: u128 | AnyNumber | Uint8Array,
                    paraId: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u128, u32]
            >;
            /**
             * Same as the [`place_order_allow_death`](Self::place_order_allow_death) call , but with a
             * check that placing the order will not reap the account.
             *
             * Parameters:
             * - `origin`: The sender of the call, funds will be withdrawn from this account.
             * - `max_amount`: The maximum balance to withdraw from the origin to place an order.
             * - `para_id`: A `ParaId` the origin wants to provide blockspace for.
             *
             * Errors:
             * - `InsufficientBalance`: from the Currency implementation
             * - `QueueFull`
             * - `SpotPriceHigherThanMaxAmount`
             *
             * Events:
             * - `OnDemandOrderPlaced`
             **/
            placeOrderKeepAlive: AugmentedSubmittable<
                (
                    maxAmount: u128 | AnyNumber | Uint8Array,
                    paraId: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u128, u32]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        paraInclusion: {
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        paraInherent: {
            /**
             * Enter the paras inherent. This will process bitfields and backed candidates.
             **/
            enter: AugmentedSubmittable<
                (
                    data:
                        | PolkadotPrimitivesVstagingInherentData
                        | { bitfields?: any; backedCandidates?: any; disputes?: any; parentHeader?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PolkadotPrimitivesVstagingInherentData]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        parameters: {
            /**
             * Set the value of a parameter.
             *
             * The dispatch origin of this call must be `AdminOrigin` for the given `key`. Values be
             * deleted by setting them to `None`.
             **/
            setParameter: AugmentedSubmittable<
                (
                    keyValue: DancelightRuntimeRuntimeParameters | { Preimage: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [DancelightRuntimeRuntimeParameters]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        paras: {
            /**
             * Adds the validation code to the storage.
             *
             * The code will not be added if it is already present. Additionally, if PVF pre-checking
             * is running for that code, it will be instantly accepted.
             *
             * Otherwise, the code will be added into the storage. Note that the code will be added
             * into storage with reference count 0. This is to account the fact that there are no users
             * for this code yet. The caller will have to make sure that this code eventually gets
             * used by some parachain or removed from the storage to avoid storage leaks. For the
             * latter prefer to use the `poke_unused_validation_code` dispatchable to raw storage
             * manipulation.
             *
             * This function is mainly meant to be used for upgrading parachains that do not follow
             * the go-ahead signal while the PVF pre-checking feature is enabled.
             **/
            addTrustedValidationCode: AugmentedSubmittable<
                (validationCode: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Note a new block head for para within the context of the current block.
             **/
            forceNoteNewHead: AugmentedSubmittable<
                (
                    para: u32 | AnyNumber | Uint8Array,
                    newHead: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Bytes]
            >;
            /**
             * Put a parachain directly into the next session's action queue.
             * We can't queue it any sooner than this without going into the
             * initializer...
             **/
            forceQueueAction: AugmentedSubmittable<
                (para: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Schedule an upgrade as if it was scheduled in the given relay parent block.
             **/
            forceScheduleCodeUpgrade: AugmentedSubmittable<
                (
                    para: u32 | AnyNumber | Uint8Array,
                    newCode: Bytes | string | Uint8Array,
                    relayParentNumber: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Bytes, u32]
            >;
            /**
             * Set the storage for the parachain validation code immediately.
             **/
            forceSetCurrentCode: AugmentedSubmittable<
                (
                    para: u32 | AnyNumber | Uint8Array,
                    newCode: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Bytes]
            >;
            /**
             * Set the storage for the current parachain head data immediately.
             **/
            forceSetCurrentHead: AugmentedSubmittable<
                (
                    para: u32 | AnyNumber | Uint8Array,
                    newHead: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Bytes]
            >;
            /**
             * Set the storage for the current parachain head data immediately.
             **/
            forceSetMostRecentContext: AugmentedSubmittable<
                (
                    para: u32 | AnyNumber | Uint8Array,
                    context: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * Includes a statement for a PVF pre-checking vote. Potentially, finalizes the vote and
             * enacts the results if that was the last vote before achieving the supermajority.
             **/
            includePvfCheckStatement: AugmentedSubmittable<
                (
                    stmt:
                        | PolkadotPrimitivesV8PvfCheckStatement
                        | { accept?: any; subject?: any; sessionIndex?: any; validatorIndex?: any }
                        | string
                        | Uint8Array,
                    signature: PolkadotPrimitivesV8ValidatorAppSignature | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PolkadotPrimitivesV8PvfCheckStatement, PolkadotPrimitivesV8ValidatorAppSignature]
            >;
            /**
             * Remove the validation code from the storage iff the reference count is 0.
             *
             * This is better than removing the storage directly, because it will not remove the code
             * that was suddenly got used by some parachain while this dispatchable was pending
             * dispatching.
             **/
            pokeUnusedValidationCode: AugmentedSubmittable<
                (validationCodeHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        parasDisputes: {
            forceUnfreeze: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        parasShared: {
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        parasSlashing: {
            reportDisputeLostUnsigned: AugmentedSubmittable<
                (
                    disputeProof:
                        | PolkadotPrimitivesV8SlashingDisputeProof
                        | { timeSlot?: any; kind?: any; validatorIndex?: any; validatorId?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof:
                        | SpSessionMembershipProof
                        | { session?: any; trieNodes?: any; validatorCount?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [PolkadotPrimitivesV8SlashingDisputeProof, SpSessionMembershipProof]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        parasSudoWrapper: {
            /**
             * Forcefully establish a channel from the sender to the recipient.
             *
             * This is equivalent to sending an `Hrmp::hrmp_init_open_channel` extrinsic followed by
             * `Hrmp::hrmp_accept_open_channel`.
             **/
            sudoEstablishHrmpChannel: AugmentedSubmittable<
                (
                    sender: u32 | AnyNumber | Uint8Array,
                    recipient: u32 | AnyNumber | Uint8Array,
                    maxCapacity: u32 | AnyNumber | Uint8Array,
                    maxMessageSize: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32, u32, u32]
            >;
            /**
             * Send a downward XCM to the given para.
             *
             * The given parachain should exist and the payload should not exceed the preconfigured
             * size `config.max_downward_message_size`.
             **/
            sudoQueueDownwardXcm: AugmentedSubmittable<
                (
                    id: u32 | AnyNumber | Uint8Array,
                    xcm: XcmVersionedXcm | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, XcmVersionedXcm]
            >;
            /**
             * Downgrade a lease holding parachain to an on-demand parachain
             **/
            sudoScheduleParachainDowngrade: AugmentedSubmittable<
                (id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Schedule a para to be cleaned up at the start of the next session.
             **/
            sudoScheduleParaCleanup: AugmentedSubmittable<
                (id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Schedule a para to be initialized at the start of the next session.
             *
             * This should only be used for TESTING and not on PRODUCTION chains. It automatically
             * assigns Coretime to the chain and increases the number of cores. Thus, there is no
             * running coretime chain required.
             **/
            sudoScheduleParaInitialize: AugmentedSubmittable<
                (
                    id: u32 | AnyNumber | Uint8Array,
                    genesis:
                        | PolkadotRuntimeParachainsParasParaGenesisArgs
                        | { genesisHead?: any; validationCode?: any; paraKind?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, PolkadotRuntimeParachainsParasParaGenesisArgs]
            >;
            /**
             * Upgrade a parathread (on-demand parachain) to a lease holding parachain
             **/
            sudoScheduleParathreadUpgrade: AugmentedSubmittable<
                (id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Generic tx
             **/
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
            /**
             * Execute pending operations can incur in claim manual rewards per operation, we simply add the worst case
             **/
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
                        | PalletPooledStakingPoolsPoolKind
                        | "Joining"
                        | "AutoCompounding"
                        | "ManualRewards"
                        | "Leaving"
                        | number
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, AccountId32, PalletPooledStakingPoolsPoolKind]
            >;
            requestDelegate: AugmentedSubmittable<
                (
                    candidate: AccountId32 | string | Uint8Array,
                    pool:
                        | PalletPooledStakingPoolsActivePoolKind
                        | "AutoCompounding"
                        | "ManualRewards"
                        | number
                        | Uint8Array,
                    stake: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, PalletPooledStakingPoolsActivePoolKind, u128]
            >;
            /**
             * Request undelegate can incur in either claim manual rewards or hold rebalances, we simply add the worst case
             **/
            requestUndelegate: AugmentedSubmittable<
                (
                    candidate: AccountId32 | string | Uint8Array,
                    pool:
                        | PalletPooledStakingPoolsActivePoolKind
                        | "AutoCompounding"
                        | "ManualRewards"
                        | number
                        | Uint8Array,
                    amount: PalletPooledStakingSharesOrStake | { Shares: any } | { Stake: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, PalletPooledStakingPoolsActivePoolKind, PalletPooledStakingSharesOrStake]
            >;
            swapPool: AugmentedSubmittable<
                (
                    candidate: AccountId32 | string | Uint8Array,
                    sourcePool:
                        | PalletPooledStakingPoolsActivePoolKind
                        | "AutoCompounding"
                        | "ManualRewards"
                        | number
                        | Uint8Array,
                    amount: PalletPooledStakingSharesOrStake | { Shares: any } | { Stake: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, PalletPooledStakingPoolsActivePoolKind, PalletPooledStakingSharesOrStake]
            >;
            updateCandidatePosition: AugmentedSubmittable<
                (candidates: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        preimage: {
            /**
             * Ensure that the a bulk of pre-images is upgraded.
             *
             * The caller pays no fee if at least 90% of pre-images were successfully updated.
             **/
            ensureUpdated: AugmentedSubmittable<
                (hashes: Vec<H256> | (H256 | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<H256>]
            >;
            /**
             * Register a preimage on-chain.
             *
             * If the preimage was previously requested, no fees or deposits are taken for providing
             * the preimage. Otherwise, a deposit is taken proportional to the size of the preimage.
             **/
            notePreimage: AugmentedSubmittable<
                (bytes: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Request a preimage be uploaded to the chain without paying any fees or deposits.
             *
             * If the preimage requests has already been provided on-chain, we unreserve any deposit
             * a user may have paid, and take the control of the preimage out of their hands.
             **/
            requestPreimage: AugmentedSubmittable<
                (hash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Clear an unrequested preimage from the runtime storage.
             *
             * If `len` is provided, then it will be a much cheaper operation.
             *
             * - `hash`: The hash of the preimage to be removed from the store.
             * - `len`: The length of the preimage of `hash`.
             **/
            unnotePreimage: AugmentedSubmittable<
                (hash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Clear a previously made request for a preimage.
             *
             * NOTE: THIS MUST NOT BE CALLED ON `hash` MORE TIMES THAN `request_preimage`.
             **/
            unrequestPreimage: AugmentedSubmittable<
                (hash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        proxy: {
            /**
             * Register a proxy account for the sender that is able to make calls on its behalf.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `proxy`: The account that the `caller` would like to make a proxy.
             * - `proxy_type`: The permissions allowed for this proxy account.
             * - `delay`: The announcement period required of the initial proxy. Will generally be
             * zero.
             **/
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
                        | DancelightRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "Governance"
                        | "IdentityJudgement"
                        | "CancelProxy"
                        | "Auction"
                        | "OnDemandOrdering"
                        | "SudoRegistrar"
                        | "SudoValidatorManagement"
                        | "SessionKeyManagement"
                        | "Staking"
                        | number
                        | Uint8Array,
                    delay: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, DancelightRuntimeProxyType, u32]
            >;
            /**
             * Publish the hash of a proxy-call that will be made in the future.
             *
             * This must be called some number of blocks before the corresponding `proxy` is attempted
             * if the delay associated with the proxy relationship is greater than zero.
             *
             * No more than `MaxPending` announcements may be made at any one time.
             *
             * This will take a deposit of `AnnouncementDepositFactor` as well as
             * `AnnouncementDepositBase` if there are no other pending announcements.
             *
             * The dispatch origin for this call must be _Signed_ and a proxy of `real`.
             *
             * Parameters:
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `call_hash`: The hash of the call to be made by the `real` account.
             **/
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
             * Spawn a fresh new account that is guaranteed to be otherwise inaccessible, and
             * initialize it with a proxy of `proxy_type` for `origin` sender.
             *
             * Requires a `Signed` origin.
             *
             * - `proxy_type`: The type of the proxy that the sender will be registered as over the
             * new account. This will almost always be the most permissive `ProxyType` possible to
             * allow for maximum flexibility.
             * - `index`: A disambiguation index, in case this is called multiple times in the same
             * transaction (e.g. with `utility::batch`). Unless you're using `batch` you probably just
             * want to use `0`.
             * - `delay`: The announcement period required of the initial proxy. Will generally be
             * zero.
             *
             * Fails with `Duplicate` if this has already been called in this transaction, from the
             * same sender, with the same parameters.
             *
             * Fails if there are insufficient funds to pay for deposit.
             **/
            createPure: AugmentedSubmittable<
                (
                    proxyType:
                        | DancelightRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "Governance"
                        | "IdentityJudgement"
                        | "CancelProxy"
                        | "Auction"
                        | "OnDemandOrdering"
                        | "SudoRegistrar"
                        | "SudoValidatorManagement"
                        | "SessionKeyManagement"
                        | "Staking"
                        | number
                        | Uint8Array,
                    delay: u32 | AnyNumber | Uint8Array,
                    index: u16 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [DancelightRuntimeProxyType, u32, u16]
            >;
            /**
             * Removes a previously spawned pure proxy.
             *
             * WARNING: **All access to this account will be lost.** Any funds held in it will be
             * inaccessible.
             *
             * Requires a `Signed` origin, and the sender account must have been created by a call to
             * `pure` with corresponding parameters.
             *
             * - `spawner`: The account that originally called `pure` to create this account.
             * - `index`: The disambiguation index originally passed to `pure`. Probably `0`.
             * - `proxy_type`: The proxy type originally passed to `pure`.
             * - `height`: The height of the chain when the call to `pure` was processed.
             * - `ext_index`: The extrinsic index in which the call to `pure` was processed.
             *
             * Fails with `NoPermission` in case the caller is not a previously created pure
             * account whose `pure` call has corresponding parameters.
             **/
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
                        | DancelightRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "Governance"
                        | "IdentityJudgement"
                        | "CancelProxy"
                        | "Auction"
                        | "OnDemandOrdering"
                        | "SudoRegistrar"
                        | "SudoValidatorManagement"
                        | "SessionKeyManagement"
                        | "Staking"
                        | number
                        | Uint8Array,
                    index: u16 | AnyNumber | Uint8Array,
                    height: Compact<u32> | AnyNumber | Uint8Array,
                    extIndex: Compact<u32> | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, DancelightRuntimeProxyType, u16, Compact<u32>, Compact<u32>]
            >;
            /**
             * Dispatch the given `call` from an account that the sender is authorised for through
             * `add_proxy`.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `force_proxy_type`: Specify the exact proxy type to be used and checked for this call.
             * - `call`: The call to be made by the `real` account.
             **/
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
                        | Option<DancelightRuntimeProxyType>
                        | null
                        | Uint8Array
                        | DancelightRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "Governance"
                        | "IdentityJudgement"
                        | "CancelProxy"
                        | "Auction"
                        | "OnDemandOrdering"
                        | "SudoRegistrar"
                        | "SudoValidatorManagement"
                        | "SessionKeyManagement"
                        | "Staking"
                        | number,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Option<DancelightRuntimeProxyType>, Call]
            >;
            /**
             * Dispatch the given `call` from an account that the sender is authorized for through
             * `add_proxy`.
             *
             * Removes any corresponding announcement(s).
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `force_proxy_type`: Specify the exact proxy type to be used and checked for this call.
             * - `call`: The call to be made by the `real` account.
             **/
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
                        | Option<DancelightRuntimeProxyType>
                        | null
                        | Uint8Array
                        | DancelightRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "Governance"
                        | "IdentityJudgement"
                        | "CancelProxy"
                        | "Auction"
                        | "OnDemandOrdering"
                        | "SudoRegistrar"
                        | "SudoValidatorManagement"
                        | "SessionKeyManagement"
                        | "Staking"
                        | number,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, MultiAddress, Option<DancelightRuntimeProxyType>, Call]
            >;
            /**
             * Remove the given announcement of a delegate.
             *
             * May be called by a target (proxied) account to remove a call that one of their delegates
             * (`delegate`) has announced they want to execute. The deposit is returned.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `delegate`: The account that previously announced the call.
             * - `call_hash`: The hash of the call to be made.
             **/
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
             * May be called by a proxy account to remove a call they previously announced and return
             * the deposit.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `call_hash`: The hash of the call to be made by the `real` account.
             **/
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
             * WARNING: This may be called on accounts created by `pure`, however if done, then
             * the unreserved fees will be inaccessible. **All access to this account will be lost.**
             **/
            removeProxies: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Unregister a proxy account for the sender.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `proxy`: The account that the `caller` would like to remove as a proxy.
             * - `proxy_type`: The permissions currently enabled for the removed proxy account.
             **/
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
                        | DancelightRuntimeProxyType
                        | "Any"
                        | "NonTransfer"
                        | "Governance"
                        | "IdentityJudgement"
                        | "CancelProxy"
                        | "Auction"
                        | "OnDemandOrdering"
                        | "SudoRegistrar"
                        | "SudoValidatorManagement"
                        | "SessionKeyManagement"
                        | "Staking"
                        | number
                        | Uint8Array,
                    delay: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, DancelightRuntimeProxyType, u32]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        referenda: {
            /**
             * Cancel an ongoing referendum.
             *
             * - `origin`: must be the `CancelOrigin`.
             * - `index`: The index of the referendum to be cancelled.
             *
             * Emits `Cancelled`.
             **/
            cancel: AugmentedSubmittable<(index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
            /**
             * Cancel an ongoing referendum and slash the deposits.
             *
             * - `origin`: must be the `KillOrigin`.
             * - `index`: The index of the referendum to be cancelled.
             *
             * Emits `Killed` and `DepositSlashed`.
             **/
            kill: AugmentedSubmittable<(index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
            /**
             * Advance a referendum onto its next logical state. Only used internally.
             *
             * - `origin`: must be `Root`.
             * - `index`: the referendum to be advanced.
             **/
            nudgeReferendum: AugmentedSubmittable<
                (index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Advance a track onto its next logical state. Only used internally.
             *
             * - `origin`: must be `Root`.
             * - `track`: the track to be advanced.
             *
             * Action item for when there is now one fewer referendum in the deciding phase and the
             * `DecidingCount` is not yet updated. This means that we should either:
             * - begin deciding another referendum (and leave `DecidingCount` alone); or
             * - decrement `DecidingCount`.
             **/
            oneFewerDeciding: AugmentedSubmittable<
                (track: u16 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u16]
            >;
            /**
             * Post the Decision Deposit for a referendum.
             *
             * - `origin`: must be `Signed` and the account must have funds available for the
             * referendum's track's Decision Deposit.
             * - `index`: The index of the submitted referendum whose Decision Deposit is yet to be
             * posted.
             *
             * Emits `DecisionDepositPlaced`.
             **/
            placeDecisionDeposit: AugmentedSubmittable<
                (index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Refund the Decision Deposit for a closed referendum back to the depositor.
             *
             * - `origin`: must be `Signed` or `Root`.
             * - `index`: The index of a closed referendum whose Decision Deposit has not yet been
             * refunded.
             *
             * Emits `DecisionDepositRefunded`.
             **/
            refundDecisionDeposit: AugmentedSubmittable<
                (index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Refund the Submission Deposit for a closed referendum back to the depositor.
             *
             * - `origin`: must be `Signed` or `Root`.
             * - `index`: The index of a closed referendum whose Submission Deposit has not yet been
             * refunded.
             *
             * Emits `SubmissionDepositRefunded`.
             **/
            refundSubmissionDeposit: AugmentedSubmittable<
                (index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Set or clear metadata of a referendum.
             *
             * Parameters:
             * - `origin`: Must be `Signed` by a creator of a referendum or by anyone to clear a
             * metadata of a finished referendum.
             * - `index`:  The index of a referendum to set or clear metadata for.
             * - `maybe_hash`: The hash of an on-chain stored preimage. `None` to clear a metadata.
             **/
            setMetadata: AugmentedSubmittable<
                (
                    index: u32 | AnyNumber | Uint8Array,
                    maybeHash: Option<H256> | null | Uint8Array | H256 | string
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<H256>]
            >;
            /**
             * Propose a referendum on a privileged action.
             *
             * - `origin`: must be `SubmitOrigin` and the account must have `SubmissionDeposit` funds
             * available.
             * - `proposal_origin`: The origin from which the proposal should be executed.
             * - `proposal`: The proposal.
             * - `enactment_moment`: The moment that the proposal should be enacted.
             *
             * Emits `Submitted`.
             **/
            submit: AugmentedSubmittable<
                (
                    proposalOrigin:
                        | DancelightRuntimeOriginCaller
                        | { system: any }
                        | { Void: any }
                        | { Origins: any }
                        | { ParachainsOrigin: any }
                        | { XcmPallet: any }
                        | string
                        | Uint8Array,
                    proposal:
                        | FrameSupportPreimagesBounded
                        | { Legacy: any }
                        | { Inline: any }
                        | { Lookup: any }
                        | string
                        | Uint8Array,
                    enactmentMoment:
                        | FrameSupportScheduleDispatchTime
                        | { At: any }
                        | { After: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [DancelightRuntimeOriginCaller, FrameSupportPreimagesBounded, FrameSupportScheduleDispatchTime]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        registrar: {
            /**
             * Add a manager lock from a para. This will prevent the manager of a
             * para to deregister or swap a para.
             *
             * Can be called by Root, the parachain, or the parachain manager if the parachain is
             * unlocked.
             **/
            addLock: AugmentedSubmittable<(para: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
            /**
             * Deregister a Para Id, freeing all data and returning any deposit.
             *
             * The caller must be Root, the `para` owner, or the `para` itself. The para must be an
             * on-demand parachain.
             **/
            deregister: AugmentedSubmittable<
                (id: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Force the registration of a Para Id on the relay chain.
             *
             * This function must be called by a Root origin.
             *
             * The deposit taken can be specified for this registration. Any `ParaId`
             * can be registered, including sub-1000 IDs which are System Parachains.
             **/
            forceRegister: AugmentedSubmittable<
                (
                    who: AccountId32 | string | Uint8Array,
                    deposit: u128 | AnyNumber | Uint8Array,
                    id: u32 | AnyNumber | Uint8Array,
                    genesisHead: Bytes | string | Uint8Array,
                    validationCode: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, u128, u32, Bytes, Bytes]
            >;
            /**
             * Register head data and validation code for a reserved Para Id.
             *
             * ## Arguments
             * - `origin`: Must be called by a `Signed` origin.
             * - `id`: The para ID. Must be owned/managed by the `origin` signing account.
             * - `genesis_head`: The genesis head data of the parachain/thread.
             * - `validation_code`: The initial validation code of the parachain/thread.
             *
             * ## Deposits/Fees
             * The account with the originating signature must reserve a deposit.
             *
             * The deposit is required to cover the costs associated with storing the genesis head
             * data and the validation code.
             * This accounts for the potential to store validation code of a size up to the
             * `max_code_size`, as defined in the configuration pallet
             *
             * Anything already reserved previously for this para ID is accounted for.
             *
             * ## Events
             * The `Registered` event is emitted in case of success.
             **/
            register: AugmentedSubmittable<
                (
                    id: u32 | AnyNumber | Uint8Array,
                    genesisHead: Bytes | string | Uint8Array,
                    validationCode: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Bytes, Bytes]
            >;
            /**
             * Remove a manager lock from a para. This will allow the manager of a
             * previously locked para to deregister or swap a para without using governance.
             *
             * Can only be called by the Root origin or the parachain.
             **/
            removeLock: AugmentedSubmittable<
                (para: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Reserve a Para Id on the relay chain.
             *
             * This function will reserve a new Para Id to be owned/managed by the origin account.
             * The origin account is able to register head data and validation code using `register` to
             * create an on-demand parachain. Using the Slots pallet, an on-demand parachain can then
             * be upgraded to a lease holding parachain.
             *
             * ## Arguments
             * - `origin`: Must be called by a `Signed` origin. Becomes the manager/owner of the new
             * para ID.
             *
             * ## Deposits/Fees
             * The origin must reserve a deposit of `ParaDeposit` for the registration.
             *
             * ## Events
             * The `Reserved` event is emitted in case of success, which provides the ID reserved for
             * use.
             **/
            reserve: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Schedule a parachain upgrade.
             *
             * This will kick off a check of `new_code` by all validators. After the majority of the
             * validators have reported on the validity of the code, the code will either be enacted
             * or the upgrade will be rejected. If the code will be enacted, the current code of the
             * parachain will be overwritten directly. This means that any PoV will be checked by this
             * new code. The parachain itself will not be informed explicitly that the validation code
             * has changed.
             *
             * Can be called by Root, the parachain, or the parachain manager if the parachain is
             * unlocked.
             **/
            scheduleCodeUpgrade: AugmentedSubmittable<
                (
                    para: u32 | AnyNumber | Uint8Array,
                    newCode: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Bytes]
            >;
            /**
             * Set the parachain's current head.
             *
             * Can be called by Root, the parachain, or the parachain manager if the parachain is
             * unlocked.
             **/
            setCurrentHead: AugmentedSubmittable<
                (
                    para: u32 | AnyNumber | Uint8Array,
                    newHead: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Bytes]
            >;
            /**
             * Swap a lease holding parachain with another parachain, either on-demand or lease
             * holding.
             *
             * The origin must be Root, the `para` owner, or the `para` itself.
             *
             * The swap will happen only if there is already an opposite swap pending. If there is not,
             * the swap will be stored in the pending swaps map, ready for a later confirmatory swap.
             *
             * The `ParaId`s remain mapped to the same head data and code so external code can rely on
             * `ParaId` to be a long-term identifier of a notional "parachain". However, their
             * scheduling info (i.e. whether they're an on-demand parachain or lease holding
             * parachain), auction information and the auction deposit are switched.
             **/
            swap: AugmentedSubmittable<
                (
                    id: u32 | AnyNumber | Uint8Array,
                    other: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        rootTesting: {
            /**
             * A dispatch that will fill the block weight up to the given ratio.
             **/
            fillBlock: AugmentedSubmittable<
                (ratio: Perbill | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Perbill]
            >;
            triggerDefensive: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        scheduler: {
            /**
             * Cancel an anonymously scheduled task.
             **/
            cancel: AugmentedSubmittable<
                (
                    when: u32 | AnyNumber | Uint8Array,
                    index: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * Cancel a named scheduled task.
             **/
            cancelNamed: AugmentedSubmittable<
                (id: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [U8aFixed]
            >;
            /**
             * Removes the retry configuration of a task.
             **/
            cancelRetry: AugmentedSubmittable<
                (
                    task: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array]
                ) => SubmittableExtrinsic<ApiType>,
                [ITuple<[u32, u32]>]
            >;
            /**
             * Cancel the retry configuration of a named task.
             **/
            cancelRetryNamed: AugmentedSubmittable<
                (id: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [U8aFixed]
            >;
            /**
             * Anonymously schedule a task.
             **/
            schedule: AugmentedSubmittable<
                (
                    when: u32 | AnyNumber | Uint8Array,
                    maybePeriodic:
                        | Option<ITuple<[u32, u32]>>
                        | null
                        | Uint8Array
                        | ITuple<[u32, u32]>
                        | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array],
                    priority: u8 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<ITuple<[u32, u32]>>, u8, Call]
            >;
            /**
             * Anonymously schedule a task after a delay.
             **/
            scheduleAfter: AugmentedSubmittable<
                (
                    after: u32 | AnyNumber | Uint8Array,
                    maybePeriodic:
                        | Option<ITuple<[u32, u32]>>
                        | null
                        | Uint8Array
                        | ITuple<[u32, u32]>
                        | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array],
                    priority: u8 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<ITuple<[u32, u32]>>, u8, Call]
            >;
            /**
             * Schedule a named task.
             **/
            scheduleNamed: AugmentedSubmittable<
                (
                    id: U8aFixed | string | Uint8Array,
                    when: u32 | AnyNumber | Uint8Array,
                    maybePeriodic:
                        | Option<ITuple<[u32, u32]>>
                        | null
                        | Uint8Array
                        | ITuple<[u32, u32]>
                        | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array],
                    priority: u8 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [U8aFixed, u32, Option<ITuple<[u32, u32]>>, u8, Call]
            >;
            /**
             * Schedule a named task after a delay.
             **/
            scheduleNamedAfter: AugmentedSubmittable<
                (
                    id: U8aFixed | string | Uint8Array,
                    after: u32 | AnyNumber | Uint8Array,
                    maybePeriodic:
                        | Option<ITuple<[u32, u32]>>
                        | null
                        | Uint8Array
                        | ITuple<[u32, u32]>
                        | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array],
                    priority: u8 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [U8aFixed, u32, Option<ITuple<[u32, u32]>>, u8, Call]
            >;
            /**
             * Set a retry configuration for a task so that, in case its scheduled run fails, it will
             * be retried after `period` blocks, for a total amount of `retries` retries or until it
             * succeeds.
             *
             * Tasks which need to be scheduled for a retry are still subject to weight metering and
             * agenda space, same as a regular task. If a periodic task fails, it will be scheduled
             * normally while the task is retrying.
             *
             * Tasks scheduled as a result of a retry for a periodic task are unnamed, non-periodic
             * clones of the original task. Their retry configuration will be derived from the
             * original task's configuration, but will have a lower value for `remaining` than the
             * original `total_retries`.
             **/
            setRetry: AugmentedSubmittable<
                (
                    task: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array],
                    retries: u8 | AnyNumber | Uint8Array,
                    period: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [ITuple<[u32, u32]>, u8, u32]
            >;
            /**
             * Set a retry configuration for a named task so that, in case its scheduled run fails, it
             * will be retried after `period` blocks, for a total amount of `retries` retries or until
             * it succeeds.
             *
             * Tasks which need to be scheduled for a retry are still subject to weight metering and
             * agenda space, same as a regular task. If a periodic task fails, it will be scheduled
             * normally while the task is retrying.
             *
             * Tasks scheduled as a result of a retry for a periodic task are unnamed, non-periodic
             * clones of the original task. Their retry configuration will be derived from the
             * original task's configuration, but will have a lower value for `remaining` than the
             * original `total_retries`.
             **/
            setRetryNamed: AugmentedSubmittable<
                (
                    id: U8aFixed | string | Uint8Array,
                    retries: u8 | AnyNumber | Uint8Array,
                    period: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [U8aFixed, u8, u32]
            >;
            /**
             * Generic tx
             **/
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
             * Set the number of block production credits for this para_id without paying for them.
             * Can only be called by root.
             **/
            setBlockProductionCredits: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    freeBlockCredits: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * Set the number of block production credits for this para_id without paying for them.
             * Can only be called by root.
             **/
            setCollatorAssignmentCredits: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    freeCollatorAssignmentCredits: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * Helper to set and cleanup the `GivenFreeCredits` storage.
             * Can only be called by root.
             **/
            setGivenFreeCredits: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    givenFreeCredits: bool | boolean | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u32, bool]
            >;
            /**
             * Max core price for parathread in relay chain currency
             **/
            setMaxCorePrice: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    maxCorePrice: Option<u128> | null | Uint8Array | u128 | AnyNumber
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<u128>]
            >;
            /**
             * Set the maximum tip a container chain is willing to pay to be assigned a collator on congestion.
             * Can only be called by container chain manager.
             **/
            setMaxTip: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    maxTip: Option<u128> | null | Uint8Array | u128 | AnyNumber
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<u128>]
            >;
            /**
             * Call index to set the refund address for non-spent tokens
             **/
            setRefundAddress: AugmentedSubmittable<
                (
                    paraId: u32 | AnyNumber | Uint8Array,
                    refundAddress: Option<AccountId32> | null | Uint8Array | AccountId32 | string
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<AccountId32>]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        session: {
            /**
             * Removes any session key(s) of the function caller.
             *
             * This doesn't take effect until the next session.
             *
             * The dispatch origin of this function must be Signed and the account must be either be
             * convertible to a validator ID using the chain's typical addressing system (this usually
             * means being a controller account) or directly convertible into a validator ID (which
             * usually means being a stash account).
             *
             * ## Complexity
             * - `O(1)` in number of key types. Actual cost depends on the number of length of
             * `T::Keys::key_ids()` which is fixed.
             **/
            purgeKeys: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Sets the session key(s) of the function caller to `keys`.
             * Allows an account to set its session key prior to becoming a validator.
             * This doesn't take effect until the next session.
             *
             * The dispatch origin of this function must be signed.
             *
             * ## Complexity
             * - `O(1)`. Actual cost depends on the number of length of `T::Keys::key_ids()` which is
             * fixed.
             **/
            setKeys: AugmentedSubmittable<
                (
                    keys:
                        | DancelightRuntimeSessionKeys
                        | {
                              grandpa?: any;
                              babe?: any;
                              paraValidator?: any;
                              paraAssignment?: any;
                              authorityDiscovery?: any;
                              beefy?: any;
                              nimbus?: any;
                          }
                        | string
                        | Uint8Array,
                    proof: Bytes | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [DancelightRuntimeSessionKeys, Bytes]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        streamPayment: {
            /**
             * Accepts a change requested before by the other party. Takes a nonce to prevent
             * frontrunning attacks. If the target made a request, the source is able to change their
             * deposit.
             **/
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
             * Close a given stream in which the origin is involved. It performs the pending payment
             * before closing the stream.
             **/
            closeStream: AugmentedSubmittable<
                (streamId: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /**
             * Allows immediately changing the deposit for a stream, which is simpler than
             * calling `request_change` with the proper parameters.
             * The call takes an asset id to ensure it has not changed (by an accepted request) before
             * the call is included in a block, in which case the unit is no longer the same and quantities
             * will not have the same scale/value.
             **/
            immediatelyChangeDeposit: AugmentedSubmittable<
                (
                    streamId: u64 | AnyNumber | Uint8Array,
                    assetId: TpStreamPaymentCommonAssetId | "Native" | number | Uint8Array,
                    change:
                        | PalletStreamPaymentDepositChange
                        | { Increase: any }
                        | { Decrease: any }
                        | { Absolute: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [u64, TpStreamPaymentCommonAssetId, PalletStreamPaymentDepositChange]
            >;
            /**
             * Create a payment stream from the origin to the target with provided config
             * and initial deposit (in the asset defined in the config).
             **/
            openStream: AugmentedSubmittable<
                (
                    target: AccountId32 | string | Uint8Array,
                    config:
                        | PalletStreamPaymentStreamConfig
                        | {
                              timeUnit?: any;
                              assetId?: any;
                              rate?: any;
                              minimumRequestDeadlineDelay?: any;
                              softMinimumDeposit?: any;
                          }
                        | string
                        | Uint8Array,
                    initialDeposit: u128 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, PalletStreamPaymentStreamConfig, u128]
            >;
            /**
             * Perform the pending payment of a stream. Anyone can call this.
             **/
            performPayment: AugmentedSubmittable<
                (streamId: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /**
             * Requests a change to a stream config or deposit.
             *
             * If the new config don't change the time unit and asset id, the change will be applied
             * immediately if it is at the desadvantage of the caller. Otherwise, the request is stored
             * in the stream and will have to be approved by the other party.
             *
             * This call accepts a deposit change, which can only be provided by the source of the
             * stream. An absolute change is required when changing asset id, as the current deposit
             * will be released and a new deposit is required in the new asset.
             **/
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
                        | {
                              timeUnit?: any;
                              assetId?: any;
                              rate?: any;
                              minimumRequestDeadlineDelay?: any;
                              softMinimumDeposit?: any;
                          }
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        sudo: {
            /**
             * Permanently removes the sudo key.
             *
             * **This cannot be un-done.**
             **/
            removeKey: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo
             * key.
             **/
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
            /**
             * Authenticates the sudo key and dispatches a function call with `Root` origin.
             **/
            sudo: AugmentedSubmittable<
                (call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Call]
            >;
            /**
             * Authenticates the sudo key and dispatches a function call with `Signed` origin from
             * a given account.
             *
             * The dispatch origin for this call must be _Signed_.
             **/
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
             * Authenticates the sudo key and dispatches a function call with `Root` origin.
             * This function does not check the weight of the call, and instead allows the
             * Sudo user to specify the weight of the call.
             *
             * The dispatch origin for this call must be _Signed_.
             **/
            sudoUncheckedWeight: AugmentedSubmittable<
                (
                    call: Call | IMethod | string | Uint8Array,
                    weight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Call, SpWeightsWeightV2Weight]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        system: {
            /**
             * Provide the preimage (runtime binary) `code` for an upgrade that has been authorized.
             *
             * If the authorization required a version check, this call will ensure the spec name
             * remains unchanged and that the spec version has increased.
             *
             * Depending on the runtime's `OnSetCode` configuration, this function may directly apply
             * the new `code` in the same block or attempt to schedule the upgrade.
             *
             * All origins are allowed.
             **/
            applyAuthorizedUpgrade: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied
             * later.
             *
             * This call requires Root origin.
             **/
            authorizeUpgrade: AugmentedSubmittable<
                (codeHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied
             * later.
             *
             * WARNING: This authorizes an upgrade that will take place without any safety checks, for
             * example that the spec name remains the same and that the version number increases. Not
             * recommended for normal use. Use `authorize_upgrade` instead.
             *
             * This call requires Root origin.
             **/
            authorizeUpgradeWithoutChecks: AugmentedSubmittable<
                (codeHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Kill all storage items with a key that starts with the given prefix.
             *
             * **NOTE:** We rely on the Root origin to provide us the number of subkeys under
             * the prefix we are removing to accurately calculate the weight of this function.
             **/
            killPrefix: AugmentedSubmittable<
                (
                    prefix: Bytes | string | Uint8Array,
                    subkeys: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, u32]
            >;
            /**
             * Kill some items from storage.
             **/
            killStorage: AugmentedSubmittable<
                (keys: Vec<Bytes> | (Bytes | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<Bytes>]
            >;
            /**
             * Make some on-chain remark.
             *
             * Can be executed by every `origin`.
             **/
            remark: AugmentedSubmittable<
                (remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Make some on-chain remark and emit event.
             **/
            remarkWithEvent: AugmentedSubmittable<
                (remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Set the new runtime code.
             **/
            setCode: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Set the new runtime code without doing any checks of the given `code`.
             *
             * Note that runtime upgrades will not run if this is called with a not-increasing spec
             * version!
             **/
            setCodeWithoutChecks: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Set the number of pages in the WebAssembly environment's heap.
             **/
            setHeapPages: AugmentedSubmittable<
                (pages: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /**
             * Set some items of storage.
             **/
            setStorage: AugmentedSubmittable<
                (
                    items: Vec<ITuple<[Bytes, Bytes]>> | [Bytes | string | Uint8Array, Bytes | string | Uint8Array][]
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<ITuple<[Bytes, Bytes]>>]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        tanssiAuthorityAssignment: {
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        tanssiCollatorAssignment: {
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        tanssiInvulnerables: {
            /**
             * Add a new account `who` to the list of `Invulnerables` collators.
             *
             * The origin for this call must be the `UpdateOrigin`.
             **/
            addInvulnerable: AugmentedSubmittable<
                (who: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Remove an account `who` from the list of `Invulnerables` collators.
             *
             * The origin for this call must be the `UpdateOrigin`.
             **/
            removeInvulnerable: AugmentedSubmittable<
                (who: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        timestamp: {
            /**
             * Set the current time.
             *
             * This call should be invoked exactly once per block. It will panic at the finalization
             * phase, if this call hasn't been invoked by that time.
             *
             * The timestamp should be greater than the previous one by the amount specified by
             * [`Config::MinimumPeriod`].
             *
             * The dispatch origin for this call must be _None_.
             *
             * This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware
             * that changing the complexity of this call could result exhausting the resources in a
             * block to execute any other calls.
             *
             * ## Complexity
             * - `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)
             * - 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in
             * `on_finalize`)
             * - 1 event handler `on_timestamp_set`. Must be `O(1)`.
             **/
            set: AugmentedSubmittable<
                (now: Compact<u64> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u64>]
            >;
            /**
             * Generic tx
             **/
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
             * The status check is a prerequisite for retrying a failed payout.
             * If a spend has either succeeded or expired, it is removed from the storage by this
             * function. In such instances, transaction fees are refunded.
             *
             * ### Parameters
             * - `index`: The spend index.
             *
             * ## Events
             *
             * Emits [`Event::PaymentFailed`] if the spend payout has failed.
             * Emits [`Event::SpendProcessed`] if the spend payout has succeed.
             **/
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
             * Spends must be claimed within some temporal bounds. A spend may be claimed within one
             * [`Config::PayoutPeriod`] from the `valid_from` block.
             * In case of a payout failure, the spend status must be updated with the `check_status`
             * dispatchable before retrying with the current function.
             *
             * ### Parameters
             * - `index`: The spend index.
             *
             * ## Events
             *
             * Emits [`Event::Paid`] if successful.
             **/
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
             * - `proposal_id`: The index of a proposal
             *
             * ### Complexity
             * - O(A) where `A` is the number of approvals
             *
             * ### Errors
             * - [`Error::ProposalNotApproved`]: The `proposal_id` supplied was not found in the
             * approval queue, i.e., the proposal has not been approved. This could also mean the
             * proposal does not exist altogether, thus there is no way it would have been approved
             * in the first place.
             **/
            removeApproval: AugmentedSubmittable<
                (proposalId: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /**
             * Propose and approve a spend of treasury funds.
             *
             * ## Dispatch Origin
             *
             * Must be [`Config::SpendOrigin`] with the `Success` value being at least
             * `amount` of `asset_kind` in the native asset. The amount of `asset_kind` is converted
             * for assertion using the [`Config::BalanceConverter`].
             *
             * ## Details
             *
             * Create an approved spend for transferring a specific `amount` of `asset_kind` to a
             * designated beneficiary. The spend must be claimed using the `payout` dispatchable within
             * the [`Config::PayoutPeriod`].
             *
             * ### Parameters
             * - `asset_kind`: An indicator of the specific asset class to be spent.
             * - `amount`: The amount to be transferred from the treasury to the `beneficiary`.
             * - `beneficiary`: The beneficiary of the spend.
             * - `valid_from`: The block number from which the spend can be claimed. It can refer to
             * the past if the resulting spend has not yet expired according to the
             * [`Config::PayoutPeriod`]. If `None`, the spend can be claimed immediately after
             * approval.
             *
             * ## Events
             *
             * Emits [`Event::AssetSpendApproved`] if successful.
             **/
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
             * NOTE: For record-keeping purposes, the proposer is deemed to be equivalent to the
             * beneficiary.
             *
             * ### Parameters
             * - `amount`: The amount to be transferred from the treasury to the `beneficiary`.
             * - `beneficiary`: The destination account for the transfer.
             *
             * ## Events
             *
             * Emits [`Event::SpendApproved`] if successful.
             **/
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
             * - `index`: The spend index.
             *
             * ## Events
             *
             * Emits [`Event::AssetSpendVoided`] if successful.
             **/
            voidSpend: AugmentedSubmittable<
                (index: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        utility: {
            /**
             * Send a call through an indexed pseudonym of the sender.
             *
             * Filter from origin are passed along. The call will be dispatched with an origin which
             * use the same filter as the origin of this call.
             *
             * NOTE: If you need to ensure that any account-based filtering is not honored (i.e.
             * because you expect `proxy` to have been used prior in the call stack and you do not want
             * the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`
             * in the Multisig pallet instead.
             *
             * NOTE: Prior to version *12, this was called `as_limited_sub`.
             *
             * The dispatch origin for this call must be _Signed_.
             **/
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
             * - `calls`: The calls to be dispatched from the same origin. The number of call must not
             * exceed the constant: `batched_calls_limit` (available in constant metadata).
             *
             * If origin is root then the calls are dispatched without checking origin filter. (This
             * includes bypassing `frame_system::Config::BaseCallFilter`).
             *
             * ## Complexity
             * - O(C) where C is the number of calls to be batched.
             *
             * This will return `Ok` in all circumstances. To determine the success of the batch, an
             * event is deposited. If a call failed and the batch was interrupted, then the
             * `BatchInterrupted` event is deposited, along with the number of successful calls made
             * and the error of the failed call. If all were successful, then the `BatchCompleted`
             * event is deposited.
             **/
            batch: AugmentedSubmittable<
                (calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<Call>]
            >;
            /**
             * Send a batch of dispatch calls and atomically execute them.
             * The whole transaction will rollback and fail if any of the calls failed.
             *
             * May be called from any origin except `None`.
             *
             * - `calls`: The calls to be dispatched from the same origin. The number of call must not
             * exceed the constant: `batched_calls_limit` (available in constant metadata).
             *
             * If origin is root then the calls are dispatched without checking origin filter. (This
             * includes bypassing `frame_system::Config::BaseCallFilter`).
             *
             * ## Complexity
             * - O(C) where C is the number of calls to be batched.
             **/
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
             * - O(1).
             **/
            dispatchAs: AugmentedSubmittable<
                (
                    asOrigin:
                        | DancelightRuntimeOriginCaller
                        | { system: any }
                        | { Void: any }
                        | { Origins: any }
                        | { ParachainsOrigin: any }
                        | { XcmPallet: any }
                        | string
                        | Uint8Array,
                    call: Call | IMethod | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [DancelightRuntimeOriginCaller, Call]
            >;
            /**
             * Send a batch of dispatch calls.
             * Unlike `batch`, it allows errors and won't interrupt.
             *
             * May be called from any origin except `None`.
             *
             * - `calls`: The calls to be dispatched from the same origin. The number of call must not
             * exceed the constant: `batched_calls_limit` (available in constant metadata).
             *
             * If origin is root then the calls are dispatch without checking origin filter. (This
             * includes bypassing `frame_system::Config::BaseCallFilter`).
             *
             * ## Complexity
             * - O(C) where C is the number of calls to be batched.
             **/
            forceBatch: AugmentedSubmittable<
                (calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<Call>]
            >;
            /**
             * Dispatch a function call with a specified weight.
             *
             * This function does not check the weight of the call, and instead allows the
             * Root origin to specify the weight of the call.
             *
             * The dispatch origin for this call must be _Root_.
             **/
            withWeight: AugmentedSubmittable<
                (
                    call: Call | IMethod | string | Uint8Array,
                    weight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [Call, SpWeightsWeightV2Weight]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        whitelist: {
            dispatchWhitelistedCall: AugmentedSubmittable<
                (
                    callHash: H256 | string | Uint8Array,
                    callEncodedLen: u32 | AnyNumber | Uint8Array,
                    callWeightWitness:
                        | SpWeightsWeightV2Weight
                        | { refTime?: any; proofSize?: any }
                        | string
                        | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [H256, u32, SpWeightsWeightV2Weight]
            >;
            dispatchWhitelistedCallWithPreimage: AugmentedSubmittable<
                (call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Call]
            >;
            removeWhitelistedCall: AugmentedSubmittable<
                (callHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            whitelistCall: AugmentedSubmittable<
                (callHash: H256 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [H256]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        xcmPallet: {
            /**
             * Claims assets trapped on this pallet because of leftover assets during XCM execution.
             *
             * - `origin`: Anyone can call this extrinsic.
             * - `assets`: The exact assets that were trapped. Use the version to specify what version
             * was the latest when they were trapped.
             * - `beneficiary`: The location/account where the claimed assets will be deposited.
             **/
            claimAssets: AugmentedSubmittable<
                (
                    assets: XcmVersionedAssets | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    beneficiary: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedAssets, XcmVersionedLocation]
            >;
            /**
             * Execute an XCM message from a local, signed, origin.
             *
             * An event is deposited indicating whether `msg` could be executed completely or only
             * partially.
             *
             * No more than `max_weight` will be used in its attempted execution. If this is less than
             * the maximum amount of weight that the message could take to be executed, then no
             * execution attempt will be made.
             **/
            execute: AugmentedSubmittable<
                (
                    message: XcmVersionedXcm | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    maxWeight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedXcm, SpWeightsWeightV2Weight]
            >;
            /**
             * Set a safe XCM version (the version that XCM should be encoded with if the most recent
             * version a destination can accept is unknown).
             *
             * - `origin`: Must be an origin specified by AdminOrigin.
             * - `maybe_xcm_version`: The default XCM encoding version, or `None` to disable.
             **/
            forceDefaultXcmVersion: AugmentedSubmittable<
                (maybeXcmVersion: Option<u32> | null | Uint8Array | u32 | AnyNumber) => SubmittableExtrinsic<ApiType>,
                [Option<u32>]
            >;
            /**
             * Ask a location to notify us regarding their XCM version and any changes to it.
             *
             * - `origin`: Must be an origin specified by AdminOrigin.
             * - `location`: The location to which we should subscribe for XCM version notifications.
             **/
            forceSubscribeVersionNotify: AugmentedSubmittable<
                (
                    location: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation]
            >;
            /**
             * Set or unset the global suspension state of the XCM executor.
             *
             * - `origin`: Must be an origin specified by AdminOrigin.
             * - `suspended`: `true` to suspend, `false` to resume.
             **/
            forceSuspension: AugmentedSubmittable<
                (suspended: bool | boolean | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [bool]
            >;
            /**
             * Require that a particular destination should no longer notify us regarding any XCM
             * version changes.
             *
             * - `origin`: Must be an origin specified by AdminOrigin.
             * - `location`: The location to which we are currently subscribed for XCM version
             * notifications which we no longer desire.
             **/
            forceUnsubscribeVersionNotify: AugmentedSubmittable<
                (
                    location: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation]
            >;
            /**
             * Extoll that a particular destination can be communicated with through a particular
             * version of XCM.
             *
             * - `origin`: Must be an origin specified by AdminOrigin.
             * - `location`: The destination that is being described.
             * - `xcm_version`: The latest version of XCM that `location` supports.
             **/
            forceXcmVersion: AugmentedSubmittable<
                (
                    location: StagingXcmV5Location | { parents?: any; interior?: any } | string | Uint8Array,
                    version: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [StagingXcmV5Location, u32]
            >;
            /**
             * Transfer some assets from the local chain to the destination chain through their local,
             * destination or remote reserve.
             *
             * `assets` must have same reserve location and may not be teleportable to `dest`.
             * - `assets` have local reserve: transfer assets to sovereign account of destination
             * chain and forward a notification XCM to `dest` to mint and deposit reserve-based
             * assets to `beneficiary`.
             * - `assets` have destination reserve: burn local assets and forward a notification to
             * `dest` chain to withdraw the reserve assets from this chain's sovereign account and
             * deposit them to `beneficiary`.
             * - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move
             * reserves from this chain's SA to `dest` chain's SA, and forward another XCM to `dest`
             * to mint and deposit reserve-based assets to `beneficiary`.
             *
             * Fee payment on the destination side is made from the asset in the `assets` vector of
             * index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight
             * is needed than `weight_limit`, then the operation will fail and the sent assets may be
             * at risk.
             *
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `[Parent,
             * Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from
             * relay to parachain.
             * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will
             * generally be an `AccountId32` value.
             * - `assets`: The assets to be withdrawn. This should include the assets used to pay the
             * fee on the `dest` (and possibly reserve) chains.
             * - `fee_asset_item`: The index into `assets` of the item which should be used to pay
             * fees.
             * - `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase.
             **/
            limitedReserveTransferAssets: AugmentedSubmittable<
                (
                    dest: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    beneficiary: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    assets: XcmVersionedAssets | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array,
                    weightLimit: XcmV3WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation, XcmVersionedLocation, XcmVersionedAssets, u32, XcmV3WeightLimit]
            >;
            /**
             * Teleport some assets from the local chain to some destination chain.
             *
             * Fee payment on the destination side is made from the asset in the `assets` vector of
             * index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight
             * is needed than `weight_limit`, then the operation will fail and the sent assets may be
             * at risk.
             *
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `[Parent,
             * Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from
             * relay to parachain.
             * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will
             * generally be an `AccountId32` value.
             * - `assets`: The assets to be withdrawn. This should include the assets used to pay the
             * fee on the `dest` chain.
             * - `fee_asset_item`: The index into `assets` of the item which should be used to pay
             * fees.
             * - `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase.
             **/
            limitedTeleportAssets: AugmentedSubmittable<
                (
                    dest: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    beneficiary: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    assets: XcmVersionedAssets | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array,
                    weightLimit: XcmV3WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation, XcmVersionedLocation, XcmVersionedAssets, u32, XcmV3WeightLimit]
            >;
            /**
             * Transfer some assets from the local chain to the destination chain through their local,
             * destination or remote reserve.
             *
             * `assets` must have same reserve location and may not be teleportable to `dest`.
             * - `assets` have local reserve: transfer assets to sovereign account of destination
             * chain and forward a notification XCM to `dest` to mint and deposit reserve-based
             * assets to `beneficiary`.
             * - `assets` have destination reserve: burn local assets and forward a notification to
             * `dest` chain to withdraw the reserve assets from this chain's sovereign account and
             * deposit them to `beneficiary`.
             * - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move
             * reserves from this chain's SA to `dest` chain's SA, and forward another XCM to `dest`
             * to mint and deposit reserve-based assets to `beneficiary`.
             *
             * **This function is deprecated: Use `limited_reserve_transfer_assets` instead.**
             *
             * Fee payment on the destination side is made from the asset in the `assets` vector of
             * index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,
             * with all fees taken as needed from the asset.
             *
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `[Parent,
             * Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from
             * relay to parachain.
             * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will
             * generally be an `AccountId32` value.
             * - `assets`: The assets to be withdrawn. This should include the assets used to pay the
             * fee on the `dest` (and possibly reserve) chains.
             * - `fee_asset_item`: The index into `assets` of the item which should be used to pay
             * fees.
             **/
            reserveTransferAssets: AugmentedSubmittable<
                (
                    dest: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    beneficiary: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    assets: XcmVersionedAssets | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation, XcmVersionedLocation, XcmVersionedAssets, u32]
            >;
            send: AugmentedSubmittable<
                (
                    dest: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    message: XcmVersionedXcm | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation, XcmVersionedXcm]
            >;
            /**
             * Teleport some assets from the local chain to some destination chain.
             *
             * **This function is deprecated: Use `limited_teleport_assets` instead.**
             *
             * Fee payment on the destination side is made from the asset in the `assets` vector of
             * index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,
             * with all fees taken as needed from the asset.
             *
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `[Parent,
             * Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from
             * relay to parachain.
             * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will
             * generally be an `AccountId32` value.
             * - `assets`: The assets to be withdrawn. This should include the assets used to pay the
             * fee on the `dest` chain.
             * - `fee_asset_item`: The index into `assets` of the item which should be used to pay
             * fees.
             **/
            teleportAssets: AugmentedSubmittable<
                (
                    dest: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    beneficiary: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    assets: XcmVersionedAssets | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation, XcmVersionedLocation, XcmVersionedAssets, u32]
            >;
            /**
             * Transfer some assets from the local chain to the destination chain through their local,
             * destination or remote reserve, or through teleports.
             *
             * Fee payment on the destination side is made from the asset in the `assets` vector of
             * index `fee_asset_item` (hence referred to as `fees`), up to enough to pay for
             * `weight_limit` of weight. If more weight is needed than `weight_limit`, then the
             * operation will fail and the sent assets may be at risk.
             *
             * `assets` (excluding `fees`) must have same reserve location or otherwise be teleportable
             * to `dest`, no limitations imposed on `fees`.
             * - for local reserve: transfer assets to sovereign account of destination chain and
             * forward a notification XCM to `dest` to mint and deposit reserve-based assets to
             * `beneficiary`.
             * - for destination reserve: burn local assets and forward a notification to `dest` chain
             * to withdraw the reserve assets from this chain's sovereign account and deposit them
             * to `beneficiary`.
             * - for remote reserve: burn local assets, forward XCM to reserve chain to move reserves
             * from this chain's SA to `dest` chain's SA, and forward another XCM to `dest` to mint
             * and deposit reserve-based assets to `beneficiary`.
             * - for teleports: burn local assets and forward XCM to `dest` chain to mint/teleport
             * assets and deposit them to `beneficiary`.
             *
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `X2(Parent,
             * Parachain(..))` to send from parachain to parachain, or `X1(Parachain(..))` to send
             * from relay to parachain.
             * - `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will
             * generally be an `AccountId32` value.
             * - `assets`: The assets to be withdrawn. This should include the assets used to pay the
             * fee on the `dest` (and possibly reserve) chains.
             * - `fee_asset_item`: The index into `assets` of the item which should be used to pay
             * fees.
             * - `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase.
             **/
            transferAssets: AugmentedSubmittable<
                (
                    dest: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    beneficiary: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    assets: XcmVersionedAssets | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    feeAssetItem: u32 | AnyNumber | Uint8Array,
                    weightLimit: XcmV3WeightLimit | { Unlimited: any } | { Limited: any } | string | Uint8Array
                ) => SubmittableExtrinsic<ApiType>,
                [XcmVersionedLocation, XcmVersionedLocation, XcmVersionedAssets, u32, XcmV3WeightLimit]
            >;
            /**
             * Transfer assets from the local chain to the destination chain using explicit transfer
             * types for assets and fees.
             *
             * `assets` must have same reserve location or may be teleportable to `dest`. Caller must
             * provide the `assets_transfer_type` to be used for `assets`:
             * - `TransferType::LocalReserve`: transfer assets to sovereign account of destination
             * chain and forward a notification XCM to `dest` to mint and deposit reserve-based
             * assets to `beneficiary`.
             * - `TransferType::DestinationReserve`: burn local assets and forward a notification to
             * `dest` chain to withdraw the reserve assets from this chain's sovereign account and
             * deposit them to `beneficiary`.
             * - `TransferType::RemoteReserve(reserve)`: burn local assets, forward XCM to `reserve`
             * chain to move reserves from this chain's SA to `dest` chain's SA, and forward another
             * XCM to `dest` to mint and deposit reserve-based assets to `beneficiary`. Typically
             * the remote `reserve` is Asset Hub.
             * - `TransferType::Teleport`: burn local assets and forward XCM to `dest` chain to
             * mint/teleport assets and deposit them to `beneficiary`.
             *
             * On the destination chain, as well as any intermediary hops, `BuyExecution` is used to
             * buy execution using transferred `assets` identified by `remote_fees_id`.
             * Make sure enough of the specified `remote_fees_id` asset is included in the given list
             * of `assets`. `remote_fees_id` should be enough to pay for `weight_limit`. If more weight
             * is needed than `weight_limit`, then the operation will fail and the sent assets may be
             * at risk.
             *
             * `remote_fees_id` may use different transfer type than rest of `assets` and can be
             * specified through `fees_transfer_type`.
             *
             * The caller needs to specify what should happen to the transferred assets once they reach
             * the `dest` chain. This is done through the `custom_xcm_on_dest` parameter, which
             * contains the instructions to execute on `dest` as a final step.
             * This is usually as simple as:
             * `Xcm(vec![DepositAsset { assets: Wild(AllCounted(assets.len())), beneficiary }])`,
             * but could be something more exotic like sending the `assets` even further.
             *
             * - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
             * - `dest`: Destination context for the assets. Will typically be `[Parent,
             * Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from
             * relay to parachain, or `(parents: 2, (GlobalConsensus(..), ..))` to send from
             * parachain across a bridge to another ecosystem destination.
             * - `assets`: The assets to be withdrawn. This should include the assets used to pay the
             * fee on the `dest` (and possibly reserve) chains.
             * - `assets_transfer_type`: The XCM `TransferType` used to transfer the `assets`.
             * - `remote_fees_id`: One of the included `assets` to be used to pay fees.
             * - `fees_transfer_type`: The XCM `TransferType` used to transfer the `fees` assets.
             * - `custom_xcm_on_dest`: The XCM to be executed on `dest` chain as the last step of the
             * transfer, which also determines what happens to the assets on the destination chain.
             * - `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase.
             **/
            transferAssetsUsingTypeAndThen: AugmentedSubmittable<
                (
                    dest: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    assets: XcmVersionedAssets | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    assetsTransferType:
                        | StagingXcmExecutorAssetTransferTransferType
                        | { Teleport: any }
                        | { LocalReserve: any }
                        | { DestinationReserve: any }
                        | { RemoteReserve: any }
                        | string
                        | Uint8Array,
                    remoteFeesId: XcmVersionedAssetId | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
                    feesTransferType:
                        | StagingXcmExecutorAssetTransferTransferType
                        | { Teleport: any }
                        | { LocalReserve: any }
                        | { DestinationReserve: any }
                        | { RemoteReserve: any }
                        | string
                        | Uint8Array,
                    customXcmOnDest: XcmVersionedXcm | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array,
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
    } // AugmentedSubmittables
} // declare module
