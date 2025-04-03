use {cumulus_primitives_core::relay_chain::AccountId, keyring::Sr25519Keyring};

pub const ALICE: [u8; 32] = [4u8; 32];
pub const BOB: [u8; 32] = [5u8; 32];
pub const RANDOM: &str = "Random//stash";

pub fn init_balances() -> Vec<AccountId> {
    Sr25519Keyring::well_known()
        .map(|k| k.to_account_id())
        .collect()
}
