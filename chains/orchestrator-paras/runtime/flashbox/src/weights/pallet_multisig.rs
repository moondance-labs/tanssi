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


//! Autogenerated weights for pallet_multisig
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-05-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_multisig
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
// tmp/flashbox_weights/pallet_multisig.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_multisig using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for SubstrateWeight<T> {
	/// Storage: `MaintenanceMode::MaintenanceMode` (r:1 w:0)
	/// Proof: `MaintenanceMode::MaintenanceMode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `46`
		//  Estimated: `3997`
		// Minimum execution time: 27_469_000 picoseconds.
		Weight::from_parts(28_643_298, 3997)
			// Standard Error: 4
			.saturating_add(Weight::from_parts(513, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `193 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 55_017_000 picoseconds.
		Weight::from_parts(42_856_542, 6811)
			// Standard Error: 595
			.saturating_add(Weight::from_parts(138_193, 0).saturating_mul(s.into()))
			// Standard Error: 5
			.saturating_add(Weight::from_parts(1_937, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211`
		//  Estimated: `6811`
		// Minimum execution time: 35_386_000 picoseconds.
		Weight::from_parts(23_996_192, 6811)
			// Standard Error: 488
			.saturating_add(Weight::from_parts(123_448, 0).saturating_mul(s.into()))
			// Standard Error: 4
			.saturating_add(Weight::from_parts(1_963, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MaintenanceMode::MaintenanceMode` (r:1 w:0)
	/// Proof: `MaintenanceMode::MaintenanceMode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `363 + s * (33 ±0)`
		//  Estimated: `6811 + s * (34 ±0)`
		// Minimum execution time: 68_714_000 picoseconds.
		Weight::from_parts(53_676_479, 6811)
			// Standard Error: 731
			.saturating_add(Weight::from_parts(172_348, 0).saturating_mul(s.into()))
			// Standard Error: 7
			.saturating_add(Weight::from_parts(2_040, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 34).saturating_mul(s.into()))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn approve_as_multi_create(s: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `193 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 39_961_000 picoseconds.
		Weight::from_parts(41_153_655, 6811)
			// Standard Error: 630
			.saturating_add(Weight::from_parts(140_181, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn approve_as_multi_approve(s: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211`
		//  Estimated: `6811`
		// Minimum execution time: 21_542_000 picoseconds.
		Weight::from_parts(22_428_365, 6811)
			// Standard Error: 482
			.saturating_add(Weight::from_parts(119_641, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn cancel_as_multi(s: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `383 + s * (1 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 39_933_000 picoseconds.
		Weight::from_parts(40_793_379, 6811)
			// Standard Error: 545
			.saturating_add(Weight::from_parts(131_972, 0).saturating_mul(s.into()))
			// Standard Error: 5
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}