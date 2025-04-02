use {
    babe_primitives::AuthorityId as BabeId,
    beefy_primitives::ecdsa_crypto::AuthorityId as BeefyId,
    cumulus_primitives_core::relay_chain::{
        AccountId, AssignmentId, AuthorityDiscoveryId, ValidatorId,
    },
    dancelight_runtime_constants::currency::UNITS as UNIT,
    emulated_integration_tests_common::build_genesis_storage,
    emulated_integration_tests_common::xcm_emulator::decl_test_parachains,
    frame_support::traits::OnIdle,
    frame_system::Pallet as SystemPallet,
    keyring::Sr25519Keyring,
    sc_consensus_grandpa::AuthorityId as GrandpaId,
    snowbridge_pallet_outbound_queue::CommittedMessage,
    sp_core::{crypto::get_public_from_string_or_panic, sr25519, storage::Storage},
    sp_runtime::{traits::Verify, MultiSignature},
    sp_std::cell::RefCell,
    xcm_emulator::{Parachain, RelayChain},
};

pub mod accounts;
pub mod impls;
pub mod validators;

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
            dancelight_runtime::MessageQueueServiceWeight::get(),
        );
    });

    // Execute empty block in parachain to trigger bridge message
    P::execute_with(|| {});
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
