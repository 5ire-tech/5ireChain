// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Test utilities
#![allow(dead_code)]
#![allow(unused_imports)]

#![cfg(test)]

use sp_std::collections::btree_map::BTreeMap;

use pallet_session::historical as pallet_session_historical;
use sp_core::H256;
use sp_runtime::{
	curve::PiecewiseLinear,
	testing::{TestXt, UintAuthorityId},
	traits::{BlakeTwo256, ConvertInto, IdentityLookup, Zero},
	BuildStorage, Permill,
	Perbill,
};
use sp_staking::{
	offence::{OffenceError, ReportOffence},
	SessionIndex,
};

use frame_support::{
	assert_ok,
	parameter_types,
	traits::{
		ConstU32, ConstU64,
		Currency, FindAuthor, GenesisBuild, Get, Hooks, Imbalance,
		OnUnbalanced, OneSessionHandler,
	},
	weights::{Weight, constants::RocksDbWeight}, Parameter,
};
use frame_election_provider_support::{onchain, SequentialPhragmen, VoteWeight};
use pallet_staking::{BalanceOf, Exposure, ExposureOf, RewardDestination, ValidatorPrefs};

type AccountId = u64;
type AccountIndex = u64;
type BlockNumber = u64;
type Balance = u128;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

const THRESHOLDS: [sp_npos_elections::VoteWeight; 9] =
	[10, 20, 30, 40, 50, 60, 1_000, 2_000, 10_000];

pub const INIT_TIMESTAMP: u64 = 30_000;
pub const BLOCK_TIME: u64 = 1000;

use crate as imonline;
use crate::Config;

frame_support::construct_runtime!(
	pub struct Test
	{
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		EsgScore: pallet_esg::{Pallet, Call, Storage, Event<T>},
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		ImOnline: imonline::{Pallet, Call, Storage, Config<T>, Event<T>},
		Historical: pallet_session_historical::{Pallet},
		Staking: pallet_staking::{Pallet, Call, Config<T>, Storage, Event<T>},
		VoterBagsList: pallet_bags_list::<Instance1>::{Pallet, Call, Storage, Event<T>},
	}
);

pallet_staking_reward_curve::build! {
	const I_NPOS: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000,
		max_inflation: 0_100_000,
		ideal_stake: 0_500_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}

parameter_types! {
	pub MaxOnChainElectableTargets: u16 = 1250;
	pub BlockWeights: frame_system::limits::BlockWeights =
	frame_system::limits::BlockWeights::simple_max(frame_support::weights::Weight::from_ref_time(1024));

	// pub const Period: u64 = 1;
	// pub const Offset: u64 = 0;
	pub const BondingDuration: EraIndex = 3;
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &I_NPOS;
	pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(75);

	pub static MaxWinners: u32 = 100;
	pub static Period: BlockNumber = 1;
	pub static Offset: BlockNumber = 0;
	pub static HistoryDepth: u32 = 80;
	pub static MaxNominations: u32 = 16;
	pub static MaxUnlockingChunks: u32 = 32;
	pub static ExistentialDeposit: Balance = 1;
	pub static SlashDeferDuration: EraIndex = 0;
	pub static SessionsPerEra: SessionIndex = 4;
	pub static RewardRemainderUnbalanced: u128 = 0;
	pub static RewardOnUnbalanceWasCalled: bool = false;
	pub static Offences: Vec<(Vec<u64>, Offence)> = vec![];
	pub static MockAverageSessionLength: Option<u64> = None;
	pub static MockCurrentSessionProgress: Option<Option<Permill>> = None;
	pub static BagThresholds: &'static [sp_npos_elections::VoteWeight] = &THRESHOLDS;
	pub static LedgerSlashPerEra: (BalanceOf<Test>, BTreeMap<EraIndex, BalanceOf<Test>>) = (Zero::zero(), BTreeMap::new());

	pub static Validators: Option<Vec<u64>> = Some(vec![
		1,
		2,
		3,
	]);
}

pub struct TestSessionManager;
impl pallet_session::SessionManager<u64> for TestSessionManager {
	fn new_session(_new_index: SessionIndex) -> Option<Vec<u64>> {
		Validators::mutate(|l| l.take())
	}
	fn end_session(_: SessionIndex) {}
	fn start_session(_: SessionIndex) {}
}

impl pallet_session::historical::SessionManager<u64, u64> for TestSessionManager {
	fn new_session(_new_index: SessionIndex) -> Option<Vec<(u64, u64)>> {
		Validators::mutate(|l| {
			l.take().map(|validators| validators.iter().map(|v| (*v, *v)).collect())
		})
	}
	fn end_session(_: SessionIndex) {}
	fn start_session(_: SessionIndex) {}
}

