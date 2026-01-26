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
    crate::{mock::*, Error},
    frame_support::{assert_noop, assert_ok},
    sp_runtime::FixedU128,
};

#[test]
fn set_token_price_works() {
    new_test_ext().execute_with(|| {
        // Price not set initially
        assert!(ServicesPaymentPriceOracle::token_price_usd().is_none());

        // Set price to $1.00 (1 * 10^18 in FixedU128)
        let price = FixedU128::from_u32(1);
        assert_ok!(ServicesPaymentPriceOracle::set_token_price(
            RuntimeOrigin::root(),
            price
        ));

        // Verify price is set
        assert_eq!(ServicesPaymentPriceOracle::token_price_usd(), Some(price));
    });
}

#[test]
fn set_token_price_zero_fails() {
    new_test_ext().execute_with(|| {
        let zero_price = FixedU128::from_u32(0);
        assert_noop!(
            ServicesPaymentPriceOracle::set_token_price(RuntimeOrigin::root(), zero_price),
            Error::<Test>::PriceCannotBeZero
        );
    });
}

#[test]
fn set_token_price_non_root_fails() {
    new_test_ext().execute_with(|| {
        let price = FixedU128::from_u32(1);
        assert_noop!(
            ServicesPaymentPriceOracle::set_token_price(RuntimeOrigin::signed(ALICE), price),
            sp_runtime::DispatchError::BadOrigin
        );
    });
}

#[test]
fn genesis_config_sets_price() {
    // $0.50 price = 0.5 * 10^18
    let initial_price = FixedU128::from_rational(1, 2).into_inner();
    new_test_ext_with_price(initial_price).execute_with(|| {
        assert_eq!(
            ServicesPaymentPriceOracle::token_price_usd(),
            Some(FixedU128::from_inner(initial_price))
        );
    });
}

#[test]
fn blocks_per_month_calculation() {
    new_test_ext().execute_with(|| {
        // With 6 second blocks:
        // blocks_per_month = (30 * 24 * 60 * 60 * 1000) / 6000 = 432_000
        let blocks_per_month = ServicesPaymentPriceOracle::blocks_per_month();
        assert_eq!(blocks_per_month, 432_000);
    });
}

#[test]
fn sessions_per_month_calculation() {
    new_test_ext().execute_with(|| {
        // With 600 blocks per session and 432_000 blocks per month:
        // sessions_per_month = 432_000 / 600 = 720
        let sessions_per_month = ServicesPaymentPriceOracle::sessions_per_month();
        assert_eq!(sessions_per_month, 720);
    });
}

#[test]
fn one_token_calculation() {
    new_test_ext().execute_with(|| {
        // With 12 decimals, one token = 10^12
        let one_token = ServicesPaymentPriceOracle::one_token();
        assert_eq!(one_token, 1_000_000_000_000);
    });
}

#[test]
fn calculate_costs_returns_none_when_price_not_set() {
    new_test_ext().execute_with(|| {
        assert!(ServicesPaymentPriceOracle::calculate_block_production_cost().is_none());
        assert!(ServicesPaymentPriceOracle::calculate_collator_assignment_cost().is_none());
    });
}

#[test]
fn ratio_is_preserved_between_block_and_session_costs() {
    // Price = $1.00
    let price = FixedU128::from_u32(1).into_inner();
    new_test_ext_with_price(price).execute_with(|| {
        let block_cost = ServicesPaymentPriceOracle::calculate_block_production_cost().unwrap();
        let session_cost =
            ServicesPaymentPriceOracle::calculate_collator_assignment_cost().unwrap();

        // Reference ratio: 0.03 : 50 = 30_000_000_000 : 50_000_000_000_000 = 1 : 1666.67
        // Check that the ratio is approximately preserved
        let actual_ratio = session_cost as f64 / block_cost as f64;
        let expected_ratio = 50_000_000_000_000_f64 / 30_000_000_000_f64; // ~1666.67

        // Allow 1% tolerance for rounding
        let diff = (actual_ratio - expected_ratio).abs() / expected_ratio;
        assert!(
            diff < 0.01,
            "Ratio not preserved: actual={}, expected={}, diff={}%",
            actual_ratio,
            expected_ratio,
            diff * 100.0
        );
    });
}

