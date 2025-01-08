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


//! Autogenerated weights for pallet_invulnerables
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_invulnerables
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
// tmp/flashbox_weights/pallet_invulnerables.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_invulnerables using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_invulnerables::WeightInfo for SubstrateWeight<T> {
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Invulnerables::Invulnerables` (r:1 w:1)
	/// Proof: `Invulnerables::Invulnerables` (`max_values`: Some(1), `max_size`: Some(6402), added: 6897, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 199]`.
	fn add_invulnerable(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `687 + b * (34 ±0)`
		//  Estimated: `7887 + b * (34 ±0)`
		// Minimum execution time: 19_034_000 picoseconds.
		Weight::from_parts(23_710_484, 7887)
			// Standard Error: 1_277
			.saturating_add(Weight::from_parts(76_317, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 34).saturating_mul(b.into()))
	}
	/// Storage: `Invulnerables::Invulnerables` (r:1 w:1)
	/// Proof: `Invulnerables::Invulnerables` (`max_values`: Some(1), `max_size`: Some(6402), added: 6897, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 200]`.
	fn remove_invulnerable(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `71 + b * (32 ±0)`
		//  Estimated: `7887`
		// Minimum execution time: 11_397_000 picoseconds.
		Weight::from_parts(14_385_120, 7887)
			// Standard Error: 829
			.saturating_add(Weight::from_parts(53_351, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Invulnerables::Invulnerables` (r:1 w:0)
	/// Proof: `Invulnerables::Invulnerables` (`max_values`: Some(1), `max_size`: Some(6402), added: 6897, mode: `MaxEncodedLen`)
	/// Storage: `System::BlockWeight` (r:1 w:1)
	/// Proof: `System::BlockWeight` (`max_values`: Some(1), `max_size`: Some(48), added: 543, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 200]`.
	fn new_session(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `71 + r * (32 ±0)`
		//  Estimated: `7887`
		// Minimum execution time: 10_703_000 picoseconds.
		Weight::from_parts(13_945_604, 7887)
			// Standard Error: 1_365
			.saturating_add(Weight::from_parts(73_876, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Invulnerables::Invulnerables` (r:1 w:0)
	/// Proof: `Invulnerables::Invulnerables` (`max_values`: Some(1), `max_size`: Some(6402), added: 6897, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 200]`.
	fn reward_invulnerable(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `152 + b * (34 ±0)`
		//  Estimated: `7887`
		// Minimum execution time: 23_560_000 picoseconds.
		Weight::from_parts(26_965_162, 7887)
			// Standard Error: 831
			.saturating_add(Weight::from_parts(59_254, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}