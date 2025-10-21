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
    dancelight_runtime::TreasuryAccount,
    dancelight_system_emulated_network::{
        DancelightRelay as Dancelight, DancelightSender, FrontierTemplatePara as FrontierTemplate,
        SimpleTemplatePara as SimpleTemplate, SimpleTemplateSender,
    },
    frame_support::assert_ok,
    frontier_template_emulated_chain::{EthereumSender, FrontierTemplateParaPallet},
    hex_literal::hex,
    simple_template_emulated_chain::SimpleTemplateParaPallet,
    snowbridge_core::ChannelId,
    sp_core::Get,
    xcm::latest::prelude::*,
    xcm::v5::NetworkId,
    xcm_emulator::{assert_expected_events, Chain, TestExt},
    xcm_executor::traits::{ConvertLocation, TransferType},
};

// Define common constants and accounts
const PARA_ID_FOR_CHANNEL: u32 = 2000;
const ERC20_TOKEN_ADDRESS: [u8; 20] = hex!("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef");
const ERC20_ASSET_ID: u16 = 123;
const RELAY_NATIVE_ASSET_ID: u16 = 124;
const ERC20_ASSET_AMOUNT: u128 = 123_321_000_000_000_000;
const RELAY_ASSET_FEE_AMOUNT: u128 = 3_500_000_000_000;

