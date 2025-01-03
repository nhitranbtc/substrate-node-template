#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::PrecompileHandle;
use sp_core::{crypto::UncheckedFrom, sr25519, ByteArray, ConstU32, H256};
use sp_std::marker::PhantomData;

use precompile_utils::prelude::*;

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

// SR2519 signature bytes
type SR25519SignatureBytes = ConstU32<64>;

/// A precompile to wrap substrate sr25519 functions.
pub struct Sr25519Precompile<Runtime>(PhantomData<Runtime>);

#[precompile_utils::precompile]
impl<Runtime: pallet_evm::Config> Sr25519Precompile<Runtime> {
	#[precompile::public("verify(bytes32, bytes, bytes)")]
	#[precompile::view]
	fn verify(
		_: &mut impl PrecompileHandle,
		public: H256,
		signature: BoundedBytes<SR25519SignatureBytes>,
		message: UnboundedBytes,
	) -> EvmResult<bool> {
		// Parse pub key
		let public = sr25519::Public::unchecked_from(public);
		// Parse signature
		let signature = if let Ok(sig) = sr25519::Signature::from_slice(&signature.as_bytes()) {
			sig
		} else {
			// Return `false` if signature length is wrong
			return Ok(false);
		};

		log::trace!(
			target: "sr25519-precompile",
			"Verify signature {:?} for public {:?} and message {:?}",
			signature, public, message,
		);

		let is_comfirmed =
			sp_io::crypto::sr25519_verify(&signature, &message.as_bytes(), &public.into());

		log::trace!(
			target: "sr25519-precompile",
			"Verified signature {:?} es {:?}",
			signature, is_comfirmed,
		);

		Ok(is_comfirmed)
	}
}
