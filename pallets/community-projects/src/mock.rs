use super::*;

use crate as pallet_community_projects;
use frame_support::{parameter_types, traits::AsEnsureOriginWithArg};
use sp_core::ConstU32;
use sp_runtime::{
	traits::{AccountIdLookup, BlakeTwo256, IdentifyAccount, Verify},
	MultiSignature,
};

use frame_system::EnsureRoot;

//use pallet_transaction_payment::CurrencyAdapter;

//use pallet_transaction_payment::{ConstFeeMultiplier, Multiplier};

use sp_runtime::BuildStorage;

use pallet_nfts::PalletFeatures;

pub type Block = frame_system::mocking::MockBlock<Test>;

pub type BlockNumber = u64;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Signature = MultiSignature;
pub type AccountPublic = <Signature as Verify>::Signer;

/* let id = [0: u32].into();

pub const ALICE: AccountId = id;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;
pub const DAVE: AccountId = 4;  */

pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

//use frame_benchmarking::account;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Uniques: pallet_nfts::{Pallet, Call, Storage, Event<T>},
		CommunityProjects: pallet_community_projects,
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		Assets: pallet_assets::<Instance1>,
		XcavateWhitelist: pallet_xcavate_whitelist,
	}
);

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
}

impl frame_system::Config for Test {
	type RuntimeCall = RuntimeCall;
	type Nonce = u32;
	type Block = Block;
	type Hash = sp_core::H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u32>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = frame_support::traits::Everything;
	type SystemWeightInfo = ();
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type RuntimeTask = ();
}

impl pallet_balances::Config for Test {
	type Balance = u32;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU32<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type FreezeIdentifier = ();
	// Holds are used with COLLATOR_LOCK_ID and DELEGATOR_LOCK_ID
	type MaxHolds = ConstU32<2>;
	type MaxFreezes = ConstU32<0>;
}

parameter_types! {
	pub Features: PalletFeatures = PalletFeatures::all_enabled();
	pub const ApprovalsLimit: u32 = 20;
	pub const ItemAttributesApprovalsLimit: u32 = 20;
	pub const MaxTips: u32 = 10;
	pub const MaxDeadlineDuration: BlockNumber = 12 * 30 * DAYS;
	pub const MaxAttributesPerCall: u32 = 10;
}

impl pallet_nfts::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<Self::AccountId>>;
	type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type Locker = ();
	type CollectionDeposit = ConstU32<2>;
	type ItemDeposit = ConstU32<1>;
	type MetadataDepositBase = ConstU32<1>;
	type AttributeDepositBase = ConstU32<1>;
	type DepositPerByte = ConstU32<1>;
	type StringLimit = ConstU32<50>;
	type KeyLimit = ConstU32<50>;
	type ValueLimit = ConstU32<50>;
	type WeightInfo = ();
	type ApprovalsLimit = ApprovalsLimit;
	type ItemAttributesApprovalsLimit = ItemAttributesApprovalsLimit;
	type MaxTips = MaxTips;
	type MaxDeadlineDuration = MaxDeadlineDuration;
	type MaxAttributesPerCall = MaxAttributesPerCall;
	type Features = Features;
	type OffchainSignature = Signature;
	type OffchainPublic = AccountPublic;
}

parameter_types! {
	pub const MinimumPeriod: u64 = 1;
}
impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const AssetConversionPalletId: PalletId = PalletId(*b"py/ascon");

}

impl pallet_assets::Config<Instance1> for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type AssetId = u32;
	type AssetIdParameter = codec::Compact<u32>;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<Self::AccountId>>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = ConstU32<1>;
	type AssetAccountDeposit = ConstU32<1>;
	type MetadataDepositBase = ConstU32<1>;
	type MetadataDepositPerByte = ConstU32<1>;
	type ApprovalDeposit = ConstU32<1>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = ();
	type RemoveItemsLimit = ConstU32<1000>;
}

parameter_types! {
	pub const MaxWhitelistUsers: u32 = 1000000;
}

impl pallet_xcavate_whitelist::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_xcavate_whitelist::weights::SubstrateWeight<Test>;
	type WhitelistOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type MaxUsersInWhitelist = MaxWhitelistUsers;
}  

parameter_types! {
	pub const CommunityProjectPalletId: PalletId = PalletId(*b"py/cmprj");
	pub const MaxNftType: u32 = 4;
	pub const MaxListedNftProject: u32 = 300000;
	pub const MaxNftsInCollectionProject: u32 = 10000;
	pub const MaxOngoingProject: u32 = 10000;
	pub const MaxNftHolders: u32 = 10000;
}

/// Configure the pallet-xcavate-staking in pallets/xcavate-staking.
impl pallet_community_projects::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::SubstrateWeight<Test>;
	type Currency = Balances;
	type PalletId = CommunityProjectPalletId;
	type MaxNftTypes = MaxNftType;
	type MaxNftInCollection = MaxNftsInCollectionProject;
	type TimeProvider = Timestamp;
	type MaxOngoingProjects = MaxOngoingProject;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = NftHelper;
	type AssetId = u32;
	type CollectionId = u32;
	type ItemId = u32;
	type MinimumRemainingAmount = ConstU32<100>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut test = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			([0; 32].into(), 20_000_000),
			([1; 32].into(), 15_000),
			([2; 32].into(), 150_000),
			([3; 32].into(), 5_000),
			([4; 32].into(), 5_000),
			((CommunityProjects::account_id()), 140),
		],
	}
	.assimilate_storage(&mut test)
	.unwrap();

 	pallet_assets::GenesisConfig::<Test, Instance1> {
		assets: vec![(1, /* account("buyer", SEED, SEED) */ [0; 32].into(), true, 1)], // Genesis assets: id, owner, is_sufficient, min_balance
		metadata: vec![(1, "XUSD".into(), "XUSD".into(), 0)], // Genesis metadata: id, name, symbol, decimals
		accounts: vec![
			(1, [0; 32].into(), 20_000_000),
			(1, [1; 32].into(), 1_500),
			(1, [2; 32].into(), 150_000),
			(1, [3; 32].into(), 5_000),
			(1, [4; 32].into(), 50),
			(1, [5; 32].into(), 500),
		], // Genesis accounts: id, account_id, balance
	}
	.assimilate_storage(&mut test)
	.unwrap(); 

	test.into()
}