#[test]
fn check_if_container_chain_router_is_working_for_eth_transfer_frontier() {
    let asset_sender = EthereumSender::get();

    let container_para_id: u32 = FrontierTemplate::execute_with(|| {
        <FrontierTemplate as FrontierTemplateParaPallet>::ParachainInfo::parachain_id().into()
    });

    let ethereum_network =
        ethereum_chain_id::<container_chain_template_frontier_runtime::EthereumNetwork>();

    // Common location calculations
    let container_location = Location::new(0, Parachain(container_para_id));
    let container_sovereign_account =
        dancelight_runtime::xcm_config::LocationConverter::convert_location(&container_location)
            .unwrap();

    let erc20_asset_id_for_container_context = Location {
        parents: 2,
        interior: Junctions::X2(
            [
                GlobalConsensus(NetworkId::Ethereum {
                    chain_id: ethereum_network,
                }),
                AccountKey20 {
                    network: Some(NetworkId::Ethereum {
                        chain_id: ethereum_network,
                    }),
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
                GlobalConsensus(NetworkId::Ethereum {
                    chain_id: ethereum_network,
                }),
                AccountKey20 {
                    network: Some(NetworkId::Ethereum {
                        chain_id: ethereum_network,
                    }),
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

        // Creating foreign assets for the ERC20 tokens came from Ethereum in the Relay
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

        // Artificially minting foreign assets for the ERC20 tokens came from Ethereum to
        // the sovereign account to fake the reception of the token
        assert_ok!(<Dancelight as DancelightRelayPallet>::ForeignAssets::mint(
            alice_origin.clone(),
            ERC20_ASSET_ID,
            container_sovereign_account.clone().into(),
            ERC20_ASSET_AMOUNT
        ));

        // Adding native relay tokens to the sovereign account to be able to pay fees
        assert_ok!(
            <Dancelight as DancelightRelayPallet>::Balances::transfer_allow_death(
                alice_origin.clone(),
                container_sovereign_account.clone().into(),
                RELAY_ASSET_FEE_AMOUNT
                    + dancelight_runtime_constants::currency::EXISTENTIAL_DEPOSIT
            )
        );

        // Specifying the channel for the ERC20 token transfers
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
        let alice_origin = <FrontierTemplate as Chain>::RuntimeOrigin::signed(asset_sender);

        // Creating foreign assets for the ERC20 tokens came from Ethereum in the container chain
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                erc20_asset_id_for_container_context.clone(),
                ERC20_ASSET_ID,
                asset_sender,
                true,
                1
            )
        );

        // Creating foreign assets for the Relay native tokens in the container chain
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                relay_asset_id_for_container_context.clone(),
                RELAY_NATIVE_ASSET_ID,
                asset_sender,
                true,
                1
            )
        );

        // Minting foreign assets for the ERC20 tokens came from Ethereum in the container chain
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::mint(
                alice_origin.clone(),
                ERC20_ASSET_ID,
                asset_sender,
                ERC20_ASSET_AMOUNT
            )
        );

        // Minting foreign assets for the Relay native tokens in the container chain
        assert_ok!(
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::mint(
                alice_origin.clone(),
                RELAY_NATIVE_ASSET_ID,
                asset_sender,
                RELAY_ASSET_FEE_AMOUNT
            )
        );
    });

    // Check initial balances for the relay chain
    let mut treasury_fees_account_balance_before: u128 = 0;
    Dancelight::execute_with(|| {
        let container_chain_sovereign_account_erc20_balance_before =
            <Dancelight as DancelightRelayPallet>::ForeignAssets::balance(
                ERC20_ASSET_ID,
                container_sovereign_account.clone(),
            );
        assert_eq!(
            container_chain_sovereign_account_erc20_balance_before,
            ERC20_ASSET_AMOUNT
        );

        let container_chain_sovereign_account_system_balance_before =
            <Dancelight as DancelightRelayPallet>::System::account(
                container_sovereign_account.clone(),
            )
            .data
            .free;
        assert_eq!(
            container_chain_sovereign_account_system_balance_before,
            RELAY_ASSET_FEE_AMOUNT + dancelight_runtime_constants::currency::EXISTENTIAL_DEPOSIT
        );

        treasury_fees_account_balance_before =
            <Dancelight as DancelightRelayPallet>::System::account(TreasuryAccount::get())
                .data
                .free;
        assert_eq!(
            treasury_fees_account_balance_before,
            dancelight_runtime_constants::currency::EXISTENTIAL_DEPOSIT
        );
    });

    // Check initial balances for the container chain, execute the transfer and check result balances
    FrontierTemplate::execute_with(|| {
        let relay_destination = Location {
            parents: 1,
            interior: Here,
        };

        let erc20_asset_balance_before =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                ERC20_ASSET_ID,
                asset_sender,
            );
        assert_eq!(erc20_asset_balance_before, ERC20_ASSET_AMOUNT);

        let relay_asset_balance_before =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                RELAY_NATIVE_ASSET_ID,
                asset_sender,
            );
        assert_eq!(relay_asset_balance_before, RELAY_ASSET_FEE_AMOUNT);

        let fee_assets = AssetId(relay_asset_id_for_container_context.clone())
            .into_asset(Fungibility::Fungible(RELAY_ASSET_FEE_AMOUNT));
        let erc20_assets = AssetId(erc20_asset_id_for_container_context.clone())
            .into_asset(Fungibility::Fungible(ERC20_ASSET_AMOUNT));

        let erc20_asset_id_for_ethereum_context = Location {
            parents: 0,
            interior: Junctions::X1(
                [AccountKey20 {
                    network: Some(container_chain_template_simple_runtime::EthereumNetwork::get()),
                    key: ERC20_TOKEN_ADDRESS,
                }]
                .into(),
            ),
        };

        let beneficiary_address = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef, 0x01, 0x23, 0x45, 0x67,
        ];
        let beneficiary = Location {
            parents: 0,
            interior: Junctions::X1(
                [AccountKey20 {
                    network: Some(container_chain_template_simple_runtime::EthereumNetwork::get()),
                    key: beneficiary_address,
                }]
                .into(),
            ),
        };

        // The custom XCM on the destination is used to remove the ERC20
        // tokens from the holding and send Snowbridge command to withdraw
        // assets on the reserve location
        let custom_xcm_on_dest = Xcm::<()>(vec![InitiateReserveWithdraw {
            assets: Definite(
                vec![AssetId(erc20_asset_id_for_relay_context.clone())
                    .into_asset(Fungibility::Fungible(ERC20_ASSET_AMOUNT))]
                .into(),
            ),
            reserve: Location {
                parents: 1,
                interior: Junctions::X1(
                    [GlobalConsensus(NetworkId::Ethereum {
                        chain_id: ethereum_network,
                    })]
                    .into(),
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

        let token_sender_origin = <FrontierTemplate as Chain>::RuntimeOrigin::signed(asset_sender);

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
                asset_sender,
            );
        assert_eq!(erc20_asset_balance_after, 0);

        let relay_asset_balance_after =
            <FrontierTemplate as FrontierTemplateParaPallet>::ForeignAssets::balance(
                RELAY_NATIVE_ASSET_ID,
                asset_sender,
            );
        assert_eq!(relay_asset_balance_after, 0);
    });

    // Check result balances for the relay chain
    Dancelight::execute_with(|| {
        type RuntimeEvent = <Dancelight as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancelight,
            vec![
                RuntimeEvent::EthereumOutboundQueue(snowbridge_pallet_outbound_queue::Event::MessageAccepted { nonce: 1, id: _ }) => {},
            ]
        );

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
        assert_eq!(
            container_chain_sovereign_account_system_balance_after,
            dancelight_runtime_constants::currency::EXISTENTIAL_DEPOSIT
        );

        let treasury_fees_account_balance_after =
            <Dancelight as DancelightRelayPallet>::System::account(TreasuryAccount::get())
                .data
                .free;
        assert!(
            treasury_fees_account_balance_after
                > dancelight_runtime_constants::currency::EXISTENTIAL_DEPOSIT
        );

        // Check that fees were transferred to the treasury account
        assert!(treasury_fees_account_balance_after < RELAY_ASSET_FEE_AMOUNT);
    });
}

