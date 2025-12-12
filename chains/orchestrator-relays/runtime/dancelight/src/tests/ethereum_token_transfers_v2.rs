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

#![cfg(test)]

use {
    crate::{
        filter_events,
        tests::common::*,
        xcm_config::{LocationConverter, MinV2Reward},
        Balances, EthereumLocation, EthereumSovereignAccount, EthereumSystem, ForeignAssets,
        ForeignAssetsCreator, RuntimeEvent, SnowbridgeFeesAccount, XcmPallet,
    },
    alloc::vec,
    dancelight_runtime_constants::snowbridge::EthereumNetwork,
    dancelight_runtime_constants::DANCELIGHT_GENESIS_HASH,
    frame_support::{
        assert_err_ignore_postinfo, assert_ok,
        traits::{fungible::Inspect, fungibles::Mutate},
        BoundedVec,
    },
    hex_literal::hex,
    pallet_xcm::ExecutionError,
    sp_core::H160,
    tp_bridge::ConvertLocation,
    xcm::{
        latest::{prelude::*, AssetTransferFilter, Fungibility, Junctions::*, Location},
        VersionedLocation, VersionedXcm,
    },
};

#[test]
fn send_eth_native_token_works_v2() {
    ExtBuilder::default()
        .with_balances(vec![
            (EthereumSovereignAccount::get(), 100_000 * UNIT),
            (SnowbridgeFeesAccount::get(), 100_000 * UNIT),
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            sp_tracing::try_init_simple();
            let amount_to_transfer = 10_000u128;

            // Define a mock ERC20 token address
            let token_address = H160(hex!("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef"));

            let erc20_asset_location = Location {
                parents: 1,
                interior: X2([
                    GlobalConsensus(EthereumNetwork::get()),
                    AccountKey20 {
                        network: Some(EthereumNetwork::get()),
                        key: token_address.into(),
                    },
                ]
                .into()),
            };

            let asset_id = 42u16;

            assert_ok!(ForeignAssetsCreator::create_foreign_asset(
                root_origin(),
                erc20_asset_location.clone(), // Use the ERC20 location
                asset_id,
                AccountId::from(ALICE),
                true,
                1
            ));

            // Give tokens to user as if tokens were bridged before + extra to check the correct
            // amount is sent
            ForeignAssets::mint_into(asset_id, &AccountId::from(BOB), amount_to_transfer + 10)
                .expect("to mint amount");

            // User tries to send tokens
            let beneficiary_address = H160(hex!("0123456789abcdef0123456789abcdef01234567"));

            // Beneficiary location must be represented using destination's point of view.
            let beneficiary_location = Location {
                parents: 0,
                interior: X1([AccountKey20 {
                    network: Some(EthereumNetwork::get()),
                    key: beneficiary_address.into(),
                }]
                .into()),
            };

            let eth_asset = AssetId(erc20_asset_location.clone())
                .into_asset(Fungibility::Fungible(amount_to_transfer));

            let fee_amount_withdrawn = 1_000_000_000_000u128;
            let fee_amount = 600_000_000_000u128;

            let fee_asset_withdrawn = AssetId(crate::xcm_config::TokenLocation::get().clone())
                .into_asset(Fungibility::Fungible(fee_amount_withdrawn));

            let fee_asset = AssetId(crate::xcm_config::TokenLocation::get().clone())
                .into_asset(Fungibility::Fungible(fee_amount));

            let balance_before = Balances::balance(&AccountId::from(BOB));
            let balance_fees_account_before = Balances::balance(&SnowbridgeFeesAccount::get());

            let assets = vec![fee_asset_withdrawn.clone(), eth_asset.clone()];

            // fees should go into ethereumfeesAccount
            let xcm = VersionedXcm::from(Xcm(vec![
                WithdrawAsset(assets.clone().into()),
                InitiateTransfer {
                    destination: EthereumLocation::get(),
                    remote_fees: Some(AssetTransferFilter::ReserveDeposit(
                        fee_asset.clone().into(),
                    )),
                    preserve_origin: true,
                    assets: BoundedVec::truncate_from(vec![AssetTransferFilter::ReserveWithdraw(
                        Definite(eth_asset.clone().into()),
                    )]),
                    remote_xcm: Xcm(vec![DepositAsset {
                        assets: Wild(AllCounted(1)),
                        beneficiary: beneficiary_location,
                    }]),
                },
            ]));

            assert_ok!(XcmPallet::execute(
                RuntimeOrigin::signed(AccountId::from(BOB)),
                Box::new(xcm),
                Weight::from(16_000_000_000),
            ));

            // Correct amount has been sent
            assert_eq!(ForeignAssets::balance(asset_id, AccountId::from(BOB)), 10);

            // Check some fees have been payed. ideally this covers fee_amount plus weight exec
            let balance_after = Balances::balance(&AccountId::from(BOB));
            assert!(balance_before - balance_after > fee_amount);

            let balance_fees_account_after = Balances::balance(&SnowbridgeFeesAccount::get());
            assert_eq!(
                balance_fees_account_after - balance_fees_account_before,
                fee_amount
            );

            assert_eq!(
                filter_events!(RuntimeEvent::EthereumOutboundQueueV2(
                    snowbridge_pallet_outbound_queue_v2::Event::MessageQueued { .. },
                ))
                .count(),
                1,
                "MessageQueued event should be emitted!"
            );
        })
}

