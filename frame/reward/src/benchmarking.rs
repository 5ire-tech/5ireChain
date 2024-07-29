#![cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_benchmarking::whitelisted_caller;
#[allow(unused)]
use crate::Pallet as Reward;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin as SystemOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn get_rewards() {
		let caller: T::AccountId = whitelisted_caller();
		let validator: T::AccountId = whitelisted_caller();
        let balance : T::Balance = 5000u128.into();
        ValidatorRewardAccounts::<T>::insert(validator.clone(),balance);

		#[extrinsic_call]
		get_rewards(SystemOrigin::Signed(caller), validator.clone());

		assert!(ValidatorRewardAccounts::<T>::contains_key(&validator.clone()));
	}

	impl_benchmark_test_suite!(Reward, crate::mock::new_test_ext(), crate::mock::Test);
}
