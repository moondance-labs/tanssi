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


//! Autogenerated weights for frame_system_extensions
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 47.0.0
//! DATE: 2025-07-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-1`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("flashbox_dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// frame_system_extensions
// --extrinsic
// *
// --chain=flashbox_dev
// --steps
// 50
// --repeat
// 20
// --template=benchmarking/frame-weight-runtime-template.hbs
// --json-file
// raw.json
// --output
// tmp/flashbox_weights/frame_system_extensions.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for frame_system_extensions using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::ExtensionsWeightInfo for SubstrateWeight<T> {
	fn check_genesis() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `54`
		//  Estimated: `0`
		// Minimum execution time: 6_268_000 picoseconds.
		Weight::from_parts(6_548_000, 0)
	}
	fn check_mortality_mortal_transaction() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `92`
		//  Estimated: `0`
		// Minimum execution time: 10_502_000 picoseconds.
		Weight::from_parts(10_831_000, 0)
	}
	fn check_mortality_immortal_transaction() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `92`
		//  Estimated: `0`
		// Minimum execution time: 10_480_000 picoseconds.
		Weight::from_parts(10_783_000, 0)
	}
	fn check_non_zero_sender() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 710_000 picoseconds.
		Weight::from_parts(758_000, 0)
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn check_nonce() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `3593`
		// Minimum execution time: 9_170_000 picoseconds.
		Weight::from_parts(9_402_000, 3593)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn check_spec_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 592_000 picoseconds.
		Weight::from_parts(629_000, 0)
	}
	fn check_tx_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 561_000 picoseconds.
		Weight::from_parts(630_000, 0)
	}
	/// Storage: `System::AllExtrinsicsLen` (r:1 w:1)
	/// Proof: `System::AllExtrinsicsLen` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::BlockWeight` (r:1 w:1)
	/// Proof: `System::BlockWeight` (`max_values`: Some(1), `max_size`: Some(48), added: 543, mode: `MaxEncodedLen`)
	/// Storage: `System::ExtrinsicWeightReclaimed` (r:1 w:1)
	/// Proof: `System::ExtrinsicWeightReclaimed` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn check_weight() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `24`
		//  Estimated: `1533`
		// Minimum execution time: 9_327_000 picoseconds.
		Weight::from_parts(9_559_000, 1533)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `System::ExtrinsicWeightReclaimed` (r:1 w:1)
	/// Proof: `System::ExtrinsicWeightReclaimed` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `System::BlockWeight` (r:1 w:1)
	/// Proof: `System::BlockWeight` (`max_values`: Some(1), `max_size`: Some(48), added: 543, mode: `MaxEncodedLen`)
	fn weight_reclaim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `1533`
		// Minimum execution time: 3_988_000 picoseconds.
		Weight::from_parts(4_231_000, 1533)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}