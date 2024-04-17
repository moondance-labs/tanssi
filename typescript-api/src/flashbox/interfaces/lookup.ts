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
    /** Lookup19: frame_system::EventRecord<flashbox_runtime::RuntimeEvent, primitive_types::H256> */
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
            UpgradeAuthorized: {
                codeHash: "H256",
                checkVersion: "bool",
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
    /** Lookup31: cumulus_pallet_parachain_system::pallet::Event<T> */
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
    /** Lookup33: pallet_sudo::pallet::Event<T> */
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
    /** Lookup37: pallet_utility::pallet::Event */
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
    /** Lookup38: pallet_proxy::pallet::Event<T> */
    PalletProxyEvent: {
        _enum: {
            ProxyExecuted: {
                result: "Result<Null, SpRuntimeDispatchError>",
            },
            PureCreated: {
                pure: "AccountId32",
                who: "AccountId32",
                proxyType: "FlashboxRuntimeProxyType",
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
                proxyType: "FlashboxRuntimeProxyType",
                delay: "u32",
            },
            ProxyRemoved: {
                delegator: "AccountId32",
                delegatee: "AccountId32",
                proxyType: "FlashboxRuntimeProxyType",
                delay: "u32",
            },
        },
    },
    /** Lookup39: flashbox_runtime::ProxyType */
    FlashboxRuntimeProxyType: {
        _enum: ["Any", "NonTransfer", "Governance", "Staking", "CancelProxy", "Balances", "Registrar", "SudoRegistrar"],
    },
    /** Lookup41: pallet_migrations::pallet::Event<T> */
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
    /** Lookup42: pallet_maintenance_mode::pallet::Event */
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
    /** Lookup43: pallet_tx_pause::pallet::Event<T> */
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
    /** Lookup46: pallet_balances::pallet::Event<T, I> */
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
    /** Lookup47: frame_support::traits::tokens::misc::BalanceStatus */
    FrameSupportTokensMiscBalanceStatus: {
        _enum: ["Free", "Reserved"],
    },
    /** Lookup48: pallet_transaction_payment::pallet::Event<T> */
    PalletTransactionPaymentEvent: {
        _enum: {
            TransactionFeePaid: {
                who: "AccountId32",
                actualFee: "u128",
                tip: "u128",
            },
        },
    },
    /** Lookup49: pallet_stream_payment::pallet::Event<T> */
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
    /** Lookup50: pallet_stream_payment::pallet::Party */
    PalletStreamPaymentParty: {
        _enum: ["Source", "Target"],
    },
    /**
     * Lookup51: pallet_stream_payment::pallet::StreamConfig<flashbox_runtime::TimeUnit,
     * flashbox_runtime::StreamPaymentAssetId, Balance>
     */
    PalletStreamPaymentStreamConfig: {
        timeUnit: "FlashboxRuntimeTimeUnit",
        assetId: "FlashboxRuntimeStreamPaymentAssetId",
        rate: "u128",
    },
    /** Lookup52: flashbox_runtime::TimeUnit */
    FlashboxRuntimeTimeUnit: {
        _enum: ["BlockNumber", "Timestamp"],
    },
    /** Lookup53: flashbox_runtime::StreamPaymentAssetId */
    FlashboxRuntimeStreamPaymentAssetId: {
        _enum: ["Native"],
    },
    /** Lookup55: pallet_stream_payment::pallet::DepositChange<Balance> */
    PalletStreamPaymentDepositChange: {
        _enum: {
            Increase: "u128",
            Decrease: "u128",
            Absolute: "u128",
        },
    },
    /** Lookup56: pallet_identity::pallet::Event<T> */
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
        },
    },
    /** Lookup58: pallet_multisig::pallet::Event<T> */
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
    /** Lookup59: pallet_multisig::Timepoint<BlockNumber> */
    PalletMultisigTimepoint: {
        height: "u32",
        index: "u32",
    },
    /** Lookup60: pallet_registrar::pallet::Event<T> */
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
        },
    },
    /** Lookup62: pallet_collator_assignment::pallet::Event<T> */
    PalletCollatorAssignmentEvent: {
        _enum: {
            NewPendingAssignment: {
                randomSeed: "[u8;32]",
                fullRotation: "bool",
                targetSession: "u32",
            },
        },
    },
    /** Lookup63: pallet_author_noting::pallet::Event<T> */
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
    /** Lookup65: pallet_services_payment::pallet::Event<T> */
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
                maxCorePrice: "Option<u128>",
            },
            CollatorAssignmentCreditsSet: {
                paraId: "u32",
                credits: "u32",
            },
        },
    },
    /** Lookup67: pallet_data_preservers::pallet::Event<T> */
    PalletDataPreserversEvent: {
        _enum: {
            BootNodesChanged: {
                paraId: "u32",
            },
        },
    },
    /** Lookup68: pallet_invulnerables::pallet::Event<T> */
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
    /** Lookup70: pallet_session::pallet::Event */
    PalletSessionEvent: {
        _enum: {
            NewSession: {
                sessionIndex: "u32",
            },
        },
    },
    /** Lookup71: pallet_inflation_rewards::pallet::Event<T> */
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
    /** Lookup72: pallet_treasury::pallet::Event<T, I> */
    PalletTreasuryEvent: {
        _enum: {
            Proposed: {
                proposalIndex: "u32",
            },
            Spending: {
                budgetRemaining: "u128",
            },
            Awarded: {
                proposalIndex: "u32",
                award: "u128",
                account: "AccountId32",
            },
            Rejected: {
                proposalIndex: "u32",
                slashed: "u128",
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
    /** Lookup73: pallet_root_testing::pallet::Event<T> */
    PalletRootTestingEvent: {
        _enum: ["DefensiveTestCall"],
    },
    /** Lookup74: frame_system::Phase */
    FrameSystemPhase: {
        _enum: {
            ApplyExtrinsic: "u32",
            Finalization: "Null",
            Initialization: "Null",
        },
    },
    /** Lookup78: frame_system::LastRuntimeUpgradeInfo */
    FrameSystemLastRuntimeUpgradeInfo: {
        specVersion: "Compact<u32>",
        specName: "Text",
    },
    /** Lookup81: frame_system::CodeUpgradeAuthorization<T> */
    FrameSystemCodeUpgradeAuthorization: {
        codeHash: "H256",
        checkVersion: "bool",
    },
    /** Lookup82: frame_system::pallet::Call<T> */
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
    /** Lookup86: frame_system::limits::BlockWeights */
    FrameSystemLimitsBlockWeights: {
        baseBlock: "SpWeightsWeightV2Weight",
        maxBlock: "SpWeightsWeightV2Weight",
        perClass: "FrameSupportDispatchPerDispatchClassWeightsPerClass",
    },
    /** Lookup87: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass> */
    FrameSupportDispatchPerDispatchClassWeightsPerClass: {
        normal: "FrameSystemLimitsWeightsPerClass",
        operational: "FrameSystemLimitsWeightsPerClass",
        mandatory: "FrameSystemLimitsWeightsPerClass",
    },
    /** Lookup88: frame_system::limits::WeightsPerClass */
    FrameSystemLimitsWeightsPerClass: {
        baseExtrinsic: "SpWeightsWeightV2Weight",
        maxExtrinsic: "Option<SpWeightsWeightV2Weight>",
        maxTotal: "Option<SpWeightsWeightV2Weight>",
        reserved: "Option<SpWeightsWeightV2Weight>",
    },
    /** Lookup90: frame_system::limits::BlockLength */
    FrameSystemLimitsBlockLength: {
        max: "FrameSupportDispatchPerDispatchClassU32",
    },
    /** Lookup91: frame_support::dispatch::PerDispatchClass<T> */
    FrameSupportDispatchPerDispatchClassU32: {
        normal: "u32",
        operational: "u32",
        mandatory: "u32",
    },
    /** Lookup92: sp_weights::RuntimeDbWeight */
    SpWeightsRuntimeDbWeight: {
        read: "u64",
        write: "u64",
    },
    /** Lookup93: sp_version::RuntimeVersion */
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
    /** Lookup98: frame_system::pallet::Error<T> */
    FrameSystemError: {
        _enum: [
            "InvalidSpecName",
            "SpecVersionNeedsToIncrease",
            "FailedToExtractRuntimeVersion",
            "NonDefaultComposite",
            "NonZeroRefCount",
            "CallFiltered",
            "NothingAuthorized",
            "Unauthorized",
        ],
    },
    /** Lookup100: cumulus_pallet_parachain_system::unincluded_segment::Ancestor<primitive_types::H256> */
    CumulusPalletParachainSystemUnincludedSegmentAncestor: {
        usedBandwidth: "CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth",
        paraHeadHash: "Option<H256>",
        consumedGoAheadSignal: "Option<PolkadotPrimitivesV6UpgradeGoAhead>",
    },
    /** Lookup101: cumulus_pallet_parachain_system::unincluded_segment::UsedBandwidth */
    CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth: {
        umpMsgCount: "u32",
        umpTotalBytes: "u32",
        hrmpOutgoing: "BTreeMap<u32, CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate>",
    },
    /** Lookup103: cumulus_pallet_parachain_system::unincluded_segment::HrmpChannelUpdate */
    CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate: {
        msgCount: "u32",
        totalBytes: "u32",
    },
    /** Lookup108: polkadot_primitives::v6::UpgradeGoAhead */
    PolkadotPrimitivesV6UpgradeGoAhead: {
        _enum: ["Abort", "GoAhead"],
    },
    /** Lookup109: cumulus_pallet_parachain_system::unincluded_segment::SegmentTracker<primitive_types::H256> */
    CumulusPalletParachainSystemUnincludedSegmentSegmentTracker: {
        usedBandwidth: "CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth",
        hrmpWatermark: "Option<u32>",
        consumedGoAheadSignal: "Option<PolkadotPrimitivesV6UpgradeGoAhead>",
    },
    /** Lookup111: polkadot_primitives::v6::PersistedValidationData<primitive_types::H256, N> */
    PolkadotPrimitivesV6PersistedValidationData: {
        parentHead: "Bytes",
        relayParentNumber: "u32",
        relayParentStorageRoot: "H256",
        maxPovSize: "u32",
    },
    /** Lookup114: polkadot_primitives::v6::UpgradeRestriction */
    PolkadotPrimitivesV6UpgradeRestriction: {
        _enum: ["Present"],
    },
    /** Lookup115: sp_trie::storage_proof::StorageProof */
    SpTrieStorageProof: {
        trieNodes: "BTreeSet<Bytes>",
    },
    /** Lookup117: cumulus_pallet_parachain_system::relay_state_snapshot::MessagingStateSnapshot */
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: {
        dmqMqcHead: "H256",
        relayDispatchQueueRemainingCapacity:
            "CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity",
        ingressChannels: "Vec<(u32,PolkadotPrimitivesV6AbridgedHrmpChannel)>",
        egressChannels: "Vec<(u32,PolkadotPrimitivesV6AbridgedHrmpChannel)>",
    },
    /** Lookup118: cumulus_pallet_parachain_system::relay_state_snapshot::RelayDispatchQueueRemainingCapacity */
    CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity: {
        remainingCount: "u32",
        remainingSize: "u32",
    },
    /** Lookup121: polkadot_primitives::v6::AbridgedHrmpChannel */
    PolkadotPrimitivesV6AbridgedHrmpChannel: {
        maxCapacity: "u32",
        maxTotalSize: "u32",
        maxMessageSize: "u32",
        msgCount: "u32",
        totalSize: "u32",
        mqcHead: "Option<H256>",
    },
    /** Lookup122: polkadot_primitives::v6::AbridgedHostConfiguration */
    PolkadotPrimitivesV6AbridgedHostConfiguration: {
        maxCodeSize: "u32",
        maxHeadDataSize: "u32",
        maxUpwardQueueCount: "u32",
        maxUpwardQueueSize: "u32",
        maxUpwardMessageSize: "u32",
        maxUpwardMessageNumPerCandidate: "u32",
        hrmpMaxMessageNumPerCandidate: "u32",
        validationUpgradeCooldown: "u32",
        validationUpgradeDelay: "u32",
        asyncBackingParams: "PolkadotPrimitivesV6AsyncBackingAsyncBackingParams",
    },
    /** Lookup123: polkadot_primitives::v6::async_backing::AsyncBackingParams */
    PolkadotPrimitivesV6AsyncBackingAsyncBackingParams: {
        maxCandidateDepth: "u32",
        allowedAncestryLen: "u32",
    },
    /** Lookup129: polkadot_core_primitives::OutboundHrmpMessage<polkadot_parachain_primitives::primitives::Id> */
    PolkadotCorePrimitivesOutboundHrmpMessage: {
        recipient: "u32",
        data: "Bytes",
    },
    /** Lookup131: cumulus_pallet_parachain_system::pallet::Call<T> */
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
    /** Lookup132: cumulus_primitives_parachain_inherent::ParachainInherentData */
    CumulusPrimitivesParachainInherentParachainInherentData: {
        validationData: "PolkadotPrimitivesV6PersistedValidationData",
        relayChainState: "SpTrieStorageProof",
        downwardMessages: "Vec<PolkadotCorePrimitivesInboundDownwardMessage>",
        horizontalMessages: "BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>",
    },
    /** Lookup134: polkadot_core_primitives::InboundDownwardMessage<BlockNumber> */
    PolkadotCorePrimitivesInboundDownwardMessage: {
        sentAt: "u32",
        msg: "Bytes",
    },
    /** Lookup137: polkadot_core_primitives::InboundHrmpMessage<BlockNumber> */
    PolkadotCorePrimitivesInboundHrmpMessage: {
        sentAt: "u32",
        data: "Bytes",
    },
    /** Lookup140: cumulus_pallet_parachain_system::pallet::Error<T> */
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
    /** Lookup141: pallet_timestamp::pallet::Call<T> */
    PalletTimestampCall: {
        _enum: {
            set: {
                now: "Compact<u64>",
            },
        },
    },
    /** Lookup142: staging_parachain_info::pallet::Call<T> */
    StagingParachainInfoCall: "Null",
    /** Lookup143: pallet_sudo::pallet::Call<T> */
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
    /** Lookup145: pallet_utility::pallet::Call<T> */
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
                asOrigin: "FlashboxRuntimeOriginCaller",
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
    /** Lookup147: flashbox_runtime::OriginCaller */
    FlashboxRuntimeOriginCaller: {
        _enum: {
            system: "FrameSupportDispatchRawOrigin",
            Void: "SpCoreVoid",
        },
    },
    /** Lookup148: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32> */
    FrameSupportDispatchRawOrigin: {
        _enum: {
            Root: "Null",
            Signed: "AccountId32",
            None: "Null",
        },
    },
    /** Lookup149: sp_core::Void */
    SpCoreVoid: "Null",
    /** Lookup150: pallet_proxy::pallet::Call<T> */
    PalletProxyCall: {
        _enum: {
            proxy: {
                real: "MultiAddress",
                forceProxyType: "Option<FlashboxRuntimeProxyType>",
                call: "Call",
            },
            add_proxy: {
                delegate: "MultiAddress",
                proxyType: "FlashboxRuntimeProxyType",
                delay: "u32",
            },
            remove_proxy: {
                delegate: "MultiAddress",
                proxyType: "FlashboxRuntimeProxyType",
                delay: "u32",
            },
            remove_proxies: "Null",
            create_pure: {
                proxyType: "FlashboxRuntimeProxyType",
                delay: "u32",
                index: "u16",
            },
            kill_pure: {
                spawner: "MultiAddress",
                proxyType: "FlashboxRuntimeProxyType",
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
                forceProxyType: "Option<FlashboxRuntimeProxyType>",
                call: "Call",
            },
        },
    },
    /** Lookup155: pallet_maintenance_mode::pallet::Call<T> */
    PalletMaintenanceModeCall: {
        _enum: ["enter_maintenance_mode", "resume_normal_operation"],
    },
    /** Lookup156: pallet_tx_pause::pallet::Call<T> */
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
    /** Lookup157: pallet_balances::pallet::Call<T, I> */
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
        },
    },
    /** Lookup159: pallet_stream_payment::pallet::Call<T> */
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
                assetId: "FlashboxRuntimeStreamPaymentAssetId",
                change: "PalletStreamPaymentDepositChange",
            },
        },
    },
    /** Lookup160: pallet_stream_payment::pallet::ChangeKind<Time> */
    PalletStreamPaymentChangeKind: {
        _enum: {
            Suggestion: "Null",
            Mandatory: {
                deadline: "u128",
            },
        },
    },
    /** Lookup161: pallet_identity::pallet::Call<T> */
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
                authority: "MultiAddress",
            },
            set_username_for: {
                who: "MultiAddress",
                username: "Bytes",
                signature: "Option<SpRuntimeMultiSignature>",
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
            remove_dangling_username: {
                username: "Bytes",
            },
        },
    },
    /** Lookup162: pallet_identity::legacy::IdentityInfo<FieldLimit> */
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
    /** Lookup199: pallet_identity::types::Judgement<Balance> */
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
    /** Lookup201: sp_runtime::MultiSignature */
    SpRuntimeMultiSignature: {
        _enum: {
            Ed25519: "SpCoreEd25519Signature",
            Sr25519: "SpCoreSr25519Signature",
            Ecdsa: "SpCoreEcdsaSignature",
        },
    },
    /** Lookup202: sp_core::ed25519::Signature */
    SpCoreEd25519Signature: "[u8;64]",
    /** Lookup204: sp_core::sr25519::Signature */
    SpCoreSr25519Signature: "[u8;64]",
    /** Lookup205: sp_core::ecdsa::Signature */
    SpCoreEcdsaSignature: "[u8;65]",
    /** Lookup207: pallet_multisig::pallet::Call<T> */
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
    /** Lookup209: pallet_registrar::pallet::Call<T> */
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
                genesisData: "TpContainerChainGenesisDataContainerChainGenesisData",
            },
            set_parathread_params: {
                paraId: "u32",
                slotFrequency: "TpTraitsSlotFrequency",
            },
        },
    },
    /** Lookup210: tp_container_chain_genesis_data::ContainerChainGenesisData<MaxLengthTokenSymbol> */
    TpContainerChainGenesisDataContainerChainGenesisData: {
        storage: "Vec<TpContainerChainGenesisDataContainerChainGenesisDataItem>",
        name: "Bytes",
        id: "Bytes",
        forkId: "Option<Bytes>",
        extensions: "Bytes",
        properties: "TpContainerChainGenesisDataProperties",
    },
    /** Lookup212: tp_container_chain_genesis_data::ContainerChainGenesisDataItem */
    TpContainerChainGenesisDataContainerChainGenesisDataItem: {
        key: "Bytes",
        value: "Bytes",
    },
    /** Lookup214: tp_container_chain_genesis_data::Properties<MaxLengthTokenSymbol> */
    TpContainerChainGenesisDataProperties: {
        tokenMetadata: "TpContainerChainGenesisDataTokenMetadata",
        isEthereum: "bool",
    },
    /** Lookup215: tp_container_chain_genesis_data::TokenMetadata<MaxLengthTokenSymbol> */
    TpContainerChainGenesisDataTokenMetadata: {
        tokenSymbol: "Bytes",
        ss58Format: "u32",
        tokenDecimals: "u32",
    },
    /** Lookup217: tp_traits::SlotFrequency */
    TpTraitsSlotFrequency: {
        min: "u32",
        max: "u32",
    },
    /** Lookup218: pallet_configuration::pallet::Call<T> */
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
    /** Lookup220: pallet_collator_assignment::pallet::Call<T> */
    PalletCollatorAssignmentCall: "Null",
    /** Lookup221: pallet_author_noting::pallet::Call<T> */
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
    /** Lookup222: tp_author_noting_inherent::OwnParachainInherentData */
    TpAuthorNotingInherentOwnParachainInherentData: {
        relayStorageProof: "SpTrieStorageProof",
    },
    /** Lookup223: pallet_authority_assignment::pallet::Call<T> */
    PalletAuthorityAssignmentCall: "Null",
    /** Lookup224: pallet_services_payment::pallet::Call<T> */
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
                maxCorePrice: "Option<u128>",
            },
            set_max_tip: {
                paraId: "u32",
                maxTip: "Option<u128>",
            },
        },
    },
    /** Lookup225: pallet_data_preservers::pallet::Call<T> */
    PalletDataPreserversCall: {
        _enum: {
            set_boot_nodes: {
                paraId: "u32",
                bootNodes: "Vec<Bytes>",
            },
        },
    },
    /** Lookup229: pallet_invulnerables::pallet::Call<T> */
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
    /** Lookup230: pallet_session::pallet::Call<T> */
    PalletSessionCall: {
        _enum: {
            set_keys: {
                _alias: {
                    keys_: "keys",
                },
                keys_: "FlashboxRuntimeSessionKeys",
                proof: "Bytes",
            },
            purge_keys: "Null",
        },
    },
    /** Lookup231: flashbox_runtime::SessionKeys */
    FlashboxRuntimeSessionKeys: {
        nimbus: "NimbusPrimitivesNimbusCryptoPublic",
    },
    /** Lookup232: nimbus_primitives::nimbus_crypto::Public */
    NimbusPrimitivesNimbusCryptoPublic: "SpCoreSr25519Public",
    /** Lookup233: sp_core::sr25519::Public */
    SpCoreSr25519Public: "[u8;32]",
    /** Lookup234: pallet_author_inherent::pallet::Call<T> */
    PalletAuthorInherentCall: {
        _enum: ["kick_off_authorship_validation"],
    },
    /** Lookup235: pallet_treasury::pallet::Call<T, I> */
    PalletTreasuryCall: {
        _enum: {
            propose_spend: {
                value: "Compact<u128>",
                beneficiary: "MultiAddress",
            },
            reject_proposal: {
                proposalId: "Compact<u32>",
            },
            approve_proposal: {
                proposalId: "Compact<u32>",
            },
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
    /** Lookup236: pallet_root_testing::pallet::Call<T> */
    PalletRootTestingCall: {
        _enum: {
            fill_block: {
                ratio: "Perbill",
            },
            trigger_defensive: "Null",
        },
    },
    /** Lookup237: pallet_sudo::pallet::Error<T> */
    PalletSudoError: {
        _enum: ["RequireSudo"],
    },
    /** Lookup238: pallet_utility::pallet::Error<T> */
    PalletUtilityError: {
        _enum: ["TooManyCalls"],
    },
    /** Lookup241: pallet_proxy::ProxyDefinition<sp_core::crypto::AccountId32, flashbox_runtime::ProxyType, BlockNumber> */
    PalletProxyProxyDefinition: {
        delegate: "AccountId32",
        proxyType: "FlashboxRuntimeProxyType",
        delay: "u32",
    },
    /** Lookup245: pallet_proxy::Announcement<sp_core::crypto::AccountId32, primitive_types::H256, BlockNumber> */
    PalletProxyAnnouncement: {
        real: "AccountId32",
        callHash: "H256",
        height: "u32",
    },
    /** Lookup247: pallet_proxy::pallet::Error<T> */
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
    /** Lookup248: pallet_migrations::pallet::Error<T> */
    PalletMigrationsError: {
        _enum: ["PreimageMissing", "WrongUpperBound", "PreimageIsTooBig", "PreimageAlreadyExists"],
    },
    /** Lookup249: pallet_maintenance_mode::pallet::Error<T> */
    PalletMaintenanceModeError: {
        _enum: ["AlreadyInMaintenanceMode", "NotInMaintenanceMode"],
    },
    /** Lookup250: pallet_tx_pause::pallet::Error<T> */
    PalletTxPauseError: {
        _enum: ["IsPaused", "IsUnpaused", "Unpausable", "NotFound"],
    },
    /** Lookup252: pallet_balances::types::BalanceLock<Balance> */
    PalletBalancesBalanceLock: {
        id: "[u8;8]",
        amount: "u128",
        reasons: "PalletBalancesReasons",
    },
    /** Lookup253: pallet_balances::types::Reasons */
    PalletBalancesReasons: {
        _enum: ["Fee", "Misc", "All"],
    },
    /** Lookup256: pallet_balances::types::ReserveData<ReserveIdentifier, Balance> */
    PalletBalancesReserveData: {
        id: "[u8;8]",
        amount: "u128",
    },
    /** Lookup259: pallet_balances::types::IdAmount<flashbox_runtime::RuntimeHoldReason, Balance> */
    PalletBalancesIdAmountRuntimeHoldReason: {
        id: "FlashboxRuntimeRuntimeHoldReason",
        amount: "u128",
    },
    /** Lookup260: flashbox_runtime::RuntimeHoldReason */
    FlashboxRuntimeRuntimeHoldReason: {
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
        },
    },
    /** Lookup261: pallet_stream_payment::pallet::HoldReason */
    PalletStreamPaymentHoldReason: {
        _enum: ["StreamPayment", "StreamOpened"],
    },
    /** Lookup264: pallet_balances::types::IdAmount<flashbox_runtime::RuntimeFreezeReason, Balance> */
    PalletBalancesIdAmountRuntimeFreezeReason: {
        id: "FlashboxRuntimeRuntimeFreezeReason",
        amount: "u128",
    },
    /** Lookup265: flashbox_runtime::RuntimeFreezeReason */
    FlashboxRuntimeRuntimeFreezeReason: {
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
    /** Lookup266: pallet_stream_payment::pallet::FreezeReason */
    PalletStreamPaymentFreezeReason: {
        _enum: ["StreamPayment"],
    },
    /** Lookup268: pallet_balances::pallet::Error<T, I> */
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
    /** Lookup269: pallet_transaction_payment::Releases */
    PalletTransactionPaymentReleases: {
        _enum: ["V1Ancient", "V2"],
    },
    /**
     * Lookup270: pallet_stream_payment::pallet::Stream<sp_core::crypto::AccountId32, flashbox_runtime::TimeUnit,
     * flashbox_runtime::StreamPaymentAssetId, Balance>
     */
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
     * Lookup272: pallet_stream_payment::pallet::ChangeRequest<flashbox_runtime::TimeUnit,
     * flashbox_runtime::StreamPaymentAssetId, Balance>
     */
    PalletStreamPaymentChangeRequest: {
        requester: "PalletStreamPaymentParty",
        kind: "PalletStreamPaymentChangeKind",
        newConfig: "PalletStreamPaymentStreamConfig",
        depositChange: "Option<PalletStreamPaymentDepositChange>",
    },
    /** Lookup274: pallet_stream_payment::pallet::Error<T> */
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
        ],
    },
    /** Lookup276: pallet_identity::types::Registration<Balance, MaxJudgements, pallet_identity::legacy::IdentityInfo<FieldLimit>> */
    PalletIdentityRegistration: {
        judgements: "Vec<(u32,PalletIdentityJudgement)>",
        deposit: "u128",
        info: "PalletIdentityLegacyIdentityInfo",
    },
    /** Lookup285: pallet_identity::types::RegistrarInfo<Balance, sp_core::crypto::AccountId32, IdField> */
    PalletIdentityRegistrarInfo: {
        account: "AccountId32",
        fee: "u128",
        fields: "u64",
    },
    /** Lookup287: pallet_identity::types::AuthorityProperties<bounded_collections::bounded_vec::BoundedVec<T, S>> */
    PalletIdentityAuthorityProperties: {
        suffix: "Bytes",
        allocation: "u32",
    },
    /** Lookup290: pallet_identity::pallet::Error<T> */
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
        ],
    },
    /** Lookup292: pallet_multisig::Multisig<BlockNumber, Balance, sp_core::crypto::AccountId32, MaxApprovals> */
    PalletMultisigMultisig: {
        when: "PalletMultisigTimepoint",
        deposit: "u128",
        depositor: "AccountId32",
        approvals: "Vec<AccountId32>",
    },
    /** Lookup294: pallet_multisig::pallet::Error<T> */
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
    /** Lookup299: tp_traits::ParathreadParams */
    TpTraitsParathreadParams: {
        slotFrequency: "TpTraitsSlotFrequency",
    },
    /** Lookup305: pallet_registrar::pallet::DepositInfo<T> */
    PalletRegistrarDepositInfo: {
        creator: "AccountId32",
        deposit: "u128",
    },
    /** Lookup306: pallet_registrar::pallet::Error<T> */
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
        ],
    },
    /** Lookup307: pallet_configuration::HostConfiguration */
    PalletConfigurationHostConfiguration: {
        maxCollators: "u32",
        minOrchestratorCollators: "u32",
        maxOrchestratorCollators: "u32",
        collatorsPerContainer: "u32",
        fullRotationPeriod: "u32",
        collatorsPerParathread: "u32",
        parathreadsPerCollator: "u32",
        targetContainerChainFullness: "Perbill",
    },
    /** Lookup310: pallet_configuration::pallet::Error<T> */
    PalletConfigurationError: {
        _enum: ["InvalidNewValue"],
    },
    /** Lookup311: dp_collator_assignment::AssignedCollators<sp_core::crypto::AccountId32> */
    DpCollatorAssignmentAssignedCollatorsAccountId32: {
        orchestratorChain: "Vec<AccountId32>",
        containerChains: "BTreeMap<u32, Vec<AccountId32>>",
    },
    /** Lookup316: pallet_author_noting::pallet::ContainerChainBlockInfo<T> */
    PalletAuthorNotingContainerChainBlockInfo: {
        blockNumber: "u32",
        author: "AccountId32",
        latestSlotNumber: "u64",
    },
    /** Lookup317: pallet_author_noting::pallet::Error<T> */
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
    /** Lookup318: dp_collator_assignment::AssignedCollators<nimbus_primitives::nimbus_crypto::Public> */
    DpCollatorAssignmentAssignedCollatorsPublic: {
        orchestratorChain: "Vec<NimbusPrimitivesNimbusCryptoPublic>",
        containerChains: "BTreeMap<u32, Vec<NimbusPrimitivesNimbusCryptoPublic>>",
    },
    /** Lookup323: pallet_services_payment::pallet::Error<T> */
    PalletServicesPaymentError: {
        _enum: ["InsufficientFundsToPurchaseCredits", "InsufficientCredits", "CreditPriceTooExpensive"],
    },
    /** Lookup324: pallet_data_preservers::pallet::Error<T> */
    PalletDataPreserversError: {
        _enum: ["NoBootNodes"],
    },
    /** Lookup326: pallet_invulnerables::pallet::Error<T> */
    PalletInvulnerablesError: {
        _enum: [
            "TooManyInvulnerables",
            "AlreadyInvulnerable",
            "NotInvulnerable",
            "NoKeysRegistered",
            "UnableToDeriveCollatorId",
        ],
    },
    /** Lookup331: sp_core::crypto::KeyTypeId */
    SpCoreCryptoKeyTypeId: "[u8;4]",
    /** Lookup332: pallet_session::pallet::Error<T> */
    PalletSessionError: {
        _enum: ["InvalidProof", "NoAssociatedValidatorId", "DuplicatedKey", "NoKeys", "NoAccount"],
    },
    /** Lookup336: pallet_author_inherent::pallet::Error<T> */
    PalletAuthorInherentError: {
        _enum: ["AuthorAlreadySet", "NoAccountId", "CannotBeAuthor"],
    },
    /** Lookup337: pallet_inflation_rewards::pallet::ChainsToRewardValue<T> */
    PalletInflationRewardsChainsToRewardValue: {
        paraIds: "Vec<u32>",
        rewardsPerChain: "u128",
    },
    /** Lookup338: pallet_treasury::Proposal<sp_core::crypto::AccountId32, Balance> */
    PalletTreasuryProposal: {
        proposer: "AccountId32",
        value: "u128",
        beneficiary: "AccountId32",
        bond: "u128",
    },
    /** Lookup340: pallet_treasury::SpendStatus<AssetKind, AssetBalance, sp_core::crypto::AccountId32, BlockNumber, PaymentId> */
    PalletTreasurySpendStatus: {
        assetKind: "Null",
        amount: "u128",
        beneficiary: "AccountId32",
        validFrom: "u32",
        expireAt: "u32",
        status: "PalletTreasuryPaymentState",
    },
    /** Lookup341: pallet_treasury::PaymentState<Id> */
    PalletTreasuryPaymentState: {
        _enum: {
            Pending: "Null",
            Attempted: {
                id: "Null",
            },
            Failed: "Null",
        },
    },
    /** Lookup343: frame_support::PalletId */
    FrameSupportPalletId: "[u8;8]",
    /** Lookup344: pallet_treasury::pallet::Error<T, I> */
    PalletTreasuryError: {
        _enum: [
            "InsufficientProposersBalance",
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
    /** Lookup349: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T> */
    FrameSystemExtensionsCheckNonZeroSender: "Null",
    /** Lookup350: frame_system::extensions::check_spec_version::CheckSpecVersion<T> */
    FrameSystemExtensionsCheckSpecVersion: "Null",
    /** Lookup351: frame_system::extensions::check_tx_version::CheckTxVersion<T> */
    FrameSystemExtensionsCheckTxVersion: "Null",
    /** Lookup352: frame_system::extensions::check_genesis::CheckGenesis<T> */
    FrameSystemExtensionsCheckGenesis: "Null",
    /** Lookup355: frame_system::extensions::check_nonce::CheckNonce<T> */
    FrameSystemExtensionsCheckNonce: "Compact<u32>",
    /** Lookup356: frame_system::extensions::check_weight::CheckWeight<T> */
    FrameSystemExtensionsCheckWeight: "Null",
    /** Lookup357: pallet_transaction_payment::ChargeTransactionPayment<T> */
    PalletTransactionPaymentChargeTransactionPayment: "Compact<u128>",
    /** Lookup358: flashbox_runtime::Runtime */
    FlashboxRuntimeRuntime: "Null",
};
