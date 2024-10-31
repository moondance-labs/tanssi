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

//! Genesis configs presets for the Dancelight runtime

#[cfg(not(feature = "std"))]
use sp_std::alloc::format;
use {
    crate::{SessionKeys, BABE_GENESIS_EPOCH_CONFIG},
    authority_discovery_primitives::AuthorityId as AuthorityDiscoveryId,
    babe_primitives::AuthorityId as BabeId,
    beefy_primitives::ecdsa_crypto::AuthorityId as BeefyId,
    cumulus_primitives_core::relay_chain::{ASSIGNMENT_KEY_TYPE_ID, PARACHAIN_KEY_TYPE_ID},
    dancelight_runtime_constants::currency::UNITS as STAR,
    dp_container_chain_genesis_data::ContainerChainGenesisData,
    grandpa_primitives::AuthorityId as GrandpaId,
    nimbus_primitives::NimbusId,
    pallet_configuration::HostConfiguration,
    primitives::{vstaging::SchedulerParams, AccountId, AccountPublic, AssignmentId, ValidatorId},
    scale_info::prelude::string::String,
    sp_arithmetic::{traits::Saturating, Perbill},
    sp_core::{
        crypto::{key_types, KeyTypeId},
        sr25519, ByteArray, Pair, Public,
    },
    sp_keystore::{Keystore, KeystorePtr},
    sp_runtime::traits::IdentifyAccount,
    sp_std::{cmp::max, vec::Vec},
    tp_traits::ParaId,
};

// import macro, separate due to rustfmt thinking it's the module with the
// same name ^^'
use sp_std::vec;

/// Helper function to generate a crypto pair from seed
fn get_from_seed<TPublic: Public>(
    seed: &str,
    add_to_keystore: Option<(&KeystorePtr, KeyTypeId)>,
) -> <TPublic::Pair as Pair>::Public {
    let secret_uri = format!("//{}", seed);
    let pair = TPublic::Pair::from_string(&secret_uri, None).expect("static values are valid; qed");

    let public = pair.public();

    if let Some((keystore, key_type)) = add_to_keystore {
        keystore
            .insert(key_type, &secret_uri, &public.to_raw_vec())
            .unwrap();
    }
    public
}

/// Helper function to generate an account ID from seed
fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed, None)).into_account()
}

#[derive(Clone, Debug)]
pub struct AuthorityKeys {
    pub stash: AccountId,
    pub controller: AccountId,
    pub babe: BabeId,
    pub grandpa: GrandpaId,
    pub para_validator: ValidatorId,
    pub para_assignment: AssignmentId,
    pub authority_discovery: AuthorityDiscoveryId,
    pub beefy: BeefyId,
    pub nimbus: NimbusId,
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str, keystore: Option<&KeystorePtr>) -> AuthorityKeys {
    let keys = get_authority_keys_from_seed_no_beefy(seed, keystore);

    AuthorityKeys {
        stash: keys.0,
        controller: keys.1,
        babe: keys.2,
        grandpa: keys.3,
        para_validator: keys.4,
        para_assignment: keys.5,
        authority_discovery: keys.6,
        beefy: get_from_seed::<BeefyId>(seed, None),
        nimbus: get_aura_id_from_seed(seed),
    }
}

/// Helper function to generate a crypto pair from seed
pub fn get_aura_id_from_seed(seed: &str) -> NimbusId {
    sp_core::sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
        .into()
}

/// Helper function to generate stash, controller and session key from seed
fn get_authority_keys_from_seed_no_beefy(
    seed: &str,
    keystore: Option<&KeystorePtr>,
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
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<BabeId>(seed, keystore.map(|k| (k, key_types::BABE))),
        get_from_seed::<GrandpaId>(seed, keystore.map(|k| (k, key_types::GRANDPA))),
        get_from_seed::<ValidatorId>(seed, keystore.map(|k| (k, PARACHAIN_KEY_TYPE_ID))),
        get_from_seed::<AssignmentId>(seed, keystore.map(|k| (k, ASSIGNMENT_KEY_TYPE_ID))),
        get_from_seed::<AuthorityDiscoveryId>(
            seed,
            keystore.map(|k| (k, key_types::AUTHORITY_DISCOVERY)),
        ),
    )
}

fn testnet_accounts() -> Vec<AccountId> {
    Vec::from([
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        get_account_id_from_seed::<sr25519::Public>("Bob"),
        get_account_id_from_seed::<sr25519::Public>("Charlie"),
        get_account_id_from_seed::<sr25519::Public>("Dave"),
        get_account_id_from_seed::<sr25519::Public>("Eve"),
        get_account_id_from_seed::<sr25519::Public>("Ferdie"),
        get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
        get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
        get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
        get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
        get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
        get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
    ])
}

