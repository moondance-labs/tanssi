// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>
use {
    cumulus_primitives_core::relay_chain::{
        AccountId, AssignmentId, AuthorityDiscoveryId, ValidatorId,
    },
    emulated_integration_tests_common::build_genesis_storage,
    sc_consensus_grandpa::AuthorityId as GrandpaId,
    sp_consensus_babe::AuthorityId as BabeId,
    sp_consensus_beefy::ecdsa_crypto::AuthorityId as BeefyId,
    sp_core::crypto::get_public_from_string_or_panic,
    sp_core::{sr25519, storage::Storage},
    sp_keyring::Sr25519Keyring,
};

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

// Westend
pub mod westend {
    use sp_consensus_beefy::test_utils::Keyring;
    use {
        super::*, cumulus_primitives_core::relay_chain::BlockNumber,
        polkadot_runtime_parachains::configuration::HostConfiguration, sp_runtime::Perbill,
        westend_runtime_constants::currency::UNITS as WND,
    };

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
        para_validator: ValidatorId,
        para_assignment: AssignmentId,
        authority_discovery: AuthorityDiscoveryId,
        beefy: BeefyId,
    ) -> westend_runtime::SessionKeys {
        westend_runtime::SessionKeys {
            babe,
            grandpa,
            para_validator,
            para_assignment,
            authority_discovery,
            beefy,
        }
    }

    pub fn genesis() -> Storage {
        let genesis_config = westend_runtime::RuntimeGenesisConfig {
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
                                BeefyId::from(Keyring::<BeefyId>::Alice.public()),
                            ),
                        )
                    })
                    .collect::<Vec<_>>(),
                ..Default::default()
            },
            staking: westend_runtime::StakingConfig {
                validator_count: validators::initial_authorities().len() as u32,
                minimum_validator_count: 1,
                stakers: validators::initial_authorities()
                    .iter()
                    .map(|x| {
                        (
                            x.0.clone(),
                            x.1.clone(),
                            STASH,
                            polkadot_runtime_common::StakerStatus::Validator,
                        )
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
                epoch_config: westend_runtime::BABE_GENESIS_EPOCH_CONFIG,
                ..Default::default()
            },
            configuration: westend_runtime::ConfigurationConfig {
                config: get_host_config(),
            },
            ..Default::default()
        };
        build_genesis_storage(&genesis_config, westend_runtime::WASM_BINARY.unwrap())
    }
}

// Rococo
pub mod rococo {
    use sp_consensus_beefy::test_utils::Keyring;
    use {
        super::*,
        cumulus_primitives_core::relay_chain::BlockNumber,
        polkadot_parachain_primitives::primitives::ValidationCode,
        polkadot_runtime_parachains::{
            configuration::HostConfiguration,
            paras::{ParaGenesisArgs, ParaKind},
        },
        rococo_runtime_constants::currency::UNITS as ROC,
    };
    const ENDOWMENT: u128 = 1_000_000 * ROC;

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
        para_validator: ValidatorId,
        para_assignment: AssignmentId,
        authority_discovery: AuthorityDiscoveryId,
        beefy: BeefyId,
    ) -> rococo_runtime::SessionKeys {
        rococo_runtime::SessionKeys {
            babe,
            grandpa,
            para_validator,
            para_assignment,
            authority_discovery,
            beefy,
        }
    }

    pub fn genesis() -> Storage {
        let genesis_config = rococo_runtime::RuntimeGenesisConfig {
            balances: rococo_runtime::BalancesConfig {
                balances: accounts::init_balances()
                    .iter()
                    .cloned()
                    .map(|k| (k, crate::tests::common::xcm::constants::rococo::ENDOWMENT))
                    .collect(),
            },
            session: rococo_runtime::SessionConfig {
                keys: validators::initial_authorities()
                    .iter()
                    .map(|x| {
                        (
                            x.0.clone(),
                            x.0.clone(),
                            crate::tests::common::xcm::constants::rococo::session_keys(
                                x.2.clone(),
                                x.3.clone(),
                                x.4.clone(),
                                x.5.clone(),
                                x.6.clone(),
                                BeefyId::from(Keyring::<BeefyId>::Alice.public()),
                            ),
                        )
                    })
                    .collect::<Vec<_>>(),
                ..Default::default()
            },
            babe: rococo_runtime::BabeConfig {
                authorities: Default::default(),
                epoch_config: rococo_runtime::BABE_GENESIS_EPOCH_CONFIG,
                ..Default::default()
            },
            configuration: rococo_runtime::ConfigurationConfig {
                config: crate::tests::common::xcm::constants::rococo::get_host_config(),
            },
            paras: rococo_runtime::ParasConfig {
                _config: Default::default(),
                paras: vec![(
                    3333.into(),
                    ParaGenesisArgs {
                        genesis_head: Default::default(),
                        validation_code: ValidationCode(vec![1, 1, 2, 3, 4]),
                        para_kind: ParaKind::Parathread,
                    },
                )],
            },
            ..Default::default()
        };
        build_genesis_storage(&genesis_config, rococo_runtime::WASM_BINARY.unwrap())
    }
}