#[test]
fn send_eth_native_does_not_work_if_min_reward_not_covered() {
    ExtBuilder::default()
        .with_balances(vec![
            (EthereumSovereignAccount::get(), 100_000 * UNIT),
            (SnowbridgeFeesAccount::get(), 100_000 * UNIT),
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            sp_tracing::try_init_simple();
            let amount_to_transfer = 10_000u128;

            // Define a mock ERC20 token address
            let token_address = H160(hex!("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef"));

            let erc20_asset_location = Location {
                parents: 1,
                interior: X2([
                    GlobalConsensus(EthereumNetwork::get()),
                    AccountKey20 {
                        network: Some(EthereumNetwork::get()),
                        key: token_address.into(),
                    },
                ]
                .into()),
            };

            let asset_id = 42u16;

            assert_ok!(ForeignAssetsCreator::create_foreign_asset(
                root_origin(),
                erc20_asset_location.clone(), // Use the ERC20 location
                asset_id,
                AccountId::from(ALICE),
                true,
                1
            ));

            // Give tokens to user as if tokens were bridged before + extra to check the correct
            // amount is sent
            ForeignAssets::mint_into(asset_id, &AccountId::from(BOB), amount_to_transfer + 10)
                .expect("to mint amount");

            // User tries to send tokens
            let beneficiary_address = H160(hex!("0123456789abcdef0123456789abcdef01234567"));

            // Beneficiary location must be represented using destination's point of view.
            let beneficiary_location = Location {
                parents: 0,
                interior: X1([AccountKey20 {
                    network: Some(EthereumNetwork::get()),
                    key: beneficiary_address.into(),
                }]
                .into()),
            };

            let eth_asset = AssetId(erc20_asset_location.clone())
                .into_asset(Fungibility::Fungible(amount_to_transfer));

            let fee_amount_withdrawn = 1_000_000_000_000u128;

            let fee_asset_withdrawn = AssetId(crate::xcm_config::TokenLocation::get().clone())
                .into_asset(Fungibility::Fungible(fee_amount_withdrawn));

            let fee_amount = MinV2Reward::get().saturating_sub(1);

            let fee_asset = AssetId(crate::xcm_config::TokenLocation::get().clone())
                .into_asset(Fungibility::Fungible(fee_amount));

            let assets = vec![fee_asset_withdrawn.clone(), eth_asset.clone()];

            // fees should go into ethereumfeesAccount
            let xcm = VersionedXcm::from(Xcm(vec![
                WithdrawAsset(assets.clone().into()),
                InitiateTransfer {
                    destination: EthereumLocation::get(),
                    remote_fees: Some(AssetTransferFilter::ReserveDeposit(
                        fee_asset.clone().into(),
                    )),
                    preserve_origin: true,
                    assets: BoundedVec::truncate_from(vec![AssetTransferFilter::ReserveWithdraw(
                        Definite(eth_asset.clone().into()),
                    )]),
                    remote_xcm: Xcm(vec![DepositAsset {
                        assets: Wild(AllCounted(1)),
                        beneficiary: beneficiary_location,
                    }]),
                },
            ]));

            // TODO: REVISIT THE ERROR
            // the error is that because it then tries to export through the other exporter and
            // the last error is toomany asserts
            // however one can check that if fee_amount is MinV2Reward::get().saturating_add(1),
            // everything works
            // Not sure how to make the test more expresive
            assert_err_ignore_postinfo!(
                XcmPallet::execute(
                    RuntimeOrigin::signed(AccountId::from(BOB)),
                    Box::new(xcm),
                    Weight::from(16_000_000_000),
                ),
                pallet_xcm::Error::<Runtime>::LocalExecutionIncompleteWithError {
                    index: 1,
                    error: ExecutionError::TooManyAssets
                }
            );
        })
}

