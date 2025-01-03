use polkadot_sdk::{
	frame_support::{
		parameter_types,
		traits::{ConstU32, Contains},
	},
	pallet_utility,
};

use crate::{RuntimeCall, UnifiedAccounts};
use node_primitives::precompiles::DispatchFilterValidate;
use pallet_evm_precompile_assets_erc20::Erc20AssetsPrecompileSet;
use pallet_evm_precompile_blake2::Blake2F;
use pallet_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use pallet_evm_precompile_dispatch::Dispatch;
use pallet_evm_precompile_dispatch_lockdrop::DispatchLockdrop;
use pallet_evm_precompile_ed25519::Ed25519Verify;
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use pallet_evm_precompile_sr25519::Sr25519Precompile;
use pallet_evm_precompile_substrate_ecdsa::SubstrateEcdsaPrecompile;
use pallet_evm_precompile_unified_accounts::UnifiedAccountsPrecompile;
use precompile_utils::precompile_set::*;
use sp_std::fmt::Debug;

/// The asset precompile address prefix. Addresses that match against this prefix will be routed to
/// Erc20AssetsPrecompileSet.

pub const ASSET_PRECOMPILE_ADDRESS_PREFIX: &[u8] = &[255u8; 4];
parameter_types! {
	pub AssetPrefix: &'static [u8] = ASSET_PRECOMPILE_ADDRESS_PREFIX;
}

/// Precompile checks for ethereum spec precompiles
/// We allow DELEGATECALL to stay compliant with Ethereum behavior.
type EthereumPrecompilesChecks = (AcceptDelegateCall, CallableByContract, CallableByPrecompile);

/// Filter that only allows whitelisted runtime all to pass through dispatch precompile
pub struct WhitelistedCalls;

impl Contains<RuntimeCall> for WhitelistedCalls {
	fn contains(t: &RuntimeCall) -> bool {
		match t {
			RuntimeCall::Utility(pallet_utility::Call::batch { calls }) |
			RuntimeCall::Utility(pallet_utility::Call::batch_all { calls }) =>
				calls.iter().all(|call| WhitelistedCalls::contains(call)),
			_ => false,
		}
	}
}

/// Filter that only allows whitelisted runtime call to pass through dispatch-lockdrop precompile
pub struct WhitelistedLockdropCalls;

impl Contains<RuntimeCall> for WhitelistedLockdropCalls {
	fn contains(t: &RuntimeCall) -> bool {
		match t {
			RuntimeCall::Utility(pallet_utility::Call::batch { calls }) |
			RuntimeCall::Utility(pallet_utility::Call::batch_all { calls }) =>
				calls.iter().all(|call| WhitelistedLockdropCalls::contains(call)),
			_ => false,
		}
	}
}

/// The PrecompileSet installed in the Local runtime.
#[precompile_utils::precompile_name_from_address]
pub type LocalPrecompilesSetAt<R> = (
	// Ethereum precompiles:
	// We allow DELEGATECALL to stay compliant with Ethereum behavior.
	PrecompileAt<AddressU64<1>, ECRecover, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<2>, Sha256, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<3>, Ripemd160, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<4>, Identity, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<5>, Modexp, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<6>, Bn128Add, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<7>, Bn128Mul, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<8>, Bn128Pairing, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<9>, Blake2F, EthereumPrecompilesChecks>,
	// Non-Local specific nor Ethereum precompiles :
	PrecompileAt<AddressU64<1024>, Sha3FIPS256, (CallableByContract, CallableByPrecompile)>,
	PrecompileAt<
		AddressU64<1025>,
		Dispatch<R, DispatchFilterValidate<RuntimeCall, WhitelistedCalls>>,
		// Not callable from smart contract nor precompiles, only EOA accounts
		(),
	>,
	PrecompileAt<AddressU64<1026>, ECRecoverPublicKey, (CallableByContract, CallableByPrecompile)>,
	PrecompileAt<AddressU64<1027>, Ed25519Verify, (CallableByContract, CallableByPrecompile)>,
	// // Local specific precompiles:
	// PrecompileAt<
	// 	AddressU64<20481>,
	// 	DappStakingV3Precompile<R>,
	// 	(CallableByContract, CallableByPrecompile),
	// >,
	PrecompileAt<
		AddressU64<20482>,
		Sr25519Precompile<R>,
		(CallableByContract, CallableByPrecompile),
	>,
	PrecompileAt<
		AddressU64<20483>,
		SubstrateEcdsaPrecompile<R>,
		(CallableByContract, CallableByPrecompile),
	>,
	// skip 20484 for xcm precompile
	// Skipping 20485 to make sure all network have consistent precompiles address
	PrecompileAt<
		AddressU64<20486>,
		UnifiedAccountsPrecompile<R, UnifiedAccounts>,
		(CallableByContract, CallableByPrecompile),
	>,
	PrecompileAt<
		AddressU64<20487>,
		DispatchLockdrop<
			R,
			DispatchFilterValidate<RuntimeCall, WhitelistedLockdropCalls>,
			ConstU32<8>,
		>,
		// Not callable from smart contract nor precompiled, only EOA accounts
		(),
	>,
);

pub type LocalPrecompiles<R> = PrecompileSetBuilder<
	R,
	(
		// Skip precompiles if out of range.
		PrecompilesInRangeInclusive<
			// We take range as last precompile index, UPDATE this once new precompile is added
			(AddressU64<1>, AddressU64<20487>),
			LocalPrecompilesSetAt<R>,
		>,
	),
>;
