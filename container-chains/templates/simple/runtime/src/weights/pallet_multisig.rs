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
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-04-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `girazoki-XPS-15-9530`, CPU: `13th Gen Intel(R) Core(TM) i9-13900H`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/container-chain-template-frontier-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_multisig
// --extrinsic
// *
// --chain=dev
// --steps
// 50
// --repeat
// 20
// --template=benchmarking/frame-weight-runtime-template.hbs
// --json-file
// raw.json
// --output
// container-chains/templates/simple/runtime/src/weights/pallet_multisig.rs

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
		// Minimum execution time: 14_900_000 picoseconds.
		Weight::from_parts(17_583_580, 3997)
			// Standard Error: 25
			.saturating_add(Weight::from_parts(340, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(2122), added: 4597, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `213`
		//  Estimated: `5587`
		// Minimum execution time: 35_199_000 picoseconds.
		Weight::from_parts(28_904_679, 5587)
			// Standard Error: 1_774
			.saturating_add(Weight::from_parts(92_010, 0).saturating_mul(s.into()))
			// Standard Error: 17
			.saturating_add(Weight::from_parts(1_253, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(2122), added: 4597, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `279`
		//  Estimated: `5587`
		// Minimum execution time: 21_380_000 picoseconds.
		Weight::from_parts(15_794_715, 5587)
			// Standard Error: 1_290
			.saturating_add(Weight::from_parts(76_700, 0).saturating_mul(s.into()))
			// Standard Error: 12
			.saturating_add(Weight::from_parts(1_192, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(2122), added: 4597, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `MaintenanceMode::MaintenanceMode` (r:1 w:0)
	/// Proof: `MaintenanceMode::MaintenanceMode` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `390 + s * (20 ±0)`
		//  Estimated: `5587 + s * (21 ±0)`
		// Minimum execution time: 45_318_000 picoseconds.
		Weight::from_parts(31_784_278, 5587)
			// Standard Error: 4_968
			.saturating_add(Weight::from_parts(191_848, 0).saturating_mul(s.into()))
			// Standard Error: 48
			.saturating_add(Weight::from_parts(1_640, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 21).saturating_mul(s.into()))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(2122), added: 4597, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `215`
		//  Estimated: `5587`
		// Minimum execution time: 24_840_000 picoseconds.
		Weight::from_parts(29_443_723, 5587)
			// Standard Error: 11_262
			.saturating_add(Weight::from_parts(99_642, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(2122), added: 4597, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `279`
		//  Estimated: `5587`
		// Minimum execution time: 13_003_000 picoseconds.
		Weight::from_parts(14_362_084, 5587)
			// Standard Error: 2_500
			.saturating_add(Weight::from_parts(81_811, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(2122), added: 4597, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `385`
		//  Estimated: `5587`
		// Minimum execution time: 26_651_000 picoseconds.
		Weight::from_parts(28_380_145, 5587)
			// Standard Error: 1_927
			.saturating_add(Weight::from_parts(103_057, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}