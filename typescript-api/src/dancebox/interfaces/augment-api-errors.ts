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
        cumulusXcm: {
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        dataPreservers: {
            /** This container chain does not have any boot nodes */
            NoBootNodes: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        dmpQueue: {
            /** The amount of weight given is possibly not enough for executing the message. */
            OverLimit: AugmentedError<ApiType>;
            /** The message index given is unknown. */
            Unknown: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
        invulnerables: {
            /** Account is already an Invulnerable. */
            AlreadyInvulnerable: AugmentedError<ApiType>;
            /** Account is not an Invulnerable. */
            NotInvulnerable: AugmentedError<ApiType>;
            /** There are too many Invulnerables. */
            TooManyInvulnerables: AugmentedError<ApiType>;
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
            /** Invalid asset for the operation. */
            InvalidAsset: AugmentedError<ApiType>;
            /** Origin is invalid for sending. */
            InvalidOrigin: AugmentedError<ApiType>;
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
        sudo: {
            /** Sender must be the Sudo account */
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
            /** The specification version is not allowed to decrease between the current runtime and the new runtime. */
            SpecVersionNeedsToIncrease: AugmentedError<ApiType>;
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
        xcmpQueue: {
            /** Bad overweight index. */
            BadOverweightIndex: AugmentedError<ApiType>;
            /** Bad XCM data. */
            BadXcm: AugmentedError<ApiType>;
            /** Bad XCM origin. */
            BadXcmOrigin: AugmentedError<ApiType>;
            /** Failed to send XCM message. */
            FailedToSend: AugmentedError<ApiType>;
            /** Provided weight is possibly not enough to execute the message. */
            WeightOverLimit: AugmentedError<ApiType>;
            /** Generic error */
            [key: string]: AugmentedError<ApiType>;
        };
    } // AugmentedErrors
} // declare module
