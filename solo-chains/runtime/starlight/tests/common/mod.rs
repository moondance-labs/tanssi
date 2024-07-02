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

use {
    crate::UNIT,
    cumulus_primitives_core::{ParaId, PersistedValidationData},
    frame_support::{
        assert_ok,
        traits::{OnFinalize, OnInitialize},
    },
    //     nimbus_primitives::{NimbusId, NIMBUS_ENGINE_ID},
    //     pallet_collator_assignment_runtime_api::runtime_decl_for_collator_assignment_api::CollatorAssignmentApi,
    pallet_registrar_runtime_api::ContainerChainGenesisData,
    //     pallet_services_payment::{ProvideBlockProductionCost, ProvideCollatorAssignmentCost},
    parity_scale_codec::{Decode, Encode, MaxEncodedLen},
    //     polkadot_parachain_primitives::primitives::HeadData,
    //     sp_consensus_aura::AURA_ENGINE_ID,
    //     sp_consensus_slots::Slot,
    //     sp_core::{Get, Pair},
    sp_runtime::{traits::{Dispatchable,SaturatedConversion,}, BuildStorage, Digest, DigestItem},
    //     sp_std::collections::btree_map::BTreeMap,
    //     test_relay_sproof_builder::ParaHeaderSproofBuilder,
    //     cumulus_primitives_parachain_inherent::ParachainInherentData,
    //     dp_consensus::runtime_decl_for_tanssi_authority_assignment_api::TanssiAuthorityAssignmentApi,
    starlight_runtime::MaxLengthTokenSymbol,
};

pub use starlight_runtime::{
    AccountId, Babe, Balance, Balances, Initializer, Runtime, Session, System, TransactionPayment,
};

pub fn session_to_block(n: u32) -> u32 {
    // let block_number = flashbox_runtime::Period::get() * n;
    let block_number = Babe::current_epoch().duration.saturated_into::<u32>() * n;

    // Add 1 because the block that emits the NewSession event cannot contain any extrinsics,
    // so this is the first block of the new session that can actually be used
    block_number + 1
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
        let summary = run_block();
    }
}

// Used to create the next block inherent data
#[derive(Clone, Encode, Decode, Default, PartialEq, Debug, scale_info::TypeInfo, MaxEncodedLen)]
pub struct MockInherentData {
    pub random_seed: Option<[u8; 32]>,
}

fn take_new_inherent_data() -> Option<MockInherentData> {
    let data: Option<MockInherentData> =
        frame_support::storage::unhashed::take(b"__mock_new_inherent_data");

    data
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

    let mut slot = current_slot() + 1;
    if block_number == 0 {
        // Hack to avoid breaking all tests. When the current block is 1, the slot number should be
        // 1. But all of our tests assume it will be 0. So use slot number = block_number - 1.
        slot = 0;
    }

    // let maybe_mock_inherent = take_new_inherent_data();

    // if let Some(mock_inherent_data) = maybe_mock_inherent {
    //     set_parachain_inherent_data(mock_inherent_data);
    // }

    // insert_authorities_and_slot_digests(slot);

    // Initialize the new block
    Babe::on_initialize(System::block_number());
    // CollatorAssignment::on_initialize(System::block_number());
    Session::on_initialize(System::block_number());
    Initializer::on_initialize(System::block_number());
    // AuthorInherent::on_initialize(System::block_number());

    // `Initializer::on_finalize` needs to run at least one to have
    // author mapping setup.
    // let author_id = current_author();

    // let current_issuance = Balances::total_issuance();
    // InflationRewards::on_initialize(System::block_number());
    // let new_issuance = Balances::total_issuance();

    // frame_support::storage::unhashed::put(
    //     &frame_support::storage::storage_prefix(b"AsyncBacking", b"SlotInfo"),
    //     // TODO: this should be 0?
    //     &(Slot::from(slot), 1),
    // );

    // pallet_author_inherent::Pallet::<Runtime>::kick_off_authorship_validation(None.into())
    //     .expect("author inherent to dispatch correctly");
}

pub fn end_block() {
    let block_number = System::block_number();
    advance_block_state_machine(RunBlockState::End(block_number));
    // Finalize the block
    Babe::on_finalize(System::block_number());
    // CollatorAssignment::on_finalize(System::block_number());
    Session::on_finalize(System::block_number());
    Initializer::on_finalize(System::block_number());
    // AuthorInherent::on_finalize(System::block_number());
    TransactionPayment::on_finalize(System::block_number());
}

pub fn run_block() {
    end_block();

    start_block()
}

#[derive(Default, Clone)]
pub struct ParaRegistrationParams {
    para_id: u32,
    genesis_data: ContainerChainGenesisData<MaxLengthTokenSymbol>,
    block_production_credits: u32,
    collator_assignment_credits: u32,
}

