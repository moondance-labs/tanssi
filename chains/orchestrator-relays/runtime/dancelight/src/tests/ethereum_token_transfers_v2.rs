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
        filter_events, tests::common::*, xcm_config::MinV2Reward, Balances, EthereumLocation,
        EthereumSovereignAccount, ForeignAssets, ForeignAssetsCreator, RuntimeEvent,
        SnowbridgeFeesAccount, XcmPallet,
    },
    alloc::vec,
    dancelight_runtime_constants::snowbridge::EthereumNetwork,
    frame_support::{
        assert_err_ignore_postinfo, assert_ok,
        traits::{fungible::Inspect, fungibles::Mutate},
        BoundedVec,
    },
    hex_literal::hex,
    pallet_xcm::ExecutionError,
    sp_core::H160,
    xcm::{
        latest::{prelude::*, AssetTransferFilter, Fungibility, Junctions::*, Location},
        VersionedXcm,
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
