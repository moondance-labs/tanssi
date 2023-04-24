use {
    crate::{self as pallet_configuration, HostConfiguration},
    frame_support::traits::{ConstU16, ConstU64},
    frame_system as system,
    sp_core::{ConstU32, H256},
    sp_runtime::{
        testing::{Header, UintAuthorityId},
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    },
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
        Configuration: pallet_configuration,
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

pub struct CurrentSessionIndexGetter;

impl pallet_configuration::GetSessionIndex<u32> for CurrentSessionIndexGetter {
    /// Returns current session index.
    fn session_index() -> u32 {
        // For tests, let 1 session be 5 blocks
        (System::block_number() / 5) as u32
    }
}

impl pallet_configuration::Config for Test {
    type WeightInfo = ();
    type SessionDelay = ConstU32<2>;
    type SessionIndex = u32;
    type CurrentSessionIndex = CurrentSessionIndexGetter;
    type AuthorityId = UintAuthorityId;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext_with_genesis(config: HostConfiguration) -> sp_io::TestExternalities {
    GenesisConfig {
        system: Default::default(),
        configuration: pallet_configuration::GenesisConfig { config },
    }
    .build_storage()
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
            Configuration::initializer_on_new_session(&session_index);
        }
    }
}
