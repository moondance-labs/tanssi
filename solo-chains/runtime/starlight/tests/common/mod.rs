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

#![allow(dead_code)]

use frame_support::assert_ok;
use {
    crate::UNIT,
    babe_primitives::{
        digests::{PreDigest, SecondaryPlainPreDigest},
        BABE_ENGINE_ID,
    },
    cumulus_primitives_core::ParaId,
    frame_support::traits::{OnFinalize, OnInitialize},
    nimbus_primitives::NimbusId,
    pallet_registrar_runtime_api::ContainerChainGenesisData,
    parity_scale_codec::{Decode, Encode, MaxEncodedLen},
    sp_runtime::traits::Dispatchable,
    sp_runtime::{traits::SaturatedConversion, BuildStorage, Digest, DigestItem},
    starlight_runtime::RuntimeCall,
    test_relay_sproof_builder::ParaHeaderSproofBuilder,
};

// The compiles breaks a bit because multiple integration tests all define `mod common`
// We should probably move them into a folder so that we only declare `mod common` once
#[allow(unused_imports)]
pub use starlight_runtime::{
    genesis_config_presets::get_authority_keys_from_seed, AccountId, AuthorNoting, Babe, Balance,
    Grandpa, Initializer, Runtime, Session, System, TanssiAuthorityAssignment,
    TanssiCollatorAssignment, TransactionPayment,
};

pub fn session_to_block(n: u32) -> u32 {
    // let block_number = flashbox_runtime::Period::get() * n;
    let block_number = Babe::current_epoch().duration.saturated_into::<u32>() * n;

    // Add 1 because the block that emits the NewSession event cannot contain any extrinsics,
    // so this is the first block of the new session that can actually be used
    block_number + 1
}

pub fn babe_authorities() -> Vec<babe_primitives::AuthorityId> {
    Babe::authorities()
        .iter()
        .map(|(key, _)| key.clone())
        .collect()
}

pub fn grandpa_authorities() -> Vec<pallet_grandpa::AuthorityId> {
    Grandpa::grandpa_authorities()
        .iter()
        .map(|(key, _)| key.clone())
        .collect()
}

pub fn authorities_for_container(para_id: ParaId) -> Option<Vec<NimbusId>> {
    let session_index = Session::current_index();

    TanssiAuthorityAssignment::collator_container_chain(session_index)
        .expect("authorities should be set")
        .container_chains
        .get(&para_id)
        .cloned()
}

pub fn accounts_for_container(para_id: ParaId) -> Option<Vec<AccountId>> {
    TanssiCollatorAssignment::collator_container_chain()
        .container_chains
        .get(&para_id)
        .cloned()
}

pub fn run_to_session(n: u32) {
    run_to_block(session_to_block(n));
}

/// Utility function that advances the chain to the desired block number.
///
/// After this function returns, the current block number will be `n`, and the block will be "open",
/// meaning that on_initialize has been executed, but on_finalize has not. To execute on_finalize as
/// well, for example to test a runtime api, manually call `end_block` after this, run the test, and
/// call `start_block` to ensure that this function keeps working as expected.
/// Extrinsics should always be executed before on_finalize.
pub fn run_to_block(n: u32) {
    let current_block_number = System::block_number();
    assert!(
        current_block_number < n,
        "run_to_block called with block {} when current block is {}",
        n,
        current_block_number
    );

    while System::block_number() < n {
        run_block();
    }
}

pub fn insert_authorities_and_slot_digests(slot: u64) {
    let pre_digest = Digest {
        logs: vec![DigestItem::PreRuntime(
            BABE_ENGINE_ID,
            PreDigest::SecondaryPlain(SecondaryPlainPreDigest {
                slot: slot.into(),
                authority_index: 0,
            })
            .encode(),
        )],
    };

    System::reset_events();
    System::initialize(
        &(System::block_number() + 1),
        &System::parent_hash(),
        &pre_digest,
    );
}

#[derive(Clone, Encode, Decode, PartialEq, Debug, scale_info::TypeInfo, MaxEncodedLen)]
enum RunBlockState {
    Start(u32),
    End(u32),
}

impl RunBlockState {
    fn assert_can_advance(&self, new_state: &RunBlockState) {
        match self {
            RunBlockState::Start(n) => {
                assert_eq!(
                    new_state,
                    &RunBlockState::End(*n),
                    "expected a call to end_block({}), but user called {:?}",
                    *n,
                    new_state
                );
            }
            RunBlockState::End(n) => {
                assert_eq!(
                    new_state,
                    &RunBlockState::Start(*n + 1),
                    "expected a call to start_block({}), but user called {:?}",
                    *n + 1,
                    new_state
                )
            }
        }
    }
}

fn advance_block_state_machine(new_state: RunBlockState) {
    if frame_support::storage::unhashed::exists(b"__mock_is_xcm_test") {
        // Disable this check in XCM tests, because the XCM emulator runs on_initialize and
        // on_finalize automatically
        return;
    }
    let old_state: RunBlockState =
        frame_support::storage::unhashed::get(b"__mock_debug_block_state").unwrap_or(
            // Initial state is expecting a call to start() with block number 1, so old state should be
            // end of block 0
            RunBlockState::End(0),
        );
    old_state.assert_can_advance(&new_state);
    frame_support::storage::unhashed::put(b"__mock_debug_block_state", &new_state);
}

