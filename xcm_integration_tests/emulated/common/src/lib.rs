use {
    babe_primitives::AuthorityId as BabeId,
    cumulus_primitives_core::relay_chain::{
        AccountId, AssignmentId, AuthorityDiscoveryId, ValidatorId,
    },
    frame_support::traits::OnIdle,
    frame_system::Pallet as SystemPallet,
    sc_consensus_grandpa::AuthorityId as GrandpaId,
    sp_core::{crypto::get_public_from_string_or_panic, sr25519},
    sp_runtime::{traits::Verify, MultiSignature},
    xcm_emulator::{Parachain, RelayChain},
};

pub mod accounts;
pub mod impls;
pub mod validators;

pub use xcm_emulator::{bx, TestExt};

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
