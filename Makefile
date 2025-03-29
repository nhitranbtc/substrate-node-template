PHONY: check
check:
	cargo check

PHONY: build
build:
	cargo build --package substrate-node-template

.PHONY: build-release
build-release:
	cargo build --package substrate-node-template --release

.PHONY: run
run:
	./target/release/substrate-node-template --dev --alice --tmp -lruntime=info --rpc-external

.PHONY: release
release:
	cargo run --release --package substrate-node-template -- --dev --tmp -lruntime=debug --rpc-external


.PHONY: build-upgrade
build-upgrade:
	cargo build --release --package substrate-node-template

PHONY: build runtime
build runtime:
	cargo build --package node-template-runtime


PHONY: evm-node
evm-node:
	cargo build --package evm-node

.PHONY: run-evm-node
run-evm-node:
	./target/debug/evm-node --port 30333 --rpc-port 9944 --rpc-cors all --alice --dev
