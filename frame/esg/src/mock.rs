//! Esg pallet tests.
#![allow(dead_code)]

use core::str::FromStr;

use crate as pallet_esg;
use fp_account::AccountId20;
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU32, ConstU64},
};
use frame_system as system;
use sp_core::{Decode, H160, H256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Esg: pallet_esg,
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = fp_account::AccountId20;
	type Lookup = IdentityLookup<Self::AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type Nonce = u64;
	type Block = Block;
}

parameter_types! {
	pub const MaxFileSize: u32 = 1024000;
	pub const MaxNumOfSudoOracles: u32 = 3;
	pub const MaxNumOfNonSudoOracles: u32 = 2;
}

impl pallet_esg::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxFileSize = MaxFileSize;
	type MaxNumOfSudoOracles = MaxNumOfSudoOracles;
	type MaxNumOfNonSudoOracles = MaxNumOfNonSudoOracles;
	type WeightInfo = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}

pub fn hexstr2acc_id20(s: &str) -> <Test as frame_system::Config>::AccountId {
	let acc_id: AccountId20 = H160::from_str(s).map(Into::into).ok().unwrap();
	<Test as frame_system::Config>::AccountId::decode(&mut acc_id.as_ref()).unwrap()
}
