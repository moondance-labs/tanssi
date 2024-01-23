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
    /** Lookup38: flashbox_runtime::ProxyType */
    FlashboxRuntimeProxyType: {
        _enum: ["Any", "NonTransfer", "Governance", "Staking", "CancelProxy", "Balances", "Registrar", "SudoRegistrar"],
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
    /** Lookup42: pallet_tx_pause::pallet::Event<T> */
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
    /** Lookup45: pallet_balances::pallet::Event<T, I> */
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
    /** Lookup46: frame_support::traits::tokens::misc::BalanceStatus */
    FrameSupportTokensMiscBalanceStatus: {
        _enum: ["Free", "Reserved"],
    },
    /** Lookup47: pallet_transaction_payment::pallet::Event<T> */
    PalletTransactionPaymentEvent: {
        _enum: {
            TransactionFeePaid: {
                who: "AccountId32",
                actualFee: "u128",
                tip: "u128",
            },
        },
    },
    /** Lookup48: pallet_identity::pallet::Event<T> */
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
        },
    },
    /** Lookup49: pallet_registrar::pallet::Event<T> */
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
        },
    },
    /** Lookup51: pallet_collator_assignment::pallet::Event<T> */
    PalletCollatorAssignmentEvent: {
        _enum: {
            NewPendingAssignment: {
                randomSeed: "[u8;32]",
                fullRotation: "bool",
                targetSession: "u32",
            },
        },
    },
    /** Lookup53: pallet_author_noting::pallet::Event<T> */
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
    /** Lookup54: pallet_services_payment::pallet::Event<T> */
    PalletServicesPaymentEvent: {
        _enum: {
            CreditsPurchased: {
                paraId: "u32",
                payer: "AccountId32",
                credit: "u128",
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
    /** Lookup55: pallet_data_preservers::pallet::Event<T> */
    PalletDataPreserversEvent: {
        _enum: {
            BootNodesChanged: {
                paraId: "u32",
            },
        },
    },
    /** Lookup56: pallet_invulnerables::pallet::Event<T> */
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
    /** Lookup58: pallet_session::pallet::Event */
    PalletSessionEvent: {
        _enum: {
            NewSession: {
                sessionIndex: "u32",
            },
        },
    },
    /** Lookup59: pallet_inflation_rewards::pallet::Event<T> */
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
    /** Lookup60: frame_system::Phase */
    FrameSystemPhase: {
        _enum: {
            ApplyExtrinsic: "u32",
            Finalization: "Null",
            Initialization: "Null",
        },
    },
    /** Lookup64: frame_system::LastRuntimeUpgradeInfo */
    FrameSystemLastRuntimeUpgradeInfo: {
        specVersion: "Compact<u32>",
        specName: "Text",
    },
    /** Lookup67: frame_system::pallet::Call<T> */
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
    /** Lookup71: frame_system::limits::BlockWeights */
    FrameSystemLimitsBlockWeights: {
        baseBlock: "SpWeightsWeightV2Weight",
        maxBlock: "SpWeightsWeightV2Weight",
        perClass: "FrameSupportDispatchPerDispatchClassWeightsPerClass",
    },
    /** Lookup72: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass> */
    FrameSupportDispatchPerDispatchClassWeightsPerClass: {
        normal: "FrameSystemLimitsWeightsPerClass",
        operational: "FrameSystemLimitsWeightsPerClass",
        mandatory: "FrameSystemLimitsWeightsPerClass",
    },
    /** Lookup73: frame_system::limits::WeightsPerClass */
    FrameSystemLimitsWeightsPerClass: {
        baseExtrinsic: "SpWeightsWeightV2Weight",
        maxExtrinsic: "Option<SpWeightsWeightV2Weight>",
        maxTotal: "Option<SpWeightsWeightV2Weight>",
        reserved: "Option<SpWeightsWeightV2Weight>",
    },
    /** Lookup75: frame_system::limits::BlockLength */
    FrameSystemLimitsBlockLength: {
        max: "FrameSupportDispatchPerDispatchClassU32",
    },
    /** Lookup76: frame_support::dispatch::PerDispatchClass<T> */
    FrameSupportDispatchPerDispatchClassU32: {
        normal: "u32",
        operational: "u32",
        mandatory: "u32",
    },
    /** Lookup77: sp_weights::RuntimeDbWeight */
    SpWeightsRuntimeDbWeight: {
        read: "u64",
        write: "u64",
    },
    /** Lookup78: sp_version::RuntimeVersion */
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
    /** Lookup83: frame_system::pallet::Error<T> */
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
    /** Lookup85: cumulus_pallet_parachain_system::unincluded_segment::Ancestor<primitive_types::H256> */
    CumulusPalletParachainSystemUnincludedSegmentAncestor: {
        usedBandwidth: "CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth",
        paraHeadHash: "Option<H256>",
        consumedGoAheadSignal: "Option<PolkadotPrimitivesV6UpgradeGoAhead>",
    },
    /** Lookup86: cumulus_pallet_parachain_system::unincluded_segment::UsedBandwidth */
    CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth: {
        umpMsgCount: "u32",
        umpTotalBytes: "u32",
        hrmpOutgoing: "BTreeMap<u32, CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate>",
    },
    /** Lookup88: cumulus_pallet_parachain_system::unincluded_segment::HrmpChannelUpdate */
    CumulusPalletParachainSystemUnincludedSegmentHrmpChannelUpdate: {
        msgCount: "u32",
        totalBytes: "u32",
    },
    /** Lookup93: polkadot_primitives::v6::UpgradeGoAhead */
    PolkadotPrimitivesV6UpgradeGoAhead: {
        _enum: ["Abort", "GoAhead"],
    },
    /** Lookup94: cumulus_pallet_parachain_system::unincluded_segment::SegmentTracker<primitive_types::H256> */
    CumulusPalletParachainSystemUnincludedSegmentSegmentTracker: {
        usedBandwidth: "CumulusPalletParachainSystemUnincludedSegmentUsedBandwidth",
        hrmpWatermark: "Option<u32>",
        consumedGoAheadSignal: "Option<PolkadotPrimitivesV6UpgradeGoAhead>",
    },
    /** Lookup96: polkadot_primitives::v6::PersistedValidationData<primitive_types::H256, N> */
    PolkadotPrimitivesV6PersistedValidationData: {
        parentHead: "Bytes",
        relayParentNumber: "u32",
        relayParentStorageRoot: "H256",
        maxPovSize: "u32",
    },
    /** Lookup99: polkadot_primitives::v6::UpgradeRestriction */
    PolkadotPrimitivesV6UpgradeRestriction: {
        _enum: ["Present"],
    },
    /** Lookup100: sp_trie::storage_proof::StorageProof */
    SpTrieStorageProof: {
        trieNodes: "BTreeSet<Bytes>",
    },
    /** Lookup102: cumulus_pallet_parachain_system::relay_state_snapshot::MessagingStateSnapshot */
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: {
        dmqMqcHead: "H256",
        relayDispatchQueueRemainingCapacity:
            "CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity",
        ingressChannels: "Vec<(u32,PolkadotPrimitivesV6AbridgedHrmpChannel)>",
        egressChannels: "Vec<(u32,PolkadotPrimitivesV6AbridgedHrmpChannel)>",
    },
    /** Lookup103: cumulus_pallet_parachain_system::relay_state_snapshot::RelayDispatchQueueRemainingCapacity */
    CumulusPalletParachainSystemRelayStateSnapshotRelayDispatchQueueRemainingCapacity: {
        remainingCount: "u32",
        remainingSize: "u32",
    },
    /** Lookup106: polkadot_primitives::v6::AbridgedHrmpChannel */
    PolkadotPrimitivesV6AbridgedHrmpChannel: {
        maxCapacity: "u32",
        maxTotalSize: "u32",
        maxMessageSize: "u32",
        msgCount: "u32",
        totalSize: "u32",
        mqcHead: "Option<H256>",
    },
    /** Lookup107: polkadot_primitives::v6::AbridgedHostConfiguration */
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
    /** Lookup108: polkadot_primitives::v6::async_backing::AsyncBackingParams */
    PolkadotPrimitivesV6AsyncBackingAsyncBackingParams: {
        maxCandidateDepth: "u32",
        allowedAncestryLen: "u32",
    },
    /** Lookup114: polkadot_core_primitives::OutboundHrmpMessage<polkadot_parachain_primitives::primitives::Id> */
    PolkadotCorePrimitivesOutboundHrmpMessage: {
        recipient: "u32",
        data: "Bytes",
    },
    /** Lookup116: cumulus_pallet_parachain_system::CodeUpgradeAuthorization<T> */
    CumulusPalletParachainSystemCodeUpgradeAuthorization: {
        codeHash: "H256",
        checkVersion: "bool",
    },
    /** Lookup117: cumulus_pallet_parachain_system::pallet::Call<T> */
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
    /** Lookup118: cumulus_primitives_parachain_inherent::ParachainInherentData */
    CumulusPrimitivesParachainInherentParachainInherentData: {
        validationData: "PolkadotPrimitivesV6PersistedValidationData",
        relayChainState: "SpTrieStorageProof",
        downwardMessages: "Vec<PolkadotCorePrimitivesInboundDownwardMessage>",
        horizontalMessages: "BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>",
    },
    /** Lookup120: polkadot_core_primitives::InboundDownwardMessage<BlockNumber> */
    PolkadotCorePrimitivesInboundDownwardMessage: {
        sentAt: "u32",
        msg: "Bytes",
    },
    /** Lookup123: polkadot_core_primitives::InboundHrmpMessage<BlockNumber> */
    PolkadotCorePrimitivesInboundHrmpMessage: {
        sentAt: "u32",
        data: "Bytes",
    },
    /** Lookup126: cumulus_pallet_parachain_system::pallet::Error<T> */
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
    /** Lookup127: pallet_timestamp::pallet::Call<T> */
    PalletTimestampCall: {
        _enum: {
            set: {
                now: "Compact<u64>",
            },
        },
    },
    /** Lookup128: parachain_info::pallet::Call<T> */
    ParachainInfoCall: "Null",
    /** Lookup129: pallet_sudo::pallet::Call<T> */
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
    /** Lookup131: pallet_utility::pallet::Call<T> */
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
    /** Lookup133: flashbox_runtime::OriginCaller */
    FlashboxRuntimeOriginCaller: {
        _enum: {
            system: "FrameSupportDispatchRawOrigin",
            Void: "SpCoreVoid",
        },
    },
    /** Lookup134: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32> */
    FrameSupportDispatchRawOrigin: {
        _enum: {
            Root: "Null",
            Signed: "AccountId32",
            None: "Null",
        },
    },
    /** Lookup135: sp_core::Void */
    SpCoreVoid: "Null",
    /** Lookup136: pallet_proxy::pallet::Call<T> */
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
    /** Lookup141: pallet_maintenance_mode::pallet::Call<T> */
    PalletMaintenanceModeCall: {
        _enum: ["enter_maintenance_mode", "resume_normal_operation"],
    },
    /** Lookup142: pallet_tx_pause::pallet::Call<T> */
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
    /** Lookup143: pallet_balances::pallet::Call<T, I> */
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
    /** Lookup145: pallet_identity::pallet::Call<T> */
    PalletIdentityCall: {
        _enum: {
            add_registrar: {
                account: "MultiAddress",
            },
            set_identity: {
                info: "PalletIdentitySimpleIdentityInfo",
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
                fields: "PalletIdentityBitFlags",
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
        },
    },
    /** Lookup146: pallet_identity::simple::IdentityInfo<FieldLimit> */
    PalletIdentitySimpleIdentityInfo: {
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
    /** Lookup183: pallet_identity::types::BitFlags<pallet_identity::simple::IdentityField> */
    PalletIdentityBitFlags: {
        _bitLength: 64,
        Display: 0,
        Legal: 1,
        Web: 2,
        Riot: 3,
        Email: 4,
        PgpFingerprint: 5,
        Image: 6,
        Twitter: 7,
    },
    /** Lookup184: pallet_identity::simple::IdentityField */
    PalletIdentitySimpleIdentityField: {
        _enum: ["Display", "Legal", "Web", "Riot", "Email", "PgpFingerprint", "Image", "Twitter"],
    },
    /** Lookup185: pallet_identity::types::Judgement<Balance> */
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
    /** Lookup186: pallet_registrar::pallet::Call<T> */
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
        },
    },
    /** Lookup187: tp_container_chain_genesis_data::ContainerChainGenesisData<MaxLengthTokenSymbol> */
    TpContainerChainGenesisDataContainerChainGenesisData: {
        storage: "Vec<TpContainerChainGenesisDataContainerChainGenesisDataItem>",
        name: "Bytes",
        id: "Bytes",
        forkId: "Option<Bytes>",
        extensions: "Bytes",
        properties: "TpContainerChainGenesisDataProperties",
    },
    /** Lookup189: tp_container_chain_genesis_data::ContainerChainGenesisDataItem */
    TpContainerChainGenesisDataContainerChainGenesisDataItem: {
        key: "Bytes",
        value: "Bytes",
    },
    /** Lookup191: tp_container_chain_genesis_data::Properties<MaxLengthTokenSymbol> */
    TpContainerChainGenesisDataProperties: {
        tokenMetadata: "TpContainerChainGenesisDataTokenMetadata",
        isEthereum: "bool",
    },
    /** Lookup192: tp_container_chain_genesis_data::TokenMetadata<MaxLengthTokenSymbol> */
    TpContainerChainGenesisDataTokenMetadata: {
        tokenSymbol: "Bytes",
        ss58Format: "u32",
        tokenDecimals: "u32",
    },
    /** Lookup194: pallet_configuration::pallet::Call<T> */
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
    /** Lookup196: pallet_collator_assignment::pallet::Call<T> */
    PalletCollatorAssignmentCall: "Null",
    /** Lookup197: pallet_author_noting::pallet::Call<T> */
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
    /** Lookup198: tp_author_noting_inherent::OwnParachainInherentData */
    TpAuthorNotingInherentOwnParachainInherentData: {
        relayStorageProof: "SpTrieStorageProof",
    },
    /** Lookup199: pallet_authority_assignment::pallet::Call<T> */
    PalletAuthorityAssignmentCall: "Null",
    /** Lookup200: pallet_services_payment::pallet::Call<T> */
    PalletServicesPaymentCall: {
        _enum: {
            purchase_credits: {
                paraId: "u32",
                credit: "u128",
            },
            set_credits: {
                paraId: "u32",
                credits: "u32",
            },
            set_given_free_credits: {
                paraId: "u32",
                givenFreeCredits: "bool",
            },
        },
    },
    /** Lookup201: pallet_data_preservers::pallet::Call<T> */
    PalletDataPreserversCall: {
        _enum: {
            set_boot_nodes: {
                paraId: "u32",
                bootNodes: "Vec<Bytes>",
            },
        },
    },
    /** Lookup205: pallet_invulnerables::pallet::Call<T> */
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
    /** Lookup206: pallet_session::pallet::Call<T> */
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
    /** Lookup207: flashbox_runtime::SessionKeys */
    FlashboxRuntimeSessionKeys: {
        nimbus: "NimbusPrimitivesNimbusCryptoPublic",
    },
    /** Lookup208: nimbus_primitives::nimbus_crypto::Public */
    NimbusPrimitivesNimbusCryptoPublic: "SpCoreSr25519Public",
    /** Lookup209: sp_core::sr25519::Public */
    SpCoreSr25519Public: "[u8;32]",
    /** Lookup210: pallet_author_inherent::pallet::Call<T> */
    PalletAuthorInherentCall: {
        _enum: ["kick_off_authorship_validation"],
    },
    /** Lookup211: pallet_root_testing::pallet::Call<T> */
    PalletRootTestingCall: {
        _enum: {
            fill_block: {
                ratio: "Perbill",
            },
        },
    },
    /** Lookup212: pallet_sudo::pallet::Error<T> */
    PalletSudoError: {
        _enum: ["RequireSudo"],
    },
    /** Lookup213: pallet_utility::pallet::Error<T> */
    PalletUtilityError: {
        _enum: ["TooManyCalls"],
    },
    /** Lookup216: pallet_proxy::ProxyDefinition<sp_core::crypto::AccountId32, flashbox_runtime::ProxyType, BlockNumber> */
    PalletProxyProxyDefinition: {
        delegate: "AccountId32",
        proxyType: "FlashboxRuntimeProxyType",
        delay: "u32",
    },
    /** Lookup220: pallet_proxy::Announcement<sp_core::crypto::AccountId32, primitive_types::H256, BlockNumber> */
    PalletProxyAnnouncement: {
        real: "AccountId32",
        callHash: "H256",
        height: "u32",
    },
    /** Lookup222: pallet_proxy::pallet::Error<T> */
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
    /** Lookup223: pallet_migrations::pallet::Error<T> */
    PalletMigrationsError: {
        _enum: ["PreimageMissing", "WrongUpperBound", "PreimageIsTooBig", "PreimageAlreadyExists"],
    },
    /** Lookup224: pallet_maintenance_mode::pallet::Error<T> */
    PalletMaintenanceModeError: {
        _enum: ["AlreadyInMaintenanceMode", "NotInMaintenanceMode"],
    },
    /** Lookup225: pallet_tx_pause::pallet::Error<T> */
    PalletTxPauseError: {
        _enum: ["IsPaused", "IsUnpaused", "Unpausable", "NotFound"],
    },
    /** Lookup227: pallet_balances::types::BalanceLock<Balance> */
    PalletBalancesBalanceLock: {
        id: "[u8;8]",
        amount: "u128",
        reasons: "PalletBalancesReasons",
    },
    /** Lookup228: pallet_balances::types::Reasons */
    PalletBalancesReasons: {
        _enum: ["Fee", "Misc", "All"],
    },
    /** Lookup231: pallet_balances::types::ReserveData<ReserveIdentifier, Balance> */
    PalletBalancesReserveData: {
        id: "[u8;8]",
        amount: "u128",
    },
    /** Lookup235: flashbox_runtime::RuntimeHoldReason */
    FlashboxRuntimeRuntimeHoldReason: "Null",
    /** Lookup238: pallet_balances::types::IdAmount<Id, Balance> */
    PalletBalancesIdAmount: {
        id: "[u8;8]",
        amount: "u128",
    },
    /** Lookup240: pallet_balances::pallet::Error<T, I> */
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
    /** Lookup241: pallet_transaction_payment::Releases */
    PalletTransactionPaymentReleases: {
        _enum: ["V1Ancient", "V2"],
    },
    /** Lookup242: pallet_identity::types::Registration<Balance, MaxJudgements, pallet_identity::simple::IdentityInfo<FieldLimit>> */
    PalletIdentityRegistration: {
        judgements: "Vec<(u32,PalletIdentityJudgement)>",
        deposit: "u128",
        info: "PalletIdentitySimpleIdentityInfo",
    },
    /**
     * Lookup250: pallet_identity::types::RegistrarInfo<Balance, sp_core::crypto::AccountId32,
     * pallet_identity::simple::IdentityField>
     */
    PalletIdentityRegistrarInfo: {
        account: "AccountId32",
        fee: "u128",
        fields: "PalletIdentityBitFlags",
    },
    /** Lookup252: pallet_identity::pallet::Error<T> */
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
            "TooManyFields",
            "TooManyRegistrars",
            "AlreadyClaimed",
            "NotSub",
            "NotOwned",
            "JudgementForDifferentIdentity",
            "JudgementPaymentFailed",
        ],
    },
    /** Lookup257: pallet_registrar::pallet::DepositInfo<T> */
    PalletRegistrarDepositInfo: {
        creator: "AccountId32",
        deposit: "u128",
    },
    /** Lookup258: pallet_registrar::pallet::Error<T> */
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
        ],
    },
    /** Lookup259: pallet_configuration::HostConfiguration */
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
    /** Lookup262: pallet_configuration::pallet::Error<T> */
    PalletConfigurationError: {
        _enum: ["InvalidNewValue"],
    },
    /** Lookup263: dp_collator_assignment::AssignedCollators<sp_core::crypto::AccountId32> */
    DpCollatorAssignmentAssignedCollatorsAccountId32: {
        orchestratorChain: "Vec<AccountId32>",
        containerChains: "BTreeMap<u32, Vec<AccountId32>>",
    },
    /** Lookup268: pallet_author_noting::pallet::ContainerChainBlockInfo<T> */
    PalletAuthorNotingContainerChainBlockInfo: {
        blockNumber: "u32",
        author: "AccountId32",
    },
    /** Lookup269: pallet_author_noting::pallet::Error<T> */
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
    /** Lookup270: dp_collator_assignment::AssignedCollators<nimbus_primitives::nimbus_crypto::Public> */
    DpCollatorAssignmentAssignedCollatorsPublic: {
        orchestratorChain: "Vec<NimbusPrimitivesNimbusCryptoPublic>",
        containerChains: "BTreeMap<u32, Vec<NimbusPrimitivesNimbusCryptoPublic>>",
    },
    /** Lookup275: pallet_services_payment::pallet::Error<T> */
    PalletServicesPaymentError: {
        _enum: ["InsufficientFundsToPurchaseCredits", "InsufficientCredits", "CreditPriceTooExpensive"],
    },
    /** Lookup276: pallet_data_preservers::pallet::Error<T> */
    PalletDataPreserversError: {
        _enum: ["NoBootNodes"],
    },
    /** Lookup278: pallet_invulnerables::pallet::Error<T> */
    PalletInvulnerablesError: {
        _enum: ["TooManyInvulnerables", "AlreadyInvulnerable", "NotInvulnerable"],
    },
    /** Lookup283: sp_core::crypto::KeyTypeId */
    SpCoreCryptoKeyTypeId: "[u8;4]",
    /** Lookup284: pallet_session::pallet::Error<T> */
    PalletSessionError: {
        _enum: ["InvalidProof", "NoAssociatedValidatorId", "DuplicatedKey", "NoKeys", "NoAccount"],
    },
    /** Lookup288: pallet_author_inherent::pallet::Error<T> */
    PalletAuthorInherentError: {
        _enum: ["AuthorAlreadySet", "NoAccountId", "CannotBeAuthor"],
    },
    /** Lookup289: pallet_inflation_rewards::pallet::ChainsToRewardValue<T> */
    PalletInflationRewardsChainsToRewardValue: {
        paraIds: "Vec<u32>",
        rewardsPerChain: "u128",
    },
    /** Lookup294: sp_runtime::MultiSignature */
    SpRuntimeMultiSignature: {
        _enum: {
            Ed25519: "SpCoreEd25519Signature",
            Sr25519: "SpCoreSr25519Signature",
            Ecdsa: "SpCoreEcdsaSignature",
        },
    },
    /** Lookup295: sp_core::ed25519::Signature */
    SpCoreEd25519Signature: "[u8;64]",
    /** Lookup297: sp_core::sr25519::Signature */
    SpCoreSr25519Signature: "[u8;64]",
    /** Lookup298: sp_core::ecdsa::Signature */
    SpCoreEcdsaSignature: "[u8;65]",
    /** Lookup301: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T> */
    FrameSystemExtensionsCheckNonZeroSender: "Null",
    /** Lookup302: frame_system::extensions::check_spec_version::CheckSpecVersion<T> */
    FrameSystemExtensionsCheckSpecVersion: "Null",
    /** Lookup303: frame_system::extensions::check_tx_version::CheckTxVersion<T> */
    FrameSystemExtensionsCheckTxVersion: "Null",
    /** Lookup304: frame_system::extensions::check_genesis::CheckGenesis<T> */
    FrameSystemExtensionsCheckGenesis: "Null",
    /** Lookup307: frame_system::extensions::check_nonce::CheckNonce<T> */
    FrameSystemExtensionsCheckNonce: "Compact<u32>",
    /** Lookup308: frame_system::extensions::check_weight::CheckWeight<T> */
    FrameSystemExtensionsCheckWeight: "Null",
    /** Lookup309: pallet_transaction_payment::ChargeTransactionPayment<T> */
    PalletTransactionPaymentChargeTransactionPayment: "Compact<u128>",
    /** Lookup310: flashbox_runtime::Runtime */
    FlashboxRuntimeRuntime: "Null",
};
