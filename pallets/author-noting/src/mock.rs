use crate::{self as author_noting_pallet, Config};
use cumulus_primitives_core::{ParaId, PersistedValidationData};
use frame_support::inherent::{InherentData, ProvideInherent};
use frame_support::parameter_types;
use frame_support::traits::{ConstU32, ConstU64};
use frame_support::traits::{Everything, UnfilteredDispatchable};
use frame_support::traits::{OnFinalize, OnInitialize};
use frame_support::Hashable;
use frame_system::RawOrigin;
use parity_scale_codec::{Decode, Encode};
use polkadot_parachain::primitives::RelayChainBlockNumber;
use sp_consensus_aura::inherents::InherentType;
use sp_core::H256;
use sp_runtime::traits::HashFor;
use sp_trie::MemoryDB;
use sp_version::RuntimeVersion;

use sp_io;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

type AccountId = u64;
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        AuthorNoting: author_noting_pallet::{Pallet, Call, Storage, Event<T>},
        MockData: mock_data,
    }
);

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
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
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

parameter_types! {
    pub const ParachainId: ParaId = ParaId::new(200);
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

impl mock_data::Config for Test {}

#[derive(Clone, Encode, Decode, PartialEq, sp_core::RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Mocks {
    pub container_chains: Vec<ParaId>,
}

impl Default for Mocks {
    fn default() -> Self {
        Self {
            container_chains: vec![1001.into()],
        }
    }
}

pub struct MockAuthorFetcher;

impl crate::GetAuthorFromSlot<Test> for MockAuthorFetcher {
    fn author_from_inherent(inherent: InherentType) -> Option<AccountId> {
        return Some(inherent.into());
    }
}

pub struct MockContainerChainGetter;

impl crate::GetContainerChains for MockContainerChainGetter {
    fn container_chains() -> Vec<ParaId> {
        MockData::mock().container_chains
    }
}

// Implement the sudo module's `Config` on the Test runtime.
impl Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type AuthorFetcher = MockAuthorFetcher;
    type ContainerChains = MockContainerChainGetter;
}

struct BlockTest {
    n: <Test as frame_system::Config>::BlockNumber,
    within_block: Box<dyn Fn()>,
    after_block: Option<Box<dyn Fn()>>,
}

struct ReadRuntimeVersion(Vec<u8>);

impl sp_core::traits::ReadRuntimeVersion for ReadRuntimeVersion {
    fn read_runtime_version(
        &self,
        _wasm_code: &[u8],
        _ext: &mut dyn sp_externalities::Externalities,
    ) -> Result<Vec<u8>, String> {
        Ok(self.0.clone())
    }
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

fn wasm_ext() -> sp_io::TestExternalities {
    let version = RuntimeVersion {
        spec_name: "test".into(),
        spec_version: 2,
        impl_version: 1,
        ..Default::default()
    };

    let mut ext = new_test_ext();
    ext.register_extension(sp_core::traits::ReadRuntimeVersionExt::new(
        ReadRuntimeVersion(version.encode()),
    ));
    ext
}

/// BlockTests exist to test blocks with some setup: we have to assume that
/// `validate_block` will mutate and check storage in certain predictable
/// ways, for example, and we want to always ensure that tests are executed
/// in the context of some particular block number.
#[derive(Default)]
pub struct BlockTests {
    tests: Vec<BlockTest>,
    ran: bool,
    relay_sproof_builder_hook:
        Option<Box<dyn Fn(&BlockTests, RelayChainBlockNumber, &mut OwnRelayStateSproofBuilder)>>,
    inherent_data_hook: Option<
        Box<
            dyn Fn(&BlockTests, RelayChainBlockNumber, &mut crate::types::OwnParachainInherentData),
        >,
    >,
}

impl BlockTests {
    pub fn new() -> BlockTests {
        Default::default()
    }

    fn add_raw(mut self, test: BlockTest) -> Self {
        self.tests.push(test);
        self
    }

    pub fn add<F>(self, n: <Test as frame_system::Config>::BlockNumber, within_block: F) -> Self
    where
        F: 'static + Fn(),
    {
        self.add_raw(BlockTest {
            n,
            within_block: Box::new(within_block),
            after_block: None,
        })
    }

    pub fn with_relay_sproof_builder<F>(mut self, f: F) -> Self
    where
        F: 'static + Fn(&BlockTests, RelayChainBlockNumber, &mut OwnRelayStateSproofBuilder),
    {
        self.relay_sproof_builder_hook = Some(Box::new(f));
        self
    }

