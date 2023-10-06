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
use frame_benchmarking_cli::*;
use node_primitives::Block;
use sc_cli::{Result, SubstrateCli};
use sc_service::PartialComponents;

use firechain_node::{
	chain_spec::{qa_chain_spec, uat_chain_spec},
	client::{FirechainQaRuntimeExecutor, FirechainUatRuntimeExecutor, IdentifyVariant},
};
#[cfg(feature = "try-runtime")]
use {
	firechain_runtime::constants::time::SLOT_DURATION,
	try_runtime_cli::block_building_info::substrate_info,
};

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
		"https://github.com/5ire-tech/5ire-evm-base/issues/new".into()
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
			"qa" => Box::new(qa_chain_spec::qa_config()?),
			path =>
				Box::new(qa_chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path))?),
		};

		#[cfg(feature = "firechain-uat")]
		let spec = match id {
			"" =>
				return Err(
					"Please specify which chain you want to run, e.g. --dev or --chain=local"
						.into(),
				),
			"uat-dev" => Box::new(uat_chain_spec::development_config()),
			"uat-local" => Box::new(uat_chain_spec::local_testnet_config()),
			"uat-staging" => Box::new(uat_chain_spec::staging_testnet_config()),
			"uat" => Box::new(uat_chain_spec::uat_config()?),
			path =>
				Box::new(uat_chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path))?),
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

				#[cfg(feature = "firechain-uat")]
				spec if spec.is_uat() => runner.run_node_until_exit(|config| async move {
					service::new_full::<
						firechain_uat_runtime::RuntimeApi,
						FirechainUatRuntimeExecutor,
					>(config, cli.no_hardware_benchmarks, cli.eth.clone())
					.map_err(sc_cli::Error::Service)
				}),

				_ => runner.run_node_until_exit(|config| async move {
					service::new_full::<
							firechain_qa_runtime::RuntimeApi,
							FirechainQaRuntimeExecutor,
						>(config, cli.no_hardware_benchmarks, cli.eth.clone())
						.map_err(sc_cli::Error::Service)
				}),
			}
		},
		// Some(Subcommand::Inspect(cmd)) => {
		// 	let runner = cli.create_runner(cmd)?;
		//
		// 	runner.sync_run(|config| cmd.run::<Block, RuntimeApi, ExecutorDispatch>(config))
		// },
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| {
				// This switch needs to be in the client, since the client decides
				// which sub-commands it wants to support.
				match cmd {
					BenchmarkCmd::Pallet(cmd) => {
						if !cfg!(feature = "runtime-benchmarks") {
							return Err(
								"Runtime benchmarking wasn't enabled when building the node. \
							You can enable it with `--features runtime-benchmarks`."
									.into(),
							)
						}

						cmd.run::<Block, sp_statement_store::runtime_api::HostFunctions>(config)
					},
					BenchmarkCmd::Block(cmd) => {
						let runner = cli.create_runner(cmd)?;
						let chain_spec = &runner.config().chain_spec;

						// ensure that we keep the task manager alive
						match chain_spec {
							#[cfg(feature = "firechain-qa")]
							spec if spec.is_qa() => {
								let partial = new_partial::<
									firechain_qa_runtime::RuntimeApi,
									FirechainQaRuntimeExecutor,
								>(&config, cli.eth.clone())?;
								cmd.run(partial.client)
							},

							#[cfg(feature = "firechain-uat")]
							spec if spec.is_uat() => {
								let partial = new_partial::<
									firechain_uat_runtime::RuntimeApi,
									FirechainUatRuntimeExecutor,
								>(&config, cli.eth.clone())?;
								cmd.run(partial.client)
							},

							_ => {
								let partial = new_partial::<
									firechain_qa_runtime::RuntimeApi,
									FirechainQaRuntimeExecutor,
								>(&config, cli.eth.clone())?;
								cmd.run(partial.client)
							},
						}
					},
					#[cfg(not(feature = "runtime-benchmarks"))]
					BenchmarkCmd::Storage(_) => Err(
						"Storage benchmarking can be enabled with `--features runtime-benchmarks`."
							.into(),
					),
					#[cfg(feature = "runtime-benchmarks")]
					BenchmarkCmd::Storage(cmd) => {
						// ensure that we keep the task manager alive
						let runner = cli.create_runner(cmd)?;
						let chain_spec = &runner.config().chain_spec;

						match chain_spec {
							#[cfg(feature = "firechain-qa")]
							spec if spec.is_qa() => {
								let partial = new_partial::<
									firechain_qa_runtime::RuntimeApi,
									FirechainQaRuntimeExecutor,
								>(&config, cli.eth.clone())?;
								let db = partial.backend.expose_db();
								let storage = partial.backend.expose_storage();

								return cmd.run(config, partial.client, db, storage)
							},

							#[cfg(feature = "firechain-uat")]
							spec if spec.is_uat() => {
								let partial = new_partial::<
									firechain_uat_runtime::RuntimeApi,
									FirechainUatRuntimeExecutor,
								>(&config, cli.eth.clone())?;
								let db = partial.backend.expose_db();
								let storage = partial.backend.expose_storage();

								return cmd.run(config, partial.client, db, storage)
							},

							_ => {
								let partial = new_partial::<
									firechain_qa_runtime::RuntimeApi,
									FirechainQaRuntimeExecutor,
								>(&config, cli.eth.clone())?;
								let db = partial.backend.expose_db();
								let storage = partial.backend.expose_storage();

								return cmd.run(config, partial.client, db, storage)
							},
						}
					},
					BenchmarkCmd::Overhead(_) |
					BenchmarkCmd::Extrinsic(_) |
					BenchmarkCmd::Machine(_) => Err("Unsupported benchmarking command".into()),
				}
			})
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

				#[cfg(feature = "firechain-uat")]
				spec if spec.is_uat() => runner.async_run(|config| {
					let PartialComponents { client, task_manager, backend, .. } =
						new_partial::<
							firechain_uat_runtime::RuntimeApi,
							FirechainUatRuntimeExecutor,
						>(&config, cli.eth.clone())?;

					Ok((cmd.run(client, backend, None), task_manager))
				}),

				_ =>
					runner.async_run(|config| {
						let PartialComponents { client, task_manager, backend, .. } =
							new_partial::<
								firechain_qa_runtime::RuntimeApi,
								FirechainQaRuntimeExecutor,
							>(&config, cli.eth.clone())?;

						Ok((cmd.run(client, backend, None), task_manager))
					}),
			}
		},
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			use sc_executor::{sp_wasm_interface::ExtendedHostFunctions, NativeExecutionDispatch};
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				// we don't need any of the components of new_partial, just a runtime, or a task
				// manager to do `async_run`.
				let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
				let task_manager =
					sc_service::TaskManager::new(config.tokio_handle.clone(), registry)
						.map_err(|e| sc_cli::Error::Service(sc_service::Error::Prometheus(e)))?;

				#[cfg(feature = "firechain-qa")]
				let info_provider = substrate_info(firechain_qa_runtime::constants::time::SLOT_DURATION);

				#[cfg(feature = "firechain-uat")]
				let info_provider = substrate_info(firechain_uat_runtime::constants::time::SLOT_DURATION);

				Ok((
					cmd.run::<Block, ExtendedHostFunctions<
						sp_io::SubstrateHostFunctions,
						<ExecutorDispatch as NativeExecutionDispatch>::ExtendHostFunctions,
					>, _>(Some(info_provider)),
					task_manager,
				))
			})
		},
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
