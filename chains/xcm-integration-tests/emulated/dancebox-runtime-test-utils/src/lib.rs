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
    cumulus_primitives_core::{ParaId, PersistedValidationData},
    cumulus_primitives_parachain_inherent::ParachainInherentData,
    dancebox_runtime::{AuthorInherent, RuntimeOrigin},
    dp_consensus::runtime_decl_for_tanssi_authority_assignment_api::TanssiAuthorityAssignmentApi,
    frame_support::{
        assert_ok,
        traits::{OnFinalize, OnInitialize},
    },
    nimbus_primitives::{NimbusId, NIMBUS_ENGINE_ID},
    pallet_registrar_runtime_api::ContainerChainGenesisData,
    parity_scale_codec::{Decode, Encode, MaxEncodedLen},
    polkadot_parachain_primitives::primitives::HeadData,
    sp_consensus_aura::AURA_ENGINE_ID,
    sp_consensus_slots::Slot,
    sp_core::Get,
    sp_runtime::{traits::Dispatchable, Digest, DigestItem},
    sp_std::collections::btree_map::BTreeMap,
};

pub use dancebox_runtime::{
    AccountId, AssetRate, AuthorNoting, AuthorityAssignment, AuthorityMapping, Balance, Balances,
    CollatorAssignment, Configuration, DataPreservers, ForeignAssets, ForeignAssetsCreator,
    InflationRewards, Initializer, Invulnerables, MinimumSelfDelegation, ParachainInfo,
    PooledStaking, Proxy, ProxyType, Registrar, RewardsPortion, Runtime, RuntimeCall,
    ServicesPayment, Session, System, TransactionPayment,
};

// TODO: This module copy/pasted for now from dancebox/tests/common/mod.rs, should be extracted and re-used in both places

pub const UNIT: Balance = 1_000_000_000_000;

pub fn session_to_block(n: u32) -> u32 {
    let block_number = dancebox_runtime::Period::get() * n;

    // Add 1 because the block that emits the NewSession event cannot contain any extrinsics,
    // so this is the first block of the new session that can actually be used
    block_number + 1
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RunSummary {
    pub author_id: AccountId,
    pub inflation: Balance,
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

pub fn insert_authorities_and_slot_digests(slot: u64) {
    let authorities =
        Runtime::para_id_authorities(ParachainInfo::get()).expect("authorities should be set");

    let authority: NimbusId = authorities[slot as usize % authorities.len()].clone();

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

    insert_authorities_and_slot_digests(slot);

    // Initialize the new block
    CollatorAssignment::on_initialize(System::block_number());
    Session::on_initialize(System::block_number());
    Initializer::on_initialize(System::block_number());
    AuthorInherent::on_initialize(System::block_number());

    // `Initializer::on_finalize` needs to run at least one to have
    // author mapping setup.
    let author_id = current_author();

    let current_issuance = Balances::total_issuance();
    InflationRewards::on_initialize(System::block_number());
    let new_issuance = Balances::total_issuance();

    frame_support::storage::unhashed::put(
        &frame_support::storage::storage_prefix(b"AsyncBacking", b"SlotInfo"),
        &(Slot::from(slot), 1),
    );

    pallet_author_inherent::Pallet::<Runtime>::kick_off_authorship_validation(None.into())
        .expect("author inherent to dispatch correctly");

    RunSummary {
        author_id,
        inflation: new_issuance - current_issuance,
    }
}

pub fn end_block() {
    let block_number = System::block_number();
    advance_block_state_machine(RunBlockState::End(block_number));
    // Finalize the block
    CollatorAssignment::on_finalize(System::block_number());
    Session::on_finalize(System::block_number());
    Initializer::on_finalize(System::block_number());
    AuthorInherent::on_finalize(System::block_number());
    TransactionPayment::on_finalize(System::block_number());
}

pub fn run_block() -> RunSummary {
    end_block();

    start_block()
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
    u64::from(
        pallet_async_backing::SlotInfo::<Runtime>::get()
            .unwrap_or_default()
            .0,
    )
}

pub fn current_author() -> AccountId {
    let current_session = Session::current_index();
    let mapping =
        pallet_authority_mapping::Pallet::<Runtime>::authority_id_mapping(current_session)
            .expect("there is a mapping for the current session");

    let author = pallet_author_inherent::Author::<Runtime>::get()
        .expect("there should be a registered author");

    mapping
        .get(&author)
        .expect("there is a mapping for the current author")
        .clone()
}

pub fn set_dummy_boot_node(para_manager: RuntimeOrigin, para_id: ParaId) {
    use {
        pallet_data_preservers::{ParaIdsFilter, Profile, ProfileMode},
        tp_data_preservers_common::{AssignerExtra, ProviderRequest},
    };

    let profile = Profile {
        url:
            b"/ip4/127.0.0.1/tcp/33049/ws/p2p/12D3KooWHVMhQDHBpj9vQmssgyfspYecgV6e3hH1dQVDUkUbCYC9"
                .to_vec()
                .try_into()
                .expect("to fit in BoundedVec"),
        para_ids: ParaIdsFilter::AnyParaId,
        mode: ProfileMode::Bootnode,
        assignment_request: ProviderRequest::Free,
    };

    let profile_id = pallet_data_preservers::NextProfileId::<Runtime>::get();
    let profile_owner = AccountId::new([1u8; 32]);
    DataPreservers::force_create_profile(RuntimeOrigin::root(), profile, profile_owner)
        .expect("profile create to succeed");

    DataPreservers::start_assignment(para_manager, profile_id, para_id, AssignerExtra::Free)
        .expect("assignement to work");

    assert!(
        pallet_data_preservers::Assignments::<Runtime>::get(para_id).contains(&profile_id),
        "profile should be correctly assigned"
    );
}