pub fn start_block() {
    let block_number = System::block_number();
    advance_block_state_machine(RunBlockState::Start(block_number + 1));

    insert_authorities_and_slot_digests(current_slot() + 1);

    // Initialize the new block
    Babe::on_initialize(System::block_number());
    Session::on_initialize(System::block_number());
    Initializer::on_initialize(System::block_number());
}

pub fn end_block() {
    let block_number = System::block_number();
    advance_block_state_machine(RunBlockState::End(block_number));
    // Finalize the block
    Babe::on_finalize(System::block_number());
    Grandpa::on_finalize(System::block_number());
    Session::on_finalize(System::block_number());
    Initializer::on_finalize(System::block_number());
    TransactionPayment::on_finalize(System::block_number());
}

pub fn run_block() {
    end_block();

    start_block()
}

#[derive(Default, Clone)]
pub struct ParaRegistrationParams {
    para_id: u32,
    genesis_data: ContainerChainGenesisData,
    block_production_credits: u32,
    collator_assignment_credits: u32,
}

impl From<(u32, ContainerChainGenesisData, u32, u32)> for ParaRegistrationParams {
    fn from(value: (u32, ContainerChainGenesisData, u32, u32)) -> Self {
        Self {
            para_id: value.0,
            genesis_data: value.1,
            block_production_credits: value.2,
            collator_assignment_credits: value.3,
        }
    }
}

pub fn default_config() -> pallet_configuration::HostConfiguration {
    pallet_configuration::HostConfiguration {
        max_collators: 100,
        min_orchestrator_collators: 2,
        max_orchestrator_collators: 2,
        collators_per_container: 2,
        full_rotation_period: 0,
        ..Default::default()
    }
}

pub struct ExtBuilder {
    // endowed accounts with balances
    balances: Vec<(AccountId, Balance)>,
    // [validator, amount]
    validators: Vec<(AccountId, Balance)>,
    // [collator, amount]
    collators: Vec<(AccountId, Balance)>,
    // sudo key
    sudo: Option<AccountId>,
    // list of registered para ids: para_id, genesis_data, boot_nodes, block_credits, session_credits
    para_ids: Vec<ParaRegistrationParams>,
    // configuration to apply
    config: pallet_configuration::HostConfiguration,
    own_para_id: Option<ParaId>,
    next_free_para_id: ParaId,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            balances: vec![
                // Alice gets 10k extra tokens for her mapping deposit
                (AccountId::from(ALICE), 210_000 * UNIT),
                (AccountId::from(BOB), 100_000 * UNIT),
            ],
            validators: vec![
                (AccountId::from(ALICE), 210 * UNIT),
                (AccountId::from(BOB), 100 * UNIT),
            ],
            collators: Default::default(),
            sudo: Default::default(),
            para_ids: Default::default(),
            config: default_config(),
            own_para_id: Default::default(),
            next_free_para_id: Default::default(),
        }
    }
}

