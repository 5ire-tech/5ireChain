// we implement OneSessionHandlerAll for pallet-session
// pallet-babe, pallet-grandpa, pallet-im-online

use codec::Decode;
use sp_runtime::{BoundToRuntimeAppPublic, RuntimeAppPublic};

pub trait OneSessionHandlerAll<ValidatorId>: BoundToRuntimeAppPublic {
	/// The key type expected.
	type Key: Decode + RuntimeAppPublic;

	/// Session set has changed; act appropriately. Note that this can be called
	/// before initialization of your module.
	///
	/// `changed` is true when at least one of the session keys
	/// or the underlying economic identities/distribution behind one the
	/// session keys has changed, false otherwise.
	///
	/// The `validators` are the validators of the incoming session, and `queued_validators`
	/// will follow.
	fn on_new_session_all<'a, I: 'a>(changed: bool, validators: I, queued_validators: I)
	where
		I: Iterator<Item = (&'a ValidatorId, Self::Key)>,
		ValidatorId: 'a;
}
