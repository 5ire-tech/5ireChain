#![cfg(feature = "runtime-benchmarks")]

use crate::*;
use crate::Pallet as Esg;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;
use frame_support::{inherent::Vec, WeakBoundedVec};

benchmarks! {

	register_an_oracle {
		let account: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Root, account.clone(), true)
	verify {
		assert_eq!(Esg::<T>::get_oracle_sudo().contains(&account), true);
	}

	deregister_an_oracle {
		let account1: T::AccountId = whitelisted_caller();
		let fn_mutate = |oracles: &mut Vec<<T as frame_system::Config>::AccountId>| oracles.push(account1.clone());
		<SudoOraclesStore<T>>::mutate(fn_mutate);

	}: _(RawOrigin::Root, account1.clone(), true)
	verify {
		assert_eq!(Esg::<T>::get_oracle_sudo().contains(&account1), false);
	}

	upsert_esg_scores {

		let caller1: T::AccountId = whitelisted_caller();
		let fn_mutate = |oracles: &mut Vec<<T as frame_system::Config>::AccountId>| oracles.push(caller1.clone());
		<SudoOraclesStore<T>>::mutate(fn_mutate);
		let company1: T::AccountId = account("apple", 0, 0);
		let company2: T::AccountId = account("microsoft", 1, 1);

		let data = include_str!("data.json");
	}: _(RawOrigin::Signed(caller1),(WeakBoundedVec::try_from(data.as_bytes().to_vec())).unwrap())
	verify {
		assert_eq!(Esg::<T>::get_score_of(company1), 1);
		assert_eq!(Esg::<T>::get_score_of(company2), 100);
	}

	impl_benchmark_test_suite!(Esg, crate::mock::new_test_ext(), crate::tests::Test)
}
