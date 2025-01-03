PHONY: check
check:
	cargo check

PHONY: build
build:
	cargo build

.PHONY: build-node
build-node:
	cargo build --package substrate-node-template --release

.PHONY: run
run:
	./target/release/substrate-node-template --dev --alice --tmp -lruntime=info --rpc-external

.PHONY: release-node
release-node:
	cargo run --release --package substrate-node-template -- --dev --tmp -lruntime=debug --rpc-external


.PHONY: build-upgrade
build-upgrade:
	cargo build --release --package substrate-node-template