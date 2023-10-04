//! Substrate Node CLI library.
#![warn(missing_docs)]

#[macro_use]
mod service;
mod cli;
mod client;
mod command;
mod eth;
mod rpc;

// mod command_helper;

fn main() -> sc_cli::Result<()> {
	command::run()
}
