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

//! Tests for the Dancelight Runtime Configuration

use {
    crate::*, frame_support::traits::WhitelistedStorageKeys, sp_core::hexdisplay::HexDisplay,
    std::collections::HashSet,
};

mod author_noting_tests;
mod beefy;
mod collator_assignment_tests;
mod common;
mod core_scheduling_tests;
mod ethereum_client;
mod ethereum_token_transfers;
mod external_validators_tests;
mod inactivity_tracking;
mod inbound_queue_tests;
mod inflation_rates;
mod inflation_rewards;
mod integration_test;
mod migrations_test;
mod offline_marking;
mod relay_configuration;
mod relay_registrar;
mod services_payment;
mod session_keys;
mod slashes;
mod staking;
mod sudo;

#[test]
fn check_whitelist() {
    let whitelist: HashSet<String> = AllPalletsWithSystem::whitelisted_storage_keys()
        .iter()
        .map(|e| HexDisplay::from(&e.key).to_string())
        .collect();

    // Block number
    assert!(whitelist.contains("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac"));
    // Total issuance
    assert!(whitelist.contains("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80"));
    // Execution phase
    assert!(whitelist.contains("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a"));
    // Event count
    assert!(whitelist.contains("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850"));
    // System events
    assert!(whitelist.contains("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7"));
    // XcmPallet VersionDiscoveryQueue
    assert!(whitelist.contains("1405f2411d0af5a7ff397e7c9dc68d194a222ba0333561192e474c59ed8e30e1"));
    // XcmPallet SafeXcmVersion
    assert!(whitelist.contains("1405f2411d0af5a7ff397e7c9dc68d196323ae84c43568be0d1394d5d0d522c4"));
}

#[test]
fn check_treasury_pallet_id() {
    assert_eq!(
        <Treasury as frame_support::traits::PalletInfoAccess>::index() as u8,
        dancelight_runtime_constants::TREASURY_PALLET_ID
    );
}
