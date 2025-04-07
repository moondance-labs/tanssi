use emulated_integration_tests_common::build_genesis_storage;
use pallet_configuration::HostConfiguration;
use sp_core::storage::Storage;
use sp_core::Pair;
use {
    cumulus_primitives_core::Junctions::X1,
    nimbus_primitives::NimbusId,
    sp_keyring::Sr25519Keyring,
    xcm::prelude::*,
    xcm_builder::{ParentIsPreset, SiblingParachainConvertsVia},
    xcm_executor::traits::ConvertLocation,
};

// TODO: Move to common
pub fn get_aura_id_from_seed(seed: &str) -> NimbusId {
    sp_core::sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
        .into()
}

pub fn genesis() -> Storage {
    let genesis_config = dancebox_runtime::RuntimeGenesisConfig {
        balances: dancebox_runtime::BalancesConfig {
            balances: vec![
                // Alice gets 10k extra tokens for her mapping deposit
                (
                    Sr25519Keyring::Alice.to_account_id(),
                    210_000 * dancebox_runtime::UNIT,
                ),
                (
                    Sr25519Keyring::Bob.to_account_id(),
                    100_000 * dancebox_runtime::UNIT,
                ),
                // Give some balance to the relay chain account
                (
                    ParentIsPreset::<dancebox_runtime::AccountId>::convert_location(
                        &Location::parent(),
                    )
                    .unwrap(),
                    100_000 * dancebox_runtime::UNIT,
                ),
                // And to sovereigns
                (
                    SiblingParachainConvertsVia::<
                        polkadot_parachain_primitives::primitives::Sibling,
                        dancebox_runtime::AccountId,
                    >::convert_location(&Location {
                        parents: 1,
                        interior: X1([Parachain(2001u32)].into()),
                    })
                    .unwrap(),
                    100_000 * dancebox_runtime::UNIT,
                ),
                (
                    SiblingParachainConvertsVia::<
                        polkadot_parachain_primitives::primitives::Sibling,
                        dancebox_runtime::AccountId,
                    >::convert_location(&Location {
                        parents: 1,
                        interior: X1([Parachain(2002u32)].into()),
                    })
                    .unwrap(),
                    100_000 * dancebox_runtime::UNIT,
                ),
            ],
        },
        configuration: dancebox_runtime::ConfigurationConfig {
            config: HostConfiguration {
                max_collators: 100,
                min_orchestrator_collators: 1,
                max_orchestrator_collators: 1,
                collators_per_container: 1,
                collators_per_parathread: 1,
                full_rotation_period: 0,
                ..Default::default()
            },
            ..Default::default()
        },
        invulnerables: dancebox_runtime::InvulnerablesConfig {
            invulnerables: vec![
                (
                    Sr25519Keyring::Alice.to_account_id(),
                    210 * dancebox_runtime::UNIT,
                ),
                (
                    Sr25519Keyring::Bob.to_account_id(),
                    100 * dancebox_runtime::UNIT,
                ),
            ]
            .clone()
            .into_iter()
            .map(|(account, _balance)| account)
            .collect(),
        },
        parachain_info: dancebox_runtime::ParachainInfoConfig {
            parachain_id: 2000u32.into(),
            ..Default::default()
        },
        polkadot_xcm: dancebox_runtime::PolkadotXcmConfig {
            // TODO: Insert top for the version
            safe_xcm_version: 3.into(),
            ..Default::default()
        },
        session: dancebox_runtime::SessionConfig {
            keys: vec![
                (
                    Sr25519Keyring::Alice.to_account_id(),
                    210 * dancebox_runtime::UNIT,
                ),
                (
                    Sr25519Keyring::Bob.to_account_id(),
                    100 * dancebox_runtime::UNIT,
                ),
            ]
            .into_iter()
            .map(|(account, _balance)| {
                let nimbus_id = get_aura_id_from_seed(&account.to_string());
                (
                    account.clone(),
                    account,
                    dancebox_runtime::SessionKeys { nimbus: nimbus_id },
                )
            })
            .collect(),
            ..Default::default()
        },
        ..Default::default()
    };

    build_genesis_storage(&genesis_config, dancebox_runtime::WASM_BINARY.unwrap())
}
