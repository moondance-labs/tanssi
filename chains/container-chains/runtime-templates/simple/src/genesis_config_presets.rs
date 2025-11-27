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
    crate as container_chain_template_simple_runtime,
    crate::{AccountId, MaintenanceModeConfig, MigrationsConfig, PolkadotXcmConfig},
    alloc::vec::Vec,
    cumulus_primitives_core::ParaId,
    sp_keyring::Sr25519Keyring,
};

/// Orcherstrator's parachain id
pub const ORCHESTRATOR: ParaId = ParaId::new(1000);

pub fn local(
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
    root_key: AccountId,
) -> serde_json::Value {
    testnet_genesis(endowed_accounts, id, root_key)
}

pub fn development(
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
    root_key: AccountId,
) -> serde_json::Value {
    testnet_genesis(endowed_accounts, id, root_key)
}

fn testnet_genesis(
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
    root_key: AccountId,
) -> serde_json::Value {
    let g = container_chain_template_simple_runtime::RuntimeGenesisConfig {
        balances: container_chain_template_simple_runtime::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
            ..Default::default()
        },
        parachain_info: container_chain_template_simple_runtime::ParachainInfoConfig {
            parachain_id: id,
            ..Default::default()
        },
        parachain_system: Default::default(),
        sudo: container_chain_template_simple_runtime::SudoConfig {
            key: Some(root_key),
        },
        authorities_noting: container_chain_template_simple_runtime::AuthoritiesNotingConfig {
            orchestrator_para_id: ORCHESTRATOR,
            ..Default::default()
        },
        migrations: MigrationsConfig::default(),
        maintenance_mode: MaintenanceModeConfig {
            start_in_maintenance_mode: false,
            ..Default::default()
        },
        // This should initialize it to whatever we have set in the pallet
        polkadot_xcm: PolkadotXcmConfig::default(),
        transaction_payment: Default::default(),
        tx_pause: Default::default(),
        system: Default::default(),
    };

    serde_json::to_value(g).unwrap()
}

/// Get pre-funded accounts
pub fn pre_funded_accounts() -> Vec<AccountId> {
    Sr25519Keyring::well_known()
        .map(|k| k.to_account_id())
        .collect()
}
