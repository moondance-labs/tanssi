// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

/* eslint-disable sort-keys */

export default {
  /**
   * Lookup3: frame_system::AccountInfo<Index,
   * pallet_balances::types::AccountData<Balance>>
   */
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
  /**
   * Lookup19: frame_system::EventRecord<dancebox_runtime::RuntimeEvent,
   * primitive_types::H256>
   */
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
    _enum: [
      "Any",
      "NonTransfer",
      "Governance",
      "Staking",
      "CancelProxy",
      "Balances",
    ],
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
      BootNodesChanged: {
        paraId: "u32",
      },
    },
  },
  /** Lookup47: pallet_author_noting::pallet::Event<T> */
  PalletAuthorNotingEvent: {
    _enum: {
      LatestAuthorChanged: {
        paraId: "u32",
        blockNumber: "u32",
        newAuthor: "AccountId32",
      },
    },
  },
  /** Lookup48: pallet_collator_selection::pallet::Event<T> */
  PalletCollatorSelectionEvent: {
    _enum: {
      NewInvulnerables: {
        invulnerables: "Vec<AccountId32>",
      },
      NewDesiredCandidates: {
        desiredCandidates: "u32",
      },
      NewCandidacyBond: {
        bondAmount: "u128",
      },
      CandidateAdded: {
        accountId: "AccountId32",
        deposit: "u128",
      },
      CandidateRemoved: {
        accountId: "AccountId32",
      },
    },
  },
  /** Lookup50: pallet_session::pallet::Event */
  PalletSessionEvent: {
    _enum: {
      NewSession: {
        sessionIndex: "u32",
      },
    },
  },
  /** Lookup51: frame_system::Phase */
  FrameSystemPhase: {
    _enum: {
      ApplyExtrinsic: "u32",
      Finalization: "Null",
      Initialization: "Null",
    },
  },
  /** Lookup55: frame_system::LastRuntimeUpgradeInfo */
  FrameSystemLastRuntimeUpgradeInfo: {
    specVersion: "Compact<u32>",
    specName: "Text",
  },
  /** Lookup59: frame_system::pallet::Call<T> */
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
  /** Lookup63: frame_system::limits::BlockWeights */
  FrameSystemLimitsBlockWeights: {
    baseBlock: "SpWeightsWeightV2Weight",
    maxBlock: "SpWeightsWeightV2Weight",
    perClass: "FrameSupportDispatchPerDispatchClassWeightsPerClass",
  },
  /**
   * Lookup64:
   * frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
   */
  FrameSupportDispatchPerDispatchClassWeightsPerClass: {
    normal: "FrameSystemLimitsWeightsPerClass",
    operational: "FrameSystemLimitsWeightsPerClass",
    mandatory: "FrameSystemLimitsWeightsPerClass",
  },
  /** Lookup65: frame_system::limits::WeightsPerClass */
  FrameSystemLimitsWeightsPerClass: {
    baseExtrinsic: "SpWeightsWeightV2Weight",
    maxExtrinsic: "Option<SpWeightsWeightV2Weight>",
    maxTotal: "Option<SpWeightsWeightV2Weight>",
    reserved: "Option<SpWeightsWeightV2Weight>",
  },
  /** Lookup67: frame_system::limits::BlockLength */
  FrameSystemLimitsBlockLength: {
    max: "FrameSupportDispatchPerDispatchClassU32",
  },
  /** Lookup68: frame_support::dispatch::PerDispatchClass<T> */
  FrameSupportDispatchPerDispatchClassU32: {
    normal: "u32",
    operational: "u32",
    mandatory: "u32",
  },
  /** Lookup69: sp_weights::RuntimeDbWeight */
  SpWeightsRuntimeDbWeight: {
    read: "u64",
    write: "u64",
  },
  /** Lookup70: sp_version::RuntimeVersion */
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
  /** Lookup75: frame_system::pallet::Error<T> */
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
  /** Lookup76: polkadot_primitives::v4::PersistedValidationData<primitive_types::H256, N> */
  PolkadotPrimitivesV4PersistedValidationData: {
    parentHead: "Bytes",
    relayParentNumber: "u32",
    relayParentStorageRoot: "H256",
    maxPovSize: "u32",
  },
  /** Lookup79: polkadot_primitives::v4::UpgradeRestriction */
  PolkadotPrimitivesV4UpgradeRestriction: {
    _enum: ["Present"],
  },
  /** Lookup80: sp_trie::storage_proof::StorageProof */
  SpTrieStorageProof: {
    trieNodes: "BTreeSet<Bytes>",
  },
  /**
   * Lookup82:
   * cumulus_pallet_parachain_system::relay_state_snapshot::MessagingStateSnapshot
   */
  CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: {
    dmqMqcHead: "H256",
    relayDispatchQueueSize:
      "CumulusPalletParachainSystemRelayStateSnapshotRelayDispachQueueSize",
    ingressChannels: "Vec<(u32,PolkadotPrimitivesV4AbridgedHrmpChannel)>",
    egressChannels: "Vec<(u32,PolkadotPrimitivesV4AbridgedHrmpChannel)>",
  },
  /**
   * Lookup83:
   * cumulus_pallet_parachain_system::relay_state_snapshot::RelayDispachQueueSize
   */
  CumulusPalletParachainSystemRelayStateSnapshotRelayDispachQueueSize: {
    remainingCount: "u32",
    remainingSize: "u32",
  },
  /** Lookup86: polkadot_primitives::v4::AbridgedHrmpChannel */
  PolkadotPrimitivesV4AbridgedHrmpChannel: {
    maxCapacity: "u32",
    maxTotalSize: "u32",
    maxMessageSize: "u32",
    msgCount: "u32",
    totalSize: "u32",
    mqcHead: "Option<H256>",
  },
  /** Lookup88: polkadot_primitives::v4::AbridgedHostConfiguration */
  PolkadotPrimitivesV4AbridgedHostConfiguration: {
    maxCodeSize: "u32",
    maxHeadDataSize: "u32",
    maxUpwardQueueCount: "u32",
    maxUpwardQueueSize: "u32",
    maxUpwardMessageSize: "u32",
    maxUpwardMessageNumPerCandidate: "u32",
    hrmpMaxMessageNumPerCandidate: "u32",
    validationUpgradeCooldown: "u32",
    validationUpgradeDelay: "u32",
  },
  /**
   * Lookup94:
   * polkadot_core_primitives::OutboundHrmpMessage<polkadot_parachain::primitives::Id>
   */
  PolkadotCorePrimitivesOutboundHrmpMessage: {
    recipient: "u32",
    data: "Bytes",
  },
  /** Lookup95: cumulus_pallet_parachain_system::CodeUpgradeAuthorization<T> */
  CumulusPalletParachainSystemCodeUpgradeAuthorization: {
    codeHash: "H256",
    checkVersion: "bool",
  },
  /** Lookup96: cumulus_pallet_parachain_system::pallet::Call<T> */
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
  /** Lookup97: cumulus_primitives_parachain_inherent::ParachainInherentData */
  CumulusPrimitivesParachainInherentParachainInherentData: {
    validationData: "PolkadotPrimitivesV4PersistedValidationData",
    relayChainState: "SpTrieStorageProof",
    downwardMessages: "Vec<PolkadotCorePrimitivesInboundDownwardMessage>",
    horizontalMessages:
      "BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>",
  },
  /** Lookup99: polkadot_core_primitives::InboundDownwardMessage<BlockNumber> */
  PolkadotCorePrimitivesInboundDownwardMessage: {
    sentAt: "u32",
    msg: "Bytes",
  },
  /** Lookup102: polkadot_core_primitives::InboundHrmpMessage<BlockNumber> */
  PolkadotCorePrimitivesInboundHrmpMessage: {
    sentAt: "u32",
    data: "Bytes",
  },
  /** Lookup105: cumulus_pallet_parachain_system::pallet::Error<T> */
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
  /** Lookup106: pallet_timestamp::pallet::Call<T> */
  PalletTimestampCall: {
    _enum: {
      set: {
        now: "Compact<u64>",
      },
    },
  },
  /** Lookup107: parachain_info::pallet::Call<T> */
  ParachainInfoCall: "Null",
  /** Lookup108: pallet_sudo::pallet::Call<T> */
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
  /** Lookup110: pallet_utility::pallet::Call<T> */
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
  /** Lookup112: dancebox_runtime::OriginCaller */
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
  /** Lookup113: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32> */
  FrameSupportDispatchRawOrigin: {
    _enum: {
      Root: "Null",
      Signed: "AccountId32",
      None: "Null",
    },
  },
  /** Lookup114: sp_core::Void */
  SpCoreVoid: "Null",
  /** Lookup115: pallet_proxy::pallet::Call<T> */
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
  /** Lookup120: pallet_maintenance_mode::pallet::Call<T> */
  PalletMaintenanceModeCall: {
    _enum: ["enter_maintenance_mode", "resume_normal_operation"],
  },
  /** Lookup121: pallet_balances::pallet::Call<T, I> */
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
  /** Lookup123: pallet_registrar::pallet::Call<T> */
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
    },
  },
  /**
   * Lookup124:
   * tp_container_chain_genesis_data::ContainerChainGenesisData<MaxLengthTokenSymbol>
   */
  TpContainerChainGenesisDataContainerChainGenesisData: {
    storage: "Vec<TpContainerChainGenesisDataContainerChainGenesisDataItem>",
    name: "Bytes",
    id: "Bytes",
    forkId: "Option<Bytes>",
    extensions: "Bytes",
    properties: "TpContainerChainGenesisDataProperties",
  },
  /** Lookup126: tp_container_chain_genesis_data::ContainerChainGenesisDataItem */
  TpContainerChainGenesisDataContainerChainGenesisDataItem: {
    key: "Bytes",
    value: "Bytes",
  },
  /** Lookup128: tp_container_chain_genesis_data::Properties<MaxLengthTokenSymbol> */
  TpContainerChainGenesisDataProperties: {
    tokenMetadata: "TpContainerChainGenesisDataTokenMetadata",
    isEthereum: "bool",
  },
  /** Lookup129: tp_container_chain_genesis_data::TokenMetadata<MaxLengthTokenSymbol> */
  TpContainerChainGenesisDataTokenMetadata: {
    tokenSymbol: "Bytes",
    ss58Format: "u32",
    tokenDecimals: "u32",
  },
  /** Lookup134: pallet_configuration::pallet::Call<T> */
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
      set_bypass_consistency_check: {
        _alias: {
          new_: "new",
        },
        new_: "bool",
      },
    },
  },
  /** Lookup135: pallet_collator_assignment::pallet::Call<T> */
  PalletCollatorAssignmentCall: "Null",
  /** Lookup136: pallet_author_noting::pallet::Call<T> */
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
    },
  },
  /** Lookup137: tp_author_noting_inherent::OwnParachainInherentData */
  TpAuthorNotingInherentOwnParachainInherentData: {
    relayStorageProof: "SpTrieStorageProof",
  },
  /** Lookup138: pallet_authority_assignment::pallet::Call<T> */
  PalletAuthorityAssignmentCall: "Null",
  /** Lookup139: pallet_collator_selection::pallet::Call<T> */
  PalletCollatorSelectionCall: {
    _enum: {
      set_invulnerables: {
        _alias: {
          new_: "new",
        },
        new_: "Vec<AccountId32>",
      },
      set_desired_candidates: {
        max: "u32",
      },
      set_candidacy_bond: {
        bond: "u128",
      },
      register_as_candidate: "Null",
      leave_intent: "Null",
    },
  },
  /** Lookup140: pallet_session::pallet::Call<T> */
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
  /** Lookup141: dancebox_runtime::SessionKeys */
  DanceboxRuntimeSessionKeys: {
    nimbus: "NimbusPrimitivesNimbusCryptoPublic",
  },
  /** Lookup142: nimbus_primitives::nimbus_crypto::Public */
  NimbusPrimitivesNimbusCryptoPublic: "SpCoreSr25519Public",
  /** Lookup143: sp_core::sr25519::Public */
  SpCoreSr25519Public: "[u8;32]",
  /** Lookup144: pallet_author_inherent::pallet::Call<T> */
  PalletAuthorInherentCall: {
    _enum: ["kick_off_authorship_validation"],
  },
  /** Lookup145: pallet_root_testing::pallet::Call<T> */
  PalletRootTestingCall: {
    _enum: {
      fill_block: {
        ratio: "Perbill",
      },
    },
  },
  /** Lookup147: pallet_sudo::pallet::Error<T> */
  PalletSudoError: {
    _enum: ["RequireSudo"],
  },
  /** Lookup148: pallet_utility::pallet::Error<T> */
  PalletUtilityError: {
    _enum: ["TooManyCalls"],
  },
  /**
   * Lookup151: pallet_proxy::ProxyDefinition<sp_core::crypto::AccountId32,
   * dancebox_runtime::ProxyType, BlockNumber>
   */
  PalletProxyProxyDefinition: {
    delegate: "AccountId32",
    proxyType: "DanceboxRuntimeProxyType",
    delay: "u32",
  },
  /**
   * Lookup155: pallet_proxy::Announcement<sp_core::crypto::AccountId32,
   * primitive_types::H256, BlockNumber>
   */
  PalletProxyAnnouncement: {
    real: "AccountId32",
    callHash: "H256",
    height: "u32",
  },
  /** Lookup157: pallet_proxy::pallet::Error<T> */
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
  /** Lookup158: pallet_migrations::pallet::Error<T> */
  PalletMigrationsError: {
    _enum: [
      "PreimageMissing",
      "WrongUpperBound",
      "PreimageIsTooBig",
      "PreimageAlreadyExists",
    ],
  },
  /** Lookup159: pallet_maintenance_mode::pallet::Error<T> */
  PalletMaintenanceModeError: {
    _enum: ["AlreadyInMaintenanceMode", "NotInMaintenanceMode"],
  },
  /** Lookup161: pallet_balances::types::BalanceLock<Balance> */
  PalletBalancesBalanceLock: {
    id: "[u8;8]",
    amount: "u128",
    reasons: "PalletBalancesReasons",
  },
  /** Lookup162: pallet_balances::types::Reasons */
  PalletBalancesReasons: {
    _enum: ["Fee", "Misc", "All"],
  },
  /** Lookup165: pallet_balances::types::ReserveData<ReserveIdentifier, Balance> */
  PalletBalancesReserveData: {
    id: "[u8;8]",
    amount: "u128",
  },
  /** Lookup168: pallet_balances::types::IdAmount<Id, Balance> */
  PalletBalancesIdAmount: {
    id: "Null",
    amount: "u128",
  },
  /** Lookup170: pallet_balances::pallet::Error<T, I> */
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
  /** Lookup172: pallet_transaction_payment::Releases */
  PalletTransactionPaymentReleases: {
    _enum: ["V1Ancient", "V2"],
  },
  /** Lookup177: pallet_registrar::pallet::DepositInfo<T> */
  PalletRegistrarDepositInfo: {
    creator: "AccountId32",
    deposit: "u128",
  },
  /** Lookup178: pallet_registrar::pallet::Error<T> */
  PalletRegistrarError: {
    _enum: [
      "ParaIdAlreadyRegistered",
      "ParaIdNotRegistered",
      "ParaIdListFull",
      "GenesisDataTooBig",
      "ParaIdNotInPendingVerification",
      "NotSufficientDeposit",
    ],
  },
  /** Lookup179: pallet_configuration::HostConfiguration */
  PalletConfigurationHostConfiguration: {
    maxCollators: "u32",
    minOrchestratorCollators: "u32",
    maxOrchestratorCollators: "u32",
    collatorsPerContainer: "u32",
  },
  /** Lookup182: pallet_configuration::pallet::Error<T> */
  PalletConfigurationError: {
    _enum: ["InvalidNewValue"],
  },
  /** Lookup183: tp_collator_assignment::AssignedCollators<sp_core::crypto::AccountId32> */
  TpCollatorAssignmentAssignedCollatorsAccountId32: {
    orchestratorChain: "Vec<AccountId32>",
    containerChains: "BTreeMap<u32, Vec<AccountId32>>",
  },
  /** Lookup188: pallet_initializer::pallet::BufferedSessionChange<T> */
  PalletInitializerBufferedSessionChange: {
    changed: "bool",
    validators: "Vec<(AccountId32,NimbusPrimitivesNimbusCryptoPublic)>",
    queued: "Vec<(AccountId32,NimbusPrimitivesNimbusCryptoPublic)>",
    sessionIndex: "u32",
  },
  /** Lookup191: pallet_author_noting::pallet::ContainerChainBlockInfo<T> */
  PalletAuthorNotingContainerChainBlockInfo: {
    blockNumber: "u32",
    author: "AccountId32",
  },
  /** Lookup192: pallet_author_noting::pallet::Error<T> */
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
   * Lookup193:
   * tp_collator_assignment::AssignedCollators<nimbus_primitives::nimbus_crypto::Public>
   */
  TpCollatorAssignmentAssignedCollatorsPublic: {
    orchestratorChain: "Vec<NimbusPrimitivesNimbusCryptoPublic>",
    containerChains: "BTreeMap<u32, Vec<NimbusPrimitivesNimbusCryptoPublic>>",
  },
  /**
   * Lookup200:
   * pallet_collator_selection::pallet::CandidateInfo<sp_core::crypto::AccountId32,
   * Balance>
   */
  PalletCollatorSelectionCandidateInfo: {
    who: "AccountId32",
    deposit: "u128",
  },
  /** Lookup202: pallet_collator_selection::pallet::Error<T> */
  PalletCollatorSelectionError: {
    _enum: [
      "TooManyCandidates",
      "TooFewCandidates",
      "Unknown",
      "Permission",
      "AlreadyCandidate",
      "NotCandidate",
      "TooManyInvulnerables",
      "AlreadyInvulnerable",
      "NoAssociatedValidatorId",
      "ValidatorNotRegistered",
    ],
  },
  /** Lookup207: sp_core::crypto::KeyTypeId */
  SpCoreCryptoKeyTypeId: "[u8;4]",
  /** Lookup208: pallet_session::pallet::Error<T> */
  PalletSessionError: {
    _enum: [
      "InvalidProof",
      "NoAssociatedValidatorId",
      "DuplicatedKey",
      "NoKeys",
      "NoAccount",
    ],
  },
  /** Lookup212: pallet_author_inherent::pallet::Error<T> */
  PalletAuthorInherentError: {
    _enum: ["AuthorAlreadySet", "NoAccountId", "CannotBeAuthor"],
  },
  /** Lookup214: sp_runtime::MultiSignature */
  SpRuntimeMultiSignature: {
    _enum: {
      Ed25519: "SpCoreEd25519Signature",
      Sr25519: "SpCoreSr25519Signature",
      Ecdsa: "SpCoreEcdsaSignature",
    },
  },
  /** Lookup215: sp_core::ed25519::Signature */
  SpCoreEd25519Signature: "[u8;64]",
  /** Lookup217: sp_core::sr25519::Signature */
  SpCoreSr25519Signature: "[u8;64]",
  /** Lookup218: sp_core::ecdsa::Signature */
  SpCoreEcdsaSignature: "[u8;65]",
  /** Lookup221: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T> */
  FrameSystemExtensionsCheckNonZeroSender: "Null",
  /** Lookup222: frame_system::extensions::check_spec_version::CheckSpecVersion<T> */
  FrameSystemExtensionsCheckSpecVersion: "Null",
  /** Lookup223: frame_system::extensions::check_tx_version::CheckTxVersion<T> */
  FrameSystemExtensionsCheckTxVersion: "Null",
  /** Lookup224: frame_system::extensions::check_genesis::CheckGenesis<T> */
  FrameSystemExtensionsCheckGenesis: "Null",
  /** Lookup227: frame_system::extensions::check_nonce::CheckNonce<T> */
  FrameSystemExtensionsCheckNonce: "Compact<u32>",
  /** Lookup228: frame_system::extensions::check_weight::CheckWeight<T> */
  FrameSystemExtensionsCheckWeight: "Null",
  /** Lookup229: pallet_transaction_payment::ChargeTransactionPayment<T> */
  PalletTransactionPaymentChargeTransactionPayment: "Compact<u128>",
  /** Lookup230: dancebox_runtime::Runtime */
  DanceboxRuntimeRuntime: "Null",
};
