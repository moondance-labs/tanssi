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
//! DATE: 2025-05-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-1`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("starlight-dev"), DB CACHE: 1024

// Executed Command:
// target/release/tanssi-relay
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_invulnerables
// --extrinsic
// *
// --chain=starlight-dev
// --steps
// 50
// --repeat
// 20
// --template=benchmarking/frame-weight-runtime-template.hbs
// --json-file
// raw.json
// --output
// tmp/starlight_weights/pallet_invulnerables.rs

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
	/// Storage: `TanssiInvulnerables::Invulnerables` (r:1 w:1)
	/// Proof: `TanssiInvulnerables::Invulnerables` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 99]`.
	fn add_invulnerable(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `845 + b * (36 ±0)`
		//  Estimated: `4687 + b * (37 ±0)`
		// Minimum execution time: 28_800_000 picoseconds.
		Weight::from_parts(34_561_523, 4687)
			// Standard Error: 1_747
			.saturating_add(Weight::from_parts(104_717, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 37).saturating_mul(b.into()))
	}
	/// Storage: `TanssiInvulnerables::Invulnerables` (r:1 w:1)
	/// Proof: `TanssiInvulnerables::Invulnerables` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 100]`.
	fn remove_invulnerable(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `137 + b * (32 ±0)`
		//  Estimated: `4687`
		// Minimum execution time: 17_604_000 picoseconds.
		Weight::from_parts(21_385_221, 4687)
			// Standard Error: 1_210
			.saturating_add(Weight::from_parts(73_054, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `TanssiInvulnerables::Invulnerables` (r:1 w:0)
	/// Proof: `TanssiInvulnerables::Invulnerables` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 100]`.
	fn new_session(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `137 + r * (32 ±0)`
		//  Estimated: `4687`
		// Minimum execution time: 15_182_000 picoseconds.
		Weight::from_parts(19_232_231, 4687)
			// Standard Error: 1_236
			.saturating_add(Weight::from_parts(62_902, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `TanssiInvulnerables::Invulnerables` (r:1 w:0)
	/// Proof: `TanssiInvulnerables::Invulnerables` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 100]`.
	fn reward_invulnerable(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285 + b * (33 ±0)`
		//  Estimated: `4687`
		// Minimum execution time: 30_744_000 picoseconds.
		Weight::from_parts(34_946_648, 4687)
			// Standard Error: 1_455
			.saturating_add(Weight::from_parts(83_470, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}