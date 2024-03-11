// This file is part of Substrate.

// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
use crate::{
	cli::{Cli, Subcommand},
	service,
	service::new_partial,
};
#[cfg(feature = "runtime-benchmarks")]
use frame_benchmarking_cli::*;

use node_primitives::Block;
use sc_cli::{Result, SubstrateCli};
use sc_service::PartialComponents;

use firechain_node::client::IdentifyVariant;

#[cfg(feature = "firechain-qa")]
use firechain_node::client::FirechainQaRuntimeExecutor;

#[cfg(feature = "firechain-mainnet")]
use firechain_node::client::FirechainMainnetRuntimeExecutor;

#[cfg(feature = "firechain-thunder")]
use firechain_node::client::FirechainThunderRuntimeExecutor;

#[cfg(feature = "firechain-qa")]
use firechain_node::chain_spec::qa_chain_spec;

#[cfg(feature = "firechain-mainnet")]
use firechain_node::chain_spec::mainnet_chain_spec;

#[cfg(feature = "firechain-thunder")]
use firechain_node::chain_spec::thunder_chain_spec;

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"FireChain Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/5ire-tech/5ireChain/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2023
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		#[allow(unused)]
		#[cfg(feature = "firechain-qa")]
		let spec = match id {
			"" =>
				return Err(
					"Please specify which chain you want to run, e.g. --dev or --chain=local"
						.into(),
				),
			"qa-dev" => Box::new(qa_chain_spec::development_config()),
			"qa-local" => Box::new(qa_chain_spec::local_testnet_config()),
			"qa-staging" => Box::new(qa_chain_spec::staging_testnet_config()),
			path =>
				Box::new(qa_chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path))?),
		};

		#[allow(unused)]
		#[cfg(feature = "firechain-mainnet")]
		let spec = match id {
			"" =>
				return Err(
					"Please specify which chain you want to run, e.g. --dev or --chain=local"
						.into(),
				),
			"mainnet-dev" => Box::new(mainnet_chain_spec::development_config()),
			"mainnet-local" => Box::new(mainnet_chain_spec::local_testnet_config()),
			"mainnet-staging" => Box::new(mainnet_chain_spec::staging_testnet_config()),
			path =>
				Box::new(mainnet_chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path))?),
		};

		#[cfg(feature = "firechain-thunder")]
		let spec = match id {
			"" =>
				return Err(
					"Please specify which chain you want to run, e.g. --dev or --chain=local"
						.into(),
				),
			"thunder-dev" => Box::new(thunder_chain_spec::development_config()),
			"thunder-local" => Box::new(thunder_chain_spec::local_testnet_config()),
			"thunder-staging" => Box::new(thunder_chain_spec::staging_testnet_config()),
			path => Box::new(thunder_chain_spec::ChainSpec::from_json_file(
				std::path::PathBuf::from(path),
			)?),
		};

		Ok(spec)
	}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		None => {
			let runner = cli.create_runner(&cli.run)?;
			let chain_spec = &runner.config().chain_spec;
			match chain_spec {
				#[cfg(feature = "firechain-qa")]
				spec if spec.is_qa() => runner.run_node_until_exit(|config| async move {
					service::new_full::<
							firechain_qa_runtime::RuntimeApi,
							FirechainQaRuntimeExecutor,
						>(config, cli.no_hardware_benchmarks, cli.eth.clone())
						.map_err(sc_cli::Error::Service)
				}),

				#[cfg(feature = "firechain-mainnet")]
				spec if spec.is_mainnet() => runner.run_node_until_exit(|config| async move {
					service::new_full::<
						firechain_mainnet_runtime::RuntimeApi,
						FirechainMainnetRuntimeExecutor,
					>(config, cli.no_hardware_benchmarks, cli.eth.clone())
					.map_err(sc_cli::Error::Service)
				}),

				#[cfg(feature = "firechain-thunder")]
				spec if spec.is_thunder() => runner.run_node_until_exit(|config| async move {
					service::new_full::<
						firechain_thunder_runtime::RuntimeApi,
						FirechainThunderRuntimeExecutor,
					>(config, cli.no_hardware_benchmarks, cli.eth.clone())
					.map_err(sc_cli::Error::Service)
				}),

				_ => Err("Chain spec not supported".into()),
			}
		},
		// Some(Subcommand::Inspect(cmd)) => {
		// 	let runner = cli.create_runner(cmd)?;
		//
		// 	runner.sync_run(|config| cmd.run::<Block, RuntimeApi, ExecutorDispatch>(config))
		// },
		#[cfg(feature = "runtime-benchmarks")]
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			// This switch needs to be in the client, since the client decides
			// which sub-commands it wants to support.
			match cmd {
				BenchmarkCmd::Pallet(cmd) =>
					if !cfg!(feature = "runtime-benchmarks") {
						return Err("Runtime benchmarking wasn't enabled when building the node. \
							You can enable it with `--features runtime-benchmarks`."
							.into())
					} else {
						match &runner.config().chain_spec {
							#[cfg(feature = "firechain-qa")]
							spec if spec.is_qa() => runner.sync_run(|config| {
								return cmd
									.run::<firechain_qa_runtime::Block, sp_statement_store::runtime_api::HostFunctions>(
										config,
									)
							}),
							#[cfg(feature = "firechain-mainnet")]
							spec if spec.is_mainnet() => runner.sync_run(|config| {
								return cmd
									.run::<firechain_mainnet_runtime::Block, sp_statement_store::runtime_api::HostFunctions>(
										config,
									)
							}),
							#[cfg(feature = "firechain-thunder")]
							spec if spec.is_thunder() => runner.sync_run(|config| {
								return cmd
									.run::<firechain_thunder_runtime::Block, sp_statement_store::runtime_api::HostFunctions>(
										config,
									)
							}),
							_ => panic!("No runtime feature (qa, mainnet, thunder) is enabled"),
						}
					},
				BenchmarkCmd::Block(cmd) => {
					let chain_spec = &runner.config().chain_spec;

					// ensure that we keep the task manager alive
					match chain_spec {
						#[cfg(feature = "firechain-qa")]
						spec if spec.is_qa() =>
							return runner.sync_run(|config| {
								let partial = new_partial::<
									firechain_qa_runtime::RuntimeApi,
									FirechainQaRuntimeExecutor,
								>(&config, cli.eth.clone())?;

								cmd.run(partial.client)
							}),

						#[cfg(feature = "firechain-mainnet")]
						spec if spec.is_mainnet() =>
							return runner.sync_run(|config| {
								let partial = new_partial::<
									firechain_mainnet_runtime::RuntimeApi,
									FirechainMainnetRuntimeExecutor,
								>(&config, cli.eth.clone())?;

								cmd.run(partial.client)
							}),

						#[cfg(feature = "firechain-thunder")]
						spec if spec.is_thunder() =>
							return runner.sync_run(|config| {
								let partial = new_partial::<
									firechain_thunder_runtime::RuntimeApi,
									FirechainThunderRuntimeExecutor,
								>(&config, cli.eth.clone())?;

								cmd.run(partial.client)
							}),

						_ => Err("Chain spec not supported".into()),
					}
				},
				#[cfg(not(feature = "runtime-benchmarks"))]
				BenchmarkCmd::Storage(_) =>
					Err("Storage benchmarking can be enabled with `--features runtime-benchmarks`."
						.into()),

				BenchmarkCmd::Storage(cmd) => {
					let chain_spec = &runner.config().chain_spec;

					// ensure that we keep the task manager alive
					match chain_spec {
						#[cfg(feature = "firechain-qa")]
						spec if spec.is_qa() =>
							return runner.sync_run(|config| {
								let partial = new_partial::<
									firechain_qa_runtime::RuntimeApi,
									FirechainQaRuntimeExecutor,
								>(&config, cli.eth.clone())?;
								let db = partial.backend.expose_db();
								let storage = partial.backend.expose_storage();

								cmd.run(config, partial.client, db, storage)
							}),

						#[cfg(feature = "firechain-mainnet")]
						spec if spec.is_mainnet() =>
							return runner.sync_run(|config| {
								let partial = new_partial::<
									firechain_mainnet_runtime::RuntimeApi,
									FirechainMainnetRuntimeExecutor,
								>(&config, cli.eth.clone())?;
								let db = partial.backend.expose_db();
								let storage = partial.backend.expose_storage();

								cmd.run(config, partial.client, db, storage)
							}),

						#[cfg(feature = "firechain-thunder")]
						spec if spec.is_thunder() =>
							return runner.sync_run(|config| {
								let partial = new_partial::<
									firechain_thunder_runtime::RuntimeApi,
									FirechainThunderRuntimeExecutor,
								>(&config, cli.eth.clone())?;
								let db = partial.backend.expose_db();
								let storage = partial.backend.expose_storage();

								cmd.run(config, partial.client, db, storage)
							}),

						_ => Err("Chain spec not supported".into()),
					}
				},
				BenchmarkCmd::Overhead(_) |
				BenchmarkCmd::Extrinsic(_) |
				BenchmarkCmd::Machine(_) => Err("Unsupported benchmarking command".into()),
			}
		},
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::Sign(cmd)) => cmd.run(),
		Some(Subcommand::Verify(cmd)) => cmd.run(),
		Some(Subcommand::Vanity(cmd)) => cmd.run(),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) =
					service::new_chain_ops(&mut config, cli.eth.clone())?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, _, _, task_manager) =
					service::new_chain_ops(&mut config, cli.eth.clone())?;

				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, _, _, task_manager) =
					service::new_chain_ops(&mut config, cli.eth.clone())?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) =
					service::new_chain_ops(&mut config, cli.eth.clone())?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;
			match chain_spec {
				#[cfg(feature = "firechain-qa")]
				spec if spec.is_qa() =>
					runner.async_run(|config| {
						let PartialComponents { client, task_manager, backend, .. } =
							new_partial::<
								firechain_qa_runtime::RuntimeApi,
								FirechainQaRuntimeExecutor,
							>(&config, cli.eth.clone())?;

						Ok((cmd.run(client, backend, None), task_manager))
					}),

				#[cfg(feature = "firechain-mainnet")]
				spec if spec.is_mainnet() => runner.async_run(|config| {
					let PartialComponents { client, task_manager, backend, .. } =
						new_partial::<
							firechain_mainnet_runtime::RuntimeApi,
							FirechainMainnetRuntimeExecutor,
						>(&config, cli.eth.clone())?;

					Ok((cmd.run(client, backend, None), task_manager))
				}),

				#[cfg(feature = "firechain-thunder")]
				spec if spec.is_thunder() => runner.async_run(|config| {
					let PartialComponents { client, task_manager, backend, .. } =
						new_partial::<
							firechain_thunder_runtime::RuntimeApi,
							FirechainThunderRuntimeExecutor,
						>(&config, cli.eth.clone())?;

					Ok((cmd.run(client, backend, None), task_manager))
				}),

				_ => Err("Chain spec not supported".into()),
			}
		},
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime) => Err(try_runtime_cli::DEPRECATION_NOTICE.into()),
		#[cfg(not(feature = "try-runtime"))]
		Some(Subcommand::TryRuntime) => Err("TryRuntime wasn't enabled when building the node. \
				You can enable it with `--features try-runtime`."
			.into()),
		Some(Subcommand::ChainInfo(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run::<Block>(&config))
		},
	}
}
