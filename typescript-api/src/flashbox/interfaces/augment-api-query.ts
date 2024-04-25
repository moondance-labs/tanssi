// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/storage";

import type { ApiTypes, AugmentedQuery, QueryableStorageEntry } from "@polkadot/api-base/types";
import type { Data } from "@polkadot/types";
import type { BTreeMap, Bytes, Null, Option, U8aFixed, Vec, bool, u128, u32, u64 } from "@polkadot/types-codec";
import type { AnyNumber, ITuple } from "@polkadot/types-codec/types";
import type { AccountId32, H256 } from "@polkadot/types/interfaces/runtime";
import type {
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot,
    CumulusPalletParachainSystemUnincludedSegmentAncestor,
    CumulusPalletParachainSystemUnincludedSegmentSegmentTracker,
    DpCollatorAssignmentAssignedCollatorsAccountId32,
    DpCollatorAssignmentAssignedCollatorsPublic,
    FlashboxRuntimeSessionKeys,
    FrameSupportDispatchPerDispatchClassWeight,
    FrameSystemAccountInfo,
    FrameSystemCodeUpgradeAuthorization,
    FrameSystemEventRecord,
    FrameSystemLastRuntimeUpgradeInfo,
    FrameSystemPhase,
    NimbusPrimitivesNimbusCryptoPublic,
    PalletAuthorNotingContainerChainBlockInfo,
    PalletBalancesAccountData,
    PalletBalancesBalanceLock,
    PalletBalancesIdAmountRuntimeFreezeReason,
    PalletBalancesIdAmountRuntimeHoldReason,
    PalletBalancesReserveData,
    PalletConfigurationHostConfiguration,
    PalletIdentityAuthorityProperties,
    PalletIdentityRegistrarInfo,
    PalletIdentityRegistration,
    PalletInflationRewardsChainsToRewardValue,
    PalletMultisigMultisig,
    PalletProxyAnnouncement,
    PalletProxyProxyDefinition,
    PalletRegistrarDepositInfo,
    PalletStreamPaymentStream,
    PalletTransactionPaymentReleases,
    PalletTreasuryProposal,
    PalletTreasurySpendStatus,
    PolkadotCorePrimitivesOutboundHrmpMessage,
    PolkadotPrimitivesV6AbridgedHostConfiguration,
    PolkadotPrimitivesV6PersistedValidationData,
    PolkadotPrimitivesV6UpgradeGoAhead,
    PolkadotPrimitivesV6UpgradeRestriction,
    SpCoreCryptoKeyTypeId,
    SpRuntimeDigest,
    SpTrieStorageProof,
    SpWeightsWeightV2Weight,
    TpContainerChainGenesisDataContainerChainGenesisData,
    TpTraitsParathreadParams,
} from "@polkadot/types/lookup";
import type { Observable } from "@polkadot/types/types";

export type __AugmentedQuery<ApiType extends ApiTypes> = AugmentedQuery<ApiType, () => unknown>;
export type __QueryableStorageEntry<ApiType extends ApiTypes> = QueryableStorageEntry<ApiType>;

