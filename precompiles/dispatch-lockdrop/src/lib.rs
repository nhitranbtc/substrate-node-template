#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use codec::DecodeLimit;
use core::marker::PhantomData;
use fp_evm::PrecompileHandle;
use frame_support::{
	dispatch::{GetDispatchInfo, PostDispatchInfo},
	pallet_prelude::IsType,
	traits::{ConstU32, Get},
	weights::Weight,
};
use frame_system::Config;
use pallet_evm::GasWeightMapping;
use pallet_evm_precompile_dispatch::DispatchValidateT;
use precompile_utils::{
	prelude::{revert, BoundedBytes, RuntimeHelper},
	EvmResult,
};
use sp_core::{crypto::AccountId32, H160, H256};
use sp_io::hashing::keccak_256;
use sp_runtime::traits::Dispatchable;
use sp_std::vec::Vec;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

pub const LOG_TARGET: &str = "precompile::dispatch-lockdrop";

// ECDSA PublicKey
type ECDSAPublic = ConstU32<64>;

// `Decodelimit` specifies the max depth a call can use when decoding, as unbounded depth
// can be used to overflow the stack.
// Default value is 8, which is the same  as in XCM call decoding.

pub struct DispatchLockdrop<Runtime, DispatchValidator, DecodeLimit = ConstU32<8>>(
	PhantomData<(Runtime, DispatchValidator, DecodeLimit)>,
);

type CallLengthLimit = ConstU32<2048>;

#[precompile_utils::precompile]
impl<Runtime, DispatchValidator, DecodeLimit>
	DispatchLockdrop<Runtime, DispatchValidator, DecodeLimit>
where
	Runtime: pallet_evm::Config,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<Runtime as Config>::AccountId: IsType<AccountId32>,
	<Runtime as Config>::AccountId: From<[u8; 32]>,
	DispatchValidator:
		DispatchValidateT<<Runtime as Config>::AccountId, <Runtime as Config>::RuntimeCall>,
	DecodeLimit: Get<u32>,
{
	#[precompile::public("dispatch_lockdrop_call(bytes, bytes)")]
	fn dispatch_lockdrop_call(
		handle: &mut impl PrecompileHandle,
		call: BoundedBytes<CallLengthLimit>,
		pubkey: BoundedBytes<ECDSAPublic>,
	) -> EvmResult<bool> {
		log::trace!(
			target: LOG_TARGET,
			"raw arguments: call {:?}, pubkey: {:?}",
			call,
			pubkey
		);

		let caller: H160 = handle.context().caller.into();
		let input: Vec<u8> = call.into();

		// Record a fixed amount of weight to ensure there is no free execution
		handle.record_cost(Runtime::GasWeightMapping::weight_to_gas(Weight::from_parts(
			1_000_000_000u64,
			0,
		)))?;

		// Ensure that the caller matches the public key
		if caller != Self::get_evm_address_from_pubkey(pubkey.as_bytes()) {
			let message: &str = "caller does not match the public key";
			log::trace!(target: LOG_TARGET, "{}", message);
			return Err(revert(message));
		}

		// Derive the account id from the public key
		let origin = Self::get_account_id_from_pubkey(pubkey.as_bytes())
			.ok_or(revert("could not derive AccountId from pubkey"))?;

		// Decode the call
		let call =
			Runtime::RuntimeCall::decode_all_with_depth_limit(DecodeLimit::get(), &mut &*input)
				.map_err(|_| revert("could not decode call"))?;

		// Validate the call - ensure that the call is allowed in filter
		DispatchValidator::validate_before_dispatch(&origin, &call)
			.map_or_else(|| Ok(()), |_| Err(revert("invalid Call")))?;

		// Dispatch the call and handle the cost
		RuntimeHelper::<Runtime>::try_dispatch::<Runtime::RuntimeCall>(
			handle,
			Some(origin).into(),
			call,
			0,
		)?;

		Ok(true)
	}

	fn get_account_id_from_pubkey(pubkey: &[u8]) -> Option<<Runtime as Config>::AccountId> {
		libsecp256k1::PublicKey::parse_slice(pubkey, None)
			.map(|k| sp_io::hashing::blake2_256(k.serialize_compressed().as_ref()).into())
			.ok()
	}

	fn get_evm_address_from_pubkey(pubkey: &[u8]) -> H160 {
		H160::from(H256::from_slice(&keccak_256(pubkey)))
	}
}
