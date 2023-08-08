use sp_core::{sr25519, storage::Storage, Pair, Public};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	BuildStorage, MultiSignature, Perbill,
};
use cumulus_primitives_core::relay_chain::AccountId;
use sp_consensus_babe::AuthorityId as BabeId;
use sc_consensus_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use cumulus_primitives_core::relay_chain::{AssignmentId, AuthorityDiscoveryId, ValidatorId};
use polkadot_service::chain_spec::get_authority_keys_from_seed_no_beefy;

type AccountPublic = <MultiSignature as Verify>::Signer;

/// Helper function to generate a crypto pair from seed
fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed.
fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}


pub mod accounts {
	use super::*;
	pub const ALICE: &str = "Alice";
	pub const BOB: &str = "Bob";
	pub const CHARLIE: &str = "Charlie";
	pub const DAVE: &str = "Dave";
	pub const EVE: &str = "Eve";
	pub const FERDIE: &str = "Ferdei";
	pub const ALICE_STASH: &str = "Alice//stash";
	pub const BOB_STASH: &str = "Bob//stash";
	pub const CHARLIE_STASH: &str = "Charlie//stash";
	pub const DAVE_STASH: &str = "Dave//stash";
	pub const EVE_STASH: &str = "Eve//stash";
	pub const FERDIE_STASH: &str = "Ferdie//stash";

	pub fn init_balances() -> Vec<AccountId> {
		vec![
			get_account_id_from_seed::<sr25519::Public>(ALICE),
			get_account_id_from_seed::<sr25519::Public>(BOB),
			get_account_id_from_seed::<sr25519::Public>(CHARLIE),
			get_account_id_from_seed::<sr25519::Public>(DAVE),
			get_account_id_from_seed::<sr25519::Public>(EVE),
			get_account_id_from_seed::<sr25519::Public>(FERDIE),
			get_account_id_from_seed::<sr25519::Public>(ALICE_STASH),
			get_account_id_from_seed::<sr25519::Public>(BOB_STASH),
			get_account_id_from_seed::<sr25519::Public>(CHARLIE_STASH),
			get_account_id_from_seed::<sr25519::Public>(DAVE_STASH),
			get_account_id_from_seed::<sr25519::Public>(EVE_STASH),
			get_account_id_from_seed::<sr25519::Public>(FERDIE_STASH),
		]
	}
}

pub mod validators {
	use super::*;

	pub fn initial_authorities() -> Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)> {
		vec![get_authority_keys_from_seed_no_beefy("Alice")]
	}
}

// Westend
pub mod westend {
	use super::*;
	use westend_runtime_constants::currency::UNITS as WND;
	use crate::{Balance, AccountId};
	use cumulus_primitives_core::relay_chain::BlockNumber;
	use polkadot_runtime_parachains::configuration::HostConfiguration;
	use sp_runtime::Perbill;
	pub const ED: Balance = westend_runtime_constants::currency::EXISTENTIAL_DEPOSIT;
	const ENDOWMENT: u128 = 1_000_000 * WND;
	const STASH: u128 = 100 * WND;

	pub fn get_host_config() -> HostConfiguration<BlockNumber> {
		HostConfiguration {
			max_upward_queue_count: 10,
			max_upward_queue_size: 51200,
			max_upward_message_size: 51200,
			max_upward_message_num_per_candidate: 10,
			max_downward_message_size: 51200,
			..Default::default()
		}
	}

	fn session_keys(
		babe: BabeId,
		grandpa: GrandpaId,
		im_online: ImOnlineId,
		para_validator: ValidatorId,
		para_assignment: AssignmentId,
		authority_discovery: AuthorityDiscoveryId,
	) -> westend_runtime::SessionKeys {
		westend_runtime::SessionKeys {
			babe,
			grandpa,
			im_online,
			para_validator,
			para_assignment,
			authority_discovery,
		}
	}

	pub fn genesis() -> Storage {
		let genesis_config = westend_runtime::GenesisConfig {
			system: westend_runtime::SystemConfig {
				code: westend_runtime::WASM_BINARY.unwrap().to_vec(),
				..Default::default()
			},
			balances: westend_runtime::BalancesConfig {
				balances: accounts::init_balances()
					.iter()
					.cloned()
					.map(|k| (k, ENDOWMENT))
					.collect(),
			},
			session: westend_runtime::SessionConfig {
				keys: validators::initial_authorities()
					.iter()
					.map(|x| {
						(
							x.0.clone(),
							x.0.clone(),
							westend::session_keys(
								x.2.clone(),
								x.3.clone(),
								x.4.clone(),
								x.5.clone(),
								x.6.clone(),
								x.7.clone(),
							),
						)
					})
					.collect::<Vec<_>>(),
			},
			staking: westend_runtime::StakingConfig {
				validator_count: validators::initial_authorities().len() as u32,
				minimum_validator_count: 1,
				stakers: validators::initial_authorities()
					.iter()
					.map(|x| {
						(x.0.clone(), x.1.clone(), STASH, westend_runtime::StakerStatus::Validator)
					})
					.collect(),
				invulnerables: validators::initial_authorities()
					.iter()
					.map(|x| x.0.clone())
					.collect(),
				force_era: pallet_staking::Forcing::ForceNone,
				slash_reward_fraction: Perbill::from_percent(10),
				..Default::default()
			},
			babe: westend_runtime::BabeConfig {
				authorities: Default::default(),
				epoch_config: Some(westend_runtime::BABE_GENESIS_EPOCH_CONFIG),
				..Default::default()
			},
			configuration: westend_runtime::ConfigurationConfig { config: get_host_config() },
			..Default::default()
		};

		genesis_config.build_storage().unwrap()
	}
}