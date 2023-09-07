//! Substrate Node CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod benchmarking;
mod cli;
mod command;
mod rpc;
mod eth;
mod client;

// mod command_helper;

fn main() -> sc_cli::Result<()> {
	command::run()
}
