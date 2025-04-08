pub use xcm_emulator::{bx, TestExt};
use {
    dancebox_emulated_chain::Dancebox,
    frame_support::parameter_types,
    frontier_template_emulated_chain::FrontierTemplate,
    simple_template_emulated_chain::SimpleTemplate,
    sp_keyring::Sr25519Keyring,
    tanssi_emulated_integration_tests_common::accounts::{ALICE, BOB, RANDOM},
    westend_emulated_chain::Westend,
    xcm_emulator::{decl_test_networks, Chain},
};

decl_test_networks! {
    pub struct WestendMockNet {
        relay_chain = Westend,
        parachains = vec![
            Dancebox,
            FrontierTemplate,
            SimpleTemplate,
        ],
        bridge = ()
    },
}

parameter_types! {
    // Westend
    pub WestendSender: cumulus_primitives_core::relay_chain::AccountId = Sr25519Keyring::Alice.to_account_id();
    pub WestendReceiver: cumulus_primitives_core::relay_chain::AccountId = Sr25519Keyring::Bob.to_account_id();
    pub WestendEmptyReceiver: cumulus_primitives_core::relay_chain::AccountId = WestendRelay::account_id_of(RANDOM);

    // Dancebox
    pub DanceboxSender: dancebox_runtime::AccountId = dancebox_runtime::AccountId::from(ALICE);
    pub DanceboxReceiver: dancebox_runtime::AccountId = dancebox_runtime::AccountId::from(BOB);
    pub DanceboxEmptyReceiver: dancebox_runtime::AccountId = DanceboxPara::account_id_of(RANDOM);

    // SimpleTemplate
    pub SimpleTemplateSender: container_chain_template_simple_runtime::AccountId = Sr25519Keyring::Alice.to_account_id();
    pub SimpleTemplateReceiver: container_chain_template_simple_runtime::AccountId = Sr25519Keyring::Bob.to_account_id();
    pub SimpleTemplateEmptyReceiver: container_chain_template_simple_runtime::AccountId = SimpleTemplatePara::account_id_of(RANDOM);
}