#[test]
fn send_eth_native_does_not_work_if_reward_diff_asset() {
    ExtBuilder::default()
        .with_balances(vec![
            (EthereumSovereignAccount::get(), 100_000 * UNIT),
            (SnowbridgeFeesAccount::get(), 100_000 * UNIT),
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            sp_tracing::try_init_simple();
            let amount_to_transfer = 10_000u128;

            // Define a mock ERC20 token address
            let token_address = H160(hex!("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef"));

            let erc20_asset_location = Location {
                parents: 1,
                interior: X2([
                    GlobalConsensus(EthereumNetwork::get()),
                    AccountKey20 {
                        network: Some(EthereumNetwork::get()),
                        key: token_address.into(),
                    },
                ]
                .into()),
            };

            let asset_id = 42u16;

            assert_ok!(ForeignAssetsCreator::create_foreign_asset(
                root_origin(),
                erc20_asset_location.clone(), // Use the ERC20 location
                asset_id,
                AccountId::from(ALICE),
                true,
                1
            ));

            // Give tokens to user as if tokens were bridged before + extra to check the correct
            // amount is sent
            ForeignAssets::mint_into(asset_id, &AccountId::from(BOB), amount_to_transfer + 10)
                .expect("to mint amount");

            // User tries to send tokens
            let beneficiary_address = H160(hex!("0123456789abcdef0123456789abcdef01234567"));

            // Beneficiary location must be represented using destination's point of view.
            let beneficiary_location = Location {
                parents: 0,
                interior: X1([AccountKey20 {
                    network: Some(EthereumNetwork::get()),
                    key: beneficiary_address.into(),
                }]
                .into()),
            };

            let eth_asset = AssetId(erc20_asset_location.clone())
                .into_asset(Fungibility::Fungible(amount_to_transfer));

            let fee_amount_withdrawn = 1_000_000_000_000u128;

            let fee_asset_withdrawn = AssetId(crate::xcm_config::TokenLocation::get().clone())
                .into_asset(Fungibility::Fungible(fee_amount_withdrawn));

            let fee_amount = 1_000_000_000u128;

            // Instead of the proper asset, we put another one here. for instance, the eth one
            // only paying in tanssi is allowed
            let fee_asset =
                AssetId(erc20_asset_location.clone()).into_asset(Fungibility::Fungible(fee_amount));

            let assets = vec![fee_asset_withdrawn.clone(), eth_asset.clone()];

            // fees should go into ethereumfeesAccount
            let xcm = VersionedXcm::from(Xcm(vec![
                WithdrawAsset(assets.clone().into()),
                InitiateTransfer {
                    destination: EthereumLocation::get(),
                    remote_fees: Some(AssetTransferFilter::ReserveDeposit(
                        fee_asset.clone().into(),
                    )),
                    preserve_origin: true,
                    assets: BoundedVec::truncate_from(vec![AssetTransferFilter::ReserveWithdraw(
                        Definite(eth_asset.clone().into()),
                    )]),
                    remote_xcm: Xcm(vec![DepositAsset {
                        assets: Wild(AllCounted(1)),
                        beneficiary: beneficiary_location,
                    }]),
                },
            ]));

            assert_err_ignore_postinfo!(
                XcmPallet::execute(
                    RuntimeOrigin::signed(AccountId::from(BOB)),
                    Box::new(xcm),
                    Weight::from(16_000_000_000),
                ),
                pallet_xcm::Error::<Runtime>::LocalExecutionIncompleteWithError {
                    index: 1,
                    error: ExecutionError::NotHoldingFees
                }
            );
        })
}

