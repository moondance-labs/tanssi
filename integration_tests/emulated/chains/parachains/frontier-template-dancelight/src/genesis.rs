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
