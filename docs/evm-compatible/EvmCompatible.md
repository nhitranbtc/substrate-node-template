# Substrate EVM Compatible Blockchain

## Introduction to H160 and H256

H160
H160 is a 160-bit hash, commonly used in blockchain systems to represent addresses. It is derived from the Keccak-256 hash function, which is then truncated to 160 bits (20 bytes). H160 is widely used in Ethereum and other EVM-compatible blockchains to represent account addresses.

H256
H256 is a 256-bit hash, which is a full output of the Keccak-256 hash function. It is commonly used in blockchain systems for various purposes, such as representing transaction hashes, block hashes, and other cryptographic identifiers.

## Generate H160 Account

`subkey` is a command-line utility provided by Substrate to generate and inspect keys. Here are some common commands:

Generate a Key Pair
To generate a key pair using the Sr25519 scheme:

```subkey generate --scheme Sr25519```


Example output:

```

Secret phrase:       canvas custom quote spring pull chair hood analyst front abandon when reward
  Network ID:        substrate
  Secret seed:       0xf3b8e69784a43ebffae1288d8f75bb2b03cdea627fe4321867f8984fe39406b4
  Public key (hex):  0x0086acd5895bf544e17e52336b97b71a7d004b452bd93f7c6353093963fd9c2c
  Account ID:        0x0086acd5895bf544e17e52336b97b71a7d004b452bd93f7c6353093963fd9c2c
  Public key (SS58): 5C5Ps4raJyqJ9CGAkWor6ffBQyWiGLnhbu236WGaNiqrET3D
  SS58 Address:      5C5Ps4raJyqJ9CGAkWor6ffBQyWiGLnhbu236WGaNiqrET3D

  ```

To truncate the hex string `0x0086acd5895bf544e17e52336b97b71a7d004b452bd93f7c6353093963fd9c2c` to convert it to H160, you take the first 20 bytes (40 hex characters) after the 0x prefix. Here is the truncated H160 value:

`0x0086acd5895bf544e17e52336b97b71a7d004b45`

## Frontier

### Key Components
`pallet-evm`: This pallet provides the EVM execution environment within a Substrate runtime. It allows the execution of Ethereum smart contracts and manages EVM accounts.

`pallet-ethereum`: This pallet handles Ethereum-specific transactions and events. It ensures that Ethereum transactions can be processed and included in Substrate blocks.

`pallet-dynamic-fee`: This pallet implements a dynamic fee mechanism similar to Ethereum's EIP-1559, allowing for more predictable transaction fees.

`pallet-base-fee`: This pallet works in conjunction with `pallet-dynamic-fee` to manage the base fee for transactions.

### Configure Runtime

```rust,

impl pallet_evm::Config for Runtime {
	type FeeCalculator = BaseFee;
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;
	type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
	type CallOrigin = pallet_evm::EnsureAddressTruncated;
    type WithdrawOrigin = pallet_evm::EnsureAddressTruncated;
	type AddressMapping = pallet_evm::HashedAddressMapping<BlakeTwo256>;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type PrecompilesType = FrontierPrecompiles<Self>;
	type PrecompilesValue = PrecompilesValue;
	type ChainId = EVMChainId;
	type BlockGasLimit = BlockGasLimit;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type OnChargeTransaction = ();
	type OnCreate = ();
	type FindAuthor = FindAuthorTruncated<Aura>;
	type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
	type SuicideQuickClearLimit = SuicideQuickClearLimit;
	type Timestamp = Timestamp;
	type WeightInfo = pallet_evm::weights::SubstrateWeight<Self>;
}


impl pallet_ethereum::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type StateRoot = pallet_ethereum::IntermediateStateRoot<Self>;
	type PostLogContent = PostBlockAndTxnHashes;
	type ExtraDataLength = ConstU32<30>;
}


impl pallet_base_fee::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Threshold = BaseFeeThreshold;
	type DefaultBaseFeePerGas = DefaultBaseFeePerGas;
	type DefaultElasticity = DefaultElasticity;
}


#[frame_support::runtime]
mod runtime {

    ...

    #[runtime::pallet_index(80)]
	pub type EVM = pallet_evm::Pallet<Runtime>;

	#[runtime::pallet_index(81)]
	pub type Ethereum = pallet_ethereum::Pallet<Runtime>;

	#[runtime::pallet_index(82)]
	pub type BaseFee = pallet_base_fee::Pallet<Runtime>;

	#[runtime::pallet_index(83)]
	pub type EVMChainId = pallet_evm_chain_id::Pallet<Runtime>;
    ...

}
```

### Genesis Configuration

```rust,

fn testnet_genesis(
    initial_authorities: Vec<(AccountId, AccountId, GrandpaId, AuraId)>,
    initial_nominators: Vec<AccountId>,
    root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>,
) -> serde_json::Value {
    let (initial_authorities, endowed_accounts, num_endowed_accounts, stakers) =
        configure_accounts(initial_authorities, initial_nominators, endowed_accounts, STASH);

    let evm_accounts = {
        let mut map = BTreeMap::new();
        map.insert(
            H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
                .expect("internal H160 is valid; qed"),
            GenesisAccount {
                balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
                    .expect("internal U256 is valid; qed"),
                code: Default::default(),
                nonce: Default::default(),
                storage: Default::default(),
            },
        );
        map
    };

    serde_json::json!({
        ...

        "evmChainId" : { "chainId": 42 },

        "evm": {
            "accounts": evm_accounts,
        },
        ...
    })
}

```

###  Interact with EVM

#### Interact on chain

#### Interact on remix

You can interact with the EVM using tools like MetaMask and Remix by connecting to your Substrate node.
