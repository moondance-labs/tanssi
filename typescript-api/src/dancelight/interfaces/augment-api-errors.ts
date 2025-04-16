// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/errors";

import type { ApiTypes, AugmentedError } from "@polkadot/api-base/types";

export type __AugmentedError<ApiType extends ApiTypes> = AugmentedError<ApiType>;

declare module "@polkadot/api-base/types/errors" {
    interface AugmentedErrors<ApiType extends ApiTypes> {
        assetRate: {
            /**
             * The given asset ID already has an assigned conversion rate and cannot be re-created.
             **/
            AlreadyExists: AugmentedError<ApiType>;
            /**
             * Overflow ocurred when calculating the inverse rate.
             **/
            Overflow: AugmentedError<ApiType>;
            /**
             * The given asset ID is unknown.
             **/
            UnknownAssetKind: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        authorNoting: {
            AsPreRuntimeError: AugmentedError<ApiType>;
            AuraDigestFirstItem: AugmentedError<ApiType>;
            AuthorNotFound: AugmentedError<ApiType>;
            FailedDecodingHeader: AugmentedError<ApiType>;
            /**
             * The new value for a configuration parameter is invalid.
             **/
            FailedReading: AugmentedError<ApiType>;
            NonAuraDigest: AugmentedError<ApiType>;
            NonDecodableSlot: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        babe: {
            /**
             * A given equivocation report is valid but already previously reported.
             **/
            DuplicateOffenceReport: AugmentedError<ApiType>;
            /**
             * Submitted configuration is invalid.
             **/
            InvalidConfiguration: AugmentedError<ApiType>;
            /**
             * An equivocation proof provided as part of an equivocation report is invalid.
             **/
            InvalidEquivocationProof: AugmentedError<ApiType>;
            /**
             * A key ownership proof provided as part of an equivocation report is invalid.
             **/
            InvalidKeyOwnershipProof: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        balances: {
            /**
             * Beneficiary account must pre-exist.
             **/
            DeadAccount: AugmentedError<ApiType>;
            /**
             * The delta cannot be zero.
             **/
            DeltaZero: AugmentedError<ApiType>;
            /**
             * Value too low to create account due to existential deposit.
             **/
            ExistentialDeposit: AugmentedError<ApiType>;
            /**
             * A vesting schedule already exists for this account.
             **/
            ExistingVestingSchedule: AugmentedError<ApiType>;
            /**
             * Transfer/payment would kill account.
             **/
            Expendability: AugmentedError<ApiType>;
            /**
             * Balance too low to send value.
             **/
            InsufficientBalance: AugmentedError<ApiType>;
            /**
             * The issuance cannot be modified since it is already deactivated.
             **/
            IssuanceDeactivated: AugmentedError<ApiType>;
            /**
             * Account liquidity restrictions prevent withdrawal.
             **/
            LiquidityRestrictions: AugmentedError<ApiType>;
            /**
             * Number of freezes exceed `MaxFreezes`.
             **/
            TooManyFreezes: AugmentedError<ApiType>;
            /**
             * Number of holds exceed `VariantCountOf<T::RuntimeHoldReason>`.
             **/
            TooManyHolds: AugmentedError<ApiType>;
            /**
             * Number of named reserves exceed `MaxReserves`.
             **/
            TooManyReserves: AugmentedError<ApiType>;
            /**
             * Vesting balance too high to send value.
             **/
            VestingBalance: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        beefy: {
            /**
             * A given equivocation report is valid but already previously reported.
             **/
            DuplicateOffenceReport: AugmentedError<ApiType>;
            /**
             * Submitted configuration is invalid.
             **/
            InvalidConfiguration: AugmentedError<ApiType>;
            /**
             * A double voting proof provided as part of an equivocation report is invalid.
             **/
            InvalidDoubleVotingProof: AugmentedError<ApiType>;
            /**
             * The session of the equivocation proof is invalid
             **/
            InvalidEquivocationProofSession: AugmentedError<ApiType>;
            /**
             * A fork voting proof provided as part of an equivocation report is invalid.
             **/
            InvalidForkVotingProof: AugmentedError<ApiType>;
            /**
             * A future block voting proof provided as part of an equivocation report is invalid.
             **/
            InvalidFutureBlockVotingProof: AugmentedError<ApiType>;
            /**
             * A key ownership proof provided as part of an equivocation report is invalid.
             **/
            InvalidKeyOwnershipProof: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        collatorConfiguration: {
            /**
             * The new value for a configuration parameter is invalid.
             **/
            InvalidNewValue: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        configuration: {
            /**
             * The new value for a configuration parameter is invalid.
             **/
            InvalidNewValue: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        containerRegistrar: {
            /**
             * Attempted to register a ParaId with a genesis data size greater than the limit
             **/
            GenesisDataTooBig: AugmentedError<ApiType>;
            /**
             * Tried to register a paraId in a relay context without specifying a proper HeadData.
             **/
            HeadDataNecessary: AugmentedError<ApiType>;
            /**
             * The provided signature from the parachain manager in the relay is not valid
             **/
            InvalidRelayManagerSignature: AugmentedError<ApiType>;
            /**
             * The provided relay storage proof is not valid
             **/
            InvalidRelayStorageProof: AugmentedError<ApiType>;
            /**
             * Tried to change parathread params for a para id that is not a registered parathread
             **/
            NotAParathread: AugmentedError<ApiType>;
            /**
             * Attempted to execute an extrinsic meant only for the para creator
             **/
            NotParaCreator: AugmentedError<ApiType>;
            /**
             * Tried to register a ParaId with an account that did not have enough balance for the deposit
             **/
            NotSufficientDeposit: AugmentedError<ApiType>;
            /**
             * Attempted to deregister a ParaId that is already being deregistered
             **/
            ParaIdAlreadyDeregistered: AugmentedError<ApiType>;
            /**
             * Attempted to pause a ParaId that was already paused
             **/
            ParaIdAlreadyPaused: AugmentedError<ApiType>;
            /**
             * Attempted to register a ParaId that was already registered
             **/
            ParaIdAlreadyRegistered: AugmentedError<ApiType>;
            /**
             * The bounded list of ParaIds has reached its limit
             **/
            ParaIdListFull: AugmentedError<ApiType>;
            /**
             * Tried to mark_valid_for_collating a ParaId that is not in PendingVerification
             **/
            ParaIdNotInPendingVerification: AugmentedError<ApiType>;
            /**
             * Attempted to unpause a ParaId that was not paused
             **/
            ParaIdNotPaused: AugmentedError<ApiType>;
            /**
             * Attempted to deregister a ParaId that is not registered
             **/
            ParaIdNotRegistered: AugmentedError<ApiType>;
            /**
             * Tried to deregister a parachain that was not deregistered from the relay chain
             **/
            ParaStillExistsInRelay: AugmentedError<ApiType>;
            /**
             * The relay storage root for the corresponding block number could not be retrieved
             **/
            RelayStorageRootNotFound: AugmentedError<ApiType>;
            /**
             * Tried to register a paraId in a relay context without specifying a wasm chain code.
             **/
            WasmCodeNecessary: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        convictionVoting: {
            /**
             * The account is already delegating.
             **/
            AlreadyDelegating: AugmentedError<ApiType>;
            /**
             * The account currently has votes attached to it and the operation cannot succeed until
             * these are removed through `remove_vote`.
             **/
            AlreadyVoting: AugmentedError<ApiType>;
            /**
             * The class ID supplied is invalid.
             **/
            BadClass: AugmentedError<ApiType>;
            /**
             * The class must be supplied since it is not easily determinable from the state.
             **/
            ClassNeeded: AugmentedError<ApiType>;
            /**
             * Too high a balance was provided that the account cannot afford.
             **/
            InsufficientFunds: AugmentedError<ApiType>;
            /**
             * Maximum number of votes reached.
             **/
            MaxVotesReached: AugmentedError<ApiType>;
            /**
             * Delegation to oneself makes no sense.
             **/
            Nonsense: AugmentedError<ApiType>;
            /**
             * The actor has no permission to conduct the action.
             **/
            NoPermission: AugmentedError<ApiType>;
            /**
             * The actor has no permission to conduct the action right now but will do in the future.
             **/
            NoPermissionYet: AugmentedError<ApiType>;
            /**
             * The account is not currently delegating.
             **/
            NotDelegating: AugmentedError<ApiType>;
            /**
             * Poll is not ongoing.
             **/
            NotOngoing: AugmentedError<ApiType>;
            /**
             * The given account did not vote on the poll.
             **/
            NotVoter: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        dataPreservers: {
            /**
             * Made for `AssignmentProcessor` implementors to report a mismatch between
             * `ProviderRequest` and `AssignerParameter`.
             **/
            AssignmentPaymentRequestParameterMismatch: AugmentedError<ApiType>;
            CantDeleteAssignedProfile: AugmentedError<ApiType>;
            MaxAssignmentsPerParaIdReached: AugmentedError<ApiType>;
            NextProfileIdShouldBeAvailable: AugmentedError<ApiType>;
            /**
             * This container chain does not have any boot nodes
             **/
            NoBootNodes: AugmentedError<ApiType>;
            ProfileAlreadyAssigned: AugmentedError<ApiType>;
            ProfileIsNotElligibleForParaId: AugmentedError<ApiType>;
            ProfileNotAssigned: AugmentedError<ApiType>;
            UnknownProfileId: AugmentedError<ApiType>;
            WrongParaId: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        ethereumBeaconClient: {
            BlockBodyHashTreeRootFailed: AugmentedError<ApiType>;
            BLSPreparePublicKeysFailed: AugmentedError<ApiType>;
            BLSVerificationFailed: AugmentedError<ApiType>;
            ExecutionHeaderSkippedBlock: AugmentedError<ApiType>;
            ExecutionHeaderTooFarBehind: AugmentedError<ApiType>;
            ExpectedFinalizedHeaderNotStored: AugmentedError<ApiType>;
            ForkDataHashTreeRootFailed: AugmentedError<ApiType>;
            Halted: AugmentedError<ApiType>;
            HeaderHashTreeRootFailed: AugmentedError<ApiType>;
            HeaderNotFinalized: AugmentedError<ApiType>;
            InvalidAncestryMerkleProof: AugmentedError<ApiType>;
            InvalidBlockRootsRootMerkleProof: AugmentedError<ApiType>;
            InvalidExecutionHeaderProof: AugmentedError<ApiType>;
            /**
             * The gap between the finalized headers is larger than the sync committee period,
             * rendering execution headers unprovable using ancestry proofs (blocks root size is
             * the same as the sync committee period slots).
             **/
            InvalidFinalizedHeaderGap: AugmentedError<ApiType>;
            InvalidHeaderMerkleProof: AugmentedError<ApiType>;
            InvalidSyncCommitteeMerkleProof: AugmentedError<ApiType>;
            /**
             * The given update is not in the expected period, or the given next sync committee does
             * not match the next sync committee in storage.
             **/
            InvalidSyncCommitteeUpdate: AugmentedError<ApiType>;
            InvalidUpdateSlot: AugmentedError<ApiType>;
            /**
             * Attested header is older than latest finalized header.
             **/
            IrrelevantUpdate: AugmentedError<ApiType>;
            NotBootstrapped: AugmentedError<ApiType>;
            SigningRootHashTreeRootFailed: AugmentedError<ApiType>;
            SkippedSyncCommitteePeriod: AugmentedError<ApiType>;
            SyncCommitteeHashTreeRootFailed: AugmentedError<ApiType>;
            SyncCommitteeParticipantsNotSupermajority: AugmentedError<ApiType>;
            SyncCommitteeUpdateRequired: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        ethereumInboundQueue: {
            /**
             * Message conversion error
             **/
            ConvertMessage: AugmentedError<ApiType>;
            /**
             * Pallet is halted
             **/
            Halted: AugmentedError<ApiType>;
            /**
             * Cannot convert location
             **/
            InvalidAccountConversion: AugmentedError<ApiType>;
            /**
             * Message channel is invalid
             **/
            InvalidChannel: AugmentedError<ApiType>;
            /**
             * Message has an invalid envelope.
             **/
            InvalidEnvelope: AugmentedError<ApiType>;
            /**
             * Message came from an invalid outbound channel on the Ethereum side.
             **/
            InvalidGateway: AugmentedError<ApiType>;
            /**
             * Message has an unexpected nonce.
             **/
            InvalidNonce: AugmentedError<ApiType>;
            /**
             * Message has an invalid payload.
             **/
            InvalidPayload: AugmentedError<ApiType>;
            /**
             * The max nonce for the type has been reached
             **/
            MaxNonceReached: AugmentedError<ApiType>;
            /**
             * XCMP send failure
             **/
            Send: AugmentedError<ApiType>;
            /**
             * Message verification error,
             **/
            Verification: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        ethereumOutboundQueue: {
            /**
             * The pallet is halted
             **/
            Halted: AugmentedError<ApiType>;
            /**
             * Invalid Channel
             **/
            InvalidChannel: AugmentedError<ApiType>;
            /**
             * The message is too large
             **/
            MessageTooLarge: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        ethereumSystem: {
            AgentAlreadyCreated: AugmentedError<ApiType>;
            ChannelAlreadyCreated: AugmentedError<ApiType>;
            InvalidLocation: AugmentedError<ApiType>;
            InvalidPricingParameters: AugmentedError<ApiType>;
            InvalidTokenTransferFees: AugmentedError<ApiType>;
            InvalidUpgradeParameters: AugmentedError<ApiType>;
            LocationConversionFailed: AugmentedError<ApiType>;
            NoAgent: AugmentedError<ApiType>;
            NoChannel: AugmentedError<ApiType>;
            Send: AugmentedError<ApiType>;
            UnsupportedLocationVersion: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        ethereumTokenTransfers: {
            /**
             * The channel's information has not been set on this pallet yet.
             **/
            ChannelInfoNotSet: AugmentedError<ApiType>;
            /**
             * The outbound message is invalid prior to send.
             **/
            InvalidMessage: AugmentedError<ApiType>;
            /**
             * The outbound message could not be sent.
             **/
            TransferMessageNotSent: AugmentedError<ApiType>;
            /**
             * Conversion from Location to TokenId failed.
             **/
            UnknownLocationForToken: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        externalValidators: {
            /**
             * Account is already whitelisted.
             **/
            AlreadyWhitelisted: AugmentedError<ApiType>;
            /**
             * Account does not have keys registered
             **/
            NoKeysRegistered: AugmentedError<ApiType>;
            /**
             * Account is not whitelisted.
             **/
            NotWhitelisted: AugmentedError<ApiType>;
            /**
             * There are too many whitelisted validators.
             **/
            TooManyWhitelisted: AugmentedError<ApiType>;
            /**
             * Unable to derive validator id from account id
             **/
            UnableToDeriveValidatorId: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        externalValidatorSlashes: {
            /**
             * The slash to be cancelled has already elapsed the DeferPeriod
             **/
            DeferPeriodIsOver: AugmentedError<ApiType>;
            /**
             * The era for which the slash wants to be cancelled has no slashes
             **/
            EmptyTargets: AugmentedError<ApiType>;
            /**
             * There was an error computing the slash
             **/
            ErrorComputingSlash: AugmentedError<ApiType>;
            /**
             * Failed to deliver the message to Ethereum
             **/
            EthereumDeliverFail: AugmentedError<ApiType>;
            /**
             * Failed to validate the message that was going to be sent to Ethereum
             **/
            EthereumValidateFail: AugmentedError<ApiType>;
            /**
             * No slash was found to be cancelled at the given index
             **/
            InvalidSlashIndex: AugmentedError<ApiType>;
            /**
             * Slash indices to be cancelled are not sorted or unique
             **/
            NotSortedAndUnique: AugmentedError<ApiType>;
            /**
             * Provided an era in the future
             **/
            ProvidedFutureEra: AugmentedError<ApiType>;
            /**
             * Provided an era that is not slashable
             **/
            ProvidedNonSlashableEra: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        fellowshipCollective: {
            /**
             * Account is already a member.
             **/
            AlreadyMember: AugmentedError<ApiType>;
            /**
             * Unexpected error in state.
             **/
            Corruption: AugmentedError<ApiType>;
            /**
             * The information provided is incorrect.
             **/
            InvalidWitness: AugmentedError<ApiType>;
            /**
             * There are no further records to be removed.
             **/
            NoneRemaining: AugmentedError<ApiType>;
            /**
             * The origin is not sufficiently privileged to do the operation.
             **/
            NoPermission: AugmentedError<ApiType>;
            /**
             * Account is not a member.
             **/
            NotMember: AugmentedError<ApiType>;
            /**
             * The given poll index is unknown or has closed.
             **/
            NotPolling: AugmentedError<ApiType>;
            /**
             * The given poll is still ongoing.
             **/
            Ongoing: AugmentedError<ApiType>;
            /**
             * The member's rank is too low to vote.
             **/
            RankTooLow: AugmentedError<ApiType>;
            /**
             * The new member to exchange is the same as the old member
             **/
            SameMember: AugmentedError<ApiType>;
            /**
             * The max member count for the rank has been reached.
             **/
            TooManyMembers: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        fellowshipReferenda: {
            /**
             * The referendum index provided is invalid in this context.
             **/
            BadReferendum: AugmentedError<ApiType>;
            /**
             * The referendum status is invalid for this operation.
             **/
            BadStatus: AugmentedError<ApiType>;
            /**
             * The track identifier given was invalid.
             **/
            BadTrack: AugmentedError<ApiType>;
            /**
             * There are already a full complement of referenda in progress for this track.
             **/
            Full: AugmentedError<ApiType>;
            /**
             * Referendum's decision deposit is already paid.
             **/
            HasDeposit: AugmentedError<ApiType>;
            /**
             * The deposit cannot be refunded since none was made.
             **/
            NoDeposit: AugmentedError<ApiType>;
            /**
             * The deposit refunder is not the depositor.
             **/
            NoPermission: AugmentedError<ApiType>;
            /**
             * There was nothing to do in the advancement.
             **/
            NothingToDo: AugmentedError<ApiType>;
            /**
             * Referendum is not ongoing.
             **/
            NotOngoing: AugmentedError<ApiType>;
            /**
             * No track exists for the proposal origin.
             **/
            NoTrack: AugmentedError<ApiType>;
            /**
             * The preimage does not exist.
             **/
            PreimageNotExist: AugmentedError<ApiType>;
            /**
             * The preimage is stored with a different length than the one provided.
             **/
            PreimageStoredWithDifferentLength: AugmentedError<ApiType>;
            /**
             * The queue of the track is empty.
             **/
            QueueEmpty: AugmentedError<ApiType>;
            /**
             * Any deposit cannot be refunded until after the decision is over.
             **/
            Unfinished: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        grandpa: {
            /**
             * Attempt to signal GRANDPA change with one already pending.
             **/
            ChangePending: AugmentedError<ApiType>;
            /**
             * A given equivocation report is valid but already previously reported.
             **/
            DuplicateOffenceReport: AugmentedError<ApiType>;
            /**
             * An equivocation proof provided as part of an equivocation report is invalid.
             **/
            InvalidEquivocationProof: AugmentedError<ApiType>;
            /**
             * A key ownership proof provided as part of an equivocation report is invalid.
             **/
            InvalidKeyOwnershipProof: AugmentedError<ApiType>;
            /**
             * Attempt to signal GRANDPA pause when the authority set isn't live
             * (either paused or already pending pause).
             **/
            PauseFailed: AugmentedError<ApiType>;
            /**
             * Attempt to signal GRANDPA resume when the authority set isn't paused
             * (either live or already pending resume).
             **/
            ResumeFailed: AugmentedError<ApiType>;
            /**
             * Cannot signal forced change so soon after last.
             **/
            TooSoon: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        hrmp: {
            /**
             * The channel is already confirmed.
             **/
            AcceptHrmpChannelAlreadyConfirmed: AugmentedError<ApiType>;
            /**
             * The channel from the sender to the origin doesn't exist.
             **/
            AcceptHrmpChannelDoesntExist: AugmentedError<ApiType>;
            /**
             * The recipient already has the maximum number of allowed inbound channels.
             **/
            AcceptHrmpChannelLimitExceeded: AugmentedError<ApiType>;
            /**
             * Canceling is requested by neither the sender nor recipient of the open channel request.
             **/
            CancelHrmpOpenChannelUnauthorized: AugmentedError<ApiType>;
            /**
             * The channel between these two chains cannot be authorized.
             **/
            ChannelCreationNotAuthorized: AugmentedError<ApiType>;
            /**
             * The channel close request is already requested.
             **/
            CloseHrmpChannelAlreadyUnderway: AugmentedError<ApiType>;
            /**
             * The channel to be closed doesn't exist.
             **/
            CloseHrmpChannelDoesntExist: AugmentedError<ApiType>;
            /**
             * The origin tries to close a channel where it is neither the sender nor the recipient.
             **/
            CloseHrmpChannelUnauthorized: AugmentedError<ApiType>;
            /**
             * Cannot cancel an HRMP open channel request because it is already confirmed.
             **/
            OpenHrmpChannelAlreadyConfirmed: AugmentedError<ApiType>;
            /**
             * The channel already exists
             **/
            OpenHrmpChannelAlreadyExists: AugmentedError<ApiType>;
            /**
             * There is already a request to open the same channel.
             **/
            OpenHrmpChannelAlreadyRequested: AugmentedError<ApiType>;
            /**
             * The requested capacity exceeds the global limit.
             **/
            OpenHrmpChannelCapacityExceedsLimit: AugmentedError<ApiType>;
            /**
             * The open request doesn't exist.
             **/
            OpenHrmpChannelDoesntExist: AugmentedError<ApiType>;
            /**
             * The recipient is not a valid para.
             **/
            OpenHrmpChannelInvalidRecipient: AugmentedError<ApiType>;
            /**
             * The sender already has the maximum number of allowed outbound channels.
             **/
            OpenHrmpChannelLimitExceeded: AugmentedError<ApiType>;
            /**
             * The open request requested the message size that exceeds the global limit.
             **/
            OpenHrmpChannelMessageSizeExceedsLimit: AugmentedError<ApiType>;
            /**
             * The sender tried to open a channel to themselves.
             **/
            OpenHrmpChannelToSelf: AugmentedError<ApiType>;
            /**
             * The requested capacity is zero.
             **/
            OpenHrmpChannelZeroCapacity: AugmentedError<ApiType>;
            /**
             * The requested maximum message size is 0.
             **/
            OpenHrmpChannelZeroMessageSize: AugmentedError<ApiType>;
            /**
             * The provided witness data is wrong.
             **/
            WrongWitness: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        identity: {
            /**
             * Account ID is already named.
             **/
            AlreadyClaimed: AugmentedError<ApiType>;
            /**
             * The username cannot be unbound because it is already unbinding.
             **/
            AlreadyUnbinding: AugmentedError<ApiType>;
            /**
             * Empty index.
             **/
            EmptyIndex: AugmentedError<ApiType>;
            /**
             * Fee is changed.
             **/
            FeeChanged: AugmentedError<ApiType>;
            /**
             * The action cannot be performed because of insufficient privileges (e.g. authority
             * trying to unbind a username provided by the system).
             **/
            InsufficientPrivileges: AugmentedError<ApiType>;
            /**
             * The index is invalid.
             **/
            InvalidIndex: AugmentedError<ApiType>;
            /**
             * Invalid judgement.
             **/
            InvalidJudgement: AugmentedError<ApiType>;
            /**
             * The signature on a username was not valid.
             **/
            InvalidSignature: AugmentedError<ApiType>;
            /**
             * The provided suffix is too long.
             **/
            InvalidSuffix: AugmentedError<ApiType>;
            /**
             * The target is invalid.
             **/
            InvalidTarget: AugmentedError<ApiType>;
            /**
             * The username does not meet the requirements.
             **/
            InvalidUsername: AugmentedError<ApiType>;
            /**
             * The provided judgement was for a different identity.
             **/
            JudgementForDifferentIdentity: AugmentedError<ApiType>;
            /**
             * Judgement given.
             **/
            JudgementGiven: AugmentedError<ApiType>;
            /**
             * Error that occurs when there is an issue paying for judgement.
             **/
            JudgementPaymentFailed: AugmentedError<ApiType>;
            /**
             * The authority cannot allocate any more usernames.
             **/
            NoAllocation: AugmentedError<ApiType>;
            /**
             * No identity found.
             **/
            NoIdentity: AugmentedError<ApiType>;
            /**
             * The username cannot be forcefully removed because it can still be accepted.
             **/
            NotExpired: AugmentedError<ApiType>;
            /**
             * Account isn't found.
             **/
            NotFound: AugmentedError<ApiType>;
            /**
             * Account isn't named.
             **/
            NotNamed: AugmentedError<ApiType>;
            /**
             * Sub-account isn't owned by sender.
             **/
            NotOwned: AugmentedError<ApiType>;
            /**
             * Sender is not a sub-account.
             **/
            NotSub: AugmentedError<ApiType>;
            /**
             * The username cannot be removed because it is not unbinding.
             **/
            NotUnbinding: AugmentedError<ApiType>;
            /**
             * The sender does not have permission to issue a username.
             **/
            NotUsernameAuthority: AugmentedError<ApiType>;
            /**
             * The requested username does not exist.
             **/
            NoUsername: AugmentedError<ApiType>;
            /**
             * Setting this username requires a signature, but none was provided.
             **/
            RequiresSignature: AugmentedError<ApiType>;
            /**
             * Sticky judgement.
             **/
            StickyJudgement: AugmentedError<ApiType>;
            /**
             * The username cannot be removed because it's still in the grace period.
             **/
            TooEarly: AugmentedError<ApiType>;
            /**
             * Maximum amount of registrars reached. Cannot add any more.
             **/
            TooManyRegistrars: AugmentedError<ApiType>;
            /**
             * Too many subs-accounts.
             **/
            TooManySubAccounts: AugmentedError<ApiType>;
            /**
             * The username is already taken.
             **/
            UsernameTaken: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        inactivityTracking: {
            /**
             * Error returned when the activity tracking status is attempted to be disabled when it is already disabled
             **/
            ActivityTrackingStatusAlreadyDisabled: AugmentedError<ApiType>;
            /**
             * Error returned when the activity tracking status is attempted to be enabled when it is already enabled
             **/
            ActivityTrackingStatusAlreadyEnabled: AugmentedError<ApiType>;
            /**
             * Error returned when the activity tracking status is attempted to be updated before the end session
             **/
            ActivityTrackingStatusUpdateSuspended: AugmentedError<ApiType>;
            /**
             * The size of a collator set for a session has already reached MaxCollatorsPerSession value
             **/
            MaxCollatorsPerSessionReached: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        messageQueue: {
            /**
             * The message was already processed and cannot be processed again.
             **/
            AlreadyProcessed: AugmentedError<ApiType>;
            /**
             * There is temporarily not enough weight to continue servicing messages.
             **/
            InsufficientWeight: AugmentedError<ApiType>;
            /**
             * The referenced message could not be found.
             **/
            NoMessage: AugmentedError<ApiType>;
            /**
             * Page to be reaped does not exist.
             **/
            NoPage: AugmentedError<ApiType>;
            /**
             * Page is not reapable because it has items remaining to be processed and is not old
             * enough.
             **/
            NotReapable: AugmentedError<ApiType>;
            /**
             * The message is queued for future execution.
             **/
            Queued: AugmentedError<ApiType>;
            /**
             * The queue is paused and no message can be executed from it.
             *
             * This can change at any time and may resolve in the future by re-trying.
             **/
            QueuePaused: AugmentedError<ApiType>;
            /**
             * Another call is in progress and needs to finish before this call can happen.
             **/
            RecursiveDisallowed: AugmentedError<ApiType>;
            /**
             * This message is temporarily unprocessable.
             *
             * Such errors are expected, but not guaranteed, to resolve themselves eventually through
             * retrying.
             **/
            TemporarilyUnprocessable: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        migrations: {
            /**
             * Preimage already exists in the new storage.
             **/
            PreimageAlreadyExists: AugmentedError<ApiType>;
            /**
             * Preimage is larger than the new max size.
             **/
            PreimageIsTooBig: AugmentedError<ApiType>;
            /**
             * Missing preimage in original democracy storage
             **/
            PreimageMissing: AugmentedError<ApiType>;
            /**
             * Provided upper bound is too low.
             **/
            WrongUpperBound: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        multiBlockMigrations: {
            /**
             * The operation cannot complete since some MBMs are ongoing.
             **/
            Ongoing: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        multisig: {
            /**
             * Call is already approved by this signatory.
             **/
            AlreadyApproved: AugmentedError<ApiType>;
            /**
             * The data to be stored is already stored.
             **/
            AlreadyStored: AugmentedError<ApiType>;
            /**
             * The maximum weight information provided was too low.
             **/
            MaxWeightTooLow: AugmentedError<ApiType>;
            /**
             * Threshold must be 2 or greater.
             **/
            MinimumThreshold: AugmentedError<ApiType>;
            /**
             * Call doesn't need any (more) approvals.
             **/
            NoApprovalsNeeded: AugmentedError<ApiType>;
            /**
             * Multisig operation not found when attempting to cancel.
             **/
            NotFound: AugmentedError<ApiType>;
            /**
             * No timepoint was given, yet the multisig operation is already underway.
             **/
            NoTimepoint: AugmentedError<ApiType>;
            /**
             * Only the account that originally created the multisig is able to cancel it.
             **/
            NotOwner: AugmentedError<ApiType>;
            /**
             * The sender was contained in the other signatories; it shouldn't be.
             **/
            SenderInSignatories: AugmentedError<ApiType>;
            /**
             * The signatories were provided out of order; they should be ordered.
             **/
            SignatoriesOutOfOrder: AugmentedError<ApiType>;
            /**
             * There are too few signatories in the list.
             **/
            TooFewSignatories: AugmentedError<ApiType>;
            /**
             * There are too many signatories in the list.
             **/
            TooManySignatories: AugmentedError<ApiType>;
            /**
             * A timepoint was given, yet no multisig operation is underway.
             **/
            UnexpectedTimepoint: AugmentedError<ApiType>;
            /**
             * A different timepoint was given to the multisig operation that is underway.
             **/
            WrongTimepoint: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        onDemandAssignmentProvider: {
            /**
             * The order queue is full, `place_order` will not continue.
             **/
            QueueFull: AugmentedError<ApiType>;
            /**
             * The current spot price is higher than the max amount specified in the `place_order`
             * call, making it invalid.
             **/
            SpotPriceHigherThanMaxAmount: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        paraInclusion: {
            /**
             * The candidate's relay-parent was not allowed. Either it was
             * not recent enough or it didn't advance based on the last parachain block.
             **/
            DisallowedRelayParent: AugmentedError<ApiType>;
            /**
             * Head data exceeds the configured maximum.
             **/
            HeadDataTooLarge: AugmentedError<ApiType>;
            /**
             * The candidate didn't follow the rules of HRMP watermark advancement.
             **/
            HrmpWatermarkMishandling: AugmentedError<ApiType>;
            /**
             * The downward message queue is not processed correctly.
             **/
            IncorrectDownwardMessageHandling: AugmentedError<ApiType>;
            /**
             * Insufficient (non-majority) backing.
             **/
            InsufficientBacking: AugmentedError<ApiType>;
            /**
             * Failed to compute group index for the core: either it's out of bounds
             * or the relay parent doesn't belong to the current session.
             **/
            InvalidAssignment: AugmentedError<ApiType>;
            /**
             * Invalid (bad signature, unknown validator, etc.) backing.
             **/
            InvalidBacking: AugmentedError<ApiType>;
            /**
             * Invalid group index in core assignment.
             **/
            InvalidGroupIndex: AugmentedError<ApiType>;
            /**
             * The HRMP messages sent by the candidate is not valid.
             **/
            InvalidOutboundHrmp: AugmentedError<ApiType>;
            /**
             * At least one upward message sent does not pass the acceptance criteria.
             **/
            InvalidUpwardMessages: AugmentedError<ApiType>;
            /**
             * The validation code hash of the candidate is not valid.
             **/
            InvalidValidationCodeHash: AugmentedError<ApiType>;
            /**
             * Output code is too large
             **/
            NewCodeTooLarge: AugmentedError<ApiType>;
            /**
             * The `para_head` hash in the candidate descriptor doesn't match the hash of the actual
             * para head in the commitments.
             **/
            ParaHeadMismatch: AugmentedError<ApiType>;
            /**
             * Code upgrade prematurely.
             **/
            PrematureCodeUpgrade: AugmentedError<ApiType>;
            /**
             * Candidate submitted but para not scheduled.
             **/
            UnscheduledCandidate: AugmentedError<ApiType>;
            /**
             * The validation data hash does not match expected.
             **/
            ValidationDataHashMismatch: AugmentedError<ApiType>;
            /**
             * Validator index out of bounds.
             **/
            ValidatorIndexOutOfBounds: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        paraInherent: {
            /**
             * Inherent data was filtered during execution. This should have only been done
             * during creation.
             **/
            InherentDataFilteredDuringExecution: AugmentedError<ApiType>;
            /**
             * The hash of the submitted parent header doesn't correspond to the saved block hash of
             * the parent.
             **/
            InvalidParentHeader: AugmentedError<ApiType>;
            /**
             * Inclusion inherent called more than once per block.
             **/
            TooManyInclusionInherents: AugmentedError<ApiType>;
            /**
             * Too many candidates supplied.
             **/
            UnscheduledCandidate: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        paras: {
            /**
             * Para cannot be downgraded to an on-demand parachain.
             **/
            CannotDowngrade: AugmentedError<ApiType>;
            /**
             * Para cannot be offboarded at this time.
             **/
            CannotOffboard: AugmentedError<ApiType>;
            /**
             * Para cannot be onboarded because it is already tracked by our system.
             **/
            CannotOnboard: AugmentedError<ApiType>;
            /**
             * Para cannot be upgraded to a lease holding parachain.
             **/
            CannotUpgrade: AugmentedError<ApiType>;
            /**
             * Parachain cannot currently schedule a code upgrade.
             **/
            CannotUpgradeCode: AugmentedError<ApiType>;
            /**
             * Invalid validation code size.
             **/
            InvalidCode: AugmentedError<ApiType>;
            /**
             * Para is not registered in our system.
             **/
            NotRegistered: AugmentedError<ApiType>;
            /**
             * The given validator already has cast a vote.
             **/
            PvfCheckDoubleVote: AugmentedError<ApiType>;
            /**
             * The signature for the PVF pre-checking is invalid.
             **/
            PvfCheckInvalidSignature: AugmentedError<ApiType>;
            /**
             * The statement for PVF pre-checking is for a future session.
             **/
            PvfCheckStatementFuture: AugmentedError<ApiType>;
            /**
             * The statement for PVF pre-checking is stale.
             **/
            PvfCheckStatementStale: AugmentedError<ApiType>;
            /**
             * The given PVF does not exist at the moment of process a vote.
             **/
            PvfCheckSubjectInvalid: AugmentedError<ApiType>;
            /**
             * Claimed validator index is out of bounds.
             **/
            PvfCheckValidatorIndexOutOfBounds: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        parasDisputes: {
            /**
             * Ancient dispute statement provided.
             **/
            AncientDisputeStatement: AugmentedError<ApiType>;
            /**
             * Duplicate dispute statement sets provided.
             **/
            DuplicateDisputeStatementSets: AugmentedError<ApiType>;
            /**
             * Validator vote submitted more than once to dispute.
             **/
            DuplicateStatement: AugmentedError<ApiType>;
            /**
             * Invalid signature on statement.
             **/
            InvalidSignature: AugmentedError<ApiType>;
            /**
             * A dispute vote from a malicious backer.
             **/
            MaliciousBacker: AugmentedError<ApiType>;
            /**
             * No backing votes were provides along dispute statements.
             **/
            MissingBackingVotes: AugmentedError<ApiType>;
            /**
             * A dispute where there are only votes on one side.
             **/
            SingleSidedDispute: AugmentedError<ApiType>;
            /**
             * Unconfirmed dispute statement sets provided.
             **/
            UnconfirmedDispute: AugmentedError<ApiType>;
            /**
             * Validator index on statement is out of bounds for session.
             **/
            ValidatorIndexOutOfBounds: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        parasSlashing: {
            /**
             * The given slashing report is valid but already previously reported.
             **/
            DuplicateSlashingReport: AugmentedError<ApiType>;
            /**
             * The candidate hash is invalid.
             **/
            InvalidCandidateHash: AugmentedError<ApiType>;
            /**
             * The key ownership proof is invalid.
             **/
            InvalidKeyOwnershipProof: AugmentedError<ApiType>;
            /**
             * The session index is too old or invalid.
             **/
            InvalidSessionIndex: AugmentedError<ApiType>;
            /**
             * There is no pending slash for the given validator index and time
             * slot.
             **/
            InvalidValidatorIndex: AugmentedError<ApiType>;
            /**
             * The validator index does not match the validator id.
             **/
            ValidatorIndexIdMismatch: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        parasSudoWrapper: {
            /**
             * Cannot downgrade lease holding parachain to on-demand.
             **/
            CannotDowngrade: AugmentedError<ApiType>;
            /**
             * Cannot upgrade on-demand parachain to lease holding parachain.
             **/
            CannotUpgrade: AugmentedError<ApiType>;
            /**
             * Could not schedule para cleanup.
             **/
            CouldntCleanup: AugmentedError<ApiType>;
            /**
             * A DMP message couldn't be sent because it exceeds the maximum size allowed for a
             * downward message.
             **/
            ExceedsMaxMessageSize: AugmentedError<ApiType>;
            /**
             * Not a lease holding parachain.
             **/
            NotParachain: AugmentedError<ApiType>;
            /**
             * Not a parathread (on-demand parachain).
             **/
            NotParathread: AugmentedError<ApiType>;
            /**
             * The specified parachain is already registered.
             **/
            ParaAlreadyExists: AugmentedError<ApiType>;
            /**
             * The specified parachain is not registered.
             **/
            ParaDoesntExist: AugmentedError<ApiType>;
            /**
             * There are more cores than supported by the runtime.
             **/
            TooManyCores: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        pooledStaking: {
            CandidateTransferingOwnSharesForbidden: AugmentedError<ApiType>;
            DisabledFeature: AugmentedError<ApiType>;
            InconsistentState: AugmentedError<ApiType>;
            InvalidPalletSetting: AugmentedError<ApiType>;
            MathOverflow: AugmentedError<ApiType>;
            MathUnderflow: AugmentedError<ApiType>;
            NoOneIsStaking: AugmentedError<ApiType>;
            NotEnoughShares: AugmentedError<ApiType>;
            PoolsExtrinsicsArePaused: AugmentedError<ApiType>;
            RequestCannotBeExecuted: AugmentedError<ApiType>;
            RewardsMustBeNonZero: AugmentedError<ApiType>;
            StakeMustBeNonZero: AugmentedError<ApiType>;
            SwapResultsInZeroShares: AugmentedError<ApiType>;
            TryingToLeaveTooSoon: AugmentedError<ApiType>;
            UnsufficientSharesForTransfer: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        preimage: {
            /**
             * Preimage has already been noted on-chain.
             **/
            AlreadyNoted: AugmentedError<ApiType>;
            /**
             * The user is not authorized to perform this action.
             **/
            NotAuthorized: AugmentedError<ApiType>;
            /**
             * The preimage cannot be removed since it has not yet been noted.
             **/
            NotNoted: AugmentedError<ApiType>;
            /**
             * The preimage request cannot be removed since no outstanding requests exist.
             **/
            NotRequested: AugmentedError<ApiType>;
            /**
             * A preimage may not be removed when there are outstanding requests.
             **/
            Requested: AugmentedError<ApiType>;
            /**
             * Preimage is too large to store on-chain.
             **/
            TooBig: AugmentedError<ApiType>;
            /**
             * Too few hashes were requested to be upgraded (i.e. zero).
             **/
            TooFew: AugmentedError<ApiType>;
            /**
             * More than `MAX_HASH_UPGRADE_BULK_COUNT` hashes were requested to be upgraded at once.
             **/
            TooMany: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        proxy: {
            /**
             * Account is already a proxy.
             **/
            Duplicate: AugmentedError<ApiType>;
            /**
             * Call may not be made by proxy because it may escalate its privileges.
             **/
            NoPermission: AugmentedError<ApiType>;
            /**
             * Cannot add self as proxy.
             **/
            NoSelfProxy: AugmentedError<ApiType>;
            /**
             * Proxy registration not found.
             **/
            NotFound: AugmentedError<ApiType>;
            /**
             * Sender is not a proxy of the account to be proxied.
             **/
            NotProxy: AugmentedError<ApiType>;
            /**
             * There are too many proxies registered or too many announcements pending.
             **/
            TooMany: AugmentedError<ApiType>;
            /**
             * Announcement, if made at all, was made too recently.
             **/
            Unannounced: AugmentedError<ApiType>;
            /**
             * A call which is incompatible with the proxy type's filter was attempted.
             **/
            Unproxyable: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        referenda: {
            /**
             * The referendum index provided is invalid in this context.
             **/
            BadReferendum: AugmentedError<ApiType>;
            /**
             * The referendum status is invalid for this operation.
             **/
            BadStatus: AugmentedError<ApiType>;
            /**
             * The track identifier given was invalid.
             **/
            BadTrack: AugmentedError<ApiType>;
            /**
             * There are already a full complement of referenda in progress for this track.
             **/
            Full: AugmentedError<ApiType>;
            /**
             * Referendum's decision deposit is already paid.
             **/
            HasDeposit: AugmentedError<ApiType>;
            /**
             * The deposit cannot be refunded since none was made.
             **/
            NoDeposit: AugmentedError<ApiType>;
            /**
             * The deposit refunder is not the depositor.
             **/
            NoPermission: AugmentedError<ApiType>;
            /**
             * There was nothing to do in the advancement.
             **/
            NothingToDo: AugmentedError<ApiType>;
            /**
             * Referendum is not ongoing.
             **/
            NotOngoing: AugmentedError<ApiType>;
            /**
             * No track exists for the proposal origin.
             **/
            NoTrack: AugmentedError<ApiType>;
            /**
             * The preimage does not exist.
             **/
            PreimageNotExist: AugmentedError<ApiType>;
            /**
             * The preimage is stored with a different length than the one provided.
             **/
            PreimageStoredWithDifferentLength: AugmentedError<ApiType>;
            /**
             * The queue of the track is empty.
             **/
            QueueEmpty: AugmentedError<ApiType>;
            /**
             * Any deposit cannot be refunded until after the decision is over.
             **/
            Unfinished: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        registrar: {
            /**
             * The ID is already registered.
             **/
            AlreadyRegistered: AugmentedError<ApiType>;
            /**
             * Cannot deregister para
             **/
            CannotDeregister: AugmentedError<ApiType>;
            /**
             * Cannot schedule downgrade of lease holding parachain to on-demand parachain
             **/
            CannotDowngrade: AugmentedError<ApiType>;
            /**
             * Cannot perform a parachain slot / lifecycle swap. Check that the state of both paras
             * are correct for the swap to work.
             **/
            CannotSwap: AugmentedError<ApiType>;
            /**
             * Cannot schedule upgrade of on-demand parachain to lease holding parachain
             **/
            CannotUpgrade: AugmentedError<ApiType>;
            /**
             * Invalid para code size.
             **/
            CodeTooLarge: AugmentedError<ApiType>;
            /**
             * Invalid para head data size.
             **/
            HeadDataTooLarge: AugmentedError<ApiType>;
            /**
             * The validation code is invalid.
             **/
            InvalidCode: AugmentedError<ApiType>;
            /**
             * The caller is not the owner of this Id.
             **/
            NotOwner: AugmentedError<ApiType>;
            /**
             * Para is not a Parachain.
             **/
            NotParachain: AugmentedError<ApiType>;
            /**
             * Para is not a Parathread (on-demand parachain).
             **/
            NotParathread: AugmentedError<ApiType>;
            /**
             * The ID is not registered.
             **/
            NotRegistered: AugmentedError<ApiType>;
            /**
             * The ID given for registration has not been reserved.
             **/
            NotReserved: AugmentedError<ApiType>;
            /**
             * Para is locked from manipulation by the manager. Must use parachain or relay chain
             * governance.
             **/
            ParaLocked: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        scheduler: {
            /**
             * Failed to schedule a call
             **/
            FailedToSchedule: AugmentedError<ApiType>;
            /**
             * Attempt to use a non-named function on a named task.
             **/
            Named: AugmentedError<ApiType>;
            /**
             * Cannot find the scheduled call.
             **/
            NotFound: AugmentedError<ApiType>;
            /**
             * Reschedule failed because it does not change scheduled time.
             **/
            RescheduleNoChange: AugmentedError<ApiType>;
            /**
             * Given target block number is in the past.
             **/
            TargetBlockNumberInPast: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        servicesPayment: {
            CreditPriceTooExpensive: AugmentedError<ApiType>;
            InsufficientCredits: AugmentedError<ApiType>;
            InsufficientFundsToPurchaseCredits: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        session: {
            /**
             * Registered duplicate key.
             **/
            DuplicatedKey: AugmentedError<ApiType>;
            /**
             * Invalid ownership proof.
             **/
            InvalidProof: AugmentedError<ApiType>;
            /**
             * Key setting account is not live, so it's impossible to associate keys.
             **/
            NoAccount: AugmentedError<ApiType>;
            /**
             * No associated validator ID for account.
             **/
            NoAssociatedValidatorId: AugmentedError<ApiType>;
            /**
             * No keys are associated with this account.
             **/
            NoKeys: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        streamPayment: {
            CanOnlyCancelOwnRequest: AugmentedError<ApiType>;
            CantAcceptOwnRequest: AugmentedError<ApiType>;
            CantBeBothSourceAndTarget: AugmentedError<ApiType>;
            CantCreateStreamWithDepositUnderSoftMinimum: AugmentedError<ApiType>;
            CantDecreaseDepositUnderSoftDepositMinimum: AugmentedError<ApiType>;
            CantFetchCurrentTime: AugmentedError<ApiType>;
            CantFetchStatusBeforeLastTimeUpdated: AugmentedError<ApiType>;
            CantOverrideMandatoryChange: AugmentedError<ApiType>;
            ChangingAssetRequiresAbsoluteDepositChange: AugmentedError<ApiType>;
            DeadlineCantBeInPast: AugmentedError<ApiType>;
            DeadlineDelayIsBelowMinium: AugmentedError<ApiType>;
            ImmediateDepositChangeRequiresSameAssetId: AugmentedError<ApiType>;
            NoPendingRequest: AugmentedError<ApiType>;
            SourceCantCloseActiveStreamWithSoftDepositMinimum: AugmentedError<ApiType>;
            SourceCantDecreaseRate: AugmentedError<ApiType>;
            StreamIdOverflow: AugmentedError<ApiType>;
            TargetCantChangeDeposit: AugmentedError<ApiType>;
            TargetCantIncreaseRate: AugmentedError<ApiType>;
            UnauthorizedOrigin: AugmentedError<ApiType>;
            UnknownStreamId: AugmentedError<ApiType>;
            WrongRequestNonce: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        sudo: {
            /**
             * Sender must be the Sudo account.
             **/
            RequireSudo: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        system: {
            /**
             * The origin filter prevent the call to be dispatched.
             **/
            CallFiltered: AugmentedError<ApiType>;
            /**
             * Failed to extract the runtime version from the new runtime.
             *
             * Either calling `Core_version` or decoding `RuntimeVersion` failed.
             **/
            FailedToExtractRuntimeVersion: AugmentedError<ApiType>;
            /**
             * The name of specification does not match between the current runtime
             * and the new runtime.
             **/
            InvalidSpecName: AugmentedError<ApiType>;
            /**
             * A multi-block migration is ongoing and prevents the current code from being replaced.
             **/
            MultiBlockMigrationsOngoing: AugmentedError<ApiType>;
            /**
             * Suicide called when the account has non-default composite data.
             **/
            NonDefaultComposite: AugmentedError<ApiType>;
            /**
             * There is a non-zero reference count preventing the account from being purged.
             **/
            NonZeroRefCount: AugmentedError<ApiType>;
            /**
             * No upgrade authorized.
             **/
            NothingAuthorized: AugmentedError<ApiType>;
            /**
             * The specification version is not allowed to decrease between the current runtime
             * and the new runtime.
             **/
            SpecVersionNeedsToIncrease: AugmentedError<ApiType>;
            /**
             * The submitted code is not authorized.
             **/
            Unauthorized: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        tanssiInvulnerables: {
            /**
             * Account is already an Invulnerable.
             **/
            AlreadyInvulnerable: AugmentedError<ApiType>;
            /**
             * Account does not have keys registered
             **/
            NoKeysRegistered: AugmentedError<ApiType>;
            /**
             * Account is not an Invulnerable.
             **/
            NotInvulnerable: AugmentedError<ApiType>;
            /**
             * There are too many Invulnerables.
             **/
            TooManyInvulnerables: AugmentedError<ApiType>;
            /**
             * Unable to derive collator id from account id
             **/
            UnableToDeriveCollatorId: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        treasury: {
            /**
             * The payment has already been attempted.
             **/
            AlreadyAttempted: AugmentedError<ApiType>;
            /**
             * The spend is not yet eligible for payout.
             **/
            EarlyPayout: AugmentedError<ApiType>;
            /**
             * The balance of the asset kind is not convertible to the balance of the native asset.
             **/
            FailedToConvertBalance: AugmentedError<ApiType>;
            /**
             * The payment has neither failed nor succeeded yet.
             **/
            Inconclusive: AugmentedError<ApiType>;
            /**
             * The spend origin is valid but the amount it is allowed to spend is lower than the
             * amount to be spent.
             **/
            InsufficientPermission: AugmentedError<ApiType>;
            /**
             * No proposal, bounty or spend at that index.
             **/
            InvalidIndex: AugmentedError<ApiType>;
            /**
             * The payout was not yet attempted/claimed.
             **/
            NotAttempted: AugmentedError<ApiType>;
            /**
             * There was some issue with the mechanism of payment.
             **/
            PayoutError: AugmentedError<ApiType>;
            /**
             * Proposal has not been approved.
             **/
            ProposalNotApproved: AugmentedError<ApiType>;
            /**
             * The spend has expired and cannot be claimed.
             **/
            SpendExpired: AugmentedError<ApiType>;
            /**
             * Too many approvals in the queue.
             **/
            TooManyApprovals: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        utility: {
            /**
             * Too many calls batched.
             **/
            TooManyCalls: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        whitelist: {
            /**
             * The call was already whitelisted; No-Op.
             **/
            CallAlreadyWhitelisted: AugmentedError<ApiType>;
            /**
             * The call was not whitelisted.
             **/
            CallIsNotWhitelisted: AugmentedError<ApiType>;
            /**
             * The weight of the decoded call was higher than the witness.
             **/
            InvalidCallWeightWitness: AugmentedError<ApiType>;
            /**
             * The preimage of the call hash could not be loaded.
             **/
            UnavailablePreImage: AugmentedError<ApiType>;
            /**
             * The call could not be decoded.
             **/
            UndecodableCall: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        xcmPallet: {
            /**
             * The given account is not an identifiable sovereign account for any location.
             **/
            AccountNotSovereign: AugmentedError<ApiType>;
            /**
             * The location is invalid since it already has a subscription from us.
             **/
            AlreadySubscribed: AugmentedError<ApiType>;
            /**
             * The given location could not be used (e.g. because it cannot be expressed in the
             * desired version of XCM).
             **/
            BadLocation: AugmentedError<ApiType>;
            /**
             * The version of the `Versioned` value used is not able to be interpreted.
             **/
            BadVersion: AugmentedError<ApiType>;
            /**
             * Could not check-out the assets for teleportation to the destination chain.
             **/
            CannotCheckOutTeleport: AugmentedError<ApiType>;
            /**
             * Could not re-anchor the assets to declare the fees for the destination chain.
             **/
            CannotReanchor: AugmentedError<ApiType>;
            /**
             * The destination `Location` provided cannot be inverted.
             **/
            DestinationNotInvertible: AugmentedError<ApiType>;
            /**
             * The assets to be sent are empty.
             **/
            Empty: AugmentedError<ApiType>;
            /**
             * The operation required fees to be paid which the initiator could not meet.
             **/
            FeesNotMet: AugmentedError<ApiType>;
            /**
             * The message execution fails the filter.
             **/
            Filtered: AugmentedError<ApiType>;
            /**
             * The unlock operation cannot succeed because there are still consumers of the lock.
             **/
            InUse: AugmentedError<ApiType>;
            /**
             * Invalid asset, reserve chain could not be determined for it.
             **/
            InvalidAssetUnknownReserve: AugmentedError<ApiType>;
            /**
             * Invalid asset, do not support remote asset reserves with different fees reserves.
             **/
            InvalidAssetUnsupportedReserve: AugmentedError<ApiType>;
            /**
             * Origin is invalid for sending.
             **/
            InvalidOrigin: AugmentedError<ApiType>;
            /**
             * Local XCM execution incomplete.
             **/
            LocalExecutionIncomplete: AugmentedError<ApiType>;
            /**
             * A remote lock with the corresponding data could not be found.
             **/
            LockNotFound: AugmentedError<ApiType>;
            /**
             * The owner does not own (all) of the asset that they wish to do the operation on.
             **/
            LowBalance: AugmentedError<ApiType>;
            /**
             * The referenced subscription could not be found.
             **/
            NoSubscription: AugmentedError<ApiType>;
            /**
             * There was some other issue (i.e. not to do with routing) in sending the message.
             * Perhaps a lack of space for buffering the message.
             **/
            SendFailure: AugmentedError<ApiType>;
            /**
             * Too many assets have been attempted for transfer.
             **/
            TooManyAssets: AugmentedError<ApiType>;
            /**
             * The asset owner has too many locks on the asset.
             **/
            TooManyLocks: AugmentedError<ApiType>;
            /**
             * Too many assets with different reserve locations have been attempted for transfer.
             **/
            TooManyReserves: AugmentedError<ApiType>;
            /**
             * The desired destination was unreachable, generally because there is a no way of routing
             * to it.
             **/
            Unreachable: AugmentedError<ApiType>;
            /**
             * The message's weight could not be determined.
             **/
            UnweighableMessage: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
    } // AugmentedErrors
} // declare module
