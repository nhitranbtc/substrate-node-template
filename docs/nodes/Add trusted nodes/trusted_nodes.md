## Start the first node

```sh
./target/release/substrate-node-template \
--chain=bin/substrate-node/res/customSpecRaw.json \
--base-path /tmp/node01 \
--validator \
--rpc-methods Unsafe \
--name MyNode01 \
--node-key=c0c614879fe5cc29086ae94d7e998b6f2fb3f782a8aa45d08e8b5ae1a6938b7b \
--port 30333 \
--rpc-port 9944
```

## Enable other participants to join


```sh
./target/release/substrate-node-template \
--chain=bin/substrate-node/res/customSpecRaw.json \
--base-path /tmp/node02 \
--validator \
--rpc-methods Unsafe \
--name MyNode02 \
--node-key=256c6d357a12a94e3b2f9a1f2a332a2f94eaf751426d4917b7cba2a426b0b8d2 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWJAE5mdsVTDnMCXLiHuJdFd3B5yfMhbHyGjggCdbPL9yK \
--port 30334 \
--rpc-port 9945
```
