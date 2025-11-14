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
    frame_support::{assert_ok, traits::PalletInfoAccess},
    frontier_template_emulated_chain::{EthereumSender, FrontierTemplateParaPallet},
    simple_template_emulated_chain::SimpleTemplateParaPallet,
    sp_core::H160,
    sp_runtime::{BoundedVec, FixedU128},
    xcm::{latest::prelude::*, opaque::latest::AssetTransferFilter, VersionedXcm},
    xcm_emulator::{assert_expected_events, Chain, TestExt},
    xcm_executor::traits::ConvertLocation,
};

const RELAY_NATIVE_TOKEN_ASSET_ID: u16 = 42;
const RELAY_TOKEN_ASSET_LOCATION: Location = Location::parent();

#[test]
fn check_if_container_chain_router_is_working_for_eth_transfer_frontier_snowbridge_v2() {
    // Define common constants and accounts
    const CONTAINER_PARA_ID: u32 = 2001;

    let container_fee =
        container_chain_template_frontier_runtime::xcm_config::ContainerToEthTransferFee::get();

    let export_fee_amount = 1_000_000_000u128;

    let fees_account = dancelight_runtime::SnowbridgeFeesAccount::get();
    let mut fees_account_balance_before = 0u128;

    // Common location calculations
    let container_location = Location::new(0, Parachain(CONTAINER_PARA_ID));
    let container_sovereign_account =
        dancelight_runtime::xcm_config::LocationConverter::convert_location(&container_location)
            .unwrap();

    Dancelight::execute_with(|| {
        // Get initial balances
        fees_account_balance_before =
            <Dancelight as DancelightRelayPallet>::System::account(fees_account.clone())
                .data
                .free;

        assert_eq!(fees_account_balance_before, INITIAL_BALANCE);

        // Setup origins
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();
        let alice_origin = <Dancelight as Chain>::RuntimeOrigin::signed(DancelightSender::get());

        // Fund container's sovereign account with some balance
        assert_ok!(
            <Dancelight as DancelightRelayPallet>::Balances::transfer_allow_death(
                alice_origin,
                container_sovereign_account.clone().into(),
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
    });

    FrontierTemplate::execute_with(|| {
        let root_origin = <FrontierTemplate as Chain>::RuntimeOrigin::root();
        let alice_account = EthereumSender::get();
        let ethereum_sovereign_account =
            container_chain_template_frontier_runtime::xcm_config::LocationToAccountId::convert_location(
                &Location::new(2, container_chain_template_frontier_runtime::EthereumNetwork::get()),
            )
                .unwrap();

        let eth_sovereign_account_balance_before =
            <FrontierTemplate as FrontierTemplateParaPallet>::System::account(
                ethereum_sovereign_account.clone(),
            )
            .data
            .free;

        assert_eq!(eth_sovereign_account_balance_before, 0u128);

        let beneficiary_address = H160([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef, 0x01, 0x23, 0x45, 0x67,
        ]);

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

        let amount_to_withdraw = 1_000_000_000_000_000u128;
        let amount_to_transfer = 1_000_000_000_000u128;

        let asset_location = Location {
            parents: 0,
            interior: Junctions::X1([PalletInstance(<<FrontierTemplate as FrontierTemplateParaPallet>::Balances as PalletInfoAccess>::index() as u8)].into()),
        };

        let relay_asset_location = Location {
            parents: 1,
            interior: Here,
        };

        let container_asset_to_withdraw =
            AssetId(asset_location.clone()).into_asset(Fungibility::Fungible(amount_to_withdraw));

        let container_asset_to_transfer =
            AssetId(asset_location.clone()).into_asset(Fungibility::Fungible(amount_to_transfer));

        let fee_asset = AssetId(relay_asset_location.clone())
            .into_asset(Fungibility::Fungible(export_fee_amount));

        let alice_origin =
            <FrontierTemplate as Chain>::RuntimeOrigin::signed(EthereumSender::get());

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                RELAY_TOKEN_ASSET_LOCATION,
                RELAY_NATIVE_TOKEN_ASSET_ID,
                alice_account.clone().into(),
                true,
                1
            )
        );

        // Create asset rate for relay token
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::AssetRate::create(
                root_origin.clone(),
                Box::new(RELAY_NATIVE_TOKEN_ASSET_ID),
                FixedU128::from_u32(500_000_000)
            )
        );

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::mint(
                alice_origin.clone(),
                RELAY_NATIVE_TOKEN_ASSET_ID,
                alice_account,
                100_000_000_000_000u128
            )
        );

        let assets = vec![container_asset_to_withdraw.clone(), fee_asset.clone()];

        let xcm: VersionedXcm<container_chain_template_frontier_runtime::RuntimeCall> =
            VersionedXcm::from(Xcm(vec![
                WithdrawAsset(assets.clone().into()),
                InitiateTransfer {
                    destination: container_chain_template_frontier_runtime::EthereumLocation::get()
                        .into(),
                    remote_fees: Some(AssetTransferFilter::ReserveWithdraw(
                        fee_asset.clone().into(),
                    )),
                    preserve_origin: true,
                    assets: BoundedVec::truncate_from(vec![AssetTransferFilter::ReserveDeposit(
                        Definite(container_asset_to_transfer.clone().into()),
                    )]),
                    remote_xcm: Xcm(vec![DepositAsset {
                        assets: Wild(AllCounted(1)),
                        beneficiary: beneficiary_location,
                    }]),
                },
            ]))
            .into();

        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::PolkadotXcm::execute(
                alice_origin.clone(),
                Box::new(xcm),
                Weight::from(16_000_000_000)
            )
        );

        let eth_sovereign_account_balance_after =
            <FrontierTemplate as FrontierTemplateParaPallet>::System::account(
                ethereum_sovereign_account.clone(),
            )
            .data
            .free;

        assert_eq!(
            eth_sovereign_account_balance_after - eth_sovereign_account_balance_before,
            amount_to_transfer
        );
    });

    Dancelight::execute_with(|| {
        type RuntimeEvent = <Dancelight as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancelight,
            vec![
                RuntimeEvent::EthereumOutboundQueueV2(snowbridge_pallet_outbound_queue_v2::Event::MessageAccepted { nonce: 1, id: _ }) => {},
            ]
        );

        // Check feesAccount balance (fees should have been collected)
        let fees_account_balance_after =
            <Dancelight as DancelightRelayPallet>::System::account(fees_account)
                .data
                .free;
        assert!(fees_account_balance_after > fees_account_balance_before);
        assert!(fees_account_balance_after <= fees_account_balance_before + export_fee_amount);

        let container_sovereign_account_balance_after =
            <Dancelight as DancelightRelayPallet>::System::account(container_sovereign_account)
                .data
                .free;

        assert!(container_sovereign_account_balance_after < INITIAL_BALANCE);
        assert!(
            container_sovereign_account_balance_after
                >= INITIAL_BALANCE - export_fee_amount - container_fee
        );
    });
}

