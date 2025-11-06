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
    crate as dancebox_runtime,
    crate::{
        prod_or_fast, AccountId, DataPreserversConfig, MaintenanceModeConfig, MigrationsConfig,
        PolkadotXcmConfig, RegistrarConfig, ServicesPaymentConfig, SudoConfig,
    },
    alloc::{string::String, vec, vec::Vec},
    cumulus_primitives_core::ParaId,
    dp_container_chain_genesis_data::{
        json::container_chain_genesis_data_from_str, ContainerChainGenesisData,
    },
    frame_support::BoundedVec,
    nimbus_primitives::NimbusId,
    pallet_configuration::HostConfiguration,
    sp_core::crypto::get_public_from_string_or_panic,
    sp_core::sr25519,
    sp_keyring::Sr25519Keyring,
    sp_runtime::{traits::AccountIdConversion, Perbill},
};

pub fn local(
    para_id: ParaId,
    container_chains_spec_contents: Vec<String>,
    mock_container_chains: Vec<ParaId>,
    invulnerables: Vec<String>,
) -> serde_json::Value {
    testnet_genesis(
        // initial collators.
        invulnerables_from_seeds(invulnerables.iter()),
        account_ids(&[
            "Alice",
            "Bob",
            "Charlie",
            "Dave",
            "Eve",
            "Ferdie",
            "Alice//stash",
            "Bob//stash",
            "Charlie//stash",
            "Dave//stash",
            "Eve//stash",
            "Ferdie//stash",
        ]),
        para_id,
        Sr25519Keyring::Alice.to_account_id(),
        &container_chains_spec_contents,
        &mock_container_chains,
        pallet_configuration::GenesisConfig {
            config: HostConfiguration {
                max_collators: 100u32,
                min_orchestrator_collators: 2u32,
                max_orchestrator_collators: 5u32,
                collators_per_container: 2u32,
                full_rotation_period: prod_or_fast!(24u32, 5u32),
                collators_per_parathread: 1,
                parathreads_per_collator: 1,
                target_container_chain_fullness: Perbill::from_percent(80),
                max_parachain_cores_percentage: None,
                full_rotation_mode: Default::default(),
            },
            ..Default::default()
        },
    )
}

pub fn development(
    para_id: ParaId,
    container_chains_spec_contents: Vec<String>,
    mock_container_chains: Vec<ParaId>,
    invulnerables: Vec<String>,
) -> serde_json::Value {
    testnet_genesis(
        // initial collators.
        invulnerables_from_seeds(invulnerables.iter()),
        account_ids(&[
            "Alice",
            "Bob",
            "Charlie",
            "Dave",
            "Eve",
            "Ferdie",
            "Alice//stash",
            "Bob//stash",
            "Charlie//stash",
            "Dave//stash",
            "Eve//stash",
            "Ferdie//stash",
        ]),
        para_id,
        Sr25519Keyring::Alice.to_account_id(),
        &container_chains_spec_contents,
        &mock_container_chains,
        pallet_configuration::GenesisConfig {
            config: HostConfiguration {
                max_collators: 100u32,
                min_orchestrator_collators: 1u32,
                max_orchestrator_collators: 1u32,
                collators_per_container: 2u32,
                full_rotation_period: prod_or_fast!(24u32, 5u32),
                collators_per_parathread: 1,
                parathreads_per_collator: 1,
                target_container_chain_fullness: Perbill::from_percent(80),
                max_parachain_cores_percentage: None,
                full_rotation_mode: Default::default(),
            },
            ..Default::default()
        },
    )
}