impl
    From<(
        u32,
        ContainerChainGenesisData<MaxLengthTokenSymbol>,
        u32,
        u32,
    )> for ParaRegistrationParams
{
    fn from(
        value: (
            u32,
            ContainerChainGenesisData<MaxLengthTokenSymbol>,
            u32,
            u32,
        ),
    ) -> Self {
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
    // [collator, amount]
    collators: Vec<(AccountId, Balance)>,
    // sudo key
    sudo: Option<AccountId>,
    // list of registered para ids: para_id, genesis_data, boot_nodes, block_credits, session_credits
    para_ids: Vec<ParaRegistrationParams>,
    // configuration to apply
    config: pallet_configuration::HostConfiguration,
    own_para_id: Option<ParaId>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            balances: vec![
                // Alice gets 10k extra tokens for her mapping deposit
                (AccountId::from(ALICE), 210_000 * UNIT),
                (AccountId::from(BOB), 100_000 * UNIT),
            ],
            collators: vec![
                (AccountId::from(ALICE), 210 * UNIT),
                (AccountId::from(BOB), 100 * UNIT),
            ],
            sudo: Default::default(),
            para_ids: Default::default(),
            config: default_config(),
            own_para_id: Default::default(),
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

    pub fn with_collators(mut self, collators: Vec<(AccountId, Balance)>) -> Self {
        self.collators = collators;
        self
    }

    pub fn with_para_ids(mut self, para_ids: Vec<ParaRegistrationParams>) -> Self {
        self.para_ids = para_ids;
        self
    }

    pub fn with_config(mut self, config: pallet_configuration::HostConfiguration) -> Self {
        self.config = config;
        self
    }

    pub fn build_storage(self) -> sp_core::storage::Storage {
        let mut t = frame_system::GenesisConfig::<Runtime>::default()
            .build_storage()
            .unwrap();

        pallet_balances::GenesisConfig::<Runtime> {
            balances: self.balances,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        // // We need to initialize these pallets first. When initializing pallet-session,
        // // these values will be taken into account for collator-assignment.

        // pallet_registrar::GenesisConfig::<Runtime> {
        //     para_ids: self
        //         .para_ids
        //         .iter()
        //         .cloned()
        //         .map(|registered_para| {
        //             (registered_para.para_id.into(), registered_para.genesis_data)
        //         })
        //         .collect(),
        // }
        // .assimilate_storage(&mut t)
        // .unwrap();

        // pallet_services_payment::GenesisConfig::<Runtime> {
        //     para_id_credits: self
        //         .para_ids
        //         .clone()
        //         .into_iter()
        //         .map(|registered_para| {
        //             (
        //                 registered_para.para_id.into(),
        //                 registered_para.block_production_credits,
        //                 registered_para.collator_assignment_credits,
        //             )
        //                 .into()
        //         })
        //         .collect(),
        // }
        // .assimilate_storage(&mut t)
        // .unwrap();

        pallet_configuration::GenesisConfig::<Runtime> {
            config: self.config,
            ..Default::default()
        }
        .assimilate_storage(&mut t)
        .unwrap();

        // if let Some(own_para_id) = self.own_para_id {
        //     parachain_info::GenesisConfig::<Runtime> {
        //         parachain_id: own_para_id,
        //         ..Default::default()
        //     }
        //     .assimilate_storage(&mut t)
        //     .unwrap();
        // }

        // if !self.collators.is_empty() {
        //     // We set invulnerables in pallet_invulnerables
        //     let invulnerables: Vec<AccountId> = self
        //         .collators
        //         .clone()
        //         .into_iter()
        //         .map(|(account, _balance)| account)
        //         .collect();

        //     pallet_invulnerables::GenesisConfig::<Runtime> {
        //         invulnerables: invulnerables.clone(),
        //     }
        //     .assimilate_storage(&mut t)
        //     .unwrap();

        //     // But we also initialize their keys in the session pallet
        //     let keys: Vec<_> = self
        //         .collators
        //         .into_iter()
        //         .map(|(account, _balance)| {
        //             let nimbus_id = get_aura_id_from_seed(&account.to_string());
        //             (
        //                 account.clone(),
        //                 account,
        //                 starlight_runtime::SessionKeys { nimbus: nimbus_id },
        //             )
        //         })
        //         .collect();
        //     pallet_session::GenesisConfig::<Runtime> { keys }
        //         .assimilate_storage(&mut t)
        //         .unwrap();
        // }
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
            // set_parachain_inherent_data(Default::default());
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

pub fn empty_genesis_data() -> ContainerChainGenesisData<MaxLengthTokenSymbol> {
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
