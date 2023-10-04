#![cfg_attr(not(feature = "std"), no_std)]

pub mod opaque {
	pub use node_primitives::{
		AccountId, AccountIndex, Balance, BlockNumber, Hash, Moment, Nonce, Signature,
	};
	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
	use sp_runtime::{generic, traits::BlakeTwo256};

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
}
