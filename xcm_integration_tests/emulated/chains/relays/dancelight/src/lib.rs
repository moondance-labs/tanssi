pub use dancelight_runtime;

use {
    dancelight_runtime_constants::currency::UNITS as UNIT,
    dancelight_runtime_test_utils::ExtBuilder,
    emulated_integration_tests_common::xcm_emulator::decl_test_parachains,
    frame_support::{parameter_types, traits::OnIdle},
    frame_system::Pallet as SystemPallet,
    snowbridge_pallet_outbound_queue::CommittedMessage,
    tanssi_emulated_integration_tests_common,
    xcm_emulator::{
        decl_test_networks, decl_test_relay_chains, Bridge, BridgeLaneId, BridgeMessage,
        BridgeMessageDispatchError, BridgeMessageHandler, Chain, Network, Parachain, RelayChain,
        TestExt,
    },
};

pub mod bridge;

decl_test_relay_chains! {
    #[api_version(11)]
    pub struct Dancelight {
        genesis = ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (dancelight_runtime::AccountId::from(tanssi_emulated_integration_tests_common::accounts::ALICE), 210_000 * UNIT),
            (dancelight_runtime::AccountId::from(tanssi_emulated_integration_tests_common::accounts::BOB), 100_000 * UNIT),
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
            SovereignAccountOf: dancelight_runtime::xcm_config::LocationConverter,
        },
        pallets = {
            System: dancelight_runtime::System,
            Session: dancelight_runtime::Session,
            Configuration: dancelight_runtime::Configuration,
            Balances: dancelight_runtime::Balances,
            Registrar: dancelight_runtime::Registrar,
            ParasSudoWrapper: dancelight_runtime::ParasSudoWrapper,
            OnDemandAssignmentProvider: dancelight_runtime::OnDemandAssignmentProvider,
            XcmPallet: dancelight_runtime::XcmPallet,
            Sudo: dancelight_runtime::Sudo,
            MessageQueue: dancelight_runtime::MessageQueue,
            ExternalValidatorSlashes: dancelight_runtime::ExternalValidatorSlashes,
            EthereumOutboundQueue: dancelight_runtime::EthereumOutboundQueue,
            EthereumInboundQueue: dancelight_runtime::EthereumInboundQueue,
            EthereumSystem: dancelight_runtime::EthereumSystem,
            ExternalValidators: dancelight_runtime::ExternalValidators,
        }
    }
}

parameter_types! {
    // Dancelight
    pub DancelightSender: dancelight_runtime::AccountId = dancelight_runtime::AccountId::from(tanssi_emulated_integration_tests_common::accounts::ALICE);
    pub DancelightReceiver: dancelight_runtime::AccountId = dancelight_runtime::AccountId::from(tanssi_emulated_integration_tests_common::accounts::BOB);
    pub DancelightEmptyReceiver: dancelight_runtime::AccountId = DancelightRelay::account_id_of(tanssi_emulated_integration_tests_common::accounts::RANDOM);
}
