use std::{
	collections::BTreeMap,
	path::PathBuf,
	sync::{Arc, Mutex},
	time::Duration,
};

use futures::{future, prelude::*};
// Substrate
use sc_client_api::BlockchainEvents;
use sp_block_builder::BlockBuilder;
use fc_mapping_sync::SyncStrategy;
use fc_mapping_sync::kv::MappingSyncWorker;
use sc_client_api::StorageProvider;
use sc_client_api::backend::Backend;
use sc_client_api::BlockOf;
use fp_rpc::EthereumRuntimeRPCApi;
use sp_core::H256;
use sc_executor::NativeExecutionDispatch;
use sc_network_sync::SyncingService;
use sp_api::{CallApiAt, ProvideRuntimeApi};
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sp_api::ConstructRuntimeApi;
use sc_client_api::StateBackend;
use sc_executor::NativeElseWasmExecutor;
use sp_blockchain::{
	Backend as BlockchainBackend, Error as BlockChainError, HeaderBackend, HeaderMetadata,
};
use sp_runtime::traits::{BlakeTwo256, Block as BlockT, Header as HeaderT};
// Frontier
pub use fc_consensus::FrontierBlockImport;
use fc_rpc::EthTask;
pub use fc_rpc_core::types::{FeeHistoryCache, FeeHistoryCacheLimit, FilterPool};
pub use fc_storage::{StorageOverride, StorageOverrideHandler};
// Local
use firechain_mainnet_runtime::opaque::Block;

/// Full backend.
pub type FullBackend = sc_service::TFullBackend<Block>;
// /// Full client.
// pub type FullClient<RuntimeApi, Executor> =
// 	sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>;

/// A specialized `WasmExecutor` intended to use across substrate node. It provides all required
/// HostFunctions.
// pub type RuntimeExecutor = sc_executor::WasmExecutor<HostFunctions>;

/// The full client type definition.
// pub type FullClient = sc_service::TFullClient<Block, RuntimeApi, RuntimeExecutor>;
    
/// Frontier DB backend type.
pub type FrontierBackend<C> = fc_db::Backend<Block, C>;

pub fn db_config_dir(config: &Configuration) -> PathBuf {
	config.base_path.config_dir(config.chain_spec.id())
}
/// Avalailable frontier backend types.
#[derive(Debug, Copy,Default, Clone, clap::ValueEnum)]
pub enum BackendType {
	/// Either RocksDb or ParityDb as per inherited from the global backend settings.
	#[default]
	KeyValue,
	/// Sql database with custom log indexing.
	Sql,
}

/// The ethereum-compatibility configuration used to run a node.
#[derive(Clone, Debug,Default, clap::Parser)]
pub struct EthConfiguration {
	/// Maximum number of logs in a query.
	#[arg(long, default_value = "10000")]
	pub max_past_logs: u32,

	/// Maximum fee history cache size.
	#[arg(long, default_value = "2048")]
	pub fee_history_limit: u64,

	#[arg(long)]
	pub enable_dev_signer: bool,

	/// The dynamic-fee pallet target gas price set by block author
	#[arg(long, default_value = "1")]
	pub target_gas_price: u64,

	/// Maximum allowed gas limit will be `block.gas_limit * execute_gas_limit_multiplier`
	/// when using eth_call/eth_estimateGas.
	#[arg(long, default_value = "10")]
	pub execute_gas_limit_multiplier: u64,

	/// Size in bytes of the LRU cache for block data.
	#[arg(long, default_value = "50")]
	pub eth_log_block_cache: usize,

	/// Size in bytes of the LRU cache for transactions statuses data.
	#[arg(long, default_value = "50")]
	pub eth_statuses_cache: usize,

	/// Sets the frontier backend type (KeyValue or Sql)
	#[arg(long, value_enum, ignore_case = true, default_value_t = BackendType::default())]
	pub frontier_backend_type: BackendType,

	// Sets the SQL backend's pool size.
	#[arg(long, default_value = "100")]
	pub frontier_sql_backend_pool_size: u32,

	/// Sets the SQL backend's query timeout in number of VM ops.
	#[arg(long, default_value = "10000000")]
	pub frontier_sql_backend_num_ops_timeout: u32,

	/// Sets the SQL backend's auxiliary thread limit.
	#[arg(long, default_value = "4")]
	pub frontier_sql_backend_thread_count: u32,

	/// Sets the SQL backend's query timeout in number of VM ops.
	/// Default value is 200MB.
	#[arg(long, default_value = "209715200")]
	pub frontier_sql_backend_cache_size: u64,
}

pub struct FrontierPartialComponents {
	pub filter_pool: Option<FilterPool>,
	pub fee_history_cache: FeeHistoryCache,
	pub fee_history_cache_limit: FeeHistoryCacheLimit,
}

