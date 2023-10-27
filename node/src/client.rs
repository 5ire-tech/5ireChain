use std::sync::Arc;
// Substrate
use sc_executor::{NativeElseWasmExecutor, NativeVersion};
use sc_service::ChainSpec;
// Local
use firechain_runtime_core_primitives::opaque::{
	AccountId, Balance, Block, BlockNumber, Hash, Header, Nonce,
};
use sc_client_api::{Backend as BackendT, BlockchainEvents, KeysIter, PairsIter};
use sp_api::{CallApiAt, NumberFor, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_consensus::BlockStatus;
use sp_runtime::{
	generic::SignedBlock,
	traits::{BlakeTwo256, Block as BlockT},
	Justifications,
};
use sp_storage::{ChildInfo, StorageData, StorageKey};

use crate::eth::EthCompatRuntimeApiCollection;

/// Full backend.
pub type FullBackend = sc_service::TFullBackend<Block>;
/// Full client.
pub type FullClient<RuntimeApi, Executor> =
	sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>;

/// A client instance
#[derive(Clone)]
pub enum Client {
	#[cfg(feature = "firechain-qa")]
	Qa(Arc<FullClient<firechain_qa_runtime::RuntimeApi, FirechainQaRuntimeExecutor>>),
	#[cfg(feature = "firechain-uat")]
	Uat(Arc<FullClient<firechain_uat_runtime::RuntimeApi, FirechainUatRuntimeExecutor>>),
	#[cfg(feature = "firechain-thunder")]
	Thunder(
		Arc<FullClient<firechain_thunder_runtime::RuntimeApi, FirechainThunderRuntimeExecutor>>,
	),
}

/// Only enable the benchmarking host functions when we actually want to benchmark.
// #[cfg(feature = "runtime-benchmarks")]
// pub type HostFunctions = frame_benchmarking::benchmarking::HostFunctions;

#[cfg(feature = "firechain-qa")]
pub struct FirechainQaRuntimeExecutor;

#[cfg(feature = "firechain-qa")]
impl sc_executor::NativeExecutionDispatch for FirechainQaRuntimeExecutor {
	type ExtendHostFunctions = sp_statement_store::runtime_api::statement_store::HostFunctions;

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		firechain_qa_runtime::api::dispatch(method, data)
	}

	fn native_version() -> NativeVersion {
		firechain_qa_runtime::native_version()
	}
}

#[cfg(feature = "firechain-thunder")]
pub struct FirechainThunderRuntimeExecutor;

#[cfg(feature = "firechain-thunder")]
impl sc_executor::NativeExecutionDispatch for FirechainThunderRuntimeExecutor {
	type ExtendHostFunctions = sp_statement_store::runtime_api::statement_store::HostFunctions;

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		firechain_thunder_runtime::api::dispatch(method, data)
	}

	fn native_version() -> NativeVersion {
		firechain_thunder_runtime::native_version()
	}
}

#[cfg(feature = "firechain-uat")]
pub struct FirechainUatRuntimeExecutor;

#[cfg(feature = "firechain-uat")]
impl sc_executor::NativeExecutionDispatch for FirechainUatRuntimeExecutor {
	type ExtendHostFunctions = sp_statement_store::runtime_api::statement_store::HostFunctions;

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		firechain_uat_runtime::api::dispatch(method, data)
	}

	fn native_version() -> NativeVersion {
		firechain_uat_runtime::native_version()
	}
}

#[cfg(feature = "firechain-qa")]
impl From<Arc<FullClient<firechain_qa_runtime::RuntimeApi, FirechainQaRuntimeExecutor>>>
	for Client
{
	fn from(
		client: Arc<FullClient<firechain_qa_runtime::RuntimeApi, FirechainQaRuntimeExecutor>>,
	) -> Self {
		Self::Qa(client)
	}
}

#[cfg(feature = "firechain-uat")]
impl From<Arc<FullClient<firechain_uat_runtime::RuntimeApi, FirechainUatRuntimeExecutor>>>
	for Client
{
	fn from(
		client: Arc<FullClient<firechain_uat_runtime::RuntimeApi, FirechainUatRuntimeExecutor>>,
	) -> Self {
		Self::Uat(client)
	}
}

#[cfg(feature = "firechain-thunder")]
impl From<Arc<FullClient<firechain_thunder_runtime::RuntimeApi, FirechainThunderRuntimeExecutor>>>
	for Client
{
	fn from(
		client: Arc<
			FullClient<firechain_thunder_runtime::RuntimeApi, FirechainThunderRuntimeExecutor>,
		>,
	) -> Self {
		Self::Thunder(client)
	}
}

/// Config that abstracts over all available client implementations.
///
/// For a concrete type there exists [`Client`].
pub trait AbstractClient<Block, Backend>:
	BlockchainEvents<Block>
	+ Sized
	+ Send
	+ Sync
	+ ProvideRuntimeApi<Block>
	+ HeaderBackend<Block>
	+ CallApiAt<Block, StateBackend = Backend::State>
