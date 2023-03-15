use codec::Encode;
use frame_support::{
    assert_ok,
    dispatch::Dispatchable,
    traits::{GenesisBuild, OnFinalize, OnInitialize},
};
use sp_consensus_aura::AURA_ENGINE_ID;
use sp_runtime::{testing::UintAuthorityId, Digest, DigestItem, MultiSigner};

pub use test_runtime::{
    AccountId, Aura, Balance, Balances, Registrar, Runtime, RuntimeEvent, System,
};

pub fn rpc_run_to_block(n: u32) {
    while System::block_number() < n {
        System::set_block_number(System::block_number() + 1);
    }
}

/// Utility function that advances the chain to the desired block number.
/// If an author is provided, that author information is injected to all the blocks in the meantime.
pub fn run_to_block(n: u32, author: Option<AccountId>) {
    /*
    // Finalize the first block
    AuthorInherent::on_finalize(System::block_number());
    */
    while System::block_number() < n {
        // Set the new block number and author
        match &author {
            Some(_author) => {
                let slot = Aura::current_slot();
                let pre_digest = Digest {
                    logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                };
                System::reset_events();
                System::initialize(
                    &(System::block_number() + 1),
                    &System::parent_hash(),
                    &pre_digest,
                );
            }
            None => {
                System::set_block_number(System::block_number() + 1);
            }
        }

        /*
        // Initialize the new block
        AuthorInherent::on_initialize(System::block_number());
        ParachainStaking::on_initialize(System::block_number());

        // Finalize the block
        AuthorInherent::on_finalize(System::block_number());
        ParachainStaking::on_finalize(System::block_number());
        */
    }
}

pub fn last_event() -> RuntimeEvent {
    System::events().pop().expect("Event expected").event
}

pub struct ExtBuilder {
    // endowed accounts with balances
    balances: Vec<(AccountId, Balance)>,
    // [collator, amount]
    collators: Vec<(AccountId, Balance)>,
    // list of registered para ids
    para_ids: Vec<u32>,
}

impl Default for ExtBuilder {
    fn default() -> ExtBuilder {
        ExtBuilder {
            balances: vec![],
            collators: vec![],
            para_ids: vec![],
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

    pub fn with_para_ids(mut self, para_ids: Vec<u32>) -> Self {
        self.para_ids = para_ids;
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        pallet_balances::GenesisConfig::<Runtime> {
            balances: self.balances,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        // TODO: set invulnerables in pallet_authorship

        pallet_aura::GenesisConfig::<Runtime> {
            authorities: self
                .collators
                .into_iter()
                .map(|(a, _b)| {
                    let a_slice: &[u8] = a.as_ref();
                    // TODO: this match is just to avoid importing the Public type, fix this
                    match MultiSigner::Sr25519(a_slice.try_into().unwrap()) {
                        MultiSigner::Sr25519(x) => x.into(),
                        _ => unreachable!(),
                    }
                })
                .collect(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        <pallet_registrar::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
            &pallet_registrar::GenesisConfig {
                para_ids: self.para_ids,
            },
            &mut t,
        )
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);

        ext.execute_with(|| {
            System::set_block_number(1);
        });
        ext
    }
}

pub const ALICE: [u8; 32] = [4u8; 32];
pub const BOB: [u8; 32] = [5u8; 32];
pub const CHARLIE: [u8; 32] = [6u8; 32];
pub const DAVE: [u8; 32] = [7u8; 32];

pub fn origin_of(account_id: AccountId) -> <Runtime as frame_system::Config>::RuntimeOrigin {
    <Runtime as frame_system::Config>::RuntimeOrigin::signed(account_id)
}

pub fn inherent_origin() -> <Runtime as frame_system::Config>::RuntimeOrigin {
    <Runtime as frame_system::Config>::RuntimeOrigin::none()
}

pub fn root_origin() -> <Runtime as frame_system::Config>::RuntimeOrigin {
    <Runtime as frame_system::Config>::RuntimeOrigin::root()
}

/// Mock the inherent that sets validation data in ParachainSystem, which
/// contains the `relay_chain_block_number`, which is used in `author-filter` as a
/// source of randomness to filter valid authors at each block.
pub fn set_parachain_inherent_data() {
    // TODO
    /*
    use cumulus_primitives_core::PersistedValidationData;
    use cumulus_test_relay_sproof_builder::RelayStateSproofBuilder;
    let (relay_parent_storage_root, relay_chain_state) =
        RelayStateSproofBuilder::default().into_state_root_and_proof();
    let vfp = PersistedValidationData {
        relay_parent_number: 1u32,
        relay_parent_storage_root,
        ..Default::default()
    };
    let parachain_inherent_data = ParachainInherentData {
        validation_data: vfp,
        relay_chain_state: relay_chain_state,
        downward_messages: Default::default(),
        horizontal_messages: Default::default(),
    };
    assert_ok!(RuntimeCall::ParachainSystem(
        cumulus_pallet_parachain_system::Call::<Runtime>::set_validation_data {
            data: parachain_inherent_data
        }
    )
    .dispatch(inherent_origin()));
    */
}