#[test]
fn check_if_container_chain_router_is_working_for_eth_transfer_simple() {
    let asset_sender = SimpleTemplateSender::get();

    let container_para_id: u32 = SimpleTemplate::execute_with(|| {
        <SimpleTemplate as SimpleTemplateParaPallet>::ParachainInfo::parachain_id().into()
    });

    let ethereum_network =
        ethereum_chain_id::<container_chain_template_frontier_runtime::EthereumNetwork>();

    // Common location calculations
    let container_location = Location::new(0, Parachain(container_para_id));
    let container_sovereign_account =
        dancelight_runtime::xcm_config::LocationConverter::convert_location(&container_location)
            .unwrap();

    let erc20_asset_id_for_container_context = Location {
        parents: 2,
        interior: Junctions::X2(
            [
                GlobalConsensus(NetworkId::Ethereum {
                    chain_id: ethereum_network,
                }),
                AccountKey20 {
                    network: Some(NetworkId::Ethereum {
                        chain_id: ethereum_network,
                    }),
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
                GlobalConsensus(NetworkId::Ethereum {
                    chain_id: ethereum_network,
                }),
                AccountKey20 {
                    network: Some(NetworkId::Ethereum {
                        chain_id: ethereum_network,
                    }),
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

        // Creating foreign assets for the ERC20 tokens came from Ethereum in the Relay
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

        // Artificially minting foreign assets for the ERC20 tokens came from Ethereum to
        // the sovereign account to fake the reception of the token
        assert_ok!(<Dancelight as DancelightRelayPallet>::ForeignAssets::mint(
            alice_origin.clone(),
            ERC20_ASSET_ID,
            container_sovereign_account.clone().into(),
            ERC20_ASSET_AMOUNT
        ));

        // Adding native relay tokens to the sovereign account to be able to pay fees
        assert_ok!(
            <Dancelight as DancelightRelayPallet>::Balances::transfer_allow_death(
                alice_origin.clone(),
                container_sovereign_account.clone().into(),
                RELAY_ASSET_FEE_AMOUNT
                    + dancelight_runtime_constants::currency::EXISTENTIAL_DEPOSIT
            )
        );

        // Specifying the channel for the ERC20 token transfers
        assert_ok!(
            <Dancelight as DancelightRelayPallet>::EthereumTokenTransfers::set_token_transfer_channel(
                root_origin.clone(),
                ChannelId::new(hex!("0000000000000000000000000000000000000000000000000000000000000004")),
                hex!("0000000000000000000000000000000000000000000000000000000000000005").into(),
                PARA_ID_FOR_CHANNEL.into()
            ),
        );
    });

    SimpleTemplate::execute_with(|| {
        let root_origin = <SimpleTemplate as Chain>::RuntimeOrigin::root();
        let alice_origin = <SimpleTemplate as Chain>::RuntimeOrigin::signed(asset_sender.clone());

        // Creating foreign assets for the ERC20 tokens came from Ethereum in the container chain
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                erc20_asset_id_for_container_context.clone(),
                ERC20_ASSET_ID,
                asset_sender.clone(),
                true,
                1
            )
        );

        // Creating foreign assets for the Relay native tokens in the container chain
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                relay_asset_id_for_container_context.clone(),
                RELAY_NATIVE_ASSET_ID,
                asset_sender.clone(),
                true,
                1
            )
        );

        // Minting foreign assets for the ERC20 tokens came from Ethereum in the container chain
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::mint(
                alice_origin.clone(),
                ERC20_ASSET_ID,
                asset_sender.clone().into(),
                ERC20_ASSET_AMOUNT
            )
        );

        // Minting foreign assets for the Relay native tokens in the container chain
        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::mint(
                alice_origin.clone(),
                RELAY_NATIVE_ASSET_ID,
                asset_sender.clone().into(),
                RELAY_ASSET_FEE_AMOUNT
            )
        );
    });

    // Check initial balances for the relay chain
    let mut treasury_fees_account_balance_before: u128 = 0;
    Dancelight::execute_with(|| {
        let container_chain_sovereign_account_erc20_balance_before =
            <Dancelight as DancelightRelayPallet>::ForeignAssets::balance(
                ERC20_ASSET_ID,
                container_sovereign_account.clone(),
            );
        assert_eq!(
            container_chain_sovereign_account_erc20_balance_before,
            ERC20_ASSET_AMOUNT
        );

        let container_chain_sovereign_account_system_balance_before =
            <Dancelight as DancelightRelayPallet>::System::account(
                container_sovereign_account.clone(),
            )
            .data
            .free;
        assert_eq!(
            container_chain_sovereign_account_system_balance_before,
            RELAY_ASSET_FEE_AMOUNT + dancelight_runtime_constants::currency::EXISTENTIAL_DEPOSIT
        );

        treasury_fees_account_balance_before =
            <Dancelight as DancelightRelayPallet>::System::account(TreasuryAccount::get())
                .data
                .free;
        assert_eq!(
            treasury_fees_account_balance_before,
            dancelight_runtime_constants::currency::EXISTENTIAL_DEPOSIT
        );
    });

    // Check initial balances for the container chain, execute the transfer and check result balances
    SimpleTemplate::execute_with(|| {
        let relay_destination = Location {
            parents: 1,
            interior: Here,
        };

        let erc20_asset_balance_before =
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::balance(
                ERC20_ASSET_ID,
                asset_sender.clone(),
            );
        assert_eq!(erc20_asset_balance_before, ERC20_ASSET_AMOUNT);

        let relay_asset_balance_before =
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::balance(
                RELAY_NATIVE_ASSET_ID,
                asset_sender.clone(),
            );
        assert_eq!(relay_asset_balance_before, RELAY_ASSET_FEE_AMOUNT);

        let fee_assets = AssetId(relay_asset_id_for_container_context.clone())
            .into_asset(Fungibility::Fungible(RELAY_ASSET_FEE_AMOUNT));
        let erc20_assets = AssetId(erc20_asset_id_for_container_context.clone())
            .into_asset(Fungibility::Fungible(ERC20_ASSET_AMOUNT));

        let erc20_asset_id_for_ethereum_context = Location {
            parents: 0,
            interior: Junctions::X1(
                [AccountKey20 {
                    network: Some(container_chain_template_simple_runtime::EthereumNetwork::get()),
                    key: ERC20_TOKEN_ADDRESS,
                }]
                .into(),
            ),
        };

        let beneficiary_address = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef, 0x01, 0x23, 0x45, 0x67,
        ];
        let beneficiary = Location {
            parents: 0,
            interior: Junctions::X1(
                [AccountKey20 {
                    network: Some(container_chain_template_simple_runtime::EthereumNetwork::get()),
                    key: beneficiary_address,
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
                    [GlobalConsensus(NetworkId::Ethereum {
                        chain_id: ethereum_network,
                    })]
                    .into(),
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
            <SimpleTemplate as Chain>::RuntimeOrigin::signed(asset_sender.clone());

        assert_ok!(
            <SimpleTemplate as SimpleTemplateParaPallet>::PolkadotXcm::transfer_assets_using_type_and_then(
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
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::balance(
                ERC20_ASSET_ID,
                asset_sender.clone(),
            );
        assert_eq!(erc20_asset_balance_after, 0);

        let relay_asset_balance_after =
            <SimpleTemplate as SimpleTemplateParaPallet>::ForeignAssets::balance(
                RELAY_NATIVE_ASSET_ID,
                asset_sender.clone(),
            );
        assert_eq!(relay_asset_balance_after, 0);
    });

    // Check result balances for the relay chain
    Dancelight::execute_with(|| {
        type RuntimeEvent = <Dancelight as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancelight,
            vec![
                RuntimeEvent::EthereumOutboundQueue(snowbridge_pallet_outbound_queue::Event::MessageAccepted { nonce: 1, id: _ }) => {},
            ]
        );

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
        assert_eq!(
            container_chain_sovereign_account_system_balance_after,
            dancelight_runtime_constants::currency::EXISTENTIAL_DEPOSIT
        );

        let treasury_fees_account_balance_after =
            <Dancelight as DancelightRelayPallet>::System::account(TreasuryAccount::get())
                .data
                .free;
        assert!(
            treasury_fees_account_balance_after
                > dancelight_runtime_constants::currency::EXISTENTIAL_DEPOSIT
        );

        // Check that fees were transferred to the treasury account
        assert!(treasury_fees_account_balance_after < RELAY_ASSET_FEE_AMOUNT);
    });
}

fn ethereum_chain_id<N: Get<NetworkId>>() -> u64 {
    match N::get() {
        NetworkId::Ethereum { chain_id } => chain_id,
        _ => panic!("Expected Ethereum NetworkId"),
    }
}
