
//! Autogenerated weights for `pallet_esg`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-08-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `antkaraghatr588`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("qa-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/firechain-node
// benchmark
// pallet
// --chain
// qa-dev
// --pallet
// pallet_esg
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ./frame/esg/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;
use frame_support::weights::constants::RocksDbWeight;
pub trait WeightInfo {
	fn register_an_oracle() -> Weight;
	fn deregister_an_oracle() -> Weight;
	fn upsert_esg_scores() -> Weight;
}

/// Weight functions for `pallet_esg`.
pub struct SubstrateWeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeightInfo<T> {
	/// Storage: `EsgScore::SudoOraclesStore` (r:1 w:1)
	/// Proof: `EsgScore::SudoOraclesStore` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EsgScore::NonSudoOraclesStore` (r:1 w:0)
	/// Proof: `EsgScore::NonSudoOraclesStore` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn register_an_oracle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2109`
		//  Estimated: `3594`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(18_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3594))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `EsgScore::SudoOraclesStore` (r:1 w:1)
	/// Proof: `EsgScore::SudoOraclesStore` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn deregister_an_oracle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `238`
		//  Estimated: `1723`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(14_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1723))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `EsgScore::SudoOraclesStore` (r:1 w:0)
	/// Proof: `EsgScore::SudoOraclesStore` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EsgScore::ESGScoresMap` (r:2 w:2)
	/// Proof: `EsgScore::ESGScoresMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn upsert_esg_scores() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `238`
		//  Estimated: `6178`
		// Minimum execution time: 1_028_000_000 picoseconds.
		Weight::from_parts(1_048_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6178))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}


impl WeightInfo for () {
	/// Storage: `EsgScore::SudoOraclesStore` (r:1 w:1)
	/// Proof: `EsgScore::SudoOraclesStore` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EsgScore::NonSudoOraclesStore` (r:1 w:0)
	/// Proof: `EsgScore::NonSudoOraclesStore` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn register_an_oracle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2109`
		//  Estimated: `3594`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(18_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3594))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: `EsgScore::SudoOraclesStore` (r:1 w:1)
	/// Proof: `EsgScore::SudoOraclesStore` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn deregister_an_oracle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `238`
		//  Estimated: `1723`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(14_000_000, 0)
			.saturating_add(Weight::from_parts(0, 1723))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	/// Storage: `EsgScore::SudoOraclesStore` (r:1 w:0)
	/// Proof: `EsgScore::SudoOraclesStore` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EsgScore::ESGScoresMap` (r:2 w:2)
	/// Proof: `EsgScore::ESGScoresMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn upsert_esg_scores() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `238`
		//  Estimated: `6178`
		// Minimum execution time: 1_028_000_000 picoseconds.
		Weight::from_parts(1_048_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6178))
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
}