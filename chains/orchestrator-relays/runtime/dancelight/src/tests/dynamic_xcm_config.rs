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

//! Tests for dynamic XCM configuration parameters

use {
    crate::{
        dynamic_params, tests::common::*, xcm_config::ThisNetwork,
        RuntimeCall, RuntimeOrigin, RuntimeParameters,
    },
    frame_support::{assert_ok, traits::Get},
    sp_runtime::traits::Dispatchable,
    xcm::latest::NetworkId,
};

#[test]
fn test_this_network_default_value() {
    ExtBuilder::default().build().execute_with(|| {
        // Check the default value matches the expected DANCELIGHT_GENESIS_HASH
        let default_network_id = dynamic_params::xcm_config::ThisNetwork::get();
        let expected_network_id =
            NetworkId::ByGenesis(dancelight_runtime_constants::DANCELIGHT_GENESIS_HASH);
        assert_eq!(default_network_id, expected_network_id);

        // Also check via the ThisNetwork wrapper
        let this_network: NetworkId = <ThisNetwork as Get<NetworkId>>::get();
        assert_eq!(this_network, expected_network_id);
    });
}

#[test]
fn test_this_network_can_be_updated() {
    ExtBuilder::default().build().execute_with(|| {
        // Get the initial value
        let initial_network: NetworkId = <ThisNetwork as Get<NetworkId>>::get();
        let expected_initial =
            NetworkId::ByGenesis(dancelight_runtime_constants::DANCELIGHT_GENESIS_HASH);
        assert_eq!(initial_network, expected_initial);

        // Create a new network ID
        let new_genesis_hash = [1u8; 32];
        let new_network_id = NetworkId::ByGenesis(new_genesis_hash);

        // Set the new value via root
        let parameter = RuntimeParameters::XcmConfig(
            dynamic_params::xcm_config::Parameters::ThisNetwork(
                dynamic_params::xcm_config::ThisNetwork,
                Some(new_network_id)
            )
        );

        assert_ok!(RuntimeCall::Parameters(pallet_parameters::Call::set_parameter {
            key_value: parameter,
        })
        .dispatch(RuntimeOrigin::root()));

        // Check that the value has been updated
        let updated_network: NetworkId = <ThisNetwork as Get<NetworkId>>::get();
        assert_eq!(updated_network, new_network_id);

        // Also check directly via the dynamic parameter
        let direct_value = dynamic_params::xcm_config::ThisNetwork::get();
        assert_eq!(direct_value, new_network_id);
    });
}

#[test]
fn test_only_root_can_modify_this_network() {
    ExtBuilder::default()
        .with_balances(vec![(AccountId::from(ALICE), 1000 * UNIT)])
        .build()
        .execute_with(|| {
            let new_genesis_hash = [1u8; 32];
            let new_network_id = NetworkId::ByGenesis(new_genesis_hash);

            let parameter = RuntimeParameters::XcmConfig(
                dynamic_params::xcm_config::Parameters::ThisNetwork(
                    dynamic_params::xcm_config::ThisNetwork,
                    Some(new_network_id)
                )
            );

            // Try to set with non-root origin (should fail)
            let alice_origin = RuntimeOrigin::signed(AccountId::from(ALICE));
            let result = RuntimeCall::Parameters(pallet_parameters::Call::set_parameter {
                key_value: parameter.clone(),
            })
            .dispatch(alice_origin);

            // This should fail with BadOrigin
            assert!(result.is_err());

            // Verify the value hasn't changed
            let expected_default =
                NetworkId::ByGenesis(dancelight_runtime_constants::DANCELIGHT_GENESIS_HASH);
            let current_network: NetworkId = <ThisNetwork as Get<NetworkId>>::get();
            assert_eq!(current_network, expected_default);

            // Now try with root (should succeed)
            assert_ok!(RuntimeCall::Parameters(pallet_parameters::Call::set_parameter {
                key_value: parameter,
            })
            .dispatch(RuntimeOrigin::root()));

            // Verify the value has been updated
            let updated_network: NetworkId = <ThisNetwork as Get<NetworkId>>::get();
            assert_eq!(updated_network, new_network_id);
        });
}

#[test]
fn test_this_network_different_network_types() {
    ExtBuilder::default().build().execute_with(|| {
        let test_cases = vec![
            NetworkId::ByGenesis([1u8; 32]),
            NetworkId::Polkadot,
            NetworkId::Kusama,
            NetworkId::Ethereum { chain_id: 1 },
            NetworkId::BitcoinCore,
            NetworkId::BitcoinCash,
        ];

        for network_id in test_cases {
            let parameter = RuntimeParameters::XcmConfig(
                dynamic_params::xcm_config::Parameters::ThisNetwork(
                    dynamic_params::xcm_config::ThisNetwork,
                    Some(network_id)
                )
            );

            assert_ok!(RuntimeCall::Parameters(pallet_parameters::Call::set_parameter {
                key_value: parameter,
            })
            .dispatch(RuntimeOrigin::root()));

            // Verify the value was set correctly
            let current_network: NetworkId = <ThisNetwork as Get<NetworkId>>::get();
            assert_eq!(current_network, network_id);
        }
    });
}