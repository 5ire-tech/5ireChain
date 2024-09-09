use super::*;
use frame_support::traits::{ Get, GetStorageVersion, StorageVersion };

/// Migrate the pallet storage to v1.
pub fn migrate_to_v1<T: Config>() -> frame_support::weights::Weight {
	let onchain_version = Pallet::<T>::on_chain_storage_version();

	if onchain_version < 2 {
		let mut count = 0;
		for (nominator_id, balance) in NominatorRewardAccounts::<T>::iter() {
			if let Some(nominations) = pallet_staking::Nominators::<T>::get(nominator_id.clone()) {
				if let Some(validator) = nominations.targets.first() {
					NominatorEarningsAccount::<T>::insert(
						validator.clone(),
						nominator_id.clone(),
						balance
					);
				}
				NominatorRewardAccounts::<T>::remove(nominator_id.clone());
			}
			count += 1;
		}
		StorageVersion::new(2).put::<Pallet<T>>();
		T::DbWeight::get().reads_writes((count as u64) + 1, (count as u64) + 1)
	} else {
		T::DbWeight::get().reads(1)
	}
}
