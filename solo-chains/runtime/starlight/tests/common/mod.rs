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
    frame_support::traits::{OnFinalize, OnInitialize},
    pallet_registrar_runtime_api::ContainerChainGenesisData,
    parity_scale_codec::{Decode, Encode, MaxEncodedLen},
    sp_runtime::{traits::SaturatedConversion, BuildStorage, Digest, DigestItem},
    starlight_runtime::MaxLengthTokenSymbol,
};

pub use starlight_runtime::{
    genesis_config_presets::get_authority_keys_from_seed, AccountId, Babe, Balance, Balances,
    CollatorConfiguration, Initializer, Runtime, Session, System, TransactionPayment,
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

        pallet_configuration::GenesisConfig::<Runtime> {
            config: self.config,
            ..Default::default()
        }
        .assimilate_storage(&mut t)
        .unwrap();

        if !self.validators.is_empty() {
            let keys: Vec<_> = self
                .validators
                .into_iter()
                .map(|(account, _balance)| {
                    let authority_keys = get_authority_keys_from_seed(&account.to_string());
                    (
                        account.clone(),
                        account,
                        starlight_runtime::SessionKeys {
                            babe: authority_keys.2.clone(),
                            grandpa: authority_keys.3.clone(),
                            para_validator: authority_keys.4.clone(),
                            para_assignment: authority_keys.5.clone(),
                            authority_discovery: authority_keys.6.clone(),
                            beefy: authority_keys.7.clone(),
                        },
                    )
                })
                .collect();
            pallet_session::GenesisConfig::<Runtime> { keys }
                .assimilate_storage(&mut t)
                .unwrap();
        }

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

pub fn current_slot() -> u64 {
    Babe::current_slot().into()
}

pub const ALICE: [u8; 32] = [4u8; 32];
pub const BOB: [u8; 32] = [5u8; 32];
pub const CHARLIE: [u8; 32] = [6u8; 32];
pub const DAVE: [u8; 32] = [7u8; 32];
pub const EVE: [u8; 32] = [8u8; 32];
pub const FERDIE: [u8; 32] = [9u8; 32];
