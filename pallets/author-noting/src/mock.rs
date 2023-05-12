use {
    crate::{self as author_noting_pallet, Config},
    cumulus_pallet_parachain_system::{RelayChainState, RelaychainStateProvider},
    cumulus_primitives_core::ParaId,
    frame_support::{
        inherent::{InherentData, ProvideInherent},
        parameter_types,
        traits::{
            ConstU32, ConstU64, Everything, OnFinalize, OnInitialize, UnfilteredDispatchable,
        },
    },
    frame_system::RawOrigin,
    parity_scale_codec::{Decode, Encode},
    polkadot_parachain::primitives::RelayChainBlockNumber,
    polkadot_primitives::Slot,
    sp_core::H256,
    sp_io,
    sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
    },
    sp_state_machine::StorageProof,
    sp_version::RuntimeVersion,
    test_relay_sproof_builder::ParaHeaderSproofBuilder,
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

impl tp_traits::GetContainerChainAuthor<AccountId> for MockAuthorFetcher {
    fn author_for_slot(slot: Slot, _para_id: ParaId) -> Option<AccountId> {
        return Some(slot.into());
    }
}

pub struct MockContainerChainGetter;

impl tp_traits::GetCurrentContainerChains for MockContainerChainGetter {
    fn current_container_chains() -> Vec<ParaId> {
        MockData::mock().container_chains
    }
}

const MOCK_RELAY_ROOT_KEY: &'static [u8] = b"MOCK_RELAY_ROOT_KEY";

pub struct MockRelayStateProvider;

impl RelaychainStateProvider for MockRelayStateProvider {
    fn current_relay_chain_state() -> RelayChainState {
        let root = frame_support::storage::unhashed::get(MOCK_RELAY_ROOT_KEY)
            .expect("root should be set by mock");

        RelayChainState {
            state_root: root,
            number: 0, // block number is not relevant here
        }
    }
}

// Implement the sudo module's `Config` on the Test runtime.
impl Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type ContainerChainAuthor = MockAuthorFetcher;
    type SelfParaId = ParachainId;
    type ContainerChains = MockContainerChainGetter;
    type RelayChainStateProvider = MockRelayStateProvider;
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
        Option<Box<dyn Fn(&BlockTests, RelayChainBlockNumber, &mut ParaHeaderSproofBuilder)>>,
    inherent_data_hook: Option<
        Box<
            dyn Fn(
                &BlockTests,
                RelayChainBlockNumber,
                &mut tp_author_noting_inherent::OwnParachainInherentData,
            ),
        >,
    >,
    overriden_state_root: Option<H256>,
    overriden_state_proof: Option<StorageProof>,
    skip_inherent_insertion: bool,
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
        F: 'static + Fn(&BlockTests, RelayChainBlockNumber, &mut ParaHeaderSproofBuilder),
    {
        self.relay_sproof_builder_hook = Some(Box::new(f));
        self
    }

    pub fn with_overriden_state_root(mut self, root: H256) -> Self {
        self.overriden_state_root = Some(root);
        self
    }

    pub fn with_overriden_state_proof(mut self, proof: StorageProof) -> Self {
        self.overriden_state_proof = Some(proof);
        self
    }

    pub fn skip_inherent_insertion(mut self) -> Self {
        self.skip_inherent_insertion = true;
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
                let mut sproof_builder = ParaHeaderSproofBuilder::default();
                if let Some(ref hook) = self.relay_sproof_builder_hook {
                    hook(self, *n as RelayChainBlockNumber, &mut sproof_builder);
                }

                let (mut relay_storage_root, mut relay_storage_proof) =
                    sproof_builder.into_state_root_and_proof();

                if let Some(root) = self.overriden_state_root {
                    relay_storage_root = root;
                }

                if let Some(state) = &self.overriden_state_proof {
                    relay_storage_proof = state.clone();
                }

                // We write relay storage root in mock storage.
                frame_support::storage::unhashed::put(MOCK_RELAY_ROOT_KEY, &relay_storage_root);

                // It is insufficient to push the author function params
                // to storage; they must also be included in the inherent data.
                let inherent_data = {
                    let mut inherent_data = InherentData::default();
                    let mut system_inherent_data =
                        tp_author_noting_inherent::OwnParachainInherentData {
                            relay_storage_proof,
                        };
                    if let Some(ref hook) = self.inherent_data_hook {
                        hook(self, *n as RelayChainBlockNumber, &mut system_inherent_data);
                    }
                    inherent_data
                        .put_data(
                            tp_author_noting_inherent::INHERENT_IDENTIFIER,
                            &system_inherent_data,
                        )
                        .expect("failed to put VFP inherent");
                    inherent_data
                };

                // execute the block
                AuthorNoting::on_initialize(*n);
                if !self.skip_inherent_insertion {
                    AuthorNoting::create_inherent(&inherent_data)
                        .expect("got an inherent")
                        .dispatch_bypass_filter(RawOrigin::None.into())
                        .expect("dispatch succeeded");
                }
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
