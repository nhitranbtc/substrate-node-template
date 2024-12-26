#![warn(missing_docs)]

use polkadot_sdk::*;
use staging_node_cli as node_cli;

fn main() -> sc_cli::Result<()> {
	node_cli::run()
}
