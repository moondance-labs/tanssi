// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/events";

import type { ApiTypes, AugmentedEvent } from "@polkadot/api-base/types";
import type {
    Bytes,
    Null,
    Option,
    Result,
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
import type { ITuple } from "@polkadot/types-codec/types";
import type { AccountId32, H160, H256, Perbill } from "@polkadot/types/interfaces/runtime";
import type {
    DancelightRuntimeAggregateMessageOrigin,
    DancelightRuntimeProxyType,
    DancelightRuntimeRuntimeParametersKey,
    DancelightRuntimeRuntimeParametersValue,
    FrameSupportDispatchDispatchInfo,
    FrameSupportDispatchPostDispatchInfo,
    FrameSupportMessagesProcessMessageError,
    FrameSupportPreimagesBounded,
    FrameSupportTokensMiscBalanceStatus,
    PalletConvictionVotingTally,
    PalletExternalValidatorsForcing,
    PalletMultisigTimepoint,
    PalletRankedCollectiveTally,
    PalletRankedCollectiveVoteRecord,
    PolkadotParachainPrimitivesPrimitivesHrmpChannelId,
    PolkadotPrimitivesV7CandidateReceipt,
    PolkadotRuntimeParachainsDisputesDisputeLocation,
    PolkadotRuntimeParachainsDisputesDisputeResult,
    SnowbridgeCoreChannelId,
    SnowbridgeCoreOperatingModeBasicOperatingMode,
    SnowbridgeCoreOutboundV1OperatingMode,
    SnowbridgeCorePricingPricingParameters,
    SpConsensusGrandpaAppPublic,
    SpRuntimeDispatchError,
    SpRuntimeDispatchErrorWithPostInfo,
    SpWeightsWeightV2Weight,
    StagingXcmV4AssetAssets,
    StagingXcmV4Location,
    StagingXcmV4Response,
    StagingXcmV4TraitsOutcome,
    StagingXcmV4Xcm,
    XcmV3TraitsError,
    XcmVersionedAssets,
    XcmVersionedLocation,
} from "@polkadot/types/lookup";

export type __AugmentedEvent<ApiType extends ApiTypes> = AugmentedEvent<ApiType>;

declare module "@polkadot/api-base/types/events" {
    interface AugmentedEvents<ApiType extends ApiTypes> {
        assetRate: {
            AssetRateCreated: AugmentedEvent<ApiType, [assetKind: Null, rate: u128], { assetKind: Null; rate: u128 }>;
            AssetRateRemoved: AugmentedEvent<ApiType, [assetKind: Null], { assetKind: Null }>;
            AssetRateUpdated: AugmentedEvent<
                ApiType,
                [assetKind: Null, old: u128, new_: u128],
                { assetKind: Null; old: u128; new_: u128 }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        authorNoting: {
            /** Latest author changed */
            LatestAuthorChanged: AugmentedEvent<
                ApiType,
                [paraId: u32, blockNumber: u32, newAuthor: AccountId32, latestSlotNumber: u64],
                { paraId: u32; blockNumber: u32; newAuthor: AccountId32; latestSlotNumber: u64 }
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
                    destinationStatus: FrameSupportTokensMiscBalanceStatus,
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
            /** The `TotalIssuance` was forcefully changed. */
            TotalIssuanceForced: AugmentedEvent<ApiType, [old: u128, new_: u128], { old: u128; new_: u128 }>;
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
        containerRegistrar: {
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
            /** Para manager has changed */
            ParaManagerChanged: AugmentedEvent<
                ApiType,
                [paraId: u32, managerAddress: AccountId32],
                { paraId: u32; managerAddress: AccountId32 }
            >;
            /** Parathread params changed */
            ParathreadParamsChanged: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        convictionVoting: {
            /** An account has delegated their vote to another account. [who, target] */
            Delegated: AugmentedEvent<ApiType, [AccountId32, AccountId32]>;
            /** An [account] has cancelled a previous delegation operation. */
            Undelegated: AugmentedEvent<ApiType, [AccountId32]>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        dataPreservers: {
            AssignmentStarted: AugmentedEvent<ApiType, [profileId: u64, paraId: u32], { profileId: u64; paraId: u32 }>;
            AssignmentStopped: AugmentedEvent<ApiType, [profileId: u64, paraId: u32], { profileId: u64; paraId: u32 }>;
            /** The list of boot_nodes changed. */
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
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        ethereumBeaconClient: {
            BeaconHeaderImported: AugmentedEvent<ApiType, [blockHash: H256, slot: u64], { blockHash: H256; slot: u64 }>;
            /** Set OperatingMode */
            OperatingModeChanged: AugmentedEvent<
                ApiType,
                [mode: SnowbridgeCoreOperatingModeBasicOperatingMode],
                { mode: SnowbridgeCoreOperatingModeBasicOperatingMode }
            >;
            SyncCommitteeUpdated: AugmentedEvent<ApiType, [period: u64], { period: u64 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        ethereumOutboundQueue: {
            /**
             * Message will be committed at the end of current block. From now on, to track the progress the message, use the
             * `nonce` of `id`.
             */
            MessageAccepted: AugmentedEvent<ApiType, [id: H256, nonce: u64], { id: H256; nonce: u64 }>;
            /** Message has been queued and will be processed in the future */
            MessageQueued: AugmentedEvent<ApiType, [id: H256], { id: H256 }>;
            /** Some messages have been committed */
            MessagesCommitted: AugmentedEvent<ApiType, [root: H256, count: u64], { root: H256; count: u64 }>;
            /** Set OperatingMode */
            OperatingModeChanged: AugmentedEvent<
                ApiType,
                [mode: SnowbridgeCoreOperatingModeBasicOperatingMode],
                { mode: SnowbridgeCoreOperatingModeBasicOperatingMode }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        ethereumSystem: {
            /** An CreateAgent message was sent to the Gateway */
            CreateAgent: AugmentedEvent<
                ApiType,
                [location: StagingXcmV4Location, agentId: H256],
                { location: StagingXcmV4Location; agentId: H256 }
            >;
            /** An CreateChannel message was sent to the Gateway */
            CreateChannel: AugmentedEvent<
                ApiType,
                [channelId: SnowbridgeCoreChannelId, agentId: H256],
                { channelId: SnowbridgeCoreChannelId; agentId: H256 }
            >;
            PricingParametersChanged: AugmentedEvent<
                ApiType,
                [params: SnowbridgeCorePricingPricingParameters],
                { params: SnowbridgeCorePricingPricingParameters }
            >;
            /** An SetOperatingMode message was sent to the Gateway */
            SetOperatingMode: AugmentedEvent<
                ApiType,
                [mode: SnowbridgeCoreOutboundV1OperatingMode],
                { mode: SnowbridgeCoreOutboundV1OperatingMode }
            >;
            /** A SetTokenTransferFees message was sent to the Gateway */
            SetTokenTransferFees: AugmentedEvent<
                ApiType,
                [createAssetXcm: u128, transferAssetXcm: u128, registerToken: U256],
                { createAssetXcm: u128; transferAssetXcm: u128; registerToken: U256 }
            >;
            /** An TransferNativeFromAgent message was sent to the Gateway */
            TransferNativeFromAgent: AugmentedEvent<
                ApiType,
                [agentId: H256, recipient: H160, amount: u128],
                { agentId: H256; recipient: H160; amount: u128 }
            >;
            /** An UpdateChannel message was sent to the Gateway */
            UpdateChannel: AugmentedEvent<
                ApiType,
                [channelId: SnowbridgeCoreChannelId, mode: SnowbridgeCoreOutboundV1OperatingMode],
                { channelId: SnowbridgeCoreChannelId; mode: SnowbridgeCoreOutboundV1OperatingMode }
            >;
            /** An Upgrade message was sent to the Gateway */
            Upgrade: AugmentedEvent<
                ApiType,
                [implAddress: H160, implCodeHash: H256, initializerParamsHash: Option<H256>],
                { implAddress: H160; implCodeHash: H256; initializerParamsHash: Option<H256> }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        externalValidators: {
            /** A new force era mode was set. */
            ForceEra: AugmentedEvent<
                ApiType,
                [mode: PalletExternalValidatorsForcing],
                { mode: PalletExternalValidatorsForcing }
            >;
            /** A new era has started. */
            NewEra: AugmentedEvent<ApiType, [era: u32], { era: u32 }>;
            /** A new whitelisted validator was added. */
            WhitelistedValidatorAdded: AugmentedEvent<ApiType, [accountId: AccountId32], { accountId: AccountId32 }>;
            /** A whitelisted validator was removed. */
            WhitelistedValidatorRemoved: AugmentedEvent<ApiType, [accountId: AccountId32], { accountId: AccountId32 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        externalValidatorSlashes: {
            /** Removed author data */
            SlashReported: AugmentedEvent<
                ApiType,
                [validator: AccountId32, fraction: Perbill, slashEra: u32],
                { validator: AccountId32; fraction: Perbill; slashEra: u32 }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        fellowshipCollective: {
            /** A member `who` has been added. */
            MemberAdded: AugmentedEvent<ApiType, [who: AccountId32], { who: AccountId32 }>;
            /** The member `who` had their `AccountId` changed to `new_who`. */
            MemberExchanged: AugmentedEvent<
                ApiType,
                [who: AccountId32, newWho: AccountId32],
                { who: AccountId32; newWho: AccountId32 }
            >;
            /** The member `who` of given `rank` has been removed from the collective. */
            MemberRemoved: AugmentedEvent<ApiType, [who: AccountId32, rank: u16], { who: AccountId32; rank: u16 }>;
            /** The member `who`se rank has been changed to the given `rank`. */
            RankChanged: AugmentedEvent<ApiType, [who: AccountId32, rank: u16], { who: AccountId32; rank: u16 }>;
            /** The member `who` has voted for the `poll` with the given `vote` leading to an updated `tally`. */
            Voted: AugmentedEvent<
                ApiType,
                [
                    who: AccountId32,
                    poll: u32,
                    vote: PalletRankedCollectiveVoteRecord,
                    tally: PalletRankedCollectiveTally,
                ],
                {
                    who: AccountId32;
                    poll: u32;
                    vote: PalletRankedCollectiveVoteRecord;
                    tally: PalletRankedCollectiveTally;
                }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        fellowshipReferenda: {
            /** A referendum has been approved and its proposal has been scheduled. */
            Approved: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /** A referendum has been cancelled. */
            Cancelled: AugmentedEvent<
                ApiType,
                [index: u32, tally: PalletRankedCollectiveTally],
                { index: u32; tally: PalletRankedCollectiveTally }
            >;
            ConfirmAborted: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /** A referendum has ended its confirmation phase and is ready for approval. */
            Confirmed: AugmentedEvent<
                ApiType,
                [index: u32, tally: PalletRankedCollectiveTally],
                { index: u32; tally: PalletRankedCollectiveTally }
            >;
            ConfirmStarted: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /** The decision deposit has been placed. */
            DecisionDepositPlaced: AugmentedEvent<
                ApiType,
                [index: u32, who: AccountId32, amount: u128],
                { index: u32; who: AccountId32; amount: u128 }
            >;
            /** The decision deposit has been refunded. */
            DecisionDepositRefunded: AugmentedEvent<
                ApiType,
                [index: u32, who: AccountId32, amount: u128],
                { index: u32; who: AccountId32; amount: u128 }
            >;
            /** A referendum has moved into the deciding phase. */
            DecisionStarted: AugmentedEvent<
                ApiType,
                [index: u32, track: u16, proposal: FrameSupportPreimagesBounded, tally: PalletRankedCollectiveTally],
                { index: u32; track: u16; proposal: FrameSupportPreimagesBounded; tally: PalletRankedCollectiveTally }
            >;
            /** A deposit has been slashed. */
            DepositSlashed: AugmentedEvent<
                ApiType,
                [who: AccountId32, amount: u128],
                { who: AccountId32; amount: u128 }
            >;
            /** A referendum has been killed. */
            Killed: AugmentedEvent<
                ApiType,
                [index: u32, tally: PalletRankedCollectiveTally],
                { index: u32; tally: PalletRankedCollectiveTally }
            >;
            /** Metadata for a referendum has been cleared. */
            MetadataCleared: AugmentedEvent<ApiType, [index: u32, hash_: H256], { index: u32; hash_: H256 }>;
            /** Metadata for a referendum has been set. */
            MetadataSet: AugmentedEvent<ApiType, [index: u32, hash_: H256], { index: u32; hash_: H256 }>;
            /** A proposal has been rejected by referendum. */
            Rejected: AugmentedEvent<
                ApiType,
                [index: u32, tally: PalletRankedCollectiveTally],
                { index: u32; tally: PalletRankedCollectiveTally }
            >;
            /** The submission deposit has been refunded. */
            SubmissionDepositRefunded: AugmentedEvent<
                ApiType,
                [index: u32, who: AccountId32, amount: u128],
                { index: u32; who: AccountId32; amount: u128 }
            >;
            /** A referendum has been submitted. */
            Submitted: AugmentedEvent<
                ApiType,
                [index: u32, track: u16, proposal: FrameSupportPreimagesBounded],
                { index: u32; track: u16; proposal: FrameSupportPreimagesBounded }
            >;
            /** A referendum has been timed out without being decided. */
            TimedOut: AugmentedEvent<
                ApiType,
                [index: u32, tally: PalletRankedCollectiveTally],
                { index: u32; tally: PalletRankedCollectiveTally }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        grandpa: {
            /** New authority set has been applied. */
            NewAuthorities: AugmentedEvent<
                ApiType,
                [authoritySet: Vec<ITuple<[SpConsensusGrandpaAppPublic, u64]>>],
                { authoritySet: Vec<ITuple<[SpConsensusGrandpaAppPublic, u64]>> }
            >;
            /** Current authority set has been paused. */
            Paused: AugmentedEvent<ApiType, []>;
            /** Current authority set has been resumed. */
            Resumed: AugmentedEvent<ApiType, []>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        hrmp: {
            /** HRMP channel closed. */
            ChannelClosed: AugmentedEvent<
                ApiType,
                [byParachain: u32, channelId: PolkadotParachainPrimitivesPrimitivesHrmpChannelId],
                { byParachain: u32; channelId: PolkadotParachainPrimitivesPrimitivesHrmpChannelId }
            >;
            /** An HRMP channel was opened via Root origin. */
            HrmpChannelForceOpened: AugmentedEvent<
                ApiType,
                [sender: u32, recipient: u32, proposedMaxCapacity: u32, proposedMaxMessageSize: u32],
                { sender: u32; recipient: u32; proposedMaxCapacity: u32; proposedMaxMessageSize: u32 }
            >;
            /** An HRMP channel was opened with a system chain. */
            HrmpSystemChannelOpened: AugmentedEvent<
                ApiType,
                [sender: u32, recipient: u32, proposedMaxCapacity: u32, proposedMaxMessageSize: u32],
                { sender: u32; recipient: u32; proposedMaxCapacity: u32; proposedMaxMessageSize: u32 }
            >;
            /** Open HRMP channel accepted. */
            OpenChannelAccepted: AugmentedEvent<
                ApiType,
                [sender: u32, recipient: u32],
                { sender: u32; recipient: u32 }
            >;
            /** An HRMP channel request sent by the receiver was canceled by either party. */
            OpenChannelCanceled: AugmentedEvent<
                ApiType,
                [byParachain: u32, channelId: PolkadotParachainPrimitivesPrimitivesHrmpChannelId],
                { byParachain: u32; channelId: PolkadotParachainPrimitivesPrimitivesHrmpChannelId }
            >;
            /** An HRMP channel's deposits were updated. */
            OpenChannelDepositsUpdated: AugmentedEvent<
                ApiType,
                [sender: u32, recipient: u32],
                { sender: u32; recipient: u32 }
            >;
            /** Open HRMP channel requested. */
            OpenChannelRequested: AugmentedEvent<
                ApiType,
                [sender: u32, recipient: u32, proposedMaxCapacity: u32, proposedMaxMessageSize: u32],
                { sender: u32; recipient: u32; proposedMaxCapacity: u32; proposedMaxMessageSize: u32 }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        identity: {
            /** A username authority was added. */
            AuthorityAdded: AugmentedEvent<ApiType, [authority: AccountId32], { authority: AccountId32 }>;
            /** A username authority was removed. */
            AuthorityRemoved: AugmentedEvent<ApiType, [authority: AccountId32], { authority: AccountId32 }>;
            /**
             * A dangling username (as in, a username corresponding to an account that has removed its identity) has been
             * removed.
             */
            DanglingUsernameRemoved: AugmentedEvent<
                ApiType,
                [who: AccountId32, username: Bytes],
                { who: AccountId32; username: Bytes }
            >;
            /** A name was cleared, and the given balance returned. */
            IdentityCleared: AugmentedEvent<
                ApiType,
                [who: AccountId32, deposit: u128],
                { who: AccountId32; deposit: u128 }
            >;
            /** A name was removed and the given balance slashed. */
            IdentityKilled: AugmentedEvent<
                ApiType,
                [who: AccountId32, deposit: u128],
                { who: AccountId32; deposit: u128 }
            >;
            /** A name was set or reset (which will remove all judgements). */
            IdentitySet: AugmentedEvent<ApiType, [who: AccountId32], { who: AccountId32 }>;
            /** A judgement was given by a registrar. */
            JudgementGiven: AugmentedEvent<
                ApiType,
                [target: AccountId32, registrarIndex: u32],
                { target: AccountId32; registrarIndex: u32 }
            >;
            /** A judgement was asked from a registrar. */
            JudgementRequested: AugmentedEvent<
                ApiType,
                [who: AccountId32, registrarIndex: u32],
                { who: AccountId32; registrarIndex: u32 }
            >;
            /** A judgement request was retracted. */
            JudgementUnrequested: AugmentedEvent<
                ApiType,
                [who: AccountId32, registrarIndex: u32],
                { who: AccountId32; registrarIndex: u32 }
            >;
            /** A queued username passed its expiration without being claimed and was removed. */
            PreapprovalExpired: AugmentedEvent<ApiType, [whose: AccountId32], { whose: AccountId32 }>;
            /** A username was set as a primary and can be looked up from `who`. */
            PrimaryUsernameSet: AugmentedEvent<
                ApiType,
                [who: AccountId32, username: Bytes],
                { who: AccountId32; username: Bytes }
            >;
            /** A registrar was added. */
            RegistrarAdded: AugmentedEvent<ApiType, [registrarIndex: u32], { registrarIndex: u32 }>;
            /** A sub-identity was added to an identity and the deposit paid. */
            SubIdentityAdded: AugmentedEvent<
                ApiType,
                [sub: AccountId32, main: AccountId32, deposit: u128],
                { sub: AccountId32; main: AccountId32; deposit: u128 }
            >;
            /** A sub-identity was removed from an identity and the deposit freed. */
            SubIdentityRemoved: AugmentedEvent<
                ApiType,
                [sub: AccountId32, main: AccountId32, deposit: u128],
                { sub: AccountId32; main: AccountId32; deposit: u128 }
            >;
            /**
             * A sub-identity was cleared, and the given deposit repatriated from the main identity account to the
             * sub-identity account.
             */
            SubIdentityRevoked: AugmentedEvent<
                ApiType,
                [sub: AccountId32, main: AccountId32, deposit: u128],
                { sub: AccountId32; main: AccountId32; deposit: u128 }
            >;
            /** A username was queued, but `who` must accept it prior to `expiration`. */
            UsernameQueued: AugmentedEvent<
                ApiType,
                [who: AccountId32, username: Bytes, expiration: u32],
                { who: AccountId32; username: Bytes; expiration: u32 }
            >;
            /** A username was set for `who`. */
            UsernameSet: AugmentedEvent<
                ApiType,
                [who: AccountId32, username: Bytes],
                { who: AccountId32; username: Bytes }
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
        messageQueue: {
            /** Message placed in overweight queue. */
            OverweightEnqueued: AugmentedEvent<
                ApiType,
                [id: U8aFixed, origin: DancelightRuntimeAggregateMessageOrigin, pageIndex: u32, messageIndex: u32],
                { id: U8aFixed; origin: DancelightRuntimeAggregateMessageOrigin; pageIndex: u32; messageIndex: u32 }
            >;
            /** This page was reaped. */
            PageReaped: AugmentedEvent<
                ApiType,
                [origin: DancelightRuntimeAggregateMessageOrigin, index: u32],
                { origin: DancelightRuntimeAggregateMessageOrigin; index: u32 }
            >;
            /** Message is processed. */
            Processed: AugmentedEvent<
                ApiType,
                [
                    id: H256,
                    origin: DancelightRuntimeAggregateMessageOrigin,
                    weightUsed: SpWeightsWeightV2Weight,
                    success: bool,
                ],
                {
                    id: H256;
                    origin: DancelightRuntimeAggregateMessageOrigin;
                    weightUsed: SpWeightsWeightV2Weight;
                    success: bool;
                }
            >;
            /** Message discarded due to an error in the `MessageProcessor` (usually a format error). */
            ProcessingFailed: AugmentedEvent<
                ApiType,
                [
                    id: H256,
                    origin: DancelightRuntimeAggregateMessageOrigin,
                    error: FrameSupportMessagesProcessMessageError,
                ],
                {
                    id: H256;
                    origin: DancelightRuntimeAggregateMessageOrigin;
                    error: FrameSupportMessagesProcessMessageError;
                }
            >;
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
        multiBlockMigrations: {
            /** The set of historical migrations has been cleared. */
            HistoricCleared: AugmentedEvent<ApiType, [nextCursor: Option<Bytes>], { nextCursor: Option<Bytes> }>;
            /** A migration progressed. */
            MigrationAdvanced: AugmentedEvent<ApiType, [index: u32, took: u32], { index: u32; took: u32 }>;
            /** A Migration completed. */
            MigrationCompleted: AugmentedEvent<ApiType, [index: u32, took: u32], { index: u32; took: u32 }>;
            /**
             * A Migration failed.
             *
             * This implies that the whole upgrade failed and governance intervention is required.
             */
            MigrationFailed: AugmentedEvent<ApiType, [index: u32, took: u32], { index: u32; took: u32 }>;
            /** A migration was skipped since it was already executed in the past. */
            MigrationSkipped: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /**
             * The current runtime upgrade completed.
             *
             * This implies that all of its migrations completed successfully as well.
             */
            UpgradeCompleted: AugmentedEvent<ApiType, []>;
            /**
             * Runtime upgrade failed.
             *
             * This is very bad and will require governance intervention.
             */
            UpgradeFailed: AugmentedEvent<ApiType, []>;
            /**
             * A Runtime upgrade started.
             *
             * Its end is indicated by `UpgradeCompleted` or `UpgradeFailed`.
             */
            UpgradeStarted: AugmentedEvent<ApiType, [migrations: u32], { migrations: u32 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        multisig: {
            /** A multisig operation has been approved by someone. */
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
            /** A multisig operation has been cancelled. */
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
            /** A multisig operation has been executed. */
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
            /** A new multisig operation has begun. */
            NewMultisig: AugmentedEvent<
                ApiType,
                [approving: AccountId32, multisig: AccountId32, callHash: U8aFixed],
                { approving: AccountId32; multisig: AccountId32; callHash: U8aFixed }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        offences: {
            /**
             * There is an offence reported of the given `kind` happened at the `session_index` and (kind-specific) time slot.
             * This event is not deposited for duplicate slashes. [kind, timeslot].
             */
            Offence: AugmentedEvent<ApiType, [kind: U8aFixed, timeslot: Bytes], { kind: U8aFixed; timeslot: Bytes }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        onDemandAssignmentProvider: {
            /** An order was placed at some spot price amount by orderer ordered_by */
            OnDemandOrderPlaced: AugmentedEvent<
                ApiType,
                [paraId: u32, spotPrice: u128, orderedBy: AccountId32],
                { paraId: u32; spotPrice: u128; orderedBy: AccountId32 }
            >;
            /** The value of the spot price has likely changed */
            SpotPriceSet: AugmentedEvent<ApiType, [spotPrice: u128], { spotPrice: u128 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        paraInclusion: {
            /** A candidate was backed. `[candidate, head_data]` */
            CandidateBacked: AugmentedEvent<ApiType, [PolkadotPrimitivesV7CandidateReceipt, Bytes, u32, u32]>;
            /** A candidate was included. `[candidate, head_data]` */
            CandidateIncluded: AugmentedEvent<ApiType, [PolkadotPrimitivesV7CandidateReceipt, Bytes, u32, u32]>;
            /** A candidate timed out. `[candidate, head_data]` */
            CandidateTimedOut: AugmentedEvent<ApiType, [PolkadotPrimitivesV7CandidateReceipt, Bytes, u32]>;
            /** Some upward messages have been received and will be processed. */
            UpwardMessagesReceived: AugmentedEvent<ApiType, [from: u32, count: u32], { from: u32; count: u32 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        parameters: {
            /**
             * A Parameter was set.
             *
             * Is also emitted when the value was not changed.
             */
            Updated: AugmentedEvent<
                ApiType,
                [
                    key: DancelightRuntimeRuntimeParametersKey,
                    oldValue: Option<DancelightRuntimeRuntimeParametersValue>,
                    newValue: Option<DancelightRuntimeRuntimeParametersValue>,
                ],
                {
                    key: DancelightRuntimeRuntimeParametersKey;
                    oldValue: Option<DancelightRuntimeRuntimeParametersValue>;
                    newValue: Option<DancelightRuntimeRuntimeParametersValue>;
                }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        paras: {
            /** A para has been queued to execute pending actions. `para_id` */
            ActionQueued: AugmentedEvent<ApiType, [u32, u32]>;
            /** A code upgrade has been scheduled for a Para. `para_id` */
            CodeUpgradeScheduled: AugmentedEvent<ApiType, [u32]>;
            /** Current code has been updated for a Para. `para_id` */
            CurrentCodeUpdated: AugmentedEvent<ApiType, [u32]>;
            /** Current head has been updated for a Para. `para_id` */
            CurrentHeadUpdated: AugmentedEvent<ApiType, [u32]>;
            /** A new head has been noted for a Para. `para_id` */
            NewHeadNoted: AugmentedEvent<ApiType, [u32]>;
            /** The given validation code was accepted by the PVF pre-checking vote. `code_hash` `para_id` */
            PvfCheckAccepted: AugmentedEvent<ApiType, [H256, u32]>;
            /** The given validation code was rejected by the PVF pre-checking vote. `code_hash` `para_id` */
            PvfCheckRejected: AugmentedEvent<ApiType, [H256, u32]>;
            /**
             * The given para either initiated or subscribed to a PVF check for the given validation code. `code_hash`
             * `para_id`
             */
            PvfCheckStarted: AugmentedEvent<ApiType, [H256, u32]>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        parasDisputes: {
            /** A dispute has concluded for or against a candidate. `\[para id, candidate hash, dispute result\]` */
            DisputeConcluded: AugmentedEvent<ApiType, [H256, PolkadotRuntimeParachainsDisputesDisputeResult]>;
            /** A dispute has been initiated. [candidate hash, dispute location] */
            DisputeInitiated: AugmentedEvent<ApiType, [H256, PolkadotRuntimeParachainsDisputesDisputeLocation]>;
            /**
             * A dispute has concluded with supermajority against a candidate. Block authors should no longer build on top of
             * this head and should instead revert the block at the given height. This should be the number of the child of
             * the last known valid block in the chain.
             */
            Revert: AugmentedEvent<ApiType, [u32]>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        preimage: {
            /** A preimage has ben cleared. */
            Cleared: AugmentedEvent<ApiType, [hash_: H256], { hash_: H256 }>;
            /** A preimage has been noted. */
            Noted: AugmentedEvent<ApiType, [hash_: H256], { hash_: H256 }>;
            /** A preimage has been requested. */
            Requested: AugmentedEvent<ApiType, [hash_: H256], { hash_: H256 }>;
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
                [delegator: AccountId32, delegatee: AccountId32, proxyType: DancelightRuntimeProxyType, delay: u32],
                { delegator: AccountId32; delegatee: AccountId32; proxyType: DancelightRuntimeProxyType; delay: u32 }
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
                [delegator: AccountId32, delegatee: AccountId32, proxyType: DancelightRuntimeProxyType, delay: u32],
                { delegator: AccountId32; delegatee: AccountId32; proxyType: DancelightRuntimeProxyType; delay: u32 }
            >;
            /** A pure account has been created by new proxy with given disambiguation index and proxy type. */
            PureCreated: AugmentedEvent<
                ApiType,
                [pure: AccountId32, who: AccountId32, proxyType: DancelightRuntimeProxyType, disambiguationIndex: u16],
                { pure: AccountId32; who: AccountId32; proxyType: DancelightRuntimeProxyType; disambiguationIndex: u16 }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        referenda: {
            /** A referendum has been approved and its proposal has been scheduled. */
            Approved: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /** A referendum has been cancelled. */
            Cancelled: AugmentedEvent<
                ApiType,
                [index: u32, tally: PalletConvictionVotingTally],
                { index: u32; tally: PalletConvictionVotingTally }
            >;
            ConfirmAborted: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /** A referendum has ended its confirmation phase and is ready for approval. */
            Confirmed: AugmentedEvent<
                ApiType,
                [index: u32, tally: PalletConvictionVotingTally],
                { index: u32; tally: PalletConvictionVotingTally }
            >;
            ConfirmStarted: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /** The decision deposit has been placed. */
            DecisionDepositPlaced: AugmentedEvent<
                ApiType,
                [index: u32, who: AccountId32, amount: u128],
                { index: u32; who: AccountId32; amount: u128 }
            >;
            /** The decision deposit has been refunded. */
            DecisionDepositRefunded: AugmentedEvent<
                ApiType,
                [index: u32, who: AccountId32, amount: u128],
                { index: u32; who: AccountId32; amount: u128 }
            >;
            /** A referendum has moved into the deciding phase. */
            DecisionStarted: AugmentedEvent<
                ApiType,
                [index: u32, track: u16, proposal: FrameSupportPreimagesBounded, tally: PalletConvictionVotingTally],
                { index: u32; track: u16; proposal: FrameSupportPreimagesBounded; tally: PalletConvictionVotingTally }
            >;
            /** A deposit has been slashed. */
            DepositSlashed: AugmentedEvent<
                ApiType,
                [who: AccountId32, amount: u128],
                { who: AccountId32; amount: u128 }
            >;
            /** A referendum has been killed. */
            Killed: AugmentedEvent<
                ApiType,
                [index: u32, tally: PalletConvictionVotingTally],
                { index: u32; tally: PalletConvictionVotingTally }
            >;
            /** Metadata for a referendum has been cleared. */
            MetadataCleared: AugmentedEvent<ApiType, [index: u32, hash_: H256], { index: u32; hash_: H256 }>;
            /** Metadata for a referendum has been set. */
            MetadataSet: AugmentedEvent<ApiType, [index: u32, hash_: H256], { index: u32; hash_: H256 }>;
            /** A proposal has been rejected by referendum. */
            Rejected: AugmentedEvent<
                ApiType,
                [index: u32, tally: PalletConvictionVotingTally],
                { index: u32; tally: PalletConvictionVotingTally }
            >;
            /** The submission deposit has been refunded. */
            SubmissionDepositRefunded: AugmentedEvent<
                ApiType,
                [index: u32, who: AccountId32, amount: u128],
                { index: u32; who: AccountId32; amount: u128 }
            >;
            /** A referendum has been submitted. */
            Submitted: AugmentedEvent<
                ApiType,
                [index: u32, track: u16, proposal: FrameSupportPreimagesBounded],
                { index: u32; track: u16; proposal: FrameSupportPreimagesBounded }
            >;
            /** A referendum has been timed out without being decided. */
            TimedOut: AugmentedEvent<
                ApiType,
                [index: u32, tally: PalletConvictionVotingTally],
                { index: u32; tally: PalletConvictionVotingTally }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        registrar: {
            Deregistered: AugmentedEvent<ApiType, [paraId: u32], { paraId: u32 }>;
            Registered: AugmentedEvent<
                ApiType,
                [paraId: u32, manager: AccountId32],
                { paraId: u32; manager: AccountId32 }
            >;
            Reserved: AugmentedEvent<ApiType, [paraId: u32, who: AccountId32], { paraId: u32; who: AccountId32 }>;
            Swapped: AugmentedEvent<ApiType, [paraId: u32, otherId: u32], { paraId: u32; otherId: u32 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        rootTesting: {
            /** Event dispatched when the trigger_defensive extrinsic is called. */
            DefensiveTestCall: AugmentedEvent<ApiType, []>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        scheduler: {
            /** The call for the provided hash was not found so the task has been aborted. */
            CallUnavailable: AugmentedEvent<
                ApiType,
                [task: ITuple<[u32, u32]>, id: Option<U8aFixed>],
                { task: ITuple<[u32, u32]>; id: Option<U8aFixed> }
            >;
            /** Canceled some task. */
            Canceled: AugmentedEvent<ApiType, [when: u32, index: u32], { when: u32; index: u32 }>;
            /** Dispatched some task. */
            Dispatched: AugmentedEvent<
                ApiType,
                [task: ITuple<[u32, u32]>, id: Option<U8aFixed>, result: Result<Null, SpRuntimeDispatchError>],
                { task: ITuple<[u32, u32]>; id: Option<U8aFixed>; result: Result<Null, SpRuntimeDispatchError> }
            >;
            /** The given task was unable to be renewed since the agenda is full at that block. */
            PeriodicFailed: AugmentedEvent<
                ApiType,
                [task: ITuple<[u32, u32]>, id: Option<U8aFixed>],
                { task: ITuple<[u32, u32]>; id: Option<U8aFixed> }
            >;
            /** The given task can never be executed since it is overweight. */
            PermanentlyOverweight: AugmentedEvent<
                ApiType,
                [task: ITuple<[u32, u32]>, id: Option<U8aFixed>],
                { task: ITuple<[u32, u32]>; id: Option<U8aFixed> }
            >;
            /** Cancel a retry configuration for some task. */
            RetryCancelled: AugmentedEvent<
                ApiType,
                [task: ITuple<[u32, u32]>, id: Option<U8aFixed>],
                { task: ITuple<[u32, u32]>; id: Option<U8aFixed> }
            >;
            /**
             * The given task was unable to be retried since the agenda is full at that block or there was not enough weight
             * to reschedule it.
             */
            RetryFailed: AugmentedEvent<
                ApiType,
                [task: ITuple<[u32, u32]>, id: Option<U8aFixed>],
                { task: ITuple<[u32, u32]>; id: Option<U8aFixed> }
            >;
            /** Set a retry configuration for some task. */
            RetrySet: AugmentedEvent<
                ApiType,
                [task: ITuple<[u32, u32]>, id: Option<U8aFixed>, period: u32, retries: u8],
                { task: ITuple<[u32, u32]>; id: Option<U8aFixed>; period: u32; retries: u8 }
            >;
            /** Scheduled some task. */
            Scheduled: AugmentedEvent<ApiType, [when: u32, index: u32], { when: u32; index: u32 }>;
            /** Generic event */
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
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        session: {
            /**
             * New session has happened. Note that the argument is the session index, not the block number as the type might
             * suggest.
             */
            NewSession: AugmentedEvent<ApiType, [sessionIndex: u32], { sessionIndex: u32 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        sudo: {
            /** The sudo key has been updated. */
            KeyChanged: AugmentedEvent<
                ApiType,
                [old: Option<AccountId32>, new_: AccountId32],
                { old: Option<AccountId32>; new_: AccountId32 }
            >;
            /** The key was permanently removed. */
            KeyRemoved: AugmentedEvent<ApiType, []>;
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
            /** An upgrade was authorized. */
            UpgradeAuthorized: AugmentedEvent<
                ApiType,
                [codeHash: H256, checkVersion: bool],
                { codeHash: H256; checkVersion: bool }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        tanssiCollatorAssignment: {
            NewPendingAssignment: AugmentedEvent<
                ApiType,
                [randomSeed: U8aFixed, fullRotation: bool, targetSession: u32],
                { randomSeed: U8aFixed; fullRotation: bool; targetSession: u32 }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        tanssiInvulnerables: {
            /** A new Invulnerable was added. */
            InvulnerableAdded: AugmentedEvent<ApiType, [accountId: AccountId32], { accountId: AccountId32 }>;
            /** An Invulnerable was removed. */
            InvulnerableRemoved: AugmentedEvent<ApiType, [accountId: AccountId32], { accountId: AccountId32 }>;
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
        treasury: {
            /** A new asset spend proposal has been approved. */
            AssetSpendApproved: AugmentedEvent<
                ApiType,
                [index: u32, assetKind: Null, amount: u128, beneficiary: AccountId32, validFrom: u32, expireAt: u32],
                { index: u32; assetKind: Null; amount: u128; beneficiary: AccountId32; validFrom: u32; expireAt: u32 }
            >;
            /** An approved spend was voided. */
            AssetSpendVoided: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /** Some funds have been allocated. */
            Awarded: AugmentedEvent<
                ApiType,
                [proposalIndex: u32, award: u128, account: AccountId32],
                { proposalIndex: u32; award: u128; account: AccountId32 }
            >;
            /** Some of our funds have been burnt. */
            Burnt: AugmentedEvent<ApiType, [burntFunds: u128], { burntFunds: u128 }>;
            /** Some funds have been deposited. */
            Deposit: AugmentedEvent<ApiType, [value: u128], { value: u128 }>;
            /** A payment happened. */
            Paid: AugmentedEvent<ApiType, [index: u32, paymentId: Null], { index: u32; paymentId: Null }>;
            /** A payment failed and can be retried. */
            PaymentFailed: AugmentedEvent<ApiType, [index: u32, paymentId: Null], { index: u32; paymentId: Null }>;
            /** Spending has finished; this is the amount that rolls over until next spend. */
            Rollover: AugmentedEvent<ApiType, [rolloverBalance: u128], { rolloverBalance: u128 }>;
            /** A new spend proposal has been approved. */
            SpendApproved: AugmentedEvent<
                ApiType,
                [proposalIndex: u32, amount: u128, beneficiary: AccountId32],
                { proposalIndex: u32; amount: u128; beneficiary: AccountId32 }
            >;
            /** We have ended a spend period and will now allocate funds. */
            Spending: AugmentedEvent<ApiType, [budgetRemaining: u128], { budgetRemaining: u128 }>;
            /**
             * A spend was processed and removed from the storage. It might have been successfully paid or it may have
             * expired.
             */
            SpendProcessed: AugmentedEvent<ApiType, [index: u32], { index: u32 }>;
            /** The inactive funds of the pallet have been updated. */
            UpdatedInactive: AugmentedEvent<
                ApiType,
                [reactivated: u128, deactivated: u128],
                { reactivated: u128; deactivated: u128 }
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
        whitelist: {
            CallWhitelisted: AugmentedEvent<ApiType, [callHash: H256], { callHash: H256 }>;
            WhitelistedCallDispatched: AugmentedEvent<
                ApiType,
                [
                    callHash: H256,
                    result: Result<FrameSupportDispatchPostDispatchInfo, SpRuntimeDispatchErrorWithPostInfo>,
                ],
                {
                    callHash: H256;
                    result: Result<FrameSupportDispatchPostDispatchInfo, SpRuntimeDispatchErrorWithPostInfo>;
                }
            >;
            WhitelistedCallRemoved: AugmentedEvent<ApiType, [callHash: H256], { callHash: H256 }>;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
        xcmPallet: {
            /** Some assets have been claimed from an asset trap */
            AssetsClaimed: AugmentedEvent<
                ApiType,
                [hash_: H256, origin: StagingXcmV4Location, assets: XcmVersionedAssets],
                { hash_: H256; origin: StagingXcmV4Location; assets: XcmVersionedAssets }
            >;
            /** Some assets have been placed in an asset trap. */
            AssetsTrapped: AugmentedEvent<
                ApiType,
                [hash_: H256, origin: StagingXcmV4Location, assets: XcmVersionedAssets],
                { hash_: H256; origin: StagingXcmV4Location; assets: XcmVersionedAssets }
            >;
            /** Execution of an XCM message was attempted. */
            Attempted: AugmentedEvent<
                ApiType,
                [outcome: StagingXcmV4TraitsOutcome],
                { outcome: StagingXcmV4TraitsOutcome }
            >;
            /** Fees were paid from a location for an operation (often for using `SendXcm`). */
            FeesPaid: AugmentedEvent<
                ApiType,
                [paying: StagingXcmV4Location, fees: StagingXcmV4AssetAssets],
                { paying: StagingXcmV4Location; fees: StagingXcmV4AssetAssets }
            >;
            /**
             * Expected query response has been received but the querier location of the response does not match the expected.
             * The query remains registered for a later, valid, response to be received and acted upon.
             */
            InvalidQuerier: AugmentedEvent<
                ApiType,
                [
                    origin: StagingXcmV4Location,
                    queryId: u64,
                    expectedQuerier: StagingXcmV4Location,
                    maybeActualQuerier: Option<StagingXcmV4Location>,
                ],
                {
                    origin: StagingXcmV4Location;
                    queryId: u64;
                    expectedQuerier: StagingXcmV4Location;
                    maybeActualQuerier: Option<StagingXcmV4Location>;
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
                [origin: StagingXcmV4Location, queryId: u64],
                { origin: StagingXcmV4Location; queryId: u64 }
            >;
            /**
             * Expected query response has been received but the origin location of the response does not match that expected.
             * The query remains registered for a later, valid, response to be received and acted upon.
             */
            InvalidResponder: AugmentedEvent<
                ApiType,
                [origin: StagingXcmV4Location, queryId: u64, expectedLocation: Option<StagingXcmV4Location>],
                { origin: StagingXcmV4Location; queryId: u64; expectedLocation: Option<StagingXcmV4Location> }
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
                [origin: StagingXcmV4Location, queryId: u64],
                { origin: StagingXcmV4Location; queryId: u64 }
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
            /**
             * Query response has been received and query is removed. There was a general error with dispatching the
             * notification call.
             */
            NotifyDispatchError: AugmentedEvent<
                ApiType,
                [queryId: u64, palletIndex: u8, callIndex: u8],
                { queryId: u64; palletIndex: u8; callIndex: u8 }
            >;
            /**
             * Query response has been received and query is removed. The registered notification could not be dispatched
             * because the dispatch weight is greater than the maximum weight originally budgeted by this runtime for the
             * query result.
             */
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
             * A given location which had a version change subscription was dropped owing to an error migrating the location
             * to our new XCM format.
             */
            NotifyTargetMigrationFail: AugmentedEvent<
                ApiType,
                [location: XcmVersionedLocation, queryId: u64],
                { location: XcmVersionedLocation; queryId: u64 }
            >;
            /**
             * A given location which had a version change subscription was dropped owing to an error sending the notification
             * to it.
             */
            NotifyTargetSendFail: AugmentedEvent<
                ApiType,
                [location: StagingXcmV4Location, queryId: u64, error: XcmV3TraitsError],
                { location: StagingXcmV4Location; queryId: u64; error: XcmV3TraitsError }
            >;
            /**
             * Query response has been received and is ready for taking with `take_response`. There is no registered
             * notification call.
             */
            ResponseReady: AugmentedEvent<
                ApiType,
                [queryId: u64, response: StagingXcmV4Response],
                { queryId: u64; response: StagingXcmV4Response }
            >;
            /** Received query response has been read and removed. */
            ResponseTaken: AugmentedEvent<ApiType, [queryId: u64], { queryId: u64 }>;
            /** A XCM message was sent. */
            Sent: AugmentedEvent<
                ApiType,
                [
                    origin: StagingXcmV4Location,
                    destination: StagingXcmV4Location,
                    message: StagingXcmV4Xcm,
                    messageId: U8aFixed,
                ],
                {
                    origin: StagingXcmV4Location;
                    destination: StagingXcmV4Location;
                    message: StagingXcmV4Xcm;
                    messageId: U8aFixed;
                }
            >;
            /**
             * The supported version of a location has been changed. This might be through an automatic notification or a
             * manual intervention.
             */
            SupportedVersionChanged: AugmentedEvent<
                ApiType,
                [location: StagingXcmV4Location, version: u32],
                { location: StagingXcmV4Location; version: u32 }
            >;
            /**
             * Query response received which does not match a registered query. This may be because a matching query was never
             * registered, it may be because it is a duplicate response, or because the query timed out.
             */
            UnexpectedResponse: AugmentedEvent<
                ApiType,
                [origin: StagingXcmV4Location, queryId: u64],
                { origin: StagingXcmV4Location; queryId: u64 }
            >;
            /**
             * An XCM version change notification message has been attempted to be sent.
             *
             * The cost of sending it (borne by the chain) is included.
             */
            VersionChangeNotified: AugmentedEvent<
                ApiType,
                [destination: StagingXcmV4Location, result: u32, cost: StagingXcmV4AssetAssets, messageId: U8aFixed],
                { destination: StagingXcmV4Location; result: u32; cost: StagingXcmV4AssetAssets; messageId: U8aFixed }
            >;
            /** A XCM version migration finished. */
            VersionMigrationFinished: AugmentedEvent<ApiType, [version: u32], { version: u32 }>;
            /** We have requested that a remote chain send us XCM version change notifications. */
            VersionNotifyRequested: AugmentedEvent<
                ApiType,
                [destination: StagingXcmV4Location, cost: StagingXcmV4AssetAssets, messageId: U8aFixed],
                { destination: StagingXcmV4Location; cost: StagingXcmV4AssetAssets; messageId: U8aFixed }
            >;
            /**
             * A remote has requested XCM version change notification from us and we have honored it. A version information
             * message is sent to them and its cost is included.
             */
            VersionNotifyStarted: AugmentedEvent<
                ApiType,
                [destination: StagingXcmV4Location, cost: StagingXcmV4AssetAssets, messageId: U8aFixed],
                { destination: StagingXcmV4Location; cost: StagingXcmV4AssetAssets; messageId: U8aFixed }
            >;
            /** We have requested that a remote chain stops sending us XCM version change notifications. */
            VersionNotifyUnrequested: AugmentedEvent<
                ApiType,
                [destination: StagingXcmV4Location, cost: StagingXcmV4AssetAssets, messageId: U8aFixed],
                { destination: StagingXcmV4Location; cost: StagingXcmV4AssetAssets; messageId: U8aFixed }
            >;
            /** Generic event */
            [key: string]: AugmentedEvent<ApiType>;
        };
    } // AugmentedEvents
} // declare module