    pub fn run(&mut self) {
        self.ran = true;
        wasm_ext().execute_with(|| {
            for BlockTest {
                n,
                within_block,
                after_block,
            } in self.tests.iter()
            {
                // begin initialization
                System::reset_events();
                System::initialize(&n, &Default::default(), &Default::default());

                // now mess with the storage the way validate_block does
                let mut sproof_builder = OwnRelayStateSproofBuilder::default();
                if let Some(ref hook) = self.relay_sproof_builder_hook {
                    hook(self, *n as RelayChainBlockNumber, &mut sproof_builder);
                }

                let (relay_parent_storage_root, relay_chain_state) =
                    sproof_builder.into_state_root_and_proof();

                let vfp = PersistedValidationData {
                    relay_parent_number: *n as RelayChainBlockNumber,
                    relay_parent_storage_root,
                    ..Default::default()
                };

                // It is insufficient to push the author function params
                // to storage; they must also be included in the inherent data.
                let inherent_data = {
                    let mut inherent_data = InherentData::default();
                    let mut system_inherent_data = crate::types::OwnParachainInherentData {
                        validation_data: vfp.clone(),
                        relay_chain_state,
                    };
                    if let Some(ref hook) = self.inherent_data_hook {
                        hook(self, *n as RelayChainBlockNumber, &mut system_inherent_data);
                    }
                    inherent_data
                        .put_data(crate::INHERENT_IDENTIFIER, &system_inherent_data)
                        .expect("failed to put VFP inherent");
                    inherent_data
                };

                // execute the block
                AuthorNoting::on_initialize(*n);
                AuthorNoting::create_inherent(&inherent_data)
                    .expect("got an inherent")
                    .dispatch_bypass_filter(RawOrigin::None.into())
                    .expect("dispatch succeeded");
                within_block();
                AuthorNoting::on_finalize(*n);

                // clean up
                System::finalize();
                if let Some(after_block) = after_block {
                    after_block();
                }
            }
        });
    }
}

impl Drop for BlockTests {
    fn drop(&mut self) {
        if !self.ran {
            self.run();
        }
    }
}

#[derive(Clone)]
pub enum HeaderAs {
    AlreadyEncoded(Vec<u8>),
    NonEncoded(sp_runtime::generic::Header<u32, BlakeTwo256>),
}

#[derive(Clone)]
pub struct OwnRelayStateSproofBuilderItem {
    /// The para id of the current parachain.
    ///
    /// This doesn't get into the storage proof produced by the builder, however, it is used for
    /// generation of the storage image and by auxiliary methods.
    ///
    /// It's recommended to change this value once in the very beginning of usage.
    ///
    /// The default value is 200.
    pub para_id: ParaId,

    pub author_id: HeaderAs,
}

impl Default for OwnRelayStateSproofBuilderItem {
    fn default() -> Self {
        Self {
            para_id: ParaId::from(200),
            author_id: HeaderAs::AlreadyEncoded(vec![]),
        }
    }
}

/// Builds a sproof (portmanteau of 'spoof' and 'proof') of the relay chain state.
#[derive(Clone, Default)]
pub struct OwnRelayStateSproofBuilder {
    pub items: Vec<OwnRelayStateSproofBuilderItem>,
}

impl OwnRelayStateSproofBuilder {
    pub fn into_state_root_and_proof(
        self,
    ) -> (
        polkadot_primitives::v2::Hash,
        sp_state_machine::StorageProof,
    ) {
        let (db, root) = MemoryDB::<HashFor<polkadot_primitives::v2::Block>>::default_with_root();
        let state_version = Default::default(); // for test using default.
        let mut backend = sp_state_machine::TrieBackendBuilder::new(db, root).build();

        let mut relevant_keys = Vec::new();
        {
            use parity_scale_codec::Encode as _;

            let mut insert = |key: Vec<u8>, value: Vec<u8>| {
                relevant_keys.push(key.clone());
                backend.insert(vec![(None, vec![(key, Some(value))])], state_version);
            };

            for item in self.items {
                let para_key = item.para_id.twox_64_concat();
                let key = [crate::PARAS_HEADS_INDEX, para_key.as_slice()].concat();

                let encoded = match item.author_id {
                    HeaderAs::AlreadyEncoded(encoded) => encoded,
                    HeaderAs::NonEncoded(header) => header.encode(),
                };
                insert(key, encoded);
            }
        }

        let root = backend.root().clone();
        let proof = sp_state_machine::prove_read(backend, relevant_keys).expect("prove read");
        (root, proof)
    }
}
