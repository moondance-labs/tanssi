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

//! Helper functions to convert from `ContainerChainGenesisData` to JSON values and back

use {
    crate::{ContainerChainGenesisData, ContainerChainGenesisDataItem, Get, Properties},
    cumulus_primitives_core::ParaId,
};

pub type ContainerChainGenesisDataResult<T> =
    Result<(ParaId, ContainerChainGenesisData<T>, Vec<Vec<u8>>), String>;

/// Reads a raw ChainSpec file stored in `path`, and returns its `ParaId` and
/// a `ContainerChainGenesisData` that can be used to recreate the ChainSpec later.
pub fn container_chain_genesis_data_from_path<MaxLengthTokenSymbol: Get<u32>>(
    path: &str,
) -> ContainerChainGenesisDataResult<MaxLengthTokenSymbol> {
    // Read raw chainspec file
    let raw_chainspec_str = std::fs::read_to_string(path)
        .map_err(|_e| format!("ChainSpec for container chain not found at {:?}", path))?;

    container_chain_genesis_data_from_str(&raw_chainspec_str)
}

pub fn container_chain_genesis_data_from_str<MaxLengthTokenSymbol: Get<u32>>(
    raw_chainspec_str: &str,
) -> ContainerChainGenesisDataResult<MaxLengthTokenSymbol> {
    let raw_chainspec_json: serde_json::Value =
        serde_json::from_str(raw_chainspec_str).map_err(|e| e.to_string())?;

    container_chain_genesis_data_from_json(&raw_chainspec_json)
}

pub fn container_chain_genesis_data_from_json<MaxLengthTokenSymbol: Get<u32>>(
    raw_chainspec_json: &serde_json::Value,
) -> ContainerChainGenesisDataResult<MaxLengthTokenSymbol> {
    // TODO: we are manually parsing a json file here, maybe we can leverage the existing
    // chainspec deserialization code.
    // TODO: this bound checking may panic, but that shouldn't be too dangerous because this
    // function is only used by the `build-spec` command.
    let para_id: u32 = u32::try_from(raw_chainspec_json["para_id"].as_u64().unwrap()).unwrap();
    let name: String = raw_chainspec_json["name"].as_str().unwrap().to_owned();
    let id: String = raw_chainspec_json["id"].as_str().unwrap().to_owned();
    let fork_id: Option<String> = raw_chainspec_json["fork_id"].as_str().map(|x| x.to_owned());
    let genesis_raw_top_json = &raw_chainspec_json["genesis"]["raw"]["top"];
    let storage = storage_from_chainspec_json(genesis_raw_top_json)?;
    let properties_json = &raw_chainspec_json["properties"];
    let properties = properties_from_chainspec_json(properties_json);
    let boot_nodes: Vec<serde_json::Value> =
        raw_chainspec_json["bootNodes"].as_array().unwrap().clone();
    let boot_nodes: Vec<Vec<u8>> = boot_nodes
        .into_iter()
        .map(|x| {
            let bytes = x.as_str().unwrap().as_bytes();
            bytes.to_vec()
        })
        .collect();

    Ok((
        para_id.into(),
        ContainerChainGenesisData {
            storage,
            name: name.into(),
            id: id.into(),
            fork_id: fork_id.map(|x| x.into()),
            extensions: vec![],
            properties,
        },
        boot_nodes,
    ))
}

pub fn storage_from_chainspec_json(
    genesis_raw_top_json: &serde_json::Value,
) -> Result<Vec<ContainerChainGenesisDataItem>, String> {
    let genesis_data_map = genesis_raw_top_json
        .as_object()
        .ok_or("genesis.raw.top is not an object".to_string())?;

    let mut genesis_data_vec = Vec::with_capacity(genesis_data_map.len());

    for (key, value) in genesis_data_map {
        let key_hex = key
            .strip_prefix("0x")
            .ok_or("key does not start with 0x".to_string())?;
        let value = value.as_str().ok_or("value is not a string".to_string())?;
        let value_hex = value
            .strip_prefix("0x")
            .ok_or("value does not start with 0x".to_string())?;

        let key_bytes = hex::decode(key_hex).map_err(|e| e.to_string())?;
        let value_bytes = hex::decode(value_hex).map_err(|e| e.to_string())?;

        genesis_data_vec.push((key_bytes, value_bytes).into());
    }

    // This sort is just to make the UI a bit easier to follow,
    // sorting the storage is not a requirement.
    // Maybe it is not even needed if the `genesis_data_map` iterator is ordered.
    // Unstable sort is fine because this was created by iterating over a map,
    // so it won't have two equal keys
    genesis_data_vec.sort_unstable();

    Ok(genesis_data_vec)
}

