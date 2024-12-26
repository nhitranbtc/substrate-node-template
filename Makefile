PHONY: check
check:
	cargo check

PHONY: build
build:
	cargo build --package staging-node-cli

.PHONY: build-release
build-release:
	cargo build --package staging-node-cli --release

.PHONY: run
run:
	./target/release/substrate-node --dev --alice --tmp -lruntime=info --rpc-external

.PHONY: release
release:
	cargo run --release --package staging-node-cli -- --dev --tmp -lruntime=debug --rpc-external