declare module "@polkadot/api-base/types/storage" {
    interface AugmentedQueries<ApiType extends ApiTypes> {
        asyncBacking: {
            /**
             * First tuple element is the highest slot that has been seen in the history of this chain. Second tuple element
             * is the number of authored blocks so far. This is a strictly-increasing value if T::AllowMultipleBlocksPerSlot = false.
             */
            slotInfo: AugmentedQuery<ApiType, () => Observable<Option<ITuple<[u64, u32]>>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        authorInherent: {
            /** Author of current block. */
            author: AugmentedQuery<ApiType, () => Observable<Option<U8aFixed>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Check if the inherent was included */
            inherentIncluded: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        authorityAssignment: {
            collatorContainerChain: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<DpCollatorAssignmentAssignedCollatorsPublic>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        authorityMapping: {
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
        authorNoting: {
            /** Was the containerAuthorData set? */
            didSetContainerAuthorData: AugmentedQuery<ApiType, () => Observable<bool>, []> &
                QueryableStorageEntry<ApiType, []>;
            latestAuthor: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<PalletAuthorNotingContainerChainBlockInfo>>,
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
                (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesIdAmountRuntimeFreezeReason>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /** Holds on account balances. */
            holds: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesIdAmountRuntimeHoldReason>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /** The total units of outstanding deactivated balance in the system. */
            inactiveIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
            /** Any liquidity locks on some account balances. NOTE: Should only be accessed when setting, changing and freeing a lock. */
            locks: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesBalanceLock>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /** Named reserves on some account balances. */
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
        collatorAssignment: {
            collatorContainerChain: AugmentedQuery<
                ApiType,
                () => Observable<DpCollatorAssignmentAssignedCollatorsAccountId32>,
                []
            > &
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
        configuration: {
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
        dataPreservers: {
            bootNodes: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<Bytes>>, [u32]> &
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
        invulnerables: {
            /** The invulnerable, permissioned collators. This list must be sorted. */
            invulnerables: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        maintenanceMode: {
            /** Whether the site is in maintenance mode */
            maintenanceMode: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
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
        parachainInfo: {
            parachainId: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        parachainSystem: {
            /**
             * Storage field that keeps track of bandwidth used by the unincluded segment along with the latest HRMP
             * watermark. Used for limiting the acceptance of new blocks with respect to relay chain constraints.
             */
            aggregatedUnincludedSegment: AugmentedQuery<
                ApiType,
                () => Observable<Option<CumulusPalletParachainSystemUnincludedSegmentSegmentTracker>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The number of HRMP messages we observed in `on_initialize` and thus used that number for announcing the weight
             * of `on_initialize` and `on_finalize`.
             */
            announcedHrmpMessagesPerCandidate: AugmentedQuery<ApiType, () => Observable<u32>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * A custom head data that should be returned as result of `validate_block`.
             *
             * See `Pallet::set_custom_validation_head_data` for more information.
             */
            customValidationHeadData: AugmentedQuery<ApiType, () => Observable<Option<Bytes>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Were the validation data set to notify the relay chain? */
            didSetValidationCode: AugmentedQuery<ApiType, () => Observable<bool>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The parachain host configuration that was obtained from the relay parent.
             *
             * This field is meant to be updated each block with the validation data inherent. Therefore, before processing of
             * the inherent, e.g. in `on_initialize` this data may be stale.
             *
             * This data is also absent from the genesis.
             */
            hostConfiguration: AugmentedQuery<
                ApiType,
                () => Observable<Option<PolkadotPrimitivesV6AbridgedHostConfiguration>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * HRMP messages that were sent in a block.
             *
             * This will be cleared in `on_initialize` of each new block.
             */
            hrmpOutboundMessages: AugmentedQuery<
                ApiType,
                () => Observable<Vec<PolkadotCorePrimitivesOutboundHrmpMessage>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * HRMP watermark that was set in a block.
             *
             * This will be cleared in `on_initialize` of each new block.
             */
            hrmpWatermark: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The last downward message queue chain head we have observed.
             *
             * This value is loaded before and saved after processing inbound downward messages carried by the system inherent.
             */
            lastDmqMqcHead: AugmentedQuery<ApiType, () => Observable<H256>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The message queue chain heads we have observed per each channel incoming channel.
             *
             * This value is loaded before and saved after processing inbound downward messages carried by the system inherent.
             */
            lastHrmpMqcHeads: AugmentedQuery<ApiType, () => Observable<BTreeMap<u32, H256>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The relay chain block number associated with the last parachain block.
             *
             * This is updated in `on_finalize`.
             */
            lastRelayChainBlockNumber: AugmentedQuery<ApiType, () => Observable<u32>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Validation code that is set by the parachain and is to be communicated to collator and consequently the relay-chain.
             *
             * This will be cleared in `on_initialize` of each new block if no other pallet already set the value.
             */
            newValidationCode: AugmentedQuery<ApiType, () => Observable<Option<Bytes>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Upward messages that are still pending and not yet send to the relay chain. */
            pendingUpwardMessages: AugmentedQuery<ApiType, () => Observable<Vec<Bytes>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * In case of a scheduled upgrade, this storage field contains the validation code to be applied.
             *
             * As soon as the relay chain gives us the go-ahead signal, we will overwrite the
             * [`:code`][sp_core::storage::well_known_keys::CODE] which will result the next block process with the new
             * validation code. This concludes the upgrade process.
             */
            pendingValidationCode: AugmentedQuery<ApiType, () => Observable<Bytes>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Number of downward messages processed in a block.
             *
             * This will be cleared in `on_initialize` of each new block.
             */
            processedDownwardMessages: AugmentedQuery<ApiType, () => Observable<u32>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The state proof for the last relay parent block.
             *
             * This field is meant to be updated each block with the validation data inherent. Therefore, before processing of
             * the inherent, e.g. in `on_initialize` this data may be stale.
             *
             * This data is also absent from the genesis.
             */
            relayStateProof: AugmentedQuery<ApiType, () => Observable<Option<SpTrieStorageProof>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The snapshot of some state related to messaging relevant to the current parachain as per the relay parent.
             *
             * This field is meant to be updated each block with the validation data inherent. Therefore, before processing of
             * the inherent, e.g. in `on_initialize` this data may be stale.
             *
             * This data is also absent from the genesis.
             */
            relevantMessagingState: AugmentedQuery<
                ApiType,
                () => Observable<Option<CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The weight we reserve at the beginning of the block for processing DMP messages. This overrides the amount set
             * in the Config trait.
             */
            reservedDmpWeightOverride: AugmentedQuery<ApiType, () => Observable<Option<SpWeightsWeightV2Weight>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The weight we reserve at the beginning of the block for processing XCMP messages. This overrides the amount set
             * in the Config trait.
             */
            reservedXcmpWeightOverride: AugmentedQuery<ApiType, () => Observable<Option<SpWeightsWeightV2Weight>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Latest included block descendants the runtime accepted. In other words, these are ancestors of the currently
             * executing block which have not been included in the observed relay-chain state.
             *
             * The segment length is limited by the capacity returned from the [`ConsensusHook`] configured in the pallet.
             */
            unincludedSegment: AugmentedQuery<
                ApiType,
                () => Observable<Vec<CumulusPalletParachainSystemUnincludedSegmentAncestor>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Optional upgrade go-ahead signal from the relay-chain.
             *
             * This storage item is a mirror of the corresponding value for the current parachain from the relay-chain. This
             * value is ephemeral which means it doesn't hit the storage. This value is set after the inherent.
             */
            upgradeGoAhead: AugmentedQuery<ApiType, () => Observable<Option<PolkadotPrimitivesV6UpgradeGoAhead>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * An option which indicates if the relay-chain restricts signalling a validation code upgrade. In other words, if
             * this is `Some` and [`NewValidationCode`] is `Some` then the produced candidate will be invalid.
             *
             * This storage item is a mirror of the corresponding value for the current parachain from the relay-chain. This
             * value is ephemeral which means it doesn't hit the storage. This value is set after the inherent.
             */
            upgradeRestrictionSignal: AugmentedQuery<
                ApiType,
                () => Observable<Option<PolkadotPrimitivesV6UpgradeRestriction>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** The factor to multiply the base delivery fee by for UMP. */
            upwardDeliveryFeeFactor: AugmentedQuery<ApiType, () => Observable<u128>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Upward messages that were sent in a block.
             *
             * This will be cleared in `on_initialize` of each new block.
             */
            upwardMessages: AugmentedQuery<ApiType, () => Observable<Vec<Bytes>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The [`PersistedValidationData`] set for this block. This value is expected to be set only once per block and
             * it's never stored in the trie.
             */
            validationData: AugmentedQuery<
                ApiType,
                () => Observable<Option<PolkadotPrimitivesV6PersistedValidationData>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
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
        registrar: {
            paraGenesisData: AugmentedQuery<
                ApiType,
                (
                    arg: u32 | AnyNumber | Uint8Array
                ) => Observable<Option<TpContainerChainGenesisDataContainerChainGenesisData>>,
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
        relayStorageRoots: {
            /** Map of relay block number to relay storage root */
            relayStorageRoot: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Option<H256>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /** List of all the keys in `RelayStorageRoot`. Used to remove the oldest key without having to iterate over all of them. */
            relayStorageRootKeys: AugmentedQuery<ApiType, () => Observable<Vec<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
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
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<FlashboxRuntimeSessionKeys>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /** True if the underlying economic identities or weighting behind the validators has changed in the queued validator set. */
            queuedChanged: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The queued keys for the next session. When the next session begins, these keys will be used to determine the
             * validator's session keys.
             */
            queuedKeys: AugmentedQuery<
                ApiType,
                () => Observable<Vec<ITuple<[AccountId32, FlashboxRuntimeSessionKeys]>>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /** The current set of validators. */
            validators: AugmentedQuery<ApiType, () => Observable<Vec<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        streamPayment: {
            /**
             * Lookup for all streams with given source. To avoid maintaining a growing list of stream ids, they are stored in
             * the form of an entry (AccountId, StreamId). If such entry exists then this AccountId is a source in StreamId.
             * One can iterate over all storage keys starting with the AccountId to find all StreamIds.
             */
            lookupStreamsWithSource: AugmentedQuery<
                ApiType,
                (
                    arg1: AccountId32 | string | Uint8Array,
                    arg2: u64 | AnyNumber | Uint8Array
                ) => Observable<Option<Null>>,
                [AccountId32, u64]
            > &
                QueryableStorageEntry<ApiType, [AccountId32, u64]>;
            /**
             * Lookup for all streams with given target. To avoid maintaining a growing list of stream ids, they are stored in
             * the form of an entry (AccountId, StreamId). If such entry exists then this AccountId is a target in StreamId.
             * One can iterate over all storage keys starting with the AccountId to find all StreamIds.
             */
            lookupStreamsWithTarget: AugmentedQuery<
                ApiType,
                (
                    arg1: AccountId32 | string | Uint8Array,
                    arg2: u64 | AnyNumber | Uint8Array
                ) => Observable<Option<Null>>,
                [AccountId32, u64]
            > &
                QueryableStorageEntry<ApiType, [AccountId32, u64]>;
            /** Store the next available stream id. */
            nextStreamId: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /** Store each stream indexed by an Id. */
            streams: AugmentedQuery<
                ApiType,
                (arg: u64 | AnyNumber | Uint8Array) => Observable<Option<PalletStreamPaymentStream>>,
                [u64]
            > &
                QueryableStorageEntry<ApiType, [u64]>;
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
             * leverage the changes trie storage tracking mechanism and in case of changes fetch the list of events of interest.
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
        txPause: {
            /** The set of calls that are explicitly paused. */
            pausedCalls: AugmentedQuery<
                ApiType,
                (
                    arg: ITuple<[Bytes, Bytes]> | [Bytes | string | Uint8Array, Bytes | string | Uint8Array]
                ) => Observable<Option<Null>>,
                [ITuple<[Bytes, Bytes]>]
            > &
                QueryableStorageEntry<ApiType, [ITuple<[Bytes, Bytes]>]>;
            /** Generic query */
            [key: string]: QueryableStorageEntry<ApiType>;
        };
    } // AugmentedQueries
} // declare module
