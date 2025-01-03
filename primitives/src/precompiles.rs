use core::marker::PhantomData;

use fp_evm::{ExitError, PrecompileFailure};
use frame_support::{
	dispatch::{DispatchClass, GetDispatchInfo, Pays},
	traits::Contains,
};
use pallet_evm_precompile_dispatch::DispatchValidateT;

/// Struct that allows only calls based on `Filter` to pass through
pub struct DispatchFilterValidate<RuntimeCall, Filter: Contains<RuntimeCall>>(
	PhantomData<(RuntimeCall, Filter)>,
);

impl<AccountId, RuntimeCall: GetDispatchInfo, Filter: Contains<RuntimeCall>>
	DispatchValidateT<AccountId, RuntimeCall> for DispatchFilterValidate<RuntimeCall, Filter>
{
	fn validate_before_dispatch(
		_origin: &AccountId,
		call: &RuntimeCall,
	) -> Option<PrecompileFailure> {
		let info = call.get_dispatch_info();
		let paid_normal_call = info.pays_fee == Pays::Yes && info.class == DispatchClass::Normal;
		if !paid_normal_call {
			return Some(PrecompileFailure::Error {
				exit_status: ExitError::Other("invalid call".into()),
			});
		}
		if Filter::contains(call) {
			None
		} else {
			Some(PrecompileFailure::Error {
				exit_status: ExitError::Other("call filtered out".into()),
			})
		}
	}
}
