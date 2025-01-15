// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/storage";

import type { ApiTypes, AugmentedQuery, QueryableStorageEntry } from "@polkadot/api-base/types";
import type { Data } from "@polkadot/types";
import type {
    BTreeMap,
    BTreeSet,
    Bytes,
    Null,
    Option,
    Struct,
    U8aFixed,
    Vec,
    bool,
    u128,
    u16,
    u32,
    u64,
} from "@polkadot/types-codec";
import type { AnyNumber, ITuple } from "@polkadot/types-codec/types";
import type { AccountId32, H256, Perbill } from "@polkadot/types/interfaces/runtime";
import type {
    BinaryHeapEnqueuedOrder,
    DancelightRuntimeAggregateMessageOrigin,
    DancelightRuntimeRuntimeHoldReason,
    DancelightRuntimeRuntimeParametersKey,
    DancelightRuntimeRuntimeParametersValue,
    DancelightRuntimeSessionKeys,
    DpCollatorAssignmentAssignedCollatorsAccountId32,
    DpCollatorAssignmentAssignedCollatorsPublic,
    DpContainerChainGenesisDataContainerChainGenesisData,
    FrameSupportDispatchPerDispatchClassWeight,
    FrameSupportTokensMiscIdAmount,
    FrameSystemAccountInfo,
    FrameSystemCodeUpgradeAuthorization,
    FrameSystemEventRecord,
    FrameSystemLastRuntimeUpgradeInfo,
    FrameSystemPhase,
    NimbusPrimitivesNimbusCryptoPublic,
    PalletBalancesAccountData,
    PalletBalancesBalanceLock,
    PalletBalancesReserveData,
    PalletConfigurationHostConfiguration,
    PalletConvictionVotingVoteVoting,
    PalletDataPreserversRegisteredProfile,
    PalletExternalValidatorSlashesSlash,
    PalletExternalValidatorsForcing,
    PalletExternalValidatorsRewardsEraRewardPoints,
    PalletGrandpaStoredPendingChange,
    PalletGrandpaStoredState,
    PalletIdentityAuthorityProperties,
    PalletIdentityRegistrarInfo,
    PalletIdentityRegistration,
    PalletInflationRewardsChainsToRewardValue,
    PalletMessageQueueBookState,
    PalletMessageQueuePage,
    PalletMigrationsMigrationCursor,
    PalletMultisigMultisig,
    PalletPooledStakingCandidateEligibleCandidate,
    PalletPooledStakingPendingOperationKey,
    PalletPooledStakingPoolsKey,
    PalletPreimageOldRequestStatus,
    PalletPreimageRequestStatus,
    PalletProxyAnnouncement,
    PalletProxyProxyDefinition,
    PalletRankedCollectiveMemberRecord,
    PalletRankedCollectiveVoteRecord,
    PalletReferendaReferendumInfoConvictionVotingTally,
    PalletReferendaReferendumInfoRankedCollectiveTally,
    PalletRegistrarDepositInfo,
    PalletSchedulerRetryConfig,
    PalletSchedulerScheduled,
    PalletTransactionPaymentReleases,
    PalletTreasuryProposal,
    PalletTreasurySpendStatus,
    PalletXcmQueryStatus,
    PalletXcmRemoteLockedFungibleRecord,
    PalletXcmVersionMigrationStage,
    PolkadotCorePrimitivesInboundDownwardMessage,
    PolkadotCorePrimitivesInboundHrmpMessage,
    PolkadotParachainPrimitivesPrimitivesHrmpChannelId,
    PolkadotPrimitivesV8AssignmentAppPublic,
    PolkadotPrimitivesV8DisputeState,
    PolkadotPrimitivesV8ExecutorParams,
    PolkadotPrimitivesV8ScrapedOnChainVotes,
    PolkadotPrimitivesV8SessionInfo,
    PolkadotPrimitivesV8SlashingPendingSlashes,
    PolkadotPrimitivesV8UpgradeGoAhead,
    PolkadotPrimitivesV8UpgradeRestriction,
    PolkadotPrimitivesV8ValidatorAppPublic,
    PolkadotRuntimeCommonParasRegistrarParaInfo,
    PolkadotRuntimeParachainsConfigurationHostConfiguration,
    PolkadotRuntimeParachainsHrmpHrmpChannel,
    PolkadotRuntimeParachainsHrmpHrmpOpenChannelRequest,
    PolkadotRuntimeParachainsInclusionCandidatePendingAvailability,
    PolkadotRuntimeParachainsInitializerBufferedSessionChange,
    PolkadotRuntimeParachainsOnDemandTypesCoreAffinityCount,
    PolkadotRuntimeParachainsOnDemandTypesEnqueuedOrder,
    PolkadotRuntimeParachainsOnDemandTypesQueueStatusType,
    PolkadotRuntimeParachainsParasParaGenesisArgs,
    PolkadotRuntimeParachainsParasParaLifecycle,
    PolkadotRuntimeParachainsParasParaPastCodeMeta,
    PolkadotRuntimeParachainsParasPvfCheckActiveVoteState,
    PolkadotRuntimeParachainsSchedulerPalletCoreOccupied,
    PolkadotRuntimeParachainsSchedulerPalletParasEntry,
    PolkadotRuntimeParachainsSharedAllowedRelayParentsTracker,
    SnowbridgeBeaconPrimitivesCompactBeaconState,
    SnowbridgeBeaconPrimitivesSyncCommitteePrepared,
    SnowbridgeCoreChannel,
    SnowbridgeCoreChannelId,
    SnowbridgeCoreOperatingModeBasicOperatingMode,
    SnowbridgeCorePricingPricingParameters,
    SnowbridgePalletOutboundQueueCommittedMessage,
    SpAuthorityDiscoveryAppPublic,
    SpConsensusBabeAppPublic,
    SpConsensusBabeBabeEpochConfiguration,
    SpConsensusBabeDigestsNextConfigDescriptor,
    SpConsensusBabeDigestsPreDigest,
    SpConsensusBeefyEcdsaCryptoPublic,
    SpConsensusBeefyMmrBeefyAuthoritySet,
    SpConsensusGrandpaAppPublic,
    SpCoreCryptoKeyTypeId,
    SpRuntimeDigest,
    SpStakingOffenceOffenceDetails,
    SpWeightsWeightV2Weight,
    StagingXcmV4Instruction,
    StagingXcmV4Location,
    StagingXcmV4Xcm,
    TpTraitsActiveEraInfo,
    TpTraitsContainerChainBlockInfo,
    TpTraitsParathreadParams,
    XcmVersionedAssetId,
    XcmVersionedLocation,
} from "@polkadot/types/lookup";
import type { Observable } from "@polkadot/types/types";

export type __AugmentedQuery<ApiType extends ApiTypes> = AugmentedQuery<ApiType, () => unknown>;
export type __QueryableStorageEntry<ApiType extends ApiTypes> = QueryableStorageEntry<ApiType>;

