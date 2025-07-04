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

#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;

pub const TANSSI_GENESIS_HASH: [u8; 32] =
    hex_literal::hex!["dd6d086f75ec041b66e20c4186d327b23c8af244c534a2418de6574e8c041a60"];

/// Money matters.
pub mod currency {
    use primitives::Balance;

    /// The existential deposit.
    pub const EXISTENTIAL_DEPOSIT: Balance = 1 * CENTS;

    // Provide a common factor between runtimes based on a supply of 10_000_000 tokens.
    pub const SUPPLY_FACTOR: Balance = 1;

    pub const UNITS: Balance = 1_000_000_000_000;
    pub const CENTS: Balance = UNITS / 30_000;
    pub const GRAND: Balance = CENTS * 100_000;
    pub const MILLICENTS: Balance = CENTS / 1_000;
    pub const MICROUNITS: Balance = 1_000_000;
    pub const MILLIUNITS: Balance = 1_000_000_000;

    pub const STORAGE_BYTE_FEE: Balance = 100 * MICROUNITS * SUPPLY_FACTOR;
    pub const STORAGE_ITEM_FEE: Balance = 100 * MILLIUNITS * SUPPLY_FACTOR;

    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * STORAGE_ITEM_FEE + (bytes as Balance) * STORAGE_BYTE_FEE
    }
}

/// Time and blocks.
pub mod time {
    use primitives::{BlockNumber, Moment};
    pub const MILLISECS_PER_BLOCK: Moment = 6000;
    pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

    tp_traits::prod_or_fast_parameter_types! {
        pub const EpochDurationInBlocks: BlockNumber = { prod: 6 * HOURS, fast: 1 * MINUTES };
    }

    // These time units are defined in number of blocks.
    pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
    pub const HOURS: BlockNumber = MINUTES * 60;
    pub const DAYS: BlockNumber = HOURS * 24;
    pub const WEEKS: BlockNumber = DAYS * 7;

    // 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
    // The choice of is done in accordance to the slot duration and expected target
    // block time, for safely resisting network delays of maximum two seconds.
    // <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
    pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
}

pub mod snowbridge {
    use {
        frame_support::parameter_types,
        xcm::prelude::{Location, NetworkId},
    };

    parameter_types! {
            /// Network and location for the Ethereum chain. On Starlight, the Ethereum chain bridged
            /// to is the Ethereum mainnet, with chain ID 1.
            /// <https://chainlist.org/chain/1>
            /// <https://ethereum.org/en/developers/docs/apis/json-rpc/#net_version>
            pub EthereumNetwork: NetworkId = NetworkId::Ethereum { chain_id: 1 };
            pub EthereumLocation: Location = Location::new(1, EthereumNetwork::get());

    }

    #[cfg(feature = "runtime-benchmarks")]
    parameter_types! {
            // We need a different ethereum location for benchmarks as the ethereum system pallet
            // is written for benchmarks from para
            pub EthereumLocationForParaIdBenchmarks: Location = Location::new(2, EthereumNetwork::get());

    }
}

/// Fee-related.
pub mod fee {
    pub use sp_runtime::Perbill;
    use {
        crate::weights::ExtrinsicBaseWeight,
        frame_support::weights::{
            WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
        },
        primitives::Balance,
        smallvec::smallvec,
    };

    /// The block saturation level. Fees will be updates based on this value.
    pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);

    /// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
    /// node's balance type.
    ///
    /// This should typically create a mapping between the following ranges:
    ///   - [0, `frame_system::MaximumBlockWeight`]
    ///   - [Balance::min, Balance::max]
    ///
    /// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
    ///   - Setting it to `0` will essentially disable the weight fee.
    ///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
    pub struct WeightToFee;
    impl WeightToFeePolynomial for WeightToFee {
        type Balance = Balance;
        fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
            // in Starlight, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
            let p = super::currency::CENTS;
            let q = 10 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
            smallvec![WeightToFeeCoefficient {
                degree: 1,
                negative: false,
                coeff_frac: Perbill::from_rational(p % q, q),
                coeff_integer: p / q,
            }]
        }
    }
}

/// System Parachains.
pub mod system_parachain {
    use {primitives::Id, xcm_builder::IsChildSystemParachain};

    /// Network's Asset Hub parachain ID.
    pub const ASSET_HUB_ID: u32 = 1000;
    /// Contracts parachain ID.
    pub const CONTRACTS_ID: u32 = 1002;
    /// Encointer parachain ID.
    pub const ENCOINTER_ID: u32 = 1003;
    /// People parachain ID.
    pub const PEOPLE_ID: u32 = 1004;
    /// BridgeHub parachain ID.
    pub const BRIDGE_HUB_ID: u32 = 1013;
    /// Brokerage parachain ID.
    pub const BROKER_ID: u32 = 1005;

    /// All system parachains of Starlight.
    pub type SystemParachains = IsChildSystemParachain<Id>;
}

/// Starlight Treasury pallet instance.
pub const TREASURY_PALLET_ID: u8 = 40;

#[cfg(test)]
mod tests {
    use {
        super::{
            currency::{CENTS, MILLICENTS},
            fee::WeightToFee,
        },
        crate::weights::ExtrinsicBaseWeight,
        frame_support::weights::WeightToFee as WeightToFeeT,
        runtime_common::MAXIMUM_BLOCK_WEIGHT,
    };

    #[test]
    // Test that the fee for `MAXIMUM_BLOCK_WEIGHT` of weight has sane bounds.
    fn full_block_fee_is_correct() {
        // A full block should cost between 1,000 and 10,000 CENTS.
        let full_block = WeightToFee::weight_to_fee(&MAXIMUM_BLOCK_WEIGHT);
        assert!(full_block >= 1_000 * CENTS);
        assert!(full_block <= 10_000 * CENTS);
    }

    #[test]
    // This function tests that the fee for `ExtrinsicBaseWeight` of weight is correct
    fn extrinsic_base_fee_is_correct() {
        // `ExtrinsicBaseWeight` should cost 1/10 of a CENT
        println!("Base: {}", ExtrinsicBaseWeight::get());
        let x = WeightToFee::weight_to_fee(&ExtrinsicBaseWeight::get());
        let y = CENTS / 10;
        assert!(x.max(y) - x.min(y) < MILLICENTS);
    }
}