pub fn new_frontier_partial(
	config: &EthConfiguration,
) -> Result<FrontierPartialComponents, ServiceError> {
	Ok(FrontierPartialComponents {
		filter_pool: Some(Arc::new(Mutex::new(BTreeMap::new()))),
		fee_history_cache: Arc::new(Mutex::new(BTreeMap::new())),
		fee_history_cache_limit: config.fee_history_limit,
	})
}

/// A set of APIs that ethereum-compatible runtimes must implement.
pub trait EthCompatRuntimeApiCollection:
	sp_api::ApiExt<Block>
	+ fp_rpc::ConvertTransactionRuntimeApi<Block>
	+ fp_rpc::EthereumRuntimeRPCApi<Block>
{
}

impl<Api> EthCompatRuntimeApiCollection for Api where
	Api: sp_api::ApiExt<Block>
		+ fp_rpc::ConvertTransactionRuntimeApi<Block>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>
{
}

pub struct SpawnTasksParams<'a, B: BlockT, C, BE> {
	pub task_manager: &'a TaskManager,
	pub client: Arc<C>,
	pub substrate_backend: Arc<BE>,
	pub frontier_backend: Arc<fc_db::Backend<B, C>>,
	pub filter_pool: Option<FilterPool>,
	pub overrides: Arc<dyn StorageOverride<B>>,
	pub fee_history_limit: u64,
	pub fee_history_cache: FeeHistoryCache,
}

/// Spawn the tasks that are required to run Moonbeam.
pub fn spawn_essential_tasks<B, C, BE>(
	params: SpawnTasksParams<B, C, BE>,
	sync: Arc<SyncingService<B>>,
	pubsub_notification_sinks: Arc<
		fc_mapping_sync::EthereumBlockNotificationSinks<
			fc_mapping_sync::EthereumBlockNotification<B>,
		>,
	>,
) where
	C: ProvideRuntimeApi<B> + BlockOf,
	C: HeaderBackend<B> + HeaderMetadata<B, Error = BlockChainError> + 'static,
	C: BlockchainEvents<B> + StorageProvider<B, BE>,
	C: Send + Sync + 'static,
	C::Api: EthereumRuntimeRPCApi<B>,
	C::Api: BlockBuilder<B>,
	B: BlockT<Hash = H256> + Send + Sync + 'static,
	B::Header: HeaderT<Number = u32>,
	BE: Backend<B> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
{
	// Frontier offchain DB task. Essential.
	// Maps emulated ethereum data to substrate native data.
	match *params.frontier_backend {
		fc_db::Backend::KeyValue(ref b) => {
			params.task_manager.spawn_essential_handle().spawn(
				"frontier-mapping-sync-worker",
				Some("frontier"),
				MappingSyncWorker::new(
					params.client.import_notification_stream(),
					Duration::new(6, 0),
					params.client.clone(),
					params.substrate_backend.clone(),
					params.overrides.clone(),
					b.clone(),
					3,
					0,
					fc_mapping_sync::SyncStrategy::Normal,
					sync.clone(),
					pubsub_notification_sinks.clone(),
				)
				.for_each(|()| futures::future::ready(())),
			);
		}
		fc_db::Backend::Sql(ref b) => {
			params.task_manager.spawn_essential_handle().spawn_blocking(
				"frontier-mapping-sync-worker",
				Some("frontier"),
				fc_mapping_sync::sql::SyncWorker::run(
					params.client.clone(),
					params.substrate_backend.clone(),
					b.clone(),
					params.client.import_notification_stream(),
					fc_mapping_sync::sql::SyncWorkerConfig {
						read_notification_timeout: Duration::from_secs(10),
						check_indexed_blocks_interval: Duration::from_secs(60),
					},
					fc_mapping_sync::SyncStrategy::Normal,
					sync.clone(),
					pubsub_notification_sinks.clone(),
				),
			);
		}
	}

	// Frontier `EthFilterApi` maintenance.
	// Manages the pool of user-created Filters.
	if let Some(filter_pool) = params.filter_pool {
		// Each filter is allowed to stay in the pool for 100 blocks.
		const FILTER_RETAIN_THRESHOLD: u64 = 100;
		params.task_manager.spawn_essential_handle().spawn(
			"frontier-filter-pool",
			Some("frontier"),
			EthTask::filter_pool_task(
				Arc::clone(&params.client),
				filter_pool,
				FILTER_RETAIN_THRESHOLD,
			),
		);
	}

	// Spawn Frontier FeeHistory cache maintenance task.
	params.task_manager.spawn_essential_handle().spawn(
		"frontier-fee-history",
		Some("frontier"),
		EthTask::fee_history_task(
			Arc::clone(&params.client),
			Arc::clone(&params.overrides),
			params.fee_history_cache,
			params.fee_history_limit,
		),
	);
}
