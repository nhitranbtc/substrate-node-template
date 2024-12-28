use crate::*;
use alloc::format;
use node_primitives::{
	genesis::{get_account_id_from_seed, get_from_seed, GenesisAccount},
	AccountId, Balance, Signature,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use polkadot_sdk::*;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_beefy::ecdsa_crypto::AuthorityId as BeefyId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::crypto::Ss58Codec;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_mixnet::types::AuthorityId as MixnetId;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

/// Provide the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &sp_genesis_builder::PresetId) -> Option<Vec<u8>> {
	let genesis = match id.try_into() {
		Ok("development") => default_config(),
		_ => return None,
	};
	Some(
		serde_json::to_string(&genesis)
			.expect("serialization to json is expected to work. qed.")
			.into_bytes(),
	)
}

/// Helper function to generate stash, controller and session key from seed.
pub fn authority_keys_from_seed(
	seed: &str,
) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId, MixnetId, BeefyId)
{
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
		get_from_seed::<MixnetId>(seed),
		get_from_seed::<BeefyId>(seed),
	)
}

/// Get the default genesis config for the local runtime.
pub fn default_config() -> serde_json::Value {
	let alice = GenesisAccount::<sr25519::Public>::from_seed("Alice");
	let bob = GenesisAccount::<sr25519::Public>::from_seed("Bob");
	let charlie = GenesisAccount::<sr25519::Public>::from_seed("Charlie");
	let dave = GenesisAccount::<sr25519::Public>::from_seed("Dave");
	let eve = GenesisAccount::<sr25519::Public>::from_seed("Eve");
	let balances: Vec<(AccountId, Balance)> = vec![
		(alice.account_id(), 1_000_000_000 * APPLE),
		(bob.account_id(), 1_000_000_000 * APPLE),
		(TreasuryPalletId::get().into_account_truncating(), 1_000_000_000 * APPLE),
		//TODO
		//(CommunityTreasuryPalletId::get().into_account_truncating(), 1_000_000_000 * AST),
		(
			// Private key: 0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391
			// H160 public address: 0xaaafB3972B05630fCceE866eC69CdADd9baC2771
			AccountId::from_ss58check("5FQedkNQcF2fJPwkB6Z1ZcMgGti4vcJQNs6x85YPv3VhjBBT").unwrap(),
			1_000_000_000 * APPLE,
		),
	];
	let authorities = vec![&alice, &bob];
	let accounts = vec![&alice, &bob, &charlie, &dave, &eve]
		.iter()
		.map(|x| x.account_id())
		.collect::<Vec<_>>();

	serde_json::json!(
		{
			"balances": {
				"balances": balances
			},
		}
	)
}
