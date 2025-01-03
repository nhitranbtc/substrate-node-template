use polkadot_sdk::{frame_support, sp_core};

use core::str::from_utf8;
use frame_support::dispatch::GetDispatchInfo;
use frame_support::traits::Currency;
use libsecp256k1::PublicKeyFormat;
use sp_core::crypto::{AccountId32, Ss58Codec};

//use crate::mock::*;

use codec::Encode;
use hex_literal::hex;
use node_primitives::evm::EvmAddress;
use precompile_utils::testing::*;
use sp_core::{ecdsa, Pair};

// fn precompiles() -> TestPrecompileSet<TestRuntime> {
// 	PrecompilesValue::get()
// }
