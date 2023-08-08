use super::constants::{
	accounts::{ALICE, BOB}, westend
};
use frame_support::{parameter_types, sp_tracing};

pub use sp_core::{sr25519, storage::Storage, Get};
use xcm::prelude::*;
use xcm_emulator::{
	decl_test_networks, decl_test_parachains, decl_test_relay_chains,
	Parachain,
	RelayChain, TestExt,
};
use xcm_executor::traits::Convert;
use crate::{AccountId, Balance};

decl_test_relay_chains! {
	pub struct Westend {
		genesis = westend::genesis(),
		on_init = (),
		runtime = {
			Runtime: westend_runtime::Runtime,
			RuntimeOrigin: westend_runtime::RuntimeOrigin,
			RuntimeCall: westend_runtime::RuntimeCall,
			RuntimeEvent: westend_runtime::RuntimeEvent,
			MessageQueue: westend_runtime::MessageQueue,
			XcmConfig: westend_runtime::xcm_config::XcmConfig,
			SovereignAccountOf: westend_runtime::xcm_config::LocationConverter, //TODO: rename to SovereignAccountOf,
			System: westend_runtime::System,
			Balances: westend_runtime::Balances,
		},
		pallets_extra = {
			XcmPallet: westend_runtime::XcmPallet,
			Sudo: westend_runtime::Sudo,
		}
    }
}

decl_test_parachains! {
	// Parachains
	pub struct Dancebox {
		genesis = crate::ExtBuilder::default()
        .with_balances(vec![
            // Alice gets 10k extra tokens for her mapping deposit
            (AccountId::from(crate::ALICE), 210_000 * crate::UNIT),
            (AccountId::from(crate::BOB), 100_000 * crate::UNIT),
        ]).build_storage(),
		on_init = (),
		runtime = {
			Runtime: dancebox_runtime::Runtime,
			RuntimeOrigin: dancebox_runtime::RuntimeOrigin,
			RuntimeCall: dancebox_runtime::RuntimeCall,
			RuntimeEvent: dancebox_runtime::RuntimeEvent,
			XcmpMessageHandler: dancebox_runtime::XcmpQueue,
			DmpMessageHandler: dancebox_runtime::DmpQueue,
			LocationToAccountId: dancebox_runtime::xcm_config::LocationToAccountId,
			System: dancebox_runtime::System,
			Balances: dancebox_runtime::Balances,
			ParachainSystem: dancebox_runtime::ParachainSystem,
			ParachainInfo: dancebox_runtime::ParachainInfo,
		},
		pallets_extra = {
			PolkadotXcm: dancebox_runtime::PolkadotXcm,
		}
	}
}

decl_test_networks! {
	pub struct WestendMockNet {
		relay_chain = Westend,
		parachains = vec![
			Dancebox,
		],
	}
}

parameter_types! {
	// Polkadot
	pub PolkadotSender: cumulus_primitives_core::relay_chain::AccountId = Westend::account_id_of(ALICE);
	pub PolkadotReceiver: cumulus_primitives_core::relay_chain::AccountId = Westend::account_id_of(BOB);
	// Statemint
	pub DanceboxSender: dancebox_runtime::AccountId = Dancebox::account_id_of(ALICE);
	pub DanceboxReceiver: dancebox_runtime::AccountId = Dancebox::account_id_of(BOB);
}