fn dancelight_session_keys(
    babe: BabeId,
    grandpa: GrandpaId,
    para_validator: ValidatorId,
    para_assignment: AssignmentId,
    authority_discovery: AuthorityDiscoveryId,
    beefy: BeefyId,
    nimbus: NimbusId,
) -> SessionKeys {
    SessionKeys {
        babe,
        grandpa,
        para_validator,
        para_assignment,
        authority_discovery,
        beefy,
        nimbus,
    }
}

fn default_parachains_host_configuration(
) -> runtime_parachains::configuration::HostConfiguration<primitives::BlockNumber> {
    use primitives::{
        node_features::FeatureIndex, AsyncBackingParams, MAX_CODE_SIZE, MAX_POV_SIZE,
    };

    runtime_parachains::configuration::HostConfiguration {
        validation_upgrade_cooldown: 2u32,
        validation_upgrade_delay: 2,
        code_retention_period: 1200,
        max_code_size: MAX_CODE_SIZE,
        max_pov_size: MAX_POV_SIZE,
        max_head_data_size: 32 * 1024,
        max_upward_queue_count: 8,
        max_upward_queue_size: 1024 * 1024,
        max_downward_message_size: 1024 * 1024,
        max_upward_message_size: 50 * 1024,
        max_upward_message_num_per_candidate: 5,
        hrmp_sender_deposit: 0,
        hrmp_recipient_deposit: 0,
        hrmp_channel_max_capacity: 8,
        hrmp_channel_max_total_size: 8 * 1024,
        hrmp_max_parachain_inbound_channels: 4,
        hrmp_channel_max_message_size: 1024 * 1024,
        hrmp_max_parachain_outbound_channels: 4,
        hrmp_max_message_num_per_candidate: 5,
        dispute_period: 6,
        no_show_slots: 2,
        n_delay_tranches: 25,
        needed_approvals: 2,
        relay_vrf_modulo_samples: 2,
        zeroth_delay_tranche_width: 0,
        minimum_validation_upgrade_delay: 5,
        async_backing_params: AsyncBackingParams {
            max_candidate_depth: 3,
            allowed_ancestry_len: 2,
        },
        node_features: bitvec::vec::BitVec::from_element(
            1u8 << (FeatureIndex::ElasticScalingMVP as usize),
        ),
        scheduler_params: SchedulerParams {
            lookahead: 2,
            group_rotation_frequency: 20,
            paras_availability_period: 4,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn default_parachains_host_configuration_is_consistent() {
    default_parachains_host_configuration().panic_if_not_consistent();
}

fn dancelight_testnet_genesis(
    initial_authorities: Vec<AuthorityKeys>,
    root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>,
    container_chains: Vec<(ParaId, ContainerChainGenesisData, Vec<Vec<u8>>)>,
    invulnerables: Vec<String>,
    host_configuration: HostConfiguration,
) -> serde_json::Value {
    let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);
    let invulnerable_keys: Vec<_> = invulnerables
        .iter()
        .map(|seed| get_authority_keys_from_seed(seed, None))
        .collect();

    let invulnerable_accounts: Vec<_> = invulnerables
        .iter()
        .map(|seed| get_account_id_from_seed::<sr25519::Public>(seed))
        .collect();

    let data_preservers_bootnodes: Vec<_> = container_chains
        .iter()
        .flat_map(|(para_id, _genesis_data, bootnodes)| {
            bootnodes.clone().into_iter().map(|bootnode| {
                (
                    *para_id,
                    AccountId::from([0u8; 32]),
                    bootnode,
                    crate::PreserversAssignmentPaymentRequest::Free,
                    crate::PreserversAssignmentPaymentWitness::Free,
                )
            })
        })
        .collect();

    let para_ids: Vec<_> = container_chains
        .iter()
        .cloned()
        .map(|(para_id, genesis_data, _boot_nodes)| (para_id, genesis_data, None))
        .collect();

    // In order to register container-chains from genesis, we need to register their
    // head on the relay registrar. However there is no easy way to do that unless we touch all the code
    // so we generate a dummy head state for it. This can be then overriden (as zombienet does) and everything would work
    // TODO: make this cleaner
    let registrar_para_ids_info: Vec<_> = container_chains
        .into_iter()
        .filter_map(|(para_id, genesis_data, _boot_nodes)| {
            // Check if the wasm code is present in storage
            // If not present, we ignore it
            let validation_code = match genesis_data
                .storage
                .into_iter()
                .find(|item| item.key == crate::StorageWellKnownKeys::CODE)
            {
                Some(item) => Some(crate::ValidationCode(item.value.clone())),
                None => None,
            }?;
            let genesis_args = runtime_parachains::paras::ParaGenesisArgs {
                genesis_head: vec![0x01].into(),
                validation_code,
                para_kind: runtime_parachains::paras::ParaKind::Parachain,
            };

            Some((
                para_id,
                (
                    genesis_args.genesis_head,
                    genesis_args.validation_code,
                    genesis_args.para_kind,
                ),
            ))
        })
        .collect();

    // Assign 1000 block credits to all container chains registered in genesis
    // Assign 100 collator assignment credits to all container chains registered in genesis
    let para_id_credits: Vec<_> = para_ids
        .iter()
        .map(|(para_id, _genesis_data, _boot_nodes)| (*para_id, 1000, 100).into())
        .collect();

    const ENDOWMENT: u128 = 1_000_000 * STAR;

    let core_percentage_for_pool_paras = Perbill::from_percent(100).saturating_sub(
        host_configuration
            .max_parachain_cores_percentage
            .unwrap_or(Perbill::from_percent(50)),
    );

    // don't go below 4 cores
    let num_cores = max(
        para_ids.len() as u32 + core_percentage_for_pool_paras.mul_ceil(para_ids.len() as u32),
        4,
    );

    // Initialize nextFreeParaId to a para id that is greater than all registered para ids.
    // This is needed for Registrar::reserve.
    let max_para_id = para_ids
        .iter()
        .map(|(para_id, _genesis_data, _boot_nodes)| para_id)
        .max();
    let next_free_para_id = max_para_id
        .map(|x| ParaId::from(u32::from(*x) + 1))
        .unwrap_or(primitives::LOWEST_PUBLIC_ID);

    serde_json::json!({
        "balances": {
            "balances": endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect::<Vec<_>>(),
        },
        "session": {
            "keys": initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.stash.clone(),
                        x.stash.clone(),
                        dancelight_session_keys(
                            x.babe.clone(),
                            x.grandpa.clone(),
                            x.para_validator.clone(),
                            x.para_assignment.clone(),
                            x.authority_discovery.clone(),
                            x.beefy.clone(),
                            x.nimbus.clone(),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
            "nonAuthorityKeys": invulnerable_keys
                .into_iter()
                .enumerate()
                .map(|(i, x)| {
                    (
                        invulnerable_accounts[i].clone(),
                        invulnerable_accounts[i].clone(),
                        dancelight_session_keys(
                            x.babe.clone(),
                            x.grandpa.clone(),
                            x.para_validator.clone(),
                            x.para_assignment.clone(),
                            x.authority_discovery.clone(),
                            x.beefy.clone(),
                            x.nimbus.clone(),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        },
        "babe": {
            "epochConfig": Some(BABE_GENESIS_EPOCH_CONFIG)
        },
        "sudo": { "key": Some(root_key.clone()) },
        "configuration": {
            "config": runtime_parachains::configuration::HostConfiguration {
                scheduler_params: SchedulerParams {
                    max_validators_per_core: Some(1),
                    num_cores,
                    ..default_parachains_host_configuration().scheduler_params
                },
                ..default_parachains_host_configuration()
            },
        },
        "registrar": {
            "nextFreeParaId": next_free_para_id,
        },
        "tanssiInvulnerables": crate::TanssiInvulnerablesConfig {
            invulnerables: invulnerable_accounts,
        },
        "containerRegistrar": crate::ContainerRegistrarConfig { para_ids, ..Default::default() },
        "paras": {
            "paras": registrar_para_ids_info,
        },
        "servicesPayment": crate::ServicesPaymentConfig { para_id_credits },
            "dataPreservers": crate::DataPreserversConfig {
                bootnodes: data_preservers_bootnodes,
                ..Default::default()
        },
        "collatorConfiguration": crate::CollatorConfigurationConfig {
            config: host_configuration,
            ..Default::default()
        },
        "externalValidators": crate::ExternalValidatorsConfig {
            skip_external_validators: false,
            whitelisted_validators: initial_authorities
                .iter()
                .map(|x| {
                    x.stash.clone()
                })
                .collect::<Vec<_>>(),
        },
    })
}

// staging_testnet
fn dancelight_staging_testnet_config_genesis() -> serde_json::Value {
    use {hex_literal::hex, sp_core::crypto::UncheckedInto};

    // subkey inspect "$SECRET"
    let endowed_accounts = Vec::from([
        // 5DwBmEFPXRESyEam5SsQF1zbWSCn2kCjyLW51hJHXe9vW4xs
        hex!["52bc71c1eca5353749542dfdf0af97bf764f9c2f44e860cd485f1cd86400f649"].into(),
    ]);

    let initial_authorities = Vec::from([
        AuthorityKeys {
            stash: //5EHZkbp22djdbuMFH9qt1DVzSCvqi3zWpj6DAYfANa828oei
                hex!["62475fe5406a7cb6a64c51d0af9d3ab5c2151bcae982fb812f7a76b706914d6a"].into(),
                controller: //5FeSEpi9UYYaWwXXb3tV88qtZkmSdB3mvgj3pXkxKyYLGhcd
                hex!["9e6e781a76810fe93187af44c79272c290c2b9e2b8b92ee11466cd79d8023f50"].into(),
                babe: //5Fh6rDpMDhM363o1Z3Y9twtaCPfizGQWCi55BSykTQjGbP7H
                hex!["a076ef1280d768051f21d060623da3ab5b56944d681d303ed2d4bf658c5bed35"].unchecked_into(),
                grandpa: //5CPd3zoV9Aaah4xWucuDivMHJ2nEEmpdi864nPTiyRZp4t87
                hex!["0e6d7d1afbcc6547b92995a394ba0daed07a2420be08220a5a1336c6731f0bfa"].unchecked_into(),
                para_validator: //5CP6oGfwqbEfML8efqm1tCZsUgRsJztp9L8ZkEUxA16W8PPz
                hex!["0e07a51d3213842f8e9363ce8e444255990a225f87e80a3d651db7841e1a0205"].unchecked_into(),
                para_assignment: //5HQdwiDh8Qtd5dSNWajNYpwDvoyNWWA16Y43aEkCNactFc2b
                hex!["ec60e71fe4a567ef9fef99d4bbf37ffae70564b41aa6f94ef0317c13e0a5477b"].unchecked_into(),
                authority_discovery: //5HbSgM72xVuscsopsdeG3sCSCYdAeM1Tay9p79N6ky6vwDGq
                hex!["f49eae66a0ac9f610316906ec8f1a0928e20d7059d76a5ca53cbcb5a9b50dd3c"].unchecked_into(),
                beefy: //5DPSWdgw38Spu315r6LSvYCggeeieBAJtP5A1qzuzKhqmjVu
                hex!["034f68c5661a41930c82f26a662276bf89f33467e1c850f2fb8ef687fe43d62276"].unchecked_into(),
                nimbus: //5Fh6rDpMDhM363o1Z3Y9twtaCPfizGQWCi55BSykTQjGbP7H
                hex!["a076ef1280d768051f21d060623da3ab5b56944d681d303ed2d4bf658c5bed35"].unchecked_into(),
            },
        AuthorityKeys {
                stash: //5DvH8oEjQPYhzCoQVo7WDU91qmQfLZvxe9wJcrojmJKebCmG
                hex!["520b48452969f6ddf263b664de0adb0c729d0e0ad3b0e5f3cb636c541bc9022a"].into(),
                controller: //5ENZvCRzyXJJYup8bM6yEzb2kQHEb1NDpY2ZEyVGBkCfRdj3
                hex!["6618289af7ae8621981ffab34591e7a6486e12745dfa3fd3b0f7e6a3994c7b5b"].into(),
                babe: //5DLjSUfqZVNAADbwYLgRvHvdzXypiV1DAEaDMjcESKTcqMoM
                hex!["38757d0de00a0c739e7d7984ef4bc01161bd61e198b7c01b618425c16bb5bd5f"].unchecked_into(),
                grandpa: //5HnDVBN9mD6mXyx8oryhDbJtezwNSj1VRXgLoYCBA6uEkiao
                hex!["fcd5f87a6fd5707a25122a01b4dac0a8482259df7d42a9a096606df1320df08d"].unchecked_into(),
                para_validator: //5EPEWRecy2ApL5n18n3aHyU1956zXTRqaJpzDa9DoqiggNwF
                hex!["669a10892119453e9feb4e3f1ee8e028916cc3240022920ad643846fbdbee816"].unchecked_into(),
                para_assignment: //5ES3fw5X4bndSgLNmtPfSbM2J1kLqApVB2CCLS4CBpM1UxUZ
                hex!["68bf52c482630a8d1511f2edd14f34127a7d7082219cccf7fd4c6ecdb535f80d"].unchecked_into(),
                authority_discovery: //5HeXbwb5PxtcRoopPZTp5CQun38atn2UudQ8p2AxR5BzoaXw
                hex!["f6f8fe475130d21165446a02fb1dbce3a7bf36412e5d98f4f0473aed9252f349"].unchecked_into(),
                beefy: //5F7nTtN8MyJV4UsXpjg7tHSnfANXZ5KRPJmkASc1ZSH2Xoa5
                hex!["03a90c2bb6d3b7000020f6152fe2e5002fa970fd1f42aafb6c8edda8dacc2ea77e"].unchecked_into(),
                nimbus: //5DLjSUfqZVNAADbwYLgRvHvdzXypiV1DAEaDMjcESKTcqMoM
                hex!["38757d0de00a0c739e7d7984ef4bc01161bd61e198b7c01b618425c16bb5bd5f"].unchecked_into(),
            },
        AuthorityKeys {
                stash: //5FPMzsezo1PRxYbVpJMWK7HNbR2kUxidsAAxH4BosHa4wd6S
                hex!["92ef83665b39d7a565e11bf8d18d41d45a8011601c339e57a8ea88c8ff7bba6f"].into(),
                controller: //5G6NQidFG7YiXsvV7hQTLGArir9tsYqD4JDxByhgxKvSKwRx
                hex!["b235f57244230589523271c27b8a490922ffd7dccc83b044feaf22273c1dc735"].into(),
                babe: //5GpZhzAVg7SAtzLvaAC777pjquPEcNy1FbNUAG2nZvhmd6eY
                hex!["d2644c1ab2c63a3ad8d40ad70d4b260969e3abfe6d7e6665f50dc9f6365c9d2a"].unchecked_into(),
                grandpa: //5HAes2RQYPbYKbLBfKb88f4zoXv6pPA6Ke8CjN7dob3GpmSP
                hex!["e1b68fbd84333e31486c08e6153d9a1415b2e7e71b413702b7d64e9b631184a1"].unchecked_into(),
                para_validator: //5FtAGDZYJKXkhVhAxCQrXmaP7EE2mGbBMfmKDHjfYDgq2BiU
                hex!["a8e61ffacafaf546283dc92d14d7cc70ea0151a5dd81fdf73ff5a2951f2b6037"].unchecked_into(),
                para_assignment: //5CtK7JHv3h6UQZ44y54skxdwSVBRtuxwPE1FYm7UZVhg8rJV
                hex!["244f3421b310c68646e99cdbf4963e02067601f57756b072a4b19431448c186e"].unchecked_into(),
                authority_discovery: //5D4r6YaB6F7A7nvMRHNFNF6zrR9g39bqDJFenrcaFmTCRwfa
                hex!["2c57f81fd311c1ab53813c6817fe67f8947f8d39258252663b3384ab4195494d"].unchecked_into(),
                beefy: //5EPoHj8uV4fFKQHYThc6Z9fDkU7B6ih2ncVzQuDdNFb8UyhF
                hex!["039d065fe4f9234f0a4f13cc3ae585f2691e9c25afa469618abb6645111f607a53"].unchecked_into(),
                nimbus: hex!["d2644c1ab2c63a3ad8d40ad70d4b260969e3abfe6d7e6665f50dc9f6365c9d2a"].unchecked_into(),
            },
        AuthorityKeys {
                stash: //5DMNx7RoX6d7JQ38NEM7DWRcW2THu92LBYZEWvBRhJeqcWgR
                hex!["38f3c2f38f6d47f161e98c697bbe3ca0e47c033460afda0dda314ab4222a0404"].into(),
                controller: //5GGdKNDr9P47dpVnmtq3m8Tvowwf1ot1abw6tPsTYYFoKm2v
                hex!["ba0898c1964196474c0be08d364cdf4e9e1d47088287f5235f70b0590dfe1704"].into(),
                babe: //5EjkyPCzR2SjhDZq8f7ufsw6TfkvgNRepjCRQFc4TcdXdaB1
                hex!["764186bc30fd5a02477f19948dc723d6d57ab174debd4f80ed6038ec960bfe21"]
                    .unchecked_into(),
                grandpa: //5DJV3zCBTJBLGNDCcdWrYxWDacSz84goGTa4pFeKVvehEBte
                hex!["36be9069cdb4a8a07ecd51f257875150f0a8a1be44a10d9d98dabf10a030aef4"]
                    .unchecked_into(),
                para_validator: //5F9FsRjpecP9GonktmtFL3kjqNAMKjHVFjyjRdTPa4hbQRZA
                hex!["882d72965e642677583b333b2d173ac94b5fd6c405c76184bb14293be748a13b"]
                    .unchecked_into(),
                para_assignment: //5F1FZWZSj3JyTLs8sRBxU6QWyGLSL9BMRtmSKDmVEoiKFxSP
                hex!["821271c99c958b9220f1771d9f5e29af969edfa865631dba31e1ab7bc0582b75"]
                    .unchecked_into(),
                authority_discovery: //5CtgRR74VypK4h154s369abs78hDUxZSJqcbWsfXvsjcHJNA
                hex!["2496f28d887d84705c6dae98aee8bf90fc5ad10bb5545eca1de6b68425b70f7c"]
                    .unchecked_into(),
                beefy: //5CPx6dsr11SCJHKFkcAQ9jpparS7FwXQBrrMznRo4Hqv1PXz
                hex!["0307d29bbf6a5c4061c2157b44fda33b7bb4ec52a5a0305668c74688cedf288d58"]
                    .unchecked_into(),
                nimbus: hex!["764186bc30fd5a02477f19948dc723d6d57ab174debd4f80ed6038ec960bfe21"]
                    .unchecked_into(),
            },
        AuthorityKeys {
                stash: //5C8AL1Zb4bVazgT3EgDxFgcow1L4SJjVu44XcLC9CrYqFN4N
                hex!["02a2d8cfcf75dda85fafc04ace3bcb73160034ed1964c43098fb1fe831de1b16"].into(),
                controller: //5FLYy3YKsAnooqE4hCudttAsoGKbVG3hYYBtVzwMjJQrevPa
                hex!["90cab33f0bb501727faa8319f0845faef7d31008f178b65054b6629fe531b772"].into(),
                babe: //5Et3tfbVf1ByFThNAuUq5pBssdaPPskip5yob5GNyUFojXC7
            hex!["7c94715e5dd8ab54221b1b6b2bfa5666f593f28a92a18e28052531de1bd80813"]
                .unchecked_into(),
            grandpa: //5EX1JBghGbQqWohTPU6msR9qZ2nYPhK9r3RTQ2oD1K8TCxaG
            hex!["6c878e33b83c20324238d22240f735457b6fba544b383e70bb62a27b57380c81"]
                .unchecked_into(),
            para_validator: //5EUNaBpX9mJgcmLQHyG5Pkms6tbDiKuLbeTEJS924Js9cA1N
            hex!["6a8570b9c6408e54bacf123cc2bb1b0f087f9c149147d0005badba63a5a4ac01"]
                .unchecked_into(),
            para_assignment: //5CaZuueRVpMATZG4hkcrgDoF4WGixuz7zu83jeBdY3bgWGaG
            hex!["16c69ea8d595e80b6736f44be1eaeeef2ac9c04a803cc4fd944364cb0d617a33"]
                .unchecked_into(),
            authority_discovery: //5DABsdQCDUGuhzVGWe5xXzYQ9rtrVxRygW7RXf9Tsjsw1aGJ
            hex!["306ac5c772fe858942f92b6e28bd82fb7dd8cdd25f9a4626c1b0eee075fcb531"]
                .unchecked_into(),
            beefy: //5H91T5mHhoCw9JJG4NjghDdQyhC6L7XcSuBWKD3q3TAhEVvQ
            hex!["02fb0330356e63a35dd930bc74525edf28b3bf5eb44aab9e9e4962c8309aaba6a6"]
                .unchecked_into(),
            nimbus: hex!["7c94715e5dd8ab54221b1b6b2bfa5666f593f28a92a18e28052531de1bd80813"]
                .unchecked_into(),
        },
        AuthorityKeys {
            stash: //5C8XbDXdMNKJrZSrQURwVCxdNdk8AzG6xgLggbzuA399bBBF
            hex!["02ea6bfa8b23b92fe4b5db1063a1f9475e3acd0ab61e6b4f454ed6ba00b5f864"].into(),
            controller: //5GsyzFP8qtF8tXPSsjhjxAeU1v7D1PZofuQKN9TdCc7Dp1JM
            hex!["d4ffc4c05b47d1115ad200f7f86e307b20b46c50e1b72a912ec4f6f7db46b616"].into(),
            babe: //5GHWB8ZDzegLcMW7Gdd1BS6WHVwDdStfkkE4G7KjPjZNJBtD
            hex!["bab3cccdcc34401e9b3971b96a662686cf755aa869a5c4b762199ce531b12c5b"]
                .unchecked_into(),
            grandpa: //5GzDPGbUM9uH52ZEwydasTj8edokGUJ7vEpoFWp9FE1YNuFB
            hex!["d9c056c98ca0e6b4eb7f5c58c007c1db7be0fe1f3776108f797dd4990d1ccc33"]
                .unchecked_into(),
            para_validator: //5CmLCFeSurRXXtwMmLcVo7sdJ9EqDguvJbuCYDcHkr3cpqyE
            hex!["1efc23c0b51ad609ab670ecf45807e31acbd8e7e5cb7c07cf49ee42992d2867c"]
                .unchecked_into(),
            para_assignment: //5DnsSy8a8pfE2aFjKBDtKw7WM1V4nfE5sLzP15MNTka53GqS
            hex!["4c64d3f06d28adeb36a892fdaccecace150bec891f04694448a60b74fa469c22"]
                .unchecked_into(),
            authority_discovery: //5CZdFnyzZvKetZTeUwj5APAYskVJe4QFiTezo5dQNsrnehGd
            hex!["160ea09c5717270e958a3da42673fa011613a9539b2e4ebcad8626bc117ca04a"]
                .unchecked_into(),
            beefy: //5HgoR9JJkdBusxKrrs3zgd3ToppgNoGj1rDyAJp4e7eZiYyT
            hex!["020019a8bb188f8145d02fa855e9c36e9914457d37c500e03634b5223aa5702474"]
                .unchecked_into(),
            nimbus: //5GHWB8ZDzegLcMW7Gdd1BS6WHVwDdStfkkE4G7KjPjZNJBtD
            hex!["bab3cccdcc34401e9b3971b96a662686cf755aa869a5c4b762199ce531b12c5b"]
                .unchecked_into(),
        },
        AuthorityKeys {
            stash: //5HinEonzr8MywkqedcpsmwpxKje2jqr9miEwuzyFXEBCvVXM
            hex!["fa373e25a1c4fe19c7148acde13bc3db1811cf656dc086820f3dda736b9c4a00"].into(),
            controller: //5EHJbj6Td6ks5HDnyfN4ttTSi57osxcQsQexm7XpazdeqtV7
            hex!["62145d721967bd88622d08625f0f5681463c0f1b8bcd97eb3c2c53f7660fd513"].into(),
            babe: //5EeCsC58XgJ1DFaoYA1WktEpP27jvwGpKdxPMFjicpLeYu96
            hex!["720537e2c1c554654d73b3889c3ef4c3c2f95a65dd3f7c185ebe4afebed78372"]
                .unchecked_into(),
            grandpa: //5DnEySxbnppWEyN8cCLqvGjAorGdLRg2VmkY96dbJ1LHFK8N
            hex!["4bea0b37e0cce9bddd80835fa2bfd5606f5dcfb8388bbb10b10c483f0856cf14"]
                .unchecked_into(),
            para_validator: //5CAC278tFCHAeHYqE51FTWYxHmeLcENSS1RG77EFRTvPZMJT
            hex!["042f07fc5268f13c026bbe199d63e6ac77a0c2a780f71cda05cee5a6f1b3f11f"]
                .unchecked_into(),
            para_assignment: //5HjRTLWcQjZzN3JDvaj1UzjNSayg5ZD9ZGWMstaL7Ab2jjAa
            hex!["fab485e87ed1537d089df521edf983a777c57065a702d7ed2b6a2926f31da74f"]
                .unchecked_into(),
            authority_discovery: //5ELv74v7QcsS6FdzvG4vL2NnYDGWmRnJUSMKYwdyJD7Xcdi7
            hex!["64d59feddb3d00316a55906953fb3db8985797472bd2e6c7ea1ab730cc339d7f"]
                .unchecked_into(),
            beefy: //5FaUcPt4fPz93vBhcrCJqmDkjYZ7jCbzAF56QJoCmvPaKrmx
            hex!["033f1a6d47fe86f88934e4b83b9fae903b92b5dcf4fec97d5e3e8bf4f39df03685"]
                .unchecked_into(),
            nimbus: hex!["720537e2c1c554654d73b3889c3ef4c3c2f95a65dd3f7c185ebe4afebed78372"]
                .unchecked_into(),
        },
        AuthorityKeys {
            stash: //5Ey3NQ3dfabaDc16NUv7wRLsFCMDFJSqZFzKVycAsWuUC6Di
            hex!["8062e9c21f1d92926103119f7e8153cebdb1e5ab3e52d6f395be80bb193eab47"].into(),
            controller: //5HiWsuSBqt8nS9pnggexXuHageUifVPKPHDE2arTKqhTp1dV
            hex!["fa0388fa88f3f0cb43d583e2571fbc0edad57dff3a6fd89775451dd2c2b8ea00"].into(),
            babe: //5H168nKX2Yrfo3bxj7rkcg25326Uv3CCCnKUGK6uHdKMdPt8
            hex!["da6b2df18f0f9001a6dcf1d301b92534fe9b1f3ccfa10c49449fee93adaa8349"]
                .unchecked_into(),
            grandpa: //5DrA2fZdzmNqT5j6DXNwVxPBjDV9jhkAqvjt6Us3bQHKy3cF
            hex!["4ee66173993dd0db5d628c4c9cb61a27b76611ad3c3925947f0d0011ee2c5dcc"]
                .unchecked_into(),
            para_validator: //5Gx6YeNhynqn8qkda9QKpc9S7oDr4sBrfAu516d3sPpEt26F
            hex!["d822d4088b20dca29a580a577a97d6f024bb24c9550bebdfd7d2d18e946a1c7d"]
                .unchecked_into(),
            para_assignment: //5DhDcHqwxoes5s89AyudGMjtZXx1nEgrk5P45X88oSTR3iyx
            hex!["481538f8c2c011a76d7d57db11c2789a5e83b0f9680dc6d26211d2f9c021ae4c"]
                .unchecked_into(),
            authority_discovery: //5DqAvikdpfRdk5rR35ZobZhqaC5bJXZcEuvzGtexAZP1hU3T
            hex!["4e262811acdfe94528bfc3c65036080426a0e1301b9ada8d687a70ffcae99c26"]
                .unchecked_into(),
            beefy: //5E41Znrr2YtZu8bZp3nvRuLVHg3jFksfQ3tXuviLku4wsao7
            hex!["025e84e95ed043e387ddb8668176b42f8e2773ddd84f7f58a6d9bf436a4b527986"]
                .unchecked_into(),
            nimbus: hex!["da6b2df18f0f9001a6dcf1d301b92534fe9b1f3ccfa10c49449fee93adaa8349"]
                .unchecked_into(),
        },
    ]);

    const ENDOWMENT: u128 = 1_000_000 * STAR;
    const STASH: u128 = 100 * STAR;

    serde_json::json!({
        "balances": {
            "balances": endowed_accounts
                .iter()
                .map(|k: &AccountId| (k.clone(), ENDOWMENT))
                .chain(initial_authorities.iter().map(|x| (x.stash.clone(), STASH)))
                .collect::<Vec<_>>(),
        },
        "session": {
            "keys": initial_authorities
                .into_iter()
                .map(|x| {
                    (
                        x.stash.clone(),
                        x.stash,
                        dancelight_session_keys(
                            x.babe,
                            x.grandpa,
                            x.para_validator,
                            x.para_assignment,
                            x.authority_discovery,
                            x.beefy,
                            x.nimbus,
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        },
        "babe": {
            "epochConfig": Some(BABE_GENESIS_EPOCH_CONFIG),
        },
        "sudo": { "key": Some(endowed_accounts[0].clone()) },
        "configuration": {
            "config": default_parachains_host_configuration(),
        },
        "registrar": {
            "nextFreeParaId": primitives::LOWEST_PUBLIC_ID,
        },
    })
}

//development
pub fn dancelight_development_config_genesis(
    container_chains: Vec<(ParaId, ContainerChainGenesisData, Vec<Vec<u8>>)>,
    invulnerables: Vec<String>,
) -> serde_json::Value {
    dancelight_testnet_genesis(
        Vec::from([get_authority_keys_from_seed("Alice", None)]),
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        None,
        container_chains,
        invulnerables,
        HostConfiguration {
            max_collators: 100u32,
            min_orchestrator_collators: 0u32,
            max_orchestrator_collators: 0u32,
            collators_per_container: 2u32,
            full_rotation_period: runtime_common::prod_or_fast!(24u32, 5u32),
            max_parachain_cores_percentage: Some(Perbill::from_percent(60)),
            ..Default::default()
        },
    )
}

//local_testnet
pub fn dancelight_local_testnet_genesis(
    container_chains: Vec<(ParaId, ContainerChainGenesisData, Vec<Vec<u8>>)>,
    invulnerables: Vec<String>,
) -> serde_json::Value {
    dancelight_testnet_genesis(
        Vec::from([
            get_authority_keys_from_seed("Alice", None),
            get_authority_keys_from_seed("Bob", None),
        ]),
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        None,
        container_chains,
        invulnerables,
        HostConfiguration {
            max_collators: 100u32,
            min_orchestrator_collators: 0u32,
            max_orchestrator_collators: 0u32,
            collators_per_container: 2u32,
            full_rotation_period: runtime_common::prod_or_fast!(24u32, 5u32),
            max_parachain_cores_percentage: Some(Perbill::from_percent(60)),
            ..Default::default()
        },
    )
}

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &sp_genesis_builder::PresetId) -> Option<sp_std::vec::Vec<u8>> {
    let patch = match id.try_into() {
        Ok("local_testnet") => dancelight_local_testnet_genesis(vec![], vec![]),
        Ok("development") => dancelight_development_config_genesis(vec![], vec![]),
        Ok("staging_testnet") => dancelight_staging_testnet_config_genesis(),
        _ => return None,
    };
    Some(
        serde_json::to_string(&patch)
            .expect("serialization to json is expected to work. qed.")
            .into_bytes(),
    )
}
