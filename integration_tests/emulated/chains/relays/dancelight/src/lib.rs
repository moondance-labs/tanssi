use {
    super::constants::{
        accounts::{ALICE, BOB, RANDOM},
        frontier_template, simple_template,
    },
    crate::tests::common::ExtBuilder,
    dancelight_runtime_constants::currency::UNITS as UNIT,
    emulated_integration_tests_common::xcm_emulator::decl_test_parachains,
    frame_support::{parameter_types, traits::OnIdle},
    frame_system::Pallet as SystemPallet,
    snowbridge_pallet_outbound_queue::CommittedMessage,
    sp_std::cell::RefCell,
    xcm_emulator::{
        decl_test_networks, decl_test_relay_chains, Bridge, BridgeLaneId, BridgeMessage,
        BridgeMessageDispatchError, BridgeMessageHandler, Chain, Network, Parachain, RelayChain,
        TestExt,
    },
};

decl_test_relay_chains! {
    #[api_version(11)]
    pub struct Dancelight {
        genesis = ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (crate::AccountId::from(crate::tests::common::ALICE), 210_000 * UNIT),
            (crate::AccountId::from(crate::tests::common::BOB), 100_000 * UNIT),
        ])
        .with_relay_config(runtime_parachains::configuration::HostConfiguration {
            max_downward_message_size: 1024 * 1024,
            ..Default::default()
        })
        .with_safe_xcm_version(3)
        .build_storage(),
        on_init = (),
        runtime = crate,
        core = {
            SovereignAccountOf: crate::xcm_config::LocationConverter,
        },
        pallets = {
            System: crate::System,
            Session: crate::Session,
            Configuration: crate::Configuration,
            Balances: crate::Balances,
            Registrar: crate::Registrar,
            ParasSudoWrapper: crate::ParasSudoWrapper,
            OnDemandAssignmentProvider: crate::OnDemandAssignmentProvider,
            XcmPallet: crate::XcmPallet,
            Sudo: crate::Sudo,
            MessageQueue: crate::MessageQueue,
            ExternalValidatorSlashes: crate::ExternalValidatorSlashes,
            EthereumOutboundQueue: crate::EthereumOutboundQueue,
            EthereumInboundQueue: crate::EthereumInboundQueue,
            EthereumSystem: crate::EthereumSystem,
            ExternalValidators: crate::ExternalValidators,
        }
    }
}

parameter_types! {
    // Dancelight
    pub DancelightSender: crate::AccountId = crate::AccountId::from(crate::tests::common::ALICE);
    pub DancelightReceiver: crate::AccountId = crate::AccountId::from(crate::tests::common::BOB);
    pub DancelightEmptyReceiver: crate::AccountId = DancelightRelay::account_id_of(RANDOM);
}
