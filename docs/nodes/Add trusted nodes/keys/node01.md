## Generate a Key Pair Sr25519 scheme

To generate a key pair using the Sr25519 scheme, run the following command:
```sh
./target/release/substrate-node-template key generate --scheme Sr25519 --password-filename ./bin/substrate-node/res/password.txt
```


Secret phrase:       canvas custom quote spring pull chair hood analyst front abandon when reward
  Network ID:        substrate
  Secret seed:       0xf3b8e69784a43ebffae1288d8f75bb2b03cdea627fe4321867f8984fe39406b4
  Public key (hex):  0x0086acd5895bf544e17e52336b97b71a7d004b452bd93f7c6353093963fd9c2c
  Account ID:        0x0086acd5895bf544e17e52336b97b71a7d004b452bd93f7c6353093963fd9c2c
  Public key (SS58): 5C5Ps4raJyqJ9CGAkWor6ffBQyWiGLnhbu236WGaNiqrET3D
  SS58 Address:      5C5Ps4raJyqJ9CGAkWor6ffBQyWiGLnhbu236WGaNiqrET3D


## Inspect a Key Pair Ed25519 scheme


```sh
./target/release/substrate-node-template key inspect --password-filename ./bin/substrate-node/res/password.txt --scheme Ed25519 "canvas custom quote spring pull chair hood analyst front abandon when reward"
```

Secret phrase:       canvas custom quote spring pull chair hood analyst front abandon when reward
  Network ID:        substrate
  Secret seed:       0xf3b8e69784a43ebffae1288d8f75bb2b03cdea627fe4321867f8984fe39406b4
  Public key (hex):  0xb5292427dda53da4a05e7a0f63dae2b8fdea0abd7b97f569b737fc2891c1e17c
  Account ID:        0xb5292427dda53da4a05e7a0f63dae2b8fdea0abd7b97f569b737fc2891c1e17c
  Public key (SS58): 5GAEkD6RzaKbMftQbimKnU1aXWc1i1HsF7JsyVvuKpA4VH1b
  SS58 Address:      5GAEkD6RzaKbMftQbimKnU1aXWc1i1HsF7JsyVvuKpA4VH1b


## Insert a Key: /tmp/node02/keystore/

### Insert babe keystore (Sr25519 scheme)


```sh
./target/release/substrate-node-template key insert --base-path /tmp/node01 --chain=./bin/substrate-node/res/customSpecRaw.json --password-filename ./bin/substrate-node/res/password.txt --scheme Sr25519 --suri 0xf3b8e69784a43ebffae1288d8f75bb2b03cdea627fe4321867f8984fe39406b4 --key-type babe
```

### Insert grandpa keystore (Ed25519 scheme)

```sh
./target/release/substrate-node-template key insert --base-path /tmp/node01 --chain=./bin/substrate-node/res/customSpecRaw.json --password-filename ./bin/substrate-node/res/password.txt --scheme Ed25519 --suri 0xf3b8e69784a43ebffae1288d8f75bb2b03cdea627fe4321867f8984fe39406b4 --key-type gran
```
