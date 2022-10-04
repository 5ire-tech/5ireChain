//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::{collections::BTreeMap, sync::Arc};

use jsonrpsee::RpcModule;
use node_template_runtime::{opaque::Block};
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sc_finality_grandpa::{FinalityProofProvider, GrandpaJustificationStream, SharedVoterState, SharedAuthoritySet};
use sc_consensus_epochs::SharedEpochChanges;
use sc_consensus_babe::{Config, Epoch};
use sp_keystore::SyncCryptoStorePtr;
use node_primitives::{Hash, BlockNumber};
use sc_rpc::SubscriptionTaskExecutor;
use sp_consensus::SelectChain;
use sp_consensus_babe::BabeApi;
use sc_client_api::AuxStore;
use sc_client_api::{
	backend::{ Backend, StateBackend, StorageProvider},
	client::BlockchainEvents,
};
use sp_runtime::traits::BlakeTwo256;

use node_primitives::{AccountId, Balance, Index};
use sc_transaction_pool::{ChainApi, Pool};
use sc_network::NetworkService;
use fc_rpc_core::types::{FeeHistoryCache, FeeHistoryCacheLimit, FilterPool};

// Frontier
use fc_rpc::{
	EthBlockDataCacheTask, OverrideHandle, RuntimeApiStorageOverride, SchemaV1Override,
	SchemaV2Override, SchemaV3Override, StorageOverride,
};
use fp_storage::EthereumStorageSchema;

/// Extra dependencies for BABE.
pub struct BabeDeps {
	/// BABE protocol config.
	pub babe_config: Config,
	/// BABE pending epoch changes.
	pub shared_epoch_changes: SharedEpochChanges<Block, Epoch>,
	/// The keystore that manages the keys of the node.
	pub keystore: SyncCryptoStorePtr,
}

/// Extra dependencies for GRANDPA
pub struct GrandpaDeps<B> {
	/// Voting round info.
	pub shared_voter_state: SharedVoterState,
	/// Authority set info.
	pub shared_authority_set: SharedAuthoritySet<Hash, BlockNumber>,
	/// Receives notifications about justification events from Grandpa.
	pub justification_stream: GrandpaJustificationStream<Block>,
	/// Executor to drive the subscription manager in the Grandpa RPC handler.
	pub subscription_executor: SubscriptionTaskExecutor,
	/// Finality proof provider.
	pub finality_provider: Arc<FinalityProofProvider<B, Block>>,
}

/// Full client dependencies.
pub struct FullDeps<C, P, SC, B,A: ChainApi> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// The SelectChain Strategy
	pub select_chain: SC,
	/// A copy of the chain spec.
	pub chain_spec: Box<dyn sc_chain_spec::ChainSpec>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// BABE specific dependencies.
	pub babe: BabeDeps,
	/// GRANDPA specific dependencies.
	pub grandpa: GrandpaDeps<B>,
    	/// Graph pool instance.
	pub graph: Arc<Pool<A>>,
    	/// The Node authority flag
	pub is_authority: bool,
	/// Whether to enable dev signer
	pub enable_dev_signer: bool,
	/// Network service
	pub network: Arc<NetworkService<Block, Hash>>,
	/// EthFilterApi pool.
	pub filter_pool: Option<FilterPool>,
	/// Backend.
	pub backend: Arc<fc_db::Backend<Block>>,
	/// Maximum number of logs in a query.
	pub max_past_logs: u32,
	/// Fee history cache.
	pub fee_history_cache: FeeHistoryCache,
	/// Maximum fee history cache size.
	pub fee_history_cache_limit: FeeHistoryCacheLimit,
	/// Ethereum data access overrides.
	pub overrides: Arc<OverrideHandle<Block>>,
	/// Cache for Ethereum block data.
	pub block_data_cache: Arc<EthBlockDataCacheTask<Block>>,
}


