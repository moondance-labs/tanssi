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


//! Autogenerated weights for pallet_assets
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 47.0.0
//! DATE: 2025-07-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-1`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_assets
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
// tmp/dancebox_weights/pallet_assets.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_assets using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_assets::WeightInfo for SubstrateWeight<T> {
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 0_000 picoseconds.
		Weight::from_parts(0, 0)
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::NextAssetId` (r:1 w:0)
	/// Proof: `ForeignAssets::NextAssetId` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	fn force_create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `3673`
		// Minimum execution time: 14_699_000 picoseconds.
		Weight::from_parts(14_897_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn start_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `274`
		//  Estimated: `3673`
		// Minimum execution time: 14_112_000 picoseconds.
		Weight::from_parts(14_472_000, 3673)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:1001 w:1000)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1000 w:1000)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 1000]`.
	fn destroy_accounts(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `212 + c * (208 ±0)`
		//  Estimated: `3673 + c * (2607 ±0)`
		// Minimum execution time: 20_271_000 picoseconds.
		Weight::from_parts(20_644_000, 3673)
			// Standard Error: 11_487
			.saturating_add(Weight::from_parts(16_314_311, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2607).saturating_mul(c.into()))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Approvals` (r:1001 w:1000)
	/// Proof: `ForeignAssets::Approvals` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 1000]`.
	fn destroy_approvals(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `409 + a * (86 ±0)`
		//  Estimated: `3673 + a * (2621 ±0)`
		// Minimum execution time: 20_487_000 picoseconds.
		Weight::from_parts(20_695_000, 3673)
			// Standard Error: 4_020
			.saturating_add(Weight::from_parts(6_284_307, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 2621).saturating_mul(a.into()))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Metadata` (r:1 w:0)
	/// Proof: `ForeignAssets::Metadata` (`max_values`: None, `max_size`: Some(138), added: 2613, mode: `MaxEncodedLen`)
	fn finish_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `3673`
		// Minimum execution time: 16_917_000 picoseconds.
		Weight::from_parts(17_496_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `3673`
		// Minimum execution time: 29_167_000 picoseconds.
		Weight::from_parts(29_556_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346`
		//  Estimated: `3673`
		// Minimum execution time: 38_098_000 picoseconds.
		Weight::from_parts(39_072_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:2 w:2)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `385`
		//  Estimated: `6204`
		// Minimum execution time: 53_874_000 picoseconds.
		Weight::from_parts(54_853_000, 6204)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:2 w:2)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `385`
		//  Estimated: `6204`
		// Minimum execution time: 48_500_000 picoseconds.
		Weight::from_parts(49_307_000, 6204)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:2 w:2)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `385`
		//  Estimated: `6204`
		// Minimum execution time: 53_824_000 picoseconds.
		Weight::from_parts(55_023_000, 6204)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:0)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	fn freeze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346`
		//  Estimated: `3673`
		// Minimum execution time: 19_444_000 picoseconds.
		Weight::from_parts(20_070_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:0)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	fn thaw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346`
		//  Estimated: `3673`
		// Minimum execution time: 19_767_000 picoseconds.
		Weight::from_parts(20_093_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn freeze_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `274`
		//  Estimated: `3673`
		// Minimum execution time: 14_128_000 picoseconds.
		Weight::from_parts(14_572_000, 3673)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn thaw_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `274`
		//  Estimated: `3673`
		// Minimum execution time: 14_233_000 picoseconds.
		Weight::from_parts(14_557_000, 3673)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Metadata` (r:1 w:0)
	/// Proof: `ForeignAssets::Metadata` (`max_values`: None, `max_size`: Some(138), added: 2613, mode: `MaxEncodedLen`)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `3673`
		// Minimum execution time: 17_449_000 picoseconds.
		Weight::from_parts(17_803_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn set_team() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `3673`
		// Minimum execution time: 15_148_000 picoseconds.
		Weight::from_parts(15_537_000, 3673)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:0)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Metadata` (r:1 w:1)
	/// Proof: `ForeignAssets::Metadata` (`max_values`: None, `max_size`: Some(138), added: 2613, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn set_metadata(n: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `3673`
		// Minimum execution time: 17_411_000 picoseconds.
		Weight::from_parts(18_141_850, 3673)
			// Standard Error: 379
			.saturating_add(Weight::from_parts(1_132, 0).saturating_mul(n.into()))
			// Standard Error: 379
			.saturating_add(Weight::from_parts(1_970, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:0)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Metadata` (r:1 w:1)
	/// Proof: `ForeignAssets::Metadata` (`max_values`: None, `max_size`: Some(138), added: 2613, mode: `MaxEncodedLen`)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `402`
		//  Estimated: `3673`
		// Minimum execution time: 18_403_000 picoseconds.
		Weight::from_parts(18_898_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:0)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Metadata` (r:1 w:1)
	/// Proof: `ForeignAssets::Metadata` (`max_values`: None, `max_size`: Some(138), added: 2613, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn force_set_metadata(_n: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `79`
		//  Estimated: `3673`
		// Minimum execution time: 16_512_000 picoseconds.
		Weight::from_parts(17_173_795, 3673)
			// Standard Error: 302
			.saturating_add(Weight::from_parts(886, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:0)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Metadata` (r:1 w:1)
	/// Proof: `ForeignAssets::Metadata` (`max_values`: None, `max_size`: Some(138), added: 2613, mode: `MaxEncodedLen`)
	fn force_clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `402`
		//  Estimated: `3673`
		// Minimum execution time: 17_607_000 picoseconds.
		Weight::from_parts(18_462_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn force_asset_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `3673`
		// Minimum execution time: 14_640_000 picoseconds.
		Weight::from_parts(14_875_000, 3673)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Approvals` (r:1 w:1)
	/// Proof: `ForeignAssets::Approvals` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	fn approve_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `274`
		//  Estimated: `3673`
		// Minimum execution time: 22_734_000 picoseconds.
		Weight::from_parts(23_113_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Approvals` (r:1 w:1)
	/// Proof: `ForeignAssets::Approvals` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:2 w:2)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `553`
		//  Estimated: `6204`
		// Minimum execution time: 66_548_000 picoseconds.
		Weight::from_parts(67_396_000, 6204)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Approvals` (r:1 w:1)
	/// Proof: `ForeignAssets::Approvals` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	fn cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `442`
		//  Estimated: `3673`
		// Minimum execution time: 25_327_000 picoseconds.
		Weight::from_parts(25_994_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Approvals` (r:1 w:1)
	/// Proof: `ForeignAssets::Approvals` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	fn force_cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `442`
		//  Estimated: `3673`
		// Minimum execution time: 25_531_000 picoseconds.
		Weight::from_parts(25_966_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn set_min_balance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `3673`
		// Minimum execution time: 16_014_000 picoseconds.
		Weight::from_parts(16_591_000, 3673)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn touch() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `3673`
		// Minimum execution time: 22_342_000 picoseconds.
		Weight::from_parts(22_806_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn touch_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `3673`
		// Minimum execution time: 21_432_000 picoseconds.
		Weight::from_parts(22_011_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `364`
		//  Estimated: `3673`
		// Minimum execution time: 20_144_000 picoseconds.
		Weight::from_parts(20_561_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn refund_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `397`
		//  Estimated: `3673`
		// Minimum execution time: 19_317_000 picoseconds.
		Weight::from_parts(19_848_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:0)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	fn block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346`
		//  Estimated: `3673`
		// Minimum execution time: 19_514_000 picoseconds.
		Weight::from_parts(20_105_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:2 w:2)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `385`
		//  Estimated: `6204`
		// Minimum execution time: 65_520_000 picoseconds.
		Weight::from_parts(66_954_000, 6204)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}