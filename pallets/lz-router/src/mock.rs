// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

use {
    crate::{self as pallet_lz_router},
    frame_support::{
        derive_impl, parameter_types,
        traits::{ConstU32, Everything, Nothing},
        weights::Weight,
    },
    frame_system::EnsureRoot,
    pallet_xcm::TestWeightInfo,
    sp_core::H256,
    sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    },
    std::cell::RefCell,
    std::sync::Arc,
    xcm::latest::{
        Assets, InteriorLocation, Junction, Junctions, Location, NetworkId, SendError, SendResult,
        SendXcm, Xcm, XcmHash,
    },
    xcm_builder::FixedWeightBounds,
};

type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u64;
pub type Balance = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances,
        XcmPallet: pallet_xcm,
        LzRouter: pallet_lz_router,
    }
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type Block = Block;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
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
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
    type WeightInfo = ();
}

// Thread-local storage for tracking sent XCM messages
thread_local! {
    pub static SENT_XCM: RefCell<Vec<(Location, Xcm<()>)>> = const { RefCell::new(Vec::new()) };
}

/// Mock XCM sender that records all sent messages
pub struct MockXcmSender;

impl SendXcm for MockXcmSender {
    type Ticket = (Location, Xcm<()>);

    fn validate(
        destination: &mut Option<Location>,
        message: &mut Option<Xcm<()>>,
    ) -> SendResult<Self::Ticket> {
        let dest = destination.take().ok_or(SendError::MissingArgument)?;
        let msg = message.take().ok_or(SendError::MissingArgument)?;
        Ok(((dest, msg), Assets::new()))
    }

    fn deliver(ticket: Self::Ticket) -> Result<XcmHash, SendError> {
        SENT_XCM.with(|sent| sent.borrow_mut().push(ticket));
        Ok([0u8; 32])
    }
}

/// Get all XCM messages that were sent
pub fn sent_xcm() -> Vec<(Location, Xcm<()>)> {
    SENT_XCM.with(|sent| sent.borrow().clone())
}

/// Clear all sent XCM messages
pub fn clear_sent_xcm() {
    SENT_XCM.with(|sent| sent.borrow_mut().clear());
}

/// Mock origin that simulates a container chain XCM origin
pub struct MockContainerChainOrigin;

impl frame_support::traits::EnsureOrigin<RuntimeOrigin> for MockContainerChainOrigin {
    type Success = Location;

    fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
        match o.clone().into() {
            Ok(frame_system::RawOrigin::Signed(account)) => {
                // Signed origin is treated as parachain with id = account
                Ok(Location::new(0, [Junction::Parachain(account as u32)]))
            }
            _ => Err(o),
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
        Ok(RuntimeOrigin::root())
    }
}

parameter_types! {
    pub const MaxWhitelistedSendersValue: u32 = 10;
    pub UniversalLocation: InteriorLocation = Junctions::X1(
        Arc::new([Junction::GlobalConsensus(NetworkId::ByGenesis([0u8; 32]))])
    );
    pub const BaseXcmWeight: Weight = Weight::from_parts(10_000, 0);
    pub const MaxInstructions: u32 = 100;
}

/// Wrapper type that implements Clone for parameter_types
#[derive(Clone)]
pub struct MaxWhitelistedSenders;

impl frame_support::traits::Get<u32> for MaxWhitelistedSenders {
    fn get() -> u32 {
        MaxWhitelistedSendersValue::get()
    }
}

impl pallet_xcm::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type SendXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, ()>;
    type XcmRouter = MockXcmSender;
    type ExecuteXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, ()>;
    type XcmExecuteFilter = Nothing;
    type XcmExecutor = ();
    type XcmTeleportFilter = Nothing;
    type XcmReserveTransferFilter = Nothing;
    type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
    type UniversalLocation = UniversalLocation;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
    type AdvertisedXcmVersion = ConstU32<3>;
    type Currency = Balances;
    type CurrencyMatcher = ();
    type TrustedLockers = ();
    type SovereignAccountOf = ();
    type MaxLockers = ConstU32<8>;
    type WeightInfo = TestWeightInfo;
    type AdminOrigin = EnsureRoot<AccountId>;
    type MaxRemoteLockConsumers = ConstU32<0>;
    type RemoteLockConsumerIdentifier = ();
    type AuthorizedAliasConsideration = ();
}

impl pallet_lz_router::Config for Test {
    type MaxWhitelistedSenders = MaxWhitelistedSenders;
    type ContainerChainOrigin = MockContainerChainOrigin;
}

#[derive(Default)]
pub struct ExtBuilder;

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();

        let mut ext: sp_io::TestExternalities = t.into();
        ext.execute_with(|| {
            System::set_block_number(1);
            clear_sent_xcm();
        });
        ext
    }
}

pub(crate) fn lz_router_events() -> Vec<crate::Event<Test>> {
    System::events()
        .into_iter()
        .map(|r| r.event)
        .filter_map(|e| {
            if let RuntimeEvent::LzRouter(inner) = e {
                Some(inner)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
