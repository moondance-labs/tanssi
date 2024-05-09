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
    container_chain_template_frontier_runtime::AuthorInherent,
    cumulus_primitives_core::{ParaId, PersistedValidationData},
    cumulus_primitives_parachain_inherent::ParachainInherentData,
    frame_support::{
        assert_ok,
        traits::{OnFinalize, OnInitialize},
    },
    nimbus_primitives::{NimbusId, NIMBUS_ENGINE_ID},
    parity_scale_codec::{Decode, Encode, MaxEncodedLen},
    polkadot_parachain_primitives::primitives::HeadData,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_consensus_slots::Slot,
    sp_core::Pair,
    sp_runtime::{traits::Dispatchable, BuildStorage, Digest, DigestItem},
    sp_std::collections::btree_map::BTreeMap,
};

pub use container_chain_template_frontier_runtime::{
    AccountId, Balance, Balances, Runtime, RuntimeCall, System,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RunSummary {
    pub author_id: AccountId,
}

/* pub fn run_to_session(n: u32) {
    run_to_block(session_to_block(n));
} */

/// Utility function that advances the chain to the desired block number.
///
/// After this function returns, the current block number will be `n`, and the block will be "open",
/// meaning that on_initialize has been executed, but on_finalize has not. To execute on_finalize as
/// well, for example to test a runtime api, manually call `end_block` after this, run the test, and
/// call `start_block` to ensure that this function keeps working as expected.
/// Extrinsics should always be executed before on_finalize.
pub fn _run_to_block(n: u32) -> BTreeMap<u32, RunSummary> {
    let current_block_number = System::block_number();
    assert!(
        current_block_number < n,
        "run_to_block called with block {} when current block is {}",
        n,
        current_block_number
    );
    let mut summaries = BTreeMap::new();

    while System::block_number() < n {
        let summary = _run_block();
        let block_number = System::block_number();
        summaries.insert(block_number, summary);
    }

    summaries
}

pub fn insert_digests(slot: u64) {
    let authority: NimbusId = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
    let pre_digest = Digest {
        logs: vec![
            DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode()),
            DigestItem::PreRuntime(NIMBUS_ENGINE_ID, authority.encode()),
        ],
    };

    System::reset_events();
    System::initialize(
        &(System::block_number() + 1),
        &System::parent_hash(),
        &pre_digest,
    );
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

fn _set_new_inherent_data(data: MockInherentData) {
    frame_support::storage::unhashed::put(b"__mock_new_inherent_data", &Some(data));
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
    let mut slot = current_slot() + 1;
    if block_number == 0 {
        // Hack to avoid breaking all tests. When the current block is 1, the slot number should be
        // 1. But all of our tests assume it will be 0. So use slot number = block_number - 1.
        slot = 0;
    }

    let maybe_mock_inherent = take_new_inherent_data();

    if let Some(mock_inherent_data) = maybe_mock_inherent {
        set_parachain_inherent_data(mock_inherent_data);
    }

    // Set authorities of pallet_cc_authorities_noting
    set_authorities_inherent();

    // Nimbus and Aura digests
    insert_digests(slot);

    // Initialize the new block
    AuthorInherent::on_initialize(System::block_number());

    frame_support::storage::unhashed::put(
        &frame_support::storage::storage_prefix(b"AsyncBacking", b"SlotInfo"),
        &(Slot::from(slot), 1),
    );

    pallet_author_inherent::Pallet::<Runtime>::kick_off_authorship_validation(None.into())
        .expect("author inherent to dispatch correctly");

    RunSummary {
        author_id: AccountId::from(ALICE),
    }
}

pub fn _end_block() {
    let block_number = System::block_number();
    advance_block_state_machine(RunBlockState::End(block_number));
    // Finalize the block
    //CollatorAssignment::on_finalize(System::block_number());
    //Session::on_finalize(System::block_number());
    //Initializer::on_finalize(System::block_number());
    AuthorInherent::on_finalize(System::block_number());
    //TransactionPayment::on_finalize(System::block_number());
}

pub fn _run_block() -> RunSummary {
    _end_block();
    let summary = start_block();

    summary
}

/// Mock the inherent that sets validation data in ParachainSystem, which
/// contains the `relay_chain_block_number`, which is used in `collator-assignment` as a
/// source of randomness.
pub fn set_parachain_inherent_data(mock_inherent_data: MockInherentData) {
    use {
        cumulus_primitives_core::relay_chain::well_known_keys,
        cumulus_test_relay_sproof_builder::RelayStateSproofBuilder,
    };

    let relay_sproof = RelayStateSproofBuilder {
        para_id: 100u32.into(),
        included_para_head: Some(HeadData(vec![1, 2, 3])),
        current_slot: (current_slot()).into(),
        additional_key_values: if mock_inherent_data.random_seed.is_some() {
            vec![(
                well_known_keys::CURRENT_BLOCK_RANDOMNESS.to_vec(),
                Some(mock_inherent_data.random_seed).encode(),
            )]
        } else {
            vec![]
        },
        ..Default::default()
    };

    let (relay_parent_storage_root, relay_chain_state) = relay_sproof.into_state_root_and_proof();
    let vfp = PersistedValidationData {
        relay_parent_number: 1u32,
        relay_parent_storage_root,
        ..Default::default()
    };
    let parachain_inherent_data = ParachainInherentData {
        validation_data: vfp,
        relay_chain_state,
        downward_messages: Default::default(),
        horizontal_messages: Default::default(),
    };

    // Delete existing flag to avoid error
    // 'ValidationData must be updated only once in a block'
    // TODO: this is a hack
    frame_support::storage::unhashed::kill(&frame_support::storage::storage_prefix(
        b"ParachainSystem",
        b"ValidationData",
    ));

    assert_ok!(RuntimeCall::ParachainSystem(
        cumulus_pallet_parachain_system::Call::<Runtime>::set_validation_data {
            data: parachain_inherent_data
        }
    )
    .dispatch(inherent_origin()));
}

pub fn _set_parachain_inherent_data_random_seed(random_seed: [u8; 32]) {
    _set_new_inherent_data(MockInherentData {
        random_seed: Some(random_seed),
    });
}

pub struct ExtBuilder {
    // endowed accounts with balances
    balances: Vec<(AccountId, Balance)>,
    // [collator, amount]
    collators: Vec<(AccountId, Balance)>,
    // sudo key
    sudo: Option<AccountId>,
    safe_xcm_version: Option<u32>,
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
            safe_xcm_version: Default::default(),
            own_para_id: Default::default(),
        }
    }
}

