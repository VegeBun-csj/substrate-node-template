
//! Autogenerated weights for pallet_template
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-10-06, STEPS: `[20, ]`, REPEAT: 500, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/node-template
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_template
// --extrinsic
// do_something
// --steps
// 20
// --repeat
// 500
// --output=./pallets/template/src/weights.rs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_template.
// pub struct WeightInfo<T>(PhantomData<T>);
// impl<T: frame_system::Config> pallet_template::WeightInfo for WeightInfo<T> {
// 	fn do_something(_s: u32, ) -> Weight {
// 		(18_252_000 as Weight)
// 			.saturating_add(T::DbWeight::get().writes(1 as Weight))
// 	}
// }
/// Weight functions needed for pallet_benchmark_demo.
pub trait WeightInfo {
	fn do_something(b: u32, ) -> Weight;
}

/// Weights for pallet_benchmark_demo using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn do_something(b: u32, ) -> Weight {
		(21_261_000 as Weight)
			// Standard Error: 0
			.saturating_add((8_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn do_something(b: u32, ) -> Weight {
		(21_261_000 as Weight)
			// Standard Error: 0
			.saturating_add((8_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}