#[test]
fn check_if_container_chain_router_is_working_for_eth_transfer_simple_snowbridge_v2() {
    // Define common constants and accounts
    const CONTAINER_PARA_ID: u32 = 2002;

    let container_fee =
        container_chain_template_simple_runtime::xcm_config::ContainerToEthTransferFee::get();

    let export_fee_amount = 1_000_000_000u128;

    let fees_account = dancelight_runtime::SnowbridgeFeesAccount::get();
    let mut fees_account_balance_before = 0u128;

    // Common location calculations
    let container_location = Location::new(0, Parachain(CONTAINER_PARA_ID));
    let container_sovereign_account =
        dancelight_runtime::xcm_config::LocationConverter::convert_location(&container_location)
            .unwrap();

    Dancelight::execute_with(|| {
        // Get initial balances
        fees_account_balance_before =
            <Dancelight as DancelightRelayPallet>::System::account(fees_account.clone())
                .data
                .free;

        assert_eq!(fees_account_balance_before, INITIAL_BALANCE);

        // Setup origins
        let root_origin = <Dancelight as Chain>::RuntimeOrigin::root();
        let alice_origin = <Dancelight as Chain>::RuntimeOrigin::signed(DancelightSender::get());

        // Fund container's sovereign account with some balance
        assert_ok!(
            <Dancelight as DancelightRelayPallet>::Balances::transfer_allow_death(
                alice_origin,
                container_sovereign_account.clone().into(),
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
                    name: "container2000".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "container2000".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 18,
                }
            )
        );
    });

    SimpleTemplate::execute_with(|| {
        let root_origin = <SimpleTemplate as Chain>::RuntimeOrigin::root();
        let alice_account = SimpleTemplateSender::get();
        let ethereum_sovereign_account =
            container_chain_template_simple_runtime::xcm_config::LocationToAccountId::convert_location(
                &Location::new(2, container_chain_template_simple_runtime::EthereumNetwork::get()),
            )
                .unwrap();

        let eth_sovereign_account_balance_before =
            <SimpleTemplate as SimpleTemplateParaPallet>::System::account(
                ethereum_sovereign_account.clone(),
            )
            .data
            .free;

        assert_eq!(eth_sovereign_account_balance_before, 0u128);

        let beneficiary_address = H160([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef, 0x01, 0x23, 0x45, 0x67,
        ]);

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

        let amount_to_withdraw = 1_000_000_000_000_000u128;
        let amount_to_transfer = 1_000_000_000_000u128;

        let asset_location = Location {
            parents: 0,
            interior: Junctions::X1([PalletInstance(<<SimpleTemplate as SimpleTemplateParaPallet>::Balances as PalletInfoAccess>::index() as u8)].into()),
        };

        let relay_asset_location = Location {
            parents: 1,
            interior: Here,
        };

        let container_asset_to_withdraw =
            AssetId(asset_location.clone()).into_asset(Fungibility::Fungible(amount_to_withdraw));

        let container_asset_to_transfer =
            AssetId(asset_location.clone()).into_asset(Fungibility::Fungible(amount_to_transfer));

        let fee_asset = AssetId(relay_asset_location.clone())
            .into_asset(Fungibility::Fungible(export_fee_amount));

        let alice_origin =
            <SimpleTemplate as Chain>::RuntimeOrigin::signed(SimpleTemplateSender::get());

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                RELAY_TOKEN_ASSET_LOCATION,
                RELAY_NATIVE_TOKEN_ASSET_ID,
                alice_account.clone().into(),
                true,
                1
            )
        );

        // Create asset rate for relay token
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::AssetRate::create(
                root_origin.clone(),
                Box::new(RELAY_NATIVE_TOKEN_ASSET_ID),
                FixedU128::from_u32(500_000_000)
            )
        );

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::mint(
                alice_origin.clone(),
                RELAY_NATIVE_TOKEN_ASSET_ID,
                alice_account.into(),
                100_000_000_000_000u128
            )
        );

        let assets = vec![container_asset_to_withdraw.clone(), fee_asset.clone()];

        let xcm: VersionedXcm<container_chain_template_simple_runtime::RuntimeCall> =
            VersionedXcm::from(Xcm(vec![
                WithdrawAsset(assets.clone().into()),
                InitiateTransfer {
                    destination: container_chain_template_simple_runtime::EthereumLocation::get()
                        .into(),
                    remote_fees: Some(AssetTransferFilter::ReserveWithdraw(
                        fee_asset.clone().into(),
                    )),
                    preserve_origin: true,
                    assets: BoundedVec::truncate_from(vec![AssetTransferFilter::ReserveDeposit(
                        Definite(container_asset_to_transfer.clone().into()),
                    )]),
                    remote_xcm: Xcm(vec![DepositAsset {
                        assets: Wild(AllCounted(1)),
                        beneficiary: beneficiary_location,
                    }]),
                },
            ]))
            .into();

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::PolkadotXcm::execute(
                alice_origin.clone(),
                Box::new(xcm),
                Weight::from(16_000_000_000)
            )
        );

        let eth_sovereign_account_balance_after =
            <SimpleTemplate as SimpleTemplateParaPallet>::System::account(
                ethereum_sovereign_account.clone(),
            )
            .data
            .free;

        assert_eq!(
            eth_sovereign_account_balance_after - eth_sovereign_account_balance_before,
            amount_to_transfer
        );
    });

    Dancelight::execute_with(|| {
        type RuntimeEvent = <Dancelight as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancelight,
            vec![
                RuntimeEvent::EthereumOutboundQueueV2(snowbridge_pallet_outbound_queue_v2::Event::MessageAccepted { nonce: 1, id: _ }) => {},
            ]
        );

        // Check feesAccount balance (fees should have been collected)
        let fees_account_balance_after =
            <Dancelight as DancelightRelayPallet>::System::account(fees_account)
                .data
                .free;
        assert!(fees_account_balance_after > fees_account_balance_before);
        assert!(fees_account_balance_after <= fees_account_balance_before + export_fee_amount);

        let container_sovereign_account_balance_after =
            <Dancelight as DancelightRelayPallet>::System::account(container_sovereign_account)
                .data
                .free;

        assert!(container_sovereign_account_balance_after < INITIAL_BALANCE);
        assert!(
            container_sovereign_account_balance_after
                >= INITIAL_BALANCE - export_fee_amount - container_fee
        );
    });
}
