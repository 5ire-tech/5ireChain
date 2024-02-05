//! Esg pallet tests.

use crate as pallet_esg;
use frame_support::traits::{ConstU16, ConstU32, ConstU64};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	AccountId32,
};
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
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
	type Index = u64;
	type BlockNumber = u64;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId32;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
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
}

impl pallet_esg::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxFileSize = ConstU32<1024000>;
	type WeightInfo = ();
}

pub const ROOT: AccountId32 = AccountId32::new([0u8; 32]);
pub const SUDO_ORACLE: AccountId32 = AccountId32::new([0u8; 32]);
pub const SUDO_ORACLE_2: AccountId32 = AccountId32::new([1u8; 32]);
pub const NON_SUDO_ORACLE: AccountId32 = AccountId32::new([2u8; 32]);
pub const NON_SUDO_ORACLE_2: AccountId32 = AccountId32::new([3u8; 32]);

pub const ALICE: AccountId32 = AccountId32::new([4u8; 32]);
pub const DUMMY_SUDO_ORACLE: AccountId32 = AccountId32::new([5u8; 32]);

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
