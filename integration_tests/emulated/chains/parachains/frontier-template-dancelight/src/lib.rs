use {
    // super::constants::{
    //     accounts::{ALICE, BOB, RANDOM},
    //     frontier_template, simple_template,
    // },
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

decl_test_parachains! {
    // Dancelight parachains
    pub struct FrontierTemplateDancelight {
        genesis = frontier_template::genesis(),
        on_init = (),
        runtime = container_chain_template_frontier_runtime,
        core = {
            XcmpMessageHandler: container_chain_template_frontier_runtime::XcmpQueue,
            LocationToAccountId: container_chain_template_frontier_runtime::xcm_config::LocationToAccountId,
            ParachainInfo: container_chain_template_frontier_runtime::ParachainInfo,
            MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
        },
        pallets = {
            System: container_chain_template_frontier_runtime::System,
            Balances: container_chain_template_frontier_runtime::Balances,
            ParachainSystem: container_chain_template_frontier_runtime::ParachainSystem,
            PolkadotXcm: container_chain_template_frontier_runtime::PolkadotXcm,
            ForeignAssets:  container_chain_template_frontier_runtime::ForeignAssets,
            AssetRate:  container_chain_template_frontier_runtime::AssetRate,
            ForeignAssetsCreator: container_chain_template_frontier_runtime::ForeignAssetsCreator,
        }
    },
}
