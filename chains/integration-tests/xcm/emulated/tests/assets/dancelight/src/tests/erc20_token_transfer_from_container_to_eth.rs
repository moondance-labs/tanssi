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
    dancelight_emulated_chain::DancelightRelayPallet,
    dancelight_system_emulated_network::{
        DancelightRelay as Dancelight, DancelightSender, FrontierTemplatePara as FrontierTemplate,
    },
    frame_support::assert_ok,
    frontier_template_emulated_chain::{EthereumSender, FrontierTemplateParaPallet},
    hex_literal::hex,
    snowbridge_core::ChannelId,
    sp_core::H160,
    xcm::latest::prelude::*,
    xcm_emulator::{Chain, TestExt},
    xcm_executor::traits::{ConvertLocation, TransferType},
};

#[test]
fn check_if_container_chain_router_is_working_for_eth_transfer_frontier() {
    // Define common constants and accounts
    const CONTAINER_PARA_ID: u32 = 2001;

    const PARA_ID_FOR_CHANNEL: u32 = 2000;

    const ERC20_TOKEN_ADDRESS: [u8; 20] = hex!("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef");

    const ERC20_ASSET_ID: u16 = 123;

    const RELAY_NATIVE_ASSET_ID: u16 = 124;

    const ERC20_ASSET_AMOUNT: u128 = 123_321_000_000_000_000;

    const RELAY_ASSET_FEE_AMOUNT: u128 = 3_500_000_000_000;

    // Common location calculations
    let container_location = Location::new(0, Parachain(CONTAINER_PARA_ID));
    let container_sovereign_account =
        dancelight_runtime::xcm_config::LocationConverter::convert_location(&container_location)
            .unwrap();

    let erc20_asset_id_for_container_context = Location {
        parents: 2,
        interior: Junctions::X2(
            [
                GlobalConsensus(NetworkId::Ethereum { chain_id: 11155111 }),
                AccountKey20 {
                    network: Some(NetworkId::Ethereum { chain_id: 11155111 }),
                    key: ERC20_TOKEN_ADDRESS,
                },
            ]
            .into(),
        ),
    };

    let erc20_asset_id_for_relay_context = Location {
        parents: 1,
        interior: Junctions::X2(
            [
                GlobalConsensus(NetworkId::Ethereum { chain_id: 11155111 }),
                AccountKey20 {
                    network: Some(NetworkId::Ethereum { chain_id: 11155111 }),
                    key: ERC20_TOKEN_ADDRESS,
                },
            ]
            .into(),
        ),
    };

    let relay_asset_id_for_container_context = Location {
        parents: 1,
        interior: Here,
    };

    Dancelight::execute_with(|| {
        // Setup origins
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();
        let alice_origin = <Dancelight as Chain>::RuntimeOrigin::signed(DancelightSender::get());

        assert_ok!(
            <Dancelight as DancelightRelayPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                erc20_asset_id_for_relay_context.clone(),
                ERC20_ASSET_ID,
                DancelightSender::get(),
                true,
                1
            )
        );

        assert_ok!(<Dancelight as DancelightRelayPallet>::ForeignAssets::mint(
            alice_origin.clone(),
            ERC20_ASSET_ID,
            container_sovereign_account.clone().into(),
            ERC20_ASSET_AMOUNT
        ));

        assert_ok!(
            <Dancelight as DancelightRelayPallet>::Balances::transfer_allow_death(
                alice_origin.clone(),
                container_sovereign_account.clone().into(),
                RELAY_ASSET_FEE_AMOUNT
            )
        );

        assert_ok!(
            <Dancelight as DancelightRelayPallet>::EthereumTokenTransfers::set_token_transfer_channel(
                root_origin.clone(),
                ChannelId::new(hex!("0000000000000000000000000000000000000000000000000000000000000004")),
                hex!("0000000000000000000000000000000000000000000000000000000000000005").into(),
                PARA_ID_FOR_CHANNEL.into()
            ),
        );
    });

    FrontierTemplate::execute_with(|| {
        let root_origin = <FrontierTemplate as Chain>::RuntimeOrigin::root();
        let alice_origin =
            <FrontierTemplate as Chain>::RuntimeOrigin::signed(EthereumSender::get());

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                erc20_asset_id_for_container_context.clone(),
                ERC20_ASSET_ID,
                EthereumSender::get(),
                true,
                1
            )
        );

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                relay_asset_id_for_container_context.clone(),
                RELAY_NATIVE_ASSET_ID,
                EthereumSender::get(),
                true,
                1
            )
        );

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::mint(
                alice_origin.clone(),
                ERC20_ASSET_ID,
                EthereumSender::get(),
                ERC20_ASSET_AMOUNT
            )
        );

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::mint(
                alice_origin.clone(),
                RELAY_NATIVE_ASSET_ID,
                EthereumSender::get(),
                RELAY_ASSET_FEE_AMOUNT
            )
        );
    });

    Dancelight::execute_with(|| {
        let container_chain_sovereign_account_system_balance_before =
            <Dancelight as DancelightRelayPallet>::System::account(
                container_sovereign_account.clone(),
            )
            .data
            .free;
        assert_eq!(
            container_chain_sovereign_account_system_balance_before,
            RELAY_ASSET_FEE_AMOUNT
        );
    });

    FrontierTemplate::execute_with(|| {
        let beneficiary_address = H160([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef, 0x01, 0x23, 0x45, 0x67,
        ]);

        let relay_destination = Location {
            parents: 1,
            interior: Here,
        };

        let erc20_asset_balance_before =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                ERC20_ASSET_ID,
                EthereumSender::get(),
            );
        assert_eq!(erc20_asset_balance_before, ERC20_ASSET_AMOUNT);

        let relay_asset_balance_before =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                RELAY_NATIVE_ASSET_ID,
                EthereumSender::get(),
            );
        assert_eq!(relay_asset_balance_before, RELAY_ASSET_FEE_AMOUNT);

        let container_chain_sovereign_account_erc20_balance_before =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                ERC20_ASSET_ID,
                EthereumSender::get(),
            );
        assert_eq!(
            container_chain_sovereign_account_erc20_balance_before,
            ERC20_ASSET_AMOUNT
        );

        let fee_assets = AssetId(relay_asset_id_for_container_context.clone())
            .into_asset(Fungibility::Fungible(RELAY_ASSET_FEE_AMOUNT));
        let erc20_assets = AssetId(erc20_asset_id_for_container_context.clone())
            .into_asset(Fungibility::Fungible(ERC20_ASSET_AMOUNT));

        let erc20_asset_id_for_ethereum_context = Location {
            parents: 0,
            interior: Junctions::X1(
                [AccountKey20 {
                    network: Some(container_chain_template_simple_runtime::EthereumNetwork::get()),
                    key: beneficiary_address.into(),
                }]
                .into(),
            ),
        };

        let beneficiary = Location {
            parents: 0,
            interior: Junctions::X1(
                [AccountKey20 {
                    network: Some(container_chain_template_simple_runtime::EthereumNetwork::get()),
                    key: beneficiary_address.into(),
                }]
                .into(),
            ),
        };

        let custom_xcm_on_dest = Xcm::<()>(vec![InitiateReserveWithdraw {
            assets: Definite(
                vec![AssetId(erc20_asset_id_for_relay_context.clone())
                    .into_asset(Fungibility::Fungible(ERC20_ASSET_AMOUNT))]
                .into(),
            ),
            reserve: Location {
                parents: 1,
                interior: Junctions::X1(
                    [GlobalConsensus(NetworkId::Ethereum { chain_id: 11155111 })].into(),
                ),
            },
            xcm: Xcm::<()>(vec![DepositAsset {
                assets: Definite(
                    vec![AssetId(erc20_asset_id_for_ethereum_context.clone())
                        .into_asset(Fungibility::Fungible(ERC20_ASSET_AMOUNT))]
                    .into(),
                ),
                beneficiary,
            }]),
        }]);

        let token_sender_origin =
            <FrontierTemplate as Chain>::RuntimeOrigin::signed(EthereumSender::get());

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::PolkadotXcm::transfer_assets_using_type_and_then(
                token_sender_origin.clone(),
                Box::new(relay_destination.into()),
                Box::new(vec![fee_assets, erc20_assets].into()),
                Box::new(TransferType::DestinationReserve),
                Box::new(relay_asset_id_for_container_context.into()),
                Box::new(TransferType::DestinationReserve),
                Box::new(xcm::VersionedXcm::from(custom_xcm_on_dest)),
                Unlimited
            )
        );

        let erc20_asset_balance_after =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                ERC20_ASSET_ID,
                EthereumSender::get(),
            );
        assert_eq!(erc20_asset_balance_after, 0);

        let relay_asset_balance_after =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                RELAY_NATIVE_ASSET_ID,
                EthereumSender::get(),
            );
        assert_eq!(relay_asset_balance_after, 0);
    });

    Dancelight::execute_with(|| {
        let container_chain_sovereign_account_erc20_balance_after =
            <Dancelight as DancelightRelayPallet>::ForeignAssets::balance(
                ERC20_ASSET_ID,
                container_sovereign_account.clone(),
            );
        assert_eq!(container_chain_sovereign_account_erc20_balance_after, 0);

        let container_chain_sovereign_account_system_balance_after =
            <Dancelight as DancelightRelayPallet>::System::account(
                container_sovereign_account.clone(),
            )
            .data
            .free;
        assert_eq!(container_chain_sovereign_account_system_balance_after, 0);
    });
}