where
	Block: BlockT,
	Backend: BackendT<Block>,
	Backend::State: sp_api::StateBackend<BlakeTwo256>,
	Self::Api: RuntimeApiCollection<StateBackend = Backend::State>,
{
}

impl<Block, Backend, Client> AbstractClient<Block, Backend> for Client
where
	Block: BlockT,
	Backend: BackendT<Block>,
	Backend::State: sp_api::StateBackend<BlakeTwo256>,
	Client: BlockchainEvents<Block>
		+ ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ Sized
		+ Send
		+ Sync
		+ CallApiAt<Block, StateBackend = Backend::State>,
	Client::Api: RuntimeApiCollection<StateBackend = Backend::State>,
{
}

pub trait ExecuteWithClient {
	/// The return type when calling this instance.
	type Output;

	/// Execute whatever should be executed with the given client instance.
	fn execute_with_client<Client, Api, Backend>(self, client: Arc<Client>) -> Self::Output
	where
		<Api as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
		Backend: sc_client_api::Backend<Block>,
		Backend::State: sp_api::StateBackend<BlakeTwo256>,
		Api: RuntimeApiCollection<StateBackend = Backend::State>,
		Client: AbstractClient<Block, Backend, Api = Api> + 'static;
}

impl ClientHandle for Client {
	fn execute_with<T: ExecuteWithClient>(&self, t: T) -> T::Output {
		match self {
			#[cfg(feature = "firechain-qa")]
			Self::Qa(client) => T::execute_with_client::<_, _, FullBackend>(t, client.clone()),
			#[cfg(feature = "firechain-uat")]
			Self::Uat(client) => T::execute_with_client::<_, _, FullBackend>(t, client.clone()),
			#[cfg(feature = "firechain-thunder")]
			Self::Thunder(client) => T::execute_with_client::<_, _, FullBackend>(t, client.clone()),
		}
	}
}

/// A handle to a client instance.
///
/// The service supports multiple different runtimes (Qa, Uat, Thunder e.t.c.).
/// As each runtime has a specialized client, we need to hide them
/// behind a trait. This is this trait.
///
/// When wanting to work with the inner client, you need to use `execute_with`.
pub trait ClientHandle {
	/// Execute the given something with the client.
	fn execute_with<T: ExecuteWithClient>(&self, t: T) -> T::Output;
}

macro_rules! match_client {
	($self:ident, $method:ident($($param:ident),*)) => {
		match $self {
			#[cfg(feature = "firechain-qa")]
			Self::Qa(client) => client.$method($($param),*),
			#[cfg(feature = "firechain-uat")]
			Self::Uat(client) => client.$method($($param),*),
			#[cfg(feature = "firechain-thunder")]
			Self::Thunder(client) => client.$method($($param),*),
		}
	};
}

/// Trivial enum representing runtime variant
#[derive(Clone)]
pub enum RuntimeVariant {
	#[allow(dead_code)]
	#[cfg(feature = "firechain-qa")]
	Qa,
	#[allow(dead_code)]
	#[cfg(feature = "firechain-uat")]
	Uat,
	#[allow(dead_code)]
	#[cfg(feature = "firechain-thunder")]
	Thunder,
	#[allow(dead_code)]
	Unrecognized,
}

/// Can be called for a `Configuration` to check if it is a configuration for
/// the `Firechain` network.
pub trait IdentifyVariant {
	/// Returns `true` if this is a configuration for the `Firechain` qa network.
	fn is_qa(&self) -> bool;

	/// Returns `true` if this is a configuration for the `Firechain` uat network.
	fn is_uat(&self) -> bool;

	/// Returns `true` if this is a configuration for the `Firechain` thunder network.
	fn is_thunder(&self) -> bool;
}

impl IdentifyVariant for Box<dyn ChainSpec> {
	fn is_qa(&self) -> bool {
		self.id().starts_with("qa")
	}

	fn is_uat(&self) -> bool {
		self.id().starts_with("uat")
	}

	fn is_thunder(&self) -> bool {
		self.id().starts_with("thunder")
	}
}

/// A set of APIs that every runtimes must implement.
pub trait BaseRuntimeApiCollection:
	sp_api::ApiExt<Block>
	+ sp_api::Metadata<Block>
	+ sp_block_builder::BlockBuilder<Block>
	+ sp_offchain::OffchainWorkerApi<Block>
	+ sp_session::SessionKeys<Block>
	+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
where
	<Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

impl<Api> BaseRuntimeApiCollection for Api
where
	Api: sp_api::ApiExt<Block>
		+ sp_api::Metadata<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ sp_offchain::OffchainWorkerApi<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>,
	<Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

