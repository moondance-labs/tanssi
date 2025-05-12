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
     * Lookup20: frame_system::EventRecord<dancelight_runtime::RuntimeEvent, primitive_types::H256>
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
     * Lookup32: pallet_balances::pallet::Event<T, I>
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
     * Lookup33: frame_support::traits::tokens::misc::BalanceStatus
     **/
    FrameSupportTokensMiscBalanceStatus: {
        _enum: ["Free", "Reserved"],
    },
    /**
     * Lookup34: pallet_parameters::pallet::Event<T>
     **/
    PalletParametersEvent: {
        _enum: {
            Updated: {
                key: "DancelightRuntimeRuntimeParametersKey",
                oldValue: "Option<DancelightRuntimeRuntimeParametersValue>",
                newValue: "Option<DancelightRuntimeRuntimeParametersValue>",
            },
        },
    },
    /**
     * Lookup35: dancelight_runtime::RuntimeParametersKey
     **/
    DancelightRuntimeRuntimeParametersKey: {
        _enum: {
            Preimage: "DancelightRuntimeDynamicParamsPreimageParametersKey",
        },
    },
    /**
     * Lookup36: dancelight_runtime::dynamic_params::preimage::ParametersKey
     **/
    DancelightRuntimeDynamicParamsPreimageParametersKey: {
        _enum: ["BaseDeposit", "ByteDeposit"],
    },
    /**
     * Lookup37: dancelight_runtime::dynamic_params::preimage::BaseDeposit
     **/
    DancelightRuntimeDynamicParamsPreimageBaseDeposit: "Null",
    /**
     * Lookup38: dancelight_runtime::dynamic_params::preimage::ByteDeposit
     **/
    DancelightRuntimeDynamicParamsPreimageByteDeposit: "Null",
    /**
     * Lookup40: dancelight_runtime::RuntimeParametersValue
     **/
    DancelightRuntimeRuntimeParametersValue: {
        _enum: {
            Preimage: "DancelightRuntimeDynamicParamsPreimageParametersValue",
        },
    },
    /**
     * Lookup41: dancelight_runtime::dynamic_params::preimage::ParametersValue
     **/
    DancelightRuntimeDynamicParamsPreimageParametersValue: {
        _enum: {
            BaseDeposit: "u128",
            ByteDeposit: "u128",
        },
    },
    /**
     * Lookup42: pallet_transaction_payment::pallet::Event<T>
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
     * Lookup43: pallet_offences::pallet::Event
     **/
    PalletOffencesEvent: {
        _enum: {
            Offence: {
                kind: "[u8;16]",
                timeslot: "Bytes",
            },
        },
    },
    /**
     * Lookup45: pallet_registrar::pallet::Event<T>
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
     * Lookup47: pallet_invulnerables::pallet::Event<T>
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
     * Lookup48: pallet_collator_assignment::pallet::Event<T>
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
     * Lookup49: tp_traits::FullRotationModes
     **/
    TpTraitsFullRotationModes: {
        orchestrator: "TpTraitsFullRotationMode",
        parachain: "TpTraitsFullRotationMode",
        parathread: "TpTraitsFullRotationMode",
    },
    /**
     * Lookup50: tp_traits::FullRotationMode
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
     * Lookup52: pallet_author_noting::pallet::Event<T>
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
     * Lookup54: pallet_services_payment::pallet::Event<T>
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
     * Lookup56: pallet_data_preservers::pallet::Event<T>
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
     * Lookup57: pallet_external_validators::pallet::Event<T>
     **/
    PalletExternalValidatorsEvent: {
        _enum: {
            WhitelistedValidatorAdded: {
                accountId: "AccountId32",
            },
            WhitelistedValidatorRemoved: {
                accountId: "AccountId32",
            },
            NewEra: {
                era: "u32",
            },
            ForceEra: {
                mode: "PalletExternalValidatorsForcing",
            },
            ExternalValidatorsSet: {
                validators: "Vec<AccountId32>",
                externalIndex: "u64",
            },
        },
    },
    /**
     * Lookup58: pallet_external_validators::Forcing
     **/
    PalletExternalValidatorsForcing: {
        _enum: ["NotForcing", "ForceNew", "ForceNone", "ForceAlways"],
    },
    /**
     * Lookup60: pallet_external_validator_slashes::pallet::Event<T>
     **/
    PalletExternalValidatorSlashesEvent: {
        _enum: {
            SlashReported: {
                validator: "AccountId32",
                fraction: "Perbill",
                slashEra: "u32",
            },
            SlashesMessageSent: {
                messageId: "H256",
                slashesCommand: "TpBridgeCommand",
            },
        },
    },
    /**
     * Lookup61: tp_bridge::Command
     **/
    TpBridgeCommand: {
        _enum: {
            Test: "Bytes",
            ReportRewards: {
                externalIdx: "u64",
                eraIndex: "u32",
                totalPoints: "u128",
                tokensInflated: "u128",
                rewardsMerkleRoot: "H256",
                tokenId: "H256",
            },
            ReportSlashes: {
                eraIndex: "u32",
                slashes: "Vec<TpBridgeSlashData>",
            },
        },
    },
    /**
     * Lookup63: tp_bridge::SlashData
     **/
    TpBridgeSlashData: {
        encodedValidatorId: "Bytes",
        slashFraction: "u32",
        externalIdx: "u64",
    },
    /**
     * Lookup64: pallet_external_validators_rewards::pallet::Event<T>
     **/
    PalletExternalValidatorsRewardsEvent: {
        _enum: {
            RewardsMessageSent: {
                messageId: "H256",
                rewardsCommand: "TpBridgeCommand",
            },
        },
    },
    /**
     * Lookup65: snowbridge_pallet_outbound_queue::pallet::Event<T>
     **/
    SnowbridgePalletOutboundQueueEvent: {
        _enum: {
            MessageQueued: {
                id: "H256",
            },
            MessageAccepted: {
                id: "H256",
                nonce: "u64",
            },
            MessagesCommitted: {
                root: "H256",
                count: "u64",
            },
            OperatingModeChanged: {
                mode: "SnowbridgeCoreOperatingModeBasicOperatingMode",
            },
        },
    },
    /**
     * Lookup66: snowbridge_core::operating_mode::BasicOperatingMode
     **/
    SnowbridgeCoreOperatingModeBasicOperatingMode: {
        _enum: ["Normal", "Halted"],
    },
    /**
     * Lookup67: snowbridge_pallet_inbound_queue::pallet::Event<T>
     **/
    SnowbridgePalletInboundQueueEvent: {
        _enum: {
            MessageReceived: {
                channelId: "SnowbridgeCoreChannelId",
                nonce: "u64",
                messageId: "[u8;32]",
                feeBurned: "u128",
            },
            OperatingModeChanged: {
                mode: "SnowbridgeCoreOperatingModeBasicOperatingMode",
            },
        },
    },
    /**
     * Lookup68: snowbridge_core::ChannelId
     **/
    SnowbridgeCoreChannelId: "[u8;32]",
    /**
     * Lookup69: snowbridge_pallet_system::pallet::Event<T>
     **/
    SnowbridgePalletSystemEvent: {
        _enum: {
            Upgrade: {
                implAddress: "H160",
                implCodeHash: "H256",
                initializerParamsHash: "Option<H256>",
            },
            CreateAgent: {
                location: "StagingXcmV5Location",
                agentId: "H256",
            },
            CreateChannel: {
                channelId: "SnowbridgeCoreChannelId",
                agentId: "H256",
            },
            UpdateChannel: {
                channelId: "SnowbridgeCoreChannelId",
                mode: "SnowbridgeCoreOutboundV1OperatingMode",
            },
            SetOperatingMode: {
                mode: "SnowbridgeCoreOutboundV1OperatingMode",
            },
            TransferNativeFromAgent: {
                agentId: "H256",
                recipient: "H160",
                amount: "u128",
            },
            SetTokenTransferFees: {
                createAssetXcm: "u128",
                transferAssetXcm: "u128",
                registerToken: "U256",
            },
            PricingParametersChanged: {
                params: "SnowbridgeCorePricingPricingParameters",
            },
            RegisterToken: {
                location: "XcmVersionedLocation",
                foreignTokenId: "H256",
            },
        },
    },
    /**
     * Lookup73: staging_xcm::v5::location::Location
     **/
    StagingXcmV5Location: {
        parents: "u8",
        interior: "StagingXcmV5Junctions",
    },
    /**
     * Lookup74: staging_xcm::v5::junctions::Junctions
     **/
    StagingXcmV5Junctions: {
        _enum: {
            Here: "Null",
            X1: "[Lookup76;1]",
            X2: "[Lookup76;2]",
            X3: "[Lookup76;3]",
            X4: "[Lookup76;4]",
            X5: "[Lookup76;5]",
            X6: "[Lookup76;6]",
            X7: "[Lookup76;7]",
            X8: "[Lookup76;8]",
        },
    },
    /**
     * Lookup76: staging_xcm::v5::junction::Junction
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
     * Lookup79: staging_xcm::v5::junction::NetworkId
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
     * Lookup81: xcm::v3::junction::BodyId
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
     * Lookup82: xcm::v3::junction::BodyPart
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
     * Lookup90: snowbridge_core::outbound::v1::OperatingMode
     **/
    SnowbridgeCoreOutboundV1OperatingMode: {
        _enum: ["Normal", "RejectingOutboundMessages"],
    },
    /**
     * Lookup93: snowbridge_core::pricing::PricingParameters<Balance>
     **/
    SnowbridgeCorePricingPricingParameters: {
        exchangeRate: "u128",
        rewards: "SnowbridgeCorePricingRewards",
        feePerGas: "U256",
        multiplier: "u128",
    },
    /**
     * Lookup95: snowbridge_core::pricing::Rewards<Balance>
     **/
    SnowbridgeCorePricingRewards: {
        local: "u128",
        remote: "U256",
    },
    /**
     * Lookup96: xcm::VersionedLocation
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
     * Lookup97: staging_xcm::v3::multilocation::MultiLocation
     **/
    StagingXcmV3MultiLocation: {
        parents: "u8",
        interior: "XcmV3Junctions",
    },
    /**
     * Lookup98: xcm::v3::junctions::Junctions
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
     * Lookup99: xcm::v3::junction::Junction
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
     * Lookup101: xcm::v3::junction::NetworkId
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
     * Lookup102: staging_xcm::v4::location::Location
     **/
    StagingXcmV4Location: {
        parents: "u8",
        interior: "StagingXcmV4Junctions",
    },
    /**
     * Lookup103: staging_xcm::v4::junctions::Junctions
     **/
    StagingXcmV4Junctions: {
        _enum: {
            Here: "Null",
            X1: "[Lookup105;1]",
            X2: "[Lookup105;2]",
            X3: "[Lookup105;3]",
            X4: "[Lookup105;4]",
            X5: "[Lookup105;5]",
            X6: "[Lookup105;6]",
            X7: "[Lookup105;7]",
            X8: "[Lookup105;8]",
        },
    },
    /**
     * Lookup105: staging_xcm::v4::junction::Junction
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
     * Lookup107: staging_xcm::v4::junction::NetworkId
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
     * Lookup115: pallet_outbound_message_commitment_recorder::pallet::Event<T>
     **/
    PalletOutboundMessageCommitmentRecorderEvent: {
        _enum: {
            NewCommitmentRootRecorded: {
                commitment: "H256",
            },
            CommitmentRootRead: {
                commitment: "H256",
            },
        },
    },
    /**
     * Lookup116: pallet_ethereum_token_transfers::pallet::Event<T>
     **/
    PalletEthereumTokenTransfersEvent: {
        _enum: {
            ChannelInfoSet: {
                channelInfo: "TpBridgeChannelInfo",
            },
            NativeTokenTransferred: {
                messageId: "H256",
                channelId: "SnowbridgeCoreChannelId",
                source: "AccountId32",
                recipient: "H160",
                tokenId: "H256",
                amount: "u128",
                fee: "u128",
            },
        },
    },
    /**
     * Lookup117: tp_bridge::ChannelInfo
     **/
    TpBridgeChannelInfo: {
        channelId: "SnowbridgeCoreChannelId",
        paraId: "u32",
        agentId: "H256",
    },
    /**
     * Lookup118: pallet_session::pallet::Event
     **/
    PalletSessionEvent: {
        _enum: {
            NewSession: {
                sessionIndex: "u32",
            },
        },
    },
    /**
     * Lookup119: pallet_grandpa::pallet::Event
     **/
    PalletGrandpaEvent: {
        _enum: {
            NewAuthorities: {
                authoritySet: "Vec<(SpConsensusGrandpaAppPublic,u64)>",
            },
            Paused: "Null",
            Resumed: "Null",
        },
    },
    /**
     * Lookup122: sp_consensus_grandpa::app::Public
     **/
    SpConsensusGrandpaAppPublic: "[u8;32]",
    /**
     * Lookup123: pallet_inflation_rewards::pallet::Event<T>
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
     * Lookup124: pallet_pooled_staking::pallet::Event<T>
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
     * Lookup126: pallet_pooled_staking::pools::ActivePoolKind
     **/
    PalletPooledStakingPoolsActivePoolKind: {
        _enum: ["AutoCompounding", "ManualRewards"],
    },
    /**
     * Lookup127: pallet_inactivity_tracking::pallet::Event<T>
     **/
    PalletInactivityTrackingEvent: {
        _enum: {
            ActivityTrackingStatusSet: {
                status: "PalletInactivityTrackingActivityTrackingStatus",
            },
        },
    },
    /**
     * Lookup128: pallet_inactivity_tracking::pallet::ActivityTrackingStatus
     **/
    PalletInactivityTrackingActivityTrackingStatus: {
        _enum: {
            Enabled: {
                start: "u32",
                end: "u32",
            },
            Disabled: {
                end: "u32",
            },
        },
    },
    /**
     * Lookup129: pallet_treasury::pallet::Event<T, I>
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
     * Lookup131: pallet_conviction_voting::pallet::Event<T, I>
     **/
    PalletConvictionVotingEvent: {
        _enum: {
            Delegated: "(AccountId32,AccountId32)",
            Undelegated: "AccountId32",
            Voted: {
                who: "AccountId32",
                vote: "PalletConvictionVotingVoteAccountVote",
            },
            VoteRemoved: {
                who: "AccountId32",
                vote: "PalletConvictionVotingVoteAccountVote",
            },
        },
    },
    /**
     * Lookup132: pallet_conviction_voting::vote::AccountVote<Balance>
     **/
    PalletConvictionVotingVoteAccountVote: {
        _enum: {
            Standard: {
                vote: "Vote",
                balance: "u128",
            },
            Split: {
                aye: "u128",
                nay: "u128",
            },
            SplitAbstain: {
                aye: "u128",
                nay: "u128",
                abstain: "u128",
            },
        },
    },
    /**
     * Lookup134: pallet_referenda::pallet::Event<T, I>
     **/
    PalletReferendaEvent: {
        _enum: {
            Submitted: {
                index: "u32",
                track: "u16",
                proposal: "FrameSupportPreimagesBounded",
            },
            DecisionDepositPlaced: {
                index: "u32",
                who: "AccountId32",
                amount: "u128",
            },
            DecisionDepositRefunded: {
                index: "u32",
                who: "AccountId32",
                amount: "u128",
            },
            DepositSlashed: {
                who: "AccountId32",
                amount: "u128",
            },
            DecisionStarted: {
                index: "u32",
                track: "u16",
                proposal: "FrameSupportPreimagesBounded",
                tally: "PalletConvictionVotingTally",
            },
            ConfirmStarted: {
                index: "u32",
            },
            ConfirmAborted: {
                index: "u32",
            },
            Confirmed: {
                index: "u32",
                tally: "PalletConvictionVotingTally",
            },
            Approved: {
                index: "u32",
            },
            Rejected: {
                index: "u32",
                tally: "PalletConvictionVotingTally",
            },
            TimedOut: {
                index: "u32",
                tally: "PalletConvictionVotingTally",
            },
            Cancelled: {
                index: "u32",
                tally: "PalletConvictionVotingTally",
            },
            Killed: {
                index: "u32",
                tally: "PalletConvictionVotingTally",
            },
            SubmissionDepositRefunded: {
                index: "u32",
                who: "AccountId32",
                amount: "u128",
            },
            MetadataSet: {
                _alias: {
                    hash_: "hash",
                },
                index: "u32",
                hash_: "H256",
            },
            MetadataCleared: {
                _alias: {
                    hash_: "hash",
                },
                index: "u32",
                hash_: "H256",
            },
        },
    },
    /**
     * Lookup136: frame_support::traits::preimages::Bounded<dancelight_runtime::RuntimeCall, sp_runtime::traits::BlakeTwo256>
     **/
    FrameSupportPreimagesBounded: {
        _enum: {
            Legacy: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
            Inline: "Bytes",
            Lookup: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
                len: "u32",
            },
        },
    },
    /**
     * Lookup138: frame_system::pallet::Call<T>
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
     * Lookup142: pallet_babe::pallet::Call<T>
     **/
    PalletBabeCall: {
        _enum: {
            report_equivocation: {
                equivocationProof: "SpConsensusSlotsEquivocationProof",
                keyOwnerProof: "SpSessionMembershipProof",
            },
            report_equivocation_unsigned: {
                equivocationProof: "SpConsensusSlotsEquivocationProof",
                keyOwnerProof: "SpSessionMembershipProof",
            },
            plan_config_change: {
                config: "SpConsensusBabeDigestsNextConfigDescriptor",
            },
        },
    },
    /**
     * Lookup143: sp_consensus_slots::EquivocationProof<sp_runtime::generic::header::Header<Number, Hash>, sp_consensus_babe::app::Public>
     **/
    SpConsensusSlotsEquivocationProof: {
        offender: "SpConsensusBabeAppPublic",
        slot: "u64",
        firstHeader: "SpRuntimeHeader",
        secondHeader: "SpRuntimeHeader",
    },
    /**
     * Lookup144: sp_runtime::generic::header::Header<Number, Hash>
     **/
    SpRuntimeHeader: {
        parentHash: "H256",
        number: "Compact<u32>",
        stateRoot: "H256",
        extrinsicsRoot: "H256",
        digest: "SpRuntimeDigest",
    },
    /**
     * Lookup145: sp_consensus_babe::app::Public
     **/
    SpConsensusBabeAppPublic: "[u8;32]",
    /**
     * Lookup146: sp_session::MembershipProof
     **/
    SpSessionMembershipProof: {
        session: "u32",
        trieNodes: "Vec<Bytes>",
        validatorCount: "u32",
    },
    /**
     * Lookup147: sp_consensus_babe::digests::NextConfigDescriptor
     **/
    SpConsensusBabeDigestsNextConfigDescriptor: {
        _enum: {
            __Unused0: "Null",
            V1: {
                c: "(u64,u64)",
                allowedSlots: "SpConsensusBabeAllowedSlots",
            },
        },
    },
    /**
     * Lookup149: sp_consensus_babe::AllowedSlots
     **/
    SpConsensusBabeAllowedSlots: {
        _enum: ["PrimarySlots", "PrimaryAndSecondaryPlainSlots", "PrimaryAndSecondaryVRFSlots"],
    },
    /**
     * Lookup150: pallet_timestamp::pallet::Call<T>
     **/
    PalletTimestampCall: {
        _enum: {
            set: {
                now: "Compact<u64>",
            },
        },
    },
    /**
     * Lookup151: pallet_balances::pallet::Call<T, I>
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
     * Lookup154: pallet_balances::types::AdjustmentDirection
     **/
    PalletBalancesAdjustmentDirection: {
        _enum: ["Increase", "Decrease"],
    },
    /**
     * Lookup155: pallet_parameters::pallet::Call<T>
     **/
    PalletParametersCall: {
        _enum: {
            set_parameter: {
                keyValue: "DancelightRuntimeRuntimeParameters",
            },
        },
    },
    /**
     * Lookup156: dancelight_runtime::RuntimeParameters
     **/
    DancelightRuntimeRuntimeParameters: {
        _enum: {
            Preimage: "DancelightRuntimeDynamicParamsPreimageParameters",
        },
    },
    /**
     * Lookup157: dancelight_runtime::dynamic_params::preimage::Parameters
     **/
    DancelightRuntimeDynamicParamsPreimageParameters: {
        _enum: {
            BaseDeposit: "(DancelightRuntimeDynamicParamsPreimageBaseDeposit,Option<u128>)",
            ByteDeposit: "(DancelightRuntimeDynamicParamsPreimageByteDeposit,Option<u128>)",
        },
    },
    /**
     * Lookup159: pallet_registrar::pallet::Call<T>
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
     * Lookup160: dp_container_chain_genesis_data::ContainerChainGenesisData
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
     * Lookup162: dp_container_chain_genesis_data::ContainerChainGenesisDataItem
     **/
    DpContainerChainGenesisDataContainerChainGenesisDataItem: {
        key: "Bytes",
        value: "Bytes",
    },
    /**
     * Lookup166: dp_container_chain_genesis_data::Properties
     **/
    DpContainerChainGenesisDataProperties: {
        tokenMetadata: "DpContainerChainGenesisDataTokenMetadata",
        isEthereum: "bool",
    },
    /**
     * Lookup167: dp_container_chain_genesis_data::TokenMetadata
     **/
    DpContainerChainGenesisDataTokenMetadata: {
        tokenSymbol: "Bytes",
        ss58Format: "u32",
        tokenDecimals: "u32",
    },
    /**
     * Lookup171: tp_traits::SlotFrequency
     **/
    TpTraitsSlotFrequency: {
        min: "u32",
        max: "u32",
    },
    /**
     * Lookup173: tp_traits::ParathreadParams
     **/
    TpTraitsParathreadParams: {
        slotFrequency: "TpTraitsSlotFrequency",
    },
    /**
     * Lookup174: sp_trie::storage_proof::StorageProof
     **/
    SpTrieStorageProof: {
        trieNodes: "BTreeSet<Bytes>",
    },
    /**
     * Lookup176: sp_runtime::MultiSignature
     **/
    SpRuntimeMultiSignature: {
        _enum: {
            Ed25519: "[u8;64]",
            Sr25519: "[u8;64]",
            Ecdsa: "[u8;65]",
        },
    },
    /**
     * Lookup179: pallet_configuration::pallet::Call<T>
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
     * Lookup182: pallet_invulnerables::pallet::Call<T>
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
     * Lookup183: pallet_collator_assignment::pallet::Call<T>
     **/
    PalletCollatorAssignmentCall: "Null",
    /**
     * Lookup184: pallet_authority_assignment::pallet::Call<T>
     **/
    PalletAuthorityAssignmentCall: "Null",
    /**
     * Lookup185: pallet_author_noting::pallet::Call<T>
     **/
    PalletAuthorNotingCall: {
        _enum: {
            set_latest_author_data: {
                data: "Null",
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
     * Lookup186: pallet_services_payment::pallet::Call<T>
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
     * Lookup187: pallet_data_preservers::pallet::Call<T>
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
     * Lookup188: pallet_data_preservers::types::Profile<T>
     **/
    PalletDataPreserversProfile: {
        url: "Bytes",
        paraIds: "PalletDataPreserversParaIdsFilter",
        mode: "PalletDataPreserversProfileMode",
        assignmentRequest: "TpDataPreserversCommonProviderRequest",
    },
    /**
     * Lookup190: pallet_data_preservers::types::ParaIdsFilter<T>
     **/
    PalletDataPreserversParaIdsFilter: {
        _enum: {
            AnyParaId: "Null",
            Whitelist: "BTreeSet<u32>",
            Blacklist: "BTreeSet<u32>",
        },
    },
    /**
     * Lookup194: pallet_data_preservers::types::ProfileMode
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
     * Lookup195: tp_data_preservers_common::ProviderRequest
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
     * Lookup196: pallet_stream_payment::pallet::StreamConfig<tp_stream_payment_common::TimeUnit, tp_stream_payment_common::AssetId, BalanceOrDuration>
     **/
    PalletStreamPaymentStreamConfig: {
        timeUnit: "TpStreamPaymentCommonTimeUnit",
        assetId: "TpStreamPaymentCommonAssetId",
        rate: "u128",
        minimumRequestDeadlineDelay: "u128",
        softMinimumDeposit: "u128",
    },
    /**
     * Lookup197: tp_stream_payment_common::TimeUnit
     **/
    TpStreamPaymentCommonTimeUnit: {
        _enum: ["BlockNumber", "Timestamp"],
    },
    /**
     * Lookup198: tp_stream_payment_common::AssetId
     **/
    TpStreamPaymentCommonAssetId: {
        _enum: ["Native"],
    },
    /**
     * Lookup199: tp_data_preservers_common::AssignerExtra
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
     * Lookup200: tp_data_preservers_common::AssignmentWitness
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
     * Lookup201: pallet_external_validators::pallet::Call<T>
     **/
    PalletExternalValidatorsCall: {
        _enum: {
            skip_external_validators: {
                skip: "bool",
            },
            add_whitelisted: {
                who: "AccountId32",
            },
            remove_whitelisted: {
                who: "AccountId32",
            },
            force_era: {
                mode: "PalletExternalValidatorsForcing",
            },
            set_external_validators: {
                validators: "Vec<AccountId32>",
                externalIndex: "u64",
            },
        },
    },
    /**
     * Lookup202: pallet_external_validator_slashes::pallet::Call<T>
     **/
    PalletExternalValidatorSlashesCall: {
        _enum: {
            cancel_deferred_slash: {
                era: "u32",
                slashIndices: "Vec<u32>",
            },
            force_inject_slash: {
                era: "u32",
                validator: "AccountId32",
                percentage: "Perbill",
                externalIdx: "u64",
            },
            root_test_send_msg_to_eth: {
                nonce: "H256",
                numMsgs: "u32",
                msgSize: "u32",
            },
            set_slashing_mode: {
                mode: "PalletExternalValidatorSlashesSlashingModeOption",
            },
        },
    },
    /**
     * Lookup204: pallet_external_validator_slashes::pallet::SlashingModeOption
     **/
    PalletExternalValidatorSlashesSlashingModeOption: {
        _enum: ["Enabled", "LogOnly", "Disabled"],
    },
    /**
     * Lookup205: snowbridge_pallet_outbound_queue::pallet::Call<T>
     **/
    SnowbridgePalletOutboundQueueCall: {
        _enum: {
            set_operating_mode: {
                mode: "SnowbridgeCoreOperatingModeBasicOperatingMode",
            },
        },
    },
    /**
     * Lookup206: snowbridge_pallet_inbound_queue::pallet::Call<T>
     **/
    SnowbridgePalletInboundQueueCall: {
        _enum: {
            submit: {
                message: "SnowbridgeCoreInboundMessage",
            },
            set_operating_mode: {
                mode: "SnowbridgeCoreOperatingModeBasicOperatingMode",
            },
        },
    },
    /**
     * Lookup207: snowbridge_core::inbound::Message
     **/
    SnowbridgeCoreInboundMessage: {
        eventLog: "SnowbridgeCoreInboundLog",
        proof: "SnowbridgeCoreInboundProof",
    },
    /**
     * Lookup208: snowbridge_core::inbound::Log
     **/
    SnowbridgeCoreInboundLog: {
        address: "H160",
        topics: "Vec<H256>",
        data: "Bytes",
    },
    /**
     * Lookup210: snowbridge_core::inbound::Proof
     **/
    SnowbridgeCoreInboundProof: {
        receiptProof: "(Vec<Bytes>,Vec<Bytes>)",
        executionProof: "SnowbridgeBeaconPrimitivesExecutionProof",
    },
    /**
     * Lookup212: snowbridge_beacon_primitives::types::ExecutionProof
     **/
    SnowbridgeBeaconPrimitivesExecutionProof: {
        header: "SnowbridgeBeaconPrimitivesBeaconHeader",
        ancestryProof: "Option<SnowbridgeBeaconPrimitivesAncestryProof>",
        executionHeader: "SnowbridgeBeaconPrimitivesVersionedExecutionPayloadHeader",
        executionBranch: "Vec<H256>",
    },
    /**
     * Lookup213: snowbridge_beacon_primitives::types::BeaconHeader
     **/
    SnowbridgeBeaconPrimitivesBeaconHeader: {
        slot: "u64",
        proposerIndex: "u64",
        parentRoot: "H256",
        stateRoot: "H256",
        bodyRoot: "H256",
    },
    /**
     * Lookup215: snowbridge_beacon_primitives::types::AncestryProof
     **/
    SnowbridgeBeaconPrimitivesAncestryProof: {
        headerBranch: "Vec<H256>",
        finalizedBlockRoot: "H256",
    },
    /**
     * Lookup216: snowbridge_beacon_primitives::types::VersionedExecutionPayloadHeader
     **/
    SnowbridgeBeaconPrimitivesVersionedExecutionPayloadHeader: {
        _enum: {
            Capella: "SnowbridgeBeaconPrimitivesExecutionPayloadHeader",
            Deneb: "SnowbridgeBeaconPrimitivesDenebExecutionPayloadHeader",
        },
    },
    /**
     * Lookup217: snowbridge_beacon_primitives::types::ExecutionPayloadHeader
     **/
    SnowbridgeBeaconPrimitivesExecutionPayloadHeader: {
        parentHash: "H256",
        feeRecipient: "H160",
        stateRoot: "H256",
        receiptsRoot: "H256",
        logsBloom: "Bytes",
        prevRandao: "H256",
        blockNumber: "u64",
        gasLimit: "u64",
        gasUsed: "u64",
        timestamp: "u64",
        extraData: "Bytes",
        baseFeePerGas: "U256",
        blockHash: "H256",
        transactionsRoot: "H256",
        withdrawalsRoot: "H256",
    },
    /**
     * Lookup218: snowbridge_beacon_primitives::types::deneb::ExecutionPayloadHeader
     **/
    SnowbridgeBeaconPrimitivesDenebExecutionPayloadHeader: {
        parentHash: "H256",
        feeRecipient: "H160",
        stateRoot: "H256",
        receiptsRoot: "H256",
        logsBloom: "Bytes",
        prevRandao: "H256",
        blockNumber: "u64",
        gasLimit: "u64",
        gasUsed: "u64",
        timestamp: "u64",
        extraData: "Bytes",
        baseFeePerGas: "U256",
        blockHash: "H256",
        transactionsRoot: "H256",
        withdrawalsRoot: "H256",
        blobGasUsed: "u64",
        excessBlobGas: "u64",
    },
    /**
     * Lookup219: snowbridge_pallet_system::pallet::Call<T>
     **/
    SnowbridgePalletSystemCall: {
        _enum: {
            upgrade: {
                implAddress: "H160",
                implCodeHash: "H256",
                initializer: "Option<SnowbridgeCoreOutboundV1Initializer>",
            },
            set_operating_mode: {
                mode: "SnowbridgeCoreOutboundV1OperatingMode",
            },
            set_pricing_parameters: {
                params: "SnowbridgeCorePricingPricingParameters",
            },
            create_agent: "Null",
            create_channel: {
                mode: "SnowbridgeCoreOutboundV1OperatingMode",
            },
            update_channel: {
                mode: "SnowbridgeCoreOutboundV1OperatingMode",
            },
            force_update_channel: {
                channelId: "SnowbridgeCoreChannelId",
                mode: "SnowbridgeCoreOutboundV1OperatingMode",
            },
            transfer_native_from_agent: {
                recipient: "H160",
                amount: "u128",
            },
            force_transfer_native_from_agent: {
                location: "XcmVersionedLocation",
                recipient: "H160",
                amount: "u128",
            },
            set_token_transfer_fees: {
                createAssetXcm: "u128",
                transferAssetXcm: "u128",
                registerToken: "U256",
            },
            register_token: {
                location: "XcmVersionedLocation",
                metadata: "SnowbridgeCoreAssetMetadata",
            },
        },
    },
    /**
     * Lookup221: snowbridge_core::outbound::v1::Initializer
     **/
    SnowbridgeCoreOutboundV1Initializer: {
        params: "Bytes",
        maximumRequiredGas: "u64",
    },
    /**
     * Lookup222: snowbridge_core::AssetMetadata
     **/
    SnowbridgeCoreAssetMetadata: {
        name: "Bytes",
        symbol: "Bytes",
        decimals: "u8",
    },
    /**
     * Lookup224: pallet_ethereum_token_transfers::pallet::Call<T>
     **/
    PalletEthereumTokenTransfersCall: {
        _enum: {
            set_token_transfer_channel: {
                channelId: "SnowbridgeCoreChannelId",
                agentId: "H256",
                paraId: "u32",
            },
            transfer_native_token: {
                amount: "u128",
                recipient: "H160",
            },
        },
    },
    /**
     * Lookup225: pallet_session::pallet::Call<T>
     **/
    PalletSessionCall: {
        _enum: {
            set_keys: {
                _alias: {
                    keys_: "keys",
                },
                keys_: "DancelightRuntimeSessionKeys",
                proof: "Bytes",
            },
            purge_keys: "Null",
        },
    },
    /**
     * Lookup226: dancelight_runtime::SessionKeys
     **/
    DancelightRuntimeSessionKeys: {
        grandpa: "SpConsensusGrandpaAppPublic",
        babe: "SpConsensusBabeAppPublic",
        paraValidator: "PolkadotPrimitivesV8ValidatorAppPublic",
        paraAssignment: "PolkadotPrimitivesV8AssignmentAppPublic",
        authorityDiscovery: "SpAuthorityDiscoveryAppPublic",
        beefy: "SpConsensusBeefyEcdsaCryptoPublic",
        nimbus: "NimbusPrimitivesNimbusCryptoPublic",
    },
    /**
     * Lookup227: polkadot_primitives::v8::validator_app::Public
     **/
    PolkadotPrimitivesV8ValidatorAppPublic: "[u8;32]",
    /**
     * Lookup228: polkadot_primitives::v8::assignment_app::Public
     **/
    PolkadotPrimitivesV8AssignmentAppPublic: "[u8;32]",
    /**
     * Lookup229: sp_authority_discovery::app::Public
     **/
    SpAuthorityDiscoveryAppPublic: "[u8;32]",
    /**
     * Lookup230: sp_consensus_beefy::ecdsa_crypto::Public
     **/
    SpConsensusBeefyEcdsaCryptoPublic: "[u8;33]",
    /**
     * Lookup232: nimbus_primitives::nimbus_crypto::Public
     **/
    NimbusPrimitivesNimbusCryptoPublic: "[u8;32]",
    /**
     * Lookup233: pallet_grandpa::pallet::Call<T>
     **/
    PalletGrandpaCall: {
        _enum: {
            report_equivocation: {
                equivocationProof: "SpConsensusGrandpaEquivocationProof",
                keyOwnerProof: "SpSessionMembershipProof",
            },
            report_equivocation_unsigned: {
                equivocationProof: "SpConsensusGrandpaEquivocationProof",
                keyOwnerProof: "SpSessionMembershipProof",
            },
            note_stalled: {
                delay: "u32",
                bestFinalizedBlockNumber: "u32",
            },
        },
    },
    /**
     * Lookup234: sp_consensus_grandpa::EquivocationProof<primitive_types::H256, N>
     **/
    SpConsensusGrandpaEquivocationProof: {
        setId: "u64",
        equivocation: "SpConsensusGrandpaEquivocation",
    },
    /**
     * Lookup235: sp_consensus_grandpa::Equivocation<primitive_types::H256, N>
     **/
    SpConsensusGrandpaEquivocation: {
        _enum: {
            Prevote: "FinalityGrandpaEquivocationPrevote",
            Precommit: "FinalityGrandpaEquivocationPrecommit",
        },
    },
    /**
     * Lookup236: finality_grandpa::Equivocation<sp_consensus_grandpa::app::Public, finality_grandpa::Prevote<primitive_types::H256, N>, sp_consensus_grandpa::app::Signature>
     **/
    FinalityGrandpaEquivocationPrevote: {
        roundNumber: "u64",
        identity: "SpConsensusGrandpaAppPublic",
        first: "(FinalityGrandpaPrevote,SpConsensusGrandpaAppSignature)",
        second: "(FinalityGrandpaPrevote,SpConsensusGrandpaAppSignature)",
    },
    /**
     * Lookup237: finality_grandpa::Prevote<primitive_types::H256, N>
     **/
    FinalityGrandpaPrevote: {
        targetHash: "H256",
        targetNumber: "u32",
    },
    /**
     * Lookup238: sp_consensus_grandpa::app::Signature
     **/
    SpConsensusGrandpaAppSignature: "[u8;64]",
    /**
     * Lookup240: finality_grandpa::Equivocation<sp_consensus_grandpa::app::Public, finality_grandpa::Precommit<primitive_types::H256, N>, sp_consensus_grandpa::app::Signature>
     **/
    FinalityGrandpaEquivocationPrecommit: {
        roundNumber: "u64",
        identity: "SpConsensusGrandpaAppPublic",
        first: "(FinalityGrandpaPrecommit,SpConsensusGrandpaAppSignature)",
        second: "(FinalityGrandpaPrecommit,SpConsensusGrandpaAppSignature)",
    },
    /**
     * Lookup241: finality_grandpa::Precommit<primitive_types::H256, N>
     **/
    FinalityGrandpaPrecommit: {
        targetHash: "H256",
        targetNumber: "u32",
    },
    /**
     * Lookup243: pallet_pooled_staking::pallet::Call<T>
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
     * Lookup244: pallet_pooled_staking::pools::PoolKind
     **/
    PalletPooledStakingPoolsPoolKind: {
        _enum: ["Joining", "AutoCompounding", "ManualRewards", "Leaving"],
    },
    /**
     * Lookup246: pallet_pooled_staking::pallet::PendingOperationQuery<sp_core::crypto::AccountId32, J, L>
     **/
    PalletPooledStakingPendingOperationQuery: {
        delegator: "AccountId32",
        operation: "PalletPooledStakingPendingOperationKey",
    },
    /**
     * Lookup247: pallet_pooled_staking::pallet::PendingOperationKey<sp_core::crypto::AccountId32, J, L>
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
     * Lookup248: pallet_pooled_staking::pallet::SharesOrStake<T>
     **/
    PalletPooledStakingSharesOrStake: {
        _enum: {
            Shares: "u128",
            Stake: "u128",
        },
    },
    /**
     * Lookup251: pallet_inactivity_tracking::pallet::Call<T>
     **/
    PalletInactivityTrackingCall: {
        _enum: {
            set_inactivity_tracking_status: {
                enableInactivityTracking: "bool",
            },
        },
    },
    /**
     * Lookup252: pallet_treasury::pallet::Call<T, I>
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
     * Lookup253: pallet_conviction_voting::pallet::Call<T, I>
     **/
    PalletConvictionVotingCall: {
        _enum: {
            vote: {
                pollIndex: "Compact<u32>",
                vote: "PalletConvictionVotingVoteAccountVote",
            },
            delegate: {
                class: "u16",
                to: "MultiAddress",
                conviction: "PalletConvictionVotingConviction",
                balance: "u128",
            },
            undelegate: {
                class: "u16",
            },
            unlock: {
                class: "u16",
                target: "MultiAddress",
            },
            remove_vote: {
                class: "Option<u16>",
                index: "u32",
            },
            remove_other_vote: {
                target: "MultiAddress",
                class: "u16",
                index: "u32",
            },
        },
    },
    /**
     * Lookup254: pallet_conviction_voting::conviction::Conviction
     **/
    PalletConvictionVotingConviction: {
        _enum: ["None", "Locked1x", "Locked2x", "Locked3x", "Locked4x", "Locked5x", "Locked6x"],
    },
    /**
     * Lookup256: pallet_referenda::pallet::Call<T, I>
     **/
    PalletReferendaCall: {
        _enum: {
            submit: {
                proposalOrigin: "DancelightRuntimeOriginCaller",
                proposal: "FrameSupportPreimagesBounded",
                enactmentMoment: "FrameSupportScheduleDispatchTime",
            },
            place_decision_deposit: {
                index: "u32",
            },
            refund_decision_deposit: {
                index: "u32",
            },
            cancel: {
                index: "u32",
            },
            kill: {
                index: "u32",
            },
            nudge_referendum: {
                index: "u32",
            },
            one_fewer_deciding: {
                track: "u16",
            },
            refund_submission_deposit: {
                index: "u32",
            },
            set_metadata: {
                index: "u32",
                maybeHash: "Option<H256>",
            },
        },
    },
    /**
     * Lookup257: dancelight_runtime::OriginCaller
     **/
    DancelightRuntimeOriginCaller: {
        _enum: {
            system: "FrameSupportDispatchRawOrigin",
            __Unused1: "Null",
            __Unused2: "Null",
            __Unused3: "Null",
            Void: "SpCoreVoid",
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
            Origins: "DancelightRuntimeGovernanceOriginsPalletCustomOriginsOrigin",
            __Unused46: "Null",
            __Unused47: "Null",
            __Unused48: "Null",
            __Unused49: "Null",
            ParachainsOrigin: "PolkadotRuntimeParachainsOriginPalletOrigin",
            __Unused51: "Null",
            __Unused52: "Null",
            __Unused53: "Null",
            __Unused54: "Null",
            __Unused55: "Null",
            __Unused56: "Null",
            __Unused57: "Null",
            __Unused58: "Null",
            __Unused59: "Null",
            __Unused60: "Null",
            __Unused61: "Null",
            __Unused62: "Null",
            __Unused63: "Null",
            __Unused64: "Null",
            __Unused65: "Null",
            __Unused66: "Null",
            __Unused67: "Null",
            __Unused68: "Null",
            __Unused69: "Null",
            __Unused70: "Null",
            __Unused71: "Null",
            __Unused72: "Null",
            __Unused73: "Null",
            __Unused74: "Null",
            __Unused75: "Null",
            __Unused76: "Null",
            __Unused77: "Null",
            __Unused78: "Null",
            __Unused79: "Null",
            __Unused80: "Null",
            __Unused81: "Null",
            __Unused82: "Null",
            __Unused83: "Null",
            __Unused84: "Null",
            __Unused85: "Null",
            __Unused86: "Null",
            __Unused87: "Null",
            __Unused88: "Null",
            __Unused89: "Null",
            XcmPallet: "PalletXcmOrigin",
        },
    },
    /**
     * Lookup258: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
     **/
    FrameSupportDispatchRawOrigin: {
        _enum: {
            Root: "Null",
            Signed: "AccountId32",
            None: "Null",
        },
    },
    /**
     * Lookup259: dancelight_runtime::governance::origins::pallet_custom_origins::Origin
     **/
    DancelightRuntimeGovernanceOriginsPalletCustomOriginsOrigin: {
        _enum: [
            "StakingAdmin",
            "Treasurer",
            "FellowshipAdmin",
            "GeneralAdmin",
            "AuctionAdmin",
            "LeaseAdmin",
            "ReferendumCanceller",
            "ReferendumKiller",
            "SmallTipper",
            "BigTipper",
            "SmallSpender",
            "MediumSpender",
            "BigSpender",
            "WhitelistedCaller",
            "FellowshipInitiates",
            "Fellows",
            "FellowshipExperts",
            "FellowshipMasters",
            "Fellowship1Dan",
            "Fellowship2Dan",
            "Fellowship3Dan",
            "Fellowship4Dan",
            "Fellowship5Dan",
            "Fellowship6Dan",
            "Fellowship7Dan",
            "Fellowship8Dan",
            "Fellowship9Dan",
        ],
    },
    /**
     * Lookup260: polkadot_runtime_parachains::origin::pallet::Origin
     **/
    PolkadotRuntimeParachainsOriginPalletOrigin: {
        _enum: {
            Parachain: "u32",
        },
    },
    /**
     * Lookup261: pallet_xcm::pallet::Origin
     **/
    PalletXcmOrigin: {
        _enum: {
            Xcm: "StagingXcmV5Location",
            Response: "StagingXcmV5Location",
        },
    },
    /**
     * Lookup262: sp_core::Void
     **/
    SpCoreVoid: "Null",
    /**
     * Lookup263: frame_support::traits::schedule::DispatchTime<BlockNumber>
     **/
    FrameSupportScheduleDispatchTime: {
        _enum: {
            At: "u32",
            After: "u32",
        },
    },
    /**
     * Lookup264: pallet_ranked_collective::pallet::Call<T, I>
     **/
    PalletRankedCollectiveCall: {
        _enum: {
            add_member: {
                who: "MultiAddress",
            },
            promote_member: {
                who: "MultiAddress",
            },
            demote_member: {
                who: "MultiAddress",
            },
            remove_member: {
                who: "MultiAddress",
                minRank: "u16",
            },
            vote: {
                poll: "u32",
                aye: "bool",
            },
            cleanup_poll: {
                pollIndex: "u32",
                max: "u32",
            },
            exchange_member: {
                who: "MultiAddress",
                newWho: "MultiAddress",
            },
        },
    },
    /**
     * Lookup266: pallet_whitelist::pallet::Call<T>
     **/
    PalletWhitelistCall: {
        _enum: {
            whitelist_call: {
                callHash: "H256",
            },
            remove_whitelisted_call: {
                callHash: "H256",
            },
            dispatch_whitelisted_call: {
                callHash: "H256",
                callEncodedLen: "u32",
                callWeightWitness: "SpWeightsWeightV2Weight",
            },
            dispatch_whitelisted_call_with_preimage: {
                call: "Call",
            },
        },
    },
    /**
     * Lookup267: polkadot_runtime_parachains::configuration::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsConfigurationPalletCall: {
        _enum: {
            set_validation_upgrade_cooldown: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_validation_upgrade_delay: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_code_retention_period: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_max_code_size: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_max_pov_size: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_max_head_data_size: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_coretime_cores: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            __Unused7: "Null",
            set_group_rotation_frequency: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_paras_availability_period: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            __Unused10: "Null",
            set_scheduling_lookahead: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_max_validators_per_core: {
                _alias: {
                    new_: "new",
                },
                new_: "Option<u32>",
            },
            set_max_validators: {
                _alias: {
                    new_: "new",
                },
                new_: "Option<u32>",
            },
            set_dispute_period: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_dispute_post_conclusion_acceptance_period: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            __Unused16: "Null",
            __Unused17: "Null",
            set_no_show_slots: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_n_delay_tranches: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_zeroth_delay_tranche_width: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_needed_approvals: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_relay_vrf_modulo_samples: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_max_upward_queue_count: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_max_upward_queue_size: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_max_downward_message_size: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            __Unused26: "Null",
            set_max_upward_message_size: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_max_upward_message_num_per_candidate: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_hrmp_open_request_ttl: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_hrmp_sender_deposit: {
                _alias: {
                    new_: "new",
                },
                new_: "u128",
            },
            set_hrmp_recipient_deposit: {
                _alias: {
                    new_: "new",
                },
                new_: "u128",
            },
            set_hrmp_channel_max_capacity: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_hrmp_channel_max_total_size: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_hrmp_max_parachain_inbound_channels: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            __Unused35: "Null",
            set_hrmp_channel_max_message_size: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_hrmp_max_parachain_outbound_channels: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            __Unused38: "Null",
            set_hrmp_max_message_num_per_candidate: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            __Unused40: "Null",
            __Unused41: "Null",
            set_pvf_voting_ttl: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_minimum_validation_upgrade_delay: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_bypass_consistency_check: {
                _alias: {
                    new_: "new",
                },
                new_: "bool",
            },
            set_async_backing_params: {
                _alias: {
                    new_: "new",
                },
                new_: "PolkadotPrimitivesV8AsyncBackingAsyncBackingParams",
            },
            set_executor_params: {
                _alias: {
                    new_: "new",
                },
                new_: "PolkadotPrimitivesV8ExecutorParams",
            },
            set_on_demand_base_fee: {
                _alias: {
                    new_: "new",
                },
                new_: "u128",
            },
            set_on_demand_fee_variability: {
                _alias: {
                    new_: "new",
                },
                new_: "Perbill",
            },
            set_on_demand_queue_max_size: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_on_demand_target_queue_utilization: {
                _alias: {
                    new_: "new",
                },
                new_: "Perbill",
            },
            __Unused51: "Null",
            set_minimum_backing_votes: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_node_feature: {
                index: "u8",
                value: "bool",
            },
            set_approval_voting_params: {
                _alias: {
                    new_: "new",
                },
                new_: "PolkadotPrimitivesV8ApprovalVotingParams",
            },
            set_scheduler_params: {
                _alias: {
                    new_: "new",
                },
                new_: "PolkadotPrimitivesV8SchedulerParams",
            },
        },
    },
    /**
     * Lookup268: polkadot_primitives::v8::async_backing::AsyncBackingParams
     **/
    PolkadotPrimitivesV8AsyncBackingAsyncBackingParams: {
        maxCandidateDepth: "u32",
        allowedAncestryLen: "u32",
    },
    /**
     * Lookup269: polkadot_primitives::v8::executor_params::ExecutorParams
     **/
    PolkadotPrimitivesV8ExecutorParams: "Vec<PolkadotPrimitivesV8ExecutorParamsExecutorParam>",
    /**
     * Lookup271: polkadot_primitives::v8::executor_params::ExecutorParam
     **/
    PolkadotPrimitivesV8ExecutorParamsExecutorParam: {
        _enum: {
            __Unused0: "Null",
            MaxMemoryPages: "u32",
            StackLogicalMax: "u32",
            StackNativeMax: "u32",
            PrecheckingMaxMemory: "u64",
            PvfPrepTimeout: "(PolkadotPrimitivesV8PvfPrepKind,u64)",
            PvfExecTimeout: "(PolkadotPrimitivesV8PvfExecKind,u64)",
            WasmExtBulkMemory: "Null",
        },
    },
    /**
     * Lookup272: polkadot_primitives::v8::PvfPrepKind
     **/
    PolkadotPrimitivesV8PvfPrepKind: {
        _enum: ["Precheck", "Prepare"],
    },
    /**
     * Lookup273: polkadot_primitives::v8::PvfExecKind
     **/
    PolkadotPrimitivesV8PvfExecKind: {
        _enum: ["Backing", "Approval"],
    },
    /**
     * Lookup274: polkadot_primitives::v8::ApprovalVotingParams
     **/
    PolkadotPrimitivesV8ApprovalVotingParams: {
        maxApprovalCoalesceCount: "u32",
    },
    /**
     * Lookup275: polkadot_primitives::v8::SchedulerParams<BlockNumber>
     **/
    PolkadotPrimitivesV8SchedulerParams: {
        groupRotationFrequency: "u32",
        parasAvailabilityPeriod: "u32",
        maxValidatorsPerCore: "Option<u32>",
        lookahead: "u32",
        numCores: "u32",
        maxAvailabilityTimeouts: "u32",
        onDemandQueueMaxSize: "u32",
        onDemandTargetQueueUtilization: "Perbill",
        onDemandFeeVariability: "Perbill",
        onDemandBaseFee: "u128",
        ttl: "u32",
    },
    /**
     * Lookup276: polkadot_runtime_parachains::shared::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsSharedPalletCall: "Null",
    /**
     * Lookup277: polkadot_runtime_parachains::inclusion::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsInclusionPalletCall: "Null",
    /**
     * Lookup278: polkadot_runtime_parachains::paras_inherent::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsParasInherentPalletCall: {
        _enum: {
            enter: {
                data: "PolkadotPrimitivesVstagingInherentData",
            },
        },
    },
    /**
     * Lookup279: polkadot_primitives::vstaging::InherentData<sp_runtime::generic::header::Header<Number, Hash>>
     **/
    PolkadotPrimitivesVstagingInherentData: {
        bitfields: "Vec<PolkadotPrimitivesV8SignedUncheckedSigned>",
        backedCandidates: "Vec<PolkadotPrimitivesVstagingBackedCandidate>",
        disputes: "Vec<PolkadotPrimitivesV8DisputeStatementSet>",
        parentHeader: "SpRuntimeHeader",
    },
    /**
     * Lookup281: polkadot_primitives::v8::signed::UncheckedSigned<polkadot_primitives::v8::AvailabilityBitfield, polkadot_primitives::v8::AvailabilityBitfield>
     **/
    PolkadotPrimitivesV8SignedUncheckedSigned: {
        payload: "BitVec",
        validatorIndex: "u32",
        signature: "PolkadotPrimitivesV8ValidatorAppSignature",
    },
    /**
     * Lookup284: bitvec::order::Lsb0
     **/
    BitvecOrderLsb0: "Null",
    /**
     * Lookup286: polkadot_primitives::v8::validator_app::Signature
     **/
    PolkadotPrimitivesV8ValidatorAppSignature: "[u8;64]",
    /**
     * Lookup288: polkadot_primitives::vstaging::BackedCandidate<primitive_types::H256>
     **/
    PolkadotPrimitivesVstagingBackedCandidate: {
        candidate: "PolkadotPrimitivesVstagingCommittedCandidateReceiptV2",
        validityVotes: "Vec<PolkadotPrimitivesV8ValidityAttestation>",
        validatorIndices: "BitVec",
    },
    /**
     * Lookup289: polkadot_primitives::vstaging::CommittedCandidateReceiptV2<primitive_types::H256>
     **/
    PolkadotPrimitivesVstagingCommittedCandidateReceiptV2: {
        descriptor: "PolkadotPrimitivesVstagingCandidateDescriptorV2",
        commitments: "PolkadotPrimitivesV8CandidateCommitments",
    },
    /**
     * Lookup290: polkadot_primitives::vstaging::CandidateDescriptorV2<primitive_types::H256>
     **/
    PolkadotPrimitivesVstagingCandidateDescriptorV2: {
        paraId: "u32",
        relayParent: "H256",
        version: "u8",
        coreIndex: "u16",
        sessionIndex: "u32",
        reserved1: "[u8;25]",
        persistedValidationDataHash: "H256",
        povHash: "H256",
        erasureRoot: "H256",
        reserved2: "[u8;64]",
        paraHead: "H256",
        validationCodeHash: "H256",
    },
    /**
     * Lookup294: polkadot_primitives::v8::CandidateCommitments<N>
     **/
    PolkadotPrimitivesV8CandidateCommitments: {
        upwardMessages: "Vec<Bytes>",
        horizontalMessages: "Vec<PolkadotCorePrimitivesOutboundHrmpMessage>",
        newValidationCode: "Option<Bytes>",
        headData: "Bytes",
        processedDownwardMessages: "u32",
        hrmpWatermark: "u32",
    },
    /**
     * Lookup297: polkadot_core_primitives::OutboundHrmpMessage<polkadot_parachain_primitives::primitives::Id>
     **/
    PolkadotCorePrimitivesOutboundHrmpMessage: {
        recipient: "u32",
        data: "Bytes",
    },
    /**
     * Lookup302: polkadot_primitives::v8::ValidityAttestation
     **/
    PolkadotPrimitivesV8ValidityAttestation: {
        _enum: {
            __Unused0: "Null",
            Implicit: "PolkadotPrimitivesV8ValidatorAppSignature",
            Explicit: "PolkadotPrimitivesV8ValidatorAppSignature",
        },
    },
    /**
     * Lookup304: polkadot_primitives::v8::DisputeStatementSet
     **/
    PolkadotPrimitivesV8DisputeStatementSet: {
        candidateHash: "H256",
        session: "u32",
        statements: "Vec<(PolkadotPrimitivesV8DisputeStatement,u32,PolkadotPrimitivesV8ValidatorAppSignature)>",
    },
    /**
     * Lookup308: polkadot_primitives::v8::DisputeStatement
     **/
    PolkadotPrimitivesV8DisputeStatement: {
        _enum: {
            Valid: "PolkadotPrimitivesV8ValidDisputeStatementKind",
            Invalid: "PolkadotPrimitivesV8InvalidDisputeStatementKind",
        },
    },
    /**
     * Lookup309: polkadot_primitives::v8::ValidDisputeStatementKind
     **/
    PolkadotPrimitivesV8ValidDisputeStatementKind: {
        _enum: {
            Explicit: "Null",
            BackingSeconded: "H256",
            BackingValid: "H256",
            ApprovalChecking: "Null",
            ApprovalCheckingMultipleCandidates: "Vec<H256>",
        },
    },
    /**
     * Lookup311: polkadot_primitives::v8::InvalidDisputeStatementKind
     **/
    PolkadotPrimitivesV8InvalidDisputeStatementKind: {
        _enum: ["Explicit"],
    },
    /**
     * Lookup312: polkadot_runtime_parachains::paras::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsParasPalletCall: {
        _enum: {
            force_set_current_code: {
                para: "u32",
                newCode: "Bytes",
            },
            force_set_current_head: {
                para: "u32",
                newHead: "Bytes",
            },
            force_schedule_code_upgrade: {
                para: "u32",
                newCode: "Bytes",
                relayParentNumber: "u32",
            },
            force_note_new_head: {
                para: "u32",
                newHead: "Bytes",
            },
            force_queue_action: {
                para: "u32",
            },
            add_trusted_validation_code: {
                validationCode: "Bytes",
            },
            poke_unused_validation_code: {
                validationCodeHash: "H256",
            },
            include_pvf_check_statement: {
                stmt: "PolkadotPrimitivesV8PvfCheckStatement",
                signature: "PolkadotPrimitivesV8ValidatorAppSignature",
            },
            force_set_most_recent_context: {
                para: "u32",
                context: "u32",
            },
        },
    },
    /**
     * Lookup313: polkadot_primitives::v8::PvfCheckStatement
     **/
    PolkadotPrimitivesV8PvfCheckStatement: {
        accept: "bool",
        subject: "H256",
        sessionIndex: "u32",
        validatorIndex: "u32",
    },
    /**
     * Lookup314: polkadot_runtime_parachains::initializer::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsInitializerPalletCall: {
        _enum: {
            force_approve: {
                upTo: "u32",
            },
        },
    },
    /**
     * Lookup315: polkadot_runtime_parachains::hrmp::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsHrmpPalletCall: {
        _enum: {
            hrmp_init_open_channel: {
                recipient: "u32",
                proposedMaxCapacity: "u32",
                proposedMaxMessageSize: "u32",
            },
            hrmp_accept_open_channel: {
                sender: "u32",
            },
            hrmp_close_channel: {
                channelId: "PolkadotParachainPrimitivesPrimitivesHrmpChannelId",
            },
            force_clean_hrmp: {
                para: "u32",
                numInbound: "u32",
                numOutbound: "u32",
            },
            force_process_hrmp_open: {
                channels: "u32",
            },
            force_process_hrmp_close: {
                channels: "u32",
            },
            hrmp_cancel_open_request: {
                channelId: "PolkadotParachainPrimitivesPrimitivesHrmpChannelId",
                openRequests: "u32",
            },
            force_open_hrmp_channel: {
                sender: "u32",
                recipient: "u32",
                maxCapacity: "u32",
                maxMessageSize: "u32",
            },
            establish_system_channel: {
                sender: "u32",
                recipient: "u32",
            },
            poke_channel_deposits: {
                sender: "u32",
                recipient: "u32",
            },
            establish_channel_with_system: {
                targetSystemChain: "u32",
            },
        },
    },
    /**
     * Lookup316: polkadot_parachain_primitives::primitives::HrmpChannelId
     **/
    PolkadotParachainPrimitivesPrimitivesHrmpChannelId: {
        sender: "u32",
        recipient: "u32",
    },
    /**
     * Lookup317: polkadot_runtime_parachains::disputes::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsDisputesPalletCall: {
        _enum: ["force_unfreeze"],
    },
    /**
     * Lookup318: polkadot_runtime_parachains::disputes::slashing::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsDisputesSlashingPalletCall: {
        _enum: {
            report_dispute_lost_unsigned: {
                disputeProof: "PolkadotPrimitivesV8SlashingDisputeProof",
                keyOwnerProof: "SpSessionMembershipProof",
            },
        },
    },
    /**
     * Lookup319: polkadot_primitives::v8::slashing::DisputeProof
     **/
    PolkadotPrimitivesV8SlashingDisputeProof: {
        timeSlot: "PolkadotPrimitivesV8SlashingDisputesTimeSlot",
        kind: "PolkadotPrimitivesV8SlashingSlashingOffenceKind",
        validatorIndex: "u32",
        validatorId: "PolkadotPrimitivesV8ValidatorAppPublic",
    },
    /**
     * Lookup320: polkadot_primitives::v8::slashing::DisputesTimeSlot
     **/
    PolkadotPrimitivesV8SlashingDisputesTimeSlot: {
        sessionIndex: "u32",
        candidateHash: "H256",
    },
    /**
     * Lookup321: polkadot_primitives::v8::slashing::SlashingOffenceKind
     **/
    PolkadotPrimitivesV8SlashingSlashingOffenceKind: {
        _enum: ["ForInvalid", "AgainstValid"],
    },
    /**
     * Lookup322: pallet_message_queue::pallet::Call<T>
     **/
    PalletMessageQueueCall: {
        _enum: {
            reap_page: {
                messageOrigin: "DancelightRuntimeAggregateMessageOrigin",
                pageIndex: "u32",
            },
            execute_overweight: {
                messageOrigin: "DancelightRuntimeAggregateMessageOrigin",
                page: "u32",
                index: "u32",
                weightLimit: "SpWeightsWeightV2Weight",
            },
        },
    },
    /**
     * Lookup323: dancelight_runtime::AggregateMessageOrigin
     **/
    DancelightRuntimeAggregateMessageOrigin: {
        _enum: {
            Ump: "PolkadotRuntimeParachainsInclusionUmpQueueId",
            Snowbridge: "SnowbridgeCoreChannelId",
            SnowbridgeTanssi: "SnowbridgeCoreChannelId",
        },
    },
    /**
     * Lookup324: polkadot_runtime_parachains::inclusion::UmpQueueId
     **/
    PolkadotRuntimeParachainsInclusionUmpQueueId: {
        _enum: {
            Para: "u32",
        },
    },
    /**
     * Lookup325: polkadot_runtime_parachains::on_demand::pallet::Call<T>
     **/
    PolkadotRuntimeParachainsOnDemandPalletCall: {
        _enum: {
            place_order_allow_death: {
                maxAmount: "u128",
                paraId: "u32",
            },
            place_order_keep_alive: {
                maxAmount: "u128",
                paraId: "u32",
            },
        },
    },
    /**
     * Lookup326: polkadot_runtime_common::paras_registrar::pallet::Call<T>
     **/
    PolkadotRuntimeCommonParasRegistrarPalletCall: {
        _enum: {
            register: {
                id: "u32",
                genesisHead: "Bytes",
                validationCode: "Bytes",
            },
            force_register: {
                who: "AccountId32",
                deposit: "u128",
                id: "u32",
                genesisHead: "Bytes",
                validationCode: "Bytes",
            },
            deregister: {
                id: "u32",
            },
            swap: {
                id: "u32",
                other: "u32",
            },
            remove_lock: {
                para: "u32",
            },
            reserve: "Null",
            add_lock: {
                para: "u32",
            },
            schedule_code_upgrade: {
                para: "u32",
                newCode: "Bytes",
            },
            set_current_head: {
                para: "u32",
                newHead: "Bytes",
            },
        },
    },
    /**
     * Lookup327: pallet_utility::pallet::Call<T>
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
                asOrigin: "DancelightRuntimeOriginCaller",
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
     * Lookup329: pallet_identity::pallet::Call<T>
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
     * Lookup330: pallet_identity::legacy::IdentityInfo<FieldLimit>
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
     * Lookup366: pallet_identity::types::Judgement<Balance>
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
     * Lookup368: pallet_scheduler::pallet::Call<T>
     **/
    PalletSchedulerCall: {
        _enum: {
            schedule: {
                when: "u32",
                maybePeriodic: "Option<(u32,u32)>",
                priority: "u8",
                call: "Call",
            },
            cancel: {
                when: "u32",
                index: "u32",
            },
            schedule_named: {
                id: "[u8;32]",
                when: "u32",
                maybePeriodic: "Option<(u32,u32)>",
                priority: "u8",
                call: "Call",
            },
            cancel_named: {
                id: "[u8;32]",
            },
            schedule_after: {
                after: "u32",
                maybePeriodic: "Option<(u32,u32)>",
                priority: "u8",
                call: "Call",
            },
            schedule_named_after: {
                id: "[u8;32]",
                after: "u32",
                maybePeriodic: "Option<(u32,u32)>",
                priority: "u8",
                call: "Call",
            },
            set_retry: {
                task: "(u32,u32)",
                retries: "u8",
                period: "u32",
            },
            set_retry_named: {
                id: "[u8;32]",
                retries: "u8",
                period: "u32",
            },
            cancel_retry: {
                task: "(u32,u32)",
            },
            cancel_retry_named: {
                id: "[u8;32]",
            },
        },
    },
    /**
     * Lookup371: pallet_proxy::pallet::Call<T>
     **/
    PalletProxyCall: {
        _enum: {
            proxy: {
                real: "MultiAddress",
                forceProxyType: "Option<DancelightRuntimeProxyType>",
                call: "Call",
            },
            add_proxy: {
                delegate: "MultiAddress",
                proxyType: "DancelightRuntimeProxyType",
                delay: "u32",
            },
            remove_proxy: {
                delegate: "MultiAddress",
                proxyType: "DancelightRuntimeProxyType",
                delay: "u32",
            },
            remove_proxies: "Null",
            create_pure: {
                proxyType: "DancelightRuntimeProxyType",
                delay: "u32",
                index: "u16",
            },
            kill_pure: {
                spawner: "MultiAddress",
                proxyType: "DancelightRuntimeProxyType",
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
                forceProxyType: "Option<DancelightRuntimeProxyType>",
                call: "Call",
            },
        },
    },
    /**
     * Lookup373: dancelight_runtime::ProxyType
     **/
    DancelightRuntimeProxyType: {
        _enum: [
            "Any",
            "NonTransfer",
            "Governance",
            "IdentityJudgement",
            "CancelProxy",
            "Auction",
            "OnDemandOrdering",
            "SudoRegistrar",
            "SudoValidatorManagement",
            "SessionKeyManagement",
            "Staking",
        ],
    },
    /**
     * Lookup374: pallet_multisig::pallet::Call<T>
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
     * Lookup376: pallet_multisig::Timepoint<BlockNumber>
     **/
    PalletMultisigTimepoint: {
        height: "u32",
        index: "u32",
    },
    /**
     * Lookup377: pallet_preimage::pallet::Call<T>
     **/
    PalletPreimageCall: {
        _enum: {
            note_preimage: {
                bytes: "Bytes",
            },
            unnote_preimage: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
            request_preimage: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
            unrequest_preimage: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
            ensure_updated: {
                hashes: "Vec<H256>",
            },
        },
    },
    /**
     * Lookup378: pallet_asset_rate::pallet::Call<T>
     **/
    PalletAssetRateCall: {
        _enum: {
            create: {
                assetKind: "Null",
                rate: "u128",
            },
            update: {
                assetKind: "Null",
                rate: "u128",
            },
            remove: {
                assetKind: "Null",
            },
        },
    },
    /**
     * Lookup379: pallet_xcm::pallet::Call<T>
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
     * Lookup380: xcm::VersionedXcm<RuntimeCall>
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
     * Lookup381: xcm::v3::Xcm<Call>
     **/
    XcmV3Xcm: "Vec<XcmV3Instruction>",
    /**
     * Lookup383: xcm::v3::Instruction<Call>
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
     * Lookup384: xcm::v3::multiasset::MultiAssets
     **/
    XcmV3MultiassetMultiAssets: "Vec<XcmV3MultiAsset>",
    /**
     * Lookup386: xcm::v3::multiasset::MultiAsset
     **/
    XcmV3MultiAsset: {
        id: "XcmV3MultiassetAssetId",
        fun: "XcmV3MultiassetFungibility",
    },
    /**
     * Lookup387: xcm::v3::multiasset::AssetId
     **/
    XcmV3MultiassetAssetId: {
        _enum: {
            Concrete: "StagingXcmV3MultiLocation",
            Abstract: "[u8;32]",
        },
    },
    /**
     * Lookup388: xcm::v3::multiasset::Fungibility
     **/
    XcmV3MultiassetFungibility: {
        _enum: {
            Fungible: "Compact<u128>",
            NonFungible: "XcmV3MultiassetAssetInstance",
        },
    },
    /**
     * Lookup389: xcm::v3::multiasset::AssetInstance
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
     * Lookup390: xcm::v3::Response
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
     * Lookup393: xcm::v3::traits::Error
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
     * Lookup395: xcm::v3::PalletInfo
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
     * Lookup398: xcm::v3::MaybeErrorCode
     **/
    XcmV3MaybeErrorCode: {
        _enum: {
            Success: "Null",
            Error: "Bytes",
            TruncatedError: "Bytes",
        },
    },
    /**
     * Lookup401: xcm::v3::OriginKind
     **/
    XcmV3OriginKind: {
        _enum: ["Native", "SovereignAccount", "Superuser", "Xcm"],
    },
    /**
     * Lookup402: xcm::double_encoded::DoubleEncoded<T>
     **/
    XcmDoubleEncoded: {
        encoded: "Bytes",
    },
    /**
     * Lookup403: xcm::v3::QueryResponseInfo
     **/
    XcmV3QueryResponseInfo: {
        destination: "StagingXcmV3MultiLocation",
        queryId: "Compact<u64>",
        maxWeight: "SpWeightsWeightV2Weight",
    },
    /**
     * Lookup404: xcm::v3::multiasset::MultiAssetFilter
     **/
    XcmV3MultiassetMultiAssetFilter: {
        _enum: {
            Definite: "XcmV3MultiassetMultiAssets",
            Wild: "XcmV3MultiassetWildMultiAsset",
        },
    },
    /**
     * Lookup405: xcm::v3::multiasset::WildMultiAsset
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
     * Lookup406: xcm::v3::multiasset::WildFungibility
     **/
    XcmV3MultiassetWildFungibility: {
        _enum: ["Fungible", "NonFungible"],
    },
    /**
     * Lookup407: xcm::v3::WeightLimit
     **/
    XcmV3WeightLimit: {
        _enum: {
            Unlimited: "Null",
            Limited: "SpWeightsWeightV2Weight",
        },
    },
    /**
     * Lookup408: staging_xcm::v4::Xcm<Call>
     **/
    StagingXcmV4Xcm: "Vec<StagingXcmV4Instruction>",
    /**
     * Lookup410: staging_xcm::v4::Instruction<Call>
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
     * Lookup411: staging_xcm::v4::asset::Assets
     **/
    StagingXcmV4AssetAssets: "Vec<StagingXcmV4Asset>",
    /**
     * Lookup413: staging_xcm::v4::asset::Asset
     **/
    StagingXcmV4Asset: {
        id: "StagingXcmV4AssetAssetId",
        fun: "StagingXcmV4AssetFungibility",
    },
    /**
     * Lookup414: staging_xcm::v4::asset::AssetId
     **/
    StagingXcmV4AssetAssetId: "StagingXcmV4Location",
    /**
     * Lookup415: staging_xcm::v4::asset::Fungibility
     **/
    StagingXcmV4AssetFungibility: {
        _enum: {
            Fungible: "Compact<u128>",
            NonFungible: "StagingXcmV4AssetAssetInstance",
        },
    },
    /**
     * Lookup416: staging_xcm::v4::asset::AssetInstance
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
     * Lookup417: staging_xcm::v4::Response
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
     * Lookup419: staging_xcm::v4::PalletInfo
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
     * Lookup423: staging_xcm::v4::QueryResponseInfo
     **/
    StagingXcmV4QueryResponseInfo: {
        destination: "StagingXcmV4Location",
        queryId: "Compact<u64>",
        maxWeight: "SpWeightsWeightV2Weight",
    },
    /**
     * Lookup424: staging_xcm::v4::asset::AssetFilter
     **/
    StagingXcmV4AssetAssetFilter: {
        _enum: {
            Definite: "StagingXcmV4AssetAssets",
            Wild: "StagingXcmV4AssetWildAsset",
        },
    },
    /**
     * Lookup425: staging_xcm::v4::asset::WildAsset
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
     * Lookup426: staging_xcm::v4::asset::WildFungibility
     **/
    StagingXcmV4AssetWildFungibility: {
        _enum: ["Fungible", "NonFungible"],
    },
    /**
     * Lookup427: staging_xcm::v5::Xcm<Call>
     **/
    StagingXcmV5Xcm: "Vec<StagingXcmV5Instruction>",
    /**
     * Lookup429: staging_xcm::v5::Instruction<Call>
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
     * Lookup430: staging_xcm::v5::asset::Assets
     **/
    StagingXcmV5AssetAssets: "Vec<StagingXcmV5Asset>",
    /**
     * Lookup432: staging_xcm::v5::asset::Asset
     **/
    StagingXcmV5Asset: {
        id: "StagingXcmV5AssetAssetId",
        fun: "StagingXcmV5AssetFungibility",
    },
    /**
     * Lookup433: staging_xcm::v5::asset::AssetId
     **/
    StagingXcmV5AssetAssetId: "StagingXcmV5Location",
    /**
     * Lookup434: staging_xcm::v5::asset::Fungibility
     **/
    StagingXcmV5AssetFungibility: {
        _enum: {
            Fungible: "Compact<u128>",
            NonFungible: "StagingXcmV5AssetAssetInstance",
        },
    },
    /**
     * Lookup435: staging_xcm::v5::asset::AssetInstance
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
     * Lookup436: staging_xcm::v5::Response
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
     * Lookup439: xcm::v5::traits::Error
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
     * Lookup441: staging_xcm::v5::PalletInfo
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
     * Lookup446: staging_xcm::v5::QueryResponseInfo
     **/
    StagingXcmV5QueryResponseInfo: {
        destination: "StagingXcmV5Location",
        queryId: "Compact<u64>",
        maxWeight: "SpWeightsWeightV2Weight",
    },
    /**
     * Lookup447: staging_xcm::v5::asset::AssetFilter
     **/
    StagingXcmV5AssetAssetFilter: {
        _enum: {
            Definite: "StagingXcmV5AssetAssets",
            Wild: "StagingXcmV5AssetWildAsset",
        },
    },
    /**
     * Lookup448: staging_xcm::v5::asset::WildAsset
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
     * Lookup449: staging_xcm::v5::asset::WildFungibility
     **/
    StagingXcmV5AssetWildFungibility: {
        _enum: ["Fungible", "NonFungible"],
    },
    /**
     * Lookup451: staging_xcm::v5::asset::AssetTransferFilter
     **/
    StagingXcmV5AssetAssetTransferFilter: {
        _enum: {
            Teleport: "StagingXcmV5AssetAssetFilter",
            ReserveDeposit: "StagingXcmV5AssetAssetFilter",
            ReserveWithdraw: "StagingXcmV5AssetAssetFilter",
        },
    },
    /**
     * Lookup455: staging_xcm::v5::Hint
     **/
    StagingXcmV5Hint: {
        _enum: {
            AssetClaimer: {
                location: "StagingXcmV5Location",
            },
        },
    },
    /**
     * Lookup457: xcm::VersionedAssets
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
     * Lookup469: staging_xcm_executor::traits::asset_transfer::TransferType
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
     * Lookup470: xcm::VersionedAssetId
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
     * Lookup471: pallet_stream_payment::pallet::Call<T>
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
     * Lookup472: pallet_stream_payment::pallet::ChangeKind<Time>
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
     * Lookup474: pallet_stream_payment::pallet::DepositChange<Balance>
     **/
    PalletStreamPaymentDepositChange: {
        _enum: {
            Increase: "u128",
            Decrease: "u128",
            Absolute: "u128",
        },
    },
    /**
     * Lookup475: pallet_migrations::pallet::Call<T>
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
     * Lookup477: pallet_migrations::MigrationCursor<bounded_collections::bounded_vec::BoundedVec<T, S>, BlockNumber>
     **/
    PalletMigrationsMigrationCursor: {
        _enum: {
            Active: "PalletMigrationsActiveCursor",
            Stuck: "Null",
        },
    },
    /**
     * Lookup479: pallet_migrations::ActiveCursor<bounded_collections::bounded_vec::BoundedVec<T, S>, BlockNumber>
     **/
    PalletMigrationsActiveCursor: {
        index: "u32",
        innerCursor: "Option<Bytes>",
        startedAt: "u32",
    },
    /**
     * Lookup481: pallet_migrations::HistoricCleanupSelector<bounded_collections::bounded_vec::BoundedVec<T, S>>
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
     * Lookup485: pallet_beefy::pallet::Call<T>
     **/
    PalletBeefyCall: {
        _enum: {
            report_double_voting: {
                equivocationProof: "SpConsensusBeefyDoubleVotingProof",
                keyOwnerProof: "SpSessionMembershipProof",
            },
            report_double_voting_unsigned: {
                equivocationProof: "SpConsensusBeefyDoubleVotingProof",
                keyOwnerProof: "SpSessionMembershipProof",
            },
            set_new_genesis: {
                delayInBlocks: "u32",
            },
            report_fork_voting: {
                equivocationProof: "SpConsensusBeefyForkVotingProof",
                keyOwnerProof: "SpSessionMembershipProof",
            },
            report_fork_voting_unsigned: {
                equivocationProof: "SpConsensusBeefyForkVotingProof",
                keyOwnerProof: "SpSessionMembershipProof",
            },
            report_future_block_voting: {
                equivocationProof: "SpConsensusBeefyFutureBlockVotingProof",
                keyOwnerProof: "SpSessionMembershipProof",
            },
            report_future_block_voting_unsigned: {
                equivocationProof: "SpConsensusBeefyFutureBlockVotingProof",
                keyOwnerProof: "SpSessionMembershipProof",
            },
        },
    },
    /**
     * Lookup486: sp_consensus_beefy::DoubleVotingProof<Number, sp_consensus_beefy::ecdsa_crypto::Public, sp_consensus_beefy::ecdsa_crypto::Signature>
     **/
    SpConsensusBeefyDoubleVotingProof: {
        first: "SpConsensusBeefyVoteMessage",
        second: "SpConsensusBeefyVoteMessage",
    },
    /**
     * Lookup487: sp_consensus_beefy::ecdsa_crypto::Signature
     **/
    SpConsensusBeefyEcdsaCryptoSignature: "[u8;65]",
    /**
     * Lookup488: sp_consensus_beefy::VoteMessage<Number, sp_consensus_beefy::ecdsa_crypto::Public, sp_consensus_beefy::ecdsa_crypto::Signature>
     **/
    SpConsensusBeefyVoteMessage: {
        commitment: "SpConsensusBeefyCommitment",
        id: "SpConsensusBeefyEcdsaCryptoPublic",
        signature: "SpConsensusBeefyEcdsaCryptoSignature",
    },
    /**
     * Lookup489: sp_consensus_beefy::commitment::Commitment<TBlockNumber>
     **/
    SpConsensusBeefyCommitment: {
        payload: "SpConsensusBeefyPayload",
        blockNumber: "u32",
        validatorSetId: "u64",
    },
    /**
     * Lookup490: sp_consensus_beefy::payload::Payload
     **/
    SpConsensusBeefyPayload: "Vec<([u8;2],Bytes)>",
    /**
     * Lookup493: sp_consensus_beefy::ForkVotingProof<sp_runtime::generic::header::Header<Number, Hash>, sp_consensus_beefy::ecdsa_crypto::Public, sp_mmr_primitives::AncestryProof<primitive_types::H256>>
     **/
    SpConsensusBeefyForkVotingProof: {
        vote: "SpConsensusBeefyVoteMessage",
        ancestryProof: "SpMmrPrimitivesAncestryProof",
        header: "SpRuntimeHeader",
    },
    /**
     * Lookup494: sp_mmr_primitives::AncestryProof<primitive_types::H256>
     **/
    SpMmrPrimitivesAncestryProof: {
        prevPeaks: "Vec<H256>",
        prevLeafCount: "u64",
        leafCount: "u64",
        items: "Vec<(u64,H256)>",
    },
    /**
     * Lookup497: sp_consensus_beefy::FutureBlockVotingProof<Number, sp_consensus_beefy::ecdsa_crypto::Public>
     **/
    SpConsensusBeefyFutureBlockVotingProof: {
        vote: "SpConsensusBeefyVoteMessage",
    },
    /**
     * Lookup498: snowbridge_pallet_ethereum_client::pallet::Call<T>
     **/
    SnowbridgePalletEthereumClientCall: {
        _enum: {
            force_checkpoint: {
                update: "SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate",
            },
            submit: {
                update: "SnowbridgeBeaconPrimitivesUpdatesUpdate",
            },
            __Unused2: "Null",
            set_operating_mode: {
                mode: "SnowbridgeCoreOperatingModeBasicOperatingMode",
            },
        },
    },
    /**
     * Lookup499: snowbridge_beacon_primitives::updates::CheckpointUpdate
     **/
    SnowbridgeBeaconPrimitivesUpdatesCheckpointUpdate: {
        header: "SnowbridgeBeaconPrimitivesBeaconHeader",
        currentSyncCommittee: "SnowbridgeBeaconPrimitivesSyncCommittee",
        currentSyncCommitteeBranch: "Vec<H256>",
        validatorsRoot: "H256",
        blockRootsRoot: "H256",
        blockRootsBranch: "Vec<H256>",
    },
    /**
     * Lookup500: snowbridge_beacon_primitives::types::SyncCommittee
     **/
    SnowbridgeBeaconPrimitivesSyncCommittee: {
        pubkeys: "[[u8;48];512]",
        aggregatePubkey: "SnowbridgeBeaconPrimitivesPublicKey",
    },
    /**
     * Lookup502: snowbridge_beacon_primitives::types::PublicKey
     **/
    SnowbridgeBeaconPrimitivesPublicKey: "[u8;48]",
    /**
     * Lookup504: snowbridge_beacon_primitives::updates::Update
     **/
    SnowbridgeBeaconPrimitivesUpdatesUpdate: {
        attestedHeader: "SnowbridgeBeaconPrimitivesBeaconHeader",
        syncAggregate: "SnowbridgeBeaconPrimitivesSyncAggregate",
        signatureSlot: "u64",
        nextSyncCommitteeUpdate: "Option<SnowbridgeBeaconPrimitivesUpdatesNextSyncCommitteeUpdate>",
        finalizedHeader: "SnowbridgeBeaconPrimitivesBeaconHeader",
        finalityBranch: "Vec<H256>",
        blockRootsRoot: "H256",
        blockRootsBranch: "Vec<H256>",
    },
    /**
     * Lookup505: snowbridge_beacon_primitives::types::SyncAggregate
     **/
    SnowbridgeBeaconPrimitivesSyncAggregate: {
        syncCommitteeBits: "[u8;64]",
        syncCommitteeSignature: "SnowbridgeBeaconPrimitivesSignature",
    },
    /**
     * Lookup506: snowbridge_beacon_primitives::types::Signature
     **/
    SnowbridgeBeaconPrimitivesSignature: "[u8;96]",
    /**
     * Lookup509: snowbridge_beacon_primitives::updates::NextSyncCommitteeUpdate
     **/
    SnowbridgeBeaconPrimitivesUpdatesNextSyncCommitteeUpdate: {
        nextSyncCommittee: "SnowbridgeBeaconPrimitivesSyncCommittee",
        nextSyncCommitteeBranch: "Vec<H256>",
    },
    /**
     * Lookup510: polkadot_runtime_common::paras_sudo_wrapper::pallet::Call<T>
     **/
    PolkadotRuntimeCommonParasSudoWrapperPalletCall: {
        _enum: {
            sudo_schedule_para_initialize: {
                id: "u32",
                genesis: "PolkadotRuntimeParachainsParasParaGenesisArgs",
            },
            sudo_schedule_para_cleanup: {
                id: "u32",
            },
            sudo_schedule_parathread_upgrade: {
                id: "u32",
            },
            sudo_schedule_parachain_downgrade: {
                id: "u32",
            },
            sudo_queue_downward_xcm: {
                id: "u32",
                xcm: "XcmVersionedXcm",
            },
            sudo_establish_hrmp_channel: {
                sender: "u32",
                recipient: "u32",
                maxCapacity: "u32",
                maxMessageSize: "u32",
            },
        },
    },
    /**
     * Lookup511: polkadot_runtime_parachains::paras::ParaGenesisArgs
     **/
    PolkadotRuntimeParachainsParasParaGenesisArgs: {
        genesisHead: "Bytes",
        validationCode: "Bytes",
        paraKind: "bool",
    },
    /**
     * Lookup512: pallet_root_testing::pallet::Call<T>
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
     * Lookup513: pallet_sudo::pallet::Call<T>
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
     * Lookup514: sp_runtime::traits::BlakeTwo256
     **/
    SpRuntimeBlakeTwo256: "Null",
    /**
     * Lookup516: pallet_conviction_voting::types::Tally<Votes, Total>
     **/
    PalletConvictionVotingTally: {
        ayes: "u128",
        nays: "u128",
        support: "u128",
    },
    /**
     * Lookup517: pallet_ranked_collective::pallet::Event<T, I>
     **/
    PalletRankedCollectiveEvent: {
        _enum: {
            MemberAdded: {
                who: "AccountId32",
            },
            RankChanged: {
                who: "AccountId32",
                rank: "u16",
            },
            MemberRemoved: {
                who: "AccountId32",
                rank: "u16",
            },
            Voted: {
                who: "AccountId32",
                poll: "u32",
                vote: "PalletRankedCollectiveVoteRecord",
                tally: "PalletRankedCollectiveTally",
            },
            MemberExchanged: {
                who: "AccountId32",
                newWho: "AccountId32",
            },
        },
    },
    /**
     * Lookup518: pallet_ranked_collective::VoteRecord
     **/
    PalletRankedCollectiveVoteRecord: {
        _enum: {
            Aye: "u32",
            Nay: "u32",
        },
    },
    /**
     * Lookup519: pallet_ranked_collective::Tally<T, I, M>
     **/
    PalletRankedCollectiveTally: {
        bareAyes: "u32",
        ayes: "u32",
        nays: "u32",
    },
    /**
     * Lookup521: pallet_whitelist::pallet::Event<T>
     **/
    PalletWhitelistEvent: {
        _enum: {
            CallWhitelisted: {
                callHash: "H256",
            },
            WhitelistedCallRemoved: {
                callHash: "H256",
            },
            WhitelistedCallDispatched: {
                callHash: "H256",
                result: "Result<FrameSupportDispatchPostDispatchInfo, SpRuntimeDispatchErrorWithPostInfo>",
            },
        },
    },
    /**
     * Lookup523: frame_support::dispatch::PostDispatchInfo
     **/
    FrameSupportDispatchPostDispatchInfo: {
        actualWeight: "Option<SpWeightsWeightV2Weight>",
        paysFee: "FrameSupportDispatchPays",
    },
    /**
     * Lookup524: sp_runtime::DispatchErrorWithPostInfo<frame_support::dispatch::PostDispatchInfo>
     **/
    SpRuntimeDispatchErrorWithPostInfo: {
        postInfo: "FrameSupportDispatchPostDispatchInfo",
        error: "SpRuntimeDispatchError",
    },
    /**
     * Lookup525: polkadot_runtime_parachains::inclusion::pallet::Event<T>
     **/
    PolkadotRuntimeParachainsInclusionPalletEvent: {
        _enum: {
            CandidateBacked: "(PolkadotPrimitivesVstagingCandidateReceiptV2,Bytes,u32,u32)",
            CandidateIncluded: "(PolkadotPrimitivesVstagingCandidateReceiptV2,Bytes,u32,u32)",
            CandidateTimedOut: "(PolkadotPrimitivesVstagingCandidateReceiptV2,Bytes,u32)",
            UpwardMessagesReceived: {
                from: "u32",
                count: "u32",
            },
        },
    },
    /**
     * Lookup526: polkadot_primitives::vstaging::CandidateReceiptV2<primitive_types::H256>
     **/
    PolkadotPrimitivesVstagingCandidateReceiptV2: {
        descriptor: "PolkadotPrimitivesVstagingCandidateDescriptorV2",
        commitmentsHash: "H256",
    },
    /**
     * Lookup529: polkadot_runtime_parachains::paras::pallet::Event
     **/
    PolkadotRuntimeParachainsParasPalletEvent: {
        _enum: {
            CurrentCodeUpdated: "u32",
            CurrentHeadUpdated: "u32",
            CodeUpgradeScheduled: "u32",
            NewHeadNoted: "u32",
            ActionQueued: "(u32,u32)",
            PvfCheckStarted: "(H256,u32)",
            PvfCheckAccepted: "(H256,u32)",
            PvfCheckRejected: "(H256,u32)",
        },
    },
    /**
     * Lookup530: polkadot_runtime_parachains::hrmp::pallet::Event<T>
     **/
    PolkadotRuntimeParachainsHrmpPalletEvent: {
        _enum: {
            OpenChannelRequested: {
                sender: "u32",
                recipient: "u32",
                proposedMaxCapacity: "u32",
                proposedMaxMessageSize: "u32",
            },
            OpenChannelCanceled: {
                byParachain: "u32",
                channelId: "PolkadotParachainPrimitivesPrimitivesHrmpChannelId",
            },
            OpenChannelAccepted: {
                sender: "u32",
                recipient: "u32",
            },
            ChannelClosed: {
                byParachain: "u32",
                channelId: "PolkadotParachainPrimitivesPrimitivesHrmpChannelId",
            },
            HrmpChannelForceOpened: {
                sender: "u32",
                recipient: "u32",
                proposedMaxCapacity: "u32",
                proposedMaxMessageSize: "u32",
            },
            HrmpSystemChannelOpened: {
                sender: "u32",
                recipient: "u32",
                proposedMaxCapacity: "u32",
                proposedMaxMessageSize: "u32",
            },
            OpenChannelDepositsUpdated: {
                sender: "u32",
                recipient: "u32",
            },
        },
    },
    /**
     * Lookup531: polkadot_runtime_parachains::disputes::pallet::Event<T>
     **/
    PolkadotRuntimeParachainsDisputesPalletEvent: {
        _enum: {
            DisputeInitiated: "(H256,PolkadotRuntimeParachainsDisputesDisputeLocation)",
            DisputeConcluded: "(H256,PolkadotRuntimeParachainsDisputesDisputeResult)",
            Revert: "u32",
        },
    },
    /**
     * Lookup532: polkadot_runtime_parachains::disputes::DisputeLocation
     **/
    PolkadotRuntimeParachainsDisputesDisputeLocation: {
        _enum: ["Local", "Remote"],
    },
    /**
     * Lookup533: polkadot_runtime_parachains::disputes::DisputeResult
     **/
    PolkadotRuntimeParachainsDisputesDisputeResult: {
        _enum: ["Valid", "Invalid"],
    },
    /**
     * Lookup534: pallet_message_queue::pallet::Event<T>
     **/
    PalletMessageQueueEvent: {
        _enum: {
            ProcessingFailed: {
                id: "H256",
                origin: "DancelightRuntimeAggregateMessageOrigin",
                error: "FrameSupportMessagesProcessMessageError",
            },
            Processed: {
                id: "H256",
                origin: "DancelightRuntimeAggregateMessageOrigin",
                weightUsed: "SpWeightsWeightV2Weight",
                success: "bool",
            },
            OverweightEnqueued: {
                id: "[u8;32]",
                origin: "DancelightRuntimeAggregateMessageOrigin",
                pageIndex: "u32",
                messageIndex: "u32",
            },
            PageReaped: {
                origin: "DancelightRuntimeAggregateMessageOrigin",
                index: "u32",
            },
        },
    },
    /**
     * Lookup535: frame_support::traits::messages::ProcessMessageError
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
     * Lookup536: polkadot_runtime_parachains::on_demand::pallet::Event<T>
     **/
    PolkadotRuntimeParachainsOnDemandPalletEvent: {
        _enum: {
            OnDemandOrderPlaced: {
                paraId: "u32",
                spotPrice: "u128",
                orderedBy: "AccountId32",
            },
            SpotPriceSet: {
                spotPrice: "u128",
            },
        },
    },
    /**
     * Lookup537: polkadot_runtime_common::paras_registrar::pallet::Event<T>
     **/
    PolkadotRuntimeCommonParasRegistrarPalletEvent: {
        _enum: {
            Registered: {
                paraId: "u32",
                manager: "AccountId32",
            },
            Deregistered: {
                paraId: "u32",
            },
            Reserved: {
                paraId: "u32",
                who: "AccountId32",
            },
            Swapped: {
                paraId: "u32",
                otherId: "u32",
            },
        },
    },
    /**
     * Lookup538: pallet_utility::pallet::Event
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
     * Lookup540: pallet_identity::pallet::Event<T>
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
     * Lookup541: pallet_scheduler::pallet::Event<T>
     **/
    PalletSchedulerEvent: {
        _enum: {
            Scheduled: {
                when: "u32",
                index: "u32",
            },
            Canceled: {
                when: "u32",
                index: "u32",
            },
            Dispatched: {
                task: "(u32,u32)",
                id: "Option<[u8;32]>",
                result: "Result<Null, SpRuntimeDispatchError>",
            },
            RetrySet: {
                task: "(u32,u32)",
                id: "Option<[u8;32]>",
                period: "u32",
                retries: "u8",
            },
            RetryCancelled: {
                task: "(u32,u32)",
                id: "Option<[u8;32]>",
            },
            CallUnavailable: {
                task: "(u32,u32)",
                id: "Option<[u8;32]>",
            },
            PeriodicFailed: {
                task: "(u32,u32)",
                id: "Option<[u8;32]>",
            },
            RetryFailed: {
                task: "(u32,u32)",
                id: "Option<[u8;32]>",
            },
            PermanentlyOverweight: {
                task: "(u32,u32)",
                id: "Option<[u8;32]>",
            },
        },
    },
    /**
     * Lookup543: pallet_proxy::pallet::Event<T>
     **/
    PalletProxyEvent: {
        _enum: {
            ProxyExecuted: {
                result: "Result<Null, SpRuntimeDispatchError>",
            },
            PureCreated: {
                pure: "AccountId32",
                who: "AccountId32",
                proxyType: "DancelightRuntimeProxyType",
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
                proxyType: "DancelightRuntimeProxyType",
                delay: "u32",
            },
            ProxyRemoved: {
                delegator: "AccountId32",
                delegatee: "AccountId32",
                proxyType: "DancelightRuntimeProxyType",
                delay: "u32",
            },
        },
    },
    /**
     * Lookup544: pallet_multisig::pallet::Event<T>
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
     * Lookup545: pallet_preimage::pallet::Event<T>
     **/
    PalletPreimageEvent: {
        _enum: {
            Noted: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
            Requested: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
            Cleared: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
        },
    },
    /**
     * Lookup546: pallet_asset_rate::pallet::Event<T>
     **/
    PalletAssetRateEvent: {
        _enum: {
            AssetRateCreated: {
                assetKind: "Null",
                rate: "u128",
            },
            AssetRateRemoved: {
                assetKind: "Null",
            },
            AssetRateUpdated: {
                _alias: {
                    new_: "new",
                },
                assetKind: "Null",
                old: "u128",
                new_: "u128",
            },
        },
    },
    /**
     * Lookup547: pallet_xcm::pallet::Event<T>
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
     * Lookup548: staging_xcm::v5::traits::Outcome
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
     * Lookup549: pallet_stream_payment::pallet::Event<T>
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
     * Lookup550: pallet_stream_payment::pallet::Party
     **/
    PalletStreamPaymentParty: {
        _enum: ["Source", "Target"],
    },
    /**
     * Lookup551: pallet_migrations::pallet::Event<T>
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
     * Lookup553: snowbridge_pallet_ethereum_client::pallet::Event<T>
     **/
    SnowbridgePalletEthereumClientEvent: {
        _enum: {
            BeaconHeaderImported: {
                blockHash: "H256",
                slot: "u64",
            },
            SyncCommitteeUpdated: {
                period: "u64",
            },
            OperatingModeChanged: {
                mode: "SnowbridgeCoreOperatingModeBasicOperatingMode",
            },
        },
    },
    /**
     * Lookup554: pallet_root_testing::pallet::Event<T>
     **/
    PalletRootTestingEvent: {
        _enum: ["DefensiveTestCall"],
    },
    /**
     * Lookup555: pallet_sudo::pallet::Event<T>
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
     * Lookup556: frame_system::Phase
     **/
    FrameSystemPhase: {
        _enum: {
            ApplyExtrinsic: "u32",
            Finalization: "Null",
            Initialization: "Null",
        },
    },
    /**
     * Lookup558: frame_system::LastRuntimeUpgradeInfo
     **/
    FrameSystemLastRuntimeUpgradeInfo: {
        specVersion: "Compact<u32>",
        specName: "Text",
    },
    /**
     * Lookup561: frame_system::CodeUpgradeAuthorization<T>
     **/
    FrameSystemCodeUpgradeAuthorization: {
        codeHash: "H256",
        checkVersion: "bool",
    },
    /**
     * Lookup562: frame_system::limits::BlockWeights
     **/
    FrameSystemLimitsBlockWeights: {
        baseBlock: "SpWeightsWeightV2Weight",
        maxBlock: "SpWeightsWeightV2Weight",
        perClass: "FrameSupportDispatchPerDispatchClassWeightsPerClass",
    },
    /**
     * Lookup563: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
     **/
    FrameSupportDispatchPerDispatchClassWeightsPerClass: {
        normal: "FrameSystemLimitsWeightsPerClass",
        operational: "FrameSystemLimitsWeightsPerClass",
        mandatory: "FrameSystemLimitsWeightsPerClass",
    },
    /**
     * Lookup564: frame_system::limits::WeightsPerClass
     **/
    FrameSystemLimitsWeightsPerClass: {
        baseExtrinsic: "SpWeightsWeightV2Weight",
        maxExtrinsic: "Option<SpWeightsWeightV2Weight>",
        maxTotal: "Option<SpWeightsWeightV2Weight>",
        reserved: "Option<SpWeightsWeightV2Weight>",
    },
    /**
     * Lookup565: frame_system::limits::BlockLength
     **/
    FrameSystemLimitsBlockLength: {
        max: "FrameSupportDispatchPerDispatchClassU32",
    },
    /**
     * Lookup566: frame_support::dispatch::PerDispatchClass<T>
     **/
    FrameSupportDispatchPerDispatchClassU32: {
        normal: "u32",
        operational: "u32",
        mandatory: "u32",
    },
    /**
     * Lookup567: sp_weights::RuntimeDbWeight
     **/
    SpWeightsRuntimeDbWeight: {
        read: "u64",
        write: "u64",
    },
    /**
     * Lookup568: sp_version::RuntimeVersion
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
     * Lookup572: frame_system::pallet::Error<T>
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
     * Lookup579: sp_consensus_babe::digests::PreDigest
     **/
    SpConsensusBabeDigestsPreDigest: {
        _enum: {
            __Unused0: "Null",
            Primary: "SpConsensusBabeDigestsPrimaryPreDigest",
            SecondaryPlain: "SpConsensusBabeDigestsSecondaryPlainPreDigest",
            SecondaryVRF: "SpConsensusBabeDigestsSecondaryVRFPreDigest",
        },
    },
    /**
     * Lookup580: sp_consensus_babe::digests::PrimaryPreDigest
     **/
    SpConsensusBabeDigestsPrimaryPreDigest: {
        authorityIndex: "u32",
        slot: "u64",
        vrfSignature: "SpCoreSr25519VrfVrfSignature",
    },
    /**
     * Lookup581: sp_core::sr25519::vrf::VrfSignature
     **/
    SpCoreSr25519VrfVrfSignature: {
        preOutput: "[u8;32]",
        proof: "[u8;64]",
    },
    /**
     * Lookup582: sp_consensus_babe::digests::SecondaryPlainPreDigest
     **/
    SpConsensusBabeDigestsSecondaryPlainPreDigest: {
        authorityIndex: "u32",
        slot: "u64",
    },
    /**
     * Lookup583: sp_consensus_babe::digests::SecondaryVRFPreDigest
     **/
    SpConsensusBabeDigestsSecondaryVRFPreDigest: {
        authorityIndex: "u32",
        slot: "u64",
        vrfSignature: "SpCoreSr25519VrfVrfSignature",
    },
    /**
     * Lookup584: sp_consensus_babe::BabeEpochConfiguration
     **/
    SpConsensusBabeBabeEpochConfiguration: {
        c: "(u64,u64)",
        allowedSlots: "SpConsensusBabeAllowedSlots",
    },
    /**
     * Lookup588: pallet_babe::pallet::Error<T>
     **/
    PalletBabeError: {
        _enum: [
            "InvalidEquivocationProof",
            "InvalidKeyOwnershipProof",
            "DuplicateOffenceReport",
            "InvalidConfiguration",
        ],
    },
    /**
     * Lookup590: pallet_balances::types::BalanceLock<Balance>
     **/
    PalletBalancesBalanceLock: {
        id: "[u8;8]",
        amount: "u128",
        reasons: "PalletBalancesReasons",
    },
    /**
     * Lookup591: pallet_balances::types::Reasons
     **/
    PalletBalancesReasons: {
        _enum: ["Fee", "Misc", "All"],
    },
    /**
     * Lookup594: pallet_balances::types::ReserveData<ReserveIdentifier, Balance>
     **/
    PalletBalancesReserveData: {
        id: "[u8;8]",
        amount: "u128",
    },
    /**
     * Lookup598: dancelight_runtime::RuntimeHoldReason
     **/
    DancelightRuntimeRuntimeHoldReason: {
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
            ContainerRegistrar: "PalletRegistrarHoldReason",
            __Unused11: "Null",
            __Unused12: "Null",
            __Unused13: "Null",
            __Unused14: "Null",
            __Unused15: "Null",
            __Unused16: "Null",
            __Unused17: "Null",
            __Unused18: "Null",
            DataPreservers: "PalletDataPreserversHoldReason",
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
            PooledStaking: "PalletPooledStakingHoldReason",
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
            __Unused51: "Null",
            __Unused52: "Null",
            __Unused53: "Null",
            __Unused54: "Null",
            __Unused55: "Null",
            __Unused56: "Null",
            __Unused57: "Null",
            __Unused58: "Null",
            __Unused59: "Null",
            __Unused60: "Null",
            __Unused61: "Null",
            __Unused62: "Null",
            __Unused63: "Null",
            __Unused64: "Null",
            __Unused65: "Null",
            __Unused66: "Null",
            __Unused67: "Null",
            __Unused68: "Null",
            __Unused69: "Null",
            __Unused70: "Null",
            __Unused71: "Null",
            __Unused72: "Null",
            __Unused73: "Null",
            __Unused74: "Null",
            __Unused75: "Null",
            __Unused76: "Null",
            __Unused77: "Null",
            __Unused78: "Null",
            __Unused79: "Null",
            __Unused80: "Null",
            __Unused81: "Null",
            __Unused82: "Null",
            __Unused83: "Null",
            __Unused84: "Null",
            Preimage: "PalletPreimageHoldReason",
            __Unused86: "Null",
            __Unused87: "Null",
            __Unused88: "Null",
            __Unused89: "Null",
            __Unused90: "Null",
            __Unused91: "Null",
            __Unused92: "Null",
            __Unused93: "Null",
            __Unused94: "Null",
            __Unused95: "Null",
            __Unused96: "Null",
            __Unused97: "Null",
            __Unused98: "Null",
            __Unused99: "Null",
            StreamPayment: "PalletStreamPaymentHoldReason",
        },
    },
    /**
     * Lookup599: pallet_registrar::pallet::HoldReason
     **/
    PalletRegistrarHoldReason: {
        _enum: ["RegistrarDeposit"],
    },
    /**
     * Lookup600: pallet_data_preservers::pallet::HoldReason
     **/
    PalletDataPreserversHoldReason: {
        _enum: ["ProfileDeposit"],
    },
    /**
     * Lookup601: pallet_pooled_staking::pallet::HoldReason
     **/
    PalletPooledStakingHoldReason: {
        _enum: ["PooledStake"],
    },
    /**
     * Lookup602: pallet_preimage::pallet::HoldReason
     **/
    PalletPreimageHoldReason: {
        _enum: ["Preimage"],
    },
    /**
     * Lookup603: pallet_stream_payment::pallet::HoldReason
     **/
    PalletStreamPaymentHoldReason: {
        _enum: ["StreamPayment", "StreamOpened"],
    },
    /**
     * Lookup606: frame_support::traits::tokens::misc::IdAmount<Id, Balance>
     **/
    FrameSupportTokensMiscIdAmount: {
        id: "Null",
        amount: "u128",
    },
    /**
     * Lookup608: pallet_balances::pallet::Error<T, I>
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
     * Lookup609: pallet_transaction_payment::Releases
     **/
    PalletTransactionPaymentReleases: {
        _enum: ["V1Ancient", "V2"],
    },
    /**
     * Lookup610: sp_staking::offence::OffenceDetails<sp_core::crypto::AccountId32, Offender>
     **/
    SpStakingOffenceOffenceDetails: {
        offender: "(AccountId32,Null)",
        reporters: "Vec<AccountId32>",
    },
    /**
     * Lookup622: pallet_registrar::pallet::DepositInfo<T>
     **/
    PalletRegistrarDepositInfo: {
        creator: "AccountId32",
        deposit: "u128",
    },
    /**
     * Lookup623: pallet_registrar::pallet::Error<T>
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
     * Lookup624: pallet_configuration::HostConfiguration
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
     * Lookup627: pallet_configuration::pallet::Error<T>
     **/
    PalletConfigurationError: {
        _enum: ["InvalidNewValue"],
    },
    /**
     * Lookup629: pallet_invulnerables::pallet::Error<T>
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
     * Lookup630: dp_collator_assignment::AssignedCollators<sp_core::crypto::AccountId32>
     **/
    DpCollatorAssignmentAssignedCollatorsAccountId32: {
        orchestratorChain: "Vec<AccountId32>",
        containerChains: "BTreeMap<u32, Vec<AccountId32>>",
    },
    /**
     * Lookup635: dp_collator_assignment::AssignedCollators<nimbus_primitives::nimbus_crypto::Public>
     **/
    DpCollatorAssignmentAssignedCollatorsPublic: {
        orchestratorChain: "Vec<NimbusPrimitivesNimbusCryptoPublic>",
        containerChains: "BTreeMap<u32, Vec<NimbusPrimitivesNimbusCryptoPublic>>",
    },
    /**
     * Lookup643: tp_traits::ContainerChainBlockInfo<sp_core::crypto::AccountId32>
     **/
    TpTraitsContainerChainBlockInfo: {
        blockNumber: "u32",
        author: "AccountId32",
        latestSlotNumber: "u64",
    },
    /**
     * Lookup644: pallet_author_noting::pallet::Error<T>
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
     * Lookup645: pallet_services_payment::pallet::Error<T>
     **/
    PalletServicesPaymentError: {
        _enum: ["InsufficientFundsToPurchaseCredits", "InsufficientCredits", "CreditPriceTooExpensive"],
    },
    /**
     * Lookup646: pallet_data_preservers::types::RegisteredProfile<T>
     **/
    PalletDataPreserversRegisteredProfile: {
        account: "AccountId32",
        deposit: "u128",
        profile: "PalletDataPreserversProfile",
        assignment: "Option<(u32,TpDataPreserversCommonAssignmentWitness)>",
    },
    /**
     * Lookup652: pallet_data_preservers::pallet::Error<T>
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
     * Lookup655: tp_traits::ActiveEraInfo
     **/
    TpTraitsActiveEraInfo: {
        index: "u32",
        start: "Option<u64>",
    },
    /**
     * Lookup657: pallet_external_validators::pallet::Error<T>
     **/
    PalletExternalValidatorsError: {
        _enum: [
            "TooManyWhitelisted",
            "AlreadyWhitelisted",
            "NotWhitelisted",
            "NoKeysRegistered",
            "UnableToDeriveValidatorId",
        ],
    },
    /**
     * Lookup662: pallet_external_validator_slashes::Slash<sp_core::crypto::AccountId32, SlashId>
     **/
    PalletExternalValidatorSlashesSlash: {
        externalIdx: "u64",
        validator: "AccountId32",
        reporters: "Vec<AccountId32>",
        slashId: "u32",
        percentage: "Perbill",
        confirmed: "bool",
    },
    /**
     * Lookup663: pallet_external_validator_slashes::pallet::Error<T>
     **/
    PalletExternalValidatorSlashesError: {
        _enum: [
            "EmptyTargets",
            "InvalidSlashIndex",
            "NotSortedAndUnique",
            "ProvidedFutureEra",
            "ProvidedNonSlashableEra",
            "DeferPeriodIsOver",
            "ErrorComputingSlash",
            "EthereumValidateFail",
            "EthereumDeliverFail",
            "RootTestInvalidParams",
        ],
    },
    /**
     * Lookup664: pallet_external_validators_rewards::pallet::EraRewardPoints<sp_core::crypto::AccountId32>
     **/
    PalletExternalValidatorsRewardsEraRewardPoints: {
        total: "u32",
        individual: "BTreeMap<AccountId32, u32>",
    },
    /**
     * Lookup669: snowbridge_pallet_outbound_queue::types::CommittedMessage
     **/
    SnowbridgePalletOutboundQueueCommittedMessage: {
        channelId: "SnowbridgeCoreChannelId",
        nonce: "Compact<u64>",
        command: "u8",
        params: "Bytes",
        maxDispatchGas: "Compact<u64>",
        maxFeePerGas: "Compact<u128>",
        reward: "Compact<u128>",
        id: "H256",
    },
    /**
     * Lookup670: snowbridge_pallet_outbound_queue::pallet::Error<T>
     **/
    SnowbridgePalletOutboundQueueError: {
        _enum: ["MessageTooLarge", "Halted", "InvalidChannel"],
    },
    /**
     * Lookup671: snowbridge_pallet_inbound_queue::pallet::Error<T>
     **/
    SnowbridgePalletInboundQueueError: {
        _enum: {
            InvalidGateway: "Null",
            InvalidEnvelope: "Null",
            InvalidNonce: "Null",
            InvalidPayload: "Null",
            InvalidChannel: "Null",
            MaxNonceReached: "Null",
            InvalidAccountConversion: "Null",
            Halted: "Null",
            Verification: "SnowbridgeCoreInboundVerificationError",
            Send: "SnowbridgePalletInboundQueueSendError",
            ConvertMessage: "SnowbridgeRouterPrimitivesInboundConvertMessageError",
        },
    },
    /**
     * Lookup672: snowbridge_core::inbound::VerificationError
     **/
    SnowbridgeCoreInboundVerificationError: {
        _enum: ["HeaderNotFound", "LogNotFound", "InvalidLog", "InvalidProof", "InvalidExecutionProof"],
    },
    /**
     * Lookup673: snowbridge_pallet_inbound_queue::pallet::SendError
     **/
    SnowbridgePalletInboundQueueSendError: {
        _enum: [
            "NotApplicable",
            "NotRoutable",
            "Transport",
            "DestinationUnsupported",
            "ExceedsMaxMessageSize",
            "MissingArgument",
            "Fees",
        ],
    },
    /**
     * Lookup674: snowbridge_router_primitives::inbound::ConvertMessageError
     **/
    SnowbridgeRouterPrimitivesInboundConvertMessageError: {
        _enum: ["UnsupportedVersion", "InvalidDestination", "InvalidToken", "UnsupportedFeeAsset", "CannotReanchor"],
    },
    /**
     * Lookup675: snowbridge_core::Channel
     **/
    SnowbridgeCoreChannel: {
        agentId: "H256",
        paraId: "u32",
    },
    /**
     * Lookup676: snowbridge_pallet_system::pallet::Error<T>
     **/
    SnowbridgePalletSystemError: {
        _enum: {
            LocationConversionFailed: "Null",
            AgentAlreadyCreated: "Null",
            NoAgent: "Null",
            ChannelAlreadyCreated: "Null",
            NoChannel: "Null",
            UnsupportedLocationVersion: "Null",
            InvalidLocation: "Null",
            Send: "SnowbridgeCoreOutboundSendError",
            InvalidTokenTransferFees: "Null",
            InvalidPricingParameters: "Null",
            InvalidUpgradeParameters: "Null",
        },
    },
    /**
     * Lookup677: snowbridge_core::outbound::SendError
     **/
    SnowbridgeCoreOutboundSendError: {
        _enum: ["MessageTooLarge", "Halted", "InvalidChannel"],
    },
    /**
     * Lookup678: pallet_ethereum_token_transfers::pallet::Error<T>
     **/
    PalletEthereumTokenTransfersError: {
        _enum: {
            ChannelInfoNotSet: "Null",
            UnknownLocationForToken: "Null",
            InvalidMessage: "SnowbridgeCoreOutboundSendError",
            TransferMessageNotSent: "SnowbridgeCoreOutboundSendError",
        },
    },
    /**
     * Lookup682: sp_core::crypto::KeyTypeId
     **/
    SpCoreCryptoKeyTypeId: "[u8;4]",
    /**
     * Lookup683: pallet_session::pallet::Error<T>
     **/
    PalletSessionError: {
        _enum: ["InvalidProof", "NoAssociatedValidatorId", "DuplicatedKey", "NoKeys", "NoAccount"],
    },
    /**
     * Lookup684: pallet_grandpa::StoredState<N>
     **/
    PalletGrandpaStoredState: {
        _enum: {
            Live: "Null",
            PendingPause: {
                scheduledAt: "u32",
                delay: "u32",
            },
            Paused: "Null",
            PendingResume: {
                scheduledAt: "u32",
                delay: "u32",
            },
        },
    },
    /**
     * Lookup685: pallet_grandpa::StoredPendingChange<N, Limit>
     **/
    PalletGrandpaStoredPendingChange: {
        scheduledAt: "u32",
        delay: "u32",
        nextAuthorities: "Vec<(SpConsensusGrandpaAppPublic,u64)>",
        forced: "Option<u32>",
    },
    /**
     * Lookup687: pallet_grandpa::pallet::Error<T>
     **/
    PalletGrandpaError: {
        _enum: [
            "PauseFailed",
            "ResumeFailed",
            "ChangePending",
            "TooSoon",
            "InvalidKeyOwnershipProof",
            "InvalidEquivocationProof",
            "DuplicateOffenceReport",
        ],
    },
    /**
     * Lookup690: pallet_inflation_rewards::pallet::ChainsToRewardValue<T>
     **/
    PalletInflationRewardsChainsToRewardValue: {
        paraIds: "Vec<u32>",
        rewardsPerChain: "u128",
    },
    /**
     * Lookup692: pallet_pooled_staking::candidate::EligibleCandidate<sp_core::crypto::AccountId32, S>
     **/
    PalletPooledStakingCandidateEligibleCandidate: {
        candidate: "AccountId32",
        stake: "u128",
    },
    /**
     * Lookup695: pallet_pooled_staking::pallet::PoolsKey<sp_core::crypto::AccountId32>
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
     * Lookup698: pallet_pooled_staking::pools::CandidateSummary
     **/
    PalletPooledStakingPoolsCandidateSummary: {
        delegators: "u32",
        joiningDelegators: "u32",
        autoCompoundingDelegators: "u32",
        manualRewardsDelegators: "u32",
        leavingDelegators: "u32",
    },
    /**
     * Lookup699: pallet_pooled_staking::pallet::Error<T>
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
     * Lookup702: pallet_inactivity_tracking::pallet::Error<T>
     **/
    PalletInactivityTrackingError: {
        _enum: [
            "MaxCollatorsPerSessionReached",
            "MaxContainerChainsReached",
            "ActivityTrackingStatusUpdateSuspended",
            "ActivityTrackingStatusAlreadyEnabled",
            "ActivityTrackingStatusAlreadyDisabled",
        ],
    },
    /**
     * Lookup703: pallet_treasury::Proposal<sp_core::crypto::AccountId32, Balance>
     **/
    PalletTreasuryProposal: {
        proposer: "AccountId32",
        value: "u128",
        beneficiary: "AccountId32",
        bond: "u128",
    },
    /**
     * Lookup705: pallet_treasury::SpendStatus<AssetKind, AssetBalance, sp_core::crypto::AccountId32, BlockNumber, PaymentId>
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
     * Lookup706: pallet_treasury::PaymentState<Id>
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
     * Lookup708: frame_support::PalletId
     **/
    FrameSupportPalletId: "[u8;8]",
    /**
     * Lookup709: pallet_treasury::pallet::Error<T, I>
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
     * Lookup711: pallet_conviction_voting::vote::Voting<Balance, sp_core::crypto::AccountId32, BlockNumber, PollIndex, MaxVotes>
     **/
    PalletConvictionVotingVoteVoting: {
        _enum: {
            Casting: "PalletConvictionVotingVoteCasting",
            Delegating: "PalletConvictionVotingVoteDelegating",
        },
    },
    /**
     * Lookup712: pallet_conviction_voting::vote::Casting<Balance, BlockNumber, PollIndex, MaxVotes>
     **/
    PalletConvictionVotingVoteCasting: {
        votes: "Vec<(u32,PalletConvictionVotingVoteAccountVote)>",
        delegations: "PalletConvictionVotingDelegations",
        prior: "PalletConvictionVotingVotePriorLock",
    },
    /**
     * Lookup716: pallet_conviction_voting::types::Delegations<Balance>
     **/
    PalletConvictionVotingDelegations: {
        votes: "u128",
        capital: "u128",
    },
    /**
     * Lookup717: pallet_conviction_voting::vote::PriorLock<BlockNumber, Balance>
     **/
    PalletConvictionVotingVotePriorLock: "(u32,u128)",
    /**
     * Lookup718: pallet_conviction_voting::vote::Delegating<Balance, sp_core::crypto::AccountId32, BlockNumber>
     **/
    PalletConvictionVotingVoteDelegating: {
        balance: "u128",
        target: "AccountId32",
        conviction: "PalletConvictionVotingConviction",
        delegations: "PalletConvictionVotingDelegations",
        prior: "PalletConvictionVotingVotePriorLock",
    },
    /**
     * Lookup722: pallet_conviction_voting::pallet::Error<T, I>
     **/
    PalletConvictionVotingError: {
        _enum: [
            "NotOngoing",
            "NotVoter",
            "NoPermission",
            "NoPermissionYet",
            "AlreadyDelegating",
            "AlreadyVoting",
            "InsufficientFunds",
            "NotDelegating",
            "Nonsense",
            "MaxVotesReached",
            "ClassNeeded",
            "BadClass",
        ],
    },
    /**
     * Lookup723: pallet_referenda::types::ReferendumInfo<TrackId, dancelight_runtime::OriginCaller, Moment, frame_support::traits::preimages::Bounded<dancelight_runtime::RuntimeCall, sp_runtime::traits::BlakeTwo256>, Balance, pallet_conviction_voting::types::Tally<Votes, Total>, sp_core::crypto::AccountId32, ScheduleAddress>
     **/
    PalletReferendaReferendumInfoConvictionVotingTally: {
        _enum: {
            Ongoing: "PalletReferendaReferendumStatusConvictionVotingTally",
            Approved: "(u32,Option<PalletReferendaDeposit>,Option<PalletReferendaDeposit>)",
            Rejected: "(u32,Option<PalletReferendaDeposit>,Option<PalletReferendaDeposit>)",
            Cancelled: "(u32,Option<PalletReferendaDeposit>,Option<PalletReferendaDeposit>)",
            TimedOut: "(u32,Option<PalletReferendaDeposit>,Option<PalletReferendaDeposit>)",
            Killed: "u32",
        },
    },
    /**
     * Lookup724: pallet_referenda::types::ReferendumStatus<TrackId, dancelight_runtime::OriginCaller, Moment, frame_support::traits::preimages::Bounded<dancelight_runtime::RuntimeCall, sp_runtime::traits::BlakeTwo256>, Balance, pallet_conviction_voting::types::Tally<Votes, Total>, sp_core::crypto::AccountId32, ScheduleAddress>
     **/
    PalletReferendaReferendumStatusConvictionVotingTally: {
        track: "u16",
        origin: "DancelightRuntimeOriginCaller",
        proposal: "FrameSupportPreimagesBounded",
        enactment: "FrameSupportScheduleDispatchTime",
        submitted: "u32",
        submissionDeposit: "PalletReferendaDeposit",
        decisionDeposit: "Option<PalletReferendaDeposit>",
        deciding: "Option<PalletReferendaDecidingStatus>",
        tally: "PalletConvictionVotingTally",
        inQueue: "bool",
        alarm: "Option<(u32,(u32,u32))>",
    },
    /**
     * Lookup725: pallet_referenda::types::Deposit<sp_core::crypto::AccountId32, Balance>
     **/
    PalletReferendaDeposit: {
        who: "AccountId32",
        amount: "u128",
    },
    /**
     * Lookup728: pallet_referenda::types::DecidingStatus<BlockNumber>
     **/
    PalletReferendaDecidingStatus: {
        since: "u32",
        confirming: "Option<u32>",
    },
    /**
     * Lookup736: pallet_referenda::types::TrackInfo<Balance, Moment>
     **/
    PalletReferendaTrackInfo: {
        name: "Text",
        maxDeciding: "u32",
        decisionDeposit: "u128",
        preparePeriod: "u32",
        decisionPeriod: "u32",
        confirmPeriod: "u32",
        minEnactmentPeriod: "u32",
        minApproval: "PalletReferendaCurve",
        minSupport: "PalletReferendaCurve",
    },
    /**
     * Lookup737: pallet_referenda::types::Curve
     **/
    PalletReferendaCurve: {
        _enum: {
            LinearDecreasing: {
                length: "Perbill",
                floor: "Perbill",
                ceil: "Perbill",
            },
            SteppedDecreasing: {
                begin: "Perbill",
                end: "Perbill",
                step: "Perbill",
                period: "Perbill",
            },
            Reciprocal: {
                factor: "i64",
                xOffset: "i64",
                yOffset: "i64",
            },
        },
    },
    /**
     * Lookup740: pallet_referenda::pallet::Error<T, I>
     **/
    PalletReferendaError: {
        _enum: [
            "NotOngoing",
            "HasDeposit",
            "BadTrack",
            "Full",
            "QueueEmpty",
            "BadReferendum",
            "NothingToDo",
            "NoTrack",
            "Unfinished",
            "NoPermission",
            "NoDeposit",
            "BadStatus",
            "PreimageNotExist",
            "PreimageStoredWithDifferentLength",
        ],
    },
    /**
     * Lookup741: pallet_ranked_collective::MemberRecord
     **/
    PalletRankedCollectiveMemberRecord: {
        rank: "u16",
    },
    /**
     * Lookup745: pallet_ranked_collective::pallet::Error<T, I>
     **/
    PalletRankedCollectiveError: {
        _enum: [
            "AlreadyMember",
            "NotMember",
            "NotPolling",
            "Ongoing",
            "NoneRemaining",
            "Corruption",
            "RankTooLow",
            "InvalidWitness",
            "NoPermission",
            "SameMember",
            "TooManyMembers",
        ],
    },
    /**
     * Lookup746: pallet_referenda::types::ReferendumInfo<TrackId, dancelight_runtime::OriginCaller, Moment, frame_support::traits::preimages::Bounded<dancelight_runtime::RuntimeCall, sp_runtime::traits::BlakeTwo256>, Balance, pallet_ranked_collective::Tally<T, I, M>, sp_core::crypto::AccountId32, ScheduleAddress>
     **/
    PalletReferendaReferendumInfoRankedCollectiveTally: {
        _enum: {
            Ongoing: "PalletReferendaReferendumStatusRankedCollectiveTally",
            Approved: "(u32,Option<PalletReferendaDeposit>,Option<PalletReferendaDeposit>)",
            Rejected: "(u32,Option<PalletReferendaDeposit>,Option<PalletReferendaDeposit>)",
            Cancelled: "(u32,Option<PalletReferendaDeposit>,Option<PalletReferendaDeposit>)",
            TimedOut: "(u32,Option<PalletReferendaDeposit>,Option<PalletReferendaDeposit>)",
            Killed: "u32",
        },
    },
    /**
     * Lookup747: pallet_referenda::types::ReferendumStatus<TrackId, dancelight_runtime::OriginCaller, Moment, frame_support::traits::preimages::Bounded<dancelight_runtime::RuntimeCall, sp_runtime::traits::BlakeTwo256>, Balance, pallet_ranked_collective::Tally<T, I, M>, sp_core::crypto::AccountId32, ScheduleAddress>
     **/
    PalletReferendaReferendumStatusRankedCollectiveTally: {
        track: "u16",
        origin: "DancelightRuntimeOriginCaller",
        proposal: "FrameSupportPreimagesBounded",
        enactment: "FrameSupportScheduleDispatchTime",
        submitted: "u32",
        submissionDeposit: "PalletReferendaDeposit",
        decisionDeposit: "Option<PalletReferendaDeposit>",
        deciding: "Option<PalletReferendaDecidingStatus>",
        tally: "PalletRankedCollectiveTally",
        inQueue: "bool",
        alarm: "Option<(u32,(u32,u32))>",
    },
    /**
     * Lookup750: pallet_whitelist::pallet::Error<T>
     **/
    PalletWhitelistError: {
        _enum: [
            "UnavailablePreImage",
            "UndecodableCall",
            "InvalidCallWeightWitness",
            "CallIsNotWhitelisted",
            "CallAlreadyWhitelisted",
        ],
    },
    /**
     * Lookup751: polkadot_runtime_parachains::configuration::HostConfiguration<BlockNumber>
     **/
    PolkadotRuntimeParachainsConfigurationHostConfiguration: {
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
        maxPovSize: "u32",
        maxDownwardMessageSize: "u32",
        hrmpMaxParachainOutboundChannels: "u32",
        hrmpSenderDeposit: "u128",
        hrmpRecipientDeposit: "u128",
        hrmpChannelMaxCapacity: "u32",
        hrmpChannelMaxTotalSize: "u32",
        hrmpMaxParachainInboundChannels: "u32",
        hrmpChannelMaxMessageSize: "u32",
        executorParams: "PolkadotPrimitivesV8ExecutorParams",
        codeRetentionPeriod: "u32",
        maxValidators: "Option<u32>",
        disputePeriod: "u32",
        disputePostConclusionAcceptancePeriod: "u32",
        noShowSlots: "u32",
        nDelayTranches: "u32",
        zerothDelayTrancheWidth: "u32",
        neededApprovals: "u32",
        relayVrfModuloSamples: "u32",
        pvfVotingTtl: "u32",
        minimumValidationUpgradeDelay: "u32",
        minimumBackingVotes: "u32",
        nodeFeatures: "BitVec",
        approvalVotingParams: "PolkadotPrimitivesV8ApprovalVotingParams",
        schedulerParams: "PolkadotPrimitivesV8SchedulerParams",
    },
    /**
     * Lookup754: polkadot_runtime_parachains::configuration::pallet::Error<T>
     **/
    PolkadotRuntimeParachainsConfigurationPalletError: {
        _enum: ["InvalidNewValue"],
    },
    /**
     * Lookup757: polkadot_runtime_parachains::shared::AllowedRelayParentsTracker<primitive_types::H256, BlockNumber>
     **/
    PolkadotRuntimeParachainsSharedAllowedRelayParentsTracker: {
        buffer: "Vec<PolkadotRuntimeParachainsSharedRelayParentInfo>",
        latestNumber: "u32",
    },
    /**
     * Lookup759: polkadot_runtime_parachains::shared::RelayParentInfo<primitive_types::H256>
     **/
    PolkadotRuntimeParachainsSharedRelayParentInfo: {
        relayParent: "H256",
        stateRoot: "H256",
        claimQueue: "BTreeMap<u32, BTreeMap<u8, BTreeSet<u32>>>",
    },
    /**
     * Lookup769: polkadot_runtime_parachains::inclusion::CandidatePendingAvailability<primitive_types::H256, N>
     **/
    PolkadotRuntimeParachainsInclusionCandidatePendingAvailability: {
        _alias: {
            hash_: "hash",
        },
        core: "u32",
        hash_: "H256",
        descriptor: "PolkadotPrimitivesVstagingCandidateDescriptorV2",
        commitments: "PolkadotPrimitivesV8CandidateCommitments",
        availabilityVotes: "BitVec",
        backers: "BitVec",
        relayParentNumber: "u32",
        backedInNumber: "u32",
        backingGroup: "u32",
    },
    /**
     * Lookup770: polkadot_runtime_parachains::inclusion::pallet::Error<T>
     **/
    PolkadotRuntimeParachainsInclusionPalletError: {
        _enum: [
            "ValidatorIndexOutOfBounds",
            "UnscheduledCandidate",
            "HeadDataTooLarge",
            "PrematureCodeUpgrade",
            "NewCodeTooLarge",
            "DisallowedRelayParent",
            "InvalidAssignment",
            "InvalidGroupIndex",
            "InsufficientBacking",
            "InvalidBacking",
            "ValidationDataHashMismatch",
            "IncorrectDownwardMessageHandling",
            "InvalidUpwardMessages",
            "HrmpWatermarkMishandling",
            "InvalidOutboundHrmp",
            "InvalidValidationCodeHash",
            "ParaHeadMismatch",
        ],
    },
    /**
     * Lookup771: polkadot_primitives::vstaging::ScrapedOnChainVotes<primitive_types::H256>
     **/
    PolkadotPrimitivesVstagingScrapedOnChainVotes: {
        session: "u32",
        backingValidatorsPerCandidate:
            "Vec<(PolkadotPrimitivesVstagingCandidateReceiptV2,Vec<(u32,PolkadotPrimitivesV8ValidityAttestation)>)>",
        disputes: "Vec<PolkadotPrimitivesV8DisputeStatementSet>",
    },
    /**
     * Lookup776: polkadot_runtime_parachains::paras_inherent::pallet::Error<T>
     **/
    PolkadotRuntimeParachainsParasInherentPalletError: {
        _enum: [
            "TooManyInclusionInherents",
            "InvalidParentHeader",
            "InherentDataFilteredDuringExecution",
            "UnscheduledCandidate",
        ],
    },
    /**
     * Lookup780: polkadot_runtime_parachains::scheduler::common::Assignment
     **/
    PolkadotRuntimeParachainsSchedulerCommonAssignment: {
        _enum: {
            Pool: {
                paraId: "u32",
                coreIndex: "u32",
            },
            Bulk: "u32",
        },
    },
    /**
     * Lookup783: polkadot_runtime_parachains::paras::PvfCheckActiveVoteState<BlockNumber>
     **/
    PolkadotRuntimeParachainsParasPvfCheckActiveVoteState: {
        votesAccept: "BitVec",
        votesReject: "BitVec",
        age: "u32",
        createdAt: "u32",
        causes: "Vec<PolkadotRuntimeParachainsParasPvfCheckCause>",
    },
    /**
     * Lookup785: polkadot_runtime_parachains::paras::PvfCheckCause<BlockNumber>
     **/
    PolkadotRuntimeParachainsParasPvfCheckCause: {
        _enum: {
            Onboarding: "u32",
            Upgrade: {
                id: "u32",
                includedAt: "u32",
                upgradeStrategy: "PolkadotRuntimeParachainsParasUpgradeStrategy",
            },
        },
    },
    /**
     * Lookup786: polkadot_runtime_parachains::paras::UpgradeStrategy
     **/
    PolkadotRuntimeParachainsParasUpgradeStrategy: {
        _enum: ["SetGoAheadSignal", "ApplyAtExpectedBlock"],
    },
    /**
     * Lookup788: polkadot_runtime_parachains::paras::ParaLifecycle
     **/
    PolkadotRuntimeParachainsParasParaLifecycle: {
        _enum: [
            "Onboarding",
            "Parathread",
            "Parachain",
            "UpgradingParathread",
            "DowngradingParachain",
            "OffboardingParathread",
            "OffboardingParachain",
        ],
    },
    /**
     * Lookup790: polkadot_runtime_parachains::paras::ParaPastCodeMeta<N>
     **/
    PolkadotRuntimeParachainsParasParaPastCodeMeta: {
        upgradeTimes: "Vec<PolkadotRuntimeParachainsParasReplacementTimes>",
        lastPruned: "Option<u32>",
    },
    /**
     * Lookup792: polkadot_runtime_parachains::paras::ReplacementTimes<N>
     **/
    PolkadotRuntimeParachainsParasReplacementTimes: {
        expectedAt: "u32",
        activatedAt: "u32",
    },
    /**
     * Lookup794: polkadot_primitives::v8::UpgradeGoAhead
     **/
    PolkadotPrimitivesV8UpgradeGoAhead: {
        _enum: ["Abort", "GoAhead"],
    },
    /**
     * Lookup795: polkadot_primitives::v8::UpgradeRestriction
     **/
    PolkadotPrimitivesV8UpgradeRestriction: {
        _enum: ["Present"],
    },
    /**
     * Lookup796: polkadot_runtime_parachains::paras::pallet::Error<T>
     **/
    PolkadotRuntimeParachainsParasPalletError: {
        _enum: [
            "NotRegistered",
            "CannotOnboard",
            "CannotOffboard",
            "CannotUpgrade",
            "CannotDowngrade",
            "PvfCheckStatementStale",
            "PvfCheckStatementFuture",
            "PvfCheckValidatorIndexOutOfBounds",
            "PvfCheckInvalidSignature",
            "PvfCheckDoubleVote",
            "PvfCheckSubjectInvalid",
            "CannotUpgradeCode",
            "InvalidCode",
        ],
    },
    /**
     * Lookup798: polkadot_runtime_parachains::initializer::BufferedSessionChange
     **/
    PolkadotRuntimeParachainsInitializerBufferedSessionChange: {
        validators: "Vec<PolkadotPrimitivesV8ValidatorAppPublic>",
        queued: "Vec<PolkadotPrimitivesV8ValidatorAppPublic>",
        sessionIndex: "u32",
    },
    /**
     * Lookup800: polkadot_core_primitives::InboundDownwardMessage<BlockNumber>
     **/
    PolkadotCorePrimitivesInboundDownwardMessage: {
        sentAt: "u32",
        msg: "Bytes",
    },
    /**
     * Lookup801: polkadot_runtime_parachains::hrmp::HrmpOpenChannelRequest
     **/
    PolkadotRuntimeParachainsHrmpHrmpOpenChannelRequest: {
        confirmed: "bool",
        age: "u32",
        senderDeposit: "u128",
        maxMessageSize: "u32",
        maxCapacity: "u32",
        maxTotalSize: "u32",
    },
    /**
     * Lookup803: polkadot_runtime_parachains::hrmp::HrmpChannel
     **/
    PolkadotRuntimeParachainsHrmpHrmpChannel: {
        maxCapacity: "u32",
        maxTotalSize: "u32",
        maxMessageSize: "u32",
        msgCount: "u32",
        totalSize: "u32",
        mqcHead: "Option<H256>",
        senderDeposit: "u128",
        recipientDeposit: "u128",
    },
    /**
     * Lookup805: polkadot_core_primitives::InboundHrmpMessage<BlockNumber>
     **/
    PolkadotCorePrimitivesInboundHrmpMessage: {
        sentAt: "u32",
        data: "Bytes",
    },
    /**
     * Lookup808: polkadot_runtime_parachains::hrmp::pallet::Error<T>
     **/
    PolkadotRuntimeParachainsHrmpPalletError: {
        _enum: [
            "OpenHrmpChannelToSelf",
            "OpenHrmpChannelInvalidRecipient",
            "OpenHrmpChannelZeroCapacity",
            "OpenHrmpChannelCapacityExceedsLimit",
            "OpenHrmpChannelZeroMessageSize",
            "OpenHrmpChannelMessageSizeExceedsLimit",
            "OpenHrmpChannelAlreadyExists",
            "OpenHrmpChannelAlreadyRequested",
            "OpenHrmpChannelLimitExceeded",
            "AcceptHrmpChannelDoesntExist",
            "AcceptHrmpChannelAlreadyConfirmed",
            "AcceptHrmpChannelLimitExceeded",
            "CloseHrmpChannelUnauthorized",
            "CloseHrmpChannelDoesntExist",
            "CloseHrmpChannelAlreadyUnderway",
            "CancelHrmpOpenChannelUnauthorized",
            "OpenHrmpChannelDoesntExist",
            "OpenHrmpChannelAlreadyConfirmed",
            "WrongWitness",
            "ChannelCreationNotAuthorized",
        ],
    },
    /**
     * Lookup810: polkadot_primitives::v8::SessionInfo
     **/
    PolkadotPrimitivesV8SessionInfo: {
        activeValidatorIndices: "Vec<u32>",
        randomSeed: "[u8;32]",
        disputePeriod: "u32",
        validators: "PolkadotPrimitivesV8IndexedVecValidatorIndex",
        discoveryKeys: "Vec<SpAuthorityDiscoveryAppPublic>",
        assignmentKeys: "Vec<PolkadotPrimitivesV8AssignmentAppPublic>",
        validatorGroups: "PolkadotPrimitivesV8IndexedVecGroupIndex",
        nCores: "u32",
        zerothDelayTrancheWidth: "u32",
        relayVrfModuloSamples: "u32",
        nDelayTranches: "u32",
        noShowSlots: "u32",
        neededApprovals: "u32",
    },
    /**
     * Lookup811: polkadot_primitives::v8::IndexedVec<polkadot_primitives::v8::ValidatorIndex, polkadot_primitives::v8::validator_app::Public>
     **/
    PolkadotPrimitivesV8IndexedVecValidatorIndex: "Vec<PolkadotPrimitivesV8ValidatorAppPublic>",
    /**
     * Lookup812: polkadot_primitives::v8::IndexedVec<polkadot_primitives::v8::GroupIndex, V>
     **/
    PolkadotPrimitivesV8IndexedVecGroupIndex: "Vec<Vec<u32>>",
    /**
     * Lookup814: polkadot_primitives::v8::DisputeState<N>
     **/
    PolkadotPrimitivesV8DisputeState: {
        validatorsFor: "BitVec",
        validatorsAgainst: "BitVec",
        start: "u32",
        concludedAt: "Option<u32>",
    },
    /**
     * Lookup816: polkadot_runtime_parachains::disputes::pallet::Error<T>
     **/
    PolkadotRuntimeParachainsDisputesPalletError: {
        _enum: [
            "DuplicateDisputeStatementSets",
            "AncientDisputeStatement",
            "ValidatorIndexOutOfBounds",
            "InvalidSignature",
            "DuplicateStatement",
            "SingleSidedDispute",
            "MaliciousBacker",
            "MissingBackingVotes",
            "UnconfirmedDispute",
        ],
    },
    /**
     * Lookup817: polkadot_primitives::v8::slashing::PendingSlashes
     **/
    PolkadotPrimitivesV8SlashingPendingSlashes: {
        _alias: {
            keys_: "keys",
        },
        keys_: "BTreeMap<u32, PolkadotPrimitivesV8ValidatorAppPublic>",
        kind: "PolkadotPrimitivesV8SlashingSlashingOffenceKind",
    },
    /**
     * Lookup821: polkadot_runtime_parachains::disputes::slashing::pallet::Error<T>
     **/
    PolkadotRuntimeParachainsDisputesSlashingPalletError: {
        _enum: [
            "InvalidKeyOwnershipProof",
            "InvalidSessionIndex",
            "InvalidCandidateHash",
            "InvalidValidatorIndex",
            "ValidatorIndexIdMismatch",
            "DuplicateSlashingReport",
        ],
    },
    /**
     * Lookup822: pallet_message_queue::BookState<dancelight_runtime::AggregateMessageOrigin>
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
     * Lookup824: pallet_message_queue::Neighbours<dancelight_runtime::AggregateMessageOrigin>
     **/
    PalletMessageQueueNeighbours: {
        prev: "DancelightRuntimeAggregateMessageOrigin",
        next: "DancelightRuntimeAggregateMessageOrigin",
    },
    /**
     * Lookup826: pallet_message_queue::Page<Size, HeapSize>
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
     * Lookup828: pallet_message_queue::pallet::Error<T>
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
     * Lookup829: polkadot_runtime_parachains::on_demand::types::CoreAffinityCount
     **/
    PolkadotRuntimeParachainsOnDemandTypesCoreAffinityCount: {
        coreIndex: "u32",
        count: "u32",
    },
    /**
     * Lookup830: polkadot_runtime_parachains::on_demand::types::QueueStatusType
     **/
    PolkadotRuntimeParachainsOnDemandTypesQueueStatusType: {
        traffic: "u128",
        nextIndex: "u32",
        smallestIndex: "u32",
        freedIndices: "BinaryHeapReverseQueueIndex",
    },
    /**
     * Lookup832: BinaryHeap<polkadot_runtime_parachains::on_demand::types::ReverseQueueIndex>
     **/
    BinaryHeapReverseQueueIndex: "Vec<u32>",
    /**
     * Lookup835: BinaryHeap<polkadot_runtime_parachains::on_demand::types::EnqueuedOrder>
     **/
    BinaryHeapEnqueuedOrder: "Vec<PolkadotRuntimeParachainsOnDemandTypesEnqueuedOrder>",
    /**
     * Lookup836: polkadot_runtime_parachains::on_demand::types::EnqueuedOrder
     **/
    PolkadotRuntimeParachainsOnDemandTypesEnqueuedOrder: {
        paraId: "u32",
        idx: "u32",
    },
    /**
     * Lookup840: polkadot_runtime_parachains::on_demand::pallet::Error<T>
     **/
    PolkadotRuntimeParachainsOnDemandPalletError: {
        _enum: ["QueueFull", "SpotPriceHigherThanMaxAmount"],
    },
    /**
     * Lookup841: polkadot_runtime_common::paras_registrar::ParaInfo<sp_core::crypto::AccountId32, Balance>
     **/
    PolkadotRuntimeCommonParasRegistrarParaInfo: {
        manager: "AccountId32",
        deposit: "u128",
        locked: "Option<bool>",
    },
    /**
     * Lookup843: polkadot_runtime_common::paras_registrar::pallet::Error<T>
     **/
    PolkadotRuntimeCommonParasRegistrarPalletError: {
        _enum: [
            "NotRegistered",
            "AlreadyRegistered",
            "NotOwner",
            "CodeTooLarge",
            "HeadDataTooLarge",
            "NotParachain",
            "NotParathread",
            "CannotDeregister",
            "CannotDowngrade",
            "CannotUpgrade",
            "ParaLocked",
            "NotReserved",
            "InvalidCode",
            "CannotSwap",
        ],
    },
    /**
     * Lookup844: pallet_utility::pallet::Error<T>
     **/
    PalletUtilityError: {
        _enum: ["TooManyCalls"],
    },
    /**
     * Lookup845: pallet_identity::types::Registration<Balance, MaxJudgements, pallet_identity::legacy::IdentityInfo<FieldLimit>>
     **/
    PalletIdentityRegistration: {
        judgements: "Vec<(u32,PalletIdentityJudgement)>",
        deposit: "u128",
        info: "PalletIdentityLegacyIdentityInfo",
    },
    /**
     * Lookup853: pallet_identity::types::RegistrarInfo<Balance, sp_core::crypto::AccountId32, IdField>
     **/
    PalletIdentityRegistrarInfo: {
        account: "AccountId32",
        fee: "u128",
        fields: "u64",
    },
    /**
     * Lookup856: pallet_identity::types::AuthorityProperties<sp_core::crypto::AccountId32>
     **/
    PalletIdentityAuthorityProperties: {
        accountId: "AccountId32",
        allocation: "u32",
    },
    /**
     * Lookup857: pallet_identity::types::UsernameInformation<sp_core::crypto::AccountId32, Balance>
     **/
    PalletIdentityUsernameInformation: {
        owner: "AccountId32",
        provider: "PalletIdentityProvider",
    },
    /**
     * Lookup858: pallet_identity::types::Provider<Balance>
     **/
    PalletIdentityProvider: {
        _enum: {
            Allocation: "Null",
            AuthorityDeposit: "u128",
            System: "Null",
        },
    },
    /**
     * Lookup860: pallet_identity::pallet::Error<T>
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
     * Lookup863: pallet_scheduler::Scheduled<Name, frame_support::traits::preimages::Bounded<dancelight_runtime::RuntimeCall, sp_runtime::traits::BlakeTwo256>, BlockNumber, dancelight_runtime::OriginCaller, sp_core::crypto::AccountId32>
     **/
    PalletSchedulerScheduled: {
        maybeId: "Option<[u8;32]>",
        priority: "u8",
        call: "FrameSupportPreimagesBounded",
        maybePeriodic: "Option<(u32,u32)>",
        origin: "DancelightRuntimeOriginCaller",
    },
    /**
     * Lookup865: pallet_scheduler::RetryConfig<Period>
     **/
    PalletSchedulerRetryConfig: {
        totalRetries: "u8",
        remaining: "u8",
        period: "u32",
    },
    /**
     * Lookup866: pallet_scheduler::pallet::Error<T>
     **/
    PalletSchedulerError: {
        _enum: ["FailedToSchedule", "NotFound", "TargetBlockNumberInPast", "RescheduleNoChange", "Named"],
    },
    /**
     * Lookup869: pallet_proxy::ProxyDefinition<sp_core::crypto::AccountId32, dancelight_runtime::ProxyType, BlockNumber>
     **/
    PalletProxyProxyDefinition: {
        delegate: "AccountId32",
        proxyType: "DancelightRuntimeProxyType",
        delay: "u32",
    },
    /**
     * Lookup873: pallet_proxy::Announcement<sp_core::crypto::AccountId32, primitive_types::H256, BlockNumber>
     **/
    PalletProxyAnnouncement: {
        real: "AccountId32",
        callHash: "H256",
        height: "u32",
    },
    /**
     * Lookup875: pallet_proxy::pallet::Error<T>
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
     * Lookup877: pallet_multisig::Multisig<BlockNumber, Balance, sp_core::crypto::AccountId32, MaxApprovals>
     **/
    PalletMultisigMultisig: {
        when: "PalletMultisigTimepoint",
        deposit: "u128",
        depositor: "AccountId32",
        approvals: "Vec<AccountId32>",
    },
    /**
     * Lookup879: pallet_multisig::pallet::Error<T>
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
     * Lookup880: pallet_preimage::OldRequestStatus<sp_core::crypto::AccountId32, Balance>
     **/
    PalletPreimageOldRequestStatus: {
        _enum: {
            Unrequested: {
                deposit: "(AccountId32,u128)",
                len: "u32",
            },
            Requested: {
                deposit: "Option<(AccountId32,u128)>",
                count: "u32",
                len: "Option<u32>",
            },
        },
    },
    /**
     * Lookup883: pallet_preimage::RequestStatus<sp_core::crypto::AccountId32, frame_support::traits::tokens::fungible::HoldConsideration<A, F, R, D, Fp>>
     **/
    PalletPreimageRequestStatus: {
        _enum: {
            Unrequested: {
                ticket: "(AccountId32,u128)",
                len: "u32",
            },
            Requested: {
                maybeTicket: "Option<(AccountId32,u128)>",
                count: "u32",
                maybeLen: "Option<u32>",
            },
        },
    },
    /**
     * Lookup888: pallet_preimage::pallet::Error<T>
     **/
    PalletPreimageError: {
        _enum: [
            "TooBig",
            "AlreadyNoted",
            "NotAuthorized",
            "NotNoted",
            "Requested",
            "NotRequested",
            "TooMany",
            "TooFew",
        ],
    },
    /**
     * Lookup889: pallet_asset_rate::pallet::Error<T>
     **/
    PalletAssetRateError: {
        _enum: ["UnknownAssetKind", "AlreadyExists", "Overflow"],
    },
    /**
     * Lookup890: pallet_xcm::pallet::QueryStatus<BlockNumber>
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
     * Lookup894: xcm::VersionedResponse
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
     * Lookup900: pallet_xcm::pallet::VersionMigrationStage
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
     * Lookup902: pallet_xcm::pallet::RemoteLockedFungibleRecord<ConsumerIdentifier, MaxConsumers>
     **/
    PalletXcmRemoteLockedFungibleRecord: {
        amount: "u128",
        owner: "XcmVersionedLocation",
        locker: "XcmVersionedLocation",
        consumers: "Vec<(Null,u128)>",
    },
    /**
     * Lookup909: pallet_xcm::pallet::Error<T>
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
     * Lookup910: pallet_stream_payment::pallet::Stream<sp_core::crypto::AccountId32, tp_stream_payment_common::TimeUnit, tp_stream_payment_common::AssetId, Balance>
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
     * Lookup912: pallet_stream_payment::pallet::ChangeRequest<tp_stream_payment_common::TimeUnit, tp_stream_payment_common::AssetId, Balance>
     **/
    PalletStreamPaymentChangeRequest: {
        requester: "PalletStreamPaymentParty",
        kind: "PalletStreamPaymentChangeKind",
        newConfig: "PalletStreamPaymentStreamConfig",
        depositChange: "Option<PalletStreamPaymentDepositChange>",
    },
    /**
     * Lookup914: pallet_stream_payment::pallet::Error<T>
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
     * Lookup915: pallet_migrations::pallet::Error<T>
     **/
    PalletMigrationsError: {
        _enum: ["PreimageMissing", "WrongUpperBound", "PreimageIsTooBig", "PreimageAlreadyExists"],
    },
    /**
     * Lookup919: pallet_beefy::pallet::Error<T>
     **/
    PalletBeefyError: {
        _enum: [
            "InvalidKeyOwnershipProof",
            "InvalidDoubleVotingProof",
            "InvalidForkVotingProof",
            "InvalidFutureBlockVotingProof",
            "InvalidEquivocationProofSession",
            "DuplicateOffenceReport",
            "InvalidConfiguration",
        ],
    },
    /**
     * Lookup920: sp_consensus_beefy::mmr::BeefyAuthoritySet<primitive_types::H256>
     **/
    SpConsensusBeefyMmrBeefyAuthoritySet: {
        id: "u64",
        len: "u32",
        keysetCommitment: "H256",
    },
    /**
     * Lookup921: snowbridge_beacon_primitives::types::CompactBeaconState
     **/
    SnowbridgeBeaconPrimitivesCompactBeaconState: {
        slot: "Compact<u64>",
        blockRootsRoot: "H256",
    },
    /**
     * Lookup922: snowbridge_beacon_primitives::types::SyncCommitteePrepared
     **/
    SnowbridgeBeaconPrimitivesSyncCommitteePrepared: {
        root: "H256",
        pubkeys: "[Lookup924;512]",
        aggregatePubkey: "SnowbridgeMilagroBlsKeysPublicKey",
    },
    /**
     * Lookup924: snowbridge_milagro_bls::keys::PublicKey
     **/
    SnowbridgeMilagroBlsKeysPublicKey: {
        point: "SnowbridgeAmclBls381Ecp",
    },
    /**
     * Lookup925: snowbridge_amcl::bls381::ecp::ECP
     **/
    SnowbridgeAmclBls381Ecp: {
        x: "SnowbridgeAmclBls381Fp",
        y: "SnowbridgeAmclBls381Fp",
        z: "SnowbridgeAmclBls381Fp",
    },
    /**
     * Lookup926: snowbridge_amcl::bls381::fp::FP
     **/
    SnowbridgeAmclBls381Fp: {
        x: "SnowbridgeAmclBls381Big",
        xes: "i32",
    },
    /**
     * Lookup927: snowbridge_amcl::bls381::big::Big
     **/
    SnowbridgeAmclBls381Big: {
        w: "[i32;14]",
    },
    /**
     * Lookup930: snowbridge_beacon_primitives::types::ForkVersions
     **/
    SnowbridgeBeaconPrimitivesForkVersions: {
        genesis: "SnowbridgeBeaconPrimitivesFork",
        altair: "SnowbridgeBeaconPrimitivesFork",
        bellatrix: "SnowbridgeBeaconPrimitivesFork",
        capella: "SnowbridgeBeaconPrimitivesFork",
        deneb: "SnowbridgeBeaconPrimitivesFork",
        electra: "SnowbridgeBeaconPrimitivesFork",
    },
    /**
     * Lookup931: snowbridge_beacon_primitives::types::Fork
     **/
    SnowbridgeBeaconPrimitivesFork: {
        version: "[u8;4]",
        epoch: "u64",
    },
    /**
     * Lookup932: snowbridge_pallet_ethereum_client::pallet::Error<T>
     **/
    SnowbridgePalletEthereumClientError: {
        _enum: {
            SkippedSyncCommitteePeriod: "Null",
            SyncCommitteeUpdateRequired: "Null",
            IrrelevantUpdate: "Null",
            NotBootstrapped: "Null",
            SyncCommitteeParticipantsNotSupermajority: "Null",
            InvalidHeaderMerkleProof: "Null",
            InvalidSyncCommitteeMerkleProof: "Null",
            InvalidExecutionHeaderProof: "Null",
            InvalidAncestryMerkleProof: "Null",
            InvalidBlockRootsRootMerkleProof: "Null",
            InvalidFinalizedHeaderGap: "Null",
            HeaderNotFinalized: "Null",
            BlockBodyHashTreeRootFailed: "Null",
            HeaderHashTreeRootFailed: "Null",
            SyncCommitteeHashTreeRootFailed: "Null",
            SigningRootHashTreeRootFailed: "Null",
            ForkDataHashTreeRootFailed: "Null",
            ExpectedFinalizedHeaderNotStored: "Null",
            BLSPreparePublicKeysFailed: "Null",
            BLSVerificationFailed: "SnowbridgeBeaconPrimitivesBlsBlsError",
            InvalidUpdateSlot: "Null",
            InvalidSyncCommitteeUpdate: "Null",
            ExecutionHeaderTooFarBehind: "Null",
            ExecutionHeaderSkippedBlock: "Null",
            Halted: "Null",
        },
    },
    /**
     * Lookup933: snowbridge_beacon_primitives::bls::BlsError
     **/
    SnowbridgeBeaconPrimitivesBlsBlsError: {
        _enum: ["InvalidSignature", "InvalidPublicKey", "InvalidAggregatePublicKeys", "SignatureVerificationFailed"],
    },
    /**
     * Lookup934: polkadot_runtime_common::paras_sudo_wrapper::pallet::Error<T>
     **/
    PolkadotRuntimeCommonParasSudoWrapperPalletError: {
        _enum: [
            "ParaDoesntExist",
            "ParaAlreadyExists",
            "ExceedsMaxMessageSize",
            "CouldntCleanup",
            "NotParathread",
            "NotParachain",
            "CannotUpgrade",
            "CannotDowngrade",
            "TooManyCores",
        ],
    },
    /**
     * Lookup935: pallet_sudo::pallet::Error<T>
     **/
    PalletSudoError: {
        _enum: ["RequireSudo"],
    },
    /**
     * Lookup938: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
     **/
    FrameSystemExtensionsCheckNonZeroSender: "Null",
    /**
     * Lookup939: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
     **/
    FrameSystemExtensionsCheckSpecVersion: "Null",
    /**
     * Lookup940: frame_system::extensions::check_tx_version::CheckTxVersion<T>
     **/
    FrameSystemExtensionsCheckTxVersion: "Null",
    /**
     * Lookup941: frame_system::extensions::check_genesis::CheckGenesis<T>
     **/
    FrameSystemExtensionsCheckGenesis: "Null",
    /**
     * Lookup944: frame_system::extensions::check_nonce::CheckNonce<T>
     **/
    FrameSystemExtensionsCheckNonce: "Compact<u32>",
    /**
     * Lookup945: frame_system::extensions::check_weight::CheckWeight<T>
     **/
    FrameSystemExtensionsCheckWeight: "Null",
    /**
     * Lookup946: pallet_transaction_payment::ChargeTransactionPayment<T>
     **/
    PalletTransactionPaymentChargeTransactionPayment: "Compact<u128>",
    /**
     * Lookup947: frame_metadata_hash_extension::CheckMetadataHash<T>
     **/
    FrameMetadataHashExtensionCheckMetadataHash: {
        mode: "FrameMetadataHashExtensionMode",
    },
    /**
     * Lookup948: frame_metadata_hash_extension::Mode
     **/
    FrameMetadataHashExtensionMode: {
        _enum: ["Disabled", "Enabled"],
    },
    /**
     * Lookup949: dancelight_runtime::Runtime
     **/
    DancelightRuntimeRuntime: "Null",
};