/// An extrinsic type used for tests.
pub type Extrinsic = TestXt<RuntimeCall, ()>;
type IdentificationTuple = (u64, u64);
type Offence = crate::UnresponsivenessOffence<IdentificationTuple>;

parameter_types! {
	pub static Offences: Vec<(Vec<u64>, Offence)> = vec![];
}

/// A mock offence report handler.
pub struct OffenceHandler;
impl ReportOffence<u64, IdentificationTuple, Offence> for OffenceHandler {
	fn report_offence(reporters: Vec<u64>, offence: Offence) -> Result<(), OffenceError> {
		Offences::mutate(|l| l.push((reporters, offence)));
		Ok(())
	}

	fn is_known_offence(_offenders: &[IdentificationTuple], _time_slot: &SessionIndex) -> bool {
		false
	}
}

pub fn new_test_ext(n: u64) -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let mut result: sp_io::TestExternalities = t.into();
	// Set the default keys, otherwise session will discard the validator.
	result.execute_with(|| {
		for i in 1..=n {
			System::inc_providers(&i);
			// i'm using controller id; i same as that of stash id; i
			Staking::bond(RuntimeOrigin::signed(i), i, 1500, RewardDestination::Controller).unwrap();
			Staking::validate(RuntimeOrigin::signed(i), ValidatorPrefs::default()).unwrap();
			Session::set_keys(RuntimeOrigin::signed(i), (i).into(), vec![]).unwrap();		}
	});
	result
}

pub struct Author11;
impl FindAuthor<AccountId> for Author11 {
	fn find_author<'a, I>(_digests: I) -> Option<AccountId>
		where
			I: 'a + IntoIterator<Item = (frame_support::ConsensusEngineId, &'a [u8])>,
	{
		Some(11)
	}
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = RocksDbWeight;
	type RuntimeOrigin = RuntimeOrigin;
	type Nonce = u64;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId	;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = frame_support::traits::ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const Period: u64 = 1;
	pub const Offset: u64 = 0;
}

impl pallet_session::Config for Test {
	type WeightInfo = ();
	type DataProvider = Staking;
	type Keys = UintAuthorityId;

	type ValidatorId = u64;
	type ValidatorIdOf = ConvertInto;

	type RuntimeEvent = RuntimeEvent;
	type SessionHandler = (ImOnline,);
	type AllSessionHandler = (ImOnline,);
	type TargetsBound = MaxOnChainElectableTargets;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Test, TestSessionManager>;

}

impl pallet_session::historical::Config for Runtime {

	type FullIdentification = u64;
	type FullIdentificationOf = ConvertInto;

}

impl pallet_balances::Config for Test {
	type MaxLocks = frame_support::traits::ConstU32<1024>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}
impl pallet_esg::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type MaxFileSize = ConstU32<102400>;
}

type VoterBagsListInstance = pallet_bags_list::Instance1;
impl pallet_bags_list::Config<VoterBagsListInstance> for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	// Staking is the source of truth for voter bags list, since they are not kept up to date.
	type ScoreProvider = Staking;
	type BagThresholds = BagThresholds;
	type Score = VoteWeight;
}

pub struct ImOnlineSession;
impl pallet_staking::SessionInterface<AccountId> for ImOnlineSession {
	fn disable_validator(validator_index: u32) -> bool {
		true
	}

	fn validators() -> Vec<AccountId> {
		Validators::get().unwrap()
	}


	fn prune_historical_up_to(up_to: SessionIndex) {}
}

impl pallet_staking::Config for Test {
	type MaxNominations = MaxNominations;
	type Currency = Balances;
	type CurrencyBalance = <Self as pallet_balances::Config>::Balance;
	type UnixTime = Timestamp;
	type CurrencyToVote = frame_support::traits::SaturatingCurrencyToVote;
	type RewardRemainder = RewardRemainderMock;
	type RuntimeEvent = RuntimeEvent;
	type Slash = ();
	type Reward = MockReward;
	type SessionsPerEra = SessionsPerEra;
	type SlashDeferDuration = SlashDeferDuration;
	type SlashCancelOrigin = frame_system::EnsureRoot<AccountId>;
	type BondingDuration = BondingDuration;
	type SessionInterface = ImOnlineSession;
	type EraPayout = pallet_staking::ConvertCurve<RewardCurve>;
	type NextNewSession = Session;
	type MaxNominatorRewardedPerValidator = ConstU32<64>;
	type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
	type ElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type GenesisElectionProvider = Self::ElectionProvider;
	type VoterList = VoterBagsList;
	type TargetList = pallet_staking::UseValidatorsMap<Self>;
	type MaxUnlockingChunks = MaxUnlockingChunks;
	type HistoryDepth = HistoryDepth;
	type OnStakerSlash = OnStakerSlashMock<Test>;
	type BenchmarkingConfig = pallet_staking::TestBenchmarkingConfig;
	type WeightInfo = ();
	type ESG = EsgScore;
	type Reliability = ImOnline;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = ();
	type EventHandler = ImOnline;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
	type System = Test;
	type Solver = SequentialPhragmen<AccountId, Perbill>;
	type DataProvider = Staking;
	type WeightInfo = ();
	type MaxWinners = MaxWinners;
	type VotersBound = ConstU32<{ u32::MAX }>;
	type TargetsBound = ConstU32<{ u32::MAX }>;
}

