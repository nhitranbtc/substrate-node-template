use scale_info::prelude::format;
use sp_core::{Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

use super::{AccountId, Signature};

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper struct for genesis configuration
#[derive(Clone, PartialEq, Eq)]
pub struct GenesisAccount<TPublic: Public> {
	/// Account ID
	pub account_id: AccountId,
	/// Public key
	pub pub_key: <TPublic::Pair as Pair>::Public,
}

impl<TPublic: Public> GenesisAccount<TPublic>
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	/// Create a new genesis account from a seed.
	pub fn from_seed(seed: &str) -> Self {
		let pub_key = get_from_seed::<TPublic>(seed);
		let account_id = AccountPublic::from(pub_key.clone()).into_account();

		Self { account_id, pub_key }
	}

	/// Return the `account Id` (address) of the genesis account.
	pub fn account_id(&self) -> AccountId {
		self.account_id.clone()
	}

	/// Return the `public key` of the genesis account.
	pub fn pub_key(&self) -> <TPublic::Pair as Pair>::Public {
		self.pub_key.clone()
	}
}
