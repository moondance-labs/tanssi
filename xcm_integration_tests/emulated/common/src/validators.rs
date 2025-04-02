use {
    crate::get_authority_keys_from_seed_no_beefy,
    babe_primitives::AuthorityId as BabeId,
    cumulus_primitives_core::relay_chain::{
        AccountId, AssignmentId, AuthorityDiscoveryId, ValidatorId,
    },
    sc_consensus_grandpa::AuthorityId as GrandpaId,
};

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