/// Read `TokenMetadata` from a JSON value. The value is expected to be a map.
/// In case of error, the default `TokenMetadata` is returned.
pub fn properties_from_chainspec_json<MaxLengthTokenSymbol: Get<u32>>(
    properties_json: &serde_json::Value,
) -> Properties<MaxLengthTokenSymbol> {
    let mut properties: Properties<MaxLengthTokenSymbol> = Properties::default();
    if let Some(x) = properties_json
        .get("ss58Format")
        .and_then(|x| u32::try_from(x.as_u64()?).ok())
        .or_else(|| {
            log::warn!(
                "Failed to read properties.ss58Format from container chain chain spec, using default value instead. Invalid value was: {:?}",
                properties_json.get("ss58Format")
            );

            None
        })
    {
        properties.token_metadata.ss58_format = x;
    }
    if let Some(x) = properties_json
        .get("tokenDecimals")
        .and_then(|x: &serde_json::Value| u32::try_from(x.as_u64()?).ok()).or_else(|| {
            log::warn!(
                "Failed to read properties.tokenDecimals from container chain chain spec, using default value instead. Invalid value was: {:?}",
                properties_json.get("tokenDecimals")
            );

            None
        })
    {
        properties.token_metadata.token_decimals = x;
    }
    if let Some(x) = properties_json.get("tokenSymbol").and_then(|x| {
        let xs = x.as_str()?;
        let xv: Vec<u8> = xs.to_string().into();

        xv.try_into().ok()
    }).or_else(|| {
        log::warn!(
            "Failed to read properties.tokenSymbol from container chain chain spec, using default value instead. Invalid value was: {:?}",
            properties_json.get("tokenSymbol")
        );

        None
    }) {
        properties.token_metadata.token_symbol = x;
    }
    if let Some(x) = properties_json.get("isEthereum").and_then(|x| {
        x.as_bool()
    }).or_else(|| {
        log::warn!(
            "Failed to read properties.isEthereum from container chain chain spec, using default value instead. Invalid value was: {:?}",
            properties_json.get("isEthereum")
        );

        None
    }) {
        properties.is_ethereum = x;
    }

    properties
}

pub fn properties_to_map<MaxLengthTokenSymbol: Get<u32>>(
    properties: &Properties<MaxLengthTokenSymbol>,
) -> Result<serde_json::Map<String, serde_json::Value>, String> {
    // TODO: we can just derive Serialize for genesis_data.properties instead of this hack,
    // just ensure that the field names match. And "tokenSymbol" must be a string, in the struct
    // it is defined as a Vec<u8>.
    let properties = vec![
        (
            "ss58Format",
            serde_json::Value::from(properties.token_metadata.ss58_format),
        ),
        (
            "tokenDecimals",
            serde_json::Value::from(properties.token_metadata.token_decimals),
        ),
        (
            "tokenSymbol",
            serde_json::Value::from(
                String::from_utf8(properties.token_metadata.token_symbol.to_vec())
                    .map_err(|e| format!("tokenSymbol is not valid UTF8: {}", e))?,
            ),
        ),
        (
            "isEthereum",
            serde_json::Value::from(properties.is_ethereum),
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect();

    Ok(properties)
}

#[cfg(test)]
mod tests {
    use sp_core::ConstU32;

    use super::*;

    fn expected_container_chain_genesis_data() -> ContainerChainGenesisData<ConstU32<255>> {
        let para_id = 2000;

        ContainerChainGenesisData {
            storage: vec![(b"code".to_vec(), vec![1, 2, 3, 4, 5, 6]).into()],
            name: format!("Container Chain {}", para_id).into(),
            id: format!("container-chain-{}", para_id).into(),
            fork_id: None,
            extensions: vec![],
            properties: Default::default(),
        }
    }

    fn expected_string() -> &'static str {
        // TODO: this should be improved:
        // * name should be a string "Container Chain 2000"
        // * id should be a string
        // * token_symbol should be a string
        // * storage should be a map:
        //   "storage": { "0x636f6465": "0x010203040506" }
        r#"{
            "storage": [
              {
                "key": "0x636f6465",
                "value": "0x010203040506"
              }
            ],
            "name": "0x436f6e7461696e657220436861696e2032303030",
            "id": "0x636f6e7461696e65722d636861696e2d32303030",
            "fork_id": null,
            "extensions": "0x",
            "properties": {
              "token_metadata": {
                "token_symbol": [
                  85,
                  78,
                  73,
                  84
                ],
                "ss58_format": 42,
                "token_decimals": 12
              },
              "is_ethereum": false
            }
        }"#
    }

    #[test]
    fn test_serde_serialize() {
        let x = expected_container_chain_genesis_data();
        let xv = serde_json::to_value(x).unwrap();
        // Regenerate expected string using
        //println!("{}", serde_json::to_string_pretty(&x).unwrap());
        let expected = expected_string();
        let ev: serde_json::Value = serde_json::from_str(expected).unwrap();
        assert_eq!(xv, ev);
    }

    #[test]
    fn test_serde_deserialize() {
        let expected = expected_container_chain_genesis_data();
        let s = expected_string();
        let x: ContainerChainGenesisData<ConstU32<255>> = serde_json::from_str(s).unwrap();
        assert_eq!(x, expected);
    }
}
