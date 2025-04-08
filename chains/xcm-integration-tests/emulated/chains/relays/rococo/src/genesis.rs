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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

use {
    cumulus_primitives_core::relay_chain::BlockNumber,
    cumulus_primitives_core::relay_chain::{AssignmentId, AuthorityDiscoveryId, ValidatorId},
    emulated_integration_tests_common::build_genesis_storage,
    polkadot_parachain_primitives::primitives::ValidationCode,
    rococo_runtime_constants::currency::UNITS as ROC,
    runtime_parachains::{
        configuration::HostConfiguration,
        paras::{ParaGenesisArgs, ParaKind},
    },
    sc_consensus_grandpa::AuthorityId as GrandpaId,
    sp_consensus_babe::AuthorityId as BabeId,
    sp_consensus_beefy::ecdsa_crypto::AuthorityId as BeefyId,
    sp_consensus_beefy::test_utils::Keyring,
    sp_core::storage::Storage,
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
            balances: tanssi_emulated_integration_tests_common::accounts::init_balances()
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .collect(),
        },
        session: rococo_runtime::SessionConfig {
            keys: tanssi_emulated_integration_tests_common::validators::initial_authorities()
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(
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
            config: get_host_config(),
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
