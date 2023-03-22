use crate as pallet_initializer;
use frame_support::traits::{ConstU16, ConstU64};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::{Header, UintAuthorityId},
    traits::{BlakeTwo256, IdentityLookup},
};
use sp_std::cell::RefCell;
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
        Initializer: pallet_initializer,
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

thread_local! {
    pub static SESSION_CHANGE_VALIDATORS: RefCell<Option<(u32, Vec<u64>)>> = RefCell::new(None);
}

pub fn session_change_validators() -> Option<(u32, Vec<u64>)> {
    SESSION_CHANGE_VALIDATORS.with(|q| (*q.borrow()).clone())
}

pub struct OwnApplySession;
impl pallet_initializer::ApplyNewSession<Test> for OwnApplySession {
    fn apply_new_session(
        _changed: bool,
        session_index: u32,
        all_validators: Vec<(u64, UintAuthorityId)>,
        _queued: Vec<(u64, UintAuthorityId)>,
    ) {
        let validators: Vec<_> = all_validators.iter().map(|(k, _)| k.clone()).collect();
        SESSION_CHANGE_VALIDATORS.with(|r| *r.borrow_mut() = Some((session_index, validators)));
    }
}

impl pallet_initializer::Config for Test {
    type SessionIndex = u32;

    /// The identifier type for an authority.
    type AuthorityId = UintAuthorityId;

    type SessionHandler = OwnApplySession;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}