#[test]
fn sending_eth_native_withdrawing_non_sufficient_amount_eth_works_but_only_sends_withdrawn() {
    ExtBuilder::default()
        .with_balances(vec![
            (EthereumSovereignAccount::get(), 100_000 * UNIT),
            (SnowbridgeFeesAccount::get(), 100_000 * UNIT),
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            sp_tracing::try_init_simple();
            let amount_to_transfer = 10_000u128;

            // Define a mock ERC20 token address
            let token_address = H160(hex!("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef"));

            let erc20_asset_location = Location {
                parents: 1,
                interior: X2([
                    GlobalConsensus(EthereumNetwork::get()),
                    AccountKey20 {
                        network: Some(EthereumNetwork::get()),
                        key: token_address.into(),
                    },
                ]
                .into()),
            };

            let asset_id = 42u16;

            assert_ok!(ForeignAssetsCreator::create_foreign_asset(
                root_origin(),
                erc20_asset_location.clone(), // Use the ERC20 location
                asset_id,
                AccountId::from(ALICE),
                true,
                1
            ));

            // Give tokens to user as if tokens were bridged before + extra to check the correct
            // amount is sent
            ForeignAssets::mint_into(asset_id, &AccountId::from(BOB), amount_to_transfer + 10)
                .expect("to mint amount");

            // User tries to send tokens
            let beneficiary_address = H160(hex!("0123456789abcdef0123456789abcdef01234567"));

            // Beneficiary location must be represented using destination's point of view.
            let beneficiary_location = Location {
                parents: 0,
                interior: X1([AccountKey20 {
                    network: Some(EthereumNetwork::get()),
                    key: beneficiary_address.into(),
                }]
                .into()),
            };

            let eth_asset = AssetId(erc20_asset_location.clone())
                .into_asset(Fungibility::Fungible(amount_to_transfer));

            // We withdraw less than what is allowed to be transferred
            // We only withdraw 1 unit instead of 10_000
            let eth_asset_withdrawn =
                AssetId(erc20_asset_location.clone()).into_asset(Fungibility::Fungible(1));

            // We are withdrawing more in the fee than what we need
            let fee_amount_withdrawn = 1_000_000_000_000u128;

            let fee_asset_withdrawn = AssetId(crate::xcm_config::TokenLocation::get().clone())
                .into_asset(Fungibility::Fungible(fee_amount_withdrawn));

            let fee_amount = 1_000_000_000u128;

            // Instead of the proper asset, we put another one here. for instance, the eth one
            // only paying in tanssi is allowed
            let fee_asset = AssetId(crate::xcm_config::TokenLocation::get().clone())
                .into_asset(Fungibility::Fungible(fee_amount));

            // WithdrawnAssets
            // 1e12 tanssi
            // 1 unit ETH
            let assets = vec![fee_asset_withdrawn.clone(), eth_asset_withdrawn.clone()];

            let balance_before = Balances::balance(&AccountId::from(BOB));
            let balance_fees_account_before = Balances::balance(&SnowbridgeFeesAccount::get());

            let xcm = VersionedXcm::from(Xcm(vec![
                WithdrawAsset(assets.clone().into()),
                InitiateTransfer {
                    destination: EthereumLocation::get(),
                    remote_fees: Some(AssetTransferFilter::ReserveDeposit(
                        fee_asset.clone().into(),
                    )),
                    preserve_origin: true,
                    assets: BoundedVec::truncate_from(vec![AssetTransferFilter::ReserveWithdraw(
                        Definite(eth_asset.clone().into()),
                    )]),
                    remote_xcm: Xcm(vec![DepositAsset {
                        assets: Wild(AllCounted(1)),
                        beneficiary: beneficiary_location,
                    }]),
                },
            ]));

            assert_ok!(XcmPallet::execute(
                RuntimeOrigin::signed(AccountId::from(BOB)),
                Box::new(xcm),
                Weight::from(16_000_000_000),
            ));

            // Correct amount has been sent
            assert_eq!(
                ForeignAssets::balance(asset_id, AccountId::from(BOB)),
                amount_to_transfer + 9
            );

            // Check some fees have been payed. ideally this covers fee_amount plus weight exec
            let balance_after = Balances::balance(&AccountId::from(BOB));
            assert!(balance_before - balance_after > fee_amount);

            let balance_fees_account_after = Balances::balance(&SnowbridgeFeesAccount::get());
            assert_eq!(
                balance_fees_account_after - balance_fees_account_before,
                fee_amount
            );

            assert_eq!(
                filter_events!(RuntimeEvent::EthereumOutboundQueueV2(
                    snowbridge_pallet_outbound_queue_v2::Event::MessageQueued { .. },
                ))
                .count(),
                1,
                "MessageQueued event should be emitted!"
            );
        })
}

