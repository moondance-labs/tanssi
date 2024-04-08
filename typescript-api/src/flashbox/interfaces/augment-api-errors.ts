// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/errors";

import type { ApiTypes, AugmentedError } from "@polkadot/api-base/types";

export type __AugmentedError<ApiType extends ApiTypes> = AugmentedError<ApiType>;

declare module "@polkadot/api-base/types/errors" {
    interface AugmentedErrors<ApiType extends ApiTypes> {
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
    } // AugmentedErrors
} // declare module
