#![warn(missing_docs)]

use polkadot_sdk::*;
use evm_node as node_cli;

fn main() -> sc_cli::Result<()> {
	node_cli::run()
}