#[test]
fn sending_eth_native_withdrawing_non_sufficient_amount_fee_does_not_work() {
    ExtBuilder::default()
        .with_balances(vec![
            (EthereumSovereignAccount::get(), 100_000 * UNIT),
            (SnowbridgeFeesAccount::get(), 100_000 * UNIT),
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            sp_tracing::try_init_simple();
            let amount_to_transfer = 10_000u128;

            // Define a mock ERC20 token address
            let token_address = H160(hex!("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef"));

            let erc20_asset_location = Location {
                parents: 1,
                interior: X2([
                    GlobalConsensus(EthereumNetwork::get()),
                    AccountKey20 {
                        network: Some(EthereumNetwork::get()),
                        key: token_address.into(),
                    },
                ]
                .into()),
            };

            let asset_id = 42u16;

            assert_ok!(ForeignAssetsCreator::create_foreign_asset(
                root_origin(),
                erc20_asset_location.clone(), // Use the ERC20 location
                asset_id,
                AccountId::from(ALICE),
                true,
                1
            ));

            // Give tokens to user as if tokens were bridged before + extra to check the correct
            // amount is sent
            ForeignAssets::mint_into(asset_id, &AccountId::from(BOB), amount_to_transfer + 10)
                .expect("to mint amount");

            // User tries to send tokens
            let beneficiary_address = H160(hex!("0123456789abcdef0123456789abcdef01234567"));

            // Beneficiary location must be represented using destination's point of view.
            let beneficiary_location = Location {
                parents: 0,
                interior: X1([AccountKey20 {
                    network: Some(EthereumNetwork::get()),
                    key: beneficiary_address.into(),
                }]
                .into()),
            };

            let eth_asset = AssetId(erc20_asset_location.clone())
                .into_asset(Fungibility::Fungible(amount_to_transfer));

            // We withdraw less thant what is allowed
            let fee_amount_withdrawn = 1u128;

            let fee_asset_withdrawn = AssetId(crate::xcm_config::TokenLocation::get().clone())
                .into_asset(Fungibility::Fungible(fee_amount_withdrawn));

            let fee_amount = 1_000_000_000u128;

            // Instead of the proper asset, we put another one here. for instance, the eth one
            // only paying in tanssi is allowed
            let fee_asset = AssetId(crate::xcm_config::TokenLocation::get().clone())
                .into_asset(Fungibility::Fungible(fee_amount));

            let assets = vec![fee_asset_withdrawn.clone(), eth_asset.clone()];

            let xcm = VersionedXcm::from(Xcm(vec![
                WithdrawAsset(assets.clone().into()),
                InitiateTransfer {
                    destination: EthereumLocation::get(),
                    remote_fees: Some(AssetTransferFilter::ReserveDeposit(
                        fee_asset.clone().into(),
                    )),
                    preserve_origin: true,
                    assets: BoundedVec::truncate_from(vec![AssetTransferFilter::ReserveWithdraw(
                        Definite(eth_asset.clone().into()),
                    )]),
                    remote_xcm: Xcm(vec![DepositAsset {
                        assets: Wild(AllCounted(1)),
                        beneficiary: beneficiary_location,
                    }]),
                },
            ]));

            assert_err_ignore_postinfo!(
                XcmPallet::execute(
                    RuntimeOrigin::signed(AccountId::from(BOB)),
                    Box::new(xcm),
                    Weight::from(16_000_000_000),
                ),
                pallet_xcm::Error::<Runtime>::LocalExecutionIncompleteWithError {
                    index: 1,
                    error: ExecutionError::NotHoldingFees
                }
            );
        })
}

