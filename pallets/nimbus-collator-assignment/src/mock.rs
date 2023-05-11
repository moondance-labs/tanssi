use {
    crate::{self as pallet_collator_assignment},
    frame_support::traits::{ConstU16, ConstU64},
    frame_system as system,
    parity_scale_codec::{Decode, Encode},
    sp_core::H256,
    sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
    },
    tp_traits::ParaId,
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
    pub min_orchestrator_chain_collators: u32,
    pub max_orchestrator_chain_collators: u32,
    pub collators_per_container: u32,
    pub collators: Vec<u64>,
    pub container_chains: Vec<u32>,
}

impl Default for Mocks {
    fn default() -> Self {
        Self {
            min_orchestrator_chain_collators: 0,
            max_orchestrator_chain_collators: 0,
            collators_per_container: 0,
            collators: vec![],
            container_chains: vec![],
        }
    }
}

impl mock_data::Config for Test {}

// In tests, we ignore the session_index param, so changes to the configuration are instant

pub struct HostConfigurationGetter;

impl pallet_collator_assignment::GetHostConfiguration<u32> for HostConfigurationGetter {
    fn min_collators_for_orchestrator(_session_index: u32) -> u32 {
        MockData::mock().min_orchestrator_chain_collators
    }

    fn max_collators_for_orchestrator(_session_index: u32) -> u32 {
        MockData::mock().max_orchestrator_chain_collators
    }

    fn collators_per_container(_session_index: u32) -> u32 {
        MockData::mock().collators_per_container
    }
}

pub struct CollatorsGetter;

impl GetCollators<u64, u32> for CollatorsGetter {
    fn collators(_session_index: u32) -> Vec<u64> {
        MockData::mock().collators.clone()
    }
}

pub struct ContainerChainsGetter;

impl tp_traits::GetSessionContainerChains<u32> for ContainerChainsGetter {
    fn session_container_chains(_session_index: u32) -> Vec<ParaId> {
        MockData::mock()
            .container_chains
            .iter()
            .cloned()
            .map(|x| ParaId::from(x))
            .collect()
    }
}

impl pallet_collator_assignment::Config for Test {
    type SessionIndex = u32;
    type HostConfiguration = HostConfigurationGetter;
    type ContainerChains = ContainerChainsGetter;
    type NimbusId = u64;
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

pub fn run_to_block(n: u64) {
    let old_block_number = System::block_number();
    let session_len = 5;

    for x in (old_block_number + 1)..=n {
        System::set_block_number(x);

        if x % session_len == 1 {
            let session_index = (x / session_len) as u32;
            CollatorAssignment::initializer_on_new_session(
                &session_index,
                CollatorsGetter::collators(session_index),
            );
        }
    }
}