impl ExtBuilder {
    pub fn with_balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
        self.balances = balances;
        self
    }

    pub fn with_sudo(mut self, sudo: AccountId) -> Self {
        self.sudo = Some(sudo);
        self
    }

    pub fn with_validators(mut self, validators: Vec<(AccountId, Balance)>) -> Self {
        self.validators = validators;
        self
    }

    pub fn with_collators(mut self, collators: Vec<(AccountId, Balance)>) -> Self {
        self.collators = collators;
        self
    }

    pub fn with_para_ids(mut self, para_ids: Vec<ParaRegistrationParams>) -> Self {
        self.para_ids = para_ids;
        self
    }

    // Maybe change to with_collators_config?
    pub fn with_config(mut self, config: pallet_configuration::HostConfiguration) -> Self {
        self.config = config;
        self
    }

    // Maybe change to with_collators_config?
    pub fn with_next_free_para_id(mut self, para_id: ParaId) -> Self {
        self.next_free_para_id = para_id;
        self
    }

    pub fn build_storage(self) -> sp_core::storage::Storage {
        let mut t = frame_system::GenesisConfig::<Runtime>::default()
            .build_storage()
            .unwrap();

        pallet_babe::GenesisConfig::<Runtime> {
            ..Default::default()
        }
        .assimilate_storage(&mut t)
        .unwrap();

        pallet_balances::GenesisConfig::<Runtime> {
            balances: self.balances,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        // We need to initialize these pallets first. When initializing pallet-session,
        // these values will be taken into account for collator-assignment.

        pallet_registrar::GenesisConfig::<Runtime> {
            para_ids: self
                .para_ids
                .iter()
                .cloned()
                .map(|registered_para| {
                    (registered_para.para_id.into(), registered_para.genesis_data)
                })
                .collect(),
            phantom: Default::default(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        runtime_common::paras_registrar::GenesisConfig::<Runtime> {
            next_free_para_id: self.next_free_para_id,
            ..Default::default()
        }
        .assimilate_storage(&mut t)
        .unwrap();

        // TODO: add here pallet_services_payment::GenesisConfig

        pallet_configuration::GenesisConfig::<Runtime> {
            config: self.config,
            ..Default::default()
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut keys: Vec<_> = Vec::new();
        if !self.validators.is_empty() {
            let validator_keys: Vec<_> = self
                .validators
                .clone()
                .into_iter()
                .map(|(account, _balance)| {
                    let authority_keys = get_authority_keys_from_seed(&account.to_string());
                    (
                        account.clone(),
                        account,
                        starlight_runtime::SessionKeys {
                            babe: authority_keys.babe.clone(),
                            grandpa: authority_keys.grandpa.clone(),
                            para_validator: authority_keys.para_validator.clone(),
                            para_assignment: authority_keys.para_assignment.clone(),
                            authority_discovery: authority_keys.authority_discovery.clone(),
                            beefy: authority_keys.beefy.clone(),
                            nimbus: authority_keys.nimbus.clone(),
                        },
                    )
                })
                .collect();
            keys.extend(validator_keys)
        }

        if !self.collators.is_empty() {
            // We set invulnerables in pallet_invulnerables
            let invulnerables: Vec<AccountId> = self
                .collators
                .clone()
                .into_iter()
                .map(|(account, _balance)| account)
                .collect();

            pallet_invulnerables::GenesisConfig::<Runtime> {
                invulnerables: invulnerables.clone(),
            }
            .assimilate_storage(&mut t)
            .unwrap();

            // But we also initialize their keys in the session pallet
            // We discard those that had the key initialized already
            // from the validator list
            // in other words, for testing purposes we allow to inject a validator account
            // in the collator list
            let validator_unique_accounts: Vec<_> = self
                .validators
                .iter()
                .map(|(account, _)| account.clone())
                .collect();
            let collator_keys: Vec<_> = self
                .collators
                .into_iter()
                .filter_map(|(account, _balance)| {
                    if validator_unique_accounts.contains(&account) {
                        None
                    } else {
                        let authority_keys = get_authority_keys_from_seed(&account.to_string());
                        Some((
                            account.clone(),
                            account,
                            starlight_runtime::SessionKeys {
                                babe: authority_keys.babe.clone(),
                                grandpa: authority_keys.grandpa.clone(),
                                para_validator: authority_keys.para_validator.clone(),
                                para_assignment: authority_keys.para_assignment.clone(),
                                authority_discovery: authority_keys.authority_discovery.clone(),
                                beefy: authority_keys.beefy.clone(),
                                nimbus: authority_keys.nimbus.clone(),
                            },
                        ))
                    }
                })
                .collect();
            keys.extend(collator_keys)
        }

        pallet_session::GenesisConfig::<Runtime> { keys }
            .assimilate_storage(&mut t)
            .unwrap();

        pallet_sudo::GenesisConfig::<Runtime> { key: self.sudo }
            .assimilate_storage(&mut t)
            .unwrap();
        t
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let t = self.build_storage();
        let mut ext = sp_io::TestExternalities::new(t);

        ext.execute_with(|| {
            // Start block 1
            start_block();
        });
        ext
    }
}

pub fn root_origin() -> <Runtime as frame_system::Config>::RuntimeOrigin {
    <Runtime as frame_system::Config>::RuntimeOrigin::root()
}

pub fn origin_of(account_id: AccountId) -> <Runtime as frame_system::Config>::RuntimeOrigin {
    <Runtime as frame_system::Config>::RuntimeOrigin::signed(account_id)
}

pub fn inherent_origin() -> <Runtime as frame_system::Config>::RuntimeOrigin {
    <Runtime as frame_system::Config>::RuntimeOrigin::none()
}

/// This function is different in solochains: instead of creating a storage proof and calling the
/// `set_latest_author_data` inherent with that proof as argument, this writes to storage directly.
pub fn set_author_noting_inherent_data(builder: ParaHeaderSproofBuilder) {
    for (k, v) in builder.key_values() {
        frame_support::storage::unhashed::put_raw(&k, &v);
    }

    assert_ok!(RuntimeCall::AuthorNoting(
        pallet_author_noting::Call::<Runtime>::set_latest_author_data { data: () }
    )
    .dispatch(inherent_origin()));
}

pub fn empty_genesis_data() -> ContainerChainGenesisData {
    ContainerChainGenesisData {
        storage: Default::default(),
        name: Default::default(),
        id: Default::default(),
        fork_id: Default::default(),
        extensions: Default::default(),
        properties: Default::default(),
    }
}

pub fn current_slot() -> u64 {
    Babe::current_slot().into()
}

pub const ALICE: [u8; 32] = [4u8; 32];
pub const BOB: [u8; 32] = [5u8; 32];
pub const CHARLIE: [u8; 32] = [6u8; 32];
pub const DAVE: [u8; 32] = [7u8; 32];
pub const EVE: [u8; 32] = [8u8; 32];
pub const FERDIE: [u8; 32] = [9u8; 32];
