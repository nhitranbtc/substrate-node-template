#![warn(missing_docs)]

use polkadot_sdk::*;
use substrate_node_template as node_cli;

fn main() -> sc_cli::Result<()> {
	node_cli::run()
}