pub struct RewardRemainderMock;

impl OnUnbalanced<pallet_staking::NegativeImbalanceOf<Test>> for RewardRemainderMock {
	fn on_nonzero_unbalanced(amount: pallet_staking::NegativeImbalanceOf<Test>) {
		RewardRemainderUnbalanced::mutate(|v| {
			*v += amount.peek();
		});
		drop(amount);
	}
}

pub struct MockReward {}
impl OnUnbalanced<pallet_staking::PositiveImbalanceOf<Test>> for MockReward {
	fn on_unbalanced(_: pallet_staking::PositiveImbalanceOf<Test>) {
		RewardOnUnbalanceWasCalled::set(true);
	}
}

pub struct OnStakerSlashMock<T: Config>(core::marker::PhantomData<T>);
impl<T: Config> sp_staking::OnStakerSlash<AccountId, Balance> for OnStakerSlashMock<T> {
	fn on_slash(
		_pool_account: &AccountId,
		slashed_bonded: Balance,
		slashed_chunks: &BTreeMap<EraIndex, Balance>,
	) {
		LedgerSlashPerEra::set((slashed_bonded, slashed_chunks.clone()));
	}
}

parameter_types! {
	pub static MockCurrentSessionProgress: Option<Option<Permill>> = None;
}

parameter_types! {
	pub static MockAverageSessionLength: Option<u64> = None;
}

pub struct TestNextSessionRotation;

impl frame_support::traits::EstimateNextSessionRotation<u64> for TestNextSessionRotation {
	fn average_session_length() -> u64 {
		// take the mock result if any and return it
		let mock = MockAverageSessionLength::mutate(|p| p.take());

		mock.unwrap_or(pallet_session::PeriodicSessions::<Period, Offset>::average_session_length())
	}

	fn estimate_current_session_progress(now: u64) -> (Option<Permill>, Weight) {
		let (estimate, weight) =
			pallet_session::PeriodicSessions::<Period, Offset>::estimate_current_session_progress(
				now,
			);

		// take the mock result if any and return it
		let mock = MockCurrentSessionProgress::mutate(|p| p.take());

		(mock.unwrap_or(estimate), weight)
	}


	fn estimate_next_session_rotation(now: u64) -> (Option<u64>, Weight) {
		pallet_session::PeriodicSessions::<Period, Offset>::estimate_next_session_rotation(now)
	}
}

impl Config for Test {
	type AuthorityId = UintAuthorityId;
	type RuntimeEvent = RuntimeEvent;
	type ValidatorSet = Historical;
	type NextSessionRotation = TestNextSessionRotation;
	type ReportUnresponsiveness = OffenceHandler;
	type UnsignedPriority = ConstU64<{ 1 << 20 }>;
	type WeightInfo = ();
	type MaxKeys = ConstU32<10_000>;
	type MaxPeerInHeartbeats = ConstU32<10_000>;
	type DataProvider = Staking;
	type TargetsBound = MaxOnChainElectableTargets;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
	RuntimeCall: From<LocalCall>,
{
	type OverarchingCall = RuntimeCall;
	type Extrinsic = Extrinsic;
}

pub fn advance_session() {
	let now = System::block_number().max(1);
	System::set_block_number(now + 1);
	Session::rotate_session();
	let keys = Session::validators().into_iter().map(UintAuthorityId).collect();
	ImOnline::set_keys(keys);
	assert_eq!(Session::current_index(), (now / Period::get()) as u32);
}

pub fn print_all_events() {
	System::events()
		.iter()
		.for_each(|record|log::info!("#@! Event: {:?}", record.event));
}

pub fn auth_at_idx(i: u32) -> UintAuthorityId {
	let keys = ImOnline::all_keys();
	keys[i as usize].clone()
}
