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
    dancelight_emulated_chain::{genesis::INITIAL_BALANCE, DancelightRelayPallet},
    dancelight_system_emulated_network::{
        DancelightRelay as Dancelight, DancelightSender, FrontierTemplatePara as FrontierTemplate,
        SimpleTemplatePara as SimpleTemplate, SimpleTemplateSender,
    },
    frame_support::assert_ok,
    frame_support::traits::PalletInfoAccess,
    frontier_template_emulated_chain::{EthereumSender, FrontierTemplateParaPallet},
    simple_template_emulated_chain::SimpleTemplateParaPallet,
    snowbridge_core::{AgentId, ChannelId},
    sp_core::H160,
    xcm::latest::prelude::*,
    xcm_emulator::{Chain, TestExt},
    xcm_executor::traits::ConvertLocation,
};

#[test]
fn check_if_container_chain_router_is_working_for_eth_transfer_frontier() {
    // Define common constants and accounts
    const CONTAINER_PARA_ID: u32 = 2001;

    let container_fee =
        container_chain_template_frontier_runtime::xcm_config::ContainerToEthTransferFee::get();

    let fees_account = dancelight_runtime::SnowbridgeFeesAccount::get();

    // Common location calculations
    let container_location = Location::new(0, Parachain(CONTAINER_PARA_ID));
    let eth_network_location = Location::new(
        1,
        GlobalConsensus(dancelight_runtime_constants::snowbridge::EthereumNetwork::get()),
    );

    Dancelight::execute_with(|| {
        let container_sovereign_account =
            dancelight_runtime::xcm_config::LocationConverter::convert_location(
                &container_location,
            )
            .unwrap();
        let eth_sovereign_account =
            dancelight_runtime::xcm_config::LocationConverter::convert_location(
                &eth_network_location,
            )
            .unwrap();

        // Get initial balances
        let fees_balance_before =
            <Dancelight as DancelightRelayPallet>::System::account(fees_account.clone())
                .data
                .free;
        let eth_balance_before =
            <Dancelight as DancelightRelayPallet>::System::account(eth_sovereign_account)
                .data
                .free;

        assert_eq!(fees_balance_before, INITIAL_BALANCE);
        assert_eq!(eth_balance_before, 0u128);

        // Setup origins
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();
        let alice_origin = <Dancelight as Chain>::RuntimeOrigin::signed(DancelightSender::get());

        // Fund container's sovereign account with some balance
        assert_ok!(
            <Dancelight as DancelightRelayPallet>::Balances::transfer_allow_death(
                alice_origin,
                container_sovereign_account.into(),
                INITIAL_BALANCE
            )
        );

        // Register container asset in EthereumSystem
        let asset_location = Location {
            parents: 0,
            interior: Junctions::X2([Parachain(CONTAINER_PARA_ID), PalletInstance(<<FrontierTemplate as FrontierTemplateParaPallet>::Balances as PalletInfoAccess>::index() as u8)].into()),
        };

        assert_ok!(
            <Dancelight as DancelightRelayPallet>::EthereumSystem::register_token(
                root_origin.clone(),
                Box::new(asset_location.into()),
                snowbridge_core::AssetMetadata {
                    name: "container2001".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "container2001".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 18,
                }
            )
        );

        // Set token transfer channel in EthereumTokenTransfers
        assert_ok!(<Dancelight as DancelightRelayPallet>::EthereumTokenTransfers::set_token_transfer_channel(root_origin, ChannelId::new([5u8; 32]), AgentId::from_low_u64_be(10), 3000u32.into()));
    });

    FrontierTemplate::execute_with(|| {
        let sovereign_account =
            container_chain_template_frontier_runtime::xcm_config::LocationToAccountId::convert_location(
                &Location::new(2, container_chain_template_frontier_runtime::EthereumNetwork::get()),
            )
                .unwrap();

        let beneficiary_address = H160([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef, 0x01, 0x23, 0x45, 0x67,
        ]);

        let eth_destination = Location {
            parents: 2,
            interior: Junctions::X1(
                [GlobalConsensus(
                    container_chain_template_frontier_runtime::EthereumNetwork::get(),
                )]
                .into(),
            ),
        };

        let beneficiary_location = Location {
            parents: 0,
            interior: Junctions::X1(
                [AccountKey20 {
                    network: Some(
                        container_chain_template_frontier_runtime::EthereumNetwork::get(),
                    ),
                    key: beneficiary_address.into(),
                }]
                .into(),
            ),
        };

        let amount_to_transfer = 1_000;

        let asset_location = Location {
            parents: 0,
            interior: Junctions::X1([PalletInstance(<<FrontierTemplate as FrontierTemplateParaPallet>::Balances as PalletInfoAccess>::index() as u8)].into()),
        };

        let eth_asset =
            AssetId(asset_location.clone()).into_asset(Fungibility::Fungible(amount_to_transfer));

        let alice_origin =
            <FrontierTemplate as Chain>::RuntimeOrigin::signed(EthereumSender::get());

        let balance_before =
            <FrontierTemplate as FrontierTemplateParaPallet>::System::account(sovereign_account)
                .data
                .free;

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::PolkadotXcm::transfer_assets(
                alice_origin,
                Box::new(eth_destination.into()),
                Box::new(beneficiary_location.into()),
                Box::new(vec![eth_asset].into()),
                0u32,
                Unlimited
            )
        );

        let balance_after =
            <FrontierTemplate as FrontierTemplateParaPallet>::System::account(sovereign_account)
                .data
                .free;

        assert_eq!(balance_after - balance_before, amount_to_transfer);
    });

    Dancelight::execute_with(|| {
        let eth_sovereign_account =
            dancelight_runtime::xcm_config::LocationConverter::convert_location(
                &eth_network_location,
            )
            .unwrap();

        // Check final balances
        let fees_balance_after =
            <Dancelight as DancelightRelayPallet>::System::account(fees_account)
                .data
                .free;
        let eth_balance_after =
            <Dancelight as DancelightRelayPallet>::System::account(eth_sovereign_account)
                .data
                .free;

        // Fees are collected on Tanssi
        assert!(fees_balance_after > INITIAL_BALANCE);

        // Check we are in range
        assert!(fees_balance_after <= INITIAL_BALANCE + container_fee);

        // Check that leftover fees were deposited into the ETH sovereign account
        assert!(eth_balance_after > 0u128);
        assert!(eth_balance_after < container_fee);
    });
}

