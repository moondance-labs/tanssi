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
            /** The given asset ID already has an assigned conversion rate and cannot be re-created. */
            AlreadyExists: AugmentedError<ApiType>;
            /** Overflow ocurred when calculating the inverse rate. */
            Overflow: AugmentedError<ApiType>;
            /** The given asset ID is unknown. */
            UnknownAssetKind: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        authorInherent: {
            /** Author already set in block. */
            AuthorAlreadySet: AugmentedError<ApiType>;
            /** The author in the inherent is not an eligible author. */
            CannotBeAuthor: AugmentedError<ApiType>;
            /** No AccountId was found to be associated with this author */
            NoAccountId: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        authorNoting: {
            AsPreRuntimeError: AugmentedError<ApiType>;
            AuraDigestFirstItem: AugmentedError<ApiType>;
            AuthorNotFound: AugmentedError<ApiType>;
            FailedDecodingHeader: AugmentedError<ApiType>;
            /** The new value for a configuration parameter is invalid. */
            FailedReading: AugmentedError<ApiType>;
            NonAuraDigest: AugmentedError<ApiType>;
            NonDecodableSlot: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        balances: {
            /** Beneficiary account must pre-exist. */
            DeadAccount: AugmentedError<ApiType>;
            /** Value too low to create account due to existential deposit. */
            ExistentialDeposit: AugmentedError<ApiType>;
            /** A vesting schedule already exists for this account. */
            ExistingVestingSchedule: AugmentedError<ApiType>;
            /** Transfer/payment would kill account. */
            Expendability: AugmentedError<ApiType>;
            /** Balance too low to send value. */
            InsufficientBalance: AugmentedError<ApiType>;
            /** Account liquidity restrictions prevent withdrawal. */
            LiquidityRestrictions: AugmentedError<ApiType>;
            /** Number of freezes exceed `MaxFreezes`. */
            TooManyFreezes: AugmentedError<ApiType>;
            /** Number of holds exceed `MaxHolds`. */
            TooManyHolds: AugmentedError<ApiType>;
            /** Number of named reserves exceed `MaxReserves`. */
            TooManyReserves: AugmentedError<ApiType>;
            /** Vesting balance too high to send value. */
            VestingBalance: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        configuration: {
            /** The new value for a configuration parameter is invalid. */
            InvalidNewValue: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        dataPreservers: {
            /** This container chain does not have any boot nodes */
            NoBootNodes: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        foreignAssets: {
            /** The asset-account already exists. */
            AlreadyExists: AugmentedError<ApiType>;
            /** The asset is not live, and likely being destroyed. */
            AssetNotLive: AugmentedError<ApiType>;
            /** Invalid metadata given. */
            BadMetadata: AugmentedError<ApiType>;
            /** Invalid witness data given. */
            BadWitness: AugmentedError<ApiType>;
            /** Account balance must be greater than or equal to the transfer amount. */
            BalanceLow: AugmentedError<ApiType>;
            /** Callback action resulted in error */
            CallbackFailed: AugmentedError<ApiType>;
            /** The origin account is frozen. */
            Frozen: AugmentedError<ApiType>;
            /** The asset status is not the expected status. */
            IncorrectStatus: AugmentedError<ApiType>;
            /** The asset ID is already taken. */
            InUse: AugmentedError<ApiType>;
            /**
             * The asset is a live asset and is actively being used. Usually emit for operations such as `start_destroy` which
             * require the asset to be in a destroying state.
             */
            LiveAsset: AugmentedError<ApiType>;
            /** Minimum balance should be non-zero. */
            MinBalanceZero: AugmentedError<ApiType>;
            /** The account to alter does not exist. */
            NoAccount: AugmentedError<ApiType>;
            /** The asset-account doesn't have an associated deposit. */
            NoDeposit: AugmentedError<ApiType>;
            /** The signing account has no permission to do the operation. */
            NoPermission: AugmentedError<ApiType>;
            /** The asset should be frozen before the given operation. */
            NotFrozen: AugmentedError<ApiType>;
            /** No approval exists that would allow the transfer. */
            Unapproved: AugmentedError<ApiType>;
            /**
             * Unable to increment the consumer reference counters on the account. Either no provider reference exists to
             * allow a non-zero balance of a non-self-sufficient asset, or one fewer then the maximum number of consumers has
             * been reached.
             */
            UnavailableConsumer: AugmentedError<ApiType>;
            /** The given asset ID is unknown. */
            Unknown: AugmentedError<ApiType>;
            /** The operation would result in funds being burned. */
            WouldBurn: AugmentedError<ApiType>;
            /** The source account would not survive the transfer and it needs to stay alive. */
            WouldDie: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        foreignAssetsCreator: {
            AssetAlreadyExists: AugmentedError<ApiType>;
            AssetDoesNotExist: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        identity: {
            /** Account ID is already named. */
            AlreadyClaimed: AugmentedError<ApiType>;
            /** Empty index. */
            EmptyIndex: AugmentedError<ApiType>;
            /** Fee is changed. */
            FeeChanged: AugmentedError<ApiType>;
            /** The index is invalid. */
            InvalidIndex: AugmentedError<ApiType>;
            /** Invalid judgement. */
            InvalidJudgement: AugmentedError<ApiType>;
            /** The signature on a username was not valid. */
            InvalidSignature: AugmentedError<ApiType>;
            /** The provided suffix is too long. */
            InvalidSuffix: AugmentedError<ApiType>;
            /** The target is invalid. */
            InvalidTarget: AugmentedError<ApiType>;
            /** The username does not meet the requirements. */
            InvalidUsername: AugmentedError<ApiType>;
            /** The provided judgement was for a different identity. */
            JudgementForDifferentIdentity: AugmentedError<ApiType>;
            /** Judgement given. */
            JudgementGiven: AugmentedError<ApiType>;
            /** Error that occurs when there is an issue paying for judgement. */
            JudgementPaymentFailed: AugmentedError<ApiType>;
            /** The authority cannot allocate any more usernames. */
            NoAllocation: AugmentedError<ApiType>;
            /** No identity found. */
            NoIdentity: AugmentedError<ApiType>;
            /** The username cannot be forcefully removed because it can still be accepted. */
            NotExpired: AugmentedError<ApiType>;
            /** Account isn't found. */
            NotFound: AugmentedError<ApiType>;
            /** Account isn't named. */
            NotNamed: AugmentedError<ApiType>;
            /** Sub-account isn't owned by sender. */
            NotOwned: AugmentedError<ApiType>;
            /** Sender is not a sub-account. */
            NotSub: AugmentedError<ApiType>;
            /** The sender does not have permission to issue a username. */
            NotUsernameAuthority: AugmentedError<ApiType>;
            /** The requested username does not exist. */
            NoUsername: AugmentedError<ApiType>;
            /** Setting this username requires a signature, but none was provided. */
            RequiresSignature: AugmentedError<ApiType>;
            /** Sticky judgement. */
            StickyJudgement: AugmentedError<ApiType>;
            /** Maximum amount of registrars reached. Cannot add any more. */
            TooManyRegistrars: AugmentedError<ApiType>;
            /** Too many subs-accounts. */
            TooManySubAccounts: AugmentedError<ApiType>;
            /** The username is already taken. */
            UsernameTaken: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        invulnerables: {
            /** Account is already an Invulnerable. */
            AlreadyInvulnerable: AugmentedError<ApiType>;
            /** Account does not have keys registered */
            NoKeysRegistered: AugmentedError<ApiType>;
            /** Account is not an Invulnerable. */
            NotInvulnerable: AugmentedError<ApiType>;
            /** There are too many Invulnerables. */
            TooManyInvulnerables: AugmentedError<ApiType>;
            /** Unable to derive collator id from account id */
            UnableToDeriveCollatorId: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        maintenanceMode: {
            /** The chain cannot enter maintenance mode because it is already in maintenance mode */
            AlreadyInMaintenanceMode: AugmentedError<ApiType>;
            /** The chain cannot resume normal operation because it is not in maintenance mode */
            NotInMaintenanceMode: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        messageQueue: {
            /** The message was already processed and cannot be processed again. */
            AlreadyProcessed: AugmentedError<ApiType>;
            /** There is temporarily not enough weight to continue servicing messages. */
            InsufficientWeight: AugmentedError<ApiType>;
            /** The referenced message could not be found. */
            NoMessage: AugmentedError<ApiType>;
            /** Page to be reaped does not exist. */
            NoPage: AugmentedError<ApiType>;
            /** Page is not reapable because it has items remaining to be processed and is not old enough. */
            NotReapable: AugmentedError<ApiType>;
            /** The message is queued for future execution. */
            Queued: AugmentedError<ApiType>;
            /**
             * The queue is paused and no message can be executed from it.
             *
             * This can change at any time and may resolve in the future by re-trying.
             */
            QueuePaused: AugmentedError<ApiType>;
            /** Another call is in progress and needs to finish before this call can happen. */
            RecursiveDisallowed: AugmentedError<ApiType>;
            /**
             * This message is temporarily unprocessable.
             *
             * Such errors are expected, but not guaranteed, to resolve themselves eventually through retrying.
             */
            TemporarilyUnprocessable: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        migrations: {
            /** Preimage already exists in the new storage. */
            PreimageAlreadyExists: AugmentedError<ApiType>;
            /** Preimage is larger than the new max size. */
            PreimageIsTooBig: AugmentedError<ApiType>;
            /** Missing preimage in original democracy storage */
            PreimageMissing: AugmentedError<ApiType>;
            /** Provided upper bound is too low. */
            WrongUpperBound: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        multisig: {
            /** Call is already approved by this signatory. */
            AlreadyApproved: AugmentedError<ApiType>;
            /** The data to be stored is already stored. */
            AlreadyStored: AugmentedError<ApiType>;
            /** The maximum weight information provided was too low. */
            MaxWeightTooLow: AugmentedError<ApiType>;
            /** Threshold must be 2 or greater. */
            MinimumThreshold: AugmentedError<ApiType>;
            /** Call doesn't need any (more) approvals. */
            NoApprovalsNeeded: AugmentedError<ApiType>;
            /** Multisig operation not found when attempting to cancel. */
            NotFound: AugmentedError<ApiType>;
            /** No timepoint was given, yet the multisig operation is already underway. */
            NoTimepoint: AugmentedError<ApiType>;
            /** Only the account that originally created the multisig is able to cancel it. */
            NotOwner: AugmentedError<ApiType>;
            /** The sender was contained in the other signatories; it shouldn't be. */
            SenderInSignatories: AugmentedError<ApiType>;
            /** The signatories were provided out of order; they should be ordered. */
            SignatoriesOutOfOrder: AugmentedError<ApiType>;
            /** There are too few signatories in the list. */
            TooFewSignatories: AugmentedError<ApiType>;
            /** There are too many signatories in the list. */
            TooManySignatories: AugmentedError<ApiType>;
            /** A timepoint was given, yet no multisig operation is underway. */
            UnexpectedTimepoint: AugmentedError<ApiType>;
            /** A different timepoint was given to the multisig operation that is underway. */
            WrongTimepoint: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        parachainSystem: {
            /** The inherent which supplies the host configuration did not run this block. */
            HostConfigurationNotAvailable: AugmentedError<ApiType>;
            /** No code upgrade has been authorized. */
            NothingAuthorized: AugmentedError<ApiType>;
            /** No validation function upgrade is currently scheduled. */
            NotScheduled: AugmentedError<ApiType>;
            /** Attempt to upgrade validation function while existing upgrade pending. */
            OverlappingUpgrades: AugmentedError<ApiType>;
            /** Polkadot currently prohibits this parachain from upgrading its validation function. */
            ProhibitedByPolkadot: AugmentedError<ApiType>;
            /** The supplied validation function has compiled into a blob larger than Polkadot is willing to run. */
            TooBig: AugmentedError<ApiType>;
            /** The given code upgrade has not been authorized. */
            Unauthorized: AugmentedError<ApiType>;
            /** The inherent which supplies the validation data did not run this block. */
            ValidationDataNotAvailable: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        polkadotXcm: {
            /** The given account is not an identifiable sovereign account for any location. */
            AccountNotSovereign: AugmentedError<ApiType>;
            /** The location is invalid since it already has a subscription from us. */
            AlreadySubscribed: AugmentedError<ApiType>;
            /** The given location could not be used (e.g. because it cannot be expressed in the desired version of XCM). */
            BadLocation: AugmentedError<ApiType>;
            /** The version of the `Versioned` value used is not able to be interpreted. */
            BadVersion: AugmentedError<ApiType>;
            /** Could not check-out the assets for teleportation to the destination chain. */
            CannotCheckOutTeleport: AugmentedError<ApiType>;
            /** Could not re-anchor the assets to declare the fees for the destination chain. */
            CannotReanchor: AugmentedError<ApiType>;
            /** The destination `MultiLocation` provided cannot be inverted. */
            DestinationNotInvertible: AugmentedError<ApiType>;
            /** The assets to be sent are empty. */
            Empty: AugmentedError<ApiType>;
            /** The operation required fees to be paid which the initiator could not meet. */
            FeesNotMet: AugmentedError<ApiType>;
            /** The message execution fails the filter. */
            Filtered: AugmentedError<ApiType>;
            /** The unlock operation cannot succeed because there are still consumers of the lock. */
            InUse: AugmentedError<ApiType>;
            /** Invalid non-concrete asset. */
            InvalidAssetNotConcrete: AugmentedError<ApiType>;
            /** Invalid asset, reserve chain could not be determined for it. */
            InvalidAssetUnknownReserve: AugmentedError<ApiType>;
            /** Invalid asset, do not support remote asset reserves with different fees reserves. */
            InvalidAssetUnsupportedReserve: AugmentedError<ApiType>;
            /** Origin is invalid for sending. */
            InvalidOrigin: AugmentedError<ApiType>;
            /** Local XCM execution incomplete. */
            LocalExecutionIncomplete: AugmentedError<ApiType>;
            /** A remote lock with the corresponding data could not be found. */
            LockNotFound: AugmentedError<ApiType>;
            /** The owner does not own (all) of the asset that they wish to do the operation on. */
            LowBalance: AugmentedError<ApiType>;
            /** The referenced subscription could not be found. */
            NoSubscription: AugmentedError<ApiType>;
            /**
             * There was some other issue (i.e. not to do with routing) in sending the message. Perhaps a lack of space for
             * buffering the message.
             */
            SendFailure: AugmentedError<ApiType>;
            /** Too many assets have been attempted for transfer. */
            TooManyAssets: AugmentedError<ApiType>;
            /** The asset owner has too many locks on the asset. */
            TooManyLocks: AugmentedError<ApiType>;
            /** Too many assets with different reserve locations have been attempted for transfer. */
            TooManyReserves: AugmentedError<ApiType>;
            /** The desired destination was unreachable, generally because there is a no way of routing to it. */
            Unreachable: AugmentedError<ApiType>;
            /** The message's weight could not be determined. */
            UnweighableMessage: AugmentedError<ApiType>;
            /** Generic error */
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
            RequestCannotBeExecuted: AugmentedError<ApiType>;
            RewardsMustBeNonZero: AugmentedError<ApiType>;
            StakeMustBeNonZero: AugmentedError<ApiType>;
            SwapResultsInZeroShares: AugmentedError<ApiType>;
            TryingToLeaveTooSoon: AugmentedError<ApiType>;
            UnsufficientSharesForTransfer: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        proxy: {
            /** Account is already a proxy. */
            Duplicate: AugmentedError<ApiType>;
            /** Call may not be made by proxy because it may escalate its privileges. */
            NoPermission: AugmentedError<ApiType>;
            /** Cannot add self as proxy. */
            NoSelfProxy: AugmentedError<ApiType>;
            /** Proxy registration not found. */
            NotFound: AugmentedError<ApiType>;
            /** Sender is not a proxy of the account to be proxied. */
            NotProxy: AugmentedError<ApiType>;
            /** There are too many proxies registered or too many announcements pending. */
            TooMany: AugmentedError<ApiType>;
            /** Announcement, if made at all, was made too recently. */
            Unannounced: AugmentedError<ApiType>;
            /** A call which is incompatible with the proxy type's filter was attempted. */
            Unproxyable: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        registrar: {
            /** Attempted to register a ParaId with a genesis data size greater than the limit */
            GenesisDataTooBig: AugmentedError<ApiType>;
            /** Tried to change parathread params for a para id that is not a registered parathread */
            NotAParathread: AugmentedError<ApiType>;
            /** Tried to register a ParaId with an account that did not have enough balance for the deposit */
            NotSufficientDeposit: AugmentedError<ApiType>;
            /** Attempted to deregister a ParaId that is already being deregistered */
            ParaIdAlreadyDeregistered: AugmentedError<ApiType>;
            /** Attempted to pause a ParaId that was already paused */
            ParaIdAlreadyPaused: AugmentedError<ApiType>;
            /** Attempted to register a ParaId that was already registered */
            ParaIdAlreadyRegistered: AugmentedError<ApiType>;
            /** The bounded list of ParaIds has reached its limit */
            ParaIdListFull: AugmentedError<ApiType>;
            /** Tried to mark_valid_for_collating a ParaId that is not in PendingVerification */
            ParaIdNotInPendingVerification: AugmentedError<ApiType>;
            /** Attempted to unpause a ParaId that was not paused */
            ParaIdNotPaused: AugmentedError<ApiType>;
            /** Attempted to deregister a ParaId that is not registered */
            ParaIdNotRegistered: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        servicesPayment: {
            CreditPriceTooExpensive: AugmentedError<ApiType>;
            InsufficientCredits: AugmentedError<ApiType>;
            InsufficientFundsToPurchaseCredits: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        session: {
            /** Registered duplicate key. */
            DuplicatedKey: AugmentedError<ApiType>;
            /** Invalid ownership proof. */
            InvalidProof: AugmentedError<ApiType>;
            /** Key setting account is not live, so it's impossible to associate keys. */
            NoAccount: AugmentedError<ApiType>;
            /** No associated validator ID for account. */
            NoAssociatedValidatorId: AugmentedError<ApiType>;
            /** No keys are associated with this account. */
            NoKeys: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        streamPayment: {
            CanOnlyCancelOwnRequest: AugmentedError<ApiType>;
            CantAcceptOwnRequest: AugmentedError<ApiType>;
            CantBeBothSourceAndTarget: AugmentedError<ApiType>;
            CantFetchCurrentTime: AugmentedError<ApiType>;
            CantOverrideMandatoryChange: AugmentedError<ApiType>;
            ChangingAssetRequiresAbsoluteDepositChange: AugmentedError<ApiType>;
            ImmediateDepositChangeRequiresSameAssetId: AugmentedError<ApiType>;
            NoPendingRequest: AugmentedError<ApiType>;
            SourceCantDecreaseRate: AugmentedError<ApiType>;
            StreamIdOverflow: AugmentedError<ApiType>;
            TargetCantChangeDeposit: AugmentedError<ApiType>;
            TargetCantIncreaseRate: AugmentedError<ApiType>;
            UnauthorizedOrigin: AugmentedError<ApiType>;
            UnknownStreamId: AugmentedError<ApiType>;
            WrongRequestNonce: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        sudo: {
            /** Sender must be the Sudo account. */
            RequireSudo: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        system: {
            /** The origin filter prevent the call to be dispatched. */
            CallFiltered: AugmentedError<ApiType>;
            /**
             * Failed to extract the runtime version from the new runtime.
             *
             * Either calling `Core_version` or decoding `RuntimeVersion` failed.
             */
            FailedToExtractRuntimeVersion: AugmentedError<ApiType>;
            /** The name of specification does not match between the current runtime and the new runtime. */
            InvalidSpecName: AugmentedError<ApiType>;
            /** Suicide called when the account has non-default composite data. */
            NonDefaultComposite: AugmentedError<ApiType>;
            /** There is a non-zero reference count preventing the account from being purged. */
            NonZeroRefCount: AugmentedError<ApiType>;
            /** No upgrade authorized. */
            NothingAuthorized: AugmentedError<ApiType>;
            /** The specification version is not allowed to decrease between the current runtime and the new runtime. */
            SpecVersionNeedsToIncrease: AugmentedError<ApiType>;
            /** The submitted code is not authorized. */
            Unauthorized: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        treasury: {
            /** The payment has already been attempted. */
            AlreadyAttempted: AugmentedError<ApiType>;
            /** The spend is not yet eligible for payout. */
            EarlyPayout: AugmentedError<ApiType>;
            /** The balance of the asset kind is not convertible to the balance of the native asset. */
            FailedToConvertBalance: AugmentedError<ApiType>;
            /** The payment has neither failed nor succeeded yet. */
            Inconclusive: AugmentedError<ApiType>;
            /** The spend origin is valid but the amount it is allowed to spend is lower than the amount to be spent. */
            InsufficientPermission: AugmentedError<ApiType>;
            /** Proposer's balance is too low. */
            InsufficientProposersBalance: AugmentedError<ApiType>;
            /** No proposal, bounty or spend at that index. */
            InvalidIndex: AugmentedError<ApiType>;
            /** The payout was not yet attempted/claimed. */
            NotAttempted: AugmentedError<ApiType>;
            /** There was some issue with the mechanism of payment. */
            PayoutError: AugmentedError<ApiType>;
            /** Proposal has not been approved. */
            ProposalNotApproved: AugmentedError<ApiType>;
            /** The spend has expired and cannot be claimed. */
            SpendExpired: AugmentedError<ApiType>;
            /** Too many approvals in the queue. */
            TooManyApprovals: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        txPause: {
            /** The call is paused. */
            IsPaused: AugmentedError<ApiType>;
            /** The call is unpaused. */
            IsUnpaused: AugmentedError<ApiType>;
            NotFound: AugmentedError<ApiType>;
            /** The call is whitelisted and cannot be paused. */
            Unpausable: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        utility: {
            /** Too many calls batched. */
            TooManyCalls: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        xcmCoreBuyer: {
            /** Block production is pending for para id with successfully placed order */
            BlockProductionPending: AugmentedError<ApiType>;
            /** This collator is not assigned to this parathread */
            CollatorNotAssigned: AugmentedError<ApiType>;
            ErrorDeliveringXCM: AugmentedError<ApiType>;
            ErrorValidatingXCM: AugmentedError<ApiType>;
            /** There are too many in-flight orders, buying cores will not work until some of those orders finish. */
            InFlightLimitReached: AugmentedError<ApiType>;
            InvalidProof: AugmentedError<ApiType>;
            /** Inverting location from destination point of view failed */
            LocationInversionFailed: AugmentedError<ApiType>;
            /** There are no collators assigned to this parathread, so no point in buying a core */
            NoAssignedCollators: AugmentedError<ApiType>;
            /** The para id is not a parathread */
            NotAParathread: AugmentedError<ApiType>;
            /** An order for this para id already exists */
            OrderAlreadyExists: AugmentedError<ApiType>;
            /** Converting a multilocation into a relay relative multilocation failed */
            ReanchorFailed: AugmentedError<ApiType>;
            /** Modifying XCM to report the result of XCM failed */
            ReportNotifyingSetupFailed: AugmentedError<ApiType>;
            /** Unexpected XCM response */
            UnexpectedXCMResponse: AugmentedError<ApiType>;
            /**
             * The `XcmWeights` storage has not been set. This must have been set by root with the value of the relay chain
             * xcm call weight and extrinsic weight
             */
            XcmWeightStorageNotSet: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        xcmpQueue: {
            /** The execution is already resumed. */
            AlreadyResumed: AugmentedError<ApiType>;
            /** The execution is already suspended. */
            AlreadySuspended: AugmentedError<ApiType>;
            /** Setting the queue config failed since one of its values was invalid. */
            BadQueueConfig: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
    } // AugmentedErrors
} // declare module
