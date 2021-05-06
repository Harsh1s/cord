// Copyright 2019-2021 Dhiway.
// This file is part of CORD Platform.

// derived from kilt project

//! Generation of weight files for benchmarking.

//! //! Autogenerated weights for pallet_delegation

// Executed Command:
// ./target/release/cord
// benchmark
// --chain=dev
// --execution=wasm
// --pallet=pallet_delegation
// --extrinsic=*
// --steps=20
// --output=./pallets/delegation/src/weights.rs
// --template=./.maintain/weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_delegation.
pub trait WeightInfo {
	fn create_root() -> Weight;
	fn revoke_root(r: u32) -> Weight;
	fn add_delegation() -> Weight;
	fn revoke_delegation_root_child(r: u32) -> Weight;
	fn revoke_delegation_leaf(r: u32) -> Weight;
}

/// Weights for pallet_delegation using the Substrate node and recommended
/// hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create_root() -> Weight {
		(119_251_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn revoke_root(r: u32) -> Weight {
		(151_704_000_u64)
			// Standard Error: 2_969_000
			.saturating_add((154_352_000_u64).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r as Weight)))
	}
	fn add_delegation() -> Weight {
		(330_510_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn revoke_delegation_root_child(r: u32) -> Weight {
		(51_153_000_u64)
			// Standard Error: 894_000
			.saturating_add((160_368_000_u64).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r as Weight)))
	}
	fn revoke_delegation_leaf(r: u32) -> Weight {
		(372_268_000_u64)
			// Standard Error: 58_482_000
			.saturating_add((33_806_000_u64).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_root() -> Weight {
		(119_251_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn revoke_root(r: u32) -> Weight {
		(151_704_000_u64)
			// Standard Error: 2_969_000
			.saturating_add((154_352_000_u64).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(r as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(r as Weight)))
	}
	fn add_delegation() -> Weight {
		(330_510_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	fn revoke_delegation_root_child(r: u32) -> Weight {
		(51_153_000_u64)
			// Standard Error: 894_000
			.saturating_add((160_368_000_u64).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(r as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(r as Weight)))
	}
	fn revoke_delegation_leaf(r: u32) -> Weight {
		(372_268_000_u64)
			// Standard Error: 58_482_000
			.saturating_add((33_806_000_u64).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(r as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}