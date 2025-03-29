

use crate::{AccountId, AssetId};

use frame_support::{
    ensure,
    traits::{
        fungible::{Balanced, Credit},
        tokens::{fungible::Inspect, imbalance::OnUnbalanced},
    },
};
use pallet_evm::{AddressMapping, HashedAddressMapping, OnChargeEVMTransaction};
use codec::{Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{Hasher, H160, H256, U256};
use sp_runtime::traits::UniqueSaturatedInto;
use sp_std::marker::PhantomData;

use pallet_assets::AssetsCallback;
//use pallet_evm_precompile_assets_erc20::AddressToAssetId;

pub type EvmAddress = H160;

/// Revert opt code. It's inserted at the precompile addresses, to make them functional in EVM.
pub const EVM_REVERT_CODE: &[u8] = &[0x60, 0x00, 0x60, 0x00, 0xfd];