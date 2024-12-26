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