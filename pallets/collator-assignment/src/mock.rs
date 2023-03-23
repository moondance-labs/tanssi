use crate::{self as pallet_collator_assignment};
use frame_support::traits::{ConstU16, ConstU64};
use frame_system as system;
use parity_scale_codec::{Decode, Encode};
use sp_core::{ConstU32, H256};
use sp_runtime::{
    testing::{Header, UintAuthorityId},
    traits::{BlakeTwo256, IdentityLookup},
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
        CollatorAssignment: pallet_collator_assignment,
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
    use super::*;
    use frame_support::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
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
    pub orchestrator_chain_collators: u32,
    pub collators_per_container: u32,
    pub collators: Vec<u64>,
    pub container_chains: Vec<u32>,
}

impl Default for Mocks {
    fn default() -> Self {
        Self {
            orchestrator_chain_collators: 0,
            collators_per_container: 0,
            collators: vec![],
            container_chains: vec![],
        }
    }
}

impl mock_data::Config for Test {}

pub struct HostConfigurationGetter;

impl pallet_collator_assignment::GetHostConfiguration for HostConfigurationGetter {
    fn orchestrator_chain_collators() -> u32 {
        MockData::mock().orchestrator_chain_collators
    }

    fn collators_per_container() -> u32 {
        MockData::mock().collators_per_container
    }
}

pub struct CollatorsGetter;

impl pallet_collator_assignment::GetCollators<u64> for CollatorsGetter {
    fn collators() -> Vec<u64> {
        MockData::mock().collators.clone()
    }
}

pub struct ContainerChainsGetter;

impl pallet_collator_assignment::GetContainerChains for ContainerChainsGetter {
    fn container_chains() -> Vec<u32> {
        MockData::mock().container_chains.clone()
    }
}

pub struct CurrentSessionIndexGetter;

impl pallet_collator_assignment::GetSessionIndex<u32> for CurrentSessionIndexGetter {
    /// Returns current session index.
    fn session_index() -> u32 {
        // For tests, let 1 session be 5 blocks
        (System::block_number() / 5) as u32
    }
}

impl pallet_collator_assignment::Config for Test {
    type SessionIndex = u32;
    type AuthorityId = UintAuthorityId;
    type SelfParaId = ConstU32<999>;
    type HostConfiguration = HostConfigurationGetter;
    type Collators = CollatorsGetter;
    type ContainerChains = ContainerChainsGetter;
    type CurrentSessionIndex = CurrentSessionIndexGetter;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

pub fn run_to_block(n: u64) {
    let old_block_number = System::block_number();
    let session_len = 5;

    for x in (old_block_number + 1)..=n {
        System::set_block_number(x);

        if x % session_len == 1 {
            let session_index = (x / session_len) as u32;
            CollatorAssignment::initializer_on_new_session(&session_index);
        }
    }
}
