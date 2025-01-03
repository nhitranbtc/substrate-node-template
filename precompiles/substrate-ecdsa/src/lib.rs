#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::PrecompileHandle;
use sp_core::{ecdsa, ByteArray, ConstU32};
use sp_std::{marker::PhantomData, prelude::*};

use precompile_utils::prelude::*;

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

// ECDSA pub key bytes
type ECDSAPubKeyBytes = ConstU32<33>;
// ECDSA signature bytes
type ECDSASignatureBytes = ConstU32<65>;

/// A precompile to wrap substrate ecdsa functions.
pub struct SubstrateEcdsaPrecompile<Runtime>(PhantomData<Runtime>);

#[precompile_utils::precompile]
impl<Runtime: pallet_evm::Config> SubstrateEcdsaPrecompile<Runtime> {
	#[precompile::public("verify(bytes, bytes, bytes)")]
	#[precompile::view]
	fn verify(
		_handle: &mut impl PrecompileHandle,
		public_bytes: BoundedBytes<ECDSAPubKeyBytes>,
		signature_bytes: BoundedBytes<ECDSASignatureBytes>,
		message: UnboundedBytes,
	) -> EvmResult<bool> {
		// Parse arguments
		let public_bytes: Vec<u8> = public_bytes.into();
		let signature_bytes: Vec<u8> = signature_bytes.into();
		let message: Vec<u8> = message.into();

		// Parse public key
		let public = if let Ok(public) = ecdsa::Public::try_from(&public_bytes[..]) {
			public
		} else {
			// Return `false` if public key length is wrong
			return Ok(false);
		};

		// Parse signature
		let signature_opt = ecdsa::Signature::from_slice(&signature_bytes[..]);

		let signature = if let Ok(sig) = signature_opt {
			sig
		} else {
			// Return `false` if signature length is wrong
			return Ok(false);
		};

		log::trace!(
			target: "substrate-ecdsa-precompile",
			"Verify signature {:?} for public {:?} and message {:?}",
			signature, public, message,
		);

		let is_confirmed = sp_io::crypto::ecdsa_verify(&signature, &message[..], &public);

		log::trace!(
			target: "substrate-ecdsa-precompile",
			"Verified signature {:?} is {:?}",
			signature, is_confirmed,
		);

		Ok(is_confirmed)
	}
}