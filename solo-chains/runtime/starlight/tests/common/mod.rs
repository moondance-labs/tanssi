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

use {
    crate::UNIT,
    babe_primitives::{
        digests::{PreDigest, SecondaryPlainPreDigest},
        BABE_ENGINE_ID,
    },
    cumulus_primitives_core::ParaId,
    frame_support::{
        assert_ok,
        traits::{OnFinalize, OnInitialize},
    },
    nimbus_primitives::NimbusId,
    pallet_registrar_runtime_api::ContainerChainGenesisData,
    parity_scale_codec::{Decode, Encode, MaxEncodedLen},
    runtime_parachains::paras_inherent as parachains_paras_inherent,
    sp_runtime::{
        traits::{Dispatchable, SaturatedConversion},
        BuildStorage, Digest, DigestItem,
    },
};

use cumulus_primitives_core::relay_chain::CollatorPair;
use runtime_parachains::paras::{ParaGenesisArgs, ParaKind};
use sp_core::Pair;
use sp_keystore::KeystoreExt;
use sp_keystore::KeystorePtr;
pub use starlight_runtime::{
    genesis_config_presets::get_authority_keys_from_seed, AccountId, Babe, Balance, Grandpa,
    Initializer, Runtime, RuntimeCall, Session, System, TanssiAuthorityAssignment,
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
    let maybe_mock_inherent = take_new_inherent_data();
    if let Some(mock_inherent_data) = maybe_mock_inherent {
        set_paras_inherent(mock_inherent_data);
    }
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
            relay_config: Default::default(),
            own_para_id: Default::default(),
            next_free_para_id: Default::default(),
            keystore: None,
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
                    (registered_para.para_id.into(), registered_para.genesis_data)
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
                    (
                        registered_para.para_id.into(),
                        ParaGenesisArgs {
                            validation_code: mock_validation_code(),
                            para_kind: ParaKind::Parachain,
                            genesis_head: HeadData::from(vec![0u8]),
                        },
                    )
                })
                .collect(),
            ..Default::default()
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

        runtime_parachains::configuration::GenesisConfig::<Runtime> {
            config: self.relay_config,
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
                    let authority_keys =
                        get_authority_keys_from_seed(&account.to_string(), self.keystore.as_ref());
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
                        let authority_keys =
                            get_authority_keys_from_seed(&account.to_string(), None);
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
        let t = self.clone().build_storage();
        let mut ext = sp_io::TestExternalities::new(t);
        if let Some(keystore) = self.keystore.clone() {
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

fn take_new_inherent_data() -> Option<cumulus_primitives_core::relay_chain::InherentData> {
    let data: Option<cumulus_primitives_core::relay_chain::InherentData> =
        frame_support::storage::unhashed::take(b"ParasInherent");

    data
}

pub fn set_new_inherent_data(data: cumulus_primitives_core::relay_chain::InherentData) {
    frame_support::storage::unhashed::put(b"ParasInherent", &data);
}

/// Mock the inherent that sets validation data in ParachainSystem, which
/// contains the `relay_chain_block_number`, which is used in `collator-assignment` as a
/// source of randomness.
pub fn set_paras_inherent(data: cumulus_primitives_core::relay_chain::InherentData) {
    // In order for this inherent to work, we need to match the parent header
    // the parent header does not play a significant role in the rest of the framework so
    // we are simply going to mock it
    System::set_parent_hash(data.parent_header.hash());
    assert_ok!(
        RuntimeCall::ParaInherent(parachains_paras_inherent::Call::<Runtime>::enter { data })
            .dispatch(inherent_origin())
    );
    frame_support::storage::unhashed::kill(&frame_support::storage::storage_prefix(
        b"ParaInherent",
        b"Included",
    ));
}

use bitvec::prelude::BitVec;
use cumulus_primitives_core::relay_chain::node_features::FeatureIndex;
use cumulus_primitives_core::relay_chain::{
    AvailabilityBitfield, BackedCandidate, CandidateCommitments, CandidateDescriptor,
    CommittedCandidateReceipt, CompactStatement, CoreIndex, GroupIndex, HeadData,
    InherentData as ParachainsInherentData, PersistedValidationData, SigningContext,
    UncheckedSigned, ValidationCode, ValidatorIndex, ValidityAttestation,
};
use frame_system::pallet_prelude::BlockNumberFor;
use frame_system::pallet_prelude::HeaderFor;
use sp_runtime::traits::Header;
use sp_runtime::traits::One;
use sp_runtime::traits::Zero;
use sp_std::collections::btree_map::BTreeMap;

pub(crate) struct ParasInherentTestBuilder<T: runtime_parachains::paras_inherent::Config> {
    /// Starting block number; we expect it to get incremented on session setup.
    block_number: BlockNumberFor<T>,
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
    _phantom: core::marker::PhantomData<T>,
}

pub fn mock_validation_code() -> ValidationCode {
    ValidationCode(vec![1, 2, 3])
}

#[allow(dead_code)]
impl<T: runtime_parachains::paras_inherent::Config> ParasInherentTestBuilder<T> {
    /// Create a new `BenchBuilder` with some opinionated values that should work with the rest
    /// of the functions in this implementation.
    pub(crate) fn new() -> Self {
        ParasInherentTestBuilder {
            block_number: Zero::zero(),
            backed_and_concluding_paras: Default::default(),
            backed_in_inherent_paras: Default::default(),
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

    /// Create an `AvailabilityBitfield` where `concluding` is a map where each key is a core index
    /// that is concluding and `cores` is the total number of cores in the system.
    fn availability_bitvec(used_cores: usize, cores: usize) -> AvailabilityBitfield {
        let mut bitfields = bitvec::bitvec![u8, bitvec::order::Lsb0; 0; 0];
        for i in 0..cores {
            if i < used_cores {
                bitfields.push(true);
            } else {
                bitfields.push(false)
            }
        }

        bitfields.into()
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
                        let payload =
                            cumulus_primitives_core::relay_chain::collator_signature_payload(
                                &relay_parent,
                                &para_id,
                                &persisted_validation_data_hash,
                                &pov_hash,
                                &validation_code_hash,
                            );

                        let collator_pair = CollatorPair::generate().0;

                        let signature = collator_pair.sign(&payload);

                        let group_validators = Self::group_validators(group_idx).unwrap();

                        let candidate = CommittedCandidateReceipt::<T::Hash> {
                            descriptor: CandidateDescriptor::<T::Hash> {
                                para_id,
                                relay_parent,
                                collator: collator_pair.public(),
                                persisted_validation_data_hash,
                                pov_hash,
                                erasure_root: Default::default(),
                                signature,
                                para_head: prev_head.hash(),
                                validation_code_hash,
                            },
                            commitments: CandidateCommitments::<u32> {
                                upward_messages: Default::default(),
                                horizontal_messages: Default::default(),
                                new_validation_code: None,
                                head_data: prev_head.clone(),
                                processed_downward_messages: 0,
                                hrmp_watermark: self.relay_parent_number() + 1,
                            },
                        };

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
            .map(|g| g.clone())
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

        let availability_bitvec = Self::availability_bitvec(used_cores, max_cores);

        let bitfields: Vec<UncheckedSigned<AvailabilityBitfield>> = validators
            .iter()
            .enumerate()
            .map(|(i, public)| {
                let unchecked_signed = UncheckedSigned::<AvailabilityBitfield>::benchmark_sign(
                    public,
                    availability_bitvec.clone(),
                    &SigningContext {
                        parent_hash: Self::header(Self::block_number()).hash(),
                        session_index: Session::current_index(),
                    },
                    ValidatorIndex(i as u32),
                );

                unchecked_signed
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