#[test]
fn test_set_fees_mode_payment_succeeds_container_chain_to_eth_transfer_v2() {
    ExtBuilder::default()
        .with_balances(vec![
            (EthereumSovereignAccount::get(), 100_000 * UNIT),
            (SnowbridgeFeesAccount::get(), 100_000 * UNIT),
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            sp_tracing::try_init_simple();

            // Define parachain 2000 location for DescendOrigin
            let parachain_2000_location = Junctions::X1([Parachain(2000)].into());

            // Define Tanssi asset to withdraw
            let fee_amount_withdrawn = 2_700_000_000_000u128;
            let fee_asset_withdrawn =
                AssetId(Location::here()).into_asset(Fungibility::Fungible(fee_amount_withdrawn));

            // Define the container location
            let container_location = Location::new(0, Parachain(2000));
            let container_sovereign_account =
                LocationConverter::convert_location(&container_location).unwrap();

            // Mint relay tokens to the container sovereign account
            use frame_support::traits::fungible::Mutate;
            let initial_sovereign_balance = 100_000_000 * UNIT;
            Balances::mint_into(&container_sovereign_account, initial_sovereign_balance)
                .expect("mint to container sovereign account");

            // Define a topic for SetTopic
            let topic = [1u8; 32];

            // Define container chain asset for export message view
            let container_asset_location: Location = Location::new(
                1,
                [
                    GlobalConsensus(ByGenesis(DANCELIGHT_GENESIS_HASH)),
                    Parachain(2000),
                    PalletInstance(10),
                ],
            );

            // Register the container token on EthereumSystem
            assert_ok!(EthereumSystem::register_token(
                root_origin(),
                Box::new(VersionedLocation::V5(container_asset_location.clone())),
                snowbridge_core::AssetMetadata {
                    name: "container".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "CTR".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            ));

            let container_asset = AssetId(container_asset_location.clone())
                .into_asset(Fungibility::Fungible(1_000_000_000_000u128));

            // Define fee for export message (higher than fee_amount_withdrawn)
            let export_fee_amount = 4_000_000_000_000u128;
            let export_fee_asset = AssetId(Location::new(
                1,
                GlobalConsensus(ByGenesis(DANCELIGHT_GENESIS_HASH)),
            ))
            .into_asset(Fungibility::Fungible(export_fee_amount));

            // Define container origin for AliasOrigin in export message
            let container_origin = Location::new(
                0,
                [
                    Parachain(2000),
                    AccountKey20 {
                        network: None,
                        key: [0u8; 20],
                    },
                ],
            );

            // Define Ethereum beneficiary
            let eth_beneficiary = Location::new(
                0,
                [AccountKey20 {
                    network: Some(EthereumNetwork::get()),
                    key: hex!("2000000000000000000000000000000000000000"),
                }],
            );

            let export_topic = [2u8; 32];

            // Construct the export XCM message that will pass validation in container_token_to_ethereum_message_exporter_v2
            let export_xcm = Xcm(vec![
                WithdrawAsset(export_fee_asset.clone().into()),
                PayFees {
                    asset: export_fee_asset.clone(),
                },
                ReserveAssetDeposited(container_asset.clone().into()),
                AliasOrigin(container_origin),
                DepositAsset {
                    assets: Wild(AllCounted(1)),
                    beneficiary: eth_beneficiary,
                },
                SetTopic(export_topic),
            ]);

            // Construct the XCM message
            let xcm = VersionedXcm::from(Xcm(vec![
                DescendOrigin(parachain_2000_location),
                WithdrawAsset(fee_asset_withdrawn.clone().into()),
                BuyExecution {
                    fees: fee_asset_withdrawn.clone(),
                    weight_limit: Unlimited,
                },
                SetFeesMode { jit_withdraw: true },
                SetAppendix(Xcm(vec![DepositAsset {
                    assets: AllCounted(1).into(),
                    beneficiary: container_location,
                }])),
                ExportMessage {
                    network: EthereumNetwork::get(),
                    destination: Here,
                    xcm: export_xcm,
                },
                SetTopic(topic),
            ]));

            // Execute via root
            assert_ok!(XcmPallet::execute(
                root_origin(),
                Box::new(xcm),
                Weight::from(16_000_000_000),
            ));

            // Verify fee deductions
            let final_sovereign_balance = Balances::balance(&container_sovereign_account);
            let total_deducted = initial_sovereign_balance - final_sovereign_balance;

            // The export_fee_amount should have been deducted from the container sovereign account.
            assert!(
                total_deducted >= export_fee_amount,
                "Expected at least {} to be deducted, but only {} was deducted",
                export_fee_amount,
                total_deducted
            );

            // Check that the deducted amount includes both the export fee and some execution costs
            // but also accounts for potential refunds.
            assert!(
                total_deducted < (fee_amount_withdrawn + export_fee_amount),
                "Total deducted {} should not exceed the total withdrawn amount {}",
                total_deducted,
                (fee_amount_withdrawn + export_fee_amount)
            );

            // Verify that a message was queued for Ethereum
            assert_eq!(
                filter_events!(RuntimeEvent::EthereumOutboundQueueV2(
                    snowbridge_pallet_outbound_queue_v2::Event::MessageQueued { .. },
                ))
                .count(),
                1,
                "MessageQueued event should be emitted!"
            );
        })
}

#[test]
fn test_set_fees_mode_payment_fails_if_fees_mode_is_not_set_v2() {
    ExtBuilder::default()
        .with_balances(vec![
            (EthereumSovereignAccount::get(), 100_000 * UNIT),
            (SnowbridgeFeesAccount::get(), 100_000 * UNIT),
            (AccountId::from(ALICE), 100_000 * UNIT),
            (AccountId::from(BOB), 100_000 * UNIT),
        ])
        .build()
        .execute_with(|| {
            sp_tracing::try_init_simple();

            // Define parachain 2000 location for DescendOrigin
            let parachain_2000_location = Junctions::X1([Parachain(2000)].into());

            // Define Tanssi asset to withdraw
            let fee_amount_withdrawn = 2_700_000_000_000u128;
            let fee_asset_withdrawn =
                AssetId(Location::here()).into_asset(Fungibility::Fungible(fee_amount_withdrawn));

            // Define the container location
            let container_location = Location::new(0, Parachain(2000));
            let container_sovereign_account =
                LocationConverter::convert_location(&container_location).unwrap();

            // Mint relay tokens to the container sovereign account
            use frame_support::traits::fungible::Mutate;
            let initial_sovereign_balance = 100_000_000 * UNIT;
            Balances::mint_into(&container_sovereign_account, initial_sovereign_balance)
                .expect("mint to container sovereign account");

            // Define a topic for SetTopic
            let topic = [1u8; 32];

            // Define container chain asset for export message view
            let container_asset_location: Location = Location::new(
                1,
                [
                    GlobalConsensus(ByGenesis(DANCELIGHT_GENESIS_HASH)),
                    Parachain(2000),
                    PalletInstance(10),
                ],
            );

            // Register the container token on EthereumSystem
            assert_ok!(EthereumSystem::register_token(
                root_origin(),
                Box::new(VersionedLocation::V5(container_asset_location.clone())),
                snowbridge_core::AssetMetadata {
                    name: "container".as_bytes().to_vec().try_into().unwrap(),
                    symbol: "CTR".as_bytes().to_vec().try_into().unwrap(),
                    decimals: 12,
                }
            ));

            let container_asset = AssetId(container_asset_location.clone())
                .into_asset(Fungibility::Fungible(1_000_000_000_000u128));

            // Define fee for export message (higher than fee_amount_withdrawn)
            let export_fee_amount = 4_000_000_000_000u128;
            let export_fee_asset = AssetId(Location::new(
                1,
                GlobalConsensus(ByGenesis(DANCELIGHT_GENESIS_HASH)),
            ))
            .into_asset(Fungibility::Fungible(export_fee_amount));

            // Define container origin for AliasOrigin in export message
            let container_origin = Location::new(
                0,
                [
                    Parachain(2000),
                    AccountKey20 {
                        network: None,
                        key: [0u8; 20],
                    },
                ],
            );

            // Define Ethereum beneficiary
            let eth_beneficiary = Location::new(
                0,
                [AccountKey20 {
                    network: Some(EthereumNetwork::get()),
                    key: hex!("2000000000000000000000000000000000000000"),
                }],
            );

            let export_topic = [2u8; 32];

            // Construct the export XCM message that will pass validation in container_token_to_ethereum_message_exporter_v2
            let export_xcm = Xcm(vec![
                WithdrawAsset(export_fee_asset.clone().into()),
                PayFees {
                    asset: export_fee_asset.clone(),
                },
                ReserveAssetDeposited(container_asset.clone().into()),
                AliasOrigin(container_origin),
                DepositAsset {
                    assets: Wild(AllCounted(1)),
                    beneficiary: eth_beneficiary,
                },
                SetTopic(export_topic),
            ]);

            // Construct the XCM message
            let xcm = VersionedXcm::from(Xcm(vec![
                DescendOrigin(parachain_2000_location),
                WithdrawAsset(fee_asset_withdrawn.clone().into()),
                BuyExecution {
                    fees: fee_asset_withdrawn.clone(),
                    weight_limit: Unlimited,
                },
                // We don't set the fees mode here on purpose. This will make the execution to fail as the
                // export fees needed will not be in holding, and they will not be deducted from
                // the AssetTransactor either as SetFeesMode is missing.
                //SetFeesMode { jit_withdraw: true },
                SetAppendix(Xcm(vec![DepositAsset {
                    assets: AllCounted(1).into(),
                    beneficiary: container_location,
                }])),
                ExportMessage {
                    network: EthereumNetwork::get(),
                    destination: Here,
                    xcm: export_xcm,
                },
                SetTopic(topic),
            ]));

            // Execute via root - this should fail with NotHoldingFees
            assert_err_ignore_postinfo!(
                XcmPallet::execute(root_origin(), Box::new(xcm), Weight::from(16_000_000_000),),
                pallet_xcm::Error::<Runtime>::LocalExecutionIncompleteWithError {
                    index: 4,
                    error: ExecutionError::NotHoldingFees
                }
            );

            // Verify that NO message was queued for Ethereum
            assert_eq!(
                filter_events!(RuntimeEvent::EthereumOutboundQueueV2(
                    snowbridge_pallet_outbound_queue_v2::Event::MessageQueued { .. },
                ))
                .count(),
                0,
                "MessageQueued event should NOT be emitted when execution fails!"
            );
        })
}
