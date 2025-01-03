#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_unified_accounts.
pub trait WeightInfo {
    fn claim_evm_address() -> Weight;
    fn claim_default_evm_address() -> Weight;
    fn to_account_id() -> Weight;
    fn to_account_id_or_default() -> Weight;
    fn to_h160() -> Weight;
    fn to_h160_or_default() -> Weight;
}

/// Weights for pallet_unified_accounts using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:1)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:1)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: EVMChainId ChainId (r:1 w:0)
	/// Proof: EVMChainId ChainId (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System BlockHash (r:1 w:0)
	/// Proof: System BlockHash (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_evm_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `256`
		//  Estimated: `3593`
		// Minimum execution time: 91_231_000 picoseconds.
		Weight::from_parts(91_688_000, 3593)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:1)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:1)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn claim_default_evm_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3533`
		// Minimum execution time: 40_749_000 picoseconds.
		Weight::from_parts(41_411_000, 3533)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:0)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn to_account_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 5_478_000 picoseconds.
		Weight::from_parts(5_661_000, 3533)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:0)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn to_account_id_or_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 5_515_000 picoseconds.
		Weight::from_parts(5_700_000, 3533)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:0)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn to_h160() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 5_626_000 picoseconds.
		Weight::from_parts(5_827_000, 3533)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:0)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn to_h160_or_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 5_578_000 picoseconds.
		Weight::from_parts(5_719_000, 3533)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:1)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:1)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: EVMChainId ChainId (r:1 w:0)
	/// Proof: EVMChainId ChainId (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System BlockHash (r:1 w:0)
	/// Proof: System BlockHash (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_evm_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `256`
		//  Estimated: `3593`
		// Minimum execution time: 91_231_000 picoseconds.
		Weight::from_parts(91_688_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:1)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:1)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn claim_default_evm_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3533`
		// Minimum execution time: 40_749_000 picoseconds.
		Weight::from_parts(41_411_000, 3533)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:0)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn to_account_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 5_478_000 picoseconds.
		Weight::from_parts(5_661_000, 3533)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:0)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn to_account_id_or_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 5_515_000 picoseconds.
		Weight::from_parts(5_700_000, 3533)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:0)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn to_h160() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 5_626_000 picoseconds.
		Weight::from_parts(5_827_000, 3533)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:0)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn to_h160_or_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 5_578_000 picoseconds.
		Weight::from_parts(5_719_000, 3533)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
}