// Frontier template
pub mod frontier_template {
    use {
        container_chain_template_frontier_runtime::AccountId,
        emulated_integration_tests_common::build_genesis_storage, hex_literal::hex,
    };
    pub const PARA_ID: u32 = 2001;
    pub const ORCHESTRATOR: u32 = 2000;

    pub fn genesis() -> sp_core::storage::Storage {
        let genesis_config = container_chain_template_frontier_runtime::RuntimeGenesisConfig {
            system: Default::default(),
            balances: container_chain_template_frontier_runtime::BalancesConfig {
                balances: pre_funded_accounts()
                    .iter()
                    .cloned()
                    .map(|k| (k, 1 << 80))
                    .collect(),
            },
            parachain_info: container_chain_template_frontier_runtime::ParachainInfoConfig {
                parachain_id: PARA_ID.into(),
                ..Default::default()
            },
            // EVM compatibility
            // We should change this to something different than Moonbeam
            // For now moonwall is very tailored for moonbeam so we need it for tests
            evm_chain_id: container_chain_template_frontier_runtime::EVMChainIdConfig {
                chain_id: 1281,
                ..Default::default()
            },
            sudo: container_chain_template_frontier_runtime::SudoConfig {
                key: Some(pre_funded_accounts()[0]),
            },
            authorities_noting:
                container_chain_template_frontier_runtime::AuthoritiesNotingConfig {
                    orchestrator_para_id: ORCHESTRATOR.into(),
                    ..Default::default()
                },
            ..Default::default()
        };

        build_genesis_storage(
            &genesis_config,
            container_chain_template_frontier_runtime::WASM_BINARY.unwrap(),
        )
    }
    /// Get pre-funded accounts
    pub fn pre_funded_accounts() -> Vec<AccountId> {
        // These addresses are derived from Substrate's canonical mnemonic:
        // bottom drive obey lake curtain smoke basket hold race lonely fit walk
        vec![
            AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")), // Alith
            AccountId::from(hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")), // Baltathar
            AccountId::from(hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc")), // Charleth
            AccountId::from(hex!("773539d4Ac0e786233D90A233654ccEE26a613D9")), // Dorothy
        ]
    }
}

// Simple template
pub mod simple_template {
    use {super::*, container_chain_template_simple_runtime::UNIT as DEV};
    pub const PARA_ID: u32 = 2002;
    pub const ORCHESTRATOR: u32 = 2000;
    const ENDOWMENT: u128 = 1_000_000 * DEV;

    pub fn genesis() -> sp_core::storage::Storage {
        let genesis_config = container_chain_template_simple_runtime::RuntimeGenesisConfig {
            balances: container_chain_template_simple_runtime::BalancesConfig {
                balances: accounts::init_balances()
                    .iter()
                    .cloned()
                    .map(|k| (k, ENDOWMENT))
                    .collect(),
            },
            parachain_info: container_chain_template_simple_runtime::ParachainInfoConfig {
                parachain_id: PARA_ID.into(),
                ..Default::default()
            },
            sudo: container_chain_template_simple_runtime::SudoConfig {
                key: Some(accounts::init_balances()[0].clone()),
            },
            authorities_noting: container_chain_template_simple_runtime::AuthoritiesNotingConfig {
                orchestrator_para_id: ORCHESTRATOR.into(),
                ..Default::default()
            },
            ..Default::default()
        };
        build_genesis_storage(
            &genesis_config,
            container_chain_template_simple_runtime::WASM_BINARY.unwrap(),
        )
    }
}
