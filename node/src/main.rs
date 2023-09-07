//! Substrate Node CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod benchmarking;
mod cli;
mod client;
mod command;
mod eth;
mod rpc;

// mod command_helper;

fn main() -> sc_cli::Result<()> {
	command::run()
}
