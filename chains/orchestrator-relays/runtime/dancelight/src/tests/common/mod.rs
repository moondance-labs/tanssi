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

use std::collections::{BTreeSet, VecDeque};
use {
    crate::{
        BlockProductionCost, CollatorAssignmentCost, ExternalValidatorSlashes, MessageQueue,
        RuntimeCall,
    },
    babe_primitives::{
        digests::{PreDigest, SecondaryPlainPreDigest},
        BABE_ENGINE_ID,
    },
    beefy_primitives::{ecdsa_crypto::AuthorityId as BeefyId, ConsensusLog, BEEFY_ENGINE_ID},
    bitvec::prelude::BitVec,
    cumulus_primitives_core::{
        relay_chain::{
            node_features::FeatureIndex, AvailabilityBitfield, vstaging::BackedCandidate,
            CandidateCommitments, vstaging::CandidateDescriptorV2, CollatorPair, vstaging::CommittedCandidateReceiptV2,
            CompactStatement, CoreIndex, GroupIndex, HeadData,
            vstaging::InherentData as ParachainsInherentData, PersistedValidationData, SigningContext,
            UncheckedSigned, ValidationCode, ValidatorIndex, ValidityAttestation,
        },
        ParaId,
    },
    frame_support::{
        assert_ok,
        traits::{OnFinalize, OnInitialize},
    },
    frame_system::pallet_prelude::{BlockNumberFor, HeaderFor},
    nimbus_primitives::NimbusId,
    pallet_registrar_runtime_api::ContainerChainGenesisData,
    pallet_services_payment::{ProvideBlockProductionCost, ProvideCollatorAssignmentCost},
    parity_scale_codec::{Decode, Encode, MaxEncodedLen},
    runtime_parachains::{
        paras::{ParaGenesisArgs, ParaKind},
        paras_inherent as parachains_paras_inherent,
    },
    sp_core::Pair,
    sp_keystore::{KeystoreExt, KeystorePtr},
    sp_runtime::{
        traits::{Dispatchable, Header, One, SaturatedConversion, Zero},
        BuildStorage, Digest, DigestItem,
    },
    sp_std::collections::btree_map::BTreeMap,
    test_relay_sproof_builder::ParaHeaderSproofBuilder,
};

mod xcm;

pub use crate::{
    genesis_config_presets::get_authority_keys_from_seed, AccountId, AuthorNoting, Babe, Balance,
    Balances, Beefy, BeefyMmrLeaf, ContainerRegistrar, DataPreservers, Grandpa, InflationRewards,
    Initializer, Mmr, Runtime, RuntimeOrigin, Session, System, TanssiAuthorityAssignment,
    TanssiCollatorAssignment, TransactionPayment,
};

pub const UNIT: Balance = 1_000_000_000_000_000_000;

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

pub fn get_beefy_digest(log: ConsensusLog<BeefyId>) -> DigestItem {
    DigestItem::Consensus(BEEFY_ENGINE_ID, log.encode())
}

/// FIXME: run_to_session(n) only runs to the last block of session n-1, so Session::index() will
/// return n-1. To actually run to session n, create an additional block afterwards using `run_block()`.
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
pub fn run_to_block(n: u32) -> BTreeMap<u32, RunSummary> {
    let current_block_number = System::block_number();
    assert!(
        current_block_number < n,
        "run_to_block called with block {} when current block is {}",
        n,
        current_block_number
    );

    let mut summaries = BTreeMap::new();

    while System::block_number() < n {
        let summary = run_block();
        let block_number = System::block_number();
        summaries.insert(block_number, summary);
    }

    summaries
}

