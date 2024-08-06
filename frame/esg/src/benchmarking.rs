#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::{Pallet as Esg, *};
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::WeakBoundedVec;
use scale_info::prelude::vec::Vec;
use frame_system::RawOrigin;

// const MAX_NUM_OF_SUDO_ORACLES: u32 = MaxNumOfSudoOracles::get();
// const MAX_NUM_OF_NON_SUDO_ORACLES: u32 = Esg::<{T as Esg::Config}>::MaxNumOfNonSudoOracles::get();
const MAX_NUM_OF_SUDO_ORACLES: u32 = 5;
const MAX_NUM_OF_NON_SUDO_ORACLES: u32 = 100;

fn create_oracle<T: Config>(
	string: &'static str,
	n: u32,
) -> T::AccountId {
	let oracle = account(string, n, 0);
	oracle
}

fn register_oracles<T: Config>() {
	for i in 0 .. (MAX_NUM_OF_SUDO_ORACLES - 1) {
		Esg::<T>::register_an_oracle(RawOrigin::Root.into(), create_oracle::<T>("oracle", u32::MAX - i), true);
	}

	for j in 0 .. (MAX_NUM_OF_NON_SUDO_ORACLES - 1) {
		Esg::<T>::register_an_oracle(RawOrigin::Root.into(), create_oracle::<T>("oracle", u32::MAX - j), false);
	}
}

benchmarks! {

	register_an_oracle {
		let oracle_account: T::AccountId = whitelisted_caller();
		register_oracles::<T>();

	}: _(RawOrigin::Root, oracle_account.clone(), true)
	verify {
		assert_eq!(Esg::<T>::get_oracle_sudo().contains(&oracle_account), true);
	}

	deregister_an_oracle {
		let oracle_account: T::AccountId = whitelisted_caller();
		let fn_mutate = |oracles: &mut Vec<<T as frame_system::Config>::AccountId>| oracles.push(oracle_account.clone());
		<SudoOraclesStore<T>>::mutate(fn_mutate);
		register_oracles::<T>();

	}: _(RawOrigin::Root, oracle_account.clone(), true)
	verify {
		assert_eq!(Esg::<T>::get_oracle_sudo().contains(&oracle_account), false);
	}

	upsert_esg_scores {

		let caller1: T::AccountId = whitelisted_caller();
		let fn_mutate = |oracles: &mut Vec<<T as frame_system::Config>::AccountId>| oracles.push(caller1.clone());
		<SudoOraclesStore<T>>::mutate(fn_mutate);
		register_oracles::<T>();
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
