use {
    babe_primitives::AuthorityId as BabeId,
    beefy_primitives::ecdsa_crypto::AuthorityId as BeefyId,
    cumulus_primitives_core::relay_chain::{
        AccountId, AssignmentId, AuthorityDiscoveryId, ValidatorId,
    },
    emulated_integration_tests_common::build_genesis_storage,
    keyring::Sr25519Keyring,
    sc_consensus_grandpa::AuthorityId as GrandpaId,
    sp_core::{crypto::get_public_from_string_or_panic, sr25519, storage::Storage},
    sp_runtime::{traits::Verify, MultiSignature},
};

pub const ALICE: [u8; 32] = [4u8; 32];
pub const BOB: [u8; 32] = [5u8; 32];
pub const RANDOM: &str = "Random//stash";

pub fn init_balances() -> Vec<AccountId> {
    Sr25519Keyring::well_known()
        .map(|k| k.to_account_id())
        .collect()
}
