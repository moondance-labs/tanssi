use {
    container_chain_template_simple_runtime,
    emulated_integration_tests_common::xcm_emulator::decl_test_parachains,
    frame_support::parameter_types,
    xcm_emulator::{
        decl_test_networks, decl_test_relay_chains, Bridge, BridgeLaneId, BridgeMessage,
        BridgeMessageDispatchError, BridgeMessageHandler, Chain, Network, Parachain, RelayChain,
        TestExt,
    },
};

pub use dancelight_runtime;

pub mod genesis;

decl_test_parachains! {
    pub struct SimpleTemplateDancelight {
        genesis = crate::genesis::genesis(),
        on_init = (),
        runtime = container_chain_template_simple_runtime,
        core = {
            XcmpMessageHandler: container_chain_template_simple_runtime::XcmpQueue,
            LocationToAccountId: container_chain_template_simple_runtime::xcm_config::LocationToAccountId,
            ParachainInfo: container_chain_template_simple_runtime::ParachainInfo,
            MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
        },
        pallets = {
            System: container_chain_template_simple_runtime::System,
            Balances: container_chain_template_simple_runtime::Balances,
            ParachainSystem: container_chain_template_simple_runtime::ParachainSystem,
            PolkadotXcm: container_chain_template_simple_runtime::PolkadotXcm,
            ForeignAssets:  container_chain_template_simple_runtime::ForeignAssets,
            AssetRate:  container_chain_template_simple_runtime::AssetRate,
            ForeignAssetsCreator: container_chain_template_simple_runtime::ForeignAssetsCreator,
        }
    }
}

parameter_types! {
    // SimpleTemplate
    pub SimpleTemplateDancelightSender: container_chain_template_simple_runtime::AccountId = dancelight_runtime::AccountId::from(tanssi_emulated_integration_tests_common::accounts::ALICE);
    pub SimpleTemplateDancelightReceiver: container_chain_template_simple_runtime::AccountId = dancelight_runtime::AccountId::from(tanssi_emulated_integration_tests_common::accounts::BOB);
    pub SimpleTemplateDancelightEmptyReceiver: container_chain_template_simple_runtime::AccountId = SimpleTemplateDancelightPara::account_id_of(tanssi_emulated_integration_tests_common::accounts::RANDOM);
}
