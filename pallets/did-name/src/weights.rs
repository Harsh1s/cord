// This file is part of CORD – https://cord.network

// Copyright (C) Dhiway Networks Pvt. Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// CORD is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// CORD is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with CORD. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_did_name`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-05-17, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `cord-benchmark-16gb`, CPU: `AMD EPYC 7B13`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/production/cord
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_did_name
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/did-name/src/weights.rs
// --header=./HEADER-GPL3
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_did_name`.
pub trait WeightInfo {
	fn register(n: u32, ) -> Weight;
	fn release() -> Weight;
	fn ban(n: u32, ) -> Weight;
	fn unban(n: u32, ) -> Weight;
}

/// Weights for `pallet_did_name` using the CORD node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `DidName::Names` (r:1 w:1)
	/// Proof: `DidName::Names` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `DidName::Owner` (r:1 w:1)
	/// Proof: `DidName::Owner` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `DidName::Banned` (r:1 w:0)
	/// Proof: `DidName::Banned` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[13, 64]`.
	fn register(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `3583`
		// Minimum execution time: 14_410_000 picoseconds.
		Weight::from_parts(15_002_569, 3583)
			// Standard Error: 742
			.saturating_add(Weight::from_parts(13_594, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `DidName::Names` (r:1 w:1)
	/// Proof: `DidName::Names` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `DidName::Owner` (r:1 w:1)
	/// Proof: `DidName::Owner` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	fn release() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `275`
		//  Estimated: `3583`
		// Minimum execution time: 16_571_000 picoseconds.
		Weight::from_parts(17_151_000, 3583)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `DidName::Banned` (r:1 w:1)
	/// Proof: `DidName::Banned` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
	/// Storage: `DidName::Owner` (r:1 w:1)
	/// Proof: `DidName::Owner` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `DidName::Names` (r:0 w:1)
	/// Proof: `DidName::Names` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[13, 64]`.
	fn ban(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `113 + n * (1 ±0)`
		//  Estimated: `3583`
		// Minimum execution time: 15_620_000 picoseconds.
		Weight::from_parts(17_356_253, 3583)
			// Standard Error: 1_146
			.saturating_add(Weight::from_parts(29_211, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `DidName::Banned` (r:1 w:1)
	/// Proof: `DidName::Banned` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[13, 64]`.
	fn unban(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `41 + n * (1 ±0)`
		//  Estimated: `3547`
		// Minimum execution time: 10_480_000 picoseconds.
		Weight::from_parts(11_171_839, 3547)
			// Standard Error: 1_086
			.saturating_add(Weight::from_parts(13_221, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `DidName::Names` (r:1 w:1)
	/// Proof: `DidName::Names` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `DidName::Owner` (r:1 w:1)
	/// Proof: `DidName::Owner` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `DidName::Banned` (r:1 w:0)
	/// Proof: `DidName::Banned` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[13, 64]`.
	fn register(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `3583`
		// Minimum execution time: 14_410_000 picoseconds.
		Weight::from_parts(15_002_569, 3583)
			// Standard Error: 742
			.saturating_add(Weight::from_parts(13_594, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `DidName::Names` (r:1 w:1)
	/// Proof: `DidName::Names` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `DidName::Owner` (r:1 w:1)
	/// Proof: `DidName::Owner` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	fn release() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `275`
		//  Estimated: `3583`
		// Minimum execution time: 16_571_000 picoseconds.
		Weight::from_parts(17_151_000, 3583)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `DidName::Banned` (r:1 w:1)
	/// Proof: `DidName::Banned` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
	/// Storage: `DidName::Owner` (r:1 w:1)
	/// Proof: `DidName::Owner` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `DidName::Names` (r:0 w:1)
	/// Proof: `DidName::Names` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[13, 64]`.
	fn ban(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `113 + n * (1 ±0)`
		//  Estimated: `3583`
		// Minimum execution time: 15_620_000 picoseconds.
		Weight::from_parts(17_356_253, 3583)
			// Standard Error: 1_146
			.saturating_add(Weight::from_parts(29_211, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `DidName::Banned` (r:1 w:1)
	/// Proof: `DidName::Banned` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[13, 64]`.
	fn unban(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `41 + n * (1 ±0)`
		//  Estimated: `3547`
		// Minimum execution time: 10_480_000 picoseconds.
		Weight::from_parts(11_171_839, 3547)
			// Standard Error: 1_086
			.saturating_add(Weight::from_parts(13_221, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