pub fn get_genesis_data_with_validation_code() -> (ContainerChainGenesisData, Vec<u8>) {
    let validation_code = mock_validation_code().0;
    let genesis_data = ContainerChainGenesisData {
        storage: vec![(b":code".to_vec(), validation_code.clone()).into()],
        name: Default::default(),
        id: Default::default(),
        fork_id: Default::default(),
        extensions: vec![],
        properties: Default::default(),
    };
    (genesis_data, validation_code)
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RunSummary {
    pub inflation: Balance,
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

pub fn start_block() -> RunSummary {
    let block_number = System::block_number();
    advance_block_state_machine(RunBlockState::Start(block_number + 1));

    insert_authorities_and_slot_digests(current_slot() + 1);

    // Initialize the new block
    Babe::on_initialize(System::block_number());
    ContainerRegistrar::on_initialize(System::block_number());
    ExternalValidatorSlashes::on_initialize(System::block_number());
    Session::on_initialize(System::block_number());

    Initializer::on_initialize(System::block_number());
    TanssiCollatorAssignment::on_initialize(System::block_number());
    MessageQueue::on_initialize(System::block_number());

    let current_issuance = Balances::total_issuance();
    InflationRewards::on_initialize(System::block_number());
    let new_issuance = Balances::total_issuance();

    let maybe_mock_inherent = take_new_inherent_data();
    if let Some(mock_inherent_data) = maybe_mock_inherent {
        set_paras_inherent(mock_inherent_data);
    }

    Beefy::on_initialize(System::block_number());
    Mmr::on_initialize(System::block_number());
    BeefyMmrLeaf::on_initialize(System::block_number());
    RunSummary {
        inflation: new_issuance - current_issuance,
    }
}

pub fn end_block() {
    let block_number = System::block_number();
    advance_block_state_machine(RunBlockState::End(block_number));
    // Finalize the block
    Babe::on_finalize(System::block_number());
    Session::on_finalize(System::block_number());
    Grandpa::on_finalize(System::block_number());
    TransactionPayment::on_finalize(System::block_number());
    Initializer::on_finalize(System::block_number());
    ContainerRegistrar::on_finalize(System::block_number());
    TanssiCollatorAssignment::on_finalize(System::block_number());
    Beefy::on_finalize(System::block_number());
    Mmr::on_finalize(System::block_number());
    BeefyMmrLeaf::on_finalize(System::block_number());
}

pub fn run_block() -> RunSummary {
    end_block();

    start_block()
}

#[derive(Default, Clone)]
pub struct ParaRegistrationParams {
    pub para_id: u32,
    pub genesis_data: ContainerChainGenesisData,
    pub block_production_credits: u32,
    pub collator_assignment_credits: u32,
    pub parathread_params: Option<tp_traits::ParathreadParams>,
}

impl From<(u32, ContainerChainGenesisData, u32, u32)> for ParaRegistrationParams {
    fn from(value: (u32, ContainerChainGenesisData, u32, u32)) -> Self {
        Self {
            para_id: value.0,
            genesis_data: value.1,
            block_production_credits: value.2,
            collator_assignment_credits: value.3,
            parathread_params: None,
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

#[derive(Clone)]
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
    relay_config: runtime_parachains::configuration::HostConfiguration<BlockNumberFor<Runtime>>,
    own_para_id: Option<ParaId>,
    next_free_para_id: ParaId,
    keystore: Option<KeystorePtr>,
    safe_xcm_version: Option<u32>,
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
            relay_config: runtime_parachains::configuration::HostConfiguration {
                scheduler_params: SchedulerParams {
                    num_cores: 6,
                    ..Default::default()
                },
                max_head_data_size: 20500,
                ..Default::default()
            },
            own_para_id: Default::default(),
            next_free_para_id: Default::default(),
            keystore: None,
            safe_xcm_version: Default::default(),
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

    /// Helper function like `with_para_ids` but registering parachains with an empty genesis data,
    /// and max amount of credits.
    pub fn with_empty_parachains(mut self, para_ids: Vec<u32>) -> Self {
        self.para_ids = para_ids
            .into_iter()
            .map(|para_id| ParaRegistrationParams {
                para_id,
                genesis_data: empty_genesis_data(),
                block_production_credits: u32::MAX,
                collator_assignment_credits: u32::MAX,
                parathread_params: None,
            })
            .collect();
        self
    }

    pub fn with_additional_empty_parathreads(mut self, para_ids: Vec<u32>) -> Self {
        self.para_ids = self
            .para_ids
            .iter()
            .cloned()
            .chain(para_ids.into_iter().map(|para_id| ParaRegistrationParams {
                para_id,
                genesis_data: empty_genesis_data(),
                block_production_credits: u32::MAX,
                collator_assignment_credits: u32::MAX,
                parathread_params: Some(ParathreadParams {
                    slot_frequency: Default::default(),
                }),
            }))
            .collect();
        self
    }

    // Maybe change to with_collators_config?
    pub fn with_config(mut self, config: pallet_configuration::HostConfiguration) -> Self {
        self.config = config;
        self
    }

    pub fn with_safe_xcm_version(mut self, safe_xcm_version: u32) -> Self {
        self.safe_xcm_version = Some(safe_xcm_version);
        self
    }

    // Maybe change to with_collators_config?
    pub fn with_relay_config(
        mut self,
        relay_config: runtime_parachains::configuration::HostConfiguration<BlockNumberFor<Runtime>>,
    ) -> Self {
        self.relay_config = relay_config;
        self
    }

    // Maybe change to with_collators_config?
    pub fn with_next_free_para_id(mut self, para_id: ParaId) -> Self {
        self.next_free_para_id = para_id;
        self
    }

    // Maybe change to with_collators_config?
    pub fn with_keystore(mut self, keystore: KeystorePtr) -> Self {
        self.keystore = Some(keystore);
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
                    (
                        registered_para.para_id.into(),
                        registered_para.genesis_data,
                        registered_para.parathread_params,
                    )
                })
                .collect(),
            ..Default::default()
        }
        .assimilate_storage(&mut t)
        .unwrap();

        // We register mock wasm
        runtime_parachains::paras::GenesisConfig::<Runtime> {
            paras: self
                .para_ids
                .iter()
                .cloned()
                .map(|registered_para| {
                    let para_kind = if registered_para.parathread_params.is_some() {
                        ParaKind::Parathread
                    } else {
                        ParaKind::Parachain
                    };
                    (
                        registered_para.para_id.into(),
                        ParaGenesisArgs {
                            validation_code: mock_validation_code(),
                            para_kind,
                            genesis_head: HeadData::from(vec![0u8]),
                        },
                    )
                })
                .collect(),
            ..Default::default()
        }
        .assimilate_storage(&mut t)
        .unwrap();

        pallet_services_payment::GenesisConfig::<Runtime> {
            para_id_credits: self
                .para_ids
                .clone()
                .into_iter()
                .map(|registered_para| {
                    (
                        registered_para.para_id.into(),
                        registered_para.block_production_credits,
                        registered_para.collator_assignment_credits,
                    )
                        .into()
                })
                .collect(),
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

        pallet_xcm::GenesisConfig::<Runtime> {
            safe_xcm_version: self.safe_xcm_version,
            ..Default::default()
        }
        .assimilate_storage(&mut t)
        .unwrap();

        runtime_parachains::configuration::GenesisConfig::<Runtime> {
            config: self.relay_config,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut keys: Vec<_> = Vec::new();
        let mut non_authority_keys: Vec<_> = Vec::new();
        if !self.validators.is_empty() {
            let validator_keys: Vec<_> = self
                .validators
                .clone()
                .into_iter()
                .map(|(account, _balance)| {
                    let authority_keys =
                        get_authority_keys_from_seed(&account.to_string(), self.keystore.as_ref());
                    (
                        account.clone(),
                        account,
                        crate::SessionKeys {
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
                        let authority_keys =
                            get_authority_keys_from_seed(&account.to_string(), None);
                        Some((
                            account.clone(),
                            account,
                            crate::SessionKeys {
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
            non_authority_keys.extend(collator_keys)
        }

        pallet_external_validators::GenesisConfig::<Runtime> {
            skip_external_validators: false,
            whitelisted_validators: self
                .validators
                .iter()
                .map(|(account, _)| account.clone())
                .collect(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        pallet_session::GenesisConfig::<Runtime> {
            keys,
            non_authority_keys,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        pallet_sudo::GenesisConfig::<Runtime> { key: self.sudo }
            .assimilate_storage(&mut t)
            .unwrap();

        snowbridge_pallet_system::GenesisConfig::<Runtime> {
            // This is irrelevant, we can put any number here
            // as long as it is a non-used para id
            para_id: 1000u32.into(),
            asset_hub_para_id: 1001u32.into(),
            ..Default::default()
        }
        .assimilate_storage(&mut t)
        .unwrap();

        if self.safe_xcm_version.is_some() {
            // Disable run_block checks in XCM tests, because the XCM emulator runs on_initialize and
            // on_finalize automatically
            t.top.insert(b"__mock_is_xcm_test".to_vec(), b"1".to_vec());
        }

        t
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let keystore = self.keystore.clone();
        let t = self.build_storage();
        let mut ext = sp_io::TestExternalities::new(t);
        if let Some(keystore) = keystore {
            ext.register_extension(KeystoreExt(keystore));
        }
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

pub fn block_credits_to_required_balance(number_of_blocks: u32, para_id: ParaId) -> Balance {
    let block_cost = BlockProductionCost::block_cost(&para_id).0;
    u128::from(number_of_blocks).saturating_mul(block_cost)
}

pub fn collator_assignment_credits_to_required_balance(
    number_of_sessions: u32,
    para_id: ParaId,
) -> Balance {
    let collator_assignment_cost = CollatorAssignmentCost::collator_assignment_cost(&para_id).0;
    u128::from(number_of_sessions).saturating_mul(collator_assignment_cost)
}

pub const ALICE: [u8; 32] = [4u8; 32];
pub const BOB: [u8; 32] = [5u8; 32];
pub const CHARLIE: [u8; 32] = [6u8; 32];
pub const DAVE: [u8; 32] = [7u8; 32];
pub const EVE: [u8; 32] = [8u8; 32];
pub const FERDIE: [u8; 32] = [9u8; 32];

fn take_new_inherent_data() -> Option<cumulus_primitives_core::relay_chain::vstaging::InherentData> {
    let data: Option<cumulus_primitives_core::relay_chain::vstaging::InherentData> =
        frame_support::storage::unhashed::take(b"ParasInherent");

    data
}

pub fn set_new_inherent_data(data: cumulus_primitives_core::relay_chain::vstaging::InherentData) {
    frame_support::storage::unhashed::put(b"ParasInherent", &data);
}

pub fn set_new_randomness_data(data: Option<[u8; 32]>) {
    pallet_babe::AuthorVrfRandomness::<Runtime>::set(data);
}

/// Mock the inherent that sets validation data in ParachainSystem, which
/// contains the `relay_chain_block_number`, which is used in `collator-assignment` as a
/// source of randomness.
pub fn set_paras_inherent(data: cumulus_primitives_core::relay_chain::vstaging::InherentData) {
    // In order for this inherent to work, we need to match the parent header
    // the parent header does not play a significant role in the rest of the framework so
    // we are simply going to mock it
    System::set_parent_hash(data.parent_header.hash());
    assert_ok!(
        RuntimeCall::ParaInherent(parachains_paras_inherent::Call::<Runtime>::enter { data })
            .dispatch(inherent_origin())
    );
    // Error: InherentDataFilteredDuringExecution
    frame_support::storage::unhashed::kill(&frame_support::storage::storage_prefix(
        b"ParaInherent",
        b"Included",
    ));
}

pub(crate) struct ParasInherentTestBuilder<T: runtime_parachains::paras_inherent::Config> {
    /// Starting block number; we expect it to get incremented on session setup.
    block_number: BlockNumberFor<T>,

    /// Session index of for each dispute. Index of slice corresponds to a core,
    /// which is offset by the number of entries for `backed_and_concluding_paras`. I.E. if
    /// `backed_and_concluding_paras` has 3 entries, the first index of `dispute_sessions`
    /// will correspond to core index 3. There must be one entry for each core with a dispute
    /// statement set.
    dispute_sessions: Vec<u32>,

    /// Paras here will both be backed in the inherent data and already occupying a core (which is
    /// freed via bitfields).
    ///
    /// Map from para id to number of validity votes. Core indices are generated based on
    /// `elastic_paras` configuration. Each para id in `elastic_paras` gets the
    /// specified amount of consecutive cores assigned to it. If a para id is not present
    /// in `elastic_paras` it get assigned to a single core.
    backed_and_concluding_paras: BTreeMap<u32, u32>,

    /// Paras which don't yet occupy a core, but will after the inherent has been processed.
    backed_in_inherent_paras: BTreeMap<u32, u32>,

    /// Map from para id (seed) to number of chained candidates.
    elastic_paras: BTreeMap<u32, u8>,
    /// Make every candidate include a code upgrade by setting this to `Some` where the interior
    /// value is the byte length of the new code.
    code_upgrade: Option<u32>,

    _phantom: core::marker::PhantomData<T>,
}

pub fn mock_validation_code() -> ValidationCode {
    ValidationCode(vec![1; 10])
}

/// Create a dummy collator id suitable to be used in a V1 candidate descriptor.
pub fn junk_collator() -> CollatorId {
    CollatorId::from_slice(&mut (0..32).into_iter().collect::<Vec<_>>().as_slice())
        .expect("32 bytes; qed")
}

/// Creates a dummy collator signature suitable to be used in a V1 candidate descriptor.
pub fn junk_collator_signature() -> CollatorSignature {
    CollatorSignature::from_slice(&mut (0..64).into_iter().collect::<Vec<_>>().as_slice())
        .expect("64 bytes; qed")
}

#[allow(dead_code)]
impl<T: runtime_parachains::paras_inherent::Config> ParasInherentTestBuilder<T> {
    /// Create a new `BenchBuilder` with some opinionated values that should work with the rest
    /// of the functions in this implementation.
    pub(crate) fn new() -> Self {
        ParasInherentTestBuilder {
            block_number: Zero::zero(),
            dispute_sessions: Default::default(),
            backed_and_concluding_paras: Default::default(),
            backed_in_inherent_paras: Default::default(),
            elastic_paras: Default::default(),
            code_upgrade: None,
            _phantom: core::marker::PhantomData::<T>,
        }
    }

    /// Set a map from para id seed to number of validity votes.
    pub(crate) fn set_backed_and_concluding_paras(
        mut self,
        backed_and_concluding_paras: BTreeMap<u32, u32>,
    ) -> Self {
        self.backed_and_concluding_paras = backed_and_concluding_paras;
        self
    }

    /// Set a map from para id seed to number of validity votes for votes in inherent data.
    pub(crate) fn set_backed_in_inherent_paras(mut self, backed: BTreeMap<u32, u32>) -> Self {
        self.backed_in_inherent_paras = backed;
        self
    }

    /// Set a map from para id seed to number of cores assigned to it.
    pub(crate) fn set_elastic_paras(mut self, elastic_paras: BTreeMap<u32, u8>) -> Self {
        self.elastic_paras = elastic_paras;
        self
    }

    /// Set to include a code upgrade for all backed candidates. The value will be the byte length
    /// of the code.
    pub(crate) fn set_code_upgrade(mut self, code_upgrade: impl Into<Option<u32>>) -> Self {
        self.code_upgrade = code_upgrade.into();
        self
    }

    /// Mock header.
    pub(crate) fn header(block_number: BlockNumberFor<T>) -> HeaderFor<T> {
        HeaderFor::<T>::new(
            block_number,       // `block_number`,
            Default::default(), // `extrinsics_root`,
            Default::default(), // `storage_root`,
            Default::default(), // `parent_hash`,
            Default::default(), // digest,
        )
    }

    /// Maximum number of validators that may be part of a validator group.
    pub(crate) fn fallback_max_validators() -> u32 {
        runtime_parachains::configuration::ActiveConfig::<T>::get()
            .max_validators
            .unwrap_or(200)
    }

    /// Maximum number of validators participating in parachains consensus (a.k.a. active
    /// validators).
    fn max_validators(&self) -> u32 {
        Self::fallback_max_validators()
    }

    /// Maximum number of validators per core (a.k.a. max validators per group). This value is used
    /// if none is explicitly set on the builder.
    pub(crate) fn fallback_max_validators_per_core() -> u32 {
        runtime_parachains::configuration::ActiveConfig::<T>::get()
            .scheduler_params
            .max_validators_per_core
            .unwrap_or(5)
    }

    /// Get the maximum number of validators per core.
    fn max_validators_per_core(&self) -> u32 {
        Self::fallback_max_validators_per_core()
    }

    /// Get the maximum number of cores we expect from this configuration.
    pub(crate) fn max_cores(&self) -> u32 {
        self.max_validators() / self.max_validators_per_core()
    }

    /// Create a bitvec of `validators` length with all yes votes.
    fn validator_availability_votes_yes(validators: usize) -> BitVec<u8, bitvec::order::Lsb0> {
        // every validator confirms availability.
        bitvec::bitvec![u8, bitvec::order::Lsb0; 1; validators]
    }

    pub fn mock_head_data() -> HeadData {
        let max_head_size =
            runtime_parachains::configuration::ActiveConfig::<T>::get().max_head_data_size;
        HeadData(vec![0xFF; max_head_size as usize])
    }

    fn candidate_descriptor_mock(
        para_id: ParaId,
        candidate_descriptor_v2: bool,
    ) -> CandidateDescriptorV2<T::Hash> {
        if candidate_descriptor_v2 {
            CandidateDescriptorV2::new(
                para_id,
                Default::default(),
                CoreIndex(200),
                2,
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                mock_validation_code().hash(),
            )
        } else {
            // Convert v1 to v2.
            CandidateDescriptor::<T::Hash> {
                para_id,
                relay_parent: Default::default(),
                collator: junk_collator(),
                persisted_validation_data_hash: Default::default(),
                pov_hash: Default::default(),
                erasure_root: Default::default(),
                signature: junk_collator_signature(),
                para_head: Default::default(),
                validation_code_hash: mock_validation_code().hash(),
            }
                .into()
        }
            .into()
    }

    /*
    /// Create a mock of `CandidatePendingAvailability`.
    fn candidate_availability_mock(
        para_id: ParaId,
        group_idx: GroupIndex,
        core_idx: CoreIndex,
        candidate_hash: CandidateHash,
        availability_votes: BitVec<u8, bitvec::order::Lsb0>,
        commitments: CandidateCommitments,
        candidate_descriptor_v2: bool,
    ) -> CandidatePendingAvailability<T::Hash, BlockNumberFor<T>> {
        CandidatePendingAvailability::<T::Hash, BlockNumberFor<T>>::new(
            core_idx,                                                          // core
            candidate_hash,                                                    // hash
            Self::candidate_descriptor_mock(para_id, candidate_descriptor_v2), /* candidate descriptor */
            commitments,                                                       // commitments
            availability_votes,                                                /* availability
                                                                                            * votes */
            Default::default(), // backers
            Zero::zero(),       // relay parent
            One::one(),         /* relay chain block this
                                             * was backed in */
            group_idx, // backing group
        )
    }
     */

    /*
    /// Add `CandidatePendingAvailability` and `CandidateCommitments` to the relevant storage items.
    ///
    /// NOTE: the default `CandidateCommitments` used does not include any data that would lead to
    /// heavy code paths in `enact_candidate`. But enact_candidates does return a weight which will
    /// get taken into account.
    fn add_availability(
        para_id: ParaId,
        core_idx: CoreIndex,
        group_idx: GroupIndex,
        availability_votes: BitVec<u8, bitvec::order::Lsb0>,
        candidate_hash: CandidateHash,
        candidate_descriptor_v2: bool,
    ) {
        let commitments = CandidateCommitments::<u32> {
            upward_messages: Default::default(),
            horizontal_messages: Default::default(),
            new_validation_code: None,
            head_data: Self::mock_head_data(),
            processed_downward_messages: 0,
            hrmp_watermark: 0u32.into(),
        };
        let candidate_availability = Self::candidate_availability_mock(
            para_id,
            group_idx,
            core_idx,
            candidate_hash,
            availability_votes,
            commitments,
            candidate_descriptor_v2,
        );
        inclusion::PendingAvailability::<T>::mutate(para_id, |maybe_candidates| {
            if let Some(candidates) = maybe_candidates {
                candidates.push_back(candidate_availability);
            } else {
                *maybe_candidates =
                    Some([candidate_availability].into_iter().collect::<VecDeque<_>>());
            }
        });
    }
     */

    /// Create an `AvailabilityBitfield` where `concluding` is a map where each key is a core index
    /// that is concluding and `cores` is the total number of cores in the system.
    fn availability_bitvec(concluding_cores: &BTreeSet<u32>, cores: usize) -> AvailabilityBitfield {
        let mut bitfields = bitvec::bitvec![u8, bitvec::order::Lsb0; 0; 0];
        for i in 0..cores {
            if concluding_cores.contains(&(i as u32)) {
                bitfields.push(true);
            } else {
                bitfields.push(false)
            }
        }

        bitfields.into()
    }

    /// Number of the relay parent block.
    fn relay_parent_number(&self) -> u32 {
        (Self::block_number() - One::one())
            .try_into()
            .map_err(|_| ())
            .expect("Self::block_number() is u32")
    }

    /// Create backed candidates for `cores_with_backed_candidates`. You need these cores to be
    /// scheduled _within_ paras inherent, which requires marking the available bitfields as fully
    /// available.
    /// - `cores_with_backed_candidates` Mapping of `para_id` seed to number of
    /// validity votes.
    /// Important! this uses a BtreeMap, which means that elements will use increasing core orders
    /// example: if we have parachains 1000, 1001, and 1002, they will use respectively cores
    /// 0 1 and 2. There is no way in which we force 1002 to use core 0 in this setup
    fn create_backed_candidates(
        &self,
        paras_with_backed_candidates: &BTreeMap<u32, u32>,
    ) -> Vec<BackedCandidate<T::Hash>> {
        let current_session = runtime_parachains::shared::CurrentSessionIndex::<T>::get();
        // We need to refetch validators since they have been shuffled.
        let validators_shuffled =
            runtime_parachains::session_info::Sessions::<T>::get(current_session)
                .unwrap()
                .validators
                .clone();

        let config = runtime_parachains::configuration::ActiveConfig::<T>::get();
        let mut current_core_idx = 0u32;
        paras_with_backed_candidates
            .iter()
            .flat_map(|(seed, num_votes)| {
                assert!(*num_votes <= validators_shuffled.len() as u32);

                let para_id = ParaId::from(*seed);
                let prev_head_non_mut = runtime_parachains::paras::Heads::<T>::get(para_id);
                let prev_head = prev_head_non_mut.unwrap_or(Self::mock_head_data());
                // How many chained candidates we want to build ?
                (0..1)
                    .map(|chain_idx| {
                        let core_idx = CoreIndex::from(current_core_idx);
                        // Advance core index.
                        current_core_idx += 1;
                        let group_idx =
                            Self::group_assigned_to_core(core_idx, Self::block_number())
                                .unwrap_or_else(|| {
                                    panic!("Validator group not assigned to core {:?}", core_idx)
                                });

                        let header = Self::header(Self::block_number());
                        let relay_parent = header.hash();

                        // Set the head data so it can be used while validating the signatures on
                        // the candidate receipt.
                        let mut head_data = Self::mock_head_data();

                        if chain_idx == 0 {
                            // Only first parahead of the chain needs to be set in storage.
                            Self::heads_insert(&para_id, prev_head.clone());
                        } else {
                            // Make each candidate head data unique to avoid cycles.
                            head_data.0[0] = chain_idx;
                        }

                        let persisted_validation_data = PersistedValidationData::<T::Hash> {
                            // To form a chain we set parent head to previous block if any, or
                            // default to what is in storage already setup.
                            parent_head: prev_head.clone(),
                            relay_parent_number: self.relay_parent_number() + 1,
                            relay_parent_storage_root: Default::default(),
                            max_pov_size: config.max_pov_size,
                        };

                        let persisted_validation_data_hash = persisted_validation_data.hash();

                        let pov_hash = Default::default();
                        let validation_code_hash = mock_validation_code().hash();

                        /*
                        let mut past_code_meta =
                            paras::ParaPastCodeMeta::<BlockNumberFor<T>>::default();
                        past_code_meta.note_replacement(0u32.into(), 0u32.into());
                         */

                        let group_validators = Self::group_validators(group_idx).unwrap();

                        let descriptor = if true /* self.candidate_descriptor_v2 */ {
                            CandidateDescriptorV2::new(
                                para_id,
                                relay_parent,
                                core_idx,
                                current_session,
                                persisted_validation_data_hash,
                                pov_hash,
                                Default::default(),
                                prev_head.hash(),
                                validation_code_hash,
                            )
                        } else {
                            todo!()
                        };

                        let mut candidate = CommittedCandidateReceiptV2::<T::Hash> {
                            descriptor,
                            commitments: CandidateCommitments::<u32> {
                                upward_messages: Default::default(),
                                horizontal_messages: Default::default(),
                                new_validation_code: None,
                                head_data: prev_head.clone(),
                                processed_downward_messages: 0,
                                hrmp_watermark: self.relay_parent_number() + 1,
                            },
                        };

                        if true /* self.candidate_descriptor_v2 */ {
                            // `UMPSignal` separator.
                            candidate.commitments.upward_messages.force_push(UMP_SEPARATOR);

                            // `SelectCore` commitment.
                            // Claim queue offset must be `0` so this candidate is for the very
                            // next block.
                            candidate.commitments.upward_messages.force_push(
                                UMPSignal::SelectCore(
                                    CoreSelector(chain_idx as u8),
                                    ClaimQueueOffset(0),
                                )
                                    .encode(),
                            );
                        }

                        let candidate_hash = candidate.hash();

                        let validity_votes: Vec<_> = group_validators
                            .iter()
                            .take(*num_votes as usize)
                            .map(|val_idx| {
                                let public = validators_shuffled.get(*val_idx).unwrap();

                                let signature_ctx = SigningContext {
                                    parent_hash: Self::header(Self::block_number()).hash(),
                                    session_index: Session::current_index(),
                                };
                                let sig = UncheckedSigned::<CompactStatement>::benchmark_sign(
                                    public,
                                    CompactStatement::Valid(candidate_hash),
                                    &signature_ctx,
                                    *val_idx,
                                )
                                .benchmark_signature();

                                ValidityAttestation::Explicit(sig.clone())
                            })
                            .collect();

                        // Check if the elastic scaling bit is set, if so we need to supply the core
                        // index in the generated candidate.
                        let core_idx = runtime_parachains::configuration::ActiveConfig::<T>::get()
                            .node_features
                            .get(FeatureIndex::ElasticScalingMVP as usize)
                            .map(|_the_bit| core_idx);

                        BackedCandidate::<T::Hash>::new(
                            candidate,
                            validity_votes,
                            bitvec::bitvec![u8, bitvec::order::Lsb0; 1; group_validators.len()],
                            core_idx,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Get the group assigned to a specific core by index at the current block number. Result
    /// undefined if the core index is unknown or the block number is less than the session start
    /// index.
    pub(crate) fn group_assigned_to_core(
        core: CoreIndex,
        at: BlockNumberFor<T>,
    ) -> Option<GroupIndex> {
        let config = runtime_parachains::configuration::ActiveConfig::<T>::get();
        let session_start_block = runtime_parachains::scheduler::SessionStartBlock::<T>::get();

        if at < session_start_block {
            return None;
        }

        let validator_groups = runtime_parachains::scheduler::ValidatorGroups::<T>::get();

        if core.0 as usize >= validator_groups.len() {
            return None;
        }

        let rotations_since_session_start: BlockNumberFor<T> =
            (at - session_start_block) / config.scheduler_params.group_rotation_frequency;

        let rotations_since_session_start =
            <BlockNumberFor<T> as TryInto<u32>>::try_into(rotations_since_session_start)
                .unwrap_or(0);
        // Error case can only happen if rotations occur only once every u32::max(),
        // so functionally no difference in behavior.

        let group_idx =
            (core.0 as usize + rotations_since_session_start as usize) % validator_groups.len();
        Some(GroupIndex(group_idx as u32))
    }

    /// Get the validators in the given group, if the group index is valid for this session.
    pub(crate) fn group_validators(group_index: GroupIndex) -> Option<Vec<ValidatorIndex>> {
        runtime_parachains::scheduler::ValidatorGroups::<T>::get()
            .get(group_index.0 as usize)
            .cloned()
    }

    pub fn heads_insert(para_id: &ParaId, head_data: HeadData) {
        runtime_parachains::paras::Heads::<T>::insert(para_id, head_data);
    }

    /// Build a scenario for testing.
    ///
    /// Note that this API only allows building scenarios where the `backed_and_concluding_paras`
    /// are mutually exclusive with the cores for disputes. So
    /// `backed_and_concluding_paras.len() + dispute_sessions.len() + backed_in_inherent_paras` must
    /// be less than the max number of cores.
    pub(crate) fn build(self) -> ParachainsInherentData<HeaderFor<T>> {
        let current_session = runtime_parachains::shared::CurrentSessionIndex::<T>::get();
        // We need to refetch validators since they have been shuffled.
        let validators = runtime_parachains::session_info::Sessions::<T>::get(current_session)
            .unwrap()
            .validators
            .clone();

        let max_cores = self.max_cores() as usize;

        let used_cores =
            self.backed_and_concluding_paras.len() + self.backed_in_inherent_paras.len();
        assert!(used_cores <= max_cores);
        let mut backed_in_inherent = BTreeMap::new();
        backed_in_inherent.append(&mut self.backed_and_concluding_paras.clone());
        backed_in_inherent.append(&mut self.backed_in_inherent_paras.clone());
        let backed_candidates = self.create_backed_candidates(&backed_in_inherent);
        let used_cores_set = (0..used_cores).into_iter().map(|x| x as u32).collect();

        let availability_bitvec = Self::availability_bitvec(&used_cores_set, max_cores);

        let bitfields: Vec<UncheckedSigned<AvailabilityBitfield>> = validators
            .iter()
            .enumerate()
            .map(|(i, public)| {
                UncheckedSigned::<AvailabilityBitfield>::benchmark_sign(
                    public,
                    availability_bitvec.clone(),
                    &SigningContext {
                        parent_hash: Self::header(Self::block_number()).hash(),
                        session_index: Session::current_index(),
                    },
                    ValidatorIndex(i as u32),
                )
            })
            .collect();

        ParachainsInherentData {
            bitfields,
            backed_candidates,
            disputes: vec![],
            parent_header: Self::header(Self::block_number()),
        }
    }

    pub(crate) fn block_number() -> BlockNumberFor<T> {
        frame_system::Pallet::<T>::block_number()
    }
}

use {
    cumulus_primitives_core::relay_chain::SchedulerParams, frame_support::StorageHasher,
    tp_traits::ParathreadParams,
};

pub fn storage_map_final_key<H: frame_support::StorageHasher>(
    pallet_prefix: &str,
    map_name: &str,
    key: &[u8],
) -> Vec<u8> {
    let key_hashed = H::hash(key);
    let pallet_prefix_hashed = frame_support::Twox128::hash(pallet_prefix.as_bytes());
    let storage_prefix_hashed = frame_support::Twox128::hash(map_name.as_bytes());

    let mut final_key = Vec::with_capacity(
        pallet_prefix_hashed.len() + storage_prefix_hashed.len() + key_hashed.as_ref().len(),
    );

    final_key.extend_from_slice(&pallet_prefix_hashed[..]);
    final_key.extend_from_slice(&storage_prefix_hashed[..]);
    final_key.extend_from_slice(key_hashed.as_ref());

    final_key
}

pub fn set_dummy_boot_node(para_manager: RuntimeOrigin, para_id: ParaId) {
    use {
        crate::{PreserversAssignmentPaymentExtra, PreserversAssignmentPaymentRequest},
        pallet_data_preservers::{ParaIdsFilter, Profile, ProfileMode},
    };

    let profile = Profile {
        url:
            b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9"
                .to_vec()
                .try_into()
                .expect("to fit in BoundedVec"),
        para_ids: ParaIdsFilter::AnyParaId,
        mode: ProfileMode::Bootnode,
        assignment_request: PreserversAssignmentPaymentRequest::Free,
    };

    let profile_id = pallet_data_preservers::NextProfileId::<Runtime>::get();
    let profile_owner = AccountId::new([1u8; 32]);
    DataPreservers::force_create_profile(RuntimeOrigin::root(), profile, profile_owner)
        .expect("profile create to succeed");

    DataPreservers::start_assignment(
        para_manager,
        profile_id,
        para_id,
        PreserversAssignmentPaymentExtra::Free,
    )
    .expect("assignment to work");

    assert!(
        pallet_data_preservers::Assignments::<Runtime>::get(para_id).contains(&profile_id),
        "profile should be correctly assigned"
    );
}
use milagro_bls::Keypair;
pub fn generate_ethereum_pub_keys(n: u32) -> Vec<Keypair> {
    let mut keys = vec![];

    for _i in 0..n {
        let keypair = Keypair::random(&mut rand::thread_rng());
        keys.push(keypair);
    }
    keys
}

use babe_primitives::AuthorityPair as BabeAuthorityPair;
use grandpa_primitives::{
    AuthorityPair as GrandpaAuthorityPair, Equivocation, EquivocationProof, RoundNumber, SetId,
};
use primitives::{CandidateDescriptor, CandidateHash, CollatorId, CollatorSignature};
use primitives::vstaging::{ClaimQueueOffset, CoreSelector, UMPSignal, UMP_SEPARATOR};
use runtime_parachains::inclusion::CandidatePendingAvailability;
use sp_core::{ByteArray, H256};
pub fn generate_grandpa_equivocation_proof(
    set_id: SetId,
    vote1: (RoundNumber, H256, u32, &GrandpaAuthorityPair),
    vote2: (RoundNumber, H256, u32, &GrandpaAuthorityPair),
) -> EquivocationProof<H256, u32> {
    let signed_prevote = |round, hash, number, authority_pair: &GrandpaAuthorityPair| {
        let prevote = finality_grandpa::Prevote {
            target_hash: hash,
            target_number: number,
        };

        let prevote_msg = finality_grandpa::Message::Prevote(prevote.clone());
        let payload = grandpa_primitives::localized_payload(round, set_id, &prevote_msg);
        let signed = authority_pair.sign(&payload);
        (prevote, signed)
    };

    let (prevote1, signed1) = signed_prevote(vote1.0, vote1.1, vote1.2, vote1.3);
    let (prevote2, signed2) = signed_prevote(vote2.0, vote2.1, vote2.2, vote2.3);

    EquivocationProof::new(
        set_id,
        Equivocation::Prevote(finality_grandpa::Equivocation {
            round_number: vote1.0,
            identity: vote1.3.public(),
            first: (prevote1, signed1),
            second: (prevote2, signed2),
        }),
    )
}

/// Creates an equivocation at the current block, by generating two headers.
pub fn generate_babe_equivocation_proof(
    offender_authority_pair: &BabeAuthorityPair,
) -> babe_primitives::EquivocationProof<crate::Header> {
    use babe_primitives::digests::CompatibleDigestItem;

    let current_digest = System::digest();
    let babe_predigest = current_digest
        .clone()
        .logs()
        .iter()
        .find_map(|log| log.as_babe_pre_digest());
    let slot_proof = babe_predigest.expect("babe should be presesnt").slot();

    let make_headers = || {
        (
            HeaderFor::<Runtime>::new(
                0,
                H256::default(),
                H256::default(),
                H256::default(),
                current_digest.clone(),
            ),
            HeaderFor::<Runtime>::new(
                1,
                H256::default(),
                H256::default(),
                H256::default(),
                current_digest.clone(),
            ),
        )
    };

    // sign the header prehash and sign it, adding it to the block as the seal
    // digest item
    let seal_header = |header: &mut crate::Header| {
        let prehash = header.hash();
        let seal = <DigestItem as CompatibleDigestItem>::babe_seal(
            offender_authority_pair.sign(prehash.as_ref()),
        );
        header.digest_mut().push(seal);
    };

    // generate two headers at the current block
    let (mut h1, mut h2) = make_headers();

    seal_header(&mut h1);
    seal_header(&mut h2);

    babe_primitives::EquivocationProof {
        slot: slot_proof,
        offender: offender_authority_pair.public(),
        first_header: h1,
        second_header: h2,
    }
}

use sp_core::Public;
use crate::weights::runtime_parachains_inclusion;

/// Helper function to generate a crypto pair from seed
pub fn get_pair_from_seed<TPublic: Public>(seed: &str) -> TPublic::Pair {
    let secret_uri = format!("//{}", seed);
    let pair = TPublic::Pair::from_string(&secret_uri, None).expect("static values are valid; qed");

    pair
}