impl ExtBuilder {
    pub fn with_balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
        self.balances = balances;
        self
    }

    pub fn with_collators(mut self, collators: Vec<(AccountId, Balance)>) -> Self {
        self.collators = collators;
        self
    }

    pub fn _with_sudo(mut self, sudo: AccountId) -> Self {
        self.sudo = Some(sudo);
        self
    }

    pub fn _with_safe_xcm_version(mut self, safe_xcm_version: u32) -> Self {
        self.safe_xcm_version = Some(safe_xcm_version);
        self
    }

    pub fn _with_own_para_id(mut self, own_para_id: ParaId) -> Self {
        self.own_para_id = Some(own_para_id);
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

        pallet_xcm::GenesisConfig::<Runtime> {
            safe_xcm_version: self.safe_xcm_version,
            ..Default::default()
        }
        .assimilate_storage(&mut t)
        .unwrap();

        if let Some(own_para_id) = self.own_para_id {
            parachain_info::GenesisConfig::<Runtime> {
                parachain_id: own_para_id,
                ..Default::default()
            }
            .assimilate_storage(&mut t)
            .unwrap();
        }

        pallet_sudo::GenesisConfig::<Runtime> { key: self.sudo }
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
        let t = self.build_storage();
        let mut ext = sp_io::TestExternalities::new(t);

        ext.execute_with(|| {
            // Start block 1
            start_block();
            set_parachain_inherent_data(Default::default());
        });
        ext
    }
}

pub fn root_origin() -> <Runtime as frame_system::Config>::RuntimeOrigin {
    <Runtime as frame_system::Config>::RuntimeOrigin::root()
}

pub fn _origin_of(account_id: AccountId) -> <Runtime as frame_system::Config>::RuntimeOrigin {
    <Runtime as frame_system::Config>::RuntimeOrigin::signed(account_id)
}

pub fn inherent_origin() -> <Runtime as frame_system::Config>::RuntimeOrigin {
    <Runtime as frame_system::Config>::RuntimeOrigin::none()
}

/// Helper function to generate a crypto pair from seed
pub fn get_aura_id_from_seed(seed: &str) -> NimbusId {
    sp_core::sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
        .into()
}

pub fn set_authorities_inherent() {
    let author = get_aura_id_from_seed(&AccountId::from(ALICE).to_string());
    assert_ok!(RuntimeCall::AuthoritiesNoting(
        pallet_cc_authorities_noting::Call::<Runtime>::set_authorities {
            authorities: vec![author]
        }
    )
    .dispatch(root_origin()));
}

pub fn current_slot() -> u64 {
    u64::from(
        pallet_async_backing::SlotInfo::<Runtime>::get()
            .unwrap_or_default()
            .0,
    )
}

pub const ALICE: [u8; 20] = [4u8; 20];
pub const BOB: [u8; 20] = [5u8; 20];
pub const CHARLIE: [u8; 20] = [6u8; 20];
pub const DAVE: [u8; 20] = [7u8; 20];
pub const _EVE: [u8; 20] = [8u8; 20];
pub const _FERDIE: [u8; 20] = [9u8; 20];
