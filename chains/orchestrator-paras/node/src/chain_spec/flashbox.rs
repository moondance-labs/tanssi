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
    crate::chain_spec::Extensions,
    cumulus_primitives_core::ParaId,
    flashbox_runtime::genesis_config_presets::{development, local},
    sc_service::ChainType,
};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;

pub fn development_config(
    para_id: ParaId,
    container_chains: Vec<String>,
    mock_container_chains: Vec<ParaId>,
    invulnerables: Vec<String>,
) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "FLASH".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), false.into());

    // we do conversion from chain_spec file to the string content here because
    // local accepts contents vector
    let container_chains_spec_contents: Vec<_> = container_chains
        .iter()
        .map(|path| {
            std::fs::read_to_string(path)
                .map_err(|_e| format!("ChainSpec for container chain not found at {:?}", path))
                .unwrap()
        })
        .collect();

    ChainSpec::builder(
        flashbox_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: para_id.into(),
        },
    )
    .with_name("Flashbox Development Testnet")
    .with_id("flashbox_dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config(development(
        para_id,
        container_chains_spec_contents,
        mock_container_chains,
        invulnerables,
    ))
    .with_properties(properties)
    .build()
}

pub fn local_flashbox_config(
    para_id: ParaId,
    container_chains: Vec<String>,
    mock_container_chains: Vec<ParaId>,
    invulnerables: Vec<String>,
) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "FLASH".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), false.into());

    // we do conversion from chain_spec file to the string content here because
    // local accepts contents vector
    let container_chains_spec_contents: Vec<_> = container_chains
        .iter()
        .map(|path| {
            std::fs::read_to_string(path)
                .map_err(|_e| format!("ChainSpec for container chain not found at {:?}", path))
                .unwrap()
        })
        .collect();

    ChainSpec::builder(
        flashbox_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: para_id.into(),
        },
    )
    .with_name("Flashbox Local Testnet")
    .with_id("flashbox_local")
    .with_chain_type(ChainType::Local)
    .with_genesis_config(local(
        para_id,
        container_chains_spec_contents,
        mock_container_chains,
        invulnerables,
    ))
    .with_properties(properties)
    .with_protocol_id("orchestrator")
    .build()
}
