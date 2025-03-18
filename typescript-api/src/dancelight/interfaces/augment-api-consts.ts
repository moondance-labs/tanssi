// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/consts";

import type { ApiTypes, AugmentedConst } from "@polkadot/api-base/types";
import type { Option, Vec, u128, u16, u32, u64, u8 } from "@polkadot/types-codec";
import type { Codec, ITuple } from "@polkadot/types-codec/types";
import type { AccountId32, H160, Perbill, Permill } from "@polkadot/types/interfaces/runtime";
import type {
    FrameSupportPalletId,
    FrameSystemLimitsBlockLength,
    FrameSystemLimitsBlockWeights,
    PalletReferendaTrackInfo,
    SnowbridgeBeaconPrimitivesForkVersions,
    SpVersionRuntimeVersion,
    SpWeightsRuntimeDbWeight,
    SpWeightsWeightV2Weight,
} from "@polkadot/types/lookup";

export type __AugmentedConst<ApiType extends ApiTypes> = AugmentedConst<ApiType>;

declare module "@polkadot/api-base/types/consts" {
    interface AugmentedConsts<ApiType extends ApiTypes> {
        authorNoting: {
            /**
             * Max length of para id list, should be the same value as in other pallets.
             **/
            maxContainerChains: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        babe: {
            /**
             * The amount of time, in slots, that each epoch should last.
             * NOTE: Currently it is not possible to change the epoch duration after
             * the chain has started. Attempting to do so will brick block production.
             **/
            epochDuration: u64 & AugmentedConst<ApiType>;
            /**
             * The expected average block time at which BABE should be creating
             * blocks. Since BABE is probabilistic it is not trivial to figure out
             * what the expected average block time should be based on the slot
             * duration and the security parameter `c` (where `1 - c` represents
             * the probability of a slot being empty).
             **/
            expectedBlockTime: u64 & AugmentedConst<ApiType>;
            /**
             * Max number of authorities allowed
             **/
            maxAuthorities: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum number of nominators for each validator.
             **/
            maxNominators: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        balances: {
            /**
             * The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!
             *
             * If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for
             * this pallet. However, you do so at your own risk: this will open up a major DoS vector.
             * In case you have multiple sources of provider references, you may also get unexpected
             * behaviour if you set this to zero.
             *
             * Bottom line: Do yourself a favour and make it at least one!
             **/
            existentialDeposit: u128 & AugmentedConst<ApiType>;
            /**
             * The maximum number of individual freeze locks that can exist on an account at any time.
             **/
            maxFreezes: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum number of locks that should exist on an account.
             * Not strictly enforced, but used for weight estimation.
             *
             * Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`
             **/
            maxLocks: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum number of named reserves that can exist on an account.
             *
             * Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`
             **/
            maxReserves: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        beefy: {
            /**
             * The maximum number of authorities that can be added.
             **/
            maxAuthorities: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum number of nominators for each validator.
             **/
            maxNominators: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum number of entries to keep in the set id to session index mapping.
             *
             * Since the `SetIdSession` map is only used for validating equivocations this
             * value should relate to the bonding duration of whatever staking system is
             * being used (if any). If equivocation handling is not enabled then this value
             * can be zero.
             **/
            maxSetIdSessionEntries: u64 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        collatorConfiguration: {
            sessionDelay: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        containerRegistrar: {
            depositAmount: u128 & AugmentedConst<ApiType>;
            /**
             * Max length of encoded genesis data
             **/
            maxGenesisDataSize: u32 & AugmentedConst<ApiType>;
            /**
             * Max length of para id list
             **/
            maxLengthParaIds: u32 & AugmentedConst<ApiType>;
            sessionDelay: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        convictionVoting: {
            /**
             * The maximum number of concurrent votes an account may have.
             *
             * Also used to compute weight, an overly large value can lead to extrinsics with large
             * weight estimation: see `delegate` for instance.
             **/
            maxVotes: u32 & AugmentedConst<ApiType>;
            /**
             * The minimum period of vote locking.
             *
             * It should be no shorter than enactment period to ensure that in the case of an approval,
             * those successful voters are locked into the consequences that their votes entail.
             **/
            voteLockingPeriod: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        dataPreservers: {
            maxAssignmentsPerParaId: u32 & AugmentedConst<ApiType>;
            maxNodeUrlLen: u32 & AugmentedConst<ApiType>;
            maxParaIdsVecLen: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        ethereumBeaconClient: {
            forkVersions: SnowbridgeBeaconPrimitivesForkVersions & AugmentedConst<ApiType>;
            /**
             * Minimum gap between finalized headers for an update to be free.
             **/
            freeHeadersInterval: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        ethereumInboundQueue: {
            gatewayAddress: H160 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        ethereumOutboundQueue: {
            /**
             * Number of decimal places in native currency
             **/
            decimals: u8 & AugmentedConst<ApiType>;
            /**
             * Max bytes in a message payload
             **/
            maxMessagePayloadSize: u32 & AugmentedConst<ApiType>;
            /**
             * Max number of messages processed per block
             **/
            maxMessagesPerBlock: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        ethereumSystem: {
            /**
             * Cost of delivering a message from Ethereum
             **/
            inboundDeliveryCost: u128 & AugmentedConst<ApiType>;
            /**
             * TreasuryAccount to collect fees
             **/
            treasuryAccount: AccountId32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        externalValidators: {
            /**
             * Number of eras to keep in history.
             *
             * Following information is kept for eras in `[current_era -
             * HistoryDepth, current_era]`: `ErasStartSessionIndex`
             *
             * Must be more than the number of eras delayed by session.
             * I.e. active era must always be in history. I.e. `active_era >
             * current_era - history_depth` must be guaranteed.
             *
             * If migrating an existing pallet from storage value to config value,
             * this should be set to same value or greater as in storage.
             **/
            historyDepth: u32 & AugmentedConst<ApiType>;
            /**
             * Maximum number of external validators.
             **/
            maxExternalValidators: u32 & AugmentedConst<ApiType>;
            /**
             * Maximum number of whitelisted validators.
             **/
            maxWhitelistedValidators: u32 & AugmentedConst<ApiType>;
            /**
             * Number of sessions per era.
             **/
            sessionsPerEra: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        externalValidatorSlashes: {
            /**
             * Number of eras that staked funds must remain bonded for.
             **/
            bondingDuration: u32 & AugmentedConst<ApiType>;
            /**
             * How many queued slashes are being processed per block.
             **/
            queuedSlashesProcessedPerBlock: u32 & AugmentedConst<ApiType>;
            /**
             * Number of eras that slashes are deferred by, after computation.
             *
             * This should be less than the bonding duration. Set to 0 if slashes
             * should be applied immediately, without opportunity for intervention.
             **/
            slashDeferDuration: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        externalValidatorsRewards: {
            /**
             * The amount of era points given by backing a candidate that is included.
             **/
            backingPoints: u32 & AugmentedConst<ApiType>;
            /**
             * The amount of era points given by dispute voting on a candidate.
             **/
            disputeStatementPoints: u32 & AugmentedConst<ApiType>;
            /**
             * For how many eras points are kept in storage.
             **/
            historyDepth: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        fellowshipReferenda: {
            /**
             * Quantization level for the referendum wakeup scheduler. A higher number will result in
             * fewer storage reads/writes needed for smaller voters, but also result in delays to the
             * automatic referendum status changes. Explicit servicing instructions are unaffected.
             **/
            alarmInterval: u32 & AugmentedConst<ApiType>;
            /**
             * Maximum size of the referendum queue for a single track.
             **/
            maxQueued: u32 & AugmentedConst<ApiType>;
            /**
             * The minimum amount to be used as a deposit for a public referendum proposal.
             **/
            submissionDeposit: u128 & AugmentedConst<ApiType>;
            /**
             * Information concerning the different referendum tracks.
             **/
            tracks: Vec<ITuple<[u16, PalletReferendaTrackInfo]>> & AugmentedConst<ApiType>;
            /**
             * The number of blocks after submission that a referendum must begin being decided by.
             * Once this passes, then anyone may cancel the referendum.
             **/
            undecidingTimeout: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        grandpa: {
            /**
             * Max Authorities in use
             **/
            maxAuthorities: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum number of nominators for each validator.
             **/
            maxNominators: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum number of entries to keep in the set id to session index mapping.
             *
             * Since the `SetIdSession` map is only used for validating equivocations this
             * value should relate to the bonding duration of whatever staking system is
             * being used (if any). If equivocation handling is not enabled then this value
             * can be zero.
             **/
            maxSetIdSessionEntries: u64 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        identity: {
            /**
             * The amount held on deposit for a registered identity.
             **/
            basicDeposit: u128 & AugmentedConst<ApiType>;
            /**
             * The amount held on deposit per encoded byte for a registered identity.
             **/
            byteDeposit: u128 & AugmentedConst<ApiType>;
            /**
             * Maximum number of registrars allowed in the system. Needed to bound the complexity
             * of, e.g., updating judgements.
             **/
            maxRegistrars: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum number of sub-accounts allowed per identified account.
             **/
            maxSubAccounts: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum length of a suffix.
             **/
            maxSuffixLength: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum length of a username, including its suffix and any system-added delimiters.
             **/
            maxUsernameLength: u32 & AugmentedConst<ApiType>;
            /**
             * The number of blocks within which a username grant must be accepted.
             **/
            pendingUsernameExpiration: u32 & AugmentedConst<ApiType>;
            /**
             * The amount held on deposit for a registered subaccount. This should account for the fact
             * that one storage item's value will increase by the size of an account ID, and there will
             * be another trie item whose value is the size of an account ID plus 32 bytes.
             **/
            subAccountDeposit: u128 & AugmentedConst<ApiType>;
            /**
             * The amount held on deposit per registered username. This value should change only in
             * runtime upgrades with proper migration of existing deposits.
             **/
            usernameDeposit: u128 & AugmentedConst<ApiType>;
            /**
             * The number of blocks that must pass to enable the permanent deletion of a username by
             * its respective authority.
             **/
            usernameGracePeriod: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        inactivityTracking: {
            /**
             * The maximum amount of collators that can stored for a session
             **/
            maxCollatorsPerSession: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum number of sessions for which a collator can be inactive
             * before being moved to the offline queue
             **/
            maxInactiveSessions: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        inflationRewards: {
            /**
             * Inflation rate per orchestrator block (proportion of the total issuance)
             **/
            inflationRate: Perbill & AugmentedConst<ApiType>;
            /**
             * The account that will store rewards waiting to be paid out
             **/
            pendingRewardsAccount: AccountId32 & AugmentedConst<ApiType>;
            /**
             * Proportion of the new supply dedicated to staking
             **/
            rewardsPortion: Perbill & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        messageQueue: {
            /**
             * The size of the page; this implies the maximum message size which can be sent.
             *
             * A good value depends on the expected message sizes, their weights, the weight that is
             * available for processing them and the maximal needed message size. The maximal message
             * size is slightly lower than this as defined by [`MaxMessageLenOf`].
             **/
            heapSize: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum amount of weight (if any) to be used from remaining weight `on_idle` which
             * should be provided to the message queue for servicing enqueued items `on_idle`.
             * Useful for parachains to process messages at the same block they are received.
             *
             * If `None`, it will not call `ServiceQueues::service_queues` in `on_idle`.
             **/
            idleMaxServiceWeight: Option<SpWeightsWeightV2Weight> & AugmentedConst<ApiType>;
            /**
             * The maximum number of stale pages (i.e. of overweight messages) allowed before culling
             * can happen. Once there are more stale pages than this, then historical pages may be
             * dropped, even if they contain unprocessed overweight messages.
             **/
            maxStale: u32 & AugmentedConst<ApiType>;
            /**
             * The amount of weight (if any) which should be provided to the message queue for
             * servicing enqueued items `on_initialize`.
             *
             * This may be legitimately `None` in the case that you will call
             * `ServiceQueues::service_queues` manually or set [`Self::IdleMaxServiceWeight`] to have
             * it run in `on_idle`.
             **/
            serviceWeight: Option<SpWeightsWeightV2Weight> & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        multiBlockMigrations: {
            /**
             * The maximal length of an encoded cursor.
             *
             * A good default needs to selected such that no migration will ever have a cursor with MEL
             * above this limit. This is statically checked in `integrity_test`.
             **/
            cursorMaxLen: u32 & AugmentedConst<ApiType>;
            /**
             * The maximal length of an encoded identifier.
             *
             * A good default needs to selected such that no migration will ever have an identifier
             * with MEL above this limit. This is statically checked in `integrity_test`.
             **/
            identifierMaxLen: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        multisig: {
            /**
             * The base amount of currency needed to reserve for creating a multisig execution or to
             * store a dispatch call for later.
             *
             * This is held for an additional storage item whose value size is
             * `4 + sizeof((BlockNumber, Balance, AccountId))` bytes and whose key size is
             * `32 + sizeof(AccountId)` bytes.
             **/
            depositBase: u128 & AugmentedConst<ApiType>;
            /**
             * The amount of currency needed per unit threshold when creating a multisig execution.
             *
             * This is held for adding 32 bytes more into a pre-existing storage value.
             **/
            depositFactor: u128 & AugmentedConst<ApiType>;
            /**
             * The maximum amount of signatories allowed in the multisig.
             **/
            maxSignatories: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        onDemandAssignmentProvider: {
            /**
             * The maximum number of blocks some historical revenue
             * information stored for.
             **/
            maxHistoricalRevenue: u32 & AugmentedConst<ApiType>;
            /**
             * Identifier for the internal revenue balance.
             **/
            palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
            /**
             * The default value for the spot traffic multiplier.
             **/
            trafficDefaultValue: u128 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        paras: {
            unsignedPriority: u64 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        pooledStaking: {
            /**
             * All eligible candidates are stored in a sorted list that is modified each time
             * delegations changes. It is safer to bound this list, in which case eligible candidate
             * could fall out of this list if they have less stake than the top `EligibleCandidatesBufferSize`
             * eligible candidates. One of this top candidates leaving will then not bring the dropped candidate
             * in the list. An extrinsic is available to manually bring back such dropped candidate.
             **/
            eligibleCandidatesBufferSize: u32 & AugmentedConst<ApiType>;
            /**
             * When creating the first Shares for a candidate the supply can arbitrary.
             * Picking a value too high is a barrier of entry for staking, which will increase overtime
             * as the value of each share will increase due to auto compounding.
             **/
            initialAutoCompoundingShareValue: u128 & AugmentedConst<ApiType>;
            /**
             * When creating the first Shares for a candidate the supply can be arbitrary.
             * Picking a value too low will make an higher supply, which means each share will get
             * less rewards, and rewards calculations will have more impactful rounding errors.
             * Picking a value too high is a barrier of entry for staking.
             **/
            initialManualClaimShareValue: u128 & AugmentedConst<ApiType>;
            /**
             * Minimum amount of stake a Candidate must delegate (stake) towards itself. Not reaching
             * this minimum prevents from being elected.
             **/
            minimumSelfDelegation: u128 & AugmentedConst<ApiType>;
            /**
             * Part of the rewards that will be sent exclusively to the collator.
             **/
            rewardsCollatorCommission: Perbill & AugmentedConst<ApiType>;
            /**
             * Account holding Currency of all delegators.
             **/
            stakingAccount: AccountId32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        proxy: {
            /**
             * The base amount of currency needed to reserve for creating an announcement.
             *
             * This is held when a new storage item holding a `Balance` is created (typically 16
             * bytes).
             **/
            announcementDepositBase: u128 & AugmentedConst<ApiType>;
            /**
             * The amount of currency needed per announcement made.
             *
             * This is held for adding an `AccountId`, `Hash` and `BlockNumber` (typically 68 bytes)
             * into a pre-existing storage value.
             **/
            announcementDepositFactor: u128 & AugmentedConst<ApiType>;
            /**
             * The maximum amount of time-delayed announcements that are allowed to be pending.
             **/
            maxPending: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum amount of proxies allowed for a single account.
             **/
            maxProxies: u32 & AugmentedConst<ApiType>;
            /**
             * The base amount of currency needed to reserve for creating a proxy.
             *
             * This is held for an additional storage item whose value size is
             * `sizeof(Balance)` bytes and whose key size is `sizeof(AccountId)` bytes.
             **/
            proxyDepositBase: u128 & AugmentedConst<ApiType>;
            /**
             * The amount of currency needed per proxy added.
             *
             * This is held for adding 32 bytes plus an instance of `ProxyType` more into a
             * pre-existing storage value. Thus, when configuring `ProxyDepositFactor` one should take
             * into account `32 + proxy_type.encode().len()` bytes of data.
             **/
            proxyDepositFactor: u128 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        referenda: {
            /**
             * Quantization level for the referendum wakeup scheduler. A higher number will result in
             * fewer storage reads/writes needed for smaller voters, but also result in delays to the
             * automatic referendum status changes. Explicit servicing instructions are unaffected.
             **/
            alarmInterval: u32 & AugmentedConst<ApiType>;
            /**
             * Maximum size of the referendum queue for a single track.
             **/
            maxQueued: u32 & AugmentedConst<ApiType>;
            /**
             * The minimum amount to be used as a deposit for a public referendum proposal.
             **/
            submissionDeposit: u128 & AugmentedConst<ApiType>;
            /**
             * Information concerning the different referendum tracks.
             **/
            tracks: Vec<ITuple<[u16, PalletReferendaTrackInfo]>> & AugmentedConst<ApiType>;
            /**
             * The number of blocks after submission that a referendum must begin being decided by.
             * Once this passes, then anyone may cancel the referendum.
             **/
            undecidingTimeout: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        registrar: {
            /**
             * The deposit to be paid per byte stored on chain.
             **/
            dataDepositPerByte: u128 & AugmentedConst<ApiType>;
            /**
             * The deposit to be paid to run a on-demand parachain.
             * This should include the cost for storing the genesis head and validation code.
             **/
            paraDeposit: u128 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        scheduler: {
            /**
             * The maximum weight that may be scheduled per block for any dispatchables.
             **/
            maximumWeight: SpWeightsWeightV2Weight & AugmentedConst<ApiType>;
            /**
             * The maximum number of scheduled calls in the queue for a single block.
             *
             * NOTE:
             * + Dependent pallets' benchmarks might require a higher limit for the setting. Set a
             * higher limit under `runtime-benchmarks` feature.
             **/
            maxScheduledPerBlock: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        servicesPayment: {
            /**
             * The maximum number of block production credits that can be accumulated
             **/
            freeBlockProductionCredits: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum number of collator assigment production credits that can be accumulated
             **/
            freeCollatorAssignmentCredits: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        streamPayment: {
            openStreamHoldAmount: u128 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        system: {
            /**
             * Maximum number of block number to block hash mappings to keep (oldest pruned first).
             **/
            blockHashCount: u32 & AugmentedConst<ApiType>;
            /**
             * The maximum length of a block (in bytes).
             **/
            blockLength: FrameSystemLimitsBlockLength & AugmentedConst<ApiType>;
            /**
             * Block & extrinsics weights: base values and limits.
             **/
            blockWeights: FrameSystemLimitsBlockWeights & AugmentedConst<ApiType>;
            /**
             * The weight of runtime database operations the runtime can invoke.
             **/
            dbWeight: SpWeightsRuntimeDbWeight & AugmentedConst<ApiType>;
            /**
             * The designated SS58 prefix of this chain.
             *
             * This replaces the "ss58Format" property declared in the chain spec. Reason is
             * that the runtime should know about the prefix in order to make use of it as
             * an identifier of the chain.
             **/
            ss58Prefix: u16 & AugmentedConst<ApiType>;
            /**
             * Get the chain's in-code version.
             **/
            version: SpVersionRuntimeVersion & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        tanssiAuthorityMapping: {
            sessionRemovalBoundary: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        tanssiInvulnerables: {
            /**
             * Maximum number of invulnerables.
             **/
            maxInvulnerables: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        timestamp: {
            /**
             * The minimum period between blocks.
             *
             * Be aware that this is different to the *expected* period that the block production
             * apparatus provides. Your chosen consensus system will generally work with this to
             * determine a sensible block time. For example, in the Aura pallet it will be double this
             * period on default settings.
             **/
            minimumPeriod: u64 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        transactionPayment: {
            /**
             * A fee multiplier for `Operational` extrinsics to compute "virtual tip" to boost their
             * `priority`
             *
             * This value is multiplied by the `final_fee` to obtain a "virtual tip" that is later
             * added to a tip component in regular `priority` calculations.
             * It means that a `Normal` transaction can front-run a similarly-sized `Operational`
             * extrinsic (with no tip), by including a tip value greater than the virtual tip.
             *
             * ```rust,ignore
             * // For `Normal`
             * let priority = priority_calc(tip);
             *
             * // For `Operational`
             * let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;
             * let priority = priority_calc(tip + virtual_tip);
             * ```
             *
             * Note that since we use `final_fee` the multiplier applies also to the regular `tip`
             * sent with the transaction. So, not only does the transaction get a priority bump based
             * on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`
             * transactions.
             **/
            operationalFeeMultiplier: u8 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        treasury: {
            /**
             * Percentage of spare funds (if any) that are burnt per spend period.
             **/
            burn: Permill & AugmentedConst<ApiType>;
            /**
             * DEPRECATED: associated with `spend_local` call and will be removed in May 2025.
             * Refer to <https://github.com/paritytech/polkadot-sdk/pull/5961> for migration to `spend`.
             *
             * The maximum number of approvals that can wait in the spending queue.
             *
             * NOTE: This parameter is also used within the Bounties Pallet extension if enabled.
             **/
            maxApprovals: u32 & AugmentedConst<ApiType>;
            /**
             * The treasury's pallet id, used for deriving its sovereign account ID.
             **/
            palletId: FrameSupportPalletId & AugmentedConst<ApiType>;
            /**
             * The period during which an approved treasury spend has to be claimed.
             **/
            payoutPeriod: u32 & AugmentedConst<ApiType>;
            /**
             * Period between successive spends.
             **/
            spendPeriod: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
        utility: {
            /**
             * The limit on the number of batched calls.
             **/
            batchedCallsLimit: u32 & AugmentedConst<ApiType>;
            /**
             * Generic const
             **/
            [key: string]: Codec;
        };
    } // AugmentedConsts
} // declare module
