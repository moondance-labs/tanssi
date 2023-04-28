//! Helper functions to convert from `ContainerChainGenesisData` to JSON values and back

use crate::ContainerChainGenesisData;
use crate::ContainerChainGenesisDataItem;
use crate::TokenMetadata;
use cumulus_primitives_core::ParaId;

/// Reads a raw ChainSpec file stored in `path`, and returns its `ParaId` and
/// a `ContainerChainGenesisData` that can be used to recreate the ChainSpec later.
pub fn container_chain_genesis_data_from_path(
    path: &str,
) -> Result<(ParaId, ContainerChainGenesisData), String> {
    // Read raw chainspec file
    let raw_chainspec_str = std::fs::read_to_string(path)
        .map_err(|_e| format!("ChainSpec for container chain not found at {:?}", path))?;

    container_chain_genesis_data_from_str(&raw_chainspec_str)
}

pub fn container_chain_genesis_data_from_str(
    raw_chainspec_str: &str,
) -> Result<(ParaId, ContainerChainGenesisData), String> {
    let raw_chainspec_json: serde_json::Value =
        serde_json::from_str(&raw_chainspec_str).map_err(|e| e.to_string())?;

    container_chain_genesis_data_from_json(&raw_chainspec_json)
}

pub fn container_chain_genesis_data_from_json(
    raw_chainspec_json: &serde_json::Value,
) -> Result<(ParaId, ContainerChainGenesisData), String> {
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
    let properties = properties_from_chainspec_json(&properties_json);

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
    ))
}

pub fn storage_from_chainspec_json(
    genesis_raw_top_json: &serde_json::Value,
) -> Result<Vec<ContainerChainGenesisDataItem>, String> {
    let genesis_data_map = genesis_raw_top_json
        .as_object()
        .ok_or(format!("genesis.raw.top is not an object"))?;

    let mut genesis_data_vec = Vec::with_capacity(genesis_data_map.len());

    for (key, value) in genesis_data_map {
        let key_hex = key
            .strip_prefix("0x")
            .ok_or(format!("key does not start with 0x"))?;
        let value = value.as_str().ok_or(format!("value is not a string"))?;
        let value_hex = value
            .strip_prefix("0x")
            .ok_or(format!("value does not start with 0x"))?;

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
pub fn properties_from_chainspec_json(properties_json: &serde_json::Value) -> TokenMetadata {
    let mut properties = TokenMetadata::default();
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
        properties.ss58_format = x;
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
        properties.token_decimals = x;
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
        properties.token_symbol = x;
    }

    properties
}

pub fn properties_to_map(
    properties: &TokenMetadata,
) -> Result<serde_json::Map<String, serde_json::Value>, String> {
    // TODO: we can just derive Serialize for genesis_data.properties instead of this hack,
    // just ensure that the field names match. And "tokenSymbol" must be a string, in the struct
    // it is defined as a Vec<u8>.
    let properties = vec![
        (
            "ss58Format",
            serde_json::Value::from(properties.ss58_format),
        ),
        (
            "tokenDecimals",
            serde_json::Value::from(properties.token_decimals),
        ),
        (
            "tokenSymbol",
            serde_json::Value::from(
                String::from_utf8(properties.token_symbol.to_vec())
                    .map_err(|e| format!("tokenSymbol is not valid UTF8: {}", e))?,
            ),
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect();

    Ok(properties)
}
