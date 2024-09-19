#![cfg(test)]
use crate::{self as pallet_reward};
use frame_election_provider_support::{
	bounds::{ElectionBounds, ElectionBoundsBuilder},
	onchain, SequentialPhragmen,
};
use frame_support::{
	pallet_prelude::ConstU32,
	parameter_types,
	traits::{ConstU128, ConstU16, ConstU64, Hooks, OneSessionHandler},
};
use pallet_session::historical as pallet_session_historical;
use sp_runtime::{
	testing::{UintAuthorityId, H256},
	traits::{BlakeTwo256, IdentityLookup, Zero},
	BuildStorage,
};
use sp_staking::{SessionIndex, StakerStatus};

pub type RewardBalance = pallet_balances::Pallet<Test>;

use frame_support::PalletId;
use sp_runtime::Perbill;
use sp_staking::EraIndex;

type Block = frame_system::mocking::MockBlock<Test>;
pub const INIT_TIMESTAMP: u64 = 30_000;
pub const BLOCK_TIME: u64 = 1000;

type AccountId = u64;
type Nonce = u32;
type Balance = u64;
type BlockNumber = u64;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Timestamp: pallet_timestamp,
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

sp_runtime::impl_opaque_keys! {
	pub struct SessionKeys {
		pub other: OtherSessionHandler,
	}
}

pub struct OtherSessionHandler;
impl OneSessionHandler<AccountId> for OtherSessionHandler {
	type Key = UintAuthorityId;

	fn on_genesis_session<'a, I: 'a>(_: I)
	where
		I: Iterator<Item = (&'a AccountId, Self::Key)>,
		AccountId: 'a,
	{
	}

	fn on_new_session<'a, I: 'a>(_: bool, _: I, _: I)
	where
		I: Iterator<Item = (&'a AccountId, Self::Key)>,
		AccountId: 'a,
	{
	}

	fn on_disabled(_validator_index: u32) {}
}

impl sp_runtime::BoundToRuntimeAppPublic for OtherSessionHandler {
	type Public = UintAuthorityId;
}

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
	type Keys = SessionKeys;
	type ValidatorId = AccountId;
	type ValidatorIdOf = pallet_staking::StashOf<Test>;
	type RuntimeEvent = RuntimeEvent;
	type SessionHandler = (OtherSessionHandler,);
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
}

impl pallet_balances::Config for Test {
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<10>;
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
	type ValidatorSet = Historical;
	type Validators = Historical;
	type ValidatorId = pallet_staking::StashOf<Test>;
	type Precision = DecimalPrecision;
	type TotalMinutesPerYear = TotalMinutesPerYear;
	type EraMinutes = EraMinutes;
	type TotalReward = TotalReward;
	type PalletId = RewardPalletId;
	type WeightInfo = ();
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
	type MaxNumOfSudoOracles =  ConstU32<5>;
	type MaxNumOfNonSudoOracles = ConstU32<5>;
}

impl pallet_offences::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = Staking;
}

pub struct ExtBuilder {
	validator_count: u32,
	minimum_validator_count: u32,
	invulnerables: Vec<AccountId>,
	balance_factor: Balance,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			validator_count: 2,
			minimum_validator_count: 0,
			invulnerables: vec![],
			balance_factor: 1,
		}
	}
}

impl ExtBuilder {
	fn build(self) -> sp_io::TestExternalities {
		let mut storage = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

		pallet_balances::GenesisConfig::<Test> {
			balances: vec![
				// controllers (still used in some tests. Soon to be deprecated).
				(10, (self.balance_factor * 50).into()),
				(20, (self.balance_factor * 50).into()),
				(30, (self.balance_factor * 50).into()),
				(40, (self.balance_factor * 50).into()),
				// stashes
				(11, (self.balance_factor * 1000).into()),
				(21, (self.balance_factor * 1000).into()),
				(31, (self.balance_factor * 500).into()),
				(41, (self.balance_factor * 1000).into()),
				// normal user
				(1, (self.balance_factor * 1000).into()),
			],
		}
		.assimilate_storage(&mut storage)
		.unwrap();

		let stakers = vec![
			// (stash, ctrl, stake, status)
			// these two will be elected in the default test where we elect 2.
			(11, 11, 1000, StakerStatus::<AccountId>::Validator),
			(21, 21, 1000, StakerStatus::<AccountId>::Validator),
			// a loser validator
			(31, 31, 500, StakerStatus::<AccountId>::Validator),
			// an idle validator
			(41, 41, 1000, StakerStatus::<AccountId>::Idle),
		];

		let _ = pallet_staking::GenesisConfig::<Test> {
			stakers: stakers.clone(),
			..Default::default()
		};

		let _ = pallet_staking::GenesisConfig::<Test> {
			stakers: stakers.clone(),
			validator_count: self.validator_count,
			minimum_validator_count: self.minimum_validator_count,
			invulnerables: self.invulnerables,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}
		.assimilate_storage(&mut storage);

		let _ = pallet_session::GenesisConfig::<Test> {
			keys: stakers
				.into_iter()
				.map(|(id, ..)| (id, id, SessionKeys { other: id.into() }))
				.collect(),
		}
		.assimilate_storage(&mut storage);

		storage.into()
	}

	pub fn build_and_execute(self, test: impl FnOnce() -> ()) {
		let mut ext = self.build();
		ext.execute_with(test);
	}
}

/// Progresses from the current block number (whatever that may be) to the `P * session_index + 1`.
pub(crate) fn start_session(session_index: SessionIndex) {
	let end: u64 = if Offset::get().is_zero() {
		(session_index as u64) * Period::get()
	} else {
		Offset::get() + (session_index.saturating_sub(1) as u64) * Period::get()
	};
	run_to_block(end);
	// session must have progressed properly.
	assert_eq!(
		Session::current_index(),
		session_index,
		"current session index = {}, expected = {}",
		Session::current_index(),
		session_index,
	);
}

/// Progress to the given block, triggering session and era changes as we progress.
///
/// This will finalize the previous block, initialize up to the given block, essentially simulating
/// a block import/propose process where we first initialize the block, then execute some stuff (not
/// in the function), and then finalize the block.
pub(crate) fn run_to_block(n: BlockNumber) {
	Staking::on_finalize(System::block_number());
	for b in (System::block_number() + 1)..=n {
		System::set_block_number(b);
		Session::on_initialize(b);
		<Staking as Hooks<u64>>::on_initialize(b);
		Timestamp::set_timestamp(System::block_number() * BLOCK_TIME + INIT_TIMESTAMP);
		if b != n {
			Staking::on_finalize(System::block_number());
		}
	}
}

pub(crate) fn active_era() -> EraIndex {
	Staking::active_era().unwrap().index
}