// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

/* eslint-disable sort-keys */

export default {
    /** Lookup3: frame_system::AccountInfo<Nonce, pallet_balances::types::AccountData<Balance>> */
    FrameSystemAccountInfo: {
        nonce: "u32",
        consumers: "u32",
        providers: "u32",
        sufficients: "u32",
        data: "PalletBalancesAccountData",
    },
    /** Lookup5: pallet_balances::types::AccountData<Balance> */
    PalletBalancesAccountData: {
        free: "u128",
        reserved: "u128",
        frozen: "u128",
        flags: "u128",
    },
    /** Lookup8: frame_support::dispatch::PerDispatchClass<sp_weights::weight_v2::Weight> */
    FrameSupportDispatchPerDispatchClassWeight: {
        normal: "SpWeightsWeightV2Weight",
        operational: "SpWeightsWeightV2Weight",
        mandatory: "SpWeightsWeightV2Weight",
    },
    /** Lookup9: sp_weights::weight_v2::Weight */
    SpWeightsWeightV2Weight: {
        refTime: "Compact<u64>",
        proofSize: "Compact<u64>",
    },
    /** Lookup14: sp_runtime::generic::digest::Digest */
    SpRuntimeDigest: {
        logs: "Vec<SpRuntimeDigestDigestItem>",
    },
    /** Lookup16: sp_runtime::generic::digest::DigestItem */
    SpRuntimeDigestDigestItem: {
        _enum: {
            Other: "Bytes",
            __Unused1: "Null",
            __Unused2: "Null",
            __Unused3: "Null",
            Consensus: "([u8;4],Bytes)",
            Seal: "([u8;4],Bytes)",
            PreRuntime: "([u8;4],Bytes)",
            __Unused7: "Null",
            RuntimeEnvironmentUpdated: "Null",
        },
    },
    /** Lookup19: frame_system::EventRecord<dancebox_runtime::RuntimeEvent, primitive_types::H256> */
    FrameSystemEventRecord: {
        phase: "FrameSystemPhase",
        event: "Event",
        topics: "Vec<H256>",
    },
    /** Lookup21: frame_system::pallet::Event<T> */
    FrameSystemEvent: {
        _enum: {
            ExtrinsicSuccess: {
                dispatchInfo: "FrameSupportDispatchDispatchInfo",
            },
            ExtrinsicFailed: {
                dispatchError: "SpRuntimeDispatchError",
                dispatchInfo: "FrameSupportDispatchDispatchInfo",
            },
            CodeUpdated: "Null",
            NewAccount: {
                account: "AccountId32",
            },
            KilledAccount: {
                account: "AccountId32",
            },
            Remarked: {
                _alias: {
                    hash_: "hash",
                },
                sender: "AccountId32",
                hash_: "H256",
            },
        },
    },
    /** Lookup22: frame_support::dispatch::DispatchInfo */
    FrameSupportDispatchDispatchInfo: {
        weight: "SpWeightsWeightV2Weight",
        class: "FrameSupportDispatchDispatchClass",
        paysFee: "FrameSupportDispatchPays",
    },
    /** Lookup23: frame_support::dispatch::DispatchClass */
    FrameSupportDispatchDispatchClass: {
        _enum: ["Normal", "Operational", "Mandatory"],
    },
    /** Lookup24: frame_support::dispatch::Pays */
    FrameSupportDispatchPays: {
        _enum: ["Yes", "No"],
    },
    /** Lookup25: sp_runtime::DispatchError */
    SpRuntimeDispatchError: {
        _enum: {
            Other: "Null",
            CannotLookup: "Null",
            BadOrigin: "Null",
            Module: "SpRuntimeModuleError",
            ConsumerRemaining: "Null",
            NoProviders: "Null",
            TooManyConsumers: "Null",
            Token: "SpRuntimeTokenError",
            Arithmetic: "SpArithmeticArithmeticError",
            Transactional: "SpRuntimeTransactionalError",
            Exhausted: "Null",
            Corruption: "Null",
            Unavailable: "Null",
            RootNotAllowed: "Null",
        },
    },
    /** Lookup26: sp_runtime::ModuleError */
    SpRuntimeModuleError: {
        index: "u8",
        error: "[u8;4]",
    },
    /** Lookup27: sp_runtime::TokenError */
    SpRuntimeTokenError: {
        _enum: [
            "FundsUnavailable",
            "OnlyProvider",
            "BelowMinimum",
            "CannotCreate",
            "UnknownAsset",
            "Frozen",
            "Unsupported",
            "CannotCreateHold",
            "NotExpendable",
            "Blocked",
        ],
    },
    /** Lookup28: sp_arithmetic::ArithmeticError */
    SpArithmeticArithmeticError: {
        _enum: ["Underflow", "Overflow", "DivisionByZero"],
    },
    /** Lookup29: sp_runtime::TransactionalError */
    SpRuntimeTransactionalError: {
        _enum: ["LimitReached", "NoLayer"],
    },
    /** Lookup30: cumulus_pallet_parachain_system::pallet::Event<T> */
    CumulusPalletParachainSystemEvent: {
        _enum: {
            ValidationFunctionStored: "Null",
            ValidationFunctionApplied: {
                relayChainBlockNum: "u32",
            },
            ValidationFunctionDiscarded: "Null",
            UpgradeAuthorized: {
                codeHash: "H256",
            },
            DownwardMessagesReceived: {
                count: "u32",
            },
            DownwardMessagesProcessed: {
                weightUsed: "SpWeightsWeightV2Weight",
                dmqHead: "H256",
            },
            UpwardMessageSent: {
                messageHash: "Option<[u8;32]>",
            },
        },
    },
    /** Lookup32: pallet_sudo::pallet::Event<T> */
    PalletSudoEvent: {
        _enum: {
            Sudid: {
                sudoResult: "Result<Null, SpRuntimeDispatchError>",
            },
            KeyChanged: {
                oldSudoer: "Option<AccountId32>",
            },
            SudoAsDone: {
                sudoResult: "Result<Null, SpRuntimeDispatchError>",
            },
        },
    },
    /** Lookup36: pallet_utility::pallet::Event */
    PalletUtilityEvent: {
        _enum: {
            BatchInterrupted: {
                index: "u32",
                error: "SpRuntimeDispatchError",
            },
            BatchCompleted: "Null",
            BatchCompletedWithErrors: "Null",
            ItemCompleted: "Null",
            ItemFailed: {
                error: "SpRuntimeDispatchError",
            },
            DispatchedAs: {
                result: "Result<Null, SpRuntimeDispatchError>",
            },
        },
    },
    /** Lookup37: pallet_proxy::pallet::Event<T> */
    PalletProxyEvent: {
        _enum: {
            ProxyExecuted: {
                result: "Result<Null, SpRuntimeDispatchError>",
            },
            PureCreated: {
                pure: "AccountId32",
                who: "AccountId32",
                proxyType: "DanceboxRuntimeProxyType",
                disambiguationIndex: "u16",
            },
            Announced: {
                real: "AccountId32",
                proxy: "AccountId32",
                callHash: "H256",
            },
            ProxyAdded: {
                delegator: "AccountId32",
                delegatee: "AccountId32",
                proxyType: "DanceboxRuntimeProxyType",
                delay: "u32",
            },
            ProxyRemoved: {
                delegator: "AccountId32",
                delegatee: "AccountId32",
                proxyType: "DanceboxRuntimeProxyType",
                delay: "u32",
            },
        },
    },
    /** Lookup38: dancebox_runtime::ProxyType */
    DanceboxRuntimeProxyType: {
        _enum: ["Any", "NonTransfer", "Governance", "Staking", "CancelProxy", "Balances"],
    },
    /** Lookup40: pallet_migrations::pallet::Event<T> */
    PalletMigrationsEvent: {
        _enum: {
            RuntimeUpgradeStarted: "Null",
            RuntimeUpgradeCompleted: {
                weight: "SpWeightsWeightV2Weight",
            },
            MigrationStarted: {
                migrationName: "Bytes",
            },
            MigrationCompleted: {
                migrationName: "Bytes",
                consumedWeight: "SpWeightsWeightV2Weight",
            },
            FailedToSuspendIdleXcmExecution: {
                error: "SpRuntimeDispatchError",
            },
            FailedToResumeIdleXcmExecution: {
                error: "SpRuntimeDispatchError",
            },
        },
    },
    /** Lookup41: pallet_maintenance_mode::pallet::Event */
    PalletMaintenanceModeEvent: {
        _enum: {
            EnteredMaintenanceMode: "Null",
            NormalOperationResumed: "Null",
            FailedToSuspendIdleXcmExecution: {
                error: "SpRuntimeDispatchError",
            },
            FailedToResumeIdleXcmExecution: {
                error: "SpRuntimeDispatchError",
            },
        },
    },
    /** Lookup42: pallet_balances::pallet::Event<T, I> */
    PalletBalancesEvent: {
        _enum: {
            Endowed: {
                account: "AccountId32",
                freeBalance: "u128",
            },
            DustLost: {
                account: "AccountId32",
                amount: "u128",
            },
            Transfer: {
                from: "AccountId32",
                to: "AccountId32",
                amount: "u128",
            },
            BalanceSet: {
                who: "AccountId32",
                free: "u128",
            },
            Reserved: {
                who: "AccountId32",
                amount: "u128",
            },
            Unreserved: {
                who: "AccountId32",
                amount: "u128",
            },
            ReserveRepatriated: {
                from: "AccountId32",
                to: "AccountId32",
                amount: "u128",
                destinationStatus: "FrameSupportTokensMiscBalanceStatus",
            },
            Deposit: {
                who: "AccountId32",
                amount: "u128",
            },
            Withdraw: {
                who: "AccountId32",
                amount: "u128",
            },
            Slashed: {
                who: "AccountId32",
                amount: "u128",
            },
            Minted: {
                who: "AccountId32",
                amount: "u128",
            },
            Burned: {
                who: "AccountId32",
                amount: "u128",
            },
            Suspended: {
                who: "AccountId32",
                amount: "u128",
            },
            Restored: {
                who: "AccountId32",
                amount: "u128",
            },
            Upgraded: {
                who: "AccountId32",
            },
            Issued: {
                amount: "u128",
            },
            Rescinded: {
                amount: "u128",
            },
            Locked: {
                who: "AccountId32",
                amount: "u128",
            },
            Unlocked: {
                who: "AccountId32",
                amount: "u128",
            },
            Frozen: {
                who: "AccountId32",
                amount: "u128",
            },
            Thawed: {
                who: "AccountId32",
                amount: "u128",
            },
        },
    },
    /** Lookup43: frame_support::traits::tokens::misc::BalanceStatus */
    FrameSupportTokensMiscBalanceStatus: {
        _enum: ["Free", "Reserved"],
    },
    /** Lookup44: pallet_transaction_payment::pallet::Event<T> */
    PalletTransactionPaymentEvent: {
        _enum: {
            TransactionFeePaid: {
                who: "AccountId32",
                actualFee: "u128",
                tip: "u128",
            },
        },
    },
    /** Lookup45: pallet_registrar::pallet::Event<T> */
    PalletRegistrarEvent: {
        _enum: {
            ParaIdRegistered: {
                paraId: "u32",
            },
            ParaIdDeregistered: {
                paraId: "u32",
            },
            ParaIdValidForCollating: {
                paraId: "u32",
            },
            ParaIdPaused: {
                paraId: "u32",
            },
            BootNodesChanged: {
                paraId: "u32",
            },
        },
    },
    /** Lookup47: pallet_collator_assignment::pallet::Event<T> */
    PalletCollatorAssignmentEvent: {
        _enum: {
            NewPendingAssignment: {
                randomSeed: "[u8;32]",
                fullRotation: "bool",
                targetSession: "u32",
            },
        },
    },
    /** Lookup49: pallet_author_noting::pallet::Event<T> */
    PalletAuthorNotingEvent: {
        _enum: {
            LatestAuthorChanged: {
                paraId: "u32",
                blockNumber: "u32",
                newAuthor: "AccountId32",
            },
            RemovedAuthorData: {
                paraId: "u32",
            },
        },
    },
    /** Lookup50: pallet_services_payment::pallet::Event<T> */
    PalletServicesPaymentEvent: {
        _enum: {
            CreditsPurchased: {
                paraId: "u32",
                payer: "AccountId32",
                fee: "u128",
                creditsPurchased: "u32",
                creditsRemaining: "u32",
            },
            CreditBurned: {
                paraId: "u32",
                creditsRemaining: "u32",
            },
            CreditsSet: {
                paraId: "u32",
                credits: "u32",
            },
        },
    },
    /** Lookup51: pallet_invulnerables::pallet::Event<T> */
    PalletInvulnerablesEvent: {
        _enum: {
            NewInvulnerables: {
                invulnerables: "Vec<AccountId32>",
            },
            InvulnerableAdded: {
                accountId: "AccountId32",
            },
            InvulnerableRemoved: {
                accountId: "AccountId32",
            },
            InvalidInvulnerableSkipped: {
                accountId: "AccountId32",
            },
        },
    },
    /** Lookup53: pallet_session::pallet::Event */
    PalletSessionEvent: {
        _enum: {
            NewSession: {
                sessionIndex: "u32",
            },
        },
    },
    /** Lookup54: pallet_pooled_staking::pallet::Event<T> */
    PalletPooledStakingEvent: {
        _enum: {
            UpdatedCandidatePosition: {
                candidate: "AccountId32",
                stake: "u128",
                selfDelegation: "u128",
                before: "Option<u32>",
                after: "Option<u32>",
            },
            RequestedDelegate: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                pool: "PalletPooledStakingTargetPool",
                pending: "u128",
            },
            ExecutedDelegate: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                pool: "PalletPooledStakingTargetPool",
                staked: "u128",
                released: "u128",
            },
            RequestedUndelegate: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                from: "PalletPooledStakingTargetPool",
                pending: "u128",
                released: "u128",
            },
            ExecutedUndelegate: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                released: "u128",
            },
            IncreasedStake: {
                candidate: "AccountId32",
                stakeDiff: "u128",
            },
            DecreasedStake: {
                candidate: "AccountId32",
                stakeDiff: "u128",
            },
            StakedAutoCompounding: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                shares: "u128",
                stake: "u128",
            },
            UnstakedAutoCompounding: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                shares: "u128",
                stake: "u128",
            },
            StakedManualRewards: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                shares: "u128",
                stake: "u128",
            },
            UnstakedManualRewards: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                shares: "u128",
                stake: "u128",
            },
            RewardedCollator: {
                collator: "AccountId32",
                autoCompoundingRewards: "u128",
                manualClaimRewards: "u128",
            },
            RewardedDelegators: {
                collator: "AccountId32",
                autoCompoundingRewards: "u128",
                manualClaimRewards: "u128",
            },
            ClaimedManualRewards: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                rewards: "u128",
            },
            SwappedPool: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                sourcePool: "PalletPooledStakingTargetPool",
                sourceShares: "u128",
                sourceStake: "u128",
                targetShares: "u128",
                targetStake: "u128",
                pendingLeaving: "u128",
                released: "u128",
            },
        },
    },
    /** Lookup56: pallet_pooled_staking::pallet::TargetPool */
    PalletPooledStakingTargetPool: {
        _enum: ["AutoCompounding", "ManualRewards"],
    },
    /** Lookup57: pallet_inflation_rewards::pallet::Event<T> */
    PalletInflationRewardsEvent: {
        _enum: {
            RewardedOrchestrator: {
                accountId: "AccountId32",
                balance: "u128",
            },
            RewardedContainer: {
                accountId: "AccountId32",
                paraId: "u32",
                balance: "u128",
            },
        },
    },
    /** Lookup58: cumulus_pallet_xcmp_queue::pallet::Event<T> */
    CumulusPalletXcmpQueueEvent: {
        _enum: {
            Success: {
                messageHash: "[u8;32]",
                messageId: "[u8;32]",
                weight: "SpWeightsWeightV2Weight",
            },
            Fail: {
                messageHash: "[u8;32]",
                messageId: "[u8;32]",
                error: "StagingXcmV3TraitsError",
                weight: "SpWeightsWeightV2Weight",
            },
            BadVersion: {
                messageHash: "[u8;32]",
            },
            BadFormat: {
                messageHash: "[u8;32]",
            },
            XcmpMessageSent: {
                messageHash: "[u8;32]",
            },
            OverweightEnqueued: {
                sender: "u32",
                sentAt: "u32",
                index: "u64",
                required: "SpWeightsWeightV2Weight",
            },
            OverweightServiced: {
                index: "u64",
                used: "SpWeightsWeightV2Weight",
            },
        },
    },
    /** Lookup59: staging_xcm::v3::traits::Error */
    StagingXcmV3TraitsError: {
        _enum: {
            Overflow: "Null",
            Unimplemented: "Null",
            UntrustedReserveLocation: "Null",
            UntrustedTeleportLocation: "Null",
            LocationFull: "Null",
            LocationNotInvertible: "Null",
            BadOrigin: "Null",
            InvalidLocation: "Null",
            AssetNotFound: "Null",
            FailedToTransactAsset: "Null",
            NotWithdrawable: "Null",
            LocationCannotHold: "Null",
            ExceedsMaxMessageSize: "Null",
            DestinationUnsupported: "Null",
            Transport: "Null",
            Unroutable: "Null",
            UnknownClaim: "Null",
            FailedToDecode: "Null",
            MaxWeightInvalid: "Null",
            NotHoldingFees: "Null",
            TooExpensive: "Null",
            Trap: "u64",
            ExpectationFalse: "Null",
            PalletNotFound: "Null",
            NameMismatch: "Null",
            VersionIncompatible: "Null",
            HoldingWouldOverflow: "Null",
            ExportError: "Null",
            ReanchorFailed: "Null",
            NoDeal: "Null",
            FeesNotMet: "Null",
            LockError: "Null",
            NoPermission: "Null",
            Unanchored: "Null",
            NotDepositable: "Null",
            UnhandledXcmVersion: "Null",
            WeightLimitReached: "SpWeightsWeightV2Weight",
            Barrier: "Null",
            WeightNotComputable: "Null",
            ExceedsStackLimit: "Null",
        },
    },
    /** Lookup60: cumulus_pallet_xcm::pallet::Event<T> */
    CumulusPalletXcmEvent: {
        _enum: {
            InvalidFormat: "[u8;32]",
            UnsupportedVersion: "[u8;32]",
            ExecutedDownward: "([u8;32],StagingXcmV3TraitsOutcome)",
        },
    },
    /** Lookup61: staging_xcm::v3::traits::Outcome */
    StagingXcmV3TraitsOutcome: {
        _enum: {
            Complete: "SpWeightsWeightV2Weight",
            Incomplete: "(SpWeightsWeightV2Weight,StagingXcmV3TraitsError)",
            Error: "StagingXcmV3TraitsError",
        },
    },
    /** Lookup62: cumulus_pallet_dmp_queue::pallet::Event<T> */
    CumulusPalletDmpQueueEvent: {
        _enum: {
            InvalidFormat: {
                messageHash: "[u8;32]",
            },
            UnsupportedVersion: {
                messageHash: "[u8;32]",
            },
            ExecutedDownward: {
                messageHash: "[u8;32]",
                messageId: "[u8;32]",
                outcome: "StagingXcmV3TraitsOutcome",
            },
            WeightExhausted: {
                messageHash: "[u8;32]",
                messageId: "[u8;32]",
                remainingWeight: "SpWeightsWeightV2Weight",
                requiredWeight: "SpWeightsWeightV2Weight",
            },
            OverweightEnqueued: {
                messageHash: "[u8;32]",
                messageId: "[u8;32]",
                overweightIndex: "u64",
                requiredWeight: "SpWeightsWeightV2Weight",
            },
            OverweightServiced: {
                overweightIndex: "u64",
                weightUsed: "SpWeightsWeightV2Weight",
            },
            MaxMessagesExhausted: {
                messageHash: "[u8;32]",
            },
        },
    },
    /** Lookup63: pallet_xcm::pallet::Event<T> */
    PalletXcmEvent: {
        _enum: {
            Attempted: {
                outcome: "StagingXcmV3TraitsOutcome",
            },
            Sent: {
                origin: "StagingXcmV3MultiLocation",
                destination: "StagingXcmV3MultiLocation",
                message: "StagingXcmV3Xcm",
                messageId: "[u8;32]",
            },
            UnexpectedResponse: {
                origin: "StagingXcmV3MultiLocation",
                queryId: "u64",
            },
            ResponseReady: {
                queryId: "u64",
                response: "StagingXcmV3Response",
            },
            Notified: {
                queryId: "u64",
                palletIndex: "u8",
                callIndex: "u8",
            },
            NotifyOverweight: {
                queryId: "u64",
                palletIndex: "u8",
                callIndex: "u8",
                actualWeight: "SpWeightsWeightV2Weight",
                maxBudgetedWeight: "SpWeightsWeightV2Weight",
            },
            NotifyDispatchError: {
                queryId: "u64",
                palletIndex: "u8",
                callIndex: "u8",
            },
            NotifyDecodeFailed: {
                queryId: "u64",
                palletIndex: "u8",
                callIndex: "u8",
            },
            InvalidResponder: {
                origin: "StagingXcmV3MultiLocation",
                queryId: "u64",
                expectedLocation: "Option<StagingXcmV3MultiLocation>",
            },
            InvalidResponderVersion: {
                origin: "StagingXcmV3MultiLocation",
                queryId: "u64",
            },
            ResponseTaken: {
                queryId: "u64",
            },
            AssetsTrapped: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
                origin: "StagingXcmV3MultiLocation",
                assets: "StagingXcmVersionedMultiAssets",
            },
            VersionChangeNotified: {
                destination: "StagingXcmV3MultiLocation",
                result: "u32",
                cost: "StagingXcmV3MultiassetMultiAssets",
                messageId: "[u8;32]",
            },
            SupportedVersionChanged: {
                location: "StagingXcmV3MultiLocation",
                version: "u32",
            },
            NotifyTargetSendFail: {
                location: "StagingXcmV3MultiLocation",
                queryId: "u64",
                error: "StagingXcmV3TraitsError",
            },
            NotifyTargetMigrationFail: {
                location: "StagingXcmVersionedMultiLocation",
                queryId: "u64",
            },
            InvalidQuerierVersion: {
                origin: "StagingXcmV3MultiLocation",
                queryId: "u64",
            },
            InvalidQuerier: {
                origin: "StagingXcmV3MultiLocation",
                queryId: "u64",
                expectedQuerier: "StagingXcmV3MultiLocation",
                maybeActualQuerier: "Option<StagingXcmV3MultiLocation>",
            },
            VersionNotifyStarted: {
                destination: "StagingXcmV3MultiLocation",
                cost: "StagingXcmV3MultiassetMultiAssets",
                messageId: "[u8;32]",
            },
            VersionNotifyRequested: {
                destination: "StagingXcmV3MultiLocation",
                cost: "StagingXcmV3MultiassetMultiAssets",
                messageId: "[u8;32]",
            },
            VersionNotifyUnrequested: {
                destination: "StagingXcmV3MultiLocation",
                cost: "StagingXcmV3MultiassetMultiAssets",
                messageId: "[u8;32]",
            },
            FeesPaid: {
                paying: "StagingXcmV3MultiLocation",
                fees: "StagingXcmV3MultiassetMultiAssets",
            },
            AssetsClaimed: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
                origin: "StagingXcmV3MultiLocation",
                assets: "StagingXcmVersionedMultiAssets",
            },
        },
    },
    /** Lookup64: staging_xcm::v3::multilocation::MultiLocation */
    StagingXcmV3MultiLocation: {
        parents: "u8",
        interior: "StagingXcmV3Junctions",
    },
    /** Lookup65: staging_xcm::v3::junctions::Junctions */
    StagingXcmV3Junctions: {
        _enum: {
            Here: "Null",
            X1: "StagingXcmV3Junction",
            X2: "(StagingXcmV3Junction,StagingXcmV3Junction)",
            X3: "(StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction)",
            X4: "(StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction)",
            X5: "(StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction)",
            X6: "(StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction)",
            X7: "(StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction)",
            X8: "(StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction,StagingXcmV3Junction)",
        },
    },
    /** Lookup66: staging_xcm::v3::junction::Junction */
    StagingXcmV3Junction: {
        _enum: {
            Parachain: "Compact<u32>",
            AccountId32: {
                network: "Option<StagingXcmV3JunctionNetworkId>",
                id: "[u8;32]",
            },
            AccountIndex64: {
                network: "Option<StagingXcmV3JunctionNetworkId>",
                index: "Compact<u64>",
            },
            AccountKey20: {
                network: "Option<StagingXcmV3JunctionNetworkId>",
                key: "[u8;20]",
            },
            PalletInstance: "u8",
            GeneralIndex: "Compact<u128>",
            GeneralKey: {
                length: "u8",
                data: "[u8;32]",
            },
            OnlyChild: "Null",
            Plurality: {
                id: "StagingXcmV3JunctionBodyId",
                part: "StagingXcmV3JunctionBodyPart",
            },
            GlobalConsensus: "StagingXcmV3JunctionNetworkId",
        },
    },
    /** Lookup69: staging_xcm::v3::junction::NetworkId */
    StagingXcmV3JunctionNetworkId: {
        _enum: {
            ByGenesis: "[u8;32]",
            ByFork: {
                blockNumber: "u64",
                blockHash: "[u8;32]",
            },
            Polkadot: "Null",
            Kusama: "Null",
            Westend: "Null",
            Rococo: "Null",
            Wococo: "Null",
            Ethereum: {
                chainId: "Compact<u64>",
            },
            BitcoinCore: "Null",
            BitcoinCash: "Null",
        },
    },
    /** Lookup72: staging_xcm::v3::junction::BodyId */
    StagingXcmV3JunctionBodyId: {
        _enum: {
            Unit: "Null",
            Moniker: "[u8;4]",
            Index: "Compact<u32>",
            Executive: "Null",
            Technical: "Null",
            Legislative: "Null",
            Judicial: "Null",
            Defense: "Null",
            Administration: "Null",
            Treasury: "Null",
        },
    },
    /** Lookup73: staging_xcm::v3::junction::BodyPart */
    StagingXcmV3JunctionBodyPart: {
        _enum: {
            Voice: "Null",
            Members: {
                count: "Compact<u32>",
            },
            Fraction: {
                nom: "Compact<u32>",
                denom: "Compact<u32>",
            },
            AtLeastProportion: {
                nom: "Compact<u32>",
                denom: "Compact<u32>",
            },
            MoreThanProportion: {
                nom: "Compact<u32>",
                denom: "Compact<u32>",
            },
        },
    },
    /** Lookup74: staging_xcm::v3::Xcm<Call> */
    StagingXcmV3Xcm: "Vec<StagingXcmV3Instruction>",
    /** Lookup76: staging_xcm::v3::Instruction<Call> */
    StagingXcmV3Instruction: {
        _enum: {
            WithdrawAsset: "StagingXcmV3MultiassetMultiAssets",
            ReserveAssetDeposited: "StagingXcmV3MultiassetMultiAssets",
            ReceiveTeleportedAsset: "StagingXcmV3MultiassetMultiAssets",
            QueryResponse: {
                queryId: "Compact<u64>",
                response: "StagingXcmV3Response",
                maxWeight: "SpWeightsWeightV2Weight",
                querier: "Option<StagingXcmV3MultiLocation>",
            },
            TransferAsset: {
                assets: "StagingXcmV3MultiassetMultiAssets",
                beneficiary: "StagingXcmV3MultiLocation",
            },
            TransferReserveAsset: {
                assets: "StagingXcmV3MultiassetMultiAssets",
                dest: "StagingXcmV3MultiLocation",
                xcm: "StagingXcmV3Xcm",
            },
            Transact: {
                originKind: "StagingXcmV2OriginKind",
                requireWeightAtMost: "SpWeightsWeightV2Weight",
                call: "StagingXcmDoubleEncoded",
            },
            HrmpNewChannelOpenRequest: {
                sender: "Compact<u32>",
                maxMessageSize: "Compact<u32>",
                maxCapacity: "Compact<u32>",
            },
            HrmpChannelAccepted: {
                recipient: "Compact<u32>",
            },
            HrmpChannelClosing: {
                initiator: "Compact<u32>",
                sender: "Compact<u32>",
                recipient: "Compact<u32>",
            },
            ClearOrigin: "Null",
            DescendOrigin: "StagingXcmV3Junctions",
            ReportError: "StagingXcmV3QueryResponseInfo",
            DepositAsset: {
                assets: "StagingXcmV3MultiassetMultiAssetFilter",
                beneficiary: "StagingXcmV3MultiLocation",
            },
            DepositReserveAsset: {
                assets: "StagingXcmV3MultiassetMultiAssetFilter",
                dest: "StagingXcmV3MultiLocation",
                xcm: "StagingXcmV3Xcm",
            },
            ExchangeAsset: {
                give: "StagingXcmV3MultiassetMultiAssetFilter",
                want: "StagingXcmV3MultiassetMultiAssets",
                maximal: "bool",
            },
            InitiateReserveWithdraw: {
                assets: "StagingXcmV3MultiassetMultiAssetFilter",
                reserve: "StagingXcmV3MultiLocation",
                xcm: "StagingXcmV3Xcm",
            },
            InitiateTeleport: {
                assets: "StagingXcmV3MultiassetMultiAssetFilter",
                dest: "StagingXcmV3MultiLocation",
                xcm: "StagingXcmV3Xcm",
            },
            ReportHolding: {
                responseInfo: "StagingXcmV3QueryResponseInfo",
                assets: "StagingXcmV3MultiassetMultiAssetFilter",
            },
            BuyExecution: {
                fees: "StagingXcmV3MultiAsset",
                weightLimit: "StagingXcmV3WeightLimit",
            },
            RefundSurplus: "Null",
            SetErrorHandler: "StagingXcmV3Xcm",
            SetAppendix: "StagingXcmV3Xcm",
            ClearError: "Null",
            ClaimAsset: {
                assets: "StagingXcmV3MultiassetMultiAssets",
                ticket: "StagingXcmV3MultiLocation",
            },
            Trap: "Compact<u64>",
            SubscribeVersion: {
                queryId: "Compact<u64>",
                maxResponseWeight: "SpWeightsWeightV2Weight",
            },
            UnsubscribeVersion: "Null",
            BurnAsset: "StagingXcmV3MultiassetMultiAssets",
            ExpectAsset: "StagingXcmV3MultiassetMultiAssets",
            ExpectOrigin: "Option<StagingXcmV3MultiLocation>",
            ExpectError: "Option<(u32,StagingXcmV3TraitsError)>",
            ExpectTransactStatus: "StagingXcmV3MaybeErrorCode",
            QueryPallet: {
                moduleName: "Bytes",
                responseInfo: "StagingXcmV3QueryResponseInfo",
            },
            ExpectPallet: {
                index: "Compact<u32>",
                name: "Bytes",
                moduleName: "Bytes",
                crateMajor: "Compact<u32>",
                minCrateMinor: "Compact<u32>",
            },
            ReportTransactStatus: "StagingXcmV3QueryResponseInfo",
            ClearTransactStatus: "Null",
            UniversalOrigin: "StagingXcmV3Junction",
            ExportMessage: {
                network: "StagingXcmV3JunctionNetworkId",
                destination: "StagingXcmV3Junctions",
                xcm: "StagingXcmV3Xcm",
            },
            LockAsset: {
                asset: "StagingXcmV3MultiAsset",
                unlocker: "StagingXcmV3MultiLocation",
            },
            UnlockAsset: {
                asset: "StagingXcmV3MultiAsset",
                target: "StagingXcmV3MultiLocation",
            },
            NoteUnlockable: {
                asset: "StagingXcmV3MultiAsset",
                owner: "StagingXcmV3MultiLocation",
            },
            RequestUnlock: {
                asset: "StagingXcmV3MultiAsset",
                locker: "StagingXcmV3MultiLocation",
            },
            SetFeesMode: {
                jitWithdraw: "bool",
            },
            SetTopic: "[u8;32]",
            ClearTopic: "Null",
            AliasOrigin: "StagingXcmV3MultiLocation",
            UnpaidExecution: {
                weightLimit: "StagingXcmV3WeightLimit",
                checkOrigin: "Option<StagingXcmV3MultiLocation>",
            },
        },
    },
    /** Lookup77: staging_xcm::v3::multiasset::MultiAssets */
    StagingXcmV3MultiassetMultiAssets: "Vec<StagingXcmV3MultiAsset>",
    /** Lookup79: staging_xcm::v3::multiasset::MultiAsset */
    StagingXcmV3MultiAsset: {
        id: "StagingXcmV3MultiassetAssetId",
        fun: "StagingXcmV3MultiassetFungibility",
    },
    /** Lookup80: staging_xcm::v3::multiasset::AssetId */
    StagingXcmV3MultiassetAssetId: {
        _enum: {
            Concrete: "StagingXcmV3MultiLocation",
            Abstract: "[u8;32]",
        },
    },
    /** Lookup81: staging_xcm::v3::multiasset::Fungibility */
    StagingXcmV3MultiassetFungibility: {
        _enum: {
            Fungible: "Compact<u128>",
            NonFungible: "StagingXcmV3MultiassetAssetInstance",
        },
    },
    /** Lookup82: staging_xcm::v3::multiasset::AssetInstance */
    StagingXcmV3MultiassetAssetInstance: {
        _enum: {
            Undefined: "Null",
            Index: "Compact<u128>",
            Array4: "[u8;4]",
            Array8: "[u8;8]",
            Array16: "[u8;16]",
            Array32: "[u8;32]",
        },
    },
    /** Lookup85: staging_xcm::v3::Response */
    StagingXcmV3Response: {
        _enum: {
            Null: "Null",
            Assets: "StagingXcmV3MultiassetMultiAssets",
            ExecutionResult: "Option<(u32,StagingXcmV3TraitsError)>",
            Version: "u32",
            PalletsInfo: "Vec<StagingXcmV3PalletInfo>",
            DispatchResult: "StagingXcmV3MaybeErrorCode",
        },
    },
    /** Lookup89: staging_xcm::v3::PalletInfo */
    StagingXcmV3PalletInfo: {
        index: "Compact<u32>",
        name: "Bytes",
        moduleName: "Bytes",
        major: "Compact<u32>",
        minor: "Compact<u32>",
        patch: "Compact<u32>",
    },
    /** Lookup92: staging_xcm::v3::MaybeErrorCode */
    StagingXcmV3MaybeErrorCode: {
        _enum: {
            Success: "Null",
            Error: "Bytes",
            TruncatedError: "Bytes",
        },
    },
    /** Lookup95: staging_xcm::v2::OriginKind */
    StagingXcmV2OriginKind: {
        _enum: ["Native", "SovereignAccount", "Superuser", "Xcm"],
    },
    /** Lookup96: staging_xcm::double_encoded::DoubleEncoded<T> */
    StagingXcmDoubleEncoded: {
        encoded: "Bytes",
    },
    /** Lookup97: staging_xcm::v3::QueryResponseInfo */
    StagingXcmV3QueryResponseInfo: {
        destination: "StagingXcmV3MultiLocation",
        queryId: "Compact<u64>",
        maxWeight: "SpWeightsWeightV2Weight",
    },
    /** Lookup98: staging_xcm::v3::multiasset::MultiAssetFilter */
    StagingXcmV3MultiassetMultiAssetFilter: {
        _enum: {
            Definite: "StagingXcmV3MultiassetMultiAssets",
            Wild: "StagingXcmV3MultiassetWildMultiAsset",
        },
    },
    /** Lookup99: staging_xcm::v3::multiasset::WildMultiAsset */
    StagingXcmV3MultiassetWildMultiAsset: {
        _enum: {
            All: "Null",
            AllOf: {
                id: "StagingXcmV3MultiassetAssetId",
                fun: "StagingXcmV3MultiassetWildFungibility",
            },
            AllCounted: "Compact<u32>",
            AllOfCounted: {
                id: "StagingXcmV3MultiassetAssetId",
                fun: "StagingXcmV3MultiassetWildFungibility",
                count: "Compact<u32>",
            },
        },
    },
    /** Lookup100: staging_xcm::v3::multiasset::WildFungibility */
    StagingXcmV3MultiassetWildFungibility: {
        _enum: ["Fungible", "NonFungible"],
    },
    /** Lookup101: staging_xcm::v3::WeightLimit */
    StagingXcmV3WeightLimit: {
        _enum: {
            Unlimited: "Null",
            Limited: "SpWeightsWeightV2Weight",
        },
    },
    /** Lookup102: staging_xcm::VersionedMultiAssets */
    StagingXcmVersionedMultiAssets: {
        _enum: {
            __Unused0: "Null",
            V2: "StagingXcmV2MultiassetMultiAssets",
            __Unused2: "Null",
            V3: "StagingXcmV3MultiassetMultiAssets",
        },
    },
    /** Lookup103: staging_xcm::v2::multiasset::MultiAssets */
    StagingXcmV2MultiassetMultiAssets: "Vec<StagingXcmV2MultiAsset>",
    /** Lookup105: staging_xcm::v2::multiasset::MultiAsset */
    StagingXcmV2MultiAsset: {
        id: "StagingXcmV2MultiassetAssetId",
        fun: "StagingXcmV2MultiassetFungibility",
    },
    /** Lookup106: staging_xcm::v2::multiasset::AssetId */
    StagingXcmV2MultiassetAssetId: {
        _enum: {
            Concrete: "StagingXcmV2MultiLocation",
            Abstract: "Bytes",
        },
    },
    /** Lookup107: staging_xcm::v2::multilocation::MultiLocation */
    StagingXcmV2MultiLocation: {
        parents: "u8",
        interior: "StagingXcmV2MultilocationJunctions",
    },
    /** Lookup108: staging_xcm::v2::multilocation::Junctions */
    StagingXcmV2MultilocationJunctions: {
        _enum: {
            Here: "Null",
            X1: "StagingXcmV2Junction",
            X2: "(StagingXcmV2Junction,StagingXcmV2Junction)",
            X3: "(StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction)",
            X4: "(StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction)",
            X5: "(StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction)",
            X6: "(StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction)",
            X7: "(StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction)",
            X8: "(StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction,StagingXcmV2Junction)",
        },
    },
    /** Lookup109: staging_xcm::v2::junction::Junction */
    StagingXcmV2Junction: {
        _enum: {
            Parachain: "Compact<u32>",
            AccountId32: {
                network: "StagingXcmV2NetworkId",
                id: "[u8;32]",
            },
            AccountIndex64: {
                network: "StagingXcmV2NetworkId",
                index: "Compact<u64>",
            },
            AccountKey20: {
                network: "StagingXcmV2NetworkId",
                key: "[u8;20]",
            },
            PalletInstance: "u8",
            GeneralIndex: "Compact<u128>",
            GeneralKey: "Bytes",
            OnlyChild: "Null",
            Plurality: {
                id: "StagingXcmV2BodyId",
                part: "StagingXcmV2BodyPart",
            },
        },
    },
    /** Lookup110: staging_xcm::v2::NetworkId */
    StagingXcmV2NetworkId: {
        _enum: {
            Any: "Null",
            Named: "Bytes",
            Polkadot: "Null",
            Kusama: "Null",
        },
    },
    /** Lookup112: staging_xcm::v2::BodyId */
    StagingXcmV2BodyId: {
        _enum: {
            Unit: "Null",
            Named: "Bytes",
            Index: "Compact<u32>",
            Executive: "Null",
            Technical: "Null",
            Legislative: "Null",
            Judicial: "Null",
            Defense: "Null",
            Administration: "Null",
            Treasury: "Null",
        },
    },
    /** Lookup113: staging_xcm::v2::BodyPart */
    StagingXcmV2BodyPart: {
        _enum: {
            Voice: "Null",
            Members: {
                count: "Compact<u32>",
            },
            Fraction: {
                nom: "Compact<u32>",
                denom: "Compact<u32>",
            },
            AtLeastProportion: {
                nom: "Compact<u32>",
                denom: "Compact<u32>",
            },
            MoreThanProportion: {
                nom: "Compact<u32>",
                denom: "Compact<u32>",
            },
        },
    },
    /** Lookup114: staging_xcm::v2::multiasset::Fungibility */
    StagingXcmV2MultiassetFungibility: {
        _enum: {
            Fungible: "Compact<u128>",
            NonFungible: "StagingXcmV2MultiassetAssetInstance",
        },
    },
    /** Lookup115: staging_xcm::v2::multiasset::AssetInstance */
    StagingXcmV2MultiassetAssetInstance: {
        _enum: {
            Undefined: "Null",
            Index: "Compact<u128>",
            Array4: "[u8;4]",
            Array8: "[u8;8]",
            Array16: "[u8;16]",
            Array32: "[u8;32]",
            Blob: "Bytes",
        },
    },
    /** Lookup116: staging_xcm::VersionedMultiLocation */
    StagingXcmVersionedMultiLocation: {
        _enum: {
            __Unused0: "Null",
            V2: "StagingXcmV2MultiLocation",
            __Unused2: "Null",
            V3: "StagingXcmV3MultiLocation",
        },
    },
    /** Lookup117: frame_system::Phase */
    FrameSystemPhase: {
        _enum: {
            ApplyExtrinsic: "u32",
            Finalization: "Null",
            Initialization: "Null",
        },
    },
    /** Lookup121: frame_system::LastRuntimeUpgradeInfo */
    FrameSystemLastRuntimeUpgradeInfo: {
        specVersion: "Compact<u32>",
        specName: "Text",
    },
    /** Lookup123: frame_system::pallet::Call<T> */
    FrameSystemCall: {
        _enum: {
            remark: {
                remark: "Bytes",
            },
            set_heap_pages: {
                pages: "u64",
            },
            set_code: {
                code: "Bytes",
            },
            set_code_without_checks: {
                code: "Bytes",
            },
            set_storage: {
                items: "Vec<(Bytes,Bytes)>",
            },
            kill_storage: {
                _alias: {
                    keys_: "keys",
                },
                keys_: "Vec<Bytes>",
            },
            kill_prefix: {
                prefix: "Bytes",
                subkeys: "u32",
            },
            remark_with_event: {
                remark: "Bytes",
            },
        },
    },
    /** Lookup127: frame_system::limits::BlockWeights */
    FrameSystemLimitsBlockWeights: {
        baseBlock: "SpWeightsWeightV2Weight",
        maxBlock: "SpWeightsWeightV2Weight",
        perClass: "FrameSupportDispatchPerDispatchClassWeightsPerClass",
    },
    /** Lookup128: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass> */
    FrameSupportDispatchPerDispatchClassWeightsPerClass: {
        normal: "FrameSystemLimitsWeightsPerClass",
        operational: "FrameSystemLimitsWeightsPerClass",
        mandatory: "FrameSystemLimitsWeightsPerClass",
    },
    /** Lookup129: frame_system::limits::WeightsPerClass */
    FrameSystemLimitsWeightsPerClass: {
        baseExtrinsic: "SpWeightsWeightV2Weight",
        maxExtrinsic: "Option<SpWeightsWeightV2Weight>",
        maxTotal: "Option<SpWeightsWeightV2Weight>",
        reserved: "Option<SpWeightsWeightV2Weight>",
    },
    /** Lookup131: frame_system::limits::BlockLength */
    FrameSystemLimitsBlockLength: {
        max: "FrameSupportDispatchPerDispatchClassU32",
    },
    /** Lookup132: frame_support::dispatch::PerDispatchClass<T> */
    FrameSupportDispatchPerDispatchClassU32: {
        normal: "u32",
        operational: "u32",
        mandatory: "u32",
    },
    /** Lookup133: sp_weights::RuntimeDbWeight */
    SpWeightsRuntimeDbWeight: {
        read: "u64",
        write: "u64",
    },
    /** Lookup134: sp_version::RuntimeVersion */
    SpVersionRuntimeVersion: {
        specName: "Text",
        implName: "Text",
        authoringVersion: "u32",
        specVersion: "u32",
        implVersion: "u32",
        apis: "Vec<([u8;8],u32)>",
        transactionVersion: "u32",
        stateVersion: "u8",
    },
    /** Lookup138: frame_system::pallet::Error<T> */
    FrameSystemError: {
        _enum: [
            "InvalidSpecName",
            "SpecVersionNeedsToIncrease",
            "FailedToExtractRuntimeVersion",
            "NonDefaultComposite",
            "NonZeroRefCount",
            "CallFiltered",
        ],
    },
    /** Lookup140: cumulus_pallet_parachain_system::unincluded_segment::Ancestor<primitive_types::H256> */
    CumulusPalletParachainSystemUnincludedSegmentAncestor: {
        usedBandwidth: "CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth",
        paraHeadHash: "Option<H256>",
        consumedGoAheadSignal: "Option<PolkadotPrimitivesV5UpgradeGoAhead>",
    },
    /** Lookup141: cumulus_pallet_parachain_system::unincluded_segment::UsedBandwidth */
    CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth: {
        umpMsgCount: "u32",
        umpTotalBytes: "u32",
        hrmpOutgoing: "BTreeMap<u32, CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate>",
    },
    /** Lookup143: cumulus_pallet_parachain_system::unincluded_segment::HrmpChannelUpdate */
    CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate: {
        msgCount: "u32",
        totalBytes: "u32",
    },
    /** Lookup148: polkadot_primitives::v5::UpgradeGoAhead */
    PolkadotPrimitivesV5UpgradeGoAhead: {
        _enum: ["Abort", "GoAhead"],
    },
    /** Lookup149: cumulus_pallet_parachain_system::unincluded_segment::SegmentTracker<primitive_types::H256> */
    CumulusPalletParachainSystemUnincludedSegmentSegmentTracker: {
        usedBandwidth: "CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth",
        hrmpWatermark: "Option<u32>",
        consumedGoAheadSignal: "Option<PolkadotPrimitivesV5UpgradeGoAhead>",
    },
    /** Lookup150: polkadot_primitives::v5::PersistedValidationData<primitive_types::H256, N> */
    PolkadotPrimitivesV5PersistedValidationData: {
        parentHead: "Bytes",
        relayParentNumber: "u32",
        relayParentStorageRoot: "H256",
        maxPovSize: "u32",
    },
    /** Lookup153: polkadot_primitives::v5::UpgradeRestriction */
    PolkadotPrimitivesV5UpgradeRestriction: {
        _enum: ["Present"],
    },
    /** Lookup154: sp_trie::storage_proof::StorageProof */
    SpTrieStorageProof: {
        trieNodes: "BTreeSet<Bytes>",
    },
    /** Lookup156: cumulus_pallet_parachain_system::relay_state_snapshot::MessagingStateSnapshot */
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: {
        dmqMqcHead: "H256",
        relayDispatchQueueRemainingCapacity:
            "CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity",
        ingressChannels: "Vec<(u32,PolkadotPrimitivesV5AbridgedHrmpChannel)>",
        egressChannels: "Vec<(u32,PolkadotPrimitivesV5AbridgedHrmpChannel)>",
    },
    /** Lookup157: cumulus_pallet_parachain_system::relay_state_snapshot::RelayDispatchQueueRemainingCapacity */
    CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity: {
        remainingCount: "u32",
        remainingSize: "u32",
    },
    /** Lookup160: polkadot_primitives::v5::AbridgedHrmpChannel */
    PolkadotPrimitivesV5AbridgedHrmpChannel: {
        maxCapacity: "u32",
        maxTotalSize: "u32",
        maxMessageSize: "u32",
        msgCount: "u32",
        totalSize: "u32",
        mqcHead: "Option<H256>",
    },
    /** Lookup161: polkadot_primitives::v5::AbridgedHostConfiguration */
    PolkadotPrimitivesV5AbridgedHostConfiguration: {
        maxCodeSize: "u32",
        maxHeadDataSize: "u32",
        maxUpwardQueueCount: "u32",
        maxUpwardQueueSize: "u32",
        maxUpwardMessageSize: "u32",
        maxUpwardMessageNumPerCandidate: "u32",
        hrmpMaxMessageNumPerCandidate: "u32",
        validationUpgradeCooldown: "u32",
        validationUpgradeDelay: "u32",
        asyncBackingParams: "PolkadotPrimitivesVstagingAsyncBackingParams",
    },
    /** Lookup162: polkadot_primitives::vstaging::AsyncBackingParams */
    PolkadotPrimitivesVstagingAsyncBackingParams: {
        maxCandidateDepth: "u32",
        allowedAncestryLen: "u32",
    },
    /** Lookup168: polkadot_core_primitives::OutboundHrmpMessage<polkadot_parachain_primitives::primitives::Id> */
    PolkadotCorePrimitivesOutboundHrmpMessage: {
        recipient: "u32",
        data: "Bytes",
    },
    /** Lookup169: cumulus_pallet_parachain_system::CodeUpgradeAuthorization<T> */
    CumulusPalletParachainSystemCodeUpgradeAuthorization: {
        codeHash: "H256",
        checkVersion: "bool",
    },
    /** Lookup170: cumulus_pallet_parachain_system::pallet::Call<T> */
    CumulusPalletParachainSystemCall: {
        _enum: {
            set_validation_data: {
                data: "CumulusPrimitivesParachainInherentParachainInherentData",
            },
            sudo_send_upward_message: {
                message: "Bytes",
            },
            authorize_upgrade: {
                codeHash: "H256",
                checkVersion: "bool",
            },
            enact_authorized_upgrade: {
                code: "Bytes",
            },
        },
    },
    /** Lookup171: cumulus_primitives_parachain_inherent::ParachainInherentData */
    CumulusPrimitivesParachainInherentParachainInherentData: {
        validationData: "PolkadotPrimitivesV5PersistedValidationData",
        relayChainState: "SpTrieStorageProof",
        downwardMessages: "Vec<PolkadotCorePrimitivesInboundDownwardMessage>",
        horizontalMessages: "BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>",
    },
    /** Lookup173: polkadot_core_primitives::InboundDownwardMessage<BlockNumber> */
    PolkadotCorePrimitivesInboundDownwardMessage: {
        sentAt: "u32",
        msg: "Bytes",
    },
    /** Lookup176: polkadot_core_primitives::InboundHrmpMessage<BlockNumber> */
    PolkadotCorePrimitivesInboundHrmpMessage: {
        sentAt: "u32",
        data: "Bytes",
    },
    /** Lookup179: cumulus_pallet_parachain_system::pallet::Error<T> */
    CumulusPalletParachainSystemError: {
        _enum: [
            "OverlappingUpgrades",
            "ProhibitedByPolkadot",
            "TooBig",
            "ValidationDataNotAvailable",
            "HostConfigurationNotAvailable",
            "NotScheduled",
            "NothingAuthorized",
            "Unauthorized",
        ],
    },
    /** Lookup180: pallet_timestamp::pallet::Call<T> */
    PalletTimestampCall: {
        _enum: {
            set: {
                now: "Compact<u64>",
            },
        },
    },
    /** Lookup181: parachain_info::pallet::Call<T> */
    ParachainInfoCall: "Null",
    /** Lookup182: pallet_sudo::pallet::Call<T> */
    PalletSudoCall: {
        _enum: {
            sudo: {
                call: "Call",
            },
            sudo_unchecked_weight: {
                call: "Call",
                weight: "SpWeightsWeightV2Weight",
            },
            set_key: {
                _alias: {
                    new_: "new",
                },
                new_: "MultiAddress",
            },
            sudo_as: {
                who: "MultiAddress",
                call: "Call",
            },
        },
    },
    /** Lookup184: pallet_utility::pallet::Call<T> */
    PalletUtilityCall: {
        _enum: {
            batch: {
                calls: "Vec<Call>",
            },
            as_derivative: {
                index: "u16",
                call: "Call",
            },
            batch_all: {
                calls: "Vec<Call>",
            },
            dispatch_as: {
                asOrigin: "DanceboxRuntimeOriginCaller",
                call: "Call",
            },
            force_batch: {
                calls: "Vec<Call>",
            },
            with_weight: {
                call: "Call",
                weight: "SpWeightsWeightV2Weight",
            },
        },
    },
    /** Lookup186: dancebox_runtime::OriginCaller */
    DanceboxRuntimeOriginCaller: {
        _enum: {
            system: "FrameSupportDispatchRawOrigin",
            __Unused1: "Null",
            __Unused2: "Null",
            Void: "SpCoreVoid",
            __Unused4: "Null",
            __Unused5: "Null",
            __Unused6: "Null",
            __Unused7: "Null",
            __Unused8: "Null",
            __Unused9: "Null",
            __Unused10: "Null",
            __Unused11: "Null",
            __Unused12: "Null",
            __Unused13: "Null",
            __Unused14: "Null",
            __Unused15: "Null",
            __Unused16: "Null",
            __Unused17: "Null",
            __Unused18: "Null",
            __Unused19: "Null",
            __Unused20: "Null",
            __Unused21: "Null",
            __Unused22: "Null",
            __Unused23: "Null",
            __Unused24: "Null",
            __Unused25: "Null",
            __Unused26: "Null",
            __Unused27: "Null",
            __Unused28: "Null",
            __Unused29: "Null",
            __Unused30: "Null",
            __Unused31: "Null",
            __Unused32: "Null",
            __Unused33: "Null",
            __Unused34: "Null",
            __Unused35: "Null",
            __Unused36: "Null",
            __Unused37: "Null",
            __Unused38: "Null",
            __Unused39: "Null",
            __Unused40: "Null",
            __Unused41: "Null",
            __Unused42: "Null",
            __Unused43: "Null",
            __Unused44: "Null",
            __Unused45: "Null",
            __Unused46: "Null",
            __Unused47: "Null",
            __Unused48: "Null",
            __Unused49: "Null",
            __Unused50: "Null",
            CumulusXcm: "CumulusPalletXcmOrigin",
            __Unused52: "Null",
            PolkadotXcm: "PalletXcmOrigin",
        },
    },
    /** Lookup187: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32> */
    FrameSupportDispatchRawOrigin: {
        _enum: {
            Root: "Null",
            Signed: "AccountId32",
            None: "Null",
        },
    },
    /** Lookup188: cumulus_pallet_xcm::pallet::Origin */
    CumulusPalletXcmOrigin: {
        _enum: {
            Relay: "Null",
            SiblingParachain: "u32",
        },
    },
    /** Lookup189: pallet_xcm::pallet::Origin */
    PalletXcmOrigin: {
        _enum: {
            Xcm: "StagingXcmV3MultiLocation",
            Response: "StagingXcmV3MultiLocation",
        },
    },
    /** Lookup190: sp_core::Void */
    SpCoreVoid: "Null",
    /** Lookup191: pallet_proxy::pallet::Call<T> */
    PalletProxyCall: {
        _enum: {
            proxy: {
                real: "MultiAddress",
                forceProxyType: "Option<DanceboxRuntimeProxyType>",
                call: "Call",
            },
            add_proxy: {
                delegate: "MultiAddress",
                proxyType: "DanceboxRuntimeProxyType",
                delay: "u32",
            },
            remove_proxy: {
                delegate: "MultiAddress",
                proxyType: "DanceboxRuntimeProxyType",
                delay: "u32",
            },
            remove_proxies: "Null",
            create_pure: {
                proxyType: "DanceboxRuntimeProxyType",
                delay: "u32",
                index: "u16",
            },
            kill_pure: {
                spawner: "MultiAddress",
                proxyType: "DanceboxRuntimeProxyType",
                index: "u16",
                height: "Compact<u32>",
                extIndex: "Compact<u32>",
            },
            announce: {
                real: "MultiAddress",
                callHash: "H256",
            },
            remove_announcement: {
                real: "MultiAddress",
                callHash: "H256",
            },
            reject_announcement: {
                delegate: "MultiAddress",
                callHash: "H256",
            },
            proxy_announced: {
                delegate: "MultiAddress",
                real: "MultiAddress",
                forceProxyType: "Option<DanceboxRuntimeProxyType>",
                call: "Call",
            },
        },
    },
    /** Lookup195: pallet_maintenance_mode::pallet::Call<T> */
    PalletMaintenanceModeCall: {
        _enum: ["enter_maintenance_mode", "resume_normal_operation"],
    },
    /** Lookup196: pallet_balances::pallet::Call<T, I> */
    PalletBalancesCall: {
        _enum: {
            transfer_allow_death: {
                dest: "MultiAddress",
                value: "Compact<u128>",
            },
            set_balance_deprecated: {
                who: "MultiAddress",
                newFree: "Compact<u128>",
                oldReserved: "Compact<u128>",
            },
            force_transfer: {
                source: "MultiAddress",
                dest: "MultiAddress",
                value: "Compact<u128>",
            },
            transfer_keep_alive: {
                dest: "MultiAddress",
                value: "Compact<u128>",
            },
            transfer_all: {
                dest: "MultiAddress",
                keepAlive: "bool",
            },
            force_unreserve: {
                who: "MultiAddress",
                amount: "u128",
            },
            upgrade_accounts: {
                who: "Vec<AccountId32>",
            },
            transfer: {
                dest: "MultiAddress",
                value: "Compact<u128>",
            },
            force_set_balance: {
                who: "MultiAddress",
                newFree: "Compact<u128>",
            },
        },
    },
    /** Lookup197: pallet_registrar::pallet::Call<T> */
    PalletRegistrarCall: {
        _enum: {
            register: {
                paraId: "u32",
                genesisData: "TpContainerChainGenesisDataContainerChainGenesisData",
            },
            deregister: {
                paraId: "u32",
            },
            mark_valid_for_collating: {
                paraId: "u32",
            },
            set_boot_nodes: {
                paraId: "u32",
                bootNodes: "Vec<Bytes>",
            },
            pause_container_chain: {
                paraId: "u32",
            },
        },
    },
    /** Lookup198: tp_container_chain_genesis_data::ContainerChainGenesisData<MaxLengthTokenSymbol> */
    TpContainerChainGenesisDataContainerChainGenesisData: {
        storage: "Vec<TpContainerChainGenesisDataContainerChainGenesisDataItem>",
        name: "Bytes",
        id: "Bytes",
        forkId: "Option<Bytes>",
        extensions: "Bytes",
        properties: "TpContainerChainGenesisDataProperties",
    },
    /** Lookup200: tp_container_chain_genesis_data::ContainerChainGenesisDataItem */
    TpContainerChainGenesisDataContainerChainGenesisDataItem: {
        key: "Bytes",
        value: "Bytes",
    },
    /** Lookup202: tp_container_chain_genesis_data::Properties<MaxLengthTokenSymbol> */
    TpContainerChainGenesisDataProperties: {
        tokenMetadata: "TpContainerChainGenesisDataTokenMetadata",
        isEthereum: "bool",
    },
    /** Lookup203: tp_container_chain_genesis_data::TokenMetadata<MaxLengthTokenSymbol> */
    TpContainerChainGenesisDataTokenMetadata: {
        tokenSymbol: "Bytes",
        ss58Format: "u32",
        tokenDecimals: "u32",
    },
    /** Lookup208: pallet_configuration::pallet::Call<T> */
    PalletConfigurationCall: {
        _enum: {
            set_max_collators: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_min_orchestrator_collators: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_max_orchestrator_collators: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_collators_per_container: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_full_rotation_period: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            __Unused5: "Null",
            __Unused6: "Null",
            __Unused7: "Null",
            __Unused8: "Null",
            __Unused9: "Null",
            __Unused10: "Null",
            __Unused11: "Null",
            __Unused12: "Null",
            __Unused13: "Null",
            __Unused14: "Null",
            __Unused15: "Null",
            __Unused16: "Null",
            __Unused17: "Null",
            __Unused18: "Null",
            __Unused19: "Null",
            __Unused20: "Null",
            __Unused21: "Null",
            __Unused22: "Null",
            __Unused23: "Null",
            __Unused24: "Null",
            __Unused25: "Null",
            __Unused26: "Null",
            __Unused27: "Null",
            __Unused28: "Null",
            __Unused29: "Null",
            __Unused30: "Null",
            __Unused31: "Null",
            __Unused32: "Null",
            __Unused33: "Null",
            __Unused34: "Null",
            __Unused35: "Null",
            __Unused36: "Null",
            __Unused37: "Null",
            __Unused38: "Null",
            __Unused39: "Null",
            __Unused40: "Null",
            __Unused41: "Null",
            __Unused42: "Null",
            __Unused43: "Null",
            set_bypass_consistency_check: {
                _alias: {
                    new_: "new",
                },
                new_: "bool",
            },
        },
    },
    /** Lookup209: pallet_collator_assignment::pallet::Call<T> */
    PalletCollatorAssignmentCall: "Null",
    /** Lookup210: pallet_author_noting::pallet::Call<T> */
    PalletAuthorNotingCall: {
        _enum: {
            set_latest_author_data: {
                data: "TpAuthorNotingInherentOwnParachainInherentData",
            },
            set_author: {
                paraId: "u32",
                blockNumber: "u32",
                author: "AccountId32",
            },
            kill_author_data: {
                paraId: "u32",
            },
        },
    },
    /** Lookup211: tp_author_noting_inherent::OwnParachainInherentData */
    TpAuthorNotingInherentOwnParachainInherentData: {
        relayStorageProof: "SpTrieStorageProof",
    },
    /** Lookup212: pallet_authority_assignment::pallet::Call<T> */
    PalletAuthorityAssignmentCall: "Null",
    /** Lookup213: pallet_services_payment::pallet::Call<T> */
    PalletServicesPaymentCall: {
        _enum: {
            purchase_credits: {
                paraId: "u32",
                credits: "u32",
                maxPricePerCredit: "Option<u128>",
            },
            set_credits: {
                paraId: "u32",
                credits: "u32",
            },
        },
    },
    /** Lookup215: pallet_invulnerables::pallet::Call<T> */
    PalletInvulnerablesCall: {
        _enum: {
            set_invulnerables: {
                _alias: {
                    new_: "new",
                },
                new_: "Vec<AccountId32>",
            },
            add_invulnerable: {
                who: "AccountId32",
            },
            remove_invulnerable: {
                who: "AccountId32",
            },
        },
    },
    /** Lookup216: pallet_session::pallet::Call<T> */
    PalletSessionCall: {
        _enum: {
            set_keys: {
                _alias: {
                    keys_: "keys",
                },
                keys_: "DanceboxRuntimeSessionKeys",
                proof: "Bytes",
            },
            purge_keys: "Null",
        },
    },
    /** Lookup217: dancebox_runtime::SessionKeys */
    DanceboxRuntimeSessionKeys: {
        nimbus: "NimbusPrimitivesNimbusCryptoPublic",
    },
    /** Lookup218: nimbus_primitives::nimbus_crypto::Public */
    NimbusPrimitivesNimbusCryptoPublic: "SpCoreSr25519Public",
    /** Lookup219: sp_core::sr25519::Public */
    SpCoreSr25519Public: "[u8;32]",
    /** Lookup220: pallet_author_inherent::pallet::Call<T> */
    PalletAuthorInherentCall: {
        _enum: ["kick_off_authorship_validation"],
    },
    /** Lookup221: pallet_pooled_staking::pallet::Call<T> */
    PalletPooledStakingCall: {
        _enum: {
            rebalance_hold: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                pool: "PalletPooledStakingAllTargetPool",
            },
            request_delegate: {
                candidate: "AccountId32",
                pool: "PalletPooledStakingTargetPool",
                stake: "u128",
            },
            execute_pending_operations: {
                operations: "Vec<PalletPooledStakingPendingOperationQuery>",
            },
            request_undelegate: {
                candidate: "AccountId32",
                pool: "PalletPooledStakingTargetPool",
                amount: "PalletPooledStakingSharesOrStake",
            },
            claim_manual_rewards: {
                pairs: "Vec<(AccountId32,AccountId32)>",
            },
            update_candidate_position: {
                candidates: "Vec<AccountId32>",
            },
            swap_pool: {
                candidate: "AccountId32",
                sourcePool: "PalletPooledStakingTargetPool",
                amount: "PalletPooledStakingSharesOrStake",
            },
        },
    },
    /** Lookup222: pallet_pooled_staking::pallet::AllTargetPool */
    PalletPooledStakingAllTargetPool: {
        _enum: ["Joining", "AutoCompounding", "ManualRewards", "Leaving"],
    },
    /** Lookup224: pallet_pooled_staking::pallet::PendingOperationQuery<sp_core::crypto::AccountId32, J, L> */
    PalletPooledStakingPendingOperationQuery: {
        delegator: "AccountId32",
        operation: "PalletPooledStakingPendingOperationKey",
    },
    /** Lookup225: pallet_pooled_staking::pallet::PendingOperationKey<sp_core::crypto::AccountId32, J, L> */
    PalletPooledStakingPendingOperationKey: {
        _enum: {
            JoiningAutoCompounding: {
                candidate: "AccountId32",
                at: "u32",
            },
            JoiningManualRewards: {
                candidate: "AccountId32",
                at: "u32",
            },
            Leaving: {
                candidate: "AccountId32",
                at: "u32",
            },
        },
    },
    /** Lookup226: pallet_pooled_staking::pallet::SharesOrStake<T> */
    PalletPooledStakingSharesOrStake: {
        _enum: {
            Shares: "u128",
            Stake: "u128",
        },
    },
    /** Lookup229: cumulus_pallet_xcmp_queue::pallet::Call<T> */
    CumulusPalletXcmpQueueCall: {
        _enum: {
            service_overweight: {
                index: "u64",
                weightLimit: "SpWeightsWeightV2Weight",
            },
            suspend_xcm_execution: "Null",
            resume_xcm_execution: "Null",
            update_suspend_threshold: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            update_drop_threshold: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            update_resume_threshold: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            update_threshold_weight: {
                _alias: {
                    new_: "new",
                },
                new_: "SpWeightsWeightV2Weight",
            },
            update_weight_restrict_decay: {
                _alias: {
                    new_: "new",
                },
                new_: "SpWeightsWeightV2Weight",
            },
            update_xcmp_max_individual_weight: {
                _alias: {
                    new_: "new",
                },
                new_: "SpWeightsWeightV2Weight",
            },
        },
    },
    /** Lookup230: cumulus_pallet_dmp_queue::pallet::Call<T> */
    CumulusPalletDmpQueueCall: {
        _enum: {
            service_overweight: {
                index: "u64",
                weightLimit: "SpWeightsWeightV2Weight",
            },
        },
    },
    /** Lookup231: pallet_xcm::pallet::Call<T> */
    PalletXcmCall: {
        _enum: {
            send: {
                dest: "StagingXcmVersionedMultiLocation",
                message: "StagingXcmVersionedXcm",
            },
            teleport_assets: {
                dest: "StagingXcmVersionedMultiLocation",
                beneficiary: "StagingXcmVersionedMultiLocation",
                assets: "StagingXcmVersionedMultiAssets",
                feeAssetItem: "u32",
            },
            reserve_transfer_assets: {
                dest: "StagingXcmVersionedMultiLocation",
                beneficiary: "StagingXcmVersionedMultiLocation",
                assets: "StagingXcmVersionedMultiAssets",
                feeAssetItem: "u32",
            },
            execute: {
                message: "StagingXcmVersionedXcm",
                maxWeight: "SpWeightsWeightV2Weight",
            },
            force_xcm_version: {
                location: "StagingXcmV3MultiLocation",
                version: "u32",
            },
            force_default_xcm_version: {
                maybeXcmVersion: "Option<u32>",
            },
            force_subscribe_version_notify: {
                location: "StagingXcmVersionedMultiLocation",
            },
            force_unsubscribe_version_notify: {
                location: "StagingXcmVersionedMultiLocation",
            },
            limited_reserve_transfer_assets: {
                dest: "StagingXcmVersionedMultiLocation",
                beneficiary: "StagingXcmVersionedMultiLocation",
                assets: "StagingXcmVersionedMultiAssets",
                feeAssetItem: "u32",
                weightLimit: "StagingXcmV3WeightLimit",
            },
            limited_teleport_assets: {
                dest: "StagingXcmVersionedMultiLocation",
                beneficiary: "StagingXcmVersionedMultiLocation",
                assets: "StagingXcmVersionedMultiAssets",
                feeAssetItem: "u32",
                weightLimit: "StagingXcmV3WeightLimit",
            },
            force_suspension: {
                suspended: "bool",
            },
        },
    },
    /** Lookup232: staging_xcm::VersionedXcm<RuntimeCall> */
    StagingXcmVersionedXcm: {
        _enum: {
            __Unused0: "Null",
            __Unused1: "Null",
            V2: "StagingXcmV2Xcm",
            V3: "StagingXcmV3Xcm",
        },
    },
    /** Lookup233: staging_xcm::v2::Xcm<RuntimeCall> */
    StagingXcmV2Xcm: "Vec<StagingXcmV2Instruction>",
    /** Lookup235: staging_xcm::v2::Instruction<RuntimeCall> */
    StagingXcmV2Instruction: {
        _enum: {
            WithdrawAsset: "StagingXcmV2MultiassetMultiAssets",
            ReserveAssetDeposited: "StagingXcmV2MultiassetMultiAssets",
            ReceiveTeleportedAsset: "StagingXcmV2MultiassetMultiAssets",
            QueryResponse: {
                queryId: "Compact<u64>",
                response: "StagingXcmV2Response",
                maxWeight: "Compact<u64>",
            },
            TransferAsset: {
                assets: "StagingXcmV2MultiassetMultiAssets",
                beneficiary: "StagingXcmV2MultiLocation",
            },
            TransferReserveAsset: {
                assets: "StagingXcmV2MultiassetMultiAssets",
                dest: "StagingXcmV2MultiLocation",
                xcm: "StagingXcmV2Xcm",
            },
            Transact: {
                originType: "StagingXcmV2OriginKind",
                requireWeightAtMost: "Compact<u64>",
                call: "StagingXcmDoubleEncoded",
            },
            HrmpNewChannelOpenRequest: {
                sender: "Compact<u32>",
                maxMessageSize: "Compact<u32>",
                maxCapacity: "Compact<u32>",
            },
            HrmpChannelAccepted: {
                recipient: "Compact<u32>",
            },
            HrmpChannelClosing: {
                initiator: "Compact<u32>",
                sender: "Compact<u32>",
                recipient: "Compact<u32>",
            },
            ClearOrigin: "Null",
            DescendOrigin: "StagingXcmV2MultilocationJunctions",
            ReportError: {
                queryId: "Compact<u64>",
                dest: "StagingXcmV2MultiLocation",
                maxResponseWeight: "Compact<u64>",
            },
            DepositAsset: {
                assets: "StagingXcmV2MultiassetMultiAssetFilter",
                maxAssets: "Compact<u32>",
                beneficiary: "StagingXcmV2MultiLocation",
            },
            DepositReserveAsset: {
                assets: "StagingXcmV2MultiassetMultiAssetFilter",
                maxAssets: "Compact<u32>",
                dest: "StagingXcmV2MultiLocation",
                xcm: "StagingXcmV2Xcm",
            },
            ExchangeAsset: {
                give: "StagingXcmV2MultiassetMultiAssetFilter",
                receive: "StagingXcmV2MultiassetMultiAssets",
            },
            InitiateReserveWithdraw: {
                assets: "StagingXcmV2MultiassetMultiAssetFilter",
                reserve: "StagingXcmV2MultiLocation",
                xcm: "StagingXcmV2Xcm",
            },
            InitiateTeleport: {
                assets: "StagingXcmV2MultiassetMultiAssetFilter",
                dest: "StagingXcmV2MultiLocation",
                xcm: "StagingXcmV2Xcm",
            },
            QueryHolding: {
                queryId: "Compact<u64>",
                dest: "StagingXcmV2MultiLocation",
                assets: "StagingXcmV2MultiassetMultiAssetFilter",
                maxResponseWeight: "Compact<u64>",
            },
            BuyExecution: {
                fees: "StagingXcmV2MultiAsset",
                weightLimit: "StagingXcmV2WeightLimit",
            },
            RefundSurplus: "Null",
            SetErrorHandler: "StagingXcmV2Xcm",
            SetAppendix: "StagingXcmV2Xcm",
            ClearError: "Null",
            ClaimAsset: {
                assets: "StagingXcmV2MultiassetMultiAssets",
                ticket: "StagingXcmV2MultiLocation",
            },
            Trap: "Compact<u64>",
            SubscribeVersion: {
                queryId: "Compact<u64>",
                maxResponseWeight: "Compact<u64>",
            },
            UnsubscribeVersion: "Null",
        },
    },
    /** Lookup236: staging_xcm::v2::Response */
    StagingXcmV2Response: {
        _enum: {
            Null: "Null",
            Assets: "StagingXcmV2MultiassetMultiAssets",
            ExecutionResult: "Option<(u32,StagingXcmV2TraitsError)>",
            Version: "u32",
        },
    },
    /** Lookup239: staging_xcm::v2::traits::Error */
    StagingXcmV2TraitsError: {
        _enum: {
            Overflow: "Null",
            Unimplemented: "Null",
            UntrustedReserveLocation: "Null",
            UntrustedTeleportLocation: "Null",
            MultiLocationFull: "Null",
            MultiLocationNotInvertible: "Null",
            BadOrigin: "Null",
            InvalidLocation: "Null",
            AssetNotFound: "Null",
            FailedToTransactAsset: "Null",
            NotWithdrawable: "Null",
            LocationCannotHold: "Null",
            ExceedsMaxMessageSize: "Null",
            DestinationUnsupported: "Null",
            Transport: "Null",
            Unroutable: "Null",
            UnknownClaim: "Null",
            FailedToDecode: "Null",
            MaxWeightInvalid: "Null",
            NotHoldingFees: "Null",
            TooExpensive: "Null",
            Trap: "u64",
            UnhandledXcmVersion: "Null",
            WeightLimitReached: "u64",
            Barrier: "Null",
            WeightNotComputable: "Null",
        },
    },
    /** Lookup240: staging_xcm::v2::multiasset::MultiAssetFilter */
    StagingXcmV2MultiassetMultiAssetFilter: {
        _enum: {
            Definite: "StagingXcmV2MultiassetMultiAssets",
            Wild: "StagingXcmV2MultiassetWildMultiAsset",
        },
    },
    /** Lookup241: staging_xcm::v2::multiasset::WildMultiAsset */
    StagingXcmV2MultiassetWildMultiAsset: {
        _enum: {
            All: "Null",
            AllOf: {
                id: "StagingXcmV2MultiassetAssetId",
                fun: "StagingXcmV2MultiassetWildFungibility",
            },
        },
    },
    /** Lookup242: staging_xcm::v2::multiasset::WildFungibility */
    StagingXcmV2MultiassetWildFungibility: {
        _enum: ["Fungible", "NonFungible"],
    },
    /** Lookup243: staging_xcm::v2::WeightLimit */
    StagingXcmV2WeightLimit: {
        _enum: {
            Unlimited: "Null",
            Limited: "Compact<u64>",
        },
    },
    /** Lookup252: pallet_root_testing::pallet::Call<T> */
    PalletRootTestingCall: {
        _enum: {
            fill_block: {
                ratio: "Perbill",
            },
        },
    },
    /** Lookup254: pallet_sudo::pallet::Error<T> */
    PalletSudoError: {
        _enum: ["RequireSudo"],
    },
    /** Lookup255: pallet_utility::pallet::Error<T> */
    PalletUtilityError: {
        _enum: ["TooManyCalls"],
    },
    /** Lookup258: pallet_proxy::ProxyDefinition<sp_core::crypto::AccountId32, dancebox_runtime::ProxyType, BlockNumber> */
    PalletProxyProxyDefinition: {
        delegate: "AccountId32",
        proxyType: "DanceboxRuntimeProxyType",
        delay: "u32",
    },
    /** Lookup262: pallet_proxy::Announcement<sp_core::crypto::AccountId32, primitive_types::H256, BlockNumber> */
    PalletProxyAnnouncement: {
        real: "AccountId32",
        callHash: "H256",
        height: "u32",
    },
    /** Lookup264: pallet_proxy::pallet::Error<T> */
    PalletProxyError: {
        _enum: [
            "TooMany",
            "NotFound",
            "NotProxy",
            "Unproxyable",
            "Duplicate",
            "NoPermission",
            "Unannounced",
            "NoSelfProxy",
        ],
    },
    /** Lookup265: pallet_migrations::pallet::Error<T> */
    PalletMigrationsError: {
        _enum: ["PreimageMissing", "WrongUpperBound", "PreimageIsTooBig", "PreimageAlreadyExists"],
    },
    /** Lookup266: pallet_maintenance_mode::pallet::Error<T> */
    PalletMaintenanceModeError: {
        _enum: ["AlreadyInMaintenanceMode", "NotInMaintenanceMode"],
    },
    /** Lookup268: pallet_balances::types::BalanceLock<Balance> */
    PalletBalancesBalanceLock: {
        id: "[u8;8]",
        amount: "u128",
        reasons: "PalletBalancesReasons",
    },
    /** Lookup269: pallet_balances::types::Reasons */
    PalletBalancesReasons: {
        _enum: ["Fee", "Misc", "All"],
    },
    /** Lookup272: pallet_balances::types::ReserveData<ReserveIdentifier, Balance> */
    PalletBalancesReserveData: {
        id: "[u8;8]",
        amount: "u128",
    },
    /** Lookup276: dancebox_runtime::HoldReason */
    DanceboxRuntimeHoldReason: {
        _enum: ["PooledStake"],
    },
    /** Lookup279: pallet_balances::types::IdAmount<Id, Balance> */
    PalletBalancesIdAmount: {
        id: "[u8;8]",
        amount: "u128",
    },
    /** Lookup281: pallet_balances::pallet::Error<T, I> */
    PalletBalancesError: {
        _enum: [
            "VestingBalance",
            "LiquidityRestrictions",
            "InsufficientBalance",
            "ExistentialDeposit",
            "Expendability",
            "ExistingVestingSchedule",
            "DeadAccount",
            "TooManyReserves",
            "TooManyHolds",
            "TooManyFreezes",
        ],
    },
    /** Lookup283: pallet_transaction_payment::Releases */
    PalletTransactionPaymentReleases: {
        _enum: ["V1Ancient", "V2"],
    },
    /** Lookup288: pallet_registrar::pallet::DepositInfo<T> */
    PalletRegistrarDepositInfo: {
        creator: "AccountId32",
        deposit: "u128",
    },
    /** Lookup289: pallet_registrar::pallet::Error<T> */
    PalletRegistrarError: {
        _enum: [
            "ParaIdAlreadyRegistered",
            "ParaIdAlreadyPaused",
            "ParaIdNotRegistered",
            "ParaIdListFull",
            "GenesisDataTooBig",
            "ParaIdNotInPendingVerification",
            "NotSufficientDeposit",
        ],
    },
    /** Lookup290: pallet_configuration::HostConfiguration */
    PalletConfigurationHostConfiguration: {
        maxCollators: "u32",
        minOrchestratorCollators: "u32",
        maxOrchestratorCollators: "u32",
        collatorsPerContainer: "u32",
        fullRotationPeriod: "u32",
    },
    /** Lookup293: pallet_configuration::pallet::Error<T> */
    PalletConfigurationError: {
        _enum: ["InvalidNewValue"],
    },
    /** Lookup294: dp_collator_assignment::AssignedCollators<sp_core::crypto::AccountId32> */
    DpCollatorAssignmentAssignedCollatorsAccountId32: {
        orchestratorChain: "Vec<AccountId32>",
        containerChains: "BTreeMap<u32, Vec<AccountId32>>",
    },
    /** Lookup299: pallet_author_noting::pallet::ContainerChainBlockInfo<T> */
    PalletAuthorNotingContainerChainBlockInfo: {
        blockNumber: "u32",
        author: "AccountId32",
    },
    /** Lookup300: pallet_author_noting::pallet::Error<T> */
    PalletAuthorNotingError: {
        _enum: [
            "FailedReading",
            "FailedDecodingHeader",
            "AuraDigestFirstItem",
            "AsPreRuntimeError",
            "NonDecodableSlot",
            "AuthorNotFound",
            "NonAuraDigest",
        ],
    },
    /** Lookup301: dp_collator_assignment::AssignedCollators<nimbus_primitives::nimbus_crypto::Public> */
    DpCollatorAssignmentAssignedCollatorsPublic: {
        orchestratorChain: "Vec<NimbusPrimitivesNimbusCryptoPublic>",
        containerChains: "BTreeMap<u32, Vec<NimbusPrimitivesNimbusCryptoPublic>>",
    },
    /** Lookup306: pallet_services_payment::pallet::Error<T> */
    PalletServicesPaymentError: {
        _enum: ["InsufficientFundsToPurchaseCredits", "InsufficientCredits", "CreditPriceTooExpensive"],
    },
    /** Lookup308: pallet_invulnerables::pallet::Error<T> */
    PalletInvulnerablesError: {
        _enum: ["TooManyInvulnerables", "AlreadyInvulnerable", "NotInvulnerable"],
    },
    /** Lookup313: sp_core::crypto::KeyTypeId */
    SpCoreCryptoKeyTypeId: "[u8;4]",
    /** Lookup314: pallet_session::pallet::Error<T> */
    PalletSessionError: {
        _enum: ["InvalidProof", "NoAssociatedValidatorId", "DuplicatedKey", "NoKeys", "NoAccount"],
    },
    /** Lookup318: pallet_author_inherent::pallet::Error<T> */
    PalletAuthorInherentError: {
        _enum: ["AuthorAlreadySet", "NoAccountId", "CannotBeAuthor"],
    },
    /** Lookup320: pallet_pooled_staking::candidate::EligibleCandidate<sp_core::crypto::AccountId32, S> */
    PalletPooledStakingCandidateEligibleCandidate: {
        candidate: "AccountId32",
        stake: "u128",
    },
    /** Lookup323: pallet_pooled_staking::pallet::PoolsKey<sp_core::crypto::AccountId32> */
    PalletPooledStakingPoolsKey: {
        _enum: {
            CandidateTotalStake: "Null",
            JoiningShares: {
                delegator: "AccountId32",
            },
            JoiningSharesSupply: "Null",
            JoiningSharesTotalStaked: "Null",
            JoiningSharesHeldStake: {
                delegator: "AccountId32",
            },
            AutoCompoundingShares: {
                delegator: "AccountId32",
            },
            AutoCompoundingSharesSupply: "Null",
            AutoCompoundingSharesTotalStaked: "Null",
            AutoCompoundingSharesHeldStake: {
                delegator: "AccountId32",
            },
            ManualRewardsShares: {
                delegator: "AccountId32",
            },
            ManualRewardsSharesSupply: "Null",
            ManualRewardsSharesTotalStaked: "Null",
            ManualRewardsSharesHeldStake: {
                delegator: "AccountId32",
            },
            ManualRewardsCounter: "Null",
            ManualRewardsCheckpoint: {
                delegator: "AccountId32",
            },
            LeavingShares: {
                delegator: "AccountId32",
            },
            LeavingSharesSupply: "Null",
            LeavingSharesTotalStaked: "Null",
            LeavingSharesHeldStake: {
                delegator: "AccountId32",
            },
        },
    },
    /** Lookup325: pallet_pooled_staking::pallet::Error<T> */
    PalletPooledStakingError: {
        _enum: {
            InvalidPalletSetting: "Null",
            DisabledFeature: "Null",
            NoOneIsStaking: "Null",
            StakeMustBeNonZero: "Null",
            RewardsMustBeNonZero: "Null",
            MathUnderflow: "Null",
            MathOverflow: "Null",
            NotEnoughShares: "Null",
            TryingToLeaveTooSoon: "Null",
            InconsistentState: "Null",
            UnsufficientSharesForTransfer: "Null",
            CandidateTransferingOwnSharesForbidden: "Null",
            RequestCannotBeExecuted: "u16",
            SwapResultsInZeroShares: "Null",
        },
    },
    /** Lookup326: pallet_inflation_rewards::pallet::ChainsToRewardValue<T> */
    PalletInflationRewardsChainsToRewardValue: {
        paraIds: "Vec<u32>",
        rewardsPerChain: "u128",
    },
    /** Lookup328: cumulus_pallet_xcmp_queue::InboundChannelDetails */
    CumulusPalletXcmpQueueInboundChannelDetails: {
        sender: "u32",
        state: "CumulusPalletXcmpQueueInboundState",
        messageMetadata: "Vec<(u32,PolkadotParachainPrimitivesPrimitivesXcmpMessageFormat)>",
    },
    /** Lookup329: cumulus_pallet_xcmp_queue::InboundState */
    CumulusPalletXcmpQueueInboundState: {
        _enum: ["Ok", "Suspended"],
    },
    /** Lookup332: polkadot_parachain_primitives::primitives::XcmpMessageFormat */
    PolkadotParachainPrimitivesPrimitivesXcmpMessageFormat: {
        _enum: ["ConcatenatedVersionedXcm", "ConcatenatedEncodedBlob", "Signals"],
    },
    /** Lookup335: cumulus_pallet_xcmp_queue::OutboundChannelDetails */
    CumulusPalletXcmpQueueOutboundChannelDetails: {
        recipient: "u32",
        state: "CumulusPalletXcmpQueueOutboundState",
        signalsExist: "bool",
        firstIndex: "u16",
        lastIndex: "u16",
    },
    /** Lookup336: cumulus_pallet_xcmp_queue::OutboundState */
    CumulusPalletXcmpQueueOutboundState: {
        _enum: ["Ok", "Suspended"],
    },
    /** Lookup338: cumulus_pallet_xcmp_queue::QueueConfigData */
    CumulusPalletXcmpQueueQueueConfigData: {
        suspendThreshold: "u32",
        dropThreshold: "u32",
        resumeThreshold: "u32",
        thresholdWeight: "SpWeightsWeightV2Weight",
        weightRestrictDecay: "SpWeightsWeightV2Weight",
        xcmpMaxIndividualWeight: "SpWeightsWeightV2Weight",
    },
    /** Lookup340: cumulus_pallet_xcmp_queue::pallet::Error<T> */
    CumulusPalletXcmpQueueError: {
        _enum: ["FailedToSend", "BadXcmOrigin", "BadXcm", "BadOverweightIndex", "WeightOverLimit"],
    },
    /** Lookup341: cumulus_pallet_xcm::pallet::Error<T> */
    CumulusPalletXcmError: "Null",
    /** Lookup342: cumulus_pallet_dmp_queue::ConfigData */
    CumulusPalletDmpQueueConfigData: {
        maxIndividual: "SpWeightsWeightV2Weight",
    },
    /** Lookup343: cumulus_pallet_dmp_queue::PageIndexData */
    CumulusPalletDmpQueuePageIndexData: {
        beginUsed: "u32",
        endUsed: "u32",
        overweightCount: "u64",
    },
    /** Lookup346: cumulus_pallet_dmp_queue::pallet::Error<T> */
    CumulusPalletDmpQueueError: {
        _enum: ["Unknown", "OverLimit"],
    },
    /** Lookup347: pallet_xcm::pallet::QueryStatus<BlockNumber> */
    PalletXcmQueryStatus: {
        _enum: {
            Pending: {
                responder: "StagingXcmVersionedMultiLocation",
                maybeMatchQuerier: "Option<StagingXcmVersionedMultiLocation>",
                maybeNotify: "Option<(u8,u8)>",
                timeout: "u32",
            },
            VersionNotifier: {
                origin: "StagingXcmVersionedMultiLocation",
                isActive: "bool",
            },
            Ready: {
                response: "StagingXcmVersionedResponse",
                at: "u32",
            },
        },
    },
    /** Lookup351: staging_xcm::VersionedResponse */
    StagingXcmVersionedResponse: {
        _enum: {
            __Unused0: "Null",
            __Unused1: "Null",
            V2: "StagingXcmV2Response",
            V3: "StagingXcmV3Response",
        },
    },
    /** Lookup357: pallet_xcm::pallet::VersionMigrationStage */
    PalletXcmVersionMigrationStage: {
        _enum: {
            MigrateSupportedVersion: "Null",
            MigrateVersionNotifiers: "Null",
            NotifyCurrentTargets: "Option<Bytes>",
            MigrateAndNotifyOldTargets: "Null",
        },
    },
    /** Lookup359: staging_xcm::VersionedAssetId */
    StagingXcmVersionedAssetId: {
        _enum: {
            __Unused0: "Null",
            __Unused1: "Null",
            __Unused2: "Null",
            V3: "StagingXcmV3MultiassetAssetId",
        },
    },
    /** Lookup360: pallet_xcm::pallet::RemoteLockedFungibleRecord<ConsumerIdentifier, MaxConsumers> */
    PalletXcmRemoteLockedFungibleRecord: {
        amount: "u128",
        owner: "StagingXcmVersionedMultiLocation",
        locker: "StagingXcmVersionedMultiLocation",
        consumers: "Vec<(Null,u128)>",
    },
    /** Lookup367: pallet_xcm::pallet::Error<T> */
    PalletXcmError: {
        _enum: [
            "Unreachable",
            "SendFailure",
            "Filtered",
            "UnweighableMessage",
            "DestinationNotInvertible",
            "Empty",
            "CannotReanchor",
            "TooManyAssets",
            "InvalidOrigin",
            "BadVersion",
            "BadLocation",
            "NoSubscription",
            "AlreadySubscribed",
            "InvalidAsset",
            "LowBalance",
            "TooManyLocks",
            "AccountNotSovereign",
            "FeesNotMet",
            "LockNotFound",
            "InUse",
        ],
    },
    /** Lookup369: sp_runtime::MultiSignature */
    SpRuntimeMultiSignature: {
        _enum: {
            Ed25519: "SpCoreEd25519Signature",
            Sr25519: "SpCoreSr25519Signature",
            Ecdsa: "SpCoreEcdsaSignature",
        },
    },
    /** Lookup370: sp_core::ed25519::Signature */
    SpCoreEd25519Signature: "[u8;64]",
    /** Lookup372: sp_core::sr25519::Signature */
    SpCoreSr25519Signature: "[u8;64]",
    /** Lookup373: sp_core::ecdsa::Signature */
    SpCoreEcdsaSignature: "[u8;65]",
    /** Lookup376: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T> */
    FrameSystemExtensionsCheckNonZeroSender: "Null",
    /** Lookup377: frame_system::extensions::check_spec_version::CheckSpecVersion<T> */
    FrameSystemExtensionsCheckSpecVersion: "Null",
    /** Lookup378: frame_system::extensions::check_tx_version::CheckTxVersion<T> */
    FrameSystemExtensionsCheckTxVersion: "Null",
    /** Lookup379: frame_system::extensions::check_genesis::CheckGenesis<T> */
    FrameSystemExtensionsCheckGenesis: "Null",
    /** Lookup382: frame_system::extensions::check_nonce::CheckNonce<T> */
    FrameSystemExtensionsCheckNonce: "Compact<u32>",
    /** Lookup383: frame_system::extensions::check_weight::CheckWeight<T> */
    FrameSystemExtensionsCheckWeight: "Null",
    /** Lookup384: pallet_transaction_payment::ChargeTransactionPayment<T> */
    PalletTransactionPaymentChargeTransactionPayment: "Compact<u128>",
    /** Lookup385: dancebox_runtime::Runtime */
    DanceboxRuntimeRuntime: "Null",
};