fn testnet_genesis(
    invulnerables: Vec<(AccountId, NimbusId)>,
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
    root_key: AccountId,
    container_chains_spec_contents: &[String],
    mock_container_chains: &[ParaId],
    configuration: pallet_configuration::GenesisConfig<dancebox_runtime::Runtime>,
) -> serde_json::Value {
    let para_ids: Vec<_> = container_chains_spec_contents
        .iter()
        .map(|content_str| {
            container_chain_genesis_data_from_str(content_str).unwrap_or_else(|e| {
                panic!(
                    "Failed to build genesis data for container chain {:?}: {}",
                    content_str, e
                )
            })
        })
        .chain(
            mock_container_chains
                .iter()
                .map(|x| (*x, mock_container_chain_genesis_data(*x), vec![])),
        )
        .collect();
    // Assign 1000 block credits to all container chains registered in genesis
    // Assign 100 collator assignment credits to all container chains registered in genesis
    let para_id_credits: Vec<_> = para_ids
        .iter()
        .map(|(para_id, _genesis_data, _boot_nodes)| (*para_id, 1000, 100).into())
        .collect();
    let data_preservers_bootnodes: Vec<_> = para_ids
        .iter()
        .flat_map(|(para_id, _genesis_data, bootnodes)| {
            bootnodes.clone().into_iter().map(|bootnode| {
                (
                    *para_id,
                    AccountId::from([0u8; 32]),
                    bootnode,
                    tp_data_preservers_common::ProviderRequest::Free,
                    tp_data_preservers_common::AssignmentWitness::Free,
                )
            })
        })
        .collect();
    let para_ids: Vec<_> = para_ids
        .into_iter()
        .map(|(para_id, genesis_data, _boot_nodes)| (para_id, genesis_data, None))
        .collect();

    let accounts_with_ed = [
        dancebox_runtime::StakingAccount::get(),
        dancebox_runtime::ParachainBondAccount::get(),
        dancebox_runtime::PendingRewardsAccount::get(),
        dancebox_runtime::TreasuryId::get().into_account_truncating(),
    ];
    let g = dancebox_runtime::RuntimeGenesisConfig {
        system: Default::default(),
        balances: dancebox_runtime::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .chain(
                    accounts_with_ed
                        .iter()
                        .cloned()
                        .map(|k| (k, dancebox_runtime::EXISTENTIAL_DEPOSIT)),
                )
                .collect(),
            ..Default::default()
        },
        parachain_info: dancebox_runtime::ParachainInfoConfig {
            parachain_id: id,
            ..Default::default()
        },
        invulnerables: dancebox_runtime::InvulnerablesConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
        },
        session: dancebox_runtime::SessionConfig {
            keys: invulnerables
                .into_iter()
                .map(|(acc, aura)| {
                    (
                        acc.clone(),                 // account id
                        acc,                         // validator id
                        template_session_keys(aura), // session keys
                    )
                })
                .collect(),
            ..Default::default()
        },
        parachain_system: Default::default(),
        configuration,
        registrar: RegistrarConfig {
            para_ids,
            phantom: Default::default(),
        },
        services_payment: ServicesPaymentConfig { para_id_credits },
        sudo: SudoConfig {
            key: Some(root_key),
        },
        migrations: MigrationsConfig {
            ..Default::default()
        },
        maintenance_mode: MaintenanceModeConfig {
            start_in_maintenance_mode: false,
            ..Default::default()
        },
        // This should initialize it to whatever we have set in the pallet
        polkadot_xcm: PolkadotXcmConfig::default(),
        transaction_payment: Default::default(),
        tx_pause: Default::default(),
        treasury: Default::default(),
        data_preservers: DataPreserversConfig {
            bootnodes: data_preservers_bootnodes,
            ..Default::default()
        },
    };

    serde_json::to_value(g).unwrap()
}

// TODO: move to primitives and remove duplication from flashbox/dancebox
/// Helper function to turn a list of names into a list of `(AccountId, NimbusId)`
pub fn invulnerables_from_seeds<S: AsRef<str>, I: Iterator<Item = S>>(
    names: I,
) -> Vec<(AccountId, NimbusId)> {
    names
        .map(|name| {
            let name = name.as_ref();
            (
                get_public_from_string_or_panic::<sr25519::Public>(name).into(),
                get_collator_keys_from_seed(name),
            )
        })
        .collect()
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> NimbusId {
    get_public_from_string_or_panic::<NimbusId>(seed)
}

/// Helper function to turn a list of names into a list of `AccountId`
pub fn account_ids(names: &[&str]) -> Vec<AccountId> {
    names
        .iter()
        .map(|name| get_public_from_string_or_panic::<sr25519::Public>(name).into())
        .collect()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: NimbusId) -> dancebox_runtime::SessionKeys {
    dancebox_runtime::SessionKeys { nimbus: keys }
}

fn mock_container_chain_genesis_data(para_id: ParaId) -> ContainerChainGenesisData {
    let mut name_buf = alloc::string::String::new();
    let mut id_buf = alloc::string::String::new();

    use core::fmt::Write;
    write!(name_buf, "Container Chain {}", u32::from(para_id)).unwrap();
    write!(id_buf, "container-chain-{}", u32::from(para_id)).unwrap();

    ContainerChainGenesisData {
        storage: BoundedVec::try_from(alloc::vec![]).unwrap(),
        name: BoundedVec::try_from(name_buf.into_bytes()).unwrap(),
        id: BoundedVec::try_from(id_buf.into_bytes()).unwrap(),
        fork_id: None,
        extensions: BoundedVec::try_from(alloc::vec![]).unwrap(),
        properties: Default::default(),
    }
}
