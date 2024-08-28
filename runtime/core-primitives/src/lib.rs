#![cfg_attr(not(feature = "std"), no_std)]

pub mod opaque {
	pub use node_primitives::{
		Hash,	// AccountId, AccountIndex, Balance, BlockNumber Hash, Moment, Nonce, Signature,
	};
	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
	use sp_runtime::{generic, traits::BlakeTwo256};

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;

	use sp_runtime::{
	
		traits::{ IdentifyAccount, Verify},
	};
	
	/// An index to a block.
	pub type BlockNumber = u32;
	
	/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
	pub type Signature = fp_account::EthereumSignature;
	
	/// Some way of identifying an account on the chain. We intentionally make it equivalent
	/// to the public key of our transaction signing scheme.
	pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
	
	/// The type for looking up accounts. We don't expect more than 4 billion of them.
	pub type AccountIndex = u32;
	
	/// Balance of an account.
	pub type Balance = u128;
	
	/// Type used for expressing timestamp.
	pub type Moment = u64;
	
	/// Index of a transaction in the chain.
	pub type Nonce = u32;
	
	/// A timestamp: milliseconds since the unix epoch.
	/// `u64` is enough to represent a duration of half a billion years, when the
	/// time scale is milliseconds.
	pub type Timestamp = u64;
	
	/// Digest item type.
	pub type DigestItem = generic::DigestItem;

	
}
