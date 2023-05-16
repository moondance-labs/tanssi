use tp_collator_assignment::AssignedCollators;

use {
    crate::{self as pallet_authority_assignment},
    frame_support::traits::{ConstU16, ConstU64},
    frame_system as system,
    parity_scale_codec::{Decode, Encode},
    sp_core::H256,
    sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
    },
    sp_std::collections::btree_map::BTreeMap,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        MockData: mock_data,
        AuthorityAssignment: pallet_authority_assignment,
    }
);

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

// Pallet to provide some mock data, used to test
#[frame_support::pallet]
pub mod mock_data {
    use {super::*, frame_support::pallet_prelude::*};

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn mock)]
    pub(super) type Mock<T: Config> = StorageValue<_, Mocks, ValueQuery>;

    impl<T: Config> Pallet<T> {
        pub fn get() -> Mocks {
            Mock::<T>::get()
        }
        pub fn mutate<F, R>(f: F) -> R
        where
            F: FnOnce(&mut Mocks) -> R,
        {
            Mock::<T>::mutate(f)
        }
    }
}

#[derive(Clone, Encode, Decode, PartialEq, sp_core::RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Mocks {
    pub nimbus_map: BTreeMap<u64, String>,
    pub next_collator_assignment: AssignedCollators<u64>,
}

impl Default for Mocks {
    fn default() -> Self {
        Self {
            nimbus_map: Default::default(),
            next_collator_assignment: Default::default(),
        }
    }
}

impl mock_data::Config for Test {}

// In tests, we ignore the session_index param, so changes to the configuration are instant

impl pallet_authority_assignment::Config for Test {
    type SessionIndex = u32;
    type AuthorityId = String;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

pub trait GetCollators<AccountId, SessionIndex> {
    fn collators(session_index: SessionIndex) -> Vec<AccountId>;
}

pub const SESSION_LEN: u64 = 5;

pub fn run_to_session(n: u32) {
    let block_number = SESSION_LEN * (n as u64);
    run_to_block(block_number + 1);
}

pub fn run_to_block(n: u64) {
    let old_block_number = System::block_number();

    for x in (old_block_number + 1)..=n {
        System::set_block_number(x);

        if x % SESSION_LEN == 1 {
            let session_index = (x / SESSION_LEN) as u32;
            let nimbus_map = &MockData::mock().nimbus_map;
            let next_collator_assignment = &MockData::mock().next_collator_assignment;
            AuthorityAssignment::initializer_on_new_session(
                &session_index,
                &nimbus_map,
                &next_collator_assignment,
            );
        }
    }
}
