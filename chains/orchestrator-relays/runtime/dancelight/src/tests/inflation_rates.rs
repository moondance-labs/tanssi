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

use crate::{
    CollatorsInflationRatePerBlock, EpochDurationInBlocks, Perbill, SessionsPerEra,
    ValidatorsInflationRatePerEra, DAYS,
};

#[derive(Debug)]
struct InflationRates {
    era_inflation: f64,
    collators_block_inflation: f64,
    validators_era_inflation: f64,
}

/// Computes the following inflation rates:
/// - Collators inflation rate (per block)
/// - Validators inflation rate (per era)
/// Era inflation is split between collators and validators using collators_fraction
fn compute_inflation_rates(
    annual_inflation: f64,
    collators_fraction: f64,
    eras_per_year: u32,
    blocks_per_era: u32,
) -> InflationRates {
    assert!(collators_fraction >= 0.0 && collators_fraction <= 1.0, "collators_fraction must be between 0 and 1");
    assert!(annual_inflation >= 0.0 && annual_inflation <= 1.0, "annual_inflation is a % and should be between 0 (0%) and 1 (100%)");

    // Compute era inflation based on annual inflation
    let era_inflation = (1.0 + annual_inflation).powf(1.0 / (eras_per_year as f64)) - 1.0;

    // Compute collators and validators era inflation
    let collators_era_inflation = (1.0 + era_inflation).powf(collators_fraction) - 1.0;
    let validators_era_inflation = (1.0 + era_inflation).powf(1.0 - collators_fraction) - 1.0;

    // Compute collator block inflation
    let collators_block_inflation =
        (1.0 + collators_era_inflation).powf(1.0 / (blocks_per_era as f64)) - 1.0;

    InflationRates {
        era_inflation,
        collators_block_inflation,
        validators_era_inflation,
    }
}

#[test]
fn formula_is_sound() {
    let eras_per_year = 100;
    let blocks_per_era = 100;
    let annual_inflation = 0.1; // 10%

    let rates = compute_inflation_rates(10.0, 0.6, eras_per_year, blocks_per_era);

    println!("Rates: {rates:?}");

    let col_inf = Perbill::from_float(rates.collators_block_inflation);
    let val_inf = Perbill::from_float(rates.validators_era_inflation);

    // "big" supply to reduce rounding errors
    let initial_supply = 100_000_000_000_000_000u128;
    let mut supply = initial_supply;

    for era in 0..eras_per_year {
        let era_start_supply = supply;

        for _block in 0..blocks_per_era {
            supply += col_inf * supply;
        }

        supply += val_inf * supply;

        let actual_era_inflation = supply as f64 / era_start_supply as f64 - 1.0;

        println!("Era {era}: Supply {supply}, Actual inf: {actual_era_inflation}");
        assert!((actual_era_inflation - rates.era_inflation).abs() < 0.00001);
    }

    let actual_annual_inflation = supply as f64 / initial_supply as f64 - 1.0;
    println!("Initial supply: {initial_supply}");
    println!("Final supply:   {supply}");
    println!("Actual annual inflation: {actual_annual_inflation}");
    assert!((actual_annual_inflation - annual_inflation).abs() < 0.00001);
}

fn runtime_inflations_values_are_correct_prod_or_fast(prod: bool) {
    let sessions_per_era = SessionsPerEra::prod_if(prod);
    let blocks_per_session = EpochDurationInBlocks::prod_if(prod);
    let blocks_per_era = blocks_per_session * sessions_per_era;
    let eras_per_year = (365 * DAYS) / blocks_per_era;

    let annual_inflation = 0.1; // 10%
    let collators_fraction = 0.5; // 50% of era inflation goes to collators.

    let rates = compute_inflation_rates(
        annual_inflation,
        collators_fraction,
        eras_per_year,
        blocks_per_era,
    );
    println!("{rates:?}");

    let col_inf = Perbill::from_float(rates.collators_block_inflation);
    let val_inf = Perbill::from_float(rates.validators_era_inflation);

    assert_eq!(
        CollatorsInflationRatePerBlock::prod_if(prod),
        col_inf,
        "Collators inflation didn't match"
    );
    assert_eq!(
        ValidatorsInflationRatePerEra::prod_if(prod),
        val_inf,
        "Validators inflation didn't match"
    );
}

#[test]
fn runtime_inflations_values_are_correct_in_prod() {
    runtime_inflations_values_are_correct_prod_or_fast(true)
}

#[test]
fn runtime_inflations_values_are_correct_in_fast() {
    runtime_inflations_values_are_correct_prod_or_fast(false)
}