#[test]
fn check_if_container_chain_router_is_working_for_eth_transfer_simple() {
    // Define common constants and accounts
    const CONTAINER_PARA_ID: u32 = 2002;

    let container_fee =
        container_chain_template_simple_runtime::xcm_config::ContainerToEthTransferFee::get();

    let fees_account = dancelight_runtime::SnowbridgeFeesAccount::get();

    // Common location calculations
    let container_location = Location::new(0, Parachain(CONTAINER_PARA_ID));
    let eth_network_location = Location::new(
        1,
        GlobalConsensus(dancelight_runtime_constants::snowbridge::EthereumNetwork::get()),
    );

    Dancelight::execute_with(|| {
        let container_sovereign_account =
            dancelight_runtime::xcm_config::LocationConverter::convert_location(
                &container_location,
            )
            .unwrap();
        let eth_sovereign_account =
            dancelight_runtime::xcm_config::LocationConverter::convert_location(
                &eth_network_location,
            )
            .unwrap();

        // Get initial balances
        let fees_balance_before =
            <Dancelight as DancelightRelayPallet>::System::account(fees_account.clone())
                .data
                .free;
        let eth_balance_before =
            <Dancelight as DancelightRelayPallet>::System::account(eth_sovereign_account)
                .data
                .free;

        assert_eq!(fees_balance_before, INITIAL_BALANCE);
        assert_eq!(eth_balance_before, 0u128);

        // Setup origins
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();
        let alice_origin = <Dancelight as Chain>::RuntimeOrigin::signed(DancelightSender::get());

        // Fund container's sovereign account with some balance
        assert_ok!(
            <Dancelight as DancelightRelayPallet>::Balances::transfer_allow_death(
                alice_origin,
                container_sovereign_account.into(),
                INITIAL_BALANCE
            )
        );

        // Register container asset in EthereumSystem
        let asset_location = Location {
            parents: 0,
            interior: Junctions::X2([Parachain(CONTAINER_PARA_ID), PalletInstance(<<SimpleTemplate as SimpleTemplateParaPallet>::Balances as PalletInfoAccess>::index() as u8)].into()),
        };

        assert_ok!(
            <Dancelight as DancelightRelayPallet>::EthereumSystem::register_token(
                root_origin.clone(),
                Box::new(asset_location.into()),
                snowbridge_core::AssetMetadata {
                    name: "container2002".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "container2002".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            )
        );

        // Set token transfer channel in EthereumTokenTransfers
        assert_ok!(<Dancelight as DancelightRelayPallet>::EthereumTokenTransfers::set_token_transfer_channel(root_origin, ChannelId::new([5u8; 32]), AgentId::from_low_u64_be(10), 3000u32.into()));
    });

    SimpleTemplate::execute_with(|| {
        let sovereign_account =
            container_chain_template_simple_runtime::xcm_config::LocationToAccountId::convert_location(
                &Location::new(2, container_chain_template_simple_runtime::EthereumNetwork::get()),
            )
                .unwrap();

        let beneficiary_address = H160([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef, 0x01, 0x23, 0x45, 0x67,
        ]);

        let eth_destination = Location {
            parents: 2,
            interior: Junctions::X1(
                [GlobalConsensus(
                    container_chain_template_simple_runtime::EthereumNetwork::get(),
                )]
                .into(),
            ),
        };

        let beneficiary_location = Location {
            parents: 0,
            interior: Junctions::X1(
                [AccountKey20 {
                    network: Some(container_chain_template_simple_runtime::EthereumNetwork::get()),
                    key: beneficiary_address.into(),
                }]
                .into(),
            ),
        };

        let amount_to_transfer = 1_000_000_000;

        let asset_location = Location {
            parents: 0,
            interior: Junctions::X1([PalletInstance(<<SimpleTemplate as SimpleTemplateParaPallet>::Balances as PalletInfoAccess>::index() as u8)].into()),
        };

        let eth_asset =
            AssetId(asset_location.clone()).into_asset(Fungibility::Fungible(amount_to_transfer));

        let alice_origin = <SimpleTemplate as Chain>::RuntimeOrigin::signed(
            container_chain_template_simple_runtime::AccountId::from(SimpleTemplateSender::get()),
        );

        let balance_before = <SimpleTemplate as SimpleTemplateParaPallet>::System::account(
            sovereign_account.clone(),
        )
        .data
        .free;

        let alice_balance = <SimpleTemplate as SimpleTemplateParaPallet>::System::account(
            container_chain_template_simple_runtime::AccountId::from(SimpleTemplateSender::get()),
        )
        .data
        .free;

        assert_ne!(alice_balance, 0);

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::PolkadotXcm::transfer_assets(
                alice_origin,
                Box::new(eth_destination.into()),
                Box::new(beneficiary_location.into()),
                Box::new(vec![eth_asset].into()),
                0u32,
                Unlimited
            )
        );

        let balance_after = <SimpleTemplate as SimpleTemplateParaPallet>::System::account(
            sovereign_account.clone(),
        )
        .data
        .free;

        assert_eq!(balance_after - balance_before, amount_to_transfer);
    });

    Dancelight::execute_with(|| {
        let eth_sovereign_account =
            dancelight_runtime::xcm_config::LocationConverter::convert_location(
                &eth_network_location,
            )
            .unwrap();

        // Check final balances
        let fees_balance_after =
            <Dancelight as DancelightRelayPallet>::System::account(fees_account)
                .data
                .free;
        let eth_balance_after =
            <Dancelight as DancelightRelayPallet>::System::account(eth_sovereign_account)
                .data
                .free;

        // Fees are collected on Tanssi
        assert!(fees_balance_after > INITIAL_BALANCE);

        // Check we are in range
        assert!(fees_balance_after <= INITIAL_BALANCE + container_fee);

        // Check that leftover fees were deposited into the ETH sovereign account
        assert!(eth_balance_after > 0u128);
        assert!(eth_balance_after < container_fee);
    });
}
