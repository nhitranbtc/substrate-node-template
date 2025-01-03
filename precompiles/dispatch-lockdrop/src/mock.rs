use super::*;
use std::marker::PhantomData;

use fp_evm::{IsPrecompileResult, Precompile};
use frame_support::{construct_runtime, parameter_types, traits::ConstU64, weights::Weight};
pub use pallet_evm::{
	AddressMapping, EnsureAddressNever, EnsureAddressRoot, PrecompileResult, PrecompileSet,
};
use sp_core::{keccak_256, H160, H256};
use sp_runtime::{
	traits::{BlakeTwo256, ConstU32, IdentityLookup},
	AccountId32,
};

use frame_support::traits::Contains;

use node_primitives::precompiles::DispatchFilterValidate;

pub type AccountId = AccountId32;
pub type Balance = u128;
pub type Block = frame_system::mocking::MockBlock<TestRuntime>;
pub const PRECOMPILE_ADDRESS: H160 = H160::repeat_byte(0x7B);

pub const ONE: u128 = 1_000_000_000_000_000_000;
pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const DUMMY: AccountId32 = AccountId32::new([2u8; 32]);

pub fn alice_secret() -> libsecp256k1::SecretKey {
	libsecp256k1::SecretKey::parse(&keccak_256(b"Alice")).unwrap()
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
	frame_system::limits::BlockWeights::simple_max(Weight::from_parts(1024, 0));
}

impl frame_system::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
	type BaseCallFilter = ();
	type BlockWeights = BlockWeights;
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type RuntimeTask = ();
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<AccountId>;
	type Block = Block;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
	type SingleBlockMigrations = ();
	type MultiBlockMigrator = ();
	type PreInherents = ();
	type PostInherents = ();
	type PostTransactions = ();
}

pub struct WhitelistedCalls;

impl Contains<RuntimeCall> for WhitelistedCalls {
	fn contains(call: &RuntimeCall) -> bool {
		match call {
			RuntimeCall::Balances(pallet_balances::Call::transfer_keep_alive { .. }) => true,
			RuntimeCall::System(frame_system::Call::remark { .. }) => true,
			RuntimeCall::Utility(_) => true,
			_ => false,
		}
	}
}

// Configure a mock runtime to test the pallet.
construct_runtime!(
	pub enum TestRuntime {
		System: frame_system,
		Evm: pallet_evm,
		Balances: pallet_balances,
		Timestamp: pallet_timestamp,
		Utility: pallet_utility,
	}
);
