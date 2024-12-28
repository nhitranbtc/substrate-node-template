## Generate a Key Pair Sr25519 scheme

```sh
./target/release/substrate-node-template key generate --scheme Sr25519 --password-filename ./bin/substrate-node/res/password.txt
```

Secret phrase:       flash lamp empower crime phrase leader amateur food cloud usage affair kind
  Network ID:        substrate
  Secret seed:       0x3f445e8bfd7fa8162aa22dec768e2badff669091550b1105e4e8c5f0842bd0ac
  Public key (hex):  0x147bb1e380796d77c76cd010a923c4cbcb058577da93c08352122cd6f6eaaa34
  Account ID:        0x147bb1e380796d77c76cd010a923c4cbcb058577da93c08352122cd6f6eaaa34
  Public key (SS58): 5CXZZRnVoEhRdFR1bLdAFsFpyb2ZrznYysSZ3xm5MiHmVJnC
  SS58 Address:      5CXZZRnVoEhRdFR1bLdAFsFpyb2ZrznYysSZ3xm5MiHmVJnC


## Inspect a Key Pair Ed25519 scheme

```sh
./target/release/substrate-node-template key inspect --password-filename ./bin/substrate-node/res/password.txt --scheme Ed25519 "flash lamp empower crime phrase leader amateur food cloud usage affair kind"
```


Secret phrase:       flash lamp empower crime phrase leader amateur food cloud usage affair kind
  Network ID:        substrate
  Secret seed:       0x3f445e8bfd7fa8162aa22dec768e2badff669091550b1105e4e8c5f0842bd0ac
  Public key (hex):  0x408d62a8ae809e16caaa7af85df116864112b4fdf27900c96ae431bce6cdba9e
  Account ID:        0x408d62a8ae809e16caaa7af85df116864112b4fdf27900c96ae431bce6cdba9e
  Public key (SS58): 5DXLvRugSMHm9jzQUuhStX7ENJwYiYTu7PNZfShnBT46Rirm
  SS58 Address:      5DXLvRugSMHm9jzQUuhStX7ENJwYiYTu7PNZfShnBT46Rirm


## Insert a Key: /tmp/node02/keystore/

### Insert babe keystore (Sr25519 scheme)

```sh
./target/release/substrate-node-template key insert --base-path /tmp/node02 --chain=./bin/substrate-node/res/customSpecRaw.json --password-filename ./bin/substrate-node/res/password.txt --scheme Sr25519 --suri 0x3f445e8bfd7fa8162aa22dec768e2badff669091550b1105e4e8c5f0842bd0ac --key-type babe
```

### Insert grandpa keystore (Ed25519 scheme)

```sh
./target/release/substrate-node-template key insert --base-path /tmp/node02 --chain=./bin/substrate-node/res/customSpecRaw.json --password-filename ./bin/substrate-node/res/password.txt --scheme Ed25519 --suri 0x3f445e8bfd7fa8162aa22dec768e2badff669091550b1105e4e8c5f0842bd0ac --key-type gran
```