/// Override  extensions.
pub fn overrides_handle<C, BE>(client: Arc<C>) -> Arc<OverrideHandle<Block>>
where
	C: ProvideRuntimeApi<Block> + StorageProvider<Block, BE> + AuxStore,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError>,
	C: Send + Sync + 'static,
	C::Api: sp_api::ApiExt<Block>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>
		+ fp_rpc::ConvertTransactionRuntimeApi<Block>,
	BE: Backend<Block> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
{
	let mut overrides_map = BTreeMap::new();
	overrides_map.insert(
		EthereumStorageSchema::V1,
		Box::new(SchemaV1Override::new(client.clone()))
			as Box<dyn StorageOverride<_> + Send + Sync>,
	);
	overrides_map.insert(
		EthereumStorageSchema::V2,
		Box::new(SchemaV2Override::new(client.clone()))
			as Box<dyn StorageOverride<_> + Send + Sync>,
	);
	overrides_map.insert(
		EthereumStorageSchema::V3,
		Box::new(SchemaV3Override::new(client.clone()))
			as Box<dyn StorageOverride<_> + Send + Sync>,
	);

	Arc::new(OverrideHandle {
		schemas: overrides_map,
		fallback: Box::new(RuntimeApiStorageOverride::new(client)),
	})
}
/// Instantiate all full RPC extensions.
pub fn create_full<C, P, SC, B,BE,A>(
	deps: FullDeps<C, P, SC, B,A>,
    subscription_task_executor: SubscriptionTaskExecutor,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    BE: Backend<Block> + 'static,
    BE::State: StateBackend<BlakeTwo256>,
	C: ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ AuxStore
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ Sync
		+ Send
        + StorageProvider<Block, BE>
		+ 'static,
    C: BlockchainEvents<Block>,
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError>,
    C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber, Hash>,
	C::Api: BabeApi<Block>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool<Block=Block> + 'static,
	SC: SelectChain<Block> + 'static,
    C::Api: fp_rpc::ConvertTransactionRuntimeApi<Block>,
	C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
	B: sc_client_api::Backend<Block> + Send + Sync + 'static,
	B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashFor<Block>>,
    A: ChainApi<Block = Block> + 'static,

{
	use pallet_contracts_rpc::{Contracts, ContractsApiServer};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};
	use sc_consensus_babe_rpc::{Babe, BabeApiServer};
	use sc_finality_grandpa_rpc::{Grandpa, GrandpaApiServer};

    use fc_rpc::{
		Eth,  EthDevSigner, EthFilter, EthFilterApiServer, EthPubSub,EthPubSubApiServer,
		 EthSigner, Net, NetApiServer, Web3, Web3ApiServer,EthApiServer
	};

	// use fc_rpc::{
	// 	Eth, EthApiServer, EthDevSigner, EthFilterApi, EthFilterApiServer, EthPubSubApi,
	// 	EthPubSubApiServer, EthSigner, HexEncodedIdProvider, NetApi, NetApiServer, Web3Api,
	// 	Web3ApiServer,Web3,Net,EthFilter
	// };



	let mut io = RpcModule::new(());
	let FullDeps { client, pool, select_chain, chain_spec: _, deny_unsafe, babe, grandpa,graph,
		is_authority,
		enable_dev_signer,
		network,
		filter_pool,
		backend,
		max_past_logs,
		fee_history_cache,
		fee_history_cache_limit,
		overrides,
		block_data_cache } = deps;

	let BabeDeps { keystore, babe_config, shared_epoch_changes } = babe;
	let GrandpaDeps {
		shared_voter_state,
		shared_authority_set,
		justification_stream,
		subscription_executor,
		finality_provider,
	} = grandpa;
let  pp=pool.clone();
let  pbp=pool.clone();
	io.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;

	io.merge(TransactionPayment::new(client.clone()).into_rpc())?;

    let mut signers = Vec::new();
	if enable_dev_signer {
		signers.push(Box::new(EthDevSigner::new()) as Box<dyn EthSigner>);
	}

	io.merge(
		Babe::new(
			client.clone(),
			shared_epoch_changes.clone(),
			keystore,
			babe_config,
			select_chain,
			deny_unsafe,
		)
		.into_rpc(),
	)?;

	io.merge(
		Grandpa::new(
			subscription_executor,
			shared_authority_set.clone(),
			shared_voter_state,
			justification_stream,
			finality_provider,
		)
		.into_rpc(),
	)?;

    io.merge(
		Eth::new(
			client.clone(),
			pp,
			graph,
			Some(node_template_runtime::TransactionConverter),
			network.clone(),
			signers,
			overrides.clone(),
			backend.clone(),
			// Is authority.
			is_authority,
			block_data_cache.clone(),
			fee_history_cache,
			fee_history_cache_limit,
		)
		.into_rpc(),
	)?;
//	P: TransactionPool<Block = B> + Send + Sync + 'static,

    if let Some(filter_pool) = filter_pool {
		
		io.merge(
			EthFilter::new(
				client.clone(),
				backend,
				filter_pool,
				500_usize, // max stored filters
				max_past_logs,
				block_data_cache,
			)
			.into_rpc(),
		)?;
	}
	io.merge(
		EthPubSub::new(
			pbp,
			client.clone(),
			network.clone(),
			subscription_task_executor,
			overrides,
		)
		.into_rpc(),
	)?;

	io.merge(Contracts::new(client.clone()).into_rpc())?;
	io.merge(
		Net::new(
			client.clone(),
			network,
			// Whether to format the `peer_count` response as Hex (default) or not.
			true,
		)
		.into_rpc(),
	)?;

	io.merge(Web3::new(client).into_rpc())?;

	// Extend this RPC with a custom API by using the following syntax.
	// `YourRpcStruct` should have a reference to a client, which is needed
	// to call into the runtime.
	// `io.extend_with(YourRpcTrait::to_delegate(YourRpcStruct::new(ReferenceToClient, ...)));`

	Ok(io)
}
