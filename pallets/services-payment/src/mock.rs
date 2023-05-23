use frame_support::traits::{Currency, WithdrawReasons};

use {
    crate::{self as payment_services_pallet, OnChargeForBlockCredit, ProvideBlockProductionCost},
    cumulus_primitives_core::ParaId,
    frame_support::{
        pallet_prelude::*,
        parameter_types,
        traits::{tokens::ExistenceRequirement, ConstU32, ConstU64, Everything},
    },
    sp_core::H256,
    sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
    },
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type AccountId = u64;
type Balance = u128;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        PaymentServices: payment_services_pallet::{Pallet, Call, Config<T>, Storage, Event<T>}
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
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 1;
}

impl pallet_balances::Config for Test {
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 4];
    type MaxLocks = ();
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

parameter_types! {
    pub const MaxCreditsStored: u64 = 5;
}

impl payment_services_pallet::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeForBlockCredit = ChargeForBlockCredit<Test>;
    type Currency = Balances;
    type ProvideBlockProductionCost = BlockProductionCost<Test>;
    type MaxCreditsStored = MaxCreditsStored;
}

pub struct ChargeForBlockCredit<Test>(PhantomData<Test>);
impl OnChargeForBlockCredit<Test> for ChargeForBlockCredit<Test> {
    fn charge_credits(
        payer: &u64,
        _para_id: &ParaId,
        _credits: u64,
        fee: u128,
    ) -> Result<(), payment_services_pallet::Error<Test>> {
        use frame_support::traits::tokens::imbalance::Imbalance;

        let result = Balances::withdraw(
            &*payer,
            fee,
            WithdrawReasons::FEE,
            ExistenceRequirement::AllowDeath,
        );
        let imbalance = result
            .map_err(|_| payment_services_pallet::Error::InsufficientFundsToPurchaseCredits)?;

        if imbalance.peek() != fee {
            panic!("withdrawn balance incorrect");
        }

        Ok(())
    }
}

pub struct BlockProductionCost<Test>(PhantomData<Test>);
impl ProvideBlockProductionCost<Test> for BlockProductionCost<Test> {
    fn block_cost(_para_id: &ParaId) -> (u128, Weight) {
        (100, Weight::zero())
    }
}

#[derive(Default)]
pub struct ExtBuilder {
    balances: Vec<(AccountId, Balance)>,
}

impl ExtBuilder {
    pub fn with_balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
        self.balances = balances;
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: self.balances,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }
}

pub(crate) fn events() -> Vec<payment_services_pallet::Event<Test>> {
    System::events()
        .into_iter()
        .map(|r| r.event)
        .filter_map(|e| {
            if let RuntimeEvent::PaymentServices(inner) = e {
                Some(inner)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
