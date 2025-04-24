// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

/* eslint-disable sort-keys */

export default {
    /**
     * Lookup3: frame_system::AccountInfo<Nonce, pallet_balances::types::AccountData<Balance>>
     **/
    FrameSystemAccountInfo: {
        nonce: "u32",
        consumers: "u32",
        providers: "u32",
        sufficients: "u32",
        data: "PalletBalancesAccountData",
    },
    /**
     * Lookup5: pallet_balances::types::AccountData<Balance>
     **/
    PalletBalancesAccountData: {
        free: "u128",
        reserved: "u128",
        frozen: "u128",
        flags: "u128",
    },
    /**
     * Lookup9: frame_support::dispatch::PerDispatchClass<sp_weights::weight_v2::Weight>
     **/
    FrameSupportDispatchPerDispatchClassWeight: {
        normal: "SpWeightsWeightV2Weight",
        operational: "SpWeightsWeightV2Weight",
        mandatory: "SpWeightsWeightV2Weight",
    },
    /**
     * Lookup10: sp_weights::weight_v2::Weight
     **/
    SpWeightsWeightV2Weight: {
        refTime: "Compact<u64>",
        proofSize: "Compact<u64>",
    },
    /**
     * Lookup15: sp_runtime::generic::digest::Digest
     **/
    SpRuntimeDigest: {
        logs: "Vec<SpRuntimeDigestDigestItem>",
    },
    /**
     * Lookup17: sp_runtime::generic::digest::DigestItem
     **/
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
    /**
     * Lookup20: frame_system::EventRecord<dancebox_runtime::RuntimeEvent, primitive_types::H256>
     **/
    FrameSystemEventRecord: {
        phase: "FrameSystemPhase",
        event: "Event",
        topics: "Vec<H256>",
    },
    /**
     * Lookup22: frame_system::pallet::Event<T>
     **/
    FrameSystemEvent: {
        _enum: {
            ExtrinsicSuccess: {
                dispatchInfo: "FrameSystemDispatchEventInfo",
            },
            ExtrinsicFailed: {
                dispatchError: "SpRuntimeDispatchError",
                dispatchInfo: "FrameSystemDispatchEventInfo",
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
            UpgradeAuthorized: {
                codeHash: "H256",
                checkVersion: "bool",
            },
        },
    },
    /**
     * Lookup23: frame_system::DispatchEventInfo
     **/
    FrameSystemDispatchEventInfo: {
        weight: "SpWeightsWeightV2Weight",
        class: "FrameSupportDispatchDispatchClass",
        paysFee: "FrameSupportDispatchPays",
    },
    /**
     * Lookup24: frame_support::dispatch::DispatchClass
     **/
    FrameSupportDispatchDispatchClass: {
        _enum: ["Normal", "Operational", "Mandatory"],
    },
    /**
     * Lookup25: frame_support::dispatch::Pays
     **/
    FrameSupportDispatchPays: {
        _enum: ["Yes", "No"],
    },
    /**
     * Lookup26: sp_runtime::DispatchError
     **/
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
            Trie: "SpRuntimeProvingTrieTrieError",
        },
    },
    /**
     * Lookup27: sp_runtime::ModuleError
     **/
    SpRuntimeModuleError: {
        index: "u8",
        error: "[u8;4]",
    },
    /**
     * Lookup28: sp_runtime::TokenError
     **/
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
    /**
     * Lookup29: sp_arithmetic::ArithmeticError
     **/
    SpArithmeticArithmeticError: {
        _enum: ["Underflow", "Overflow", "DivisionByZero"],
    },
    /**
     * Lookup30: sp_runtime::TransactionalError
     **/
    SpRuntimeTransactionalError: {
        _enum: ["LimitReached", "NoLayer"],
    },
    /**
     * Lookup31: sp_runtime::proving_trie::TrieError
     **/
    SpRuntimeProvingTrieTrieError: {
        _enum: [
            "InvalidStateRoot",
            "IncompleteDatabase",
            "ValueAtIncompleteKey",
            "DecoderError",
            "InvalidHash",
            "DuplicateKey",
            "ExtraneousNode",
            "ExtraneousValue",
            "ExtraneousHashReference",
            "InvalidChildReference",
            "ValueMismatch",
            "IncompleteProof",
            "RootMismatch",
            "DecodeError",
        ],
    },
    /**
     * Lookup32: cumulus_pallet_parachain_system::pallet::Event<T>
     **/
    CumulusPalletParachainSystemEvent: {
        _enum: {
            ValidationFunctionStored: "Null",
            ValidationFunctionApplied: {
                relayChainBlockNum: "u32",
            },
            ValidationFunctionDiscarded: "Null",
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
    /**
     * Lookup34: pallet_sudo::pallet::Event<T>
     **/
    PalletSudoEvent: {
        _enum: {
            Sudid: {
                sudoResult: "Result<Null, SpRuntimeDispatchError>",
            },
            KeyChanged: {
                _alias: {
                    new_: "new",
                },
                old: "Option<AccountId32>",
                new_: "AccountId32",
            },
            KeyRemoved: "Null",
            SudoAsDone: {
                sudoResult: "Result<Null, SpRuntimeDispatchError>",
            },
        },
    },
    /**
     * Lookup38: pallet_utility::pallet::Event
     **/
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
    /**
     * Lookup39: pallet_proxy::pallet::Event<T>
     **/
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
    /**
     * Lookup40: dancebox_runtime::ProxyType
     **/
    DanceboxRuntimeProxyType: {
        _enum: [
            "Any",
            "NonTransfer",
            "Governance",
            "Staking",
            "CancelProxy",
            "Balances",
            "Registrar",
            "SudoRegistrar",
            "SessionKeyManagement",
        ],
    },
    /**
     * Lookup42: pallet_migrations::pallet::Event<T>
     **/
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
    /**
     * Lookup45: pallet_maintenance_mode::pallet::Event
     **/
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
    /**
     * Lookup46: pallet_tx_pause::pallet::Event<T>
     **/
    PalletTxPauseEvent: {
        _enum: {
            CallPaused: {
                fullName: "(Bytes,Bytes)",
            },
            CallUnpaused: {
                fullName: "(Bytes,Bytes)",
            },
        },
    },
    /**
     * Lookup49: pallet_balances::pallet::Event<T, I>
     **/
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
            TotalIssuanceForced: {
                _alias: {
                    new_: "new",
                },
                old: "u128",
                new_: "u128",
            },
        },
    },
    /**
     * Lookup50: frame_support::traits::tokens::misc::BalanceStatus
     **/
    FrameSupportTokensMiscBalanceStatus: {
        _enum: ["Free", "Reserved"],
    },
    /**
     * Lookup51: pallet_transaction_payment::pallet::Event<T>
     **/
    PalletTransactionPaymentEvent: {
        _enum: {
            TransactionFeePaid: {
                who: "AccountId32",
                actualFee: "u128",
                tip: "u128",
            },
        },
    },
    /**
     * Lookup52: pallet_stream_payment::pallet::Event<T>
     **/
    PalletStreamPaymentEvent: {
        _enum: {
            StreamOpened: {
                streamId: "u64",
            },
            StreamClosed: {
                streamId: "u64",
                refunded: "u128",
            },
            StreamPayment: {
                streamId: "u64",
                source: "AccountId32",
                target: "AccountId32",
                amount: "u128",
                stalled: "bool",
            },
            StreamConfigChangeRequested: {
                streamId: "u64",
                requestNonce: "u32",
                requester: "PalletStreamPaymentParty",
                oldConfig: "PalletStreamPaymentStreamConfig",
                newConfig: "PalletStreamPaymentStreamConfig",
            },
            StreamConfigChanged: {
                streamId: "u64",
                oldConfig: "PalletStreamPaymentStreamConfig",
                newConfig: "PalletStreamPaymentStreamConfig",
                depositChange: "Option<PalletStreamPaymentDepositChange>",
            },
        },
    },
    /**
     * Lookup53: pallet_stream_payment::pallet::Party
     **/
    PalletStreamPaymentParty: {
        _enum: ["Source", "Target"],
    },
    /**
     * Lookup54: pallet_stream_payment::pallet::StreamConfig<tp_stream_payment_common::TimeUnit, tp_stream_payment_common::AssetId, BalanceOrDuration>
     **/
    PalletStreamPaymentStreamConfig: {
        timeUnit: "TpStreamPaymentCommonTimeUnit",
        assetId: "TpStreamPaymentCommonAssetId",
        rate: "u128",
        minimumRequestDeadlineDelay: "u128",
        softMinimumDeposit: "u128",
    },
    /**
     * Lookup55: tp_stream_payment_common::TimeUnit
     **/
    TpStreamPaymentCommonTimeUnit: {
        _enum: ["BlockNumber", "Timestamp"],
    },
    /**
     * Lookup56: tp_stream_payment_common::AssetId
     **/
    TpStreamPaymentCommonAssetId: {
        _enum: ["Native"],
    },
    /**
     * Lookup58: pallet_stream_payment::pallet::DepositChange<Balance>
     **/
    PalletStreamPaymentDepositChange: {
        _enum: {
            Increase: "u128",
            Decrease: "u128",
            Absolute: "u128",
        },
    },
    /**
     * Lookup59: pallet_identity::pallet::Event<T>
     **/
    PalletIdentityEvent: {
        _enum: {
            IdentitySet: {
                who: "AccountId32",
            },
            IdentityCleared: {
                who: "AccountId32",
                deposit: "u128",
            },
            IdentityKilled: {
                who: "AccountId32",
                deposit: "u128",
            },
            JudgementRequested: {
                who: "AccountId32",
                registrarIndex: "u32",
            },
            JudgementUnrequested: {
                who: "AccountId32",
                registrarIndex: "u32",
            },
            JudgementGiven: {
                target: "AccountId32",
                registrarIndex: "u32",
            },
            RegistrarAdded: {
                registrarIndex: "u32",
            },
            SubIdentityAdded: {
                sub: "AccountId32",
                main: "AccountId32",
                deposit: "u128",
            },
            SubIdentitiesSet: {
                main: "AccountId32",
                numberOfSubs: "u32",
                newDeposit: "u128",
            },
            SubIdentityRenamed: {
                sub: "AccountId32",
                main: "AccountId32",
            },
            SubIdentityRemoved: {
                sub: "AccountId32",
                main: "AccountId32",
                deposit: "u128",
            },
            SubIdentityRevoked: {
                sub: "AccountId32",
                main: "AccountId32",
                deposit: "u128",
            },
            AuthorityAdded: {
                authority: "AccountId32",
            },
            AuthorityRemoved: {
                authority: "AccountId32",
            },
            UsernameSet: {
                who: "AccountId32",
                username: "Bytes",
            },
            UsernameQueued: {
                who: "AccountId32",
                username: "Bytes",
                expiration: "u32",
            },
            PreapprovalExpired: {
                whose: "AccountId32",
            },
            PrimaryUsernameSet: {
                who: "AccountId32",
                username: "Bytes",
            },
            DanglingUsernameRemoved: {
                who: "AccountId32",
                username: "Bytes",
            },
            UsernameUnbound: {
                username: "Bytes",
            },
            UsernameRemoved: {
                username: "Bytes",
            },
            UsernameKilled: {
                username: "Bytes",
            },
        },
    },
    /**
     * Lookup61: pallet_multisig::pallet::Event<T>
     **/
    PalletMultisigEvent: {
        _enum: {
            NewMultisig: {
                approving: "AccountId32",
                multisig: "AccountId32",
                callHash: "[u8;32]",
            },
            MultisigApproval: {
                approving: "AccountId32",
                timepoint: "PalletMultisigTimepoint",
                multisig: "AccountId32",
                callHash: "[u8;32]",
            },
            MultisigExecuted: {
                approving: "AccountId32",
                timepoint: "PalletMultisigTimepoint",
                multisig: "AccountId32",
                callHash: "[u8;32]",
                result: "Result<Null, SpRuntimeDispatchError>",
            },
            MultisigCancelled: {
                cancelling: "AccountId32",
                timepoint: "PalletMultisigTimepoint",
                multisig: "AccountId32",
                callHash: "[u8;32]",
            },
        },
    },
    /**
     * Lookup62: pallet_multisig::Timepoint<BlockNumber>
     **/
    PalletMultisigTimepoint: {
        height: "u32",
        index: "u32",
    },
    /**
     * Lookup63: pallet_registrar::pallet::Event<T>
     **/
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
            ParaIdUnpaused: {
                paraId: "u32",
            },
            ParathreadParamsChanged: {
                paraId: "u32",
            },
            ParaManagerChanged: {
                paraId: "u32",
                managerAddress: "AccountId32",
            },
        },
    },
    /**
     * Lookup65: pallet_collator_assignment::pallet::Event<T>
     **/
    PalletCollatorAssignmentEvent: {
        _enum: {
            NewPendingAssignment: {
                randomSeed: "[u8;32]",
                fullRotation: "bool",
                targetSession: "u32",
                fullRotationMode: "TpTraitsFullRotationModes",
            },
        },
    },
    /**
     * Lookup66: tp_traits::FullRotationModes
     **/
    TpTraitsFullRotationModes: {
        orchestrator: "TpTraitsFullRotationMode",
        parachain: "TpTraitsFullRotationMode",
        parathread: "TpTraitsFullRotationMode",
    },
    /**
     * Lookup67: tp_traits::FullRotationMode
     **/
    TpTraitsFullRotationMode: {
        _enum: {
            RotateAll: "Null",
            KeepAll: "Null",
            KeepCollators: {
                keep: "u32",
            },
            KeepPerbill: {
                percentage: "Perbill",
            },
        },
    },
    /**
     * Lookup69: pallet_author_noting::pallet::Event<T>
     **/
    PalletAuthorNotingEvent: {
        _enum: {
            LatestAuthorChanged: {
                paraId: "u32",
                blockNumber: "u32",
                newAuthor: "AccountId32",
                latestSlotNumber: "u64",
            },
            RemovedAuthorData: {
                paraId: "u32",
            },
        },
    },
    /**
     * Lookup71: pallet_services_payment::pallet::Event<T>
     **/
    PalletServicesPaymentEvent: {
        _enum: {
            CreditsPurchased: {
                paraId: "u32",
                payer: "AccountId32",
                credit: "u128",
            },
            BlockProductionCreditBurned: {
                paraId: "u32",
                creditsRemaining: "u32",
            },
            CollatorAssignmentCreditBurned: {
                paraId: "u32",
                creditsRemaining: "u32",
            },
            CollatorAssignmentTipCollected: {
                paraId: "u32",
                payer: "AccountId32",
                tip: "u128",
            },
            BlockProductionCreditsSet: {
                paraId: "u32",
                credits: "u32",
            },
            RefundAddressUpdated: {
                paraId: "u32",
                refundAddress: "Option<AccountId32>",
            },
            MaxCorePriceUpdated: {
                paraId: "u32",
                maxCorePrice: "u128",
            },
            CollatorAssignmentCreditsSet: {
                paraId: "u32",
                credits: "u32",
            },
        },
    },
    /**
     * Lookup72: pallet_data_preservers::pallet::Event<T>
     **/
    PalletDataPreserversEvent: {
        _enum: {
            BootNodesChanged: {
                paraId: "u32",
            },
            ProfileCreated: {
                account: "AccountId32",
                profileId: "u64",
                deposit: "u128",
            },
            ProfileUpdated: {
                profileId: "u64",
                oldDeposit: "u128",
                newDeposit: "u128",
            },
            ProfileDeleted: {
                profileId: "u64",
                releasedDeposit: "u128",
            },
            AssignmentStarted: {
                profileId: "u64",
                paraId: "u32",
            },
            AssignmentStopped: {
                profileId: "u64",
                paraId: "u32",
            },
        },
    },
    /**
     * Lookup73: pallet_invulnerables::pallet::Event<T>
     **/
    PalletInvulnerablesEvent: {
        _enum: {
            InvulnerableAdded: {
                accountId: "AccountId32",
            },
            InvulnerableRemoved: {
                accountId: "AccountId32",
            },
        },
    },
    /**
     * Lookup74: pallet_session::pallet::Event
     **/
    PalletSessionEvent: {
        _enum: {
            NewSession: {
                sessionIndex: "u32",
            },
        },
    },
    /**
     * Lookup75: pallet_pooled_staking::pallet::Event<T>
     **/
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
                pool: "PalletPooledStakingPoolsActivePoolKind",
                pending: "u128",
            },
            ExecutedDelegate: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                pool: "PalletPooledStakingPoolsActivePoolKind",
                staked: "u128",
                released: "u128",
            },
            RequestedUndelegate: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                from: "PalletPooledStakingPoolsActivePoolKind",
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
                sourcePool: "PalletPooledStakingPoolsActivePoolKind",
                sourceShares: "u128",
                sourceStake: "u128",
                targetShares: "u128",
                targetStake: "u128",
                pendingLeaving: "u128",
                released: "u128",
            },
        },
    },
    /**
     * Lookup77: pallet_pooled_staking::pools::ActivePoolKind
     **/
    PalletPooledStakingPoolsActivePoolKind: {
        _enum: ["AutoCompounding", "ManualRewards"],
    },
    /**
     * Lookup78: pallet_inflation_rewards::pallet::Event<T>
     **/
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
    /**
     * Lookup79: pallet_treasury::pallet::Event<T, I>
     **/
    PalletTreasuryEvent: {
        _enum: {
            Spending: {
                budgetRemaining: "u128",
            },
            Awarded: {
                proposalIndex: "u32",
                award: "u128",
                account: "AccountId32",
            },
            Burnt: {
                burntFunds: "u128",
            },
            Rollover: {
                rolloverBalance: "u128",
            },
            Deposit: {
                value: "u128",
            },
            SpendApproved: {
                proposalIndex: "u32",
                amount: "u128",
                beneficiary: "AccountId32",
            },
            UpdatedInactive: {
                reactivated: "u128",
                deactivated: "u128",
            },
            AssetSpendApproved: {
                index: "u32",
                assetKind: "Null",
                amount: "u128",
                beneficiary: "AccountId32",
                validFrom: "u32",
                expireAt: "u32",
            },
            AssetSpendVoided: {
                index: "u32",
            },
            Paid: {
                index: "u32",
                paymentId: "Null",
            },
            PaymentFailed: {
                index: "u32",
                paymentId: "Null",
            },
            SpendProcessed: {
                index: "u32",
            },
        },
    },
    /**
     * Lookup80: cumulus_pallet_xcmp_queue::pallet::Event<T>
     **/
    CumulusPalletXcmpQueueEvent: {
        _enum: {
            XcmpMessageSent: {
                messageHash: "[u8;32]",
            },
        },
    },
    /**
     * Lookup81: cumulus_pallet_xcm::pallet::Event<T>
     **/
    CumulusPalletXcmEvent: {
        _enum: {
            InvalidFormat: "[u8;32]",
            UnsupportedVersion: "[u8;32]",
            ExecutedDownward: "([u8;32],StagingXcmV5TraitsOutcome)",
        },
    },
    /**
     * Lookup82: staging_xcm::v5::traits::Outcome
     **/
    StagingXcmV5TraitsOutcome: {
        _enum: {
            Complete: {
                used: "SpWeightsWeightV2Weight",
            },
            Incomplete: {
                used: "SpWeightsWeightV2Weight",
                error: "XcmV5TraitsError",
            },
            Error: {
                error: "XcmV5TraitsError",
            },
        },
    },
    /**
     * Lookup83: xcm::v5::traits::Error
     **/
    XcmV5TraitsError: {
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
            TooManyAssets: "Null",
            UnhandledXcmVersion: "Null",
            WeightLimitReached: "SpWeightsWeightV2Weight",
            Barrier: "Null",
            WeightNotComputable: "Null",
            ExceedsStackLimit: "Null",
        },
    },
    /**
     * Lookup84: pallet_xcm::pallet::Event<T>
     **/
    PalletXcmEvent: {
        _enum: {
            Attempted: {
                outcome: "StagingXcmV5TraitsOutcome",
            },
            Sent: {
                origin: "StagingXcmV5Location",
                destination: "StagingXcmV5Location",
                message: "StagingXcmV5Xcm",
                messageId: "[u8;32]",
            },
            UnexpectedResponse: {
                origin: "StagingXcmV5Location",
                queryId: "u64",
            },
            ResponseReady: {
                queryId: "u64",
                response: "StagingXcmV5Response",
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
                origin: "StagingXcmV5Location",
                queryId: "u64",
                expectedLocation: "Option<StagingXcmV5Location>",
            },
            InvalidResponderVersion: {
                origin: "StagingXcmV5Location",
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
                origin: "StagingXcmV5Location",
                assets: "XcmVersionedAssets",
            },
            VersionChangeNotified: {
                destination: "StagingXcmV5Location",
                result: "u32",
                cost: "StagingXcmV5AssetAssets",
                messageId: "[u8;32]",
            },
            SupportedVersionChanged: {
                location: "StagingXcmV5Location",
                version: "u32",
            },
            NotifyTargetSendFail: {
                location: "StagingXcmV5Location",
                queryId: "u64",
                error: "XcmV5TraitsError",
            },
            NotifyTargetMigrationFail: {
                location: "XcmVersionedLocation",
                queryId: "u64",
            },
            InvalidQuerierVersion: {
                origin: "StagingXcmV5Location",
                queryId: "u64",
            },
            InvalidQuerier: {
                origin: "StagingXcmV5Location",
                queryId: "u64",
                expectedQuerier: "StagingXcmV5Location",
                maybeActualQuerier: "Option<StagingXcmV5Location>",
            },
            VersionNotifyStarted: {
                destination: "StagingXcmV5Location",
                cost: "StagingXcmV5AssetAssets",
                messageId: "[u8;32]",
            },
            VersionNotifyRequested: {
                destination: "StagingXcmV5Location",
                cost: "StagingXcmV5AssetAssets",
                messageId: "[u8;32]",
            },
            VersionNotifyUnrequested: {
                destination: "StagingXcmV5Location",
                cost: "StagingXcmV5AssetAssets",
                messageId: "[u8;32]",
            },
            FeesPaid: {
                paying: "StagingXcmV5Location",
                fees: "StagingXcmV5AssetAssets",
            },
            AssetsClaimed: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
                origin: "StagingXcmV5Location",
                assets: "XcmVersionedAssets",
            },
            VersionMigrationFinished: {
                version: "u32",
            },
        },
    },
    /**
     * Lookup85: staging_xcm::v5::location::Location
     **/
    StagingXcmV5Location: {
        parents: "u8",
        interior: "StagingXcmV5Junctions",
    },
    /**
     * Lookup86: staging_xcm::v5::junctions::Junctions
     **/
    StagingXcmV5Junctions: {
        _enum: {
            Here: "Null",
            X1: "[Lookup88;1]",
            X2: "[Lookup88;2]",
            X3: "[Lookup88;3]",
            X4: "[Lookup88;4]",
            X5: "[Lookup88;5]",
            X6: "[Lookup88;6]",
            X7: "[Lookup88;7]",
            X8: "[Lookup88;8]",
        },
    },
    /**
     * Lookup88: staging_xcm::v5::junction::Junction
     **/
    StagingXcmV5Junction: {
        _enum: {
            Parachain: "Compact<u32>",
            AccountId32: {
                network: "Option<StagingXcmV5JunctionNetworkId>",
                id: "[u8;32]",
            },
            AccountIndex64: {
                network: "Option<StagingXcmV5JunctionNetworkId>",
                index: "Compact<u64>",
            },
            AccountKey20: {
                network: "Option<StagingXcmV5JunctionNetworkId>",
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
                id: "XcmV3JunctionBodyId",
                part: "XcmV3JunctionBodyPart",
            },
            GlobalConsensus: "StagingXcmV5JunctionNetworkId",
        },
    },
    /**
     * Lookup91: staging_xcm::v5::junction::NetworkId
     **/
    StagingXcmV5JunctionNetworkId: {
        _enum: {
            ByGenesis: "[u8;32]",
            ByFork: {
                blockNumber: "u64",
                blockHash: "[u8;32]",
            },
            Polkadot: "Null",
            Kusama: "Null",
            Ethereum: {
                chainId: "Compact<u64>",
            },
            BitcoinCore: "Null",
            BitcoinCash: "Null",
            PolkadotBulletin: "Null",
        },
    },
    /**
     * Lookup94: xcm::v3::junction::BodyId
     **/
    XcmV3JunctionBodyId: {
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
    /**
     * Lookup95: xcm::v3::junction::BodyPart
     **/
    XcmV3JunctionBodyPart: {
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
    /**
     * Lookup103: staging_xcm::v5::Xcm<Call>
     **/
    StagingXcmV5Xcm: "Vec<StagingXcmV5Instruction>",
    /**
     * Lookup105: staging_xcm::v5::Instruction<Call>
     **/
    StagingXcmV5Instruction: {
        _enum: {
            WithdrawAsset: "StagingXcmV5AssetAssets",
            ReserveAssetDeposited: "StagingXcmV5AssetAssets",
            ReceiveTeleportedAsset: "StagingXcmV5AssetAssets",
            QueryResponse: {
                queryId: "Compact<u64>",
                response: "StagingXcmV5Response",
                maxWeight: "SpWeightsWeightV2Weight",
                querier: "Option<StagingXcmV5Location>",
            },
            TransferAsset: {
                assets: "StagingXcmV5AssetAssets",
                beneficiary: "StagingXcmV5Location",
            },
            TransferReserveAsset: {
                assets: "StagingXcmV5AssetAssets",
                dest: "StagingXcmV5Location",
                xcm: "StagingXcmV5Xcm",
            },
            Transact: {
                originKind: "XcmV3OriginKind",
                fallbackMaxWeight: "Option<SpWeightsWeightV2Weight>",
                call: "XcmDoubleEncoded",
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
            DescendOrigin: "StagingXcmV5Junctions",
            ReportError: "StagingXcmV5QueryResponseInfo",
            DepositAsset: {
                assets: "StagingXcmV5AssetAssetFilter",
                beneficiary: "StagingXcmV5Location",
            },
            DepositReserveAsset: {
                assets: "StagingXcmV5AssetAssetFilter",
                dest: "StagingXcmV5Location",
                xcm: "StagingXcmV5Xcm",
            },
            ExchangeAsset: {
                give: "StagingXcmV5AssetAssetFilter",
                want: "StagingXcmV5AssetAssets",
                maximal: "bool",
            },
            InitiateReserveWithdraw: {
                assets: "StagingXcmV5AssetAssetFilter",
                reserve: "StagingXcmV5Location",
                xcm: "StagingXcmV5Xcm",
            },
            InitiateTeleport: {
                assets: "StagingXcmV5AssetAssetFilter",
                dest: "StagingXcmV5Location",
                xcm: "StagingXcmV5Xcm",
            },
            ReportHolding: {
                responseInfo: "StagingXcmV5QueryResponseInfo",
                assets: "StagingXcmV5AssetAssetFilter",
            },
            BuyExecution: {
                fees: "StagingXcmV5Asset",
                weightLimit: "XcmV3WeightLimit",
            },
            RefundSurplus: "Null",
            SetErrorHandler: "StagingXcmV5Xcm",
            SetAppendix: "StagingXcmV5Xcm",
            ClearError: "Null",
            ClaimAsset: {
                assets: "StagingXcmV5AssetAssets",
                ticket: "StagingXcmV5Location",
            },
            Trap: "Compact<u64>",
            SubscribeVersion: {
                queryId: "Compact<u64>",
                maxResponseWeight: "SpWeightsWeightV2Weight",
            },
            UnsubscribeVersion: "Null",
            BurnAsset: "StagingXcmV5AssetAssets",
            ExpectAsset: "StagingXcmV5AssetAssets",
            ExpectOrigin: "Option<StagingXcmV5Location>",
            ExpectError: "Option<(u32,XcmV5TraitsError)>",
            ExpectTransactStatus: "XcmV3MaybeErrorCode",
            QueryPallet: {
                moduleName: "Bytes",
                responseInfo: "StagingXcmV5QueryResponseInfo",
            },
            ExpectPallet: {
                index: "Compact<u32>",
                name: "Bytes",
                moduleName: "Bytes",
                crateMajor: "Compact<u32>",
                minCrateMinor: "Compact<u32>",
            },
            ReportTransactStatus: "StagingXcmV5QueryResponseInfo",
            ClearTransactStatus: "Null",
            UniversalOrigin: "StagingXcmV5Junction",
            ExportMessage: {
                network: "StagingXcmV5JunctionNetworkId",
                destination: "StagingXcmV5Junctions",
                xcm: "StagingXcmV5Xcm",
            },
            LockAsset: {
                asset: "StagingXcmV5Asset",
                unlocker: "StagingXcmV5Location",
            },
            UnlockAsset: {
                asset: "StagingXcmV5Asset",
                target: "StagingXcmV5Location",
            },
            NoteUnlockable: {
                asset: "StagingXcmV5Asset",
                owner: "StagingXcmV5Location",
            },
            RequestUnlock: {
                asset: "StagingXcmV5Asset",
                locker: "StagingXcmV5Location",
            },
            SetFeesMode: {
                jitWithdraw: "bool",
            },
            SetTopic: "[u8;32]",
            ClearTopic: "Null",
            AliasOrigin: "StagingXcmV5Location",
            UnpaidExecution: {
                weightLimit: "XcmV3WeightLimit",
                checkOrigin: "Option<StagingXcmV5Location>",
            },
            PayFees: {
                asset: "StagingXcmV5Asset",
            },
            InitiateTransfer: {
                destination: "StagingXcmV5Location",
                remoteFees: "Option<StagingXcmV5AssetAssetTransferFilter>",
                preserveOrigin: "bool",
                assets: "Vec<StagingXcmV5AssetAssetTransferFilter>",
                remoteXcm: "StagingXcmV5Xcm",
            },
            ExecuteWithOrigin: {
                descendantOrigin: "Option<StagingXcmV5Junctions>",
                xcm: "StagingXcmV5Xcm",
            },
            SetHints: {
                hints: "Vec<StagingXcmV5Hint>",
            },
        },
    },
    /**
     * Lookup106: staging_xcm::v5::asset::Assets
     **/
    StagingXcmV5AssetAssets: "Vec<StagingXcmV5Asset>",
    /**
     * Lookup108: staging_xcm::v5::asset::Asset
     **/
    StagingXcmV5Asset: {
        id: "StagingXcmV5AssetAssetId",
        fun: "StagingXcmV5AssetFungibility",
    },
    /**
     * Lookup109: staging_xcm::v5::asset::AssetId
     **/
    StagingXcmV5AssetAssetId: "StagingXcmV5Location",
    /**
     * Lookup110: staging_xcm::v5::asset::Fungibility
     **/
    StagingXcmV5AssetFungibility: {
        _enum: {
            Fungible: "Compact<u128>",
            NonFungible: "StagingXcmV5AssetAssetInstance",
        },
    },
    /**
     * Lookup111: staging_xcm::v5::asset::AssetInstance
     **/
    StagingXcmV5AssetAssetInstance: {
        _enum: {
            Undefined: "Null",
            Index: "Compact<u128>",
            Array4: "[u8;4]",
            Array8: "[u8;8]",
            Array16: "[u8;16]",
            Array32: "[u8;32]",
        },
    },
    /**
     * Lookup114: staging_xcm::v5::Response
     **/
    StagingXcmV5Response: {
        _enum: {
            Null: "Null",
            Assets: "StagingXcmV5AssetAssets",
            ExecutionResult: "Option<(u32,XcmV5TraitsError)>",
            Version: "u32",
            PalletsInfo: "Vec<StagingXcmV5PalletInfo>",
            DispatchResult: "XcmV3MaybeErrorCode",
        },
    },
    /**
     * Lookup118: staging_xcm::v5::PalletInfo
     **/
    StagingXcmV5PalletInfo: {
        index: "Compact<u32>",
        name: "Bytes",
        moduleName: "Bytes",
        major: "Compact<u32>",
        minor: "Compact<u32>",
        patch: "Compact<u32>",
    },
    /**
     * Lookup121: xcm::v3::MaybeErrorCode
     **/
    XcmV3MaybeErrorCode: {
        _enum: {
            Success: "Null",
            Error: "Bytes",
            TruncatedError: "Bytes",
        },
    },
    /**
     * Lookup124: xcm::v3::OriginKind
     **/
    XcmV3OriginKind: {
        _enum: ["Native", "SovereignAccount", "Superuser", "Xcm"],
    },
    /**
     * Lookup126: xcm::double_encoded::DoubleEncoded<T>
     **/
    XcmDoubleEncoded: {
        encoded: "Bytes",
    },
    /**
     * Lookup127: staging_xcm::v5::QueryResponseInfo
     **/
    StagingXcmV5QueryResponseInfo: {
        destination: "StagingXcmV5Location",
        queryId: "Compact<u64>",
        maxWeight: "SpWeightsWeightV2Weight",
    },
    /**
     * Lookup128: staging_xcm::v5::asset::AssetFilter
     **/
    StagingXcmV5AssetAssetFilter: {
        _enum: {
            Definite: "StagingXcmV5AssetAssets",
            Wild: "StagingXcmV5AssetWildAsset",
        },
    },
    /**
     * Lookup129: staging_xcm::v5::asset::WildAsset
     **/
    StagingXcmV5AssetWildAsset: {
        _enum: {
            All: "Null",
            AllOf: {
                id: "StagingXcmV5AssetAssetId",
                fun: "StagingXcmV5AssetWildFungibility",
            },
            AllCounted: "Compact<u32>",
            AllOfCounted: {
                id: "StagingXcmV5AssetAssetId",
                fun: "StagingXcmV5AssetWildFungibility",
                count: "Compact<u32>",
            },
        },
    },
    /**
     * Lookup130: staging_xcm::v5::asset::WildFungibility
     **/
    StagingXcmV5AssetWildFungibility: {
        _enum: ["Fungible", "NonFungible"],
    },
    /**
     * Lookup131: xcm::v3::WeightLimit
     **/
    XcmV3WeightLimit: {
        _enum: {
            Unlimited: "Null",
            Limited: "SpWeightsWeightV2Weight",
        },
    },
    /**
     * Lookup133: staging_xcm::v5::asset::AssetTransferFilter
     **/
    StagingXcmV5AssetAssetTransferFilter: {
        _enum: {
            Teleport: "StagingXcmV5AssetAssetFilter",
            ReserveDeposit: "StagingXcmV5AssetAssetFilter",
            ReserveWithdraw: "StagingXcmV5AssetAssetFilter",
        },
    },
    /**
     * Lookup137: staging_xcm::v5::Hint
     **/
    StagingXcmV5Hint: {
        _enum: {
            AssetClaimer: {
                location: "StagingXcmV5Location",
            },
        },
    },
    /**
     * Lookup139: xcm::VersionedAssets
     **/
    XcmVersionedAssets: {
        _enum: {
            __Unused0: "Null",
            __Unused1: "Null",
            __Unused2: "Null",
            V3: "XcmV3MultiassetMultiAssets",
            V4: "StagingXcmV4AssetAssets",
            V5: "StagingXcmV5AssetAssets",
        },
    },
    /**
     * Lookup140: xcm::v3::multiasset::MultiAssets
     **/
    XcmV3MultiassetMultiAssets: "Vec<XcmV3MultiAsset>",
    /**
     * Lookup142: xcm::v3::multiasset::MultiAsset
     **/
    XcmV3MultiAsset: {
        id: "XcmV3MultiassetAssetId",
        fun: "XcmV3MultiassetFungibility",
    },
    /**
     * Lookup143: xcm::v3::multiasset::AssetId
     **/
    XcmV3MultiassetAssetId: {
        _enum: {
            Concrete: "StagingXcmV3MultiLocation",
            Abstract: "[u8;32]",
        },
    },
    /**
     * Lookup144: staging_xcm::v3::multilocation::MultiLocation
     **/
    StagingXcmV3MultiLocation: {
        parents: "u8",
        interior: "XcmV3Junctions",
    },
    /**
     * Lookup145: xcm::v3::junctions::Junctions
     **/
    XcmV3Junctions: {
        _enum: {
            Here: "Null",
            X1: "XcmV3Junction",
            X2: "(XcmV3Junction,XcmV3Junction)",
            X3: "(XcmV3Junction,XcmV3Junction,XcmV3Junction)",
            X4: "(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)",
            X5: "(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)",
            X6: "(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)",
            X7: "(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)",
            X8: "(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)",
        },
    },
    /**
     * Lookup146: xcm::v3::junction::Junction
     **/
    XcmV3Junction: {
        _enum: {
            Parachain: "Compact<u32>",
            AccountId32: {
                network: "Option<XcmV3JunctionNetworkId>",
                id: "[u8;32]",
            },
            AccountIndex64: {
                network: "Option<XcmV3JunctionNetworkId>",
                index: "Compact<u64>",
            },
            AccountKey20: {
                network: "Option<XcmV3JunctionNetworkId>",
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
                id: "XcmV3JunctionBodyId",
                part: "XcmV3JunctionBodyPart",
            },
            GlobalConsensus: "XcmV3JunctionNetworkId",
        },
    },
    /**
     * Lookup148: xcm::v3::junction::NetworkId
     **/
    XcmV3JunctionNetworkId: {
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
            PolkadotBulletin: "Null",
        },
    },
    /**
     * Lookup149: xcm::v3::multiasset::Fungibility
     **/
    XcmV3MultiassetFungibility: {
        _enum: {
            Fungible: "Compact<u128>",
            NonFungible: "XcmV3MultiassetAssetInstance",
        },
    },
    /**
     * Lookup150: xcm::v3::multiasset::AssetInstance
     **/
    XcmV3MultiassetAssetInstance: {
        _enum: {
            Undefined: "Null",
            Index: "Compact<u128>",
            Array4: "[u8;4]",
            Array8: "[u8;8]",
            Array16: "[u8;16]",
            Array32: "[u8;32]",
        },
    },
    /**
     * Lookup151: staging_xcm::v4::asset::Assets
     **/
    StagingXcmV4AssetAssets: "Vec<StagingXcmV4Asset>",
    /**
     * Lookup153: staging_xcm::v4::asset::Asset
     **/
    StagingXcmV4Asset: {
        id: "StagingXcmV4AssetAssetId",
        fun: "StagingXcmV4AssetFungibility",
    },
    /**
     * Lookup154: staging_xcm::v4::asset::AssetId
     **/
    StagingXcmV4AssetAssetId: "StagingXcmV4Location",
    /**
     * Lookup155: staging_xcm::v4::location::Location
     **/
    StagingXcmV4Location: {
        parents: "u8",
        interior: "StagingXcmV4Junctions",
    },
    /**
     * Lookup156: staging_xcm::v4::junctions::Junctions
     **/
    StagingXcmV4Junctions: {
        _enum: {
            Here: "Null",
            X1: "[Lookup158;1]",
            X2: "[Lookup158;2]",
            X3: "[Lookup158;3]",
            X4: "[Lookup158;4]",
            X5: "[Lookup158;5]",
            X6: "[Lookup158;6]",
            X7: "[Lookup158;7]",
            X8: "[Lookup158;8]",
        },
    },
    /**
     * Lookup158: staging_xcm::v4::junction::Junction
     **/
    StagingXcmV4Junction: {
        _enum: {
            Parachain: "Compact<u32>",
            AccountId32: {
                network: "Option<StagingXcmV4JunctionNetworkId>",
                id: "[u8;32]",
            },
            AccountIndex64: {
                network: "Option<StagingXcmV4JunctionNetworkId>",
                index: "Compact<u64>",
            },
            AccountKey20: {
                network: "Option<StagingXcmV4JunctionNetworkId>",
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
                id: "XcmV3JunctionBodyId",
                part: "XcmV3JunctionBodyPart",
            },
            GlobalConsensus: "StagingXcmV4JunctionNetworkId",
        },
    },
    /**
     * Lookup160: staging_xcm::v4::junction::NetworkId
     **/
    StagingXcmV4JunctionNetworkId: {
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
            PolkadotBulletin: "Null",
        },
    },
    /**
     * Lookup168: staging_xcm::v4::asset::Fungibility
     **/
    StagingXcmV4AssetFungibility: {
        _enum: {
            Fungible: "Compact<u128>",
            NonFungible: "StagingXcmV4AssetAssetInstance",
        },
    },
    /**
     * Lookup169: staging_xcm::v4::asset::AssetInstance
     **/
    StagingXcmV4AssetAssetInstance: {
        _enum: {
            Undefined: "Null",
            Index: "Compact<u128>",
            Array4: "[u8;4]",
            Array8: "[u8;8]",
            Array16: "[u8;16]",
            Array32: "[u8;32]",
        },
    },
    /**
     * Lookup170: xcm::VersionedLocation
     **/
    XcmVersionedLocation: {
        _enum: {
            __Unused0: "Null",
            __Unused1: "Null",
            __Unused2: "Null",
            V3: "StagingXcmV3MultiLocation",
            V4: "StagingXcmV4Location",
            V5: "StagingXcmV5Location",
        },
    },
    /**
     * Lookup171: pallet_assets::pallet::Event<T, I>
     **/
    PalletAssetsEvent: {
        _enum: {
            Created: {
                assetId: "u16",
                creator: "AccountId32",
                owner: "AccountId32",
            },
            Issued: {
                assetId: "u16",
                owner: "AccountId32",
                amount: "u128",
            },
            Transferred: {
                assetId: "u16",
                from: "AccountId32",
                to: "AccountId32",
                amount: "u128",
            },
            Burned: {
                assetId: "u16",
                owner: "AccountId32",
                balance: "u128",
            },
            TeamChanged: {
                assetId: "u16",
                issuer: "AccountId32",
                admin: "AccountId32",
                freezer: "AccountId32",
            },
            OwnerChanged: {
                assetId: "u16",
                owner: "AccountId32",
            },
            Frozen: {
                assetId: "u16",
                who: "AccountId32",
            },
            Thawed: {
                assetId: "u16",
                who: "AccountId32",
            },
            AssetFrozen: {
                assetId: "u16",
            },
            AssetThawed: {
                assetId: "u16",
            },
            AccountsDestroyed: {
                assetId: "u16",
                accountsDestroyed: "u32",
                accountsRemaining: "u32",
            },
            ApprovalsDestroyed: {
                assetId: "u16",
                approvalsDestroyed: "u32",
                approvalsRemaining: "u32",
            },
            DestructionStarted: {
                assetId: "u16",
            },
            Destroyed: {
                assetId: "u16",
            },
            ForceCreated: {
                assetId: "u16",
                owner: "AccountId32",
            },
            MetadataSet: {
                assetId: "u16",
                name: "Bytes",
                symbol: "Bytes",
                decimals: "u8",
                isFrozen: "bool",
            },
            MetadataCleared: {
                assetId: "u16",
            },
            ApprovedTransfer: {
                assetId: "u16",
                source: "AccountId32",
                delegate: "AccountId32",
                amount: "u128",
            },
            ApprovalCancelled: {
                assetId: "u16",
                owner: "AccountId32",
                delegate: "AccountId32",
            },
            TransferredApproved: {
                assetId: "u16",
                owner: "AccountId32",
                delegate: "AccountId32",
                destination: "AccountId32",
                amount: "u128",
            },
            AssetStatusChanged: {
                assetId: "u16",
            },
            AssetMinBalanceChanged: {
                assetId: "u16",
                newMinBalance: "u128",
            },
            Touched: {
                assetId: "u16",
                who: "AccountId32",
                depositor: "AccountId32",
            },
            Blocked: {
                assetId: "u16",
                who: "AccountId32",
            },
            Deposited: {
                assetId: "u16",
                who: "AccountId32",
                amount: "u128",
            },
            Withdrawn: {
                assetId: "u16",
                who: "AccountId32",
                amount: "u128",
            },
        },
    },
    /**
     * Lookup172: pallet_foreign_asset_creator::pallet::Event<T>
     **/
    PalletForeignAssetCreatorEvent: {
        _enum: {
            ForeignAssetCreated: {
                assetId: "u16",
                foreignAsset: "StagingXcmV5Location",
            },
            ForeignAssetTypeChanged: {
                assetId: "u16",
                newForeignAsset: "StagingXcmV5Location",
            },
            ForeignAssetRemoved: {
                assetId: "u16",
                foreignAsset: "StagingXcmV5Location",
            },
            ForeignAssetDestroyed: {
                assetId: "u16",
                foreignAsset: "StagingXcmV5Location",
            },
        },
    },
    /**
     * Lookup173: pallet_asset_rate::pallet::Event<T>
     **/
    PalletAssetRateEvent: {
        _enum: {
            AssetRateCreated: {
                assetKind: "u16",
                rate: "u128",
            },
            AssetRateRemoved: {
                assetKind: "u16",
            },
            AssetRateUpdated: {
                _alias: {
                    new_: "new",
                },
                assetKind: "u16",
                old: "u128",
                new_: "u128",
            },
        },
    },
    /**
     * Lookup175: pallet_message_queue::pallet::Event<T>
     **/
    PalletMessageQueueEvent: {
        _enum: {
            ProcessingFailed: {
                id: "H256",
                origin: "CumulusPrimitivesCoreAggregateMessageOrigin",
                error: "FrameSupportMessagesProcessMessageError",
            },
            Processed: {
                id: "H256",
                origin: "CumulusPrimitivesCoreAggregateMessageOrigin",
                weightUsed: "SpWeightsWeightV2Weight",
                success: "bool",
            },
            OverweightEnqueued: {
                id: "[u8;32]",
                origin: "CumulusPrimitivesCoreAggregateMessageOrigin",
                pageIndex: "u32",
                messageIndex: "u32",
            },
            PageReaped: {
                origin: "CumulusPrimitivesCoreAggregateMessageOrigin",
                index: "u32",
            },
        },
    },
    /**
     * Lookup176: cumulus_primitives_core::AggregateMessageOrigin
     **/
    CumulusPrimitivesCoreAggregateMessageOrigin: {
        _enum: {
            Here: "Null",
            Parent: "Null",
            Sibling: "u32",
        },
    },
    /**
     * Lookup177: frame_support::traits::messages::ProcessMessageError
     **/
    FrameSupportMessagesProcessMessageError: {
        _enum: {
            BadFormat: "Null",
            Corrupt: "Null",
            Unsupported: "Null",
            Overweight: "SpWeightsWeightV2Weight",
            Yield: "Null",
            StackLimitReached: "Null",
        },
    },
    /**
     * Lookup178: pallet_xcm_core_buyer::pallet::Event<T>
     **/
    PalletXcmCoreBuyerEvent: {
        _enum: {
            BuyCoreXcmSent: {
                paraId: "u32",
                transactionStatusQueryId: "u64",
            },
            ReceivedBuyCoreXCMResult: {
                paraId: "u32",
                response: "StagingXcmV5Response",
            },
            CleanedUpExpiredPendingBlocksEntries: {
                paraIds: "Vec<u32>",
            },
            CleanedUpExpiredInFlightOrderEntries: {
                paraIds: "Vec<u32>",
            },
        },
    },
    /**
     * Lookup180: pallet_root_testing::pallet::Event<T>
     **/
    PalletRootTestingEvent: {
        _enum: ["DefensiveTestCall"],
    },
    /**
     * Lookup181: frame_system::Phase
     **/
    FrameSystemPhase: {
        _enum: {
            ApplyExtrinsic: "u32",
            Finalization: "Null",
            Initialization: "Null",
        },
    },
    /**
     * Lookup185: frame_system::LastRuntimeUpgradeInfo
     **/
    FrameSystemLastRuntimeUpgradeInfo: {
        specVersion: "Compact<u32>",
        specName: "Text",
    },
    /**
     * Lookup188: frame_system::CodeUpgradeAuthorization<T>
     **/
    FrameSystemCodeUpgradeAuthorization: {
        codeHash: "H256",
        checkVersion: "bool",
    },
    /**
     * Lookup189: frame_system::pallet::Call<T>
     **/
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
            __Unused8: "Null",
            authorize_upgrade: {
                codeHash: "H256",
            },
            authorize_upgrade_without_checks: {
                codeHash: "H256",
            },
            apply_authorized_upgrade: {
                code: "Bytes",
            },
        },
    },
    /**
     * Lookup193: frame_system::limits::BlockWeights
     **/
    FrameSystemLimitsBlockWeights: {
        baseBlock: "SpWeightsWeightV2Weight",
        maxBlock: "SpWeightsWeightV2Weight",
        perClass: "FrameSupportDispatchPerDispatchClassWeightsPerClass",
    },
    /**
     * Lookup194: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
     **/
    FrameSupportDispatchPerDispatchClassWeightsPerClass: {
        normal: "FrameSystemLimitsWeightsPerClass",
        operational: "FrameSystemLimitsWeightsPerClass",
        mandatory: "FrameSystemLimitsWeightsPerClass",
    },
    /**
     * Lookup195: frame_system::limits::WeightsPerClass
     **/
    FrameSystemLimitsWeightsPerClass: {
        baseExtrinsic: "SpWeightsWeightV2Weight",
        maxExtrinsic: "Option<SpWeightsWeightV2Weight>",
        maxTotal: "Option<SpWeightsWeightV2Weight>",
        reserved: "Option<SpWeightsWeightV2Weight>",
    },
    /**
     * Lookup196: frame_system::limits::BlockLength
     **/
    FrameSystemLimitsBlockLength: {
        max: "FrameSupportDispatchPerDispatchClassU32",
    },
    /**
     * Lookup197: frame_support::dispatch::PerDispatchClass<T>
     **/
    FrameSupportDispatchPerDispatchClassU32: {
        normal: "u32",
        operational: "u32",
        mandatory: "u32",
    },
    /**
     * Lookup198: sp_weights::RuntimeDbWeight
     **/
    SpWeightsRuntimeDbWeight: {
        read: "u64",
        write: "u64",
    },
    /**
     * Lookup199: sp_version::RuntimeVersion
     **/
    SpVersionRuntimeVersion: {
        specName: "Text",
        implName: "Text",
        authoringVersion: "u32",
        specVersion: "u32",
        implVersion: "u32",
        apis: "Vec<([u8;8],u32)>",
        transactionVersion: "u32",
        systemVersion: "u8",
    },
    /**
     * Lookup203: frame_system::pallet::Error<T>
     **/
    FrameSystemError: {
        _enum: [
            "InvalidSpecName",
            "SpecVersionNeedsToIncrease",
            "FailedToExtractRuntimeVersion",
            "NonDefaultComposite",
            "NonZeroRefCount",
            "CallFiltered",
            "MultiBlockMigrationsOngoing",
            "NothingAuthorized",
            "Unauthorized",
        ],
    },
    /**
     * Lookup205: cumulus_pallet_parachain_system::unincluded_segment::Ancestor<primitive_types::H256>
     **/
    CumulusPalletParachainSystemUnincludedSegmentAncestor: {
        usedBandwidth: "CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth",
        paraHeadHash: "Option<H256>",
        consumedGoAheadSignal: "Option<PolkadotPrimitivesV8UpgradeGoAhead>",
    },
    /**
     * Lookup206: cumulus_pallet_parachain_system::unincluded_segment::UsedBandwidth
     **/
    CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth: {
        umpMsgCount: "u32",
        umpTotalBytes: "u32",
        hrmpOutgoing: "BTreeMap<u32, CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate>",
    },
    /**
     * Lookup208: cumulus_pallet_parachain_system::unincluded_segment::HrmpChannelUpdate
     **/
    CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate: {
        msgCount: "u32",
        totalBytes: "u32",
    },
    /**
     * Lookup213: polkadot_primitives::v8::UpgradeGoAhead
     **/
    PolkadotPrimitivesV8UpgradeGoAhead: {
        _enum: ["Abort", "GoAhead"],
    },
    /**
     * Lookup214: cumulus_pallet_parachain_system::unincluded_segment::SegmentTracker<primitive_types::H256>
     **/
    CumulusPalletParachainSystemUnincludedSegmentSegmentTracker: {
        usedBandwidth: "CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth",
        hrmpWatermark: "Option<u32>",
        consumedGoAheadSignal: "Option<PolkadotPrimitivesV8UpgradeGoAhead>",
    },
    /**
     * Lookup215: polkadot_primitives::v8::PersistedValidationData<primitive_types::H256, N>
     **/
    PolkadotPrimitivesV8PersistedValidationData: {
        parentHead: "Bytes",
        relayParentNumber: "u32",
        relayParentStorageRoot: "H256",
        maxPovSize: "u32",
    },
    /**
     * Lookup218: polkadot_primitives::v8::UpgradeRestriction
     **/
    PolkadotPrimitivesV8UpgradeRestriction: {
        _enum: ["Present"],
    },
    /**
     * Lookup219: sp_trie::storage_proof::StorageProof
     **/
    SpTrieStorageProof: {
        trieNodes: "BTreeSet<Bytes>",
    },
    /**
     * Lookup221: cumulus_pallet_parachain_system::relay_state_snapshot::MessagingStateSnapshot
     **/
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: {
        dmqMqcHead: "H256",
        relayDispatchQueueRemainingCapacity:
            "CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity",
        ingressChannels: "Vec<(u32,PolkadotPrimitivesV8AbridgedHrmpChannel)>",
        egressChannels: "Vec<(u32,PolkadotPrimitivesV8AbridgedHrmpChannel)>",
    },
    /**
     * Lookup222: cumulus_pallet_parachain_system::relay_state_snapshot::RelayDispatchQueueRemainingCapacity
     **/
    CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity: {
        remainingCount: "u32",
        remainingSize: "u32",
    },
    /**
     * Lookup225: polkadot_primitives::v8::AbridgedHrmpChannel
     **/
    PolkadotPrimitivesV8AbridgedHrmpChannel: {
        maxCapacity: "u32",
        maxTotalSize: "u32",
        maxMessageSize: "u32",
        msgCount: "u32",
        totalSize: "u32",
        mqcHead: "Option<H256>",
    },
    /**
     * Lookup226: polkadot_primitives::v8::AbridgedHostConfiguration
     **/
    PolkadotPrimitivesV8AbridgedHostConfiguration: {
        maxCodeSize: "u32",
        maxHeadDataSize: "u32",
        maxUpwardQueueCount: "u32",
        maxUpwardQueueSize: "u32",
        maxUpwardMessageSize: "u32",
        maxUpwardMessageNumPerCandidate: "u32",
        hrmpMaxMessageNumPerCandidate: "u32",
        validationUpgradeCooldown: "u32",
        validationUpgradeDelay: "u32",
        asyncBackingParams: "PolkadotPrimitivesV8AsyncBackingAsyncBackingParams",
    },
    /**
     * Lookup227: polkadot_primitives::v8::async_backing::AsyncBackingParams
     **/
    PolkadotPrimitivesV8AsyncBackingAsyncBackingParams: {
        maxCandidateDepth: "u32",
        allowedAncestryLen: "u32",
    },
    /**
     * Lookup233: polkadot_core_primitives::OutboundHrmpMessage<polkadot_parachain_primitives::primitives::Id>
     **/
    PolkadotCorePrimitivesOutboundHrmpMessage: {
        recipient: "u32",
        data: "Bytes",
    },
    /**
     * Lookup234: cumulus_pallet_parachain_system::pallet::Call<T>
     **/
    CumulusPalletParachainSystemCall: {
        _enum: {
            set_validation_data: {
                data: "CumulusPrimitivesParachainInherentParachainInherentData",
            },
            sudo_send_upward_message: {
                message: "Bytes",
            },
        },
    },
    /**
     * Lookup235: cumulus_primitives_parachain_inherent::ParachainInherentData
     **/
    CumulusPrimitivesParachainInherentParachainInherentData: {
        validationData: "PolkadotPrimitivesV8PersistedValidationData",
        relayChainState: "SpTrieStorageProof",
        downwardMessages: "Vec<PolkadotCorePrimitivesInboundDownwardMessage>",
        horizontalMessages: "BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>",
    },
    /**
     * Lookup237: polkadot_core_primitives::InboundDownwardMessage<BlockNumber>
     **/
    PolkadotCorePrimitivesInboundDownwardMessage: {
        sentAt: "u32",
        msg: "Bytes",
    },
    /**
     * Lookup240: polkadot_core_primitives::InboundHrmpMessage<BlockNumber>
     **/
    PolkadotCorePrimitivesInboundHrmpMessage: {
        sentAt: "u32",
        data: "Bytes",
    },
    /**
     * Lookup243: cumulus_pallet_parachain_system::pallet::Error<T>
     **/
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
    /**
     * Lookup244: pallet_timestamp::pallet::Call<T>
     **/
    PalletTimestampCall: {
        _enum: {
            set: {
                now: "Compact<u64>",
            },
        },
    },
    /**
     * Lookup245: staging_parachain_info::pallet::Call<T>
     **/
    StagingParachainInfoCall: "Null",
    /**
     * Lookup246: pallet_sudo::pallet::Call<T>
     **/
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
            remove_key: "Null",
        },
    },
    /**
     * Lookup248: pallet_utility::pallet::Call<T>
     **/
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
    /**
     * Lookup250: dancebox_runtime::OriginCaller
     **/
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
    /**
     * Lookup251: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
     **/
    FrameSupportDispatchRawOrigin: {
        _enum: {
            Root: "Null",
            Signed: "AccountId32",
            None: "Null",
        },
    },
    /**
     * Lookup252: cumulus_pallet_xcm::pallet::Origin
     **/
    CumulusPalletXcmOrigin: {
        _enum: {
            Relay: "Null",
            SiblingParachain: "u32",
        },
    },
    /**
     * Lookup253: pallet_xcm::pallet::Origin
     **/
    PalletXcmOrigin: {
        _enum: {
            Xcm: "StagingXcmV5Location",
            Response: "StagingXcmV5Location",
        },
    },
    /**
     * Lookup254: sp_core::Void
     **/
    SpCoreVoid: "Null",
    /**
     * Lookup255: pallet_proxy::pallet::Call<T>
     **/
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
    /**
     * Lookup259: pallet_migrations::pallet::Call<T>
     **/
    PalletMigrationsCall: {
        _enum: {
            force_set_cursor: {
                cursor: "Option<PalletMigrationsMigrationCursor>",
            },
            force_set_active_cursor: {
                index: "u32",
                innerCursor: "Option<Bytes>",
                startedAt: "Option<u32>",
            },
            force_onboard_mbms: "Null",
            clear_historic: {
                selector: "PalletMigrationsHistoricCleanupSelector",
            },
        },
    },
    /**
     * Lookup261: pallet_migrations::MigrationCursor<bounded_collections::bounded_vec::BoundedVec<T, S>, BlockNumber>
     **/
    PalletMigrationsMigrationCursor: {
        _enum: {
            Active: "PalletMigrationsActiveCursor",
            Stuck: "Null",
        },
    },
    /**
     * Lookup263: pallet_migrations::ActiveCursor<bounded_collections::bounded_vec::BoundedVec<T, S>, BlockNumber>
     **/
    PalletMigrationsActiveCursor: {
        index: "u32",
        innerCursor: "Option<Bytes>",
        startedAt: "u32",
    },
    /**
     * Lookup265: pallet_migrations::HistoricCleanupSelector<bounded_collections::bounded_vec::BoundedVec<T, S>>
     **/
    PalletMigrationsHistoricCleanupSelector: {
        _enum: {
            Specific: "Vec<Bytes>",
            Wildcard: {
                limit: "Option<u32>",
                previousCursor: "Option<Bytes>",
            },
        },
    },
    /**
     * Lookup267: pallet_maintenance_mode::pallet::Call<T>
     **/
    PalletMaintenanceModeCall: {
        _enum: ["enter_maintenance_mode", "resume_normal_operation"],
    },
    /**
     * Lookup268: pallet_tx_pause::pallet::Call<T>
     **/
    PalletTxPauseCall: {
        _enum: {
            pause: {
                fullName: "(Bytes,Bytes)",
            },
            unpause: {
                ident: "(Bytes,Bytes)",
            },
        },
    },
    /**
     * Lookup269: pallet_balances::pallet::Call<T, I>
     **/
    PalletBalancesCall: {
        _enum: {
            transfer_allow_death: {
                dest: "MultiAddress",
                value: "Compact<u128>",
            },
            __Unused1: "Null",
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
            __Unused7: "Null",
            force_set_balance: {
                who: "MultiAddress",
                newFree: "Compact<u128>",
            },
            force_adjust_total_issuance: {
                direction: "PalletBalancesAdjustmentDirection",
                delta: "Compact<u128>",
            },
            burn: {
                value: "Compact<u128>",
                keepAlive: "bool",
            },
        },
    },
    /**
     * Lookup271: pallet_balances::types::AdjustmentDirection
     **/
    PalletBalancesAdjustmentDirection: {
        _enum: ["Increase", "Decrease"],
    },
    /**
     * Lookup272: pallet_stream_payment::pallet::Call<T>
     **/
    PalletStreamPaymentCall: {
        _enum: {
            open_stream: {
                target: "AccountId32",
                config: "PalletStreamPaymentStreamConfig",
                initialDeposit: "u128",
            },
            close_stream: {
                streamId: "u64",
            },
            perform_payment: {
                streamId: "u64",
            },
            request_change: {
                streamId: "u64",
                kind: "PalletStreamPaymentChangeKind",
                newConfig: "PalletStreamPaymentStreamConfig",
                depositChange: "Option<PalletStreamPaymentDepositChange>",
            },
            accept_requested_change: {
                streamId: "u64",
                requestNonce: "u32",
                depositChange: "Option<PalletStreamPaymentDepositChange>",
            },
            cancel_change_request: {
                streamId: "u64",
            },
            immediately_change_deposit: {
                streamId: "u64",
                assetId: "TpStreamPaymentCommonAssetId",
                change: "PalletStreamPaymentDepositChange",
            },
        },
    },
    /**
     * Lookup273: pallet_stream_payment::pallet::ChangeKind<Time>
     **/
    PalletStreamPaymentChangeKind: {
        _enum: {
            Suggestion: "Null",
            Mandatory: {
                deadline: "u128",
            },
        },
    },
    /**
     * Lookup274: pallet_identity::pallet::Call<T>
     **/
    PalletIdentityCall: {
        _enum: {
            add_registrar: {
                account: "MultiAddress",
            },
            set_identity: {
                info: "PalletIdentityLegacyIdentityInfo",
            },
            set_subs: {
                subs: "Vec<(AccountId32,Data)>",
            },
            clear_identity: "Null",
            request_judgement: {
                regIndex: "Compact<u32>",
                maxFee: "Compact<u128>",
            },
            cancel_request: {
                regIndex: "u32",
            },
            set_fee: {
                index: "Compact<u32>",
                fee: "Compact<u128>",
            },
            set_account_id: {
                _alias: {
                    new_: "new",
                },
                index: "Compact<u32>",
                new_: "MultiAddress",
            },
            set_fields: {
                index: "Compact<u32>",
                fields: "u64",
            },
            provide_judgement: {
                regIndex: "Compact<u32>",
                target: "MultiAddress",
                judgement: "PalletIdentityJudgement",
                identity: "H256",
            },
            kill_identity: {
                target: "MultiAddress",
            },
            add_sub: {
                sub: "MultiAddress",
                data: "Data",
            },
            rename_sub: {
                sub: "MultiAddress",
                data: "Data",
            },
            remove_sub: {
                sub: "MultiAddress",
            },
            quit_sub: "Null",
            add_username_authority: {
                authority: "MultiAddress",
                suffix: "Bytes",
                allocation: "u32",
            },
            remove_username_authority: {
                suffix: "Bytes",
                authority: "MultiAddress",
            },
            set_username_for: {
                who: "MultiAddress",
                username: "Bytes",
                signature: "Option<SpRuntimeMultiSignature>",
                useAllocation: "bool",
            },
            accept_username: {
                username: "Bytes",
            },
            remove_expired_approval: {
                username: "Bytes",
            },
            set_primary_username: {
                username: "Bytes",
            },
            unbind_username: {
                username: "Bytes",
            },
            remove_username: {
                username: "Bytes",
            },
            kill_username: {
                username: "Bytes",
            },
        },
    },
    /**
     * Lookup275: pallet_identity::legacy::IdentityInfo<FieldLimit>
     **/
    PalletIdentityLegacyIdentityInfo: {
        additional: "Vec<(Data,Data)>",
        display: "Data",
        legal: "Data",
        web: "Data",
        riot: "Data",
        email: "Data",
        pgpFingerprint: "Option<[u8;20]>",
        image: "Data",
        twitter: "Data",
    },
    /**
     * Lookup311: pallet_identity::types::Judgement<Balance>
     **/
    PalletIdentityJudgement: {
        _enum: {
            Unknown: "Null",
            FeePaid: "u128",
            Reasonable: "Null",
            KnownGood: "Null",
            OutOfDate: "Null",
            LowQuality: "Null",
            Erroneous: "Null",
        },
    },
    /**
     * Lookup313: sp_runtime::MultiSignature
     **/
    SpRuntimeMultiSignature: {
        _enum: {
            Ed25519: "[u8;64]",
            Sr25519: "[u8;64]",
            Ecdsa: "[u8;65]",
        },
    },
    /**
     * Lookup316: pallet_multisig::pallet::Call<T>
     **/
    PalletMultisigCall: {
        _enum: {
            as_multi_threshold_1: {
                otherSignatories: "Vec<AccountId32>",
                call: "Call",
            },
            as_multi: {
                threshold: "u16",
                otherSignatories: "Vec<AccountId32>",
                maybeTimepoint: "Option<PalletMultisigTimepoint>",
                call: "Call",
                maxWeight: "SpWeightsWeightV2Weight",
            },
            approve_as_multi: {
                threshold: "u16",
                otherSignatories: "Vec<AccountId32>",
                maybeTimepoint: "Option<PalletMultisigTimepoint>",
                callHash: "[u8;32]",
                maxWeight: "SpWeightsWeightV2Weight",
            },
            cancel_as_multi: {
                threshold: "u16",
                otherSignatories: "Vec<AccountId32>",
                timepoint: "PalletMultisigTimepoint",
                callHash: "[u8;32]",
            },
        },
    },
    /**
     * Lookup318: pallet_registrar::pallet::Call<T>
     **/
    PalletRegistrarCall: {
        _enum: {
            register: {
                paraId: "u32",
                genesisData: "DpContainerChainGenesisDataContainerChainGenesisData",
                headData: "Option<Bytes>",
            },
            deregister: {
                paraId: "u32",
            },
            mark_valid_for_collating: {
                paraId: "u32",
            },
            __Unused3: "Null",
            pause_container_chain: {
                paraId: "u32",
            },
            unpause_container_chain: {
                paraId: "u32",
            },
            register_parathread: {
                paraId: "u32",
                slotFrequency: "TpTraitsSlotFrequency",
                genesisData: "DpContainerChainGenesisDataContainerChainGenesisData",
                headData: "Option<Bytes>",
            },
            set_parathread_params: {
                paraId: "u32",
                slotFrequency: "TpTraitsSlotFrequency",
            },
            set_para_manager: {
                paraId: "u32",
                managerAddress: "AccountId32",
            },
            register_with_relay_proof: {
                paraId: "u32",
                parathreadParams: "Option<TpTraitsParathreadParams>",
                relayProofBlockNumber: "u32",
                relayStorageProof: "SpTrieStorageProof",
                managerSignature: "SpRuntimeMultiSignature",
                genesisData: "DpContainerChainGenesisDataContainerChainGenesisData",
                headData: "Option<Bytes>",
            },
            deregister_with_relay_proof: {
                paraId: "u32",
                relayProofBlockNumber: "u32",
                relayStorageProof: "SpTrieStorageProof",
            },
        },
    },
    /**
     * Lookup319: dp_container_chain_genesis_data::ContainerChainGenesisData
     **/
    DpContainerChainGenesisDataContainerChainGenesisData: {
        storage: "Vec<DpContainerChainGenesisDataContainerChainGenesisDataItem>",
        name: "Bytes",
        id: "Bytes",
        forkId: "Option<Bytes>",
        extensions: "Bytes",
        properties: "DpContainerChainGenesisDataProperties",
    },
    /**
     * Lookup321: dp_container_chain_genesis_data::ContainerChainGenesisDataItem
     **/
    DpContainerChainGenesisDataContainerChainGenesisDataItem: {
        key: "Bytes",
        value: "Bytes",
    },
    /**
     * Lookup324: dp_container_chain_genesis_data::Properties
     **/
    DpContainerChainGenesisDataProperties: {
        tokenMetadata: "DpContainerChainGenesisDataTokenMetadata",
        isEthereum: "bool",
    },
    /**
     * Lookup325: dp_container_chain_genesis_data::TokenMetadata
     **/
    DpContainerChainGenesisDataTokenMetadata: {
        tokenSymbol: "Bytes",
        ss58Format: "u32",
        tokenDecimals: "u32",
    },
    /**
     * Lookup328: tp_traits::SlotFrequency
     **/
    TpTraitsSlotFrequency: {
        min: "u32",
        max: "u32",
    },
    /**
     * Lookup330: tp_traits::ParathreadParams
     **/
    TpTraitsParathreadParams: {
        slotFrequency: "TpTraitsSlotFrequency",
    },
    /**
     * Lookup331: pallet_configuration::pallet::Call<T>
     **/
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
            set_collators_per_parathread: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_parathreads_per_collator: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_target_container_chain_fullness: {
                _alias: {
                    new_: "new",
                },
                new_: "Perbill",
            },
            set_max_parachain_cores_percentage: {
                _alias: {
                    new_: "new",
                },
                new_: "Option<Perbill>",
            },
            set_full_rotation_mode: {
                orchestrator: "Option<TpTraitsFullRotationMode>",
                parachain: "Option<TpTraitsFullRotationMode>",
                parathread: "Option<TpTraitsFullRotationMode>",
            },
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
    /**
     * Lookup334: pallet_collator_assignment::pallet::Call<T>
     **/
    PalletCollatorAssignmentCall: "Null",
    /**
     * Lookup335: pallet_author_noting::pallet::Call<T>
     **/
    PalletAuthorNotingCall: {
        _enum: {
            set_latest_author_data: {
                data: "TpAuthorNotingInherentOwnParachainInherentData",
            },
            set_author: {
                paraId: "u32",
                blockNumber: "u32",
                author: "AccountId32",
                latestSlotNumber: "u64",
            },
            kill_author_data: {
                paraId: "u32",
            },
        },
    },
    /**
     * Lookup336: tp_author_noting_inherent::OwnParachainInherentData
     **/
    TpAuthorNotingInherentOwnParachainInherentData: {
        relayStorageProof: "SpTrieStorageProof",
    },
    /**
     * Lookup337: pallet_authority_assignment::pallet::Call<T>
     **/
    PalletAuthorityAssignmentCall: "Null",
    /**
     * Lookup338: pallet_services_payment::pallet::Call<T>
     **/
    PalletServicesPaymentCall: {
        _enum: {
            purchase_credits: {
                paraId: "u32",
                credit: "u128",
            },
            set_block_production_credits: {
                paraId: "u32",
                freeBlockCredits: "u32",
            },
            set_given_free_credits: {
                paraId: "u32",
                givenFreeCredits: "bool",
            },
            set_refund_address: {
                paraId: "u32",
                refundAddress: "Option<AccountId32>",
            },
            set_collator_assignment_credits: {
                paraId: "u32",
                freeCollatorAssignmentCredits: "u32",
            },
            set_max_core_price: {
                paraId: "u32",
                maxCorePrice: "u128",
            },
            set_max_tip: {
                paraId: "u32",
                maxTip: "Option<u128>",
            },
        },
    },
    /**
     * Lookup340: pallet_data_preservers::pallet::Call<T>
     **/
    PalletDataPreserversCall: {
        _enum: {
            __Unused0: "Null",
            create_profile: {
                profile: "PalletDataPreserversProfile",
            },
            update_profile: {
                profileId: "u64",
                profile: "PalletDataPreserversProfile",
            },
            delete_profile: {
                profileId: "u64",
            },
            force_create_profile: {
                profile: "PalletDataPreserversProfile",
                forAccount: "AccountId32",
            },
            force_update_profile: {
                profileId: "u64",
                profile: "PalletDataPreserversProfile",
            },
            force_delete_profile: {
                profileId: "u64",
            },
            start_assignment: {
                profileId: "u64",
                paraId: "u32",
                assignerParam: "TpDataPreserversCommonAssignerExtra",
            },
            stop_assignment: {
                profileId: "u64",
                paraId: "u32",
            },
            force_start_assignment: {
                profileId: "u64",
                paraId: "u32",
                assignmentWitness: "TpDataPreserversCommonAssignmentWitness",
            },
        },
    },
    /**
     * Lookup341: pallet_data_preservers::types::Profile<T>
     **/
    PalletDataPreserversProfile: {
        url: "Bytes",
        paraIds: "PalletDataPreserversParaIdsFilter",
        mode: "PalletDataPreserversProfileMode",
        assignmentRequest: "TpDataPreserversCommonProviderRequest",
    },
    /**
     * Lookup343: pallet_data_preservers::types::ParaIdsFilter<T>
     **/
    PalletDataPreserversParaIdsFilter: {
        _enum: {
            AnyParaId: "Null",
            Whitelist: "BTreeSet<u32>",
            Blacklist: "BTreeSet<u32>",
        },
    },
    /**
     * Lookup346: pallet_data_preservers::types::ProfileMode
     **/
    PalletDataPreserversProfileMode: {
        _enum: {
            Bootnode: "Null",
            Rpc: {
                supportsEthereumRpcs: "bool",
            },
        },
    },
    /**
     * Lookup347: tp_data_preservers_common::ProviderRequest
     **/
    TpDataPreserversCommonProviderRequest: {
        _enum: {
            Free: "Null",
            StreamPayment: {
                config: "PalletStreamPaymentStreamConfig",
            },
        },
    },
    /**
     * Lookup348: tp_data_preservers_common::AssignerExtra
     **/
    TpDataPreserversCommonAssignerExtra: {
        _enum: {
            Free: "Null",
            StreamPayment: {
                initialDeposit: "u128",
            },
        },
    },
    /**
     * Lookup349: tp_data_preservers_common::AssignmentWitness
     **/
    TpDataPreserversCommonAssignmentWitness: {
        _enum: {
            Free: "Null",
            StreamPayment: {
                streamId: "u64",
            },
        },
    },
    /**
     * Lookup350: pallet_invulnerables::pallet::Call<T>
     **/
    PalletInvulnerablesCall: {
        _enum: {
            __Unused0: "Null",
            add_invulnerable: {
                who: "AccountId32",
            },
            remove_invulnerable: {
                who: "AccountId32",
            },
        },
    },
    /**
     * Lookup351: pallet_session::pallet::Call<T>
     **/
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
    /**
     * Lookup352: dancebox_runtime::SessionKeys
     **/
    DanceboxRuntimeSessionKeys: {
        nimbus: "NimbusPrimitivesNimbusCryptoPublic",
    },
    /**
     * Lookup353: nimbus_primitives::nimbus_crypto::Public
     **/
    NimbusPrimitivesNimbusCryptoPublic: "[u8;32]",
    /**
     * Lookup354: pallet_author_inherent::pallet::Call<T>
     **/
    PalletAuthorInherentCall: {
        _enum: ["kick_off_authorship_validation"],
    },
    /**
     * Lookup355: pallet_pooled_staking::pallet::Call<T>
     **/
    PalletPooledStakingCall: {
        _enum: {
            rebalance_hold: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                pool: "PalletPooledStakingPoolsPoolKind",
            },
            request_delegate: {
                candidate: "AccountId32",
                pool: "PalletPooledStakingPoolsActivePoolKind",
                stake: "u128",
            },
            execute_pending_operations: {
                operations: "Vec<PalletPooledStakingPendingOperationQuery>",
            },
            request_undelegate: {
                candidate: "AccountId32",
                pool: "PalletPooledStakingPoolsActivePoolKind",
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
                sourcePool: "PalletPooledStakingPoolsActivePoolKind",
                amount: "PalletPooledStakingSharesOrStake",
            },
        },
    },
    /**
     * Lookup356: pallet_pooled_staking::pools::PoolKind
     **/
    PalletPooledStakingPoolsPoolKind: {
        _enum: ["Joining", "AutoCompounding", "ManualRewards", "Leaving"],
    },
    /**
     * Lookup358: pallet_pooled_staking::pallet::PendingOperationQuery<sp_core::crypto::AccountId32, J, L>
     **/
    PalletPooledStakingPendingOperationQuery: {
        delegator: "AccountId32",
        operation: "PalletPooledStakingPendingOperationKey",
    },
    /**
     * Lookup359: pallet_pooled_staking::pallet::PendingOperationKey<sp_core::crypto::AccountId32, J, L>
     **/
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
    /**
     * Lookup360: pallet_pooled_staking::pallet::SharesOrStake<T>
     **/
    PalletPooledStakingSharesOrStake: {
        _enum: {
            Shares: "u128",
            Stake: "u128",
        },
    },
    /**
     * Lookup363: pallet_treasury::pallet::Call<T, I>
     **/
    PalletTreasuryCall: {
        _enum: {
            __Unused0: "Null",
            __Unused1: "Null",
            __Unused2: "Null",
            spend_local: {
                amount: "Compact<u128>",
                beneficiary: "MultiAddress",
            },
            remove_approval: {
                proposalId: "Compact<u32>",
            },
            spend: {
                assetKind: "Null",
                amount: "Compact<u128>",
                beneficiary: "AccountId32",
                validFrom: "Option<u32>",
            },
            payout: {
                index: "u32",
            },
            check_status: {
                index: "u32",
            },
            void_spend: {
                index: "u32",
            },
        },
    },
    /**
     * Lookup364: cumulus_pallet_xcmp_queue::pallet::Call<T>
     **/
    CumulusPalletXcmpQueueCall: {
        _enum: {
            __Unused0: "Null",
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
        },
    },
    /**
     * Lookup365: pallet_xcm::pallet::Call<T>
     **/
    PalletXcmCall: {
        _enum: {
            send: {
                dest: "XcmVersionedLocation",
                message: "XcmVersionedXcm",
            },
            teleport_assets: {
                dest: "XcmVersionedLocation",
                beneficiary: "XcmVersionedLocation",
                assets: "XcmVersionedAssets",
                feeAssetItem: "u32",
            },
            reserve_transfer_assets: {
                dest: "XcmVersionedLocation",
                beneficiary: "XcmVersionedLocation",
                assets: "XcmVersionedAssets",
                feeAssetItem: "u32",
            },
            execute: {
                message: "XcmVersionedXcm",
                maxWeight: "SpWeightsWeightV2Weight",
            },
            force_xcm_version: {
                location: "StagingXcmV5Location",
                version: "u32",
            },
            force_default_xcm_version: {
                maybeXcmVersion: "Option<u32>",
            },
            force_subscribe_version_notify: {
                location: "XcmVersionedLocation",
            },
            force_unsubscribe_version_notify: {
                location: "XcmVersionedLocation",
            },
            limited_reserve_transfer_assets: {
                dest: "XcmVersionedLocation",
                beneficiary: "XcmVersionedLocation",
                assets: "XcmVersionedAssets",
                feeAssetItem: "u32",
                weightLimit: "XcmV3WeightLimit",
            },
            limited_teleport_assets: {
                dest: "XcmVersionedLocation",
                beneficiary: "XcmVersionedLocation",
                assets: "XcmVersionedAssets",
                feeAssetItem: "u32",
                weightLimit: "XcmV3WeightLimit",
            },
            force_suspension: {
                suspended: "bool",
            },
            transfer_assets: {
                dest: "XcmVersionedLocation",
                beneficiary: "XcmVersionedLocation",
                assets: "XcmVersionedAssets",
                feeAssetItem: "u32",
                weightLimit: "XcmV3WeightLimit",
            },
            claim_assets: {
                assets: "XcmVersionedAssets",
                beneficiary: "XcmVersionedLocation",
            },
            transfer_assets_using_type_and_then: {
                dest: "XcmVersionedLocation",
                assets: "XcmVersionedAssets",
                assetsTransferType: "StagingXcmExecutorAssetTransferTransferType",
                remoteFeesId: "XcmVersionedAssetId",
                feesTransferType: "StagingXcmExecutorAssetTransferTransferType",
                customXcmOnDest: "XcmVersionedXcm",
                weightLimit: "XcmV3WeightLimit",
            },
        },
    },
    /**
     * Lookup366: xcm::VersionedXcm<RuntimeCall>
     **/
    XcmVersionedXcm: {
        _enum: {
            __Unused0: "Null",
            __Unused1: "Null",
            __Unused2: "Null",
            V3: "XcmV3Xcm",
            V4: "StagingXcmV4Xcm",
            V5: "StagingXcmV5Xcm",
        },
    },
    /**
     * Lookup367: xcm::v3::Xcm<Call>
     **/
    XcmV3Xcm: "Vec<XcmV3Instruction>",
    /**
     * Lookup369: xcm::v3::Instruction<Call>
     **/
    XcmV3Instruction: {
        _enum: {
            WithdrawAsset: "XcmV3MultiassetMultiAssets",
            ReserveAssetDeposited: "XcmV3MultiassetMultiAssets",
            ReceiveTeleportedAsset: "XcmV3MultiassetMultiAssets",
            QueryResponse: {
                queryId: "Compact<u64>",
                response: "XcmV3Response",
                maxWeight: "SpWeightsWeightV2Weight",
                querier: "Option<StagingXcmV3MultiLocation>",
            },
            TransferAsset: {
                assets: "XcmV3MultiassetMultiAssets",
                beneficiary: "StagingXcmV3MultiLocation",
            },
            TransferReserveAsset: {
                assets: "XcmV3MultiassetMultiAssets",
                dest: "StagingXcmV3MultiLocation",
                xcm: "XcmV3Xcm",
            },
            Transact: {
                originKind: "XcmV3OriginKind",
                requireWeightAtMost: "SpWeightsWeightV2Weight",
                call: "XcmDoubleEncoded",
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
            DescendOrigin: "XcmV3Junctions",
            ReportError: "XcmV3QueryResponseInfo",
            DepositAsset: {
                assets: "XcmV3MultiassetMultiAssetFilter",
                beneficiary: "StagingXcmV3MultiLocation",
            },
            DepositReserveAsset: {
                assets: "XcmV3MultiassetMultiAssetFilter",
                dest: "StagingXcmV3MultiLocation",
                xcm: "XcmV3Xcm",
            },
            ExchangeAsset: {
                give: "XcmV3MultiassetMultiAssetFilter",
                want: "XcmV3MultiassetMultiAssets",
                maximal: "bool",
            },
            InitiateReserveWithdraw: {
                assets: "XcmV3MultiassetMultiAssetFilter",
                reserve: "StagingXcmV3MultiLocation",
                xcm: "XcmV3Xcm",
            },
            InitiateTeleport: {
                assets: "XcmV3MultiassetMultiAssetFilter",
                dest: "StagingXcmV3MultiLocation",
                xcm: "XcmV3Xcm",
            },
            ReportHolding: {
                responseInfo: "XcmV3QueryResponseInfo",
                assets: "XcmV3MultiassetMultiAssetFilter",
            },
            BuyExecution: {
                fees: "XcmV3MultiAsset",
                weightLimit: "XcmV3WeightLimit",
            },
            RefundSurplus: "Null",
            SetErrorHandler: "XcmV3Xcm",
            SetAppendix: "XcmV3Xcm",
            ClearError: "Null",
            ClaimAsset: {
                assets: "XcmV3MultiassetMultiAssets",
                ticket: "StagingXcmV3MultiLocation",
            },
            Trap: "Compact<u64>",
            SubscribeVersion: {
                queryId: "Compact<u64>",
                maxResponseWeight: "SpWeightsWeightV2Weight",
            },
            UnsubscribeVersion: "Null",
            BurnAsset: "XcmV3MultiassetMultiAssets",
            ExpectAsset: "XcmV3MultiassetMultiAssets",
            ExpectOrigin: "Option<StagingXcmV3MultiLocation>",
            ExpectError: "Option<(u32,XcmV3TraitsError)>",
            ExpectTransactStatus: "XcmV3MaybeErrorCode",
            QueryPallet: {
                moduleName: "Bytes",
                responseInfo: "XcmV3QueryResponseInfo",
            },
            ExpectPallet: {
                index: "Compact<u32>",
                name: "Bytes",
                moduleName: "Bytes",
                crateMajor: "Compact<u32>",
                minCrateMinor: "Compact<u32>",
            },
            ReportTransactStatus: "XcmV3QueryResponseInfo",
            ClearTransactStatus: "Null",
            UniversalOrigin: "XcmV3Junction",
            ExportMessage: {
                network: "XcmV3JunctionNetworkId",
                destination: "XcmV3Junctions",
                xcm: "XcmV3Xcm",
            },
            LockAsset: {
                asset: "XcmV3MultiAsset",
                unlocker: "StagingXcmV3MultiLocation",
            },
            UnlockAsset: {
                asset: "XcmV3MultiAsset",
                target: "StagingXcmV3MultiLocation",
            },
            NoteUnlockable: {
                asset: "XcmV3MultiAsset",
                owner: "StagingXcmV3MultiLocation",
            },
            RequestUnlock: {
                asset: "XcmV3MultiAsset",
                locker: "StagingXcmV3MultiLocation",
            },
            SetFeesMode: {
                jitWithdraw: "bool",
            },
            SetTopic: "[u8;32]",
            ClearTopic: "Null",
            AliasOrigin: "StagingXcmV3MultiLocation",
            UnpaidExecution: {
                weightLimit: "XcmV3WeightLimit",
                checkOrigin: "Option<StagingXcmV3MultiLocation>",
            },
        },
    },
    /**
     * Lookup370: xcm::v3::Response
     **/
    XcmV3Response: {
        _enum: {
            Null: "Null",
            Assets: "XcmV3MultiassetMultiAssets",
            ExecutionResult: "Option<(u32,XcmV3TraitsError)>",
            Version: "u32",
            PalletsInfo: "Vec<XcmV3PalletInfo>",
            DispatchResult: "XcmV3MaybeErrorCode",
        },
    },
    /**
     * Lookup373: xcm::v3::traits::Error
     **/
    XcmV3TraitsError: {
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
    /**
     * Lookup375: xcm::v3::PalletInfo
     **/
    XcmV3PalletInfo: {
        index: "Compact<u32>",
        name: "Bytes",
        moduleName: "Bytes",
        major: "Compact<u32>",
        minor: "Compact<u32>",
        patch: "Compact<u32>",
    },
    /**
     * Lookup379: xcm::v3::QueryResponseInfo
     **/
    XcmV3QueryResponseInfo: {
        destination: "StagingXcmV3MultiLocation",
        queryId: "Compact<u64>",
        maxWeight: "SpWeightsWeightV2Weight",
    },
    /**
     * Lookup380: xcm::v3::multiasset::MultiAssetFilter
     **/
    XcmV3MultiassetMultiAssetFilter: {
        _enum: {
            Definite: "XcmV3MultiassetMultiAssets",
            Wild: "XcmV3MultiassetWildMultiAsset",
        },
    },
    /**
     * Lookup381: xcm::v3::multiasset::WildMultiAsset
     **/
    XcmV3MultiassetWildMultiAsset: {
        _enum: {
            All: "Null",
            AllOf: {
                id: "XcmV3MultiassetAssetId",
                fun: "XcmV3MultiassetWildFungibility",
            },
            AllCounted: "Compact<u32>",
            AllOfCounted: {
                id: "XcmV3MultiassetAssetId",
                fun: "XcmV3MultiassetWildFungibility",
                count: "Compact<u32>",
            },
        },
    },
    /**
     * Lookup382: xcm::v3::multiasset::WildFungibility
     **/
    XcmV3MultiassetWildFungibility: {
        _enum: ["Fungible", "NonFungible"],
    },
    /**
     * Lookup383: staging_xcm::v4::Xcm<Call>
     **/
    StagingXcmV4Xcm: "Vec<StagingXcmV4Instruction>",
    /**
     * Lookup385: staging_xcm::v4::Instruction<Call>
     **/
    StagingXcmV4Instruction: {
        _enum: {
            WithdrawAsset: "StagingXcmV4AssetAssets",
            ReserveAssetDeposited: "StagingXcmV4AssetAssets",
            ReceiveTeleportedAsset: "StagingXcmV4AssetAssets",
            QueryResponse: {
                queryId: "Compact<u64>",
                response: "StagingXcmV4Response",
                maxWeight: "SpWeightsWeightV2Weight",
                querier: "Option<StagingXcmV4Location>",
            },
            TransferAsset: {
                assets: "StagingXcmV4AssetAssets",
                beneficiary: "StagingXcmV4Location",
            },
            TransferReserveAsset: {
                assets: "StagingXcmV4AssetAssets",
                dest: "StagingXcmV4Location",
                xcm: "StagingXcmV4Xcm",
            },
            Transact: {
                originKind: "XcmV3OriginKind",
                requireWeightAtMost: "SpWeightsWeightV2Weight",
                call: "XcmDoubleEncoded",
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
            DescendOrigin: "StagingXcmV4Junctions",
            ReportError: "StagingXcmV4QueryResponseInfo",
            DepositAsset: {
                assets: "StagingXcmV4AssetAssetFilter",
                beneficiary: "StagingXcmV4Location",
            },
            DepositReserveAsset: {
                assets: "StagingXcmV4AssetAssetFilter",
                dest: "StagingXcmV4Location",
                xcm: "StagingXcmV4Xcm",
            },
            ExchangeAsset: {
                give: "StagingXcmV4AssetAssetFilter",
                want: "StagingXcmV4AssetAssets",
                maximal: "bool",
            },
            InitiateReserveWithdraw: {
                assets: "StagingXcmV4AssetAssetFilter",
                reserve: "StagingXcmV4Location",
                xcm: "StagingXcmV4Xcm",
            },
            InitiateTeleport: {
                assets: "StagingXcmV4AssetAssetFilter",
                dest: "StagingXcmV4Location",
                xcm: "StagingXcmV4Xcm",
            },
            ReportHolding: {
                responseInfo: "StagingXcmV4QueryResponseInfo",
                assets: "StagingXcmV4AssetAssetFilter",
            },
            BuyExecution: {
                fees: "StagingXcmV4Asset",
                weightLimit: "XcmV3WeightLimit",
            },
            RefundSurplus: "Null",
            SetErrorHandler: "StagingXcmV4Xcm",
            SetAppendix: "StagingXcmV4Xcm",
            ClearError: "Null",
            ClaimAsset: {
                assets: "StagingXcmV4AssetAssets",
                ticket: "StagingXcmV4Location",
            },
            Trap: "Compact<u64>",
            SubscribeVersion: {
                queryId: "Compact<u64>",
                maxResponseWeight: "SpWeightsWeightV2Weight",
            },
            UnsubscribeVersion: "Null",
            BurnAsset: "StagingXcmV4AssetAssets",
            ExpectAsset: "StagingXcmV4AssetAssets",
            ExpectOrigin: "Option<StagingXcmV4Location>",
            ExpectError: "Option<(u32,XcmV3TraitsError)>",
            ExpectTransactStatus: "XcmV3MaybeErrorCode",
            QueryPallet: {
                moduleName: "Bytes",
                responseInfo: "StagingXcmV4QueryResponseInfo",
            },
            ExpectPallet: {
                index: "Compact<u32>",
                name: "Bytes",
                moduleName: "Bytes",
                crateMajor: "Compact<u32>",
                minCrateMinor: "Compact<u32>",
            },
            ReportTransactStatus: "StagingXcmV4QueryResponseInfo",
            ClearTransactStatus: "Null",
            UniversalOrigin: "StagingXcmV4Junction",
            ExportMessage: {
                network: "StagingXcmV4JunctionNetworkId",
                destination: "StagingXcmV4Junctions",
                xcm: "StagingXcmV4Xcm",
            },
            LockAsset: {
                asset: "StagingXcmV4Asset",
                unlocker: "StagingXcmV4Location",
            },
            UnlockAsset: {
                asset: "StagingXcmV4Asset",
                target: "StagingXcmV4Location",
            },
            NoteUnlockable: {
                asset: "StagingXcmV4Asset",
                owner: "StagingXcmV4Location",
            },
            RequestUnlock: {
                asset: "StagingXcmV4Asset",
                locker: "StagingXcmV4Location",
            },
            SetFeesMode: {
                jitWithdraw: "bool",
            },
            SetTopic: "[u8;32]",
            ClearTopic: "Null",
            AliasOrigin: "StagingXcmV4Location",
            UnpaidExecution: {
                weightLimit: "XcmV3WeightLimit",
                checkOrigin: "Option<StagingXcmV4Location>",
            },
        },
    },
    /**
     * Lookup386: staging_xcm::v4::Response
     **/
    StagingXcmV4Response: {
        _enum: {
            Null: "Null",
            Assets: "StagingXcmV4AssetAssets",
            ExecutionResult: "Option<(u32,XcmV3TraitsError)>",
            Version: "u32",
            PalletsInfo: "Vec<StagingXcmV4PalletInfo>",
            DispatchResult: "XcmV3MaybeErrorCode",
        },
    },
    /**
     * Lookup388: staging_xcm::v4::PalletInfo
     **/
    StagingXcmV4PalletInfo: {
        index: "Compact<u32>",
        name: "Bytes",
        moduleName: "Bytes",
        major: "Compact<u32>",
        minor: "Compact<u32>",
        patch: "Compact<u32>",
    },
    /**
     * Lookup392: staging_xcm::v4::QueryResponseInfo
     **/
    StagingXcmV4QueryResponseInfo: {
        destination: "StagingXcmV4Location",
        queryId: "Compact<u64>",
        maxWeight: "SpWeightsWeightV2Weight",
    },
    /**
     * Lookup393: staging_xcm::v4::asset::AssetFilter
     **/
    StagingXcmV4AssetAssetFilter: {
        _enum: {
            Definite: "StagingXcmV4AssetAssets",
            Wild: "StagingXcmV4AssetWildAsset",
        },
    },
    /**
     * Lookup394: staging_xcm::v4::asset::WildAsset
     **/
    StagingXcmV4AssetWildAsset: {
        _enum: {
            All: "Null",
            AllOf: {
                id: "StagingXcmV4AssetAssetId",
                fun: "StagingXcmV4AssetWildFungibility",
            },
            AllCounted: "Compact<u32>",
            AllOfCounted: {
                id: "StagingXcmV4AssetAssetId",
                fun: "StagingXcmV4AssetWildFungibility",
                count: "Compact<u32>",
            },
        },
    },
    /**
     * Lookup395: staging_xcm::v4::asset::WildFungibility
     **/
    StagingXcmV4AssetWildFungibility: {
        _enum: ["Fungible", "NonFungible"],
    },
    /**
     * Lookup407: staging_xcm_executor::traits::asset_transfer::TransferType
     **/
    StagingXcmExecutorAssetTransferTransferType: {
        _enum: {
            Teleport: "Null",
            LocalReserve: "Null",
            DestinationReserve: "Null",
            RemoteReserve: "XcmVersionedLocation",
        },
    },
    /**
     * Lookup408: xcm::VersionedAssetId
     **/
    XcmVersionedAssetId: {
        _enum: {
            __Unused0: "Null",
            __Unused1: "Null",
            __Unused2: "Null",
            V3: "XcmV3MultiassetAssetId",
            V4: "StagingXcmV4AssetAssetId",
            V5: "StagingXcmV5AssetAssetId",
        },
    },
    /**
     * Lookup409: pallet_assets::pallet::Call<T, I>
     **/
    PalletAssetsCall: {
        _enum: {
            create: {
                id: "u16",
                admin: "MultiAddress",
                minBalance: "u128",
            },
            force_create: {
                id: "u16",
                owner: "MultiAddress",
                isSufficient: "bool",
                minBalance: "Compact<u128>",
            },
            start_destroy: {
                id: "u16",
            },
            destroy_accounts: {
                id: "u16",
            },
            destroy_approvals: {
                id: "u16",
            },
            finish_destroy: {
                id: "u16",
            },
            mint: {
                id: "u16",
                beneficiary: "MultiAddress",
                amount: "Compact<u128>",
            },
            burn: {
                id: "u16",
                who: "MultiAddress",
                amount: "Compact<u128>",
            },
            transfer: {
                id: "u16",
                target: "MultiAddress",
                amount: "Compact<u128>",
            },
            transfer_keep_alive: {
                id: "u16",
                target: "MultiAddress",
                amount: "Compact<u128>",
            },
            force_transfer: {
                id: "u16",
                source: "MultiAddress",
                dest: "MultiAddress",
                amount: "Compact<u128>",
            },
            freeze: {
                id: "u16",
                who: "MultiAddress",
            },
            thaw: {
                id: "u16",
                who: "MultiAddress",
            },
            freeze_asset: {
                id: "u16",
            },
            thaw_asset: {
                id: "u16",
            },
            transfer_ownership: {
                id: "u16",
                owner: "MultiAddress",
            },
            set_team: {
                id: "u16",
                issuer: "MultiAddress",
                admin: "MultiAddress",
                freezer: "MultiAddress",
            },
            set_metadata: {
                id: "u16",
                name: "Bytes",
                symbol: "Bytes",
                decimals: "u8",
            },
            clear_metadata: {
                id: "u16",
            },
            force_set_metadata: {
                id: "u16",
                name: "Bytes",
                symbol: "Bytes",
                decimals: "u8",
                isFrozen: "bool",
            },
            force_clear_metadata: {
                id: "u16",
            },
            force_asset_status: {
                id: "u16",
                owner: "MultiAddress",
                issuer: "MultiAddress",
                admin: "MultiAddress",
                freezer: "MultiAddress",
                minBalance: "Compact<u128>",
                isSufficient: "bool",
                isFrozen: "bool",
            },
            approve_transfer: {
                id: "u16",
                delegate: "MultiAddress",
                amount: "Compact<u128>",
            },
            cancel_approval: {
                id: "u16",
                delegate: "MultiAddress",
            },
            force_cancel_approval: {
                id: "u16",
                owner: "MultiAddress",
                delegate: "MultiAddress",
            },
            transfer_approved: {
                id: "u16",
                owner: "MultiAddress",
                destination: "MultiAddress",
                amount: "Compact<u128>",
            },
            touch: {
                id: "u16",
            },
            refund: {
                id: "u16",
                allowBurn: "bool",
            },
            set_min_balance: {
                id: "u16",
                minBalance: "u128",
            },
            touch_other: {
                id: "u16",
                who: "MultiAddress",
            },
            refund_other: {
                id: "u16",
                who: "MultiAddress",
            },
            block: {
                id: "u16",
                who: "MultiAddress",
            },
            transfer_all: {
                id: "u16",
                dest: "MultiAddress",
                keepAlive: "bool",
            },
        },
    },
    /**
     * Lookup410: pallet_foreign_asset_creator::pallet::Call<T>
     **/
    PalletForeignAssetCreatorCall: {
        _enum: {
            create_foreign_asset: {
                foreignAsset: "StagingXcmV5Location",
                assetId: "u16",
                admin: "AccountId32",
                isSufficient: "bool",
                minBalance: "u128",
            },
            change_existing_asset_type: {
                assetId: "u16",
                newForeignAsset: "StagingXcmV5Location",
            },
            remove_existing_asset_type: {
                assetId: "u16",
            },
            destroy_foreign_asset: {
                assetId: "u16",
            },
        },
    },
    /**
     * Lookup411: pallet_asset_rate::pallet::Call<T>
     **/
    PalletAssetRateCall: {
        _enum: {
            create: {
                assetKind: "u16",
                rate: "u128",
            },
            update: {
                assetKind: "u16",
                rate: "u128",
            },
            remove: {
                assetKind: "u16",
            },
        },
    },
    /**
     * Lookup412: pallet_message_queue::pallet::Call<T>
     **/
    PalletMessageQueueCall: {
        _enum: {
            reap_page: {
                messageOrigin: "CumulusPrimitivesCoreAggregateMessageOrigin",
                pageIndex: "u32",
            },
            execute_overweight: {
                messageOrigin: "CumulusPrimitivesCoreAggregateMessageOrigin",
                page: "u32",
                index: "u32",
                weightLimit: "SpWeightsWeightV2Weight",
            },
        },
    },
    /**
     * Lookup413: pallet_xcm_core_buyer::pallet::Call<T>
     **/
    PalletXcmCoreBuyerCall: {
        _enum: {
            buy_core: {
                paraId: "u32",
                proof: "TpXcmCoreBuyerBuyCoreCollatorProof",
            },
            force_buy_core: {
                paraId: "u32",
            },
            set_relay_xcm_weight_config: {
                xcmWeights: "Option<PalletXcmCoreBuyerRelayXcmWeightConfigInner>",
            },
            set_relay_chain: {
                relayChain: "Option<DanceboxRuntimeXcmConfigRelayChain>",
            },
            query_response: {
                queryId: "u64",
                response: "StagingXcmV5Response",
            },
            clean_up_expired_pending_blocks: {
                expiredPendingBlocksParaId: "Vec<u32>",
            },
            clean_up_expired_in_flight_orders: {
                expiredInFlightOrders: "Vec<u32>",
            },
        },
    },
    /**
     * Lookup414: tp_xcm_core_buyer::BuyCoreCollatorProof<nimbus_primitives::nimbus_crypto::Public>
     **/
    TpXcmCoreBuyerBuyCoreCollatorProof: {
        nonce: "u64",
        publicKey: "NimbusPrimitivesNimbusCryptoPublic",
        signature: "NimbusPrimitivesNimbusCryptoSignature",
    },
    /**
     * Lookup415: nimbus_primitives::nimbus_crypto::Signature
     **/
    NimbusPrimitivesNimbusCryptoSignature: "[u8;64]",
    /**
     * Lookup417: pallet_xcm_core_buyer::pallet::RelayXcmWeightConfigInner<T>
     **/
    PalletXcmCoreBuyerRelayXcmWeightConfigInner: {
        buyExecutionCost: "u128",
        weightAtMost: "SpWeightsWeightV2Weight",
    },
    /**
     * Lookup419: dancebox_runtime::xcm_config::RelayChain
     **/
    DanceboxRuntimeXcmConfigRelayChain: {
        _enum: ["Westend", "Rococo"],
    },
    /**
     * Lookup420: pallet_root_testing::pallet::Call<T>
     **/
    PalletRootTestingCall: {
        _enum: {
            fill_block: {
                ratio: "Perbill",
            },
            trigger_defensive: "Null",
        },
    },
    /**
     * Lookup421: pallet_sudo::pallet::Error<T>
     **/
    PalletSudoError: {
        _enum: ["RequireSudo"],
    },
    /**
     * Lookup422: pallet_utility::pallet::Error<T>
     **/
    PalletUtilityError: {
        _enum: ["TooManyCalls"],
    },
    /**
     * Lookup425: pallet_proxy::ProxyDefinition<sp_core::crypto::AccountId32, dancebox_runtime::ProxyType, BlockNumber>
     **/
    PalletProxyProxyDefinition: {
        delegate: "AccountId32",
        proxyType: "DanceboxRuntimeProxyType",
        delay: "u32",
    },
    /**
     * Lookup429: pallet_proxy::Announcement<sp_core::crypto::AccountId32, primitive_types::H256, BlockNumber>
     **/
    PalletProxyAnnouncement: {
        real: "AccountId32",
        callHash: "H256",
        height: "u32",
    },
    /**
     * Lookup431: pallet_proxy::pallet::Error<T>
     **/
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
    /**
     * Lookup432: pallet_migrations::pallet::Error<T>
     **/
    PalletMigrationsError: {
        _enum: ["PreimageMissing", "WrongUpperBound", "PreimageIsTooBig", "PreimageAlreadyExists"],
    },
    /**
     * Lookup434: pallet_maintenance_mode::pallet::Error<T>
     **/
    PalletMaintenanceModeError: {
        _enum: ["AlreadyInMaintenanceMode", "NotInMaintenanceMode"],
    },
    /**
     * Lookup435: pallet_tx_pause::pallet::Error<T>
     **/
    PalletTxPauseError: {
        _enum: ["IsPaused", "IsUnpaused", "Unpausable", "NotFound"],
    },
    /**
     * Lookup437: pallet_balances::types::BalanceLock<Balance>
     **/
    PalletBalancesBalanceLock: {
        id: "[u8;8]",
        amount: "u128",
        reasons: "PalletBalancesReasons",
    },
    /**
     * Lookup438: pallet_balances::types::Reasons
     **/
    PalletBalancesReasons: {
        _enum: ["Fee", "Misc", "All"],
    },
    /**
     * Lookup441: pallet_balances::types::ReserveData<ReserveIdentifier, Balance>
     **/
    PalletBalancesReserveData: {
        id: "[u8;8]",
        amount: "u128",
    },
    /**
     * Lookup444: frame_support::traits::tokens::misc::IdAmount<dancebox_runtime::RuntimeHoldReason, Balance>
     **/
    FrameSupportTokensMiscIdAmountRuntimeHoldReason: {
        id: "DanceboxRuntimeRuntimeHoldReason",
        amount: "u128",
    },
    /**
     * Lookup445: dancebox_runtime::RuntimeHoldReason
     **/
    DanceboxRuntimeRuntimeHoldReason: {
        _enum: {
            __Unused0: "Null",
            __Unused1: "Null",
            __Unused2: "Null",
            __Unused3: "Null",
            __Unused4: "Null",
            __Unused5: "Null",
            __Unused6: "Null",
            __Unused7: "Null",
            __Unused8: "Null",
            __Unused9: "Null",
            __Unused10: "Null",
            __Unused11: "Null",
            StreamPayment: "PalletStreamPaymentHoldReason",
            __Unused13: "Null",
            __Unused14: "Null",
            __Unused15: "Null",
            __Unused16: "Null",
            __Unused17: "Null",
            __Unused18: "Null",
            __Unused19: "Null",
            Registrar: "PalletRegistrarHoldReason",
            __Unused21: "Null",
            __Unused22: "Null",
            __Unused23: "Null",
            __Unused24: "Null",
            __Unused25: "Null",
            __Unused26: "Null",
            DataPreservers: "PalletDataPreserversHoldReason",
            __Unused28: "Null",
            __Unused29: "Null",
            __Unused30: "Null",
            __Unused31: "Null",
            __Unused32: "Null",
            __Unused33: "Null",
            PooledStaking: "PalletPooledStakingHoldReason",
        },
    },
    /**
     * Lookup446: pallet_stream_payment::pallet::HoldReason
     **/
    PalletStreamPaymentHoldReason: {
        _enum: ["StreamPayment", "StreamOpened"],
    },
    /**
     * Lookup447: pallet_registrar::pallet::HoldReason
     **/
    PalletRegistrarHoldReason: {
        _enum: ["RegistrarDeposit"],
    },
    /**
     * Lookup448: pallet_data_preservers::pallet::HoldReason
     **/
    PalletDataPreserversHoldReason: {
        _enum: ["ProfileDeposit"],
    },
    /**
     * Lookup449: pallet_pooled_staking::pallet::HoldReason
     **/
    PalletPooledStakingHoldReason: {
        _enum: ["PooledStake"],
    },
    /**
     * Lookup452: frame_support::traits::tokens::misc::IdAmount<dancebox_runtime::RuntimeFreezeReason, Balance>
     **/
    FrameSupportTokensMiscIdAmountRuntimeFreezeReason: {
        id: "DanceboxRuntimeRuntimeFreezeReason",
        amount: "u128",
    },
    /**
     * Lookup453: dancebox_runtime::RuntimeFreezeReason
     **/
    DanceboxRuntimeRuntimeFreezeReason: {
        _enum: {
            __Unused0: "Null",
            __Unused1: "Null",
            __Unused2: "Null",
            __Unused3: "Null",
            __Unused4: "Null",
            __Unused5: "Null",
            __Unused6: "Null",
            __Unused7: "Null",
            __Unused8: "Null",
            __Unused9: "Null",
            __Unused10: "Null",
            __Unused11: "Null",
            StreamPayment: "PalletStreamPaymentFreezeReason",
        },
    },
    /**
     * Lookup454: pallet_stream_payment::pallet::FreezeReason
     **/
    PalletStreamPaymentFreezeReason: {
        _enum: ["StreamPayment"],
    },
    /**
     * Lookup456: pallet_balances::pallet::Error<T, I>
     **/
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
            "IssuanceDeactivated",
            "DeltaZero",
        ],
    },
    /**
     * Lookup457: pallet_transaction_payment::Releases
     **/
    PalletTransactionPaymentReleases: {
        _enum: ["V1Ancient", "V2"],
    },
    /**
     * Lookup458: pallet_stream_payment::pallet::Stream<sp_core::crypto::AccountId32, tp_stream_payment_common::TimeUnit, tp_stream_payment_common::AssetId, Balance>
     **/
    PalletStreamPaymentStream: {
        source: "AccountId32",
        target: "AccountId32",
        config: "PalletStreamPaymentStreamConfig",
        deposit: "u128",
        lastTimeUpdated: "u128",
        requestNonce: "u32",
        pendingRequest: "Option<PalletStreamPaymentChangeRequest>",
        openingDeposit: "u128",
    },
    /**
     * Lookup460: pallet_stream_payment::pallet::ChangeRequest<tp_stream_payment_common::TimeUnit, tp_stream_payment_common::AssetId, Balance>
     **/
    PalletStreamPaymentChangeRequest: {
        requester: "PalletStreamPaymentParty",
        kind: "PalletStreamPaymentChangeKind",
        newConfig: "PalletStreamPaymentStreamConfig",
        depositChange: "Option<PalletStreamPaymentDepositChange>",
    },
    /**
     * Lookup462: pallet_stream_payment::pallet::Error<T>
     **/
    PalletStreamPaymentError: {
        _enum: [
            "UnknownStreamId",
            "StreamIdOverflow",
            "UnauthorizedOrigin",
            "CantBeBothSourceAndTarget",
            "CantFetchCurrentTime",
            "SourceCantDecreaseRate",
            "TargetCantIncreaseRate",
            "CantOverrideMandatoryChange",
            "NoPendingRequest",
            "CantAcceptOwnRequest",
            "CanOnlyCancelOwnRequest",
            "WrongRequestNonce",
            "ChangingAssetRequiresAbsoluteDepositChange",
            "TargetCantChangeDeposit",
            "ImmediateDepositChangeRequiresSameAssetId",
            "DeadlineCantBeInPast",
            "CantFetchStatusBeforeLastTimeUpdated",
            "DeadlineDelayIsBelowMinium",
            "CantDecreaseDepositUnderSoftDepositMinimum",
            "SourceCantCloseActiveStreamWithSoftDepositMinimum",
            "CantCreateStreamWithDepositUnderSoftMinimum",
        ],
    },
    /**
     * Lookup463: pallet_identity::types::Registration<Balance, MaxJudgements, pallet_identity::legacy::IdentityInfo<FieldLimit>>
     **/
    PalletIdentityRegistration: {
        judgements: "Vec<(u32,PalletIdentityJudgement)>",
        deposit: "u128",
        info: "PalletIdentityLegacyIdentityInfo",
    },
    /**
     * Lookup471: pallet_identity::types::RegistrarInfo<Balance, sp_core::crypto::AccountId32, IdField>
     **/
    PalletIdentityRegistrarInfo: {
        account: "AccountId32",
        fee: "u128",
        fields: "u64",
    },
    /**
     * Lookup474: pallet_identity::types::AuthorityProperties<sp_core::crypto::AccountId32>
     **/
    PalletIdentityAuthorityProperties: {
        accountId: "AccountId32",
        allocation: "u32",
    },
    /**
     * Lookup475: pallet_identity::types::UsernameInformation<sp_core::crypto::AccountId32, Balance>
     **/
    PalletIdentityUsernameInformation: {
        owner: "AccountId32",
        provider: "PalletIdentityProvider",
    },
    /**
     * Lookup476: pallet_identity::types::Provider<Balance>
     **/
    PalletIdentityProvider: {
        _enum: {
            Allocation: "Null",
            AuthorityDeposit: "u128",
            System: "Null",
        },
    },
    /**
     * Lookup478: pallet_identity::pallet::Error<T>
     **/
    PalletIdentityError: {
        _enum: [
            "TooManySubAccounts",
            "NotFound",
            "NotNamed",
            "EmptyIndex",
            "FeeChanged",
            "NoIdentity",
            "StickyJudgement",
            "JudgementGiven",
            "InvalidJudgement",
            "InvalidIndex",
            "InvalidTarget",
            "TooManyRegistrars",
            "AlreadyClaimed",
            "NotSub",
            "NotOwned",
            "JudgementForDifferentIdentity",
            "JudgementPaymentFailed",
            "InvalidSuffix",
            "NotUsernameAuthority",
            "NoAllocation",
            "InvalidSignature",
            "RequiresSignature",
            "InvalidUsername",
            "UsernameTaken",
            "NoUsername",
            "NotExpired",
            "TooEarly",
            "NotUnbinding",
            "AlreadyUnbinding",
            "InsufficientPrivileges",
        ],
    },
    /**
     * Lookup480: pallet_multisig::Multisig<BlockNumber, Balance, sp_core::crypto::AccountId32, MaxApprovals>
     **/
    PalletMultisigMultisig: {
        when: "PalletMultisigTimepoint",
        deposit: "u128",
        depositor: "AccountId32",
        approvals: "Vec<AccountId32>",
    },
    /**
     * Lookup482: pallet_multisig::pallet::Error<T>
     **/
    PalletMultisigError: {
        _enum: [
            "MinimumThreshold",
            "AlreadyApproved",
            "NoApprovalsNeeded",
            "TooFewSignatories",
            "TooManySignatories",
            "SignatoriesOutOfOrder",
            "SenderInSignatories",
            "NotFound",
            "NotOwner",
            "NoTimepoint",
            "WrongTimepoint",
            "UnexpectedTimepoint",
            "MaxWeightTooLow",
            "AlreadyStored",
        ],
    },
    /**
     * Lookup491: pallet_registrar::pallet::DepositInfo<T>
     **/
    PalletRegistrarDepositInfo: {
        creator: "AccountId32",
        deposit: "u128",
    },
    /**
     * Lookup492: pallet_registrar::pallet::Error<T>
     **/
    PalletRegistrarError: {
        _enum: [
            "ParaIdAlreadyRegistered",
            "ParaIdNotRegistered",
            "ParaIdAlreadyDeregistered",
            "ParaIdAlreadyPaused",
            "ParaIdNotPaused",
            "ParaIdListFull",
            "GenesisDataTooBig",
            "ParaIdNotInPendingVerification",
            "NotSufficientDeposit",
            "NotAParathread",
            "NotParaCreator",
            "RelayStorageRootNotFound",
            "InvalidRelayStorageProof",
            "InvalidRelayManagerSignature",
            "ParaStillExistsInRelay",
            "HeadDataNecessary",
            "WasmCodeNecessary",
        ],
    },
    /**
     * Lookup493: pallet_configuration::HostConfiguration
     **/
    PalletConfigurationHostConfiguration: {
        maxCollators: "u32",
        minOrchestratorCollators: "u32",
        maxOrchestratorCollators: "u32",
        collatorsPerContainer: "u32",
        fullRotationPeriod: "u32",
        collatorsPerParathread: "u32",
        parathreadsPerCollator: "u32",
        targetContainerChainFullness: "Perbill",
        maxParachainCoresPercentage: "Option<Perbill>",
        fullRotationMode: "TpTraitsFullRotationModes",
    },
    /**
     * Lookup496: pallet_configuration::pallet::Error<T>
     **/
    PalletConfigurationError: {
        _enum: ["InvalidNewValue"],
    },
    /**
     * Lookup497: dp_collator_assignment::AssignedCollators<sp_core::crypto::AccountId32>
     **/
    DpCollatorAssignmentAssignedCollatorsAccountId32: {
        orchestratorChain: "Vec<AccountId32>",
        containerChains: "BTreeMap<u32, Vec<AccountId32>>",
    },
    /**
     * Lookup502: tp_traits::ContainerChainBlockInfo<sp_core::crypto::AccountId32>
     **/
    TpTraitsContainerChainBlockInfo: {
        blockNumber: "u32",
        author: "AccountId32",
        latestSlotNumber: "u64",
    },
    /**
     * Lookup503: pallet_author_noting::pallet::Error<T>
     **/
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
    /**
     * Lookup504: dp_collator_assignment::AssignedCollators<nimbus_primitives::nimbus_crypto::Public>
     **/
    DpCollatorAssignmentAssignedCollatorsPublic: {
        orchestratorChain: "Vec<NimbusPrimitivesNimbusCryptoPublic>",
        containerChains: "BTreeMap<u32, Vec<NimbusPrimitivesNimbusCryptoPublic>>",
    },
    /**
     * Lookup509: pallet_services_payment::pallet::Error<T>
     **/
    PalletServicesPaymentError: {
        _enum: ["InsufficientFundsToPurchaseCredits", "InsufficientCredits", "CreditPriceTooExpensive"],
    },
    /**
     * Lookup510: pallet_data_preservers::types::RegisteredProfile<T>
     **/
    PalletDataPreserversRegisteredProfile: {
        account: "AccountId32",
        deposit: "u128",
        profile: "PalletDataPreserversProfile",
        assignment: "Option<(u32,TpDataPreserversCommonAssignmentWitness)>",
    },
    /**
     * Lookup516: pallet_data_preservers::pallet::Error<T>
     **/
    PalletDataPreserversError: {
        _enum: [
            "NoBootNodes",
            "UnknownProfileId",
            "NextProfileIdShouldBeAvailable",
            "AssignmentPaymentRequestParameterMismatch",
            "ProfileAlreadyAssigned",
            "ProfileNotAssigned",
            "ProfileIsNotElligibleForParaId",
            "WrongParaId",
            "MaxAssignmentsPerParaIdReached",
            "CantDeleteAssignedProfile",
        ],
    },
    /**
     * Lookup518: pallet_invulnerables::pallet::Error<T>
     **/
    PalletInvulnerablesError: {
        _enum: [
            "TooManyInvulnerables",
            "AlreadyInvulnerable",
            "NotInvulnerable",
            "NoKeysRegistered",
            "UnableToDeriveCollatorId",
        ],
    },
    /**
     * Lookup523: sp_core::crypto::KeyTypeId
     **/
    SpCoreCryptoKeyTypeId: "[u8;4]",
    /**
     * Lookup524: pallet_session::pallet::Error<T>
     **/
    PalletSessionError: {
        _enum: ["InvalidProof", "NoAssociatedValidatorId", "DuplicatedKey", "NoKeys", "NoAccount"],
    },
    /**
     * Lookup528: pallet_author_inherent::pallet::Error<T>
     **/
    PalletAuthorInherentError: {
        _enum: ["AuthorAlreadySet", "NoAccountId", "CannotBeAuthor"],
    },
    /**
     * Lookup530: pallet_pooled_staking::candidate::EligibleCandidate<sp_core::crypto::AccountId32, S>
     **/
    PalletPooledStakingCandidateEligibleCandidate: {
        candidate: "AccountId32",
        stake: "u128",
    },
    /**
     * Lookup533: pallet_pooled_staking::pallet::PoolsKey<sp_core::crypto::AccountId32>
     **/
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
    /**
     * Lookup536: pallet_pooled_staking::pools::CandidateSummary
     **/
    PalletPooledStakingPoolsCandidateSummary: {
        delegators: "u32",
        joiningDelegators: "u32",
        autoCompoundingDelegators: "u32",
        manualRewardsDelegators: "u32",
        leavingDelegators: "u32",
    },
    /**
     * Lookup537: pallet_pooled_staking::pallet::Error<T>
     **/
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
            PoolsExtrinsicsArePaused: "Null",
        },
    },
    /**
     * Lookup538: pallet_inflation_rewards::pallet::ChainsToRewardValue<T>
     **/
    PalletInflationRewardsChainsToRewardValue: {
        paraIds: "Vec<u32>",
        rewardsPerChain: "u128",
    },
    /**
     * Lookup539: pallet_treasury::Proposal<sp_core::crypto::AccountId32, Balance>
     **/
    PalletTreasuryProposal: {
        proposer: "AccountId32",
        value: "u128",
        beneficiary: "AccountId32",
        bond: "u128",
    },
    /**
     * Lookup541: pallet_treasury::SpendStatus<AssetKind, AssetBalance, sp_core::crypto::AccountId32, BlockNumber, PaymentId>
     **/
    PalletTreasurySpendStatus: {
        assetKind: "Null",
        amount: "u128",
        beneficiary: "AccountId32",
        validFrom: "u32",
        expireAt: "u32",
        status: "PalletTreasuryPaymentState",
    },
    /**
     * Lookup542: pallet_treasury::PaymentState<Id>
     **/
    PalletTreasuryPaymentState: {
        _enum: {
            Pending: "Null",
            Attempted: {
                id: "Null",
            },
            Failed: "Null",
        },
    },
    /**
     * Lookup544: frame_support::PalletId
     **/
    FrameSupportPalletId: "[u8;8]",
    /**
     * Lookup545: pallet_treasury::pallet::Error<T, I>
     **/
    PalletTreasuryError: {
        _enum: [
            "InvalidIndex",
            "TooManyApprovals",
            "InsufficientPermission",
            "ProposalNotApproved",
            "FailedToConvertBalance",
            "SpendExpired",
            "EarlyPayout",
            "AlreadyAttempted",
            "PayoutError",
            "NotAttempted",
            "Inconclusive",
        ],
    },
    /**
     * Lookup548: cumulus_pallet_xcmp_queue::OutboundChannelDetails
     **/
    CumulusPalletXcmpQueueOutboundChannelDetails: {
        recipient: "u32",
        state: "CumulusPalletXcmpQueueOutboundState",
        signalsExist: "bool",
        firstIndex: "u16",
        lastIndex: "u16",
    },
    /**
     * Lookup549: cumulus_pallet_xcmp_queue::OutboundState
     **/
    CumulusPalletXcmpQueueOutboundState: {
        _enum: ["Ok", "Suspended"],
    },
    /**
     * Lookup553: cumulus_pallet_xcmp_queue::QueueConfigData
     **/
    CumulusPalletXcmpQueueQueueConfigData: {
        suspendThreshold: "u32",
        dropThreshold: "u32",
        resumeThreshold: "u32",
    },
    /**
     * Lookup554: cumulus_pallet_xcmp_queue::pallet::Error<T>
     **/
    CumulusPalletXcmpQueueError: {
        _enum: ["BadQueueConfig", "AlreadySuspended", "AlreadyResumed", "TooManyActiveOutboundChannels", "TooBig"],
    },
    /**
     * Lookup555: pallet_xcm::pallet::QueryStatus<BlockNumber>
     **/
    PalletXcmQueryStatus: {
        _enum: {
            Pending: {
                responder: "XcmVersionedLocation",
                maybeMatchQuerier: "Option<XcmVersionedLocation>",
                maybeNotify: "Option<(u8,u8)>",
                timeout: "u32",
            },
            VersionNotifier: {
                origin: "XcmVersionedLocation",
                isActive: "bool",
            },
            Ready: {
                response: "XcmVersionedResponse",
                at: "u32",
            },
        },
    },
    /**
     * Lookup559: xcm::VersionedResponse
     **/
    XcmVersionedResponse: {
        _enum: {
            __Unused0: "Null",
            __Unused1: "Null",
            __Unused2: "Null",
            V3: "XcmV3Response",
            V4: "StagingXcmV4Response",
            V5: "StagingXcmV5Response",
        },
    },
    /**
     * Lookup565: pallet_xcm::pallet::VersionMigrationStage
     **/
    PalletXcmVersionMigrationStage: {
        _enum: {
            MigrateSupportedVersion: "Null",
            MigrateVersionNotifiers: "Null",
            NotifyCurrentTargets: "Option<Bytes>",
            MigrateAndNotifyOldTargets: "Null",
        },
    },
    /**
     * Lookup567: pallet_xcm::pallet::RemoteLockedFungibleRecord<ConsumerIdentifier, MaxConsumers>
     **/
    PalletXcmRemoteLockedFungibleRecord: {
        amount: "u128",
        owner: "XcmVersionedLocation",
        locker: "XcmVersionedLocation",
        consumers: "Vec<(Null,u128)>",
    },
    /**
     * Lookup574: pallet_xcm::pallet::Error<T>
     **/
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
            "CannotCheckOutTeleport",
            "LowBalance",
            "TooManyLocks",
            "AccountNotSovereign",
            "FeesNotMet",
            "LockNotFound",
            "InUse",
            "__Unused20",
            "InvalidAssetUnknownReserve",
            "InvalidAssetUnsupportedReserve",
            "TooManyReserves",
            "LocalExecutionIncomplete",
        ],
    },
    /**
     * Lookup575: pallet_assets::types::AssetDetails<Balance, sp_core::crypto::AccountId32, DepositBalance>
     **/
    PalletAssetsAssetDetails: {
        owner: "AccountId32",
        issuer: "AccountId32",
        admin: "AccountId32",
        freezer: "AccountId32",
        supply: "u128",
        deposit: "u128",
        minBalance: "u128",
        isSufficient: "bool",
        accounts: "u32",
        sufficients: "u32",
        approvals: "u32",
        status: "PalletAssetsAssetStatus",
    },
    /**
     * Lookup576: pallet_assets::types::AssetStatus
     **/
    PalletAssetsAssetStatus: {
        _enum: ["Live", "Frozen", "Destroying"],
    },
    /**
     * Lookup578: pallet_assets::types::AssetAccount<Balance, DepositBalance, Extra, sp_core::crypto::AccountId32>
     **/
    PalletAssetsAssetAccount: {
        balance: "u128",
        status: "PalletAssetsAccountStatus",
        reason: "PalletAssetsExistenceReason",
        extra: "Null",
    },
    /**
     * Lookup579: pallet_assets::types::AccountStatus
     **/
    PalletAssetsAccountStatus: {
        _enum: ["Liquid", "Frozen", "Blocked"],
    },
    /**
     * Lookup580: pallet_assets::types::ExistenceReason<Balance, sp_core::crypto::AccountId32>
     **/
    PalletAssetsExistenceReason: {
        _enum: {
            Consumer: "Null",
            Sufficient: "Null",
            DepositHeld: "u128",
            DepositRefunded: "Null",
            DepositFrom: "(AccountId32,u128)",
        },
    },
    /**
     * Lookup582: pallet_assets::types::Approval<Balance, DepositBalance>
     **/
    PalletAssetsApproval: {
        amount: "u128",
        deposit: "u128",
    },
    /**
     * Lookup583: pallet_assets::types::AssetMetadata<DepositBalance, bounded_collections::bounded_vec::BoundedVec<T, S>>
     **/
    PalletAssetsAssetMetadata: {
        deposit: "u128",
        name: "Bytes",
        symbol: "Bytes",
        decimals: "u8",
        isFrozen: "bool",
    },
    /**
     * Lookup585: pallet_assets::pallet::Error<T, I>
     **/
    PalletAssetsError: {
        _enum: [
            "BalanceLow",
            "NoAccount",
            "NoPermission",
            "Unknown",
            "Frozen",
            "InUse",
            "BadWitness",
            "MinBalanceZero",
            "UnavailableConsumer",
            "BadMetadata",
            "Unapproved",
            "WouldDie",
            "AlreadyExists",
            "NoDeposit",
            "WouldBurn",
            "LiveAsset",
            "AssetNotLive",
            "IncorrectStatus",
            "NotFrozen",
            "CallbackFailed",
            "BadAssetId",
        ],
    },
    /**
     * Lookup586: pallet_foreign_asset_creator::pallet::Error<T>
     **/
    PalletForeignAssetCreatorError: {
        _enum: ["AssetAlreadyExists", "AssetDoesNotExist"],
    },
    /**
     * Lookup587: pallet_asset_rate::pallet::Error<T>
     **/
    PalletAssetRateError: {
        _enum: ["UnknownAssetKind", "AlreadyExists", "Overflow"],
    },
    /**
     * Lookup588: pallet_message_queue::BookState<cumulus_primitives_core::AggregateMessageOrigin>
     **/
    PalletMessageQueueBookState: {
        _alias: {
            size_: "size",
        },
        begin: "u32",
        end: "u32",
        count: "u32",
        readyNeighbours: "Option<PalletMessageQueueNeighbours>",
        messageCount: "u64",
        size_: "u64",
    },
    /**
     * Lookup590: pallet_message_queue::Neighbours<cumulus_primitives_core::AggregateMessageOrigin>
     **/
    PalletMessageQueueNeighbours: {
        prev: "CumulusPrimitivesCoreAggregateMessageOrigin",
        next: "CumulusPrimitivesCoreAggregateMessageOrigin",
    },
    /**
     * Lookup592: pallet_message_queue::Page<Size, HeapSize>
     **/
    PalletMessageQueuePage: {
        remaining: "u32",
        remainingSize: "u32",
        firstIndex: "u32",
        first: "u32",
        last: "u32",
        heap: "Bytes",
    },
    /**
     * Lookup594: pallet_message_queue::pallet::Error<T>
     **/
    PalletMessageQueueError: {
        _enum: [
            "NotReapable",
            "NoPage",
            "NoMessage",
            "AlreadyProcessed",
            "Queued",
            "InsufficientWeight",
            "TemporarilyUnprocessable",
            "QueuePaused",
            "RecursiveDisallowed",
        ],
    },
    /**
     * Lookup595: pallet_xcm_core_buyer::InFlightCoreBuyingOrder<BN>
     **/
    PalletXcmCoreBuyerInFlightCoreBuyingOrder: {
        paraId: "u32",
        queryId: "u64",
        ttl: "u32",
    },
    /**
     * Lookup596: pallet_xcm_core_buyer::pallet::Error<T>
     **/
    PalletXcmCoreBuyerError: {
        _enum: [
            "InvalidProof",
            "ErrorValidatingXCM",
            "ErrorDeliveringXCM",
            "OrderAlreadyExists",
            "NotAParathread",
            "InFlightLimitReached",
            "NoAssignedCollators",
            "CollatorNotAssigned",
            "XcmWeightStorageNotSet",
            "ReanchorFailed",
            "LocationInversionFailed",
            "ReportNotifyingSetupFailed",
            "UnexpectedXCMResponse",
            "BlockProductionPending",
            "NotAllowedToProduceBlockRightNow",
            "IncorrectCollatorSignatureNonce",
            "InvalidCollatorSignature",
        ],
    },
    /**
     * Lookup601: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
     **/
    FrameSystemExtensionsCheckNonZeroSender: "Null",
    /**
     * Lookup602: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
     **/
    FrameSystemExtensionsCheckSpecVersion: "Null",
    /**
     * Lookup603: frame_system::extensions::check_tx_version::CheckTxVersion<T>
     **/
    FrameSystemExtensionsCheckTxVersion: "Null",
    /**
     * Lookup604: frame_system::extensions::check_genesis::CheckGenesis<T>
     **/
    FrameSystemExtensionsCheckGenesis: "Null",
    /**
     * Lookup607: frame_system::extensions::check_nonce::CheckNonce<T>
     **/
    FrameSystemExtensionsCheckNonce: "Compact<u32>",
    /**
     * Lookup608: frame_system::extensions::check_weight::CheckWeight<T>
     **/
    FrameSystemExtensionsCheckWeight: "Null",
    /**
     * Lookup609: pallet_transaction_payment::ChargeTransactionPayment<T>
     **/
    PalletTransactionPaymentChargeTransactionPayment: "Compact<u128>",
    /**
     * Lookup610: cumulus_primitives_storage_weight_reclaim::StorageWeightReclaim<T>
     **/
    CumulusPrimitivesStorageWeightReclaimStorageWeightReclaim: "Null",
    /**
     * Lookup611: dancebox_runtime::Runtime
     **/
    DanceboxRuntimeRuntime: "Null",
};
