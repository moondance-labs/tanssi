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

pub fn force_process_bridge<R, P>()
where
    R: RelayChain,
    P: Parachain<Network = R::Network>,
    R::Runtime: pallet_message_queue::Config,
{
    // Process MessageQueue on relay chain to consume the message we want to send to eth
    R::execute_with(|| {
        <pallet_message_queue::Pallet<R::Runtime>>::on_idle(
            SystemPallet::<R::Runtime>::block_number(),
            crate::MessageQueueServiceWeight::get(),
        );
    });

    // Execute empty block in parachain to trigger bridge message
    P::execute_with(|| {});
}

pub fn eth_bridge_sent_msgs() -> Vec<CommittedMessage> {
    ETH_BRIDGE_SENT_MSGS.with(|q| (*q.borrow()).clone())
}

type AccountPublic = <MultiSignature as Verify>::Signer;

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed_no_beefy(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    BabeId,
    GrandpaId,
    ValidatorId,
    AssignmentId,
    AuthorityDiscoveryId,
) {
    (
        get_public_from_string_or_panic::<sr25519::Public>(&format!("{}//stash", seed)).into(),
        get_public_from_string_or_panic::<sr25519::Public>(seed).into(),
        get_public_from_string_or_panic::<BabeId>(seed),
        get_public_from_string_or_panic::<GrandpaId>(seed),
        get_public_from_string_or_panic::<ValidatorId>(seed),
        get_public_from_string_or_panic::<AssignmentId>(seed),
        get_public_from_string_or_panic::<AuthorityDiscoveryId>(seed),
    )
}

pub mod accounts {
    use super::*;
    pub const ALICE: &str = "Alice";
    pub const BOB: &str = "Bob";
    pub const CHARLIE: &str = "Charlie";
    pub const DAVE: &str = "Dave";
    pub const EVE: &str = "Eve";
    pub const FERDIE: &str = "Ferdei";
    pub const RANDOM: &str = "Random//stash";

    pub fn init_balances() -> Vec<AccountId> {
        Sr25519Keyring::well_known()
            .map(|k| k.to_account_id())
            .collect()
    }
}

pub mod validators {
    use super::*;

    pub fn initial_authorities() -> Vec<(
        AccountId,
        AccountId,
        BabeId,
        GrandpaId,
        ValidatorId,
        AssignmentId,
        AuthorityDiscoveryId,
    )> {
        vec![get_authority_keys_from_seed_no_beefy("Alice")]
    }
}
