use crate::{eth::EthCompatRuntimeApiCollection, service::FullClient};
use firechain_runtime_core_primitives::opaque::{
	AccountId, Balance, Block, BlockNumber, Hash, Header, Nonce,
};
use sp_api::ConstructRuntimeApi;
/// A set of APIs that every runtimes must implement.
pub trait BaseRuntimeApiCollection:
	sp_api::ApiExt<Block>
	+ sp_api::Metadata<Block>
	+ sp_block_builder::BlockBuilder<Block>
	+ sp_offchain::OffchainWorkerApi<Block>
	+ sp_session::SessionKeys<Block>
	+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
{
}

impl<Api> BaseRuntimeApiCollection for Api where
	Api: sp_api::ApiExt<Block>
		+ sp_api::Metadata<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ sp_offchain::OffchainWorkerApi<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
{
}


/// A set of APIs that template runtime must implement.
pub trait RuntimeApiCollection:
	BaseRuntimeApiCollection
	+ EthCompatRuntimeApiCollection
	+ sp_consensus_babe::BabeApi<Block>
    + sp_mixnet::runtime_api::MixnetApi<Block>
    + sp_statement_store::runtime_api::ValidateStatement<Block>
	+ grandpa_primitives::GrandpaApi<Block>
    + substrate_frame_rpc_system::AccountNonceApi<Block,AccountId, Nonce>
	 + frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
	+ pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
{
}

impl<Api> RuntimeApiCollection for Api where
	Api: BaseRuntimeApiCollection
		+ EthCompatRuntimeApiCollection
		+ sp_consensus_babe::BabeApi<Block>
        + sp_statement_store::runtime_api::ValidateStatement<Block>
        + sp_mixnet::runtime_api::MixnetApi<Block>
		+ grandpa_primitives::GrandpaApi<Block>
        + substrate_frame_rpc_system::AccountNonceApi<Block,AccountId, Nonce>
		+ frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
		+ pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
{
}
