
//! Storage migrations for the Reward pallet.
pub use crate::*;

use frame_support::{
	pallet_prelude::*,
	storage_alias,
	traits::{GetStorageVersion, OnRuntimeUpgrade},
};

use log::{log, Level};

#[cfg(feature = "try-runtime")]
use frame_support::ensure;
use scale_info::TypeInfo;
#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;

/// Used for release versioning upto v1.
///
/// Keeping around to make encoding/decoding of old migration code easier.
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
enum ObsoleteReleases {
	V1_0_0,

}

impl Default for ObsoleteReleases {
	fn default() -> Self {
		ObsoleteReleases::V1_0_0
	}
}

/// Alias to the old storage item used for release versioning. Obsolete since v13.
#[storage_alias]
type StorageVersion<T: Config> = StorageValue<Pallet<T>, ObsoleteReleases, ValueQuery>;

pub mod v1 {
	use super::*;
	const TARGET: &'static str = "runtime::reward::migration::v1";

	pub struct MigrateToV1<T>(sp_std::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
			// frame_support::ensure!(
			// 	StorageVersion::<T>::get() == ObsoleteReleases::V1_0_0,
			// 	"Required v0 before upgrading to v1"
			// );
			let reward_account = RewardAccount::<T>::get();
			log::info!(
				target: TARGET,
				"pre-upgrade state contains '{:?}' reward account.",
				reward_account
			);
			let validators = T::Validators::validators();
			log::info!(target: TARGET, "Validators :{:?}", validators);
			validators.iter().for_each(|validator_id| {
				let validator = T::ValidatorId::convert(validator_id.clone()).unwrap();
				log::info!(target: TARGET, "Validator :{:?}", validator);
			});			


			Ok(Default::default())
		}

		fn on_runtime_upgrade() -> Weight {
			//let current = Pallet::<T>::current_storage_version();
			let onchain = StorageVersion::<T>::get();

			if onchain == ObsoleteReleases::V1_0_0 {
				//StorageVersion::<T>::kill();
				//current.put::<Pallet<T>>();

				log!(Level::Info, "v1 applied successfully");
				T::DbWeight::get().reads_writes(1, 2)
			} else {
				log!(Level::Warn, "Skipping v1, should be removed");
				T::DbWeight::get().reads(1)
			}
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade(_state: Vec<u8>) -> Result<(), TryRuntimeError> {
			//frame_support::ensure!(
			//	Pallet::<T>::on_chain_storage_version() == 1,
			//	"v1 not applied"
			//);

			frame_support::ensure!(
				!StorageVersion::<T>::exists(),
				"Storage version not migrated correctly"
			);

			Ok(())
		}
	}
}
