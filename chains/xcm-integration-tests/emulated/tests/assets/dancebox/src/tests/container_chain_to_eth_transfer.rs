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
    frame_support::assert_ok,
    frame_support::traits::PalletInfoAccess,
    frontier_template_emulated_chain::{EthereumSender, FrontierTemplateParaPallet},
    simple_template_emulated_chain::SimpleTemplateParaPallet,
    sp_core::H160,
    westend_system_emulated_network::{
        FrontierTemplatePara as FrontierTemplate, SimpleTemplatePara as SimpleTemplate,
        SimpleTemplateSender,
    },
    xcm::latest::prelude::*,
    xcm_emulator::{Chain, TestExt},
    xcm_executor::traits::ConvertLocation,
};

#[test]
fn check_if_container_chain_router_is_working_for_eth_transfer_frontier() {
    FrontierTemplate::execute_with(|| {
        let sovereign_account =
            container_chain_template_frontier_runtime::xcm_config::LocationToAccountId::convert_location(
                &container_chain_template_frontier_runtime::EthereumLocation::get(),
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
}

#[test]
fn check_if_container_chain_router_is_working_for_eth_transfer_simple() {
    SimpleTemplate::execute_with(|| {
        let sovereign_account =
            container_chain_template_simple_runtime::xcm_config::LocationToAccountId::convert_location(
                &container_chain_template_simple_runtime::EthereumLocation::get(),
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
}