declare module "@polkadot/api-base/types/storage" {
    interface AugmentedQueries<ApiType extends ApiTypes> {
        assetRate: {
            /**
             * Maps an asset to its fixed point representation in the native balance.
             *
             * E.g. `native_amount = asset_amount * ConversionRateToNative::<T>::get(asset_kind)`
             */
            conversionRateToNative: AugmentedQuery<ApiType, (arg: Null | null) => Observable<Option<u128>>, [Null]> &
                QueryableStorageEntry<ApiType, [Null]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        authorityDiscovery: {
            /** Keys of the current authority set. */
            keys: AugmentedQuery<ApiType, () => Observable<Vec<SpAuthorityDiscoveryAppPublic>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Keys of the next authority set. */
            nextKeys: AugmentedQuery<ApiType, () => Observable<Vec<SpAuthorityDiscoveryAppPublic>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        authorNoting: {
            /** Was the containerAuthorData set? */
            didSetContainerAuthorData: AugmentedQuery<ApiType, () => Observable<bool>, []> &
                QueryableStorageEntry<ApiType, []>;
            latestAuthor: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<TpTraitsContainerChainBlockInfo>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        authorship: {
            /** Author of current block. */
            author: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        babe: {
            /** Current epoch authorities. */
            authorities: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[SpConsensusBabeAppPublic, u64]>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * This field should always be populated during block processing unless secondary plain slots are enabled (which
             * don't contain a VRF output).
             *
             * It is set in `on_finalize`, before it will contain the value from the last block.
             */
            authorVrfRandomness: AugmentedQuery<ApiType, () => Observable<Option<U8aFixed>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Current slot number. */
            currentSlot: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /** The configuration for the current epoch. Should never be `None` as it is initialized in genesis. */
            epochConfig: AugmentedQuery<ApiType, () => Observable<Option<SpConsensusBabeBabeEpochConfiguration>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Current epoch index. */
            epochIndex: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The block numbers when the last and current epoch have started, respectively `N-1` and `N`. NOTE: We track this
             * is in order to annotate the block number when a given pool of entropy was fixed (i.e. it was known to chain
             * observers). Since epochs are defined in slots, which may be skipped, the block numbers may not line up with the
             * slot numbers.
             */
            epochStart: AugmentedQuery<ApiType, () => Observable<ITuple<[u32, u32]>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** The slot at which the first epoch actually started. This is 0 until the first block of the chain. */
            genesisSlot: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Temporary value (cleared at block finalization) which is `Some` if per-block initialization has already been
             * called for current block.
             */
            initialized: AugmentedQuery<
                ApiType,
                () => Observable<Option<Option<SpConsensusBabeDigestsPreDigest>>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * How late the current block is compared to its parent.
             *
             * This entry is populated as part of block execution and is cleaned up on block finalization. Querying this
             * storage entry outside of block execution context should always yield zero.
             */
            lateness: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /** Next epoch authorities. */
            nextAuthorities: AugmentedQuery<
                ApiType,
                () => Observable<Vec<ITuple<[SpConsensusBabeAppPublic, u64]>>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The configuration for the next epoch, `None` if the config will not change (you can fallback to `EpochConfig`
             * instead in that case).
             */
            nextEpochConfig: AugmentedQuery<
                ApiType,
                () => Observable<Option<SpConsensusBabeBabeEpochConfiguration>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** Next epoch randomness. */
            nextRandomness: AugmentedQuery<ApiType, () => Observable<U8aFixed>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Pending epoch configuration change that will be applied when the next epoch is enacted. */
            pendingEpochConfigChange: AugmentedQuery<
                ApiType,
                () => Observable<Option<SpConsensusBabeDigestsNextConfigDescriptor>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The epoch randomness for the _current_ epoch.
             *
             * # Security
             *
             * This MUST NOT be used for gambling, as it can be influenced by a malicious validator in the short term. It MAY
             * be used in many cryptographic protocols, however, so long as one remembers that this (like everything else
             * on-chain) it is public. For example, it can be used where a number is needed that cannot have been chosen by an
             * adversary, for purposes such as public-coin zero-knowledge proofs.
             */
            randomness: AugmentedQuery<ApiType, () => Observable<U8aFixed>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Randomness under construction.
             *
             * We make a trade-off between storage accesses and list length. We store the under-construction randomness in
             * segments of up to `UNDER_CONSTRUCTION_SEGMENT_LENGTH`.
             *
             * Once a segment reaches this length, we begin the next one. We reset all segments and return to `0` at the
             * beginning of every epoch.
             */
            segmentIndex: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * A list of the last 100 skipped epochs and the corresponding session index when the epoch was skipped.
             *
             * This is only used for validating equivocation proofs. An equivocation proof must contains a key-ownership proof
             * for a given session, therefore we need a way to tie together sessions and epoch indices, i.e. we need to
             * validate that a validator was the owner of a given key on a given session, and what the active epoch index was
             * during that session.
             */
            skippedEpochs: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[u64, u32]>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** TWOX-NOTE: `SegmentIndex` is an increasing integer, so this is okay. */
            underConstruction: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<U8aFixed>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        balances: {
            /**
             * The Balances pallet example of storing the balance of an account.
             *
             * # Example
             *
             * ```nocompile
             * impl pallet_balances::Config for Runtime {
             * type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>
             * }
             * ```
             *
             * You can also store the balance of an account in the `System` pallet.
             *
             * # Example
             *
             * ```nocompile
             * impl pallet_balances::Config for Runtime {
             * type AccountStore = System
             * }
             * ```
             *
             * But this comes with tradeoffs, storing account balances in the system pallet stores `frame_system` data
             * alongside the account data contrary to storing account balances in the `Balances` pallet, which uses a
             * `StorageMap` to store balances data only. NOTE: This is only used in the case that this pallet is used to store
             * balances.
             */
            account: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<PalletBalancesAccountData>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /** Freeze locks on account balances. */
            freezes: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Vec<FrameSupportTokensMiscIdAmount>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /** Holds on account balances. */
            holds: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<
                    Vec<
                        {
                            readonly id: DancelightRuntimeRuntimeHoldReason;
                            readonly amount: u128;
                        } & Struct
                    >
                >,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /** The total units of outstanding deactivated balance in the system. */
            inactiveIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Any liquidity locks on some account balances. NOTE: Should only be accessed when setting, changing and freeing
             * a lock.
             *
             * Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`
             */
            locks: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesBalanceLock>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Named reserves on some account balances.
             *
             * Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`
             */
            reserves: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesReserveData>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /** The total units issued in the system. */
            totalIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        beefy: {
            /** The current authorities set */
            authorities: AugmentedQuery<ApiType, () => Observable<Vec<SpConsensusBeefyEcdsaCryptoPublic>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Block number where BEEFY consensus is enabled/started. By changing this (through privileged
             * `set_new_genesis()`), BEEFY consensus is effectively restarted from the newly set block number.
             */
            genesisBlock: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Authorities set scheduled to be used with the next session */
            nextAuthorities: AugmentedQuery<ApiType, () => Observable<Vec<SpConsensusBeefyEcdsaCryptoPublic>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * A mapping from BEEFY set ID to the index of the _most recent_ session for which its members were responsible.
             *
             * This is only used for validating equivocation proofs. An equivocation proof must contains a key-ownership proof
             * for a given session, therefore we need a way to tie together sessions and BEEFY set ids, i.e. we need to
             * validate that a validator was the owner of a given key on a given session, and what the active set ID was
             * during that session.
             *
             * TWOX-NOTE: `ValidatorSetId` is not under user control.
             */
            setIdSession: AugmentedQuery<
                ApiType,
                (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<u32>>,
                [u64]
            > &
                QueryableStorageEntry<ApiType, [u64]>;
            /** The current validator set id */
            validatorSetId: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        beefyMmrLeaf: {
            /** Details of current BEEFY authority set. */
            beefyAuthorities: AugmentedQuery<ApiType, () => Observable<SpConsensusBeefyMmrBeefyAuthoritySet>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Details of next BEEFY authority set.
             *
             * This storage entry is used as cache for calls to `update_beefy_next_authority_set`.
             */
            beefyNextAuthorities: AugmentedQuery<ApiType, () => Observable<SpConsensusBeefyMmrBeefyAuthoritySet>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        collatorConfiguration: {
            /** The active configuration for the current session. */
            activeConfig: AugmentedQuery<ApiType, () => Observable<PalletConfigurationHostConfiguration>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * If this is set, then the configuration setters will bypass the consistency checks. This is meant to be used
             * only as the last resort.
             */
            bypassConsistencyCheck: AugmentedQuery<ApiType, () => Observable<bool>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Pending configuration changes.
             *
             * This is a list of configuration changes, each with a session index at which it should be applied.
             *
             * The list is sorted ascending by session index. Also, this list can only contain at most 2 items: for the next
             * session and for the `scheduled_session`.
             */
            pendingConfigs: AugmentedQuery<
                ApiType,
                () => Observable<Vec<ITuple<[u32, PalletConfigurationHostConfiguration]>>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        configuration: {
            /** The active configuration for the current session. */
            activeConfig: AugmentedQuery<
                ApiType,
                () => Observable<PolkadotRuntimeParachainsConfigurationHostConfiguration>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * If this is set, then the configuration setters will bypass the consistency checks. This is meant to be used
             * only as the last resort.
             */
            bypassConsistencyCheck: AugmentedQuery<ApiType, () => Observable<bool>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Pending configuration changes.
             *
             * This is a list of configuration changes, each with a session index at which it should be applied.
             *
             * The list is sorted ascending by session index. Also, this list can only contain at most 2 items: for the next
             * session and for the `scheduled_session`.
             */
            pendingConfigs: AugmentedQuery<
                ApiType,
                () => Observable<Vec<ITuple<[u32, PolkadotRuntimeParachainsConfigurationHostConfiguration]>>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        containerRegistrar: {
            /**
             * This storage aims to act as a 'buffer' for paraIds that must be deregistered at the end of the block execution
             * by calling 'T::InnerRegistrar::deregister()' implementation.
             *
             * We need this buffer because when we are using this pallet on a relay-chain environment like Dancelight (where
             * 'T::InnerRegistrar' implementation is usually the 'paras_registrar' pallet) we need to deregister (via
             * 'paras_registrar::deregister') the same paraIds we have in 'PendingToRemove<T>', and we need to do this
             * deregistration process inside 'on_finalize' hook.
             *
             * It can be the case that some paraIds need to be downgraded to a parathread before deregistering on
             * 'paras_registrar'. This process usually takes 2 sessions, and the actual downgrade happens when the block
             * finalizes.
             *
             * Therefore, if we tried to perform this relay deregistration process at the beginning of the session/block
             * inside ('on_initialize') initializer_on_new_session() as we do for this pallet, it would fail due to the
             * downgrade process could have not taken place yet.
             */
            bufferedParasToDeregister: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            paraGenesisData: AugmentedQuery<
                ApiType,
                (
                    arg: u32 | AnyNumber | Uint8Array
                ) => Observable<Option<DpContainerChainGenesisDataContainerChainGenesisData>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            paraManager: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<AccountId32>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            parathreadParams: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<TpTraitsParathreadParams>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            paused: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> & QueryableStorageEntry<ApiType, []>;
            pendingParaIds: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[u32, Vec<u32>]>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            pendingParathreadParams: AugmentedQuery<
                ApiType,
                () => Observable<Vec<ITuple<[u32, Vec<ITuple<[u32, TpTraitsParathreadParams]>>]>>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            pendingPaused: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[u32, Vec<u32>]>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            pendingToRemove: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[u32, Vec<u32>]>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            pendingVerification: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<Null>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            registeredParaIds: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Registrar deposits, a mapping from paraId to a struct holding the creator (from which the deposit was reserved)
             * and the deposit amount
             */
            registrarDeposit: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletRegistrarDepositInfo>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        convictionVoting: {
            /**
             * The voting classes which have a non-zero lock requirement and the lock amounts which they require. The actual
             * amount locked on behalf of this pallet should always be the maximum of this list.
             */
            classLocksFor: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Vec<ITuple<[u16, u128]>>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * All voting for a particular voter in a particular voting class. We store the balance for the number of votes
             * that we have recorded.
             */
            votingFor: AugmentedQuery<
                ApiType,
                (
                    arg1: AccountId32 | string | Uint8Array,
                    arg2: u16 | AnyNumber | Uint8Array
                ) => Observable<PalletConvictionVotingVoteVoting>,
                [AccountId32, u16]
            > &
                QueryableStorageEntry<ApiType, [AccountId32, u16]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        dataPreservers: {
            assignments: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<BTreeSet<u64>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            nextProfileId: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            profiles: AugmentedQuery<
                ApiType,
                (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<PalletDataPreserversRegisteredProfile>>,
                [u64]
            > &
                QueryableStorageEntry<ApiType, [u64]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        dmp: {
            /** The factor to multiply the base delivery fee by. */
            deliveryFeeFactor: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<u128>, [u32]> &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * A mapping that stores the downward message queue MQC head for each para.
             *
             * Each link in this chain has a form: `(prev_head, B, H(M))`, where
             *
             * - `prev_head`: is the previous head hash or zero if none.
             * - `B`: is the relay-chain block number in which a message was appended.
             * - `H(M)`: is the hash of the message being appended.
             */
            downwardMessageQueueHeads: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<H256>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** The downward messages addressed for a certain para. */
            downwardMessageQueues: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<PolkadotCorePrimitivesInboundDownwardMessage>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        ethereumBeaconClient: {
            /** Sync committee for current period */
            currentSyncCommittee: AugmentedQuery<
                ApiType,
                () => Observable<SnowbridgeBeaconPrimitivesSyncCommitteePrepared>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** Beacon state by finalized block root */
            finalizedBeaconState: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<SnowbridgeBeaconPrimitivesCompactBeaconState>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /** Finalized Headers: Current position in ring buffer */
            finalizedBeaconStateIndex: AugmentedQuery<ApiType, () => Observable<u32>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Finalized Headers: Mapping of ring buffer index to a pruning candidate */
            finalizedBeaconStateMapping: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<H256>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Latest imported checkpoint root */
            initialCheckpointRoot: AugmentedQuery<ApiType, () => Observable<H256>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Latest imported finalized block root */
            latestFinalizedBlockRoot: AugmentedQuery<ApiType, () => Observable<H256>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** The last period where the next sync committee was updated for free. */
            latestSyncCommitteeUpdatePeriod: AugmentedQuery<ApiType, () => Observable<u64>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Sync committee for next period */
            nextSyncCommittee: AugmentedQuery<
                ApiType,
                () => Observable<SnowbridgeBeaconPrimitivesSyncCommitteePrepared>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** The current operating mode of the pallet. */
            operatingMode: AugmentedQuery<
                ApiType,
                () => Observable<SnowbridgeCoreOperatingModeBasicOperatingMode>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            validatorsRoot: AugmentedQuery<ApiType, () => Observable<H256>, []> & QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        ethereumInboundQueue: {
            /** The current nonce for each channel */
            nonce: AugmentedQuery<
                ApiType,
                (arg: SnowbridgeCoreChannelId | string | Uint8Array) => Observable<u64>,
                [SnowbridgeCoreChannelId]
            > &
                QueryableStorageEntry<ApiType, [SnowbridgeCoreChannelId]>;
            /** The current operating mode of the pallet. */
            operatingMode: AugmentedQuery<
                ApiType,
                () => Observable<SnowbridgeCoreOperatingModeBasicOperatingMode>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        ethereumOutboundQueue: {
            /**
             * Hashes of the ABI-encoded messages in the [`Messages`] storage value. Used to generate a merkle root during
             * `on_finalize`. This storage value is killed in `on_initialize`, so should never go into block PoV.
             */
            messageLeaves: AugmentedQuery<ApiType, () => Observable<Vec<H256>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Messages to be committed in the current block. This storage value is killed in `on_initialize`, so should never
             * go into block PoV.
             *
             * Is never read in the runtime, only by offchain message relayers.
             *
             * Inspired by the `frame_system::Pallet::Events` storage value
             */
            messages: AugmentedQuery<
                ApiType,
                () => Observable<Vec<SnowbridgePalletOutboundQueueCommittedMessage>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** The current nonce for each message origin */
            nonce: AugmentedQuery<
                ApiType,
                (arg: SnowbridgeCoreChannelId | string | Uint8Array) => Observable<u64>,
                [SnowbridgeCoreChannelId]
            > &
                QueryableStorageEntry<ApiType, [SnowbridgeCoreChannelId]>;
            /** The current operating mode of the pallet. */
            operatingMode: AugmentedQuery<
                ApiType,
                () => Observable<SnowbridgeCoreOperatingModeBasicOperatingMode>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        ethereumSystem: {
            /** The set of registered agents */
            agents: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<Option<Null>>, [H256]> &
                QueryableStorageEntry<ApiType, [H256]>;
            /** The set of registered channels */
            channels: AugmentedQuery<
                ApiType,
                (arg: SnowbridgeCoreChannelId | string | Uint8Array) => Observable<Option<SnowbridgeCoreChannel>>,
                [SnowbridgeCoreChannelId]
            > &
                QueryableStorageEntry<ApiType, [SnowbridgeCoreChannelId]>;
            /** Lookup table for foreign token ID to native location relative to ethereum */
            foreignToNativeId: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<StagingXcmV4Location>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /** Lookup table for native location relative to ethereum to foreign token ID */
            nativeToForeignId: AugmentedQuery<
                ApiType,
                (
                    arg: StagingXcmV4Location | { parents?: any; interior?: any } | string | Uint8Array
                ) => Observable<Option<H256>>,
                [StagingXcmV4Location]
            > &
                QueryableStorageEntry<ApiType, [StagingXcmV4Location]>;
            pricingParameters: AugmentedQuery<ApiType, () => Observable<SnowbridgeCorePricingPricingParameters>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        externalValidators: {
            /** The active era information, it holds index and start. */
            activeEra: AugmentedQuery<ApiType, () => Observable<Option<TpTraitsActiveEraInfo>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The current era information, it is either ActiveEra or ActiveEra + 1 if the new era validators have been
             * queued.
             */
            currentEra: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The session index at which the era start for the last [`Config::HistoryDepth`] eras.
             *
             * Note: This tracks the starting session (i.e. session index when era start being active) for the eras in
             * `[CurrentEra - HISTORY_DEPTH, CurrentEra]`.
             */
            erasStartSessionIndex: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<u32>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Validators set using storage proofs from another blockchain. Ignored if `SkipExternalValidators` is true. */
            externalValidators: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Mode of era forcing. */
            forceEra: AugmentedQuery<ApiType, () => Observable<PalletExternalValidatorsForcing>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Allow to disable external validators. */
            skipExternalValidators: AugmentedQuery<ApiType, () => Observable<bool>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Fixed validators set by root/governance. Have priority over the external validators. */
            whitelistedValidators: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Copy of `WhitelistedValidators` at the start of this active era. Used to check which validators we don't need
             * to reward.
             */
            whitelistedValidatorsActiveEra: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Same as `WhitelistedValidatorsActiveEra` but only exists for a brief period of time when the next era has been
             * planned but not enacted yet.
             */
            whitelistedValidatorsActiveEraPending: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        externalValidatorSlashes: {
            /**
             * A mapping from still-bonded eras to the first session index of that era.
             *
             * Must contains information for eras for the range: `[active_era - bounding_duration; active_era]`
             */
            bondedEras: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[u32, u32]>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** A counter on the number of slashes we have performed */
            nextSlashId: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /** All unapplied slashes that are queued for later. */
            slashes: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<PalletExternalValidatorSlashesSlash>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** All slashing events on validators, mapped by era to the highest slash proportion and slash value of the era. */
            validatorSlashInEra: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: AccountId32 | string | Uint8Array
                ) => Observable<Option<Perbill>>,
                [u32, AccountId32]
            > &
                QueryableStorageEntry<ApiType, [u32, AccountId32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        externalValidatorsRewards: {
            /** Store reward points per era. Note: EraRewardPoints is actually bounded by the amount of validators. */
            rewardPointsForEra: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<PalletExternalValidatorsRewardsEraRewardPoints>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        fellowshipCollective: {
            /** The index of each ranks's member into the group of members who have at least that rank. */
            idToIndex: AugmentedQuery<
                ApiType,
                (
                    arg1: u16 | AnyNumber | Uint8Array,
                    arg2: AccountId32 | string | Uint8Array
                ) => Observable<Option<u32>>,
                [u16, AccountId32]
            > &
                QueryableStorageEntry<ApiType, [u16, AccountId32]>;
            /**
             * The members in the collective by index. All indices in the range `0..MemberCount` will return `Some`, however a
             * member's index is not guaranteed to remain unchanged over time.
             */
            indexToId: AugmentedQuery<
                ApiType,
                (
                    arg1: u16 | AnyNumber | Uint8Array,
                    arg2: u32 | AnyNumber | Uint8Array
                ) => Observable<Option<AccountId32>>,
                [u16, u32]
            > &
                QueryableStorageEntry<ApiType, [u16, u32]>;
            /** The number of members in the collective who have at least the rank according to the index of the vec. */
            memberCount: AugmentedQuery<ApiType, (arg: u16 | AnyNumber | Uint8Array) => Observable<u32>, [u16]> &
                QueryableStorageEntry<ApiType, [u16]>;
            /** The current members of the collective. */
            members: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<PalletRankedCollectiveMemberRecord>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /** Votes on a given proposal, if it is ongoing. */
            voting: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: AccountId32 | string | Uint8Array
                ) => Observable<Option<PalletRankedCollectiveVoteRecord>>,
                [u32, AccountId32]
            > &
                QueryableStorageEntry<ApiType, [u32, AccountId32]>;
            votingCleanup: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<Bytes>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        fellowshipReferenda: {
            /** The number of referenda being decided currently. */
            decidingCount: AugmentedQuery<ApiType, (arg: u16 | AnyNumber | Uint8Array) => Observable<u32>, [u16]> &
                QueryableStorageEntry<ApiType, [u16]>;
            /**
             * The metadata is a general information concerning the referendum. The `Hash` refers to the preimage of the
             * `Preimages` provider which can be a JSON dump or IPFS hash of a JSON file.
             *
             * Consider a garbage collection for a metadata of finished referendums to `unrequest` (remove) large preimages.
             */
            metadataOf: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<H256>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** The next free referendum index, aka the number of referenda started so far. */
            referendumCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /** Information concerning any given referendum. */
            referendumInfoFor: AugmentedQuery<
                ApiType,
                (
                    arg: u32 | AnyNumber | Uint8Array
                ) => Observable<Option<PalletReferendaReferendumInfoRankedCollectiveTally>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * The sorted list of referenda ready to be decided but not yet being decided, ordered by conviction-weighted
             * approvals.
             *
             * This should be empty if `DecidingCount` is less than `TrackInfo::max_deciding`.
             */
            trackQueue: AugmentedQuery<
                ApiType,
                (arg: u16 | AnyNumber | Uint8Array) => Observable<Vec<ITuple<[u32, u32]>>>,
                [u16]
            > &
                QueryableStorageEntry<ApiType, [u16]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        grandpa: {
            /** The current list of authorities. */
            authorities: AugmentedQuery<
                ApiType,
                () => Observable<Vec<ITuple<[SpConsensusGrandpaAppPublic, u64]>>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The number of changes (both in terms of keys and underlying economic responsibilities) in the "set" of Grandpa
             * validators from genesis.
             */
            currentSetId: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /** Next block number where we can force a change. */
            nextForced: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> & QueryableStorageEntry<ApiType, []>;
            /** Pending change: (signaled at, scheduled change). */
            pendingChange: AugmentedQuery<ApiType, () => Observable<Option<PalletGrandpaStoredPendingChange>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * A mapping from grandpa set ID to the index of the _most recent_ session for which its members were responsible.
             *
             * This is only used for validating equivocation proofs. An equivocation proof must contains a key-ownership proof
             * for a given session, therefore we need a way to tie together sessions and GRANDPA set ids, i.e. we need to
             * validate that a validator was the owner of a given key on a given session, and what the active set ID was
             * during that session.
             *
             * TWOX-NOTE: `SetId` is not under user control.
             */
            setIdSession: AugmentedQuery<
                ApiType,
                (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<u32>>,
                [u64]
            > &
                QueryableStorageEntry<ApiType, [u64]>;
            /** `true` if we are currently stalled. */
            stalled: AugmentedQuery<ApiType, () => Observable<Option<ITuple<[u32, u32]>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** State of the current authority set. */
            state: AugmentedQuery<ApiType, () => Observable<PalletGrandpaStoredState>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        historical: {
            /** Mapping from historical session indices to session-data root hash and validator count. */
            historicalSessions: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<ITuple<[H256, u32]>>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** The range of historical sessions we store. [first, last) */
            storedRange: AugmentedQuery<ApiType, () => Observable<Option<ITuple<[u32, u32]>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        hrmp: {
            /**
             * This mapping tracks how many open channel requests were accepted by a given recipient para. Invariant:
             * `HrmpOpenChannelRequests` should contain the same number of items `(_, X)` with `confirmed` set to true, as the
             * number of `HrmpAcceptedChannelRequestCount` for `X`.
             */
            hrmpAcceptedChannelRequestCount: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<u32>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Storage for the messages for each channel. Invariant: cannot be non-empty if the corresponding channel in
             * `HrmpChannels` is `None`.
             */
            hrmpChannelContents: AugmentedQuery<
                ApiType,
                (
                    arg:
                        | PolkadotParachainPrimitivesPrimitivesHrmpChannelId
                        | { sender?: any; recipient?: any }
                        | string
                        | Uint8Array
                ) => Observable<Vec<PolkadotCorePrimitivesInboundHrmpMessage>>,
                [PolkadotParachainPrimitivesPrimitivesHrmpChannelId]
            > &
                QueryableStorageEntry<ApiType, [PolkadotParachainPrimitivesPrimitivesHrmpChannelId]>;
            /**
             * Maintains a mapping that can be used to answer the question: What paras sent a message at the given block
             * number for a given receiver. Invariants:
             *
             * - The inner `Vec<ParaId>` is never empty.
             * - The inner `Vec<ParaId>` cannot store two same `ParaId`.
             * - The outer vector is sorted ascending by block number and cannot store two items with the same block number.
             */
            hrmpChannelDigests: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<ITuple<[u32, Vec<u32>]>>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * HRMP channel data associated with each para. Invariant:
             *
             * - Each participant in the channel should satisfy `Paras::is_valid_para(P)` within a session.
             */
            hrmpChannels: AugmentedQuery<
                ApiType,
                (
                    arg:
                        | PolkadotParachainPrimitivesPrimitivesHrmpChannelId
                        | { sender?: any; recipient?: any }
                        | string
                        | Uint8Array
                ) => Observable<Option<PolkadotRuntimeParachainsHrmpHrmpChannel>>,
                [PolkadotParachainPrimitivesPrimitivesHrmpChannelId]
            > &
                QueryableStorageEntry<ApiType, [PolkadotParachainPrimitivesPrimitivesHrmpChannelId]>;
            /**
             * A set of pending HRMP close channel requests that are going to be closed during the session change. Used for
             * checking if a given channel is registered for closure.
             *
             * The set is accompanied by a list for iteration.
             *
             * Invariant:
             *
             * - There are no channels that exists in list but not in the set and vice versa.
             */
            hrmpCloseChannelRequests: AugmentedQuery<
                ApiType,
                (
                    arg:
                        | PolkadotParachainPrimitivesPrimitivesHrmpChannelId
                        | { sender?: any; recipient?: any }
                        | string
                        | Uint8Array
                ) => Observable<Option<Null>>,
                [PolkadotParachainPrimitivesPrimitivesHrmpChannelId]
            > &
                QueryableStorageEntry<ApiType, [PolkadotParachainPrimitivesPrimitivesHrmpChannelId]>;
            hrmpCloseChannelRequestsList: AugmentedQuery<
                ApiType,
                () => Observable<Vec<PolkadotParachainPrimitivesPrimitivesHrmpChannelId>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            hrmpEgressChannelsIndex: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<u32>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Ingress/egress indexes allow to find all the senders and receivers given the opposite side. I.e.
             *
             * (a) ingress index allows to find all the senders for a given recipient. (b) egress index allows to find all the
             * recipients for a given sender.
             *
             * Invariants:
             *
             * - For each ingress index entry for `P` each item `I` in the index should present in `HrmpChannels` as `(I, P)`.
             * - For each egress index entry for `P` each item `E` in the index should present in `HrmpChannels` as `(P, E)`.
             * - There should be no other dangling channels in `HrmpChannels`.
             * - The vectors are sorted.
             */
            hrmpIngressChannelsIndex: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<u32>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * This mapping tracks how many open channel requests are initiated by a given sender para. Invariant:
             * `HrmpOpenChannelRequests` should contain the same number of items that has `(X, _)` as the number of
             * `HrmpOpenChannelRequestCount` for `X`.
             */
            hrmpOpenChannelRequestCount: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<u32>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * The set of pending HRMP open channel requests.
             *
             * The set is accompanied by a list for iteration.
             *
             * Invariant:
             *
             * - There are no channels that exists in list but not in the set and vice versa.
             */
            hrmpOpenChannelRequests: AugmentedQuery<
                ApiType,
                (
                    arg:
                        | PolkadotParachainPrimitivesPrimitivesHrmpChannelId
                        | { sender?: any; recipient?: any }
                        | string
                        | Uint8Array
                ) => Observable<Option<PolkadotRuntimeParachainsHrmpHrmpOpenChannelRequest>>,
                [PolkadotParachainPrimitivesPrimitivesHrmpChannelId]
            > &
                QueryableStorageEntry<ApiType, [PolkadotParachainPrimitivesPrimitivesHrmpChannelId]>;
            hrmpOpenChannelRequestsList: AugmentedQuery<
                ApiType,
                () => Observable<Vec<PolkadotParachainPrimitivesPrimitivesHrmpChannelId>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The HRMP watermark associated with each para. Invariant:
             *
             * - Each para `P` used here as a key should satisfy `Paras::is_valid_para(P)` within a session.
             */
            hrmpWatermarks: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<u32>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        identity: {
            /**
             * Reverse lookup from `username` to the `AccountId` that has registered it. The value should be a key in the
             * `IdentityOf` map, but it may not if the user has cleared their identity.
             *
             * Multiple usernames may map to the same `AccountId`, but `IdentityOf` will only map to one primary username.
             */
            accountOfUsername: AugmentedQuery<
                ApiType,
                (arg: Bytes | string | Uint8Array) => Observable<Option<AccountId32>>,
                [Bytes]
            > &
                QueryableStorageEntry<ApiType, [Bytes]>;
            /**
             * Information that is pertinent to identify the entity behind an account. First item is the registration, second
             * is the account's primary username.
             *
             * TWOX-NOTE: OK ― `AccountId` is a secure hash.
             */
            identityOf: AugmentedQuery<
                ApiType,
                (
                    arg: AccountId32 | string | Uint8Array
                ) => Observable<Option<ITuple<[PalletIdentityRegistration, Option<Bytes>]>>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Usernames that an authority has granted, but that the account controller has not confirmed that they want it.
             * Used primarily in cases where the `AccountId` cannot provide a signature because they are a pure proxy,
             * multisig, etc. In order to confirm it, they should call [`Call::accept_username`].
             *
             * First tuple item is the account and second is the acceptance deadline.
             */
            pendingUsernames: AugmentedQuery<
                ApiType,
                (arg: Bytes | string | Uint8Array) => Observable<Option<ITuple<[AccountId32, u32]>>>,
                [Bytes]
            > &
                QueryableStorageEntry<ApiType, [Bytes]>;
            /**
             * The set of registrars. Not expected to get very big as can only be added through a special origin (likely a
             * council motion).
             *
             * The index into this can be cast to `RegistrarIndex` to get a valid value.
             */
            registrars: AugmentedQuery<ApiType, () => Observable<Vec<Option<PalletIdentityRegistrarInfo>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Alternative "sub" identities of this account.
             *
             * The first item is the deposit, the second is a vector of the accounts.
             *
             * TWOX-NOTE: OK ― `AccountId` is a secure hash.
             */
            subsOf: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<ITuple<[u128, Vec<AccountId32>]>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * The super-identity of an alternative "sub" identity together with its name, within that context. If the account
             * is not some other account's sub-identity, then just `None`.
             */
            superOf: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<ITuple<[AccountId32, Data]>>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /** A map of the accounts who are authorized to grant usernames. */
            usernameAuthorities: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<PalletIdentityAuthorityProperties>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        inflationRewards: {
            /** Container chains to reward per block */
            chainsToReward: AugmentedQuery<
                ApiType,
                () => Observable<Option<PalletInflationRewardsChainsToRewardValue>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        initializer: {
            /**
             * Buffered session changes along with the block number at which they should be applied.
             *
             * Typically this will be empty or one element long. Apart from that this item never hits the storage.
             *
             * However this is a `Vec` regardless to handle various edge cases that may occur at runtime upgrade boundaries or
             * if governance intervenes.
             */
            bufferedSessionChanges: AugmentedQuery<
                ApiType,
                () => Observable<Vec<PolkadotRuntimeParachainsInitializerBufferedSessionChange>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Whether the parachains modules have been initialized within this block.
             *
             * Semantically a `bool`, but this guarantees it should never hit the trie, as this is cleared in `on_finalize`
             * and Frame optimizes `None` values to be empty values.
             *
             * As a `bool`, `set(false)` and `remove()` both lead to the next `get()` being false, but one of them writes to
             * the trie and one does not. This confusion makes `Option<()>` more suitable for the semantics of this variable.
             */
            hasInitialized: AugmentedQuery<ApiType, () => Observable<Option<Null>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        messageQueue: {
            /** The index of the first and last (non-empty) pages. */
            bookStateFor: AugmentedQuery<
                ApiType,
                (
                    arg:
                        | DancelightRuntimeAggregateMessageOrigin
                        | { Ump: any }
                        | { Snowbridge: any }
                        | { SnowbridgeTanssi: any }
                        | string
                        | Uint8Array
                ) => Observable<PalletMessageQueueBookState>,
                [DancelightRuntimeAggregateMessageOrigin]
            > &
                QueryableStorageEntry<ApiType, [DancelightRuntimeAggregateMessageOrigin]>;
            /** The map of page indices to pages. */
            pages: AugmentedQuery<
                ApiType,
                (
                    arg1:
                        | DancelightRuntimeAggregateMessageOrigin
                        | { Ump: any }
                        | { Snowbridge: any }
                        | { SnowbridgeTanssi: any }
                        | string
                        | Uint8Array,
                    arg2: u32 | AnyNumber | Uint8Array
                ) => Observable<Option<PalletMessageQueuePage>>,
                [DancelightRuntimeAggregateMessageOrigin, u32]
            > &
                QueryableStorageEntry<ApiType, [DancelightRuntimeAggregateMessageOrigin, u32]>;
            /** The origin at which we should begin servicing. */
            serviceHead: AugmentedQuery<
                ApiType,
                () => Observable<Option<DancelightRuntimeAggregateMessageOrigin>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        migrations: {
            /** True if all required migrations have completed */
            fullyUpgraded: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * MigrationState tracks the progress of a migration. Maps name (Vec<u8>) -> whether or not migration has been
             * completed (bool)
             */
            migrationState: AugmentedQuery<ApiType, (arg: Bytes | string | Uint8Array) => Observable<bool>, [Bytes]> &
                QueryableStorageEntry<ApiType, [Bytes]>;
            /**
             * Temporary value that is set to true at the beginning of the block during which the execution of xcm messages
             * must be paused.
             */
            shouldPauseXcm: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        mmr: {
            /**
             * Hashes of the nodes in the MMR.
             *
             * Note this collection only contains MMR peaks, the inner nodes (and leaves) are pruned and only stored in the
             * Offchain DB.
             */
            nodes: AugmentedQuery<ApiType, (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<H256>>, [u64]> &
                QueryableStorageEntry<ApiType, [u64]>;
            /** Current size of the MMR (number of leaves). */
            numberOfLeaves: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /** Latest MMR Root hash. */
            rootHash: AugmentedQuery<ApiType, () => Observable<H256>, []> & QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        multiBlockMigrations: {
            /**
             * The currently active migration to run and its cursor.
             *
             * `None` indicates that no migration is running.
             */
            cursor: AugmentedQuery<ApiType, () => Observable<Option<PalletMigrationsMigrationCursor>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Set of all successfully executed migrations.
             *
             * This is used as blacklist, to not re-execute migrations that have not been removed from the codebase yet.
             * Governance can regularly clear this out via `clear_historic`.
             */
            historic: AugmentedQuery<ApiType, (arg: Bytes | string | Uint8Array) => Observable<Option<Null>>, [Bytes]> &
                QueryableStorageEntry<ApiType, [Bytes]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        multisig: {
            /** The set of open multisig operations. */
            multisigs: AugmentedQuery<
                ApiType,
                (
                    arg1: AccountId32 | string | Uint8Array,
                    arg2: U8aFixed | string | Uint8Array
                ) => Observable<Option<PalletMultisigMultisig>>,
                [AccountId32, U8aFixed]
            > &
                QueryableStorageEntry<ApiType, [AccountId32, U8aFixed]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        offences: {
            /** A vector of reports of the same kind that happened at the same time slot. */
            concurrentReportsIndex: AugmentedQuery<
                ApiType,
                (arg1: U8aFixed | string | Uint8Array, arg2: Bytes | string | Uint8Array) => Observable<Vec<H256>>,
                [U8aFixed, Bytes]
            > &
                QueryableStorageEntry<ApiType, [U8aFixed, Bytes]>;
            /** The primary structure that holds all offence records keyed by report identifiers. */
            reports: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<SpStakingOffenceOffenceDetails>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        onDemandAssignmentProvider: {
            /** Queue entries that are currently bound to a particular core due to core affinity. */
            affinityEntries: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<BinaryHeapEnqueuedOrder>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Priority queue for all orders which don't yet (or not any more) have any core affinity. */
            freeEntries: AugmentedQuery<
                ApiType,
                () => Observable<Vec<PolkadotRuntimeParachainsOnDemandTypesEnqueuedOrder>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Maps a `ParaId` to `CoreIndex` and keeps track of how many assignments the scheduler has in it's lookahead.
             * Keeping track of this affinity prevents parallel execution of the same `ParaId` on two or more `CoreIndex`es.
             */
            paraIdAffinity: AugmentedQuery<
                ApiType,
                (
                    arg: u32 | AnyNumber | Uint8Array
                ) => Observable<Option<PolkadotRuntimeParachainsOnDemandTypesCoreAffinityCount>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Overall status of queue (both free + affinity entries) */
            queueStatus: AugmentedQuery<
                ApiType,
                () => Observable<PolkadotRuntimeParachainsOnDemandTypesQueueStatusType>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** Keeps track of accumulated revenue from on demand order sales. */
            revenue: AugmentedQuery<ApiType, () => Observable<Vec<u128>>, []> & QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        paraInclusion: {
            /**
             * Candidates pending availability by `ParaId`. They form a chain starting from the latest included head of the
             * para. Use a different prefix post-migration to v1, since the v0 `PendingAvailability` storage would otherwise
             * have the exact same prefix which could cause undefined behaviour when doing the migration.
             */
            v1: AugmentedQuery<
                ApiType,
                (
                    arg: u32 | AnyNumber | Uint8Array
                ) => Observable<Option<Vec<PolkadotRuntimeParachainsInclusionCandidatePendingAvailability>>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        paraInherent: {
            /**
             * Whether the paras inherent was included within this block.
             *
             * The `Option<()>` is effectively a `bool`, but it never hits storage in the `None` variant due to the guarantees
             * of FRAME's storage APIs.
             *
             * If this is `None` at the end of the block, we panic and render the block invalid.
             */
            included: AugmentedQuery<ApiType, () => Observable<Option<Null>>, []> & QueryableStorageEntry<ApiType, []>;
            /** Scraped on chain data for extracting resolved disputes as well as backing votes. */
            onChainVotes: AugmentedQuery<
                ApiType,
                () => Observable<Option<PolkadotPrimitivesV8ScrapedOnChainVotes>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        parameters: {
            /** Stored parameters. */
            parameters: AugmentedQuery<
                ApiType,
                (
                    arg: DancelightRuntimeRuntimeParametersKey | { Preimage: any } | string | Uint8Array
                ) => Observable<Option<DancelightRuntimeRuntimeParametersValue>>,
                [DancelightRuntimeRuntimeParametersKey]
            > &
                QueryableStorageEntry<ApiType, [DancelightRuntimeRuntimeParametersKey]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        paras: {
            /** The actions to perform during the start of a specific session index. */
            actionsQueue: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<u32>>, [u32]> &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Validation code stored by its hash.
             *
             * This storage is consistent with [`FutureCodeHash`], [`CurrentCodeHash`] and [`PastCodeHash`].
             */
            codeByHash: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<Bytes>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /** The number of reference on the validation code in [`CodeByHash`] storage. */
            codeByHashRefs: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<u32>, [H256]> &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * The validation code hash of every live para.
             *
             * Corresponding code can be retrieved with [`CodeByHash`].
             */
            currentCodeHash: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<H256>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * The actual future code hash of a para.
             *
             * Corresponding code can be retrieved with [`CodeByHash`].
             */
            futureCodeHash: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<H256>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * The block number at which the planned code change is expected for a parachain.
             *
             * The change will be applied after the first parablock for this ID included which executes in the context of a
             * relay chain block with a number >= `expected_at`.
             */
            futureCodeUpgrades: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<u32>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * The list of upcoming future code upgrades.
             *
             * Each item is a pair of the parachain and the expected block at which the upgrade should be applied. The upgrade
             * will be applied at the given relay chain block. In contrast to [`FutureCodeUpgrades`] this code upgrade will be
             * applied regardless the parachain making any progress or not.
             *
             * Ordered ascending by block number.
             */
            futureCodeUpgradesAt: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[u32, u32]>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** The head-data of every registered para. */
            heads: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<Bytes>>, [u32]> &
                QueryableStorageEntry<ApiType, [u32]>;
            /** The context (relay-chain block number) of the most recent parachain head. */
            mostRecentContext: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<u32>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * All lease holding parachains. Ordered ascending by `ParaId`. On demand parachains are not included.
             *
             * Consider using the [`ParachainsCache`] type of modifying.
             */
            parachains: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> & QueryableStorageEntry<ApiType, []>;
            /** The current lifecycle of a all known Para IDs. */
            paraLifecycles: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotRuntimeParachainsParasParaLifecycle>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Actual past code hash, indicated by the para id as well as the block number at which it became outdated.
             *
             * Corresponding code can be retrieved with [`CodeByHash`].
             */
            pastCodeHash: AugmentedQuery<
                ApiType,
                (
                    arg: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array]
                ) => Observable<Option<H256>>,
                [ITuple<[u32, u32]>]
            > &
                QueryableStorageEntry<ApiType, [ITuple<[u32, u32]>]>;
            /**
             * Past code of parachains. The parachains themselves may not be registered anymore, but we also keep their code
             * on-chain for the same amount of time as outdated code to keep it available for approval checkers.
             */
            pastCodeMeta: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<PolkadotRuntimeParachainsParasParaPastCodeMeta>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Which paras have past code that needs pruning and the relay-chain block at which the code was replaced. Note
             * that this is the actual height of the included block, not the expected height at which the code upgrade would
             * be applied, although they may be equal. This is to ensure the entire acceptance period is covered, not an
             * offset acceptance period starting from the time at which the parachain perceives a code upgrade as having
             * occurred. Multiple entries for a single para are permitted. Ordered ascending by block number.
             */
            pastCodePruning: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[u32, u32]>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** The list of all currently active PVF votes. Auxiliary to `PvfActiveVoteMap`. */
            pvfActiveVoteList: AugmentedQuery<ApiType, () => Observable<Vec<H256>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * All currently active PVF pre-checking votes.
             *
             * Invariant:
             *
             * - There are no PVF pre-checking votes that exists in list but not in the set and vice versa.
             */
            pvfActiveVoteMap: AugmentedQuery<
                ApiType,
                (
                    arg: H256 | string | Uint8Array
                ) => Observable<Option<PolkadotRuntimeParachainsParasPvfCheckActiveVoteState>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * Upcoming paras instantiation arguments.
             *
             * NOTE that after PVF pre-checking is enabled the para genesis arg will have it's code set to empty. Instead, the
             * code will be saved into the storage right away via `CodeByHash`.
             */
            upcomingParasGenesis: AugmentedQuery<
                ApiType,
                (
                    arg: u32 | AnyNumber | Uint8Array
                ) => Observable<Option<PolkadotRuntimeParachainsParasParaGenesisArgs>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * The list of upcoming code upgrades.
             *
             * Each item is a pair of which para performs a code upgrade and at which relay-chain block it is expected at.
             *
             * Ordered ascending by block number.
             */
            upcomingUpgrades: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[u32, u32]>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The list of parachains that are awaiting for their upgrade restriction to cooldown.
             *
             * Ordered ascending by block number.
             */
            upgradeCooldowns: AugmentedQuery<ApiType, () => Observable<Vec<ITuple<[u32, u32]>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * This is used by the relay-chain to communicate to a parachain a go-ahead with in the upgrade procedure.
             *
             * This value is absent when there are no upgrades scheduled or during the time the relay chain performs the
             * checks. It is set at the first relay-chain block when the corresponding parachain can switch its upgrade
             * function. As soon as the parachain's block is included, the value gets reset to `None`.
             *
             * NOTE that this field is used by parachains via merkle storage proofs, therefore changing the format will
             * require migration of parachains.
             */
            upgradeGoAheadSignal: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotPrimitivesV8UpgradeGoAhead>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * This is used by the relay-chain to communicate that there are restrictions for performing an upgrade for this
             * parachain.
             *
             * This may be a because the parachain waits for the upgrade cooldown to expire. Another potential use case is
             * when we want to perform some maintenance (such as storage migration) we could restrict upgrades to make the
             * process simpler.
             *
             * NOTE that this field is used by parachains via merkle storage proofs, therefore changing the format will
             * require migration of parachains.
             */
            upgradeRestrictionSignal: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotPrimitivesV8UpgradeRestriction>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        paraScheduler: {
            /**
             * One entry for each availability core. The i'th parachain belongs to the i'th core, with the remaining cores all
             * being on demand parachain multiplexers.
             *
             * Bounded by the maximum of either of these two values:
             *
             * - The number of parachains and parathread multiplexers
             * - The number of validators divided by `configuration.max_validators_per_core`.
             */
            availabilityCores: AugmentedQuery<
                ApiType,
                () => Observable<Vec<PolkadotRuntimeParachainsSchedulerPalletCoreOccupied>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * One entry for each availability core. The `VecDeque` represents the assignments to be scheduled on that core.
             * The value contained here will not be valid after the end of a block. Runtime APIs should be used to determine
             * scheduled cores for the upcoming block.
             */
            claimQueue: AugmentedQuery<
                ApiType,
                () => Observable<BTreeMap<u32, Vec<PolkadotRuntimeParachainsSchedulerPalletParasEntry>>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The block number where the session start occurred. Used to track how many group rotations have occurred.
             *
             * Note that in the context of parachains modules the session change is signaled during the block and enacted at
             * the end of the block (at the finalization stage, to be exact). Thus for all intents and purposes the effect of
             * the session change is observed at the block following the session change, block number of which we save in this
             * storage value.
             */
            sessionStartBlock: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * All the validator groups. One for each core. Indices are into `ActiveValidators` - not the broader set of
             * Polkadot validators, but instead just the subset used for parachains during this session.
             *
             * Bound: The number of cores is the sum of the numbers of parachains and parathread multiplexers. Reasonably,
             * 100-1000. The dominant factor is the number of validators: safe upper bound at 10k.
             */
            validatorGroups: AugmentedQuery<ApiType, () => Observable<Vec<Vec<u32>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        parasDisputes: {
            /** Backing votes stored for each dispute. This storage is used for slashing. */
            backersOnDisputes: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: H256 | string | Uint8Array
                ) => Observable<Option<BTreeSet<u32>>>,
                [u32, H256]
            > &
                QueryableStorageEntry<ApiType, [u32, H256]>;
            /** All ongoing or concluded disputes for the last several sessions. */
            disputes: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: H256 | string | Uint8Array
                ) => Observable<Option<PolkadotPrimitivesV8DisputeState>>,
                [u32, H256]
            > &
                QueryableStorageEntry<ApiType, [u32, H256]>;
            /**
             * Whether the chain is frozen. Starts as `None`. When this is `Some`, the chain will not accept any new parachain
             * blocks for backing or inclusion, and its value indicates the last valid block number in the chain. It can only
             * be set back to `None` by governance intervention.
             */
            frozen: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * All included blocks on the chain, as well as the block number in this chain that should be reverted back to if
             * the candidate is disputed and determined to be invalid.
             */
            included: AugmentedQuery<
                ApiType,
                (arg1: u32 | AnyNumber | Uint8Array, arg2: H256 | string | Uint8Array) => Observable<Option<u32>>,
                [u32, H256]
            > &
                QueryableStorageEntry<ApiType, [u32, H256]>;
            /** The last pruned session, if any. All data stored by this module references sessions. */
            lastPrunedSession: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        paraSessionInfo: {
            /** The validator account keys of the validators actively participating in parachain consensus. */
            accountKeys: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<Vec<AccountId32>>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Assignment keys for the current session. Note that this API is private due to it being prone to 'off-by-one' at
             * session boundaries. When in doubt, use `Sessions` API instead.
             */
            assignmentKeysUnsafe: AugmentedQuery<
                ApiType,
                () => Observable<Vec<PolkadotPrimitivesV8AssignmentAppPublic>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** The earliest session for which previous session info is stored. */
            earliestStoredSession: AugmentedQuery<ApiType, () => Observable<u32>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Executor parameter set for a given session index */
            sessionExecutorParams: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotPrimitivesV8ExecutorParams>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Session information in a rolling window. Should have an entry in range
             * `EarliestStoredSession..=CurrentSessionIndex`. Does not have any entries before the session index in the first
             * session change notification.
             */
            sessions: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotPrimitivesV8SessionInfo>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        parasShared: {
            /** All the validators actively participating in parachain consensus. Indices are into the broader validator set. */
            activeValidatorIndices: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The parachain attestation keys of the validators actively participating in parachain consensus. This should be
             * the same length as `ActiveValidatorIndices`.
             */
            activeValidatorKeys: AugmentedQuery<
                ApiType,
                () => Observable<Vec<PolkadotPrimitivesV8ValidatorAppPublic>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** All allowed relay-parents. */
            allowedRelayParents: AugmentedQuery<
                ApiType,
                () => Observable<PolkadotRuntimeParachainsSharedAllowedRelayParentsTracker>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** The current session index. */
            currentSessionIndex: AugmentedQuery<ApiType, () => Observable<u32>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        parasSlashing: {
            /** Validators pending dispute slashes. */
            unappliedSlashes: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: H256 | string | Uint8Array
                ) => Observable<Option<PolkadotPrimitivesV8SlashingPendingSlashes>>,
                [u32, H256]
            > &
                QueryableStorageEntry<ApiType, [u32, H256]>;
            /** `ValidatorSetCount` per session. */
            validatorSetCounts: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<u32>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        pooledStaking: {
            /** Pending operations balances. Balances are expressed in joining/leaving shares amounts. */
            pendingOperations: AugmentedQuery<
                ApiType,
                (
                    arg1: AccountId32 | string | Uint8Array,
                    arg2:
                        | PalletPooledStakingPendingOperationKey
                        | { JoiningAutoCompounding: any }
                        | { JoiningManualRewards: any }
                        | { Leaving: any }
                        | string
                        | Uint8Array
                ) => Observable<u128>,
                [AccountId32, PalletPooledStakingPendingOperationKey]
            > &
                QueryableStorageEntry<ApiType, [AccountId32, PalletPooledStakingPendingOperationKey]>;
            /** Pools balances. */
            pools: AugmentedQuery<
                ApiType,
                (
                    arg1: AccountId32 | string | Uint8Array,
                    arg2:
                        | PalletPooledStakingPoolsKey
                        | { CandidateTotalStake: any }
                        | { JoiningShares: any }
                        | { JoiningSharesSupply: any }
                        | { JoiningSharesTotalStaked: any }
                        | { JoiningSharesHeldStake: any }
                        | { AutoCompoundingShares: any }
                        | { AutoCompoundingSharesSupply: any }
                        | { AutoCompoundingSharesTotalStaked: any }
                        | { AutoCompoundingSharesHeldStake: any }
                        | { ManualRewardsShares: any }
                        | { ManualRewardsSharesSupply: any }
                        | { ManualRewardsSharesTotalStaked: any }
                        | { ManualRewardsSharesHeldStake: any }
                        | { ManualRewardsCounter: any }
                        | { ManualRewardsCheckpoint: any }
                        | { LeavingShares: any }
                        | { LeavingSharesSupply: any }
                        | { LeavingSharesTotalStaked: any }
                        | { LeavingSharesHeldStake: any }
                        | string
                        | Uint8Array
                ) => Observable<u128>,
                [AccountId32, PalletPooledStakingPoolsKey]
            > &
                QueryableStorageEntry<ApiType, [AccountId32, PalletPooledStakingPoolsKey]>;
            /**
             * Keeps a list of all eligible candidates, sorted by the amount of stake backing them. This can be quickly
             * updated using a binary search, and allow to easily take the top `MaxCollatorSetSize`.
             */
            sortedEligibleCandidates: AugmentedQuery<
                ApiType,
                () => Observable<Vec<PalletPooledStakingCandidateEligibleCandidate>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        preimage: {
            preimageFor: AugmentedQuery<
                ApiType,
                (
                    arg: ITuple<[H256, u32]> | [H256 | string | Uint8Array, u32 | AnyNumber | Uint8Array]
                ) => Observable<Option<Bytes>>,
                [ITuple<[H256, u32]>]
            > &
                QueryableStorageEntry<ApiType, [ITuple<[H256, u32]>]>;
            /** The request status of a given hash. */
            requestStatusFor: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<PalletPreimageRequestStatus>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /** The request status of a given hash. */
            statusFor: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<PalletPreimageOldRequestStatus>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        proxy: {
            /** The announcements made by the proxy (key). */
            announcements: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<ITuple<[Vec<PalletProxyAnnouncement>, u128]>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * The set of account proxies. Maps the account which has delegated to the accounts which are being delegated to,
             * together with the amount held on deposit.
             */
            proxies: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<ITuple<[Vec<PalletProxyProxyDefinition>, u128]>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        referenda: {
            /** The number of referenda being decided currently. */
            decidingCount: AugmentedQuery<ApiType, (arg: u16 | AnyNumber | Uint8Array) => Observable<u32>, [u16]> &
                QueryableStorageEntry<ApiType, [u16]>;
            /**
             * The metadata is a general information concerning the referendum. The `Hash` refers to the preimage of the
             * `Preimages` provider which can be a JSON dump or IPFS hash of a JSON file.
             *
             * Consider a garbage collection for a metadata of finished referendums to `unrequest` (remove) large preimages.
             */
            metadataOf: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<H256>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** The next free referendum index, aka the number of referenda started so far. */
            referendumCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /** Information concerning any given referendum. */
            referendumInfoFor: AugmentedQuery<
                ApiType,
                (
                    arg: u32 | AnyNumber | Uint8Array
                ) => Observable<Option<PalletReferendaReferendumInfoConvictionVotingTally>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * The sorted list of referenda ready to be decided but not yet being decided, ordered by conviction-weighted
             * approvals.
             *
             * This should be empty if `DecidingCount` is less than `TrackInfo::max_deciding`.
             */
            trackQueue: AugmentedQuery<
                ApiType,
                (arg: u16 | AnyNumber | Uint8Array) => Observable<Vec<ITuple<[u32, u128]>>>,
                [u16]
            > &
                QueryableStorageEntry<ApiType, [u16]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        registrar: {
            /** The next free `ParaId`. */
            nextFreeParaId: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Amount held on deposit for each para and the original depositor.
             *
             * The given account ID is responsible for registering the code and initial head data, but may only do so if it
             * isn't yet registered. (After that, it's up to governance to do so.)
             */
            paras: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PolkadotRuntimeCommonParasRegistrarParaInfo>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Pending swap operations. */
            pendingSwap: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<u32>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        scheduler: {
            /** Items to be executed, indexed by the block number that they should be executed on. */
            agenda: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<Option<PalletSchedulerScheduled>>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            incompleteSince: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Lookup from a name to the block number and index of the task.
             *
             * For v3 -> v4 the previously unbounded identities are Blake2-256 hashed to form the v4 identities.
             */
            lookup: AugmentedQuery<
                ApiType,
                (arg: U8aFixed | string | Uint8Array) => Observable<Option<ITuple<[u32, u32]>>>,
                [U8aFixed]
            > &
                QueryableStorageEntry<ApiType, [U8aFixed]>;
            /** Retry configurations for items to be executed, indexed by task address. */
            retries: AugmentedQuery<
                ApiType,
                (
                    arg: ITuple<[u32, u32]> | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array]
                ) => Observable<Option<PalletSchedulerRetryConfig>>,
                [ITuple<[u32, u32]>]
            > &
                QueryableStorageEntry<ApiType, [ITuple<[u32, u32]>]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        servicesPayment: {
            blockProductionCredits: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<u32>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            collatorAssignmentCredits: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<u32>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** List of para ids that have already been given free credits */
            givenFreeCredits: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<Null>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Max core price for parathread in relay chain currency */
            maxCorePrice: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<u128>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Max tip for collator assignment on congestion */
            maxTip: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<u128>>, [u32]> &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Refund address */
            refundAddress: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<AccountId32>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        session: {
            /** Current index of the session. */
            currentIndex: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Indices of disabled validators.
             *
             * The vec is always kept sorted so that we can find whether a given validator is disabled using binary search. It
             * gets cleared when `on_session_ending` returns a new set of identities.
             */
            disabledValidators: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** The owner of a key. The key is the `KeyTypeId` + the encoded key. */
            keyOwner: AugmentedQuery<
                ApiType,
                (
                    arg:
                        | ITuple<[SpCoreCryptoKeyTypeId, Bytes]>
                        | [SpCoreCryptoKeyTypeId | string | Uint8Array, Bytes | string | Uint8Array]
                ) => Observable<Option<AccountId32>>,
                [ITuple<[SpCoreCryptoKeyTypeId, Bytes]>]
            > &
                QueryableStorageEntry<ApiType, [ITuple<[SpCoreCryptoKeyTypeId, Bytes]>]>;
            /** The next session keys for a validator. */
            nextKeys: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<DancelightRuntimeSessionKeys>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * True if the underlying economic identities or weighting behind the validators has changed in the queued
             * validator set.
             */
            queuedChanged: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The queued keys for the next session. When the next session begins, these keys will be used to determine the
             * validator's session keys.
             */
            queuedKeys: AugmentedQuery<
                ApiType,
                () => Observable<Vec<ITuple<[AccountId32, DancelightRuntimeSessionKeys]>>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** The current set of validators. */
            validators: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        sudo: {
            /** The `AccountId` of the sudo key. */
            key: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        system: {
            /** The full account information for a particular account ID. */
            account: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<FrameSystemAccountInfo>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /** Total length (in bytes) for all extrinsics put together, for the current block. */
            allExtrinsicsLen: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** `Some` if a code upgrade has been authorized. */
            authorizedUpgrade: AugmentedQuery<
                ApiType,
                () => Observable<Option<FrameSystemCodeUpgradeAuthorization>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** Map of block numbers to block hashes. */
            blockHash: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<H256>, [u32]> &
                QueryableStorageEntry<ApiType, [u32]>;
            /** The current weight for the block. */
            blockWeight: AugmentedQuery<ApiType, () => Observable<FrameSupportDispatchPerDispatchClassWeight>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Digest of the current block, also part of the block header. */
            digest: AugmentedQuery<ApiType, () => Observable<SpRuntimeDigest>, []> & QueryableStorageEntry<ApiType, []>;
            /** The number of events in the `Events<T>` list. */
            eventCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Events deposited for the current block.
             *
             * NOTE: The item is unbound and should therefore never be read on chain. It could otherwise inflate the PoV size
             * of a block.
             *
             * Events have a large in-memory size. Box the events to not go out-of-memory just in case someone still reads
             * them from within the runtime.
             */
            events: AugmentedQuery<ApiType, () => Observable<Vec<FrameSystemEventRecord>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Mapping between a topic (represented by T::Hash) and a vector of indexes of events in the `<Events<T>>` list.
             *
             * All topic vectors have deterministic storage locations depending on the topic. This allows light-clients to
             * leverage the changes trie storage tracking mechanism and in case of changes fetch the list of events of
             * interest.
             *
             * The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just the `EventIndex` then in
             * case if the topic has the same contents on the next block no notification will be triggered thus the event
             * might be lost.
             */
            eventTopics: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Vec<ITuple<[u32, u32]>>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /** The execution phase of the block. */
            executionPhase: AugmentedQuery<ApiType, () => Observable<Option<FrameSystemPhase>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Total extrinsics count for the current block. */
            extrinsicCount: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Extrinsics data for the current block (maps an extrinsic's index to its data). */
            extrinsicData: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Bytes>, [u32]> &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Whether all inherents have been applied. */
            inherentsApplied: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
            /** Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened. */
            lastRuntimeUpgrade: AugmentedQuery<
                ApiType,
                () => Observable<Option<FrameSystemLastRuntimeUpgradeInfo>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** The current block number being processed. Set by `execute_block`. */
            number: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /** Hash of the previous block. */
            parentHash: AugmentedQuery<ApiType, () => Observable<H256>, []> & QueryableStorageEntry<ApiType, []>;
            /** True if we have upgraded so that AccountInfo contains three types of `RefCount`. False (default) if not. */
            upgradedToTripleRefCount: AugmentedQuery<ApiType, () => Observable<bool>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** True if we have upgraded so that `type RefCount` is `u32`. False (default) if not. */
            upgradedToU32RefCount: AugmentedQuery<ApiType, () => Observable<bool>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        tanssiAuthorityAssignment: {
            collatorContainerChain: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<DpCollatorAssignmentAssignedCollatorsPublic>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        tanssiAuthorityMapping: {
            authorityIdMapping: AugmentedQuery<
                ApiType,
                (
                    arg: u32 | AnyNumber | Uint8Array
                ) => Observable<Option<BTreeMap<NimbusPrimitivesNimbusCryptoPublic, AccountId32>>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        tanssiCollatorAssignment: {
            collatorContainerChain: AugmentedQuery<
                ApiType,
                () => Observable<DpCollatorAssignmentAssignedCollatorsAccountId32>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** Ratio of assigned collators to max collators. */
            collatorFullnessRatio: AugmentedQuery<ApiType, () => Observable<Option<Perbill>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Pending configuration changes.
             *
             * This is a list of configuration changes, each with a session index at which it should be applied.
             *
             * The list is sorted ascending by session index. Also, this list can only contain at most 2 items: for the next
             * session and for the `scheduled_session`.
             */
            pendingCollatorContainerChain: AugmentedQuery<
                ApiType,
                () => Observable<Option<DpCollatorAssignmentAssignedCollatorsAccountId32>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Randomness from previous block. Used to shuffle collators on session change. Should only be set on the last
             * block of each session and should be killed on the on_initialize of the next block. The default value of [0; 32]
             * disables randomness in the pallet.
             */
            randomness: AugmentedQuery<ApiType, () => Observable<U8aFixed>, []> & QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        tanssiInvulnerables: {
            /** The invulnerable, permissioned collators. */
            invulnerables: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        timestamp: {
            /**
             * Whether the timestamp has been updated in this block.
             *
             * This value is updated to `true` upon successful submission of a timestamp by a node. It is then checked at the
             * end of each block execution in the `on_finalize` hook.
             */
            didUpdate: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
            /** The current time for the current block. */
            now: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        transactionPayment: {
            nextFeeMultiplier: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
            storageVersion: AugmentedQuery<ApiType, () => Observable<PalletTransactionPaymentReleases>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        treasury: {
            /** Proposal indices that have been approved but not yet awarded. */
            approvals: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> & QueryableStorageEntry<ApiType, []>;
            /** The amount which has been reported as inactive to Currency. */
            deactivated: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
            /** Number of proposals that have been made. */
            proposalCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /** Proposals that have been made. */
            proposals: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletTreasuryProposal>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** The count of spends that have been made. */
            spendCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /** Spends that have been approved and being processed. */
            spends: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletTreasurySpendStatus>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        whitelist: {
            whitelistedCall: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<Null>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        xcmPallet: {
            /**
             * The existing asset traps.
             *
             * Key is the blake2 256 hash of (origin, versioned `Assets`) pair. Value is the number of times this pair has
             * been trapped (usually just 1 if it exists at all).
             */
            assetTraps: AugmentedQuery<ApiType, (arg: H256 | string | Uint8Array) => Observable<u32>, [H256]> &
                QueryableStorageEntry<ApiType, [H256]>;
            /** The current migration's stage, if any. */
            currentMigration: AugmentedQuery<ApiType, () => Observable<Option<PalletXcmVersionMigrationStage>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Fungible assets which we know are locked on this chain. */
            lockedFungibles: AugmentedQuery<
                ApiType,
                (
                    arg: AccountId32 | string | Uint8Array
                ) => Observable<Option<Vec<ITuple<[u128, XcmVersionedLocation]>>>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /** The ongoing queries. */
            queries: AugmentedQuery<
                ApiType,
                (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<PalletXcmQueryStatus>>,
                [u64]
            > &
                QueryableStorageEntry<ApiType, [u64]>;
            /** The latest available query index. */
            queryCounter: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * If [`ShouldRecordXcm`] is set to true, then the last XCM program executed locally will be stored here. Runtime
             * APIs can fetch the XCM that was executed by accessing this value.
             *
             * Only relevant if this pallet is being used as the [`xcm_executor::traits::RecordXcm`] implementation in the XCM
             * executor configuration.
             */
            recordedXcm: AugmentedQuery<ApiType, () => Observable<Option<Vec<StagingXcmV4Instruction>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Fungible assets which we know are locked on a remote chain. */
            remoteLockedFungibles: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: AccountId32 | string | Uint8Array,
                    arg3: XcmVersionedAssetId | { V3: any } | { V4: any } | string | Uint8Array
                ) => Observable<Option<PalletXcmRemoteLockedFungibleRecord>>,
                [u32, AccountId32, XcmVersionedAssetId]
            > &
                QueryableStorageEntry<ApiType, [u32, AccountId32, XcmVersionedAssetId]>;
            /**
             * Default version to encode XCM when latest version of destination is unknown. If `None`, then the destinations
             * whose XCM version is unknown are considered unreachable.
             */
            safeXcmVersion: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Whether or not incoming XCMs (both executed locally and received) should be recorded. Only one XCM program will
             * be recorded at a time. This is meant to be used in runtime APIs, and it's advised it stays false for all other
             * use cases, so as to not degrade regular performance.
             *
             * Only relevant if this pallet is being used as the [`xcm_executor::traits::RecordXcm`] implementation in the XCM
             * executor configuration.
             */
            shouldRecordXcm: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
            /** The Latest versions that we know various locations support. */
            supportedVersion: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array
                ) => Observable<Option<u32>>,
                [u32, XcmVersionedLocation]
            > &
                QueryableStorageEntry<ApiType, [u32, XcmVersionedLocation]>;
            /**
             * Destinations whose latest XCM version we would like to know. Duplicates not allowed, and the `u32` counter is
             * the number of times that a send to the destination has been attempted, which is used as a prioritization.
             */
            versionDiscoveryQueue: AugmentedQuery<
                ApiType,
                () => Observable<Vec<ITuple<[XcmVersionedLocation, u32]>>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** All locations that we have requested version notifications from. */
            versionNotifiers: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array
                ) => Observable<Option<u64>>,
                [u32, XcmVersionedLocation]
            > &
                QueryableStorageEntry<ApiType, [u32, XcmVersionedLocation]>;
            /**
             * The target locations that are subscribed to our version changes, as well as the most recent of our versions we
             * informed them of.
             */
            versionNotifyTargets: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: XcmVersionedLocation | { V2: any } | { V3: any } | { V4: any } | string | Uint8Array
                ) => Observable<Option<ITuple<[u64, SpWeightsWeightV2Weight, u32]>>>,
                [u32, XcmVersionedLocation]
            > &
                QueryableStorageEntry<ApiType, [u32, XcmVersionedLocation]>;
            /** Global suspension state of the XCM executor. */
            xcmExecutionSuspended: AugmentedQuery<ApiType, () => Observable<bool>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
    } // AugmentedQueries
} // declare module
