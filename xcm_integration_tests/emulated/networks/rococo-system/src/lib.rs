pub use xcm_emulator::{bx, TestExt};
use {
    dancebox_rococo_emulated_chain::DanceboxRococo,
    frame_support::parameter_types,
    frontier_template_emulated_chain::FrontierTemplate,
    rococo_emulated_chain::Rococo,
    simple_template_emulated_chain::SimpleTemplate,
    sp_keyring::Sr25519Keyring,
    tanssi_emulated_integration_tests_common,
    tanssi_emulated_integration_tests_common::accounts::RANDOM,
    xcm_emulator::{decl_test_networks, Chain},
};

decl_test_networks! {
    pub struct RococoMockNet {
        relay_chain = Rococo,
        parachains = vec![
            DanceboxRococo,
            FrontierTemplate,
            SimpleTemplate,
        ],
        bridge = ()
    }
}

parameter_types! {
    // Rococo
    pub RococoSender: cumulus_primitives_core::relay_chain::AccountId = Sr25519Keyring::Alice.to_account_id();
    pub RococoReceiver: cumulus_primitives_core::relay_chain::AccountId = Sr25519Keyring::Bob.to_account_id();
    pub RococoEmptyReceiver: cumulus_primitives_core::relay_chain::AccountId = RococoRelay::account_id_of(RANDOM);
}
