use codec::Encode;
use frame_support::traits::{GenesisBuild, OnFinalize, OnInitialize};
use sp_consensus_aura::AURA_ENGINE_ID;
use sp_core::Pair;
use sp_runtime::{Digest, DigestItem};

pub use test_runtime::{
    AccountId, Aura, AuraId, Authorship, Balance, Balances, Registrar, Runtime, RuntimeEvent,
    Session, System,
};

pub fn run_to_session(n: u32) {
    let block_number = test_runtime::Period::get() * n;
    run_to_block(block_number + 1, false);
}

/// Utility function that advances the chain to the desired block number.
/// If add_author is true, the author information is injected to all the blocks in the meantime.
pub fn run_to_block(n: u32, add_author: bool) {
    /*
    // Finalize the first block
    AuthorInherent::on_finalize(System::block_number());
    */
    while System::block_number() < n {
        // Set the new block number and author
        if add_author {
            let slot = Aura::current_slot();
            let pre_digest = Digest {
                logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, (slot + 1).encode())],
            };
            System::reset_events();
            System::initialize(
                &(System::block_number() + 1),
                &System::parent_hash(),
                &pre_digest,
            );
        } else {
            System::set_block_number(System::block_number() + 1);
        }

        // Initialize the new block

        Session::on_initialize(System::block_number());
        Aura::on_initialize(System::block_number());
        Authorship::on_initialize(System::block_number());

        // Finalize the block
        Session::on_finalize(System::block_number());
        Aura::on_finalize(System::block_number());
        Authorship::on_finalize(System::block_number());
    }
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

        if !self.collators.is_empty() {
            // We set invulnerables in pallet_collator_selection
            let invulnerables: Vec<AccountId> = self
                .collators
                .clone()
                .into_iter()
                .map(|(account, _balance)| account)
                .collect();

            pallet_collator_selection::GenesisConfig::<Runtime> {
                invulnerables: invulnerables.clone(),
                candidacy_bond: Default::default(),
                desired_candidates: invulnerables.len() as u32,
            }
            .assimilate_storage(&mut t)
            .unwrap();

            // But we also initialize their keys in the session pallet
            let keys: Vec<_> = self
                .collators
                .into_iter()
                .map(|(account, _balance)| {
                    let aura_id = get_aura_id_from_seed(&account.to_string());
                    (
                        account.clone(),
                        account,
                        test_runtime::SessionKeys {
                            aura: aura_id.clone(),
                            config: aura_id,
                        },
                    )
                })
                .collect();
            <pallet_session::GenesisConfig<Runtime> as GenesisBuild<Runtime>>::assimilate_storage(
                &pallet_session::GenesisConfig { keys: keys },
                &mut t,
            )
            .unwrap();
        }

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

pub fn root_origin() -> <Runtime as frame_system::Config>::RuntimeOrigin {
    <Runtime as frame_system::Config>::RuntimeOrigin::root()
}

/// Helper function to generate a crypto pair from seed
pub fn get_aura_id_from_seed(seed: &str) -> AuraId {
    sp_core::sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
        .into()
}
