#![cfg(test)]
use crate::{self as pallet_reward};
use frame_election_provider_support::{
	bounds::{ElectionBounds, ElectionBoundsBuilder},
	onchain, SequentialPhragmen,
};
use frame_support::pallet_prelude::ConstU32;
use frame_support::traits::ConstU128;
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU64},
};
use pallet_session::historical as pallet_session_historical;
use sp_runtime::testing::UintAuthorityId;
use sp_runtime::testing::H256;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::traits::IdentityLookup;
use sp_runtime::BuildStorage;

pub type RewardBalance = pallet_balances::Pallet<Test>;

use frame_support::PalletId;
use sp_runtime::Perbill;

type Block = frame_system::mocking::MockBlock<Test>;

type AccountId = u64;
type Nonce = u32;
type Balance = u64;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: pallet_balances,
		Staking: pallet_staking,
		Session:pallet_session,
		Offences: pallet_offences,
		Historical: pallet_session_historical,
		EsgScore: pallet_esg,
		Reward: pallet_reward,
		ImOnline: pallet_im_online
	}
);

pub struct TestSessionHandler;
impl pallet_session::SessionHandler<AccountId> for TestSessionHandler {
	const KEY_TYPE_IDS: &'static [sp_runtime::KeyTypeId] = &[];

	fn on_genesis_session<Ks: sp_runtime::traits::OpaqueKeys>(_validators: &[(AccountId, Ks)]) {}

	fn on_new_session<Ks: sp_runtime::traits::OpaqueKeys>(
		_: bool,
		_: &[(AccountId, Ks)],
		_: &[(AccountId, Ks)],
	) {
	}

	fn on_disabled(_: u32) {}
}
/*
pub const ALICE: AccountId = AccountId32::new([0u8; 32]);
pub const BOB: AccountId = AccountId32::new([1u8; 32]);
pub const CHARLIE: AccountId = AccountId32::new([1u8; 32]);
pub const DAVE: AccountId = AccountId32::new([1u8; 32]);
*/

type Origin = <Test as frame_system::Config>::RuntimeOrigin;
pub fn who(who: AccountId) -> Origin {
	RuntimeOrigin::signed(who)
}

parameter_types! {
	pub const Period: u64 = 1;
	pub const Offset: u64 = 0;
}

impl pallet_session::historical::Config for Test {
	type FullIdentification = pallet_staking::Exposure<u64, u128>;
	type FullIdentificationOf = pallet_staking::ExposureOf<Self>;
}

impl pallet_session::Config for Test {
	type WeightInfo = ();
	type DataProvider = Staking;
	type Keys = UintAuthorityId;
	type ValidatorId = AccountId;
	type ValidatorIdOf = pallet_staking::StashOf<Test>;
	type RuntimeEvent = RuntimeEvent;
	type SessionHandler = (ImOnline,);
	type AllSessionHandler = (ImOnline,);
	type TargetsBound = MaxOnChainElectableTargets;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Test, Staking>;
}

parameter_types! {
	pub const SessionsPerEra: sp_staking::SessionIndex = 2;
	pub const BondingDuration: sp_staking::EraIndex = 28;
	pub const SlashDeferDuration: sp_staking::EraIndex = 7; // 1/4 the bonding duration.
	pub const MaxNominatorRewardedPerValidator: u32 = 256;
	pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(40);
	pub HistoryDepth: u32 = 84;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<3>;
	type WeightInfo = ();
}

parameter_types! {

	pub static ElectionsBounds: ElectionBounds = ElectionBoundsBuilder::default().build();
}

pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
	type System = Test;
	type Solver = SequentialPhragmen<AccountId, Perbill>;
	type DataProvider = Staking;
	type WeightInfo = ();
	type MaxWinners = ConstU32<100>;
	type Bounds = ElectionsBounds;
}

impl pallet_staking::Config for Test {
	type RewardRemainder = ();
	type RewardDistribution = Reward;
	type CurrencyToVote = ();
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type CurrencyBalance = <Self as pallet_balances::Config>::Balance;
	type Slash = ();
	type Reward = ();
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SlashDeferDuration = ();
	type AdminOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type SessionInterface = Self;
	type UnixTime = pallet_timestamp::Pallet<Self>;
	type EraPayout = ();
	type MaxNominatorRewardedPerValidator = ConstU32<64>;
	type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
	type NextNewSession = Session;
	type ElectionProvider =
		frame_election_provider_support::onchain::OnChainExecution<OnChainSeqPhragmen>;
	type GenesisElectionProvider = Self::ElectionProvider;
	type VoterList = pallet_staking::UseNominatorsAndValidatorsMap<Self>;
	type TargetList = pallet_staking::UseValidatorsMap<Self>;
	type NominationsQuota = pallet_staking::FixedNominationsQuota<16>;
	type MaxUnlockingChunks = ConstU32<32>;
	type HistoryDepth = ConstU32<84>;
	type EventListeners = ();
	type BenchmarkingConfig = pallet_staking::TestBenchmarkingConfig;
	type WeightInfo = ();
	type ESG = EsgScore;
	type Reliability = ImOnline;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = Nonce;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
	pub const BurnPalletId: PalletId = PalletId(*b"py/burns");
}

impl pallet_balances::Config for Test {
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<100>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type MaxHolds = ();
}

parameter_types! {
	pub const EraMinutes:u32 = 2;
	pub const DecimalPrecision:u32 = 18;
	pub const TotalMinutesPerYear:u32 = 525600;
	pub const TotalReward :u32 = 20564830;
	pub const RewardPalletId: PalletId = PalletId(*b"py/rewrd");
}

impl pallet_reward::Config for Test {
	type RewardCurrency = Balances;
	type Balance = u128;
	type RuntimeEvent = RuntimeEvent;
	type DataProvider = Staking;
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	type ValidatorSet = Historical;
	type Validators = Historical;
	type ValidatorId = pallet_staking::StashOf<Self>;
	type Precision = DecimalPrecision;
	type TotalMinutesPerYear = TotalMinutesPerYear;
	type EraMinutes = EraMinutes;
	type TotalReward = TotalReward;
	type PalletId = RewardPalletId;
}

pub type Extrinsic = sp_runtime::testing::TestXt<RuntimeCall, ()>;

impl<T> frame_system::offchain::SendTransactionTypes<T> for Test
where
	RuntimeCall: From<T>,
{
	type Extrinsic = Extrinsic;
	type OverarchingCall = RuntimeCall;
}

parameter_types! {
	pub MaxOnChainElectableTargets: u16 = 1250;
}

impl pallet_im_online::Config for Test {
	type AuthorityId = UintAuthorityId;
	type RuntimeEvent = RuntimeEvent;
	type ValidatorSet = Historical;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	type ReportUnresponsiveness = Offences;
	type UnsignedPriority = ConstU64<{ 1 << 20 }>;
	type WeightInfo = ();
	type MaxKeys = ConstU32<10_000>;
	type MaxPeerInHeartbeats = ConstU32<10_000>;
	type DataProvider = Staking;
	type TargetsBound = MaxOnChainElectableTargets;
}

impl pallet_esg::Config for Test {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type MaxFileSize = ConstU32<1024000>;
}

impl pallet_offences::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = Staking;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