#[test]
fn total_monthly_cost_matches_target() {
    // Price = $1.00
    let price = FixedU128::from_u32(1).into_inner();
    new_test_ext_with_price(price).execute_with(|| {
        let block_cost = ServicesPaymentPriceOracle::calculate_block_production_cost().unwrap();
        let session_cost =
            ServicesPaymentPriceOracle::calculate_collator_assignment_cost().unwrap();

        let blocks_per_month = ServicesPaymentPriceOracle::blocks_per_month();
        let sessions_per_month = ServicesPaymentPriceOracle::sessions_per_month();

        // Total monthly cost in tokens
        let total_monthly_tokens =
            block_cost * blocks_per_month + session_cost * sessions_per_month;

        // With $1 price, monthly_cost_usd ($2000) should equal total_monthly_tokens in dollars
        // $2000 = 2_000_000_000 micro USD = 2000 * 10^12 token base units at $1 price
        let expected_tokens = 2000 * 1_000_000_000_000_u128; // 2000 tokens

        // Allow 1% tolerance for rounding
        let diff = total_monthly_tokens.abs_diff(expected_tokens);
        let tolerance = expected_tokens / 100; // 1%

        assert!(
            diff <= tolerance,
            "Monthly cost mismatch: actual={} tokens, expected={} tokens, diff={}",
            total_monthly_tokens as f64 / 1_000_000_000_000_f64,
            expected_tokens as f64 / 1_000_000_000_000_f64,
            diff as f64 / 1_000_000_000_000_f64
        );
    });
}

#[test]
fn price_can_be_updated_multiple_times() {
    new_test_ext().execute_with(|| {
        let price1 = FixedU128::from_u32(1);
        assert_ok!(ServicesPaymentPriceOracle::set_token_price(
            RuntimeOrigin::root(),
            price1
        ));
        assert_eq!(ServicesPaymentPriceOracle::token_price_usd(), Some(price1));

        run_to_block(5);

        let price2 = FixedU128::from_u32(2);
        assert_ok!(ServicesPaymentPriceOracle::set_token_price(
            RuntimeOrigin::root(),
            price2
        ));
        assert_eq!(ServicesPaymentPriceOracle::token_price_usd(), Some(price2));
    });
}

#[test]
fn event_emitted_on_price_update() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let price = FixedU128::from_u32(1);
        assert_ok!(ServicesPaymentPriceOracle::set_token_price(
            RuntimeOrigin::root(),
            price
        ));

        // Check that the event was emitted
        System::assert_has_event(RuntimeEvent::ServicesPaymentPriceOracle(
            crate::Event::PriceUpdated { new_price: price },
        ));
    });
}

#[test]
fn set_token_price_below_minimum_fails() {
    new_test_ext().execute_with(|| {
        // Price below minimum ($0.00003)
        let price = FixedU128::from_inner(30_000_000_000_000);
        assert_noop!(
            ServicesPaymentPriceOracle::set_token_price(RuntimeOrigin::root(), price),
            Error::<Test>::PriceOutOfBounds
        );
    });
}

#[test]
fn set_token_price_above_maximum_fails() {
    new_test_ext().execute_with(|| {
        // Price above maximum ($11)
        let price = FixedU128::from_u32(11);
        assert_noop!(
            ServicesPaymentPriceOracle::set_token_price(RuntimeOrigin::root(), price),
            Error::<Test>::PriceOutOfBounds
        );
    });
}

#[test]
fn set_token_price_at_bounds_succeeds() {
    new_test_ext().execute_with(|| {
        // Price at minimum bound ($0.00004)
        let min_price = FixedU128::from_inner(MinTokenPrice::get());
        assert_ok!(ServicesPaymentPriceOracle::set_token_price(
            RuntimeOrigin::root(),
            min_price
        ));
        assert_eq!(
            ServicesPaymentPriceOracle::token_price_usd(),
            Some(min_price)
        );

        // Price at maximum bound ($10)
        let max_price = FixedU128::from_inner(MaxTokenPrice::get());
        assert_ok!(ServicesPaymentPriceOracle::set_token_price(
            RuntimeOrigin::root(),
            max_price
        ));
        assert_eq!(
            ServicesPaymentPriceOracle::token_price_usd(),
            Some(max_price)
        );
    });
}