/// A set of APIs that template runtime must implement.
pub trait RuntimeApiCollection:
	BaseRuntimeApiCollection
	+ EthCompatRuntimeApiCollection
	+ sp_consensus_babe::BabeApi<Block>
	+ sp_consensus_grandpa::GrandpaApi<Block>
	+ frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
	+ pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
where
	<Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

impl<Api> RuntimeApiCollection for Api
where
	Api: BaseRuntimeApiCollection
		+ EthCompatRuntimeApiCollection
		+ sp_consensus_babe::BabeApi<Block>
		+ sp_consensus_grandpa::GrandpaApi<Block>
		+ frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
		+ pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>,
	<Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

impl sc_client_api::UsageProvider<Block> for Client {
	fn usage_info(&self) -> sc_client_api::ClientInfo<Block> {
		match_client!(self, usage_info())
	}
}

impl sc_client_api::BlockBackend<Block> for Client {
	fn block_body(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<Vec<<Block as BlockT>::Extrinsic>>> {
		match_client!(self, block_body(hash))
	}

	fn block_indexed_body(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<Vec<Vec<u8>>>> {
		match_client!(self, block_indexed_body(hash))
	}

	fn block(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<SignedBlock<Block>>> {
		match_client!(self, block(hash))
	}

	fn block_status(&self, hash: <Block as BlockT>::Hash) -> sp_blockchain::Result<BlockStatus> {
		match_client!(self, block_status(hash))
	}

	fn justifications(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<Justifications>> {
		match_client!(self, justifications(hash))
	}

	fn block_hash(
		&self,
		number: NumberFor<Block>,
	) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
		match_client!(self, block_hash(number))
	}

	fn indexed_transaction(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<Vec<u8>>> {
		match_client!(self, indexed_transaction(hash))
	}

	fn has_indexed_transaction(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<bool> {
		match_client!(self, has_indexed_transaction(hash))
	}

	fn requires_full_sync(&self) -> bool {
		match_client!(self, requires_full_sync())
	}
}

impl sc_client_api::StorageProvider<Block, FullBackend> for Client {
	fn storage(
		&self,
		hash: <Block as BlockT>::Hash,
		key: &StorageKey,
	) -> sp_blockchain::Result<Option<StorageData>> {
		match_client!(self, storage(hash, key))
	}

	fn storage_hash(
		&self,
		hash: <Block as BlockT>::Hash,
		key: &StorageKey,
	) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
		match_client!(self, storage_hash(hash, key))
	}

	fn storage_keys(
		&self,
		hash: <Block as BlockT>::Hash,
		prefix: Option<&StorageKey>,
		start_key: Option<&StorageKey>,
	) -> sp_blockchain::Result<KeysIter<<FullBackend as sc_client_api::Backend<Block>>::State, Block>>
	{
		match_client!(self, storage_keys(hash, prefix, start_key))
	}

	fn storage_pairs(
		&self,
		hash: <Block as BlockT>::Hash,
		key_prefix: Option<&StorageKey>,
		start_key: Option<&StorageKey>,
	) -> sp_blockchain::Result<
		PairsIter<<FullBackend as sc_client_api::Backend<Block>>::State, Block>,
	> {
		match_client!(self, storage_pairs(hash, key_prefix, start_key))
	}

	fn child_storage(
		&self,
		hash: <Block as BlockT>::Hash,
		child_info: &ChildInfo,
		key: &StorageKey,
	) -> sp_blockchain::Result<Option<StorageData>> {
		match_client!(self, child_storage(hash, child_info, key))
	}

	fn child_storage_keys(
		&self,
		hash: <Block as BlockT>::Hash,
		child_info: ChildInfo,
		prefix: Option<&StorageKey>,
		start_key: Option<&StorageKey>,
	) -> sp_blockchain::Result<KeysIter<<FullBackend as sc_client_api::Backend<Block>>::State, Block>>
	{
		match_client!(self, child_storage_keys(hash, child_info, prefix, start_key))
	}

	fn child_storage_hash(
		&self,
		hash: <Block as BlockT>::Hash,
		child_info: &ChildInfo,
		key: &StorageKey,
	) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
		match_client!(self, child_storage_hash(hash, child_info, key))
	}
}

impl sp_blockchain::HeaderBackend<Block> for Client {
	fn header(&self, hash: Hash) -> sp_blockchain::Result<Option<Header>> {
		match_client!(self, header(hash))
	}

	fn info(&self) -> sp_blockchain::Info<Block> {
		match_client!(self, info())
	}

	fn status(&self, hash: Hash) -> sp_blockchain::Result<sp_blockchain::BlockStatus> {
		match_client!(self, status(hash))
	}

	fn number(&self, hash: Hash) -> sp_blockchain::Result<Option<BlockNumber>> {
		match_client!(self, number(hash))
	}

	fn hash(&self, number: NumberFor<Block>) -> sp_blockchain::Result<Option<Hash>> {
		match_client!(self, hash(number))
	}
}
