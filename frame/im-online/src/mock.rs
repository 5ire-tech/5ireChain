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

#![cfg(test)]

use frame_support::{
	derive_impl, parameter_types,
	traits::{ConstU32, ConstU64, ConstU128,Imbalance, OnUnbalanced},
	weights::Weight,
};
use frame_election_provider_support::{{bounds::ElectionBounds,bounds::ElectionBoundsBuilder},onchain,SequentialPhragmen};
use pallet_session::historical as pallet_session_historical;
use pallet_staking::{
	RewardDestination, Rewards, ValidatorPrefs,
};
use sp_core::H256;
use sp_runtime::{
	testing::{TestXt, UintAuthorityId},
	traits::{BlakeTwo256, ConvertInto, IdentityLookup},
	BuildStorage, Permill,DispatchError,Perbill
};
use sp_staking::{
	offence::{OffenceError, ReportOffence},
	SessionIndex,EraIndex
};

use crate as imonline;
use crate::Config;
type AccountId = u64;
type DummyValidatorId = u64;

type Block = frame_system::mocking::MockBlock<Runtime>;

frame_support::construct_runtime!(
	pub enum Runtime {
		System: frame_system,
		Balances:pallet_balances,
		Session: pallet_session,
		ImOnline: imonline,
		Historical: pallet_session_historical,
		Staking: pallet_staking,
		EsgScore: pallet_esg,
		Timestamp: pallet_timestamp
	}
);

parameter_types! {
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
	let t = frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();
	let mut result: sp_io::TestExternalities = t.into();
	// Set the default keys, otherwise session will discard the validator.
	result.execute_with(|| {
		for i in 1..=n {
			System::inc_providers(&i);
			// i'm using controller id; i same as that of stash id; i
			Staking::bond(
				RuntimeOrigin::signed(i),
				(100 + (100 * i)) as u128,
				RewardDestination::Staked
			)
			.unwrap();
			Staking::validate(RuntimeOrigin::signed(i), ValidatorPrefs::default()).unwrap();
			Session::set_keys(RuntimeOrigin::signed(i), (i).into(), vec![]).unwrap();
		}
	});
	result
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type Nonce = u64;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type AccountData = pallet_balances::AccountData<u128>;
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub const Period: u64 = 1;
	pub const Offset: u64 = 0;
	pub MaxOnChainElectableTargets: u16 = 1250;
	pub static SessionsPerEra: SessionIndex = 4;
	pub static SlashDeferDuration: EraIndex = 0;
	pub const BondingDuration: EraIndex = 3;
	pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(40);
	pub static RewardOnUnbalanceWasCalled: bool = false;
	pub static RewardRemainderUnbalanced: u128 = 0;
	pub static ElectionsBounds: ElectionBounds = ElectionBoundsBuilder::default().build();
}

impl pallet_session::Config for Runtime {
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type SessionManager =
		pallet_session::historical::NoteHistoricalRoot<Runtime, TestSessionManager>;
	type SessionHandler = (ImOnline,);
	type ValidatorId = u64;
	type ValidatorIdOf = ConvertInto;
	type Keys = UintAuthorityId;
	type RuntimeEvent = RuntimeEvent;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	type WeightInfo = ();
	type AllSessionHandler = (ImOnline,);
	type TargetsBound = MaxOnChainElectableTargets;
	type DataProvider = Staking;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = RuntimeFreezeReason;
}

impl pallet_timestamp::Config for Runtime {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

pub struct RewardRemainderMock;
impl OnUnbalanced<pallet_staking::NegativeImbalanceOf<Runtime>> for RewardRemainderMock {
	fn on_nonzero_unbalanced(amount: pallet_staking::NegativeImbalanceOf<Runtime>) {
		RewardRemainderUnbalanced::mutate(|v| {
			*v += amount.peek();
		});
		drop(amount);
	}
}

pub struct MockReward {}
impl OnUnbalanced<pallet_staking::PositiveImbalanceOf<Runtime>> for MockReward {
	fn on_unbalanced(_: pallet_staking::PositiveImbalanceOf<Runtime>) {
		RewardOnUnbalanceWasCalled::set(true);
	}
}

pub struct TestReward;
impl Rewards<AccountId> for TestReward {
	fn payout_validators() -> Vec<AccountId> {
		vec![]
	}
	fn claim_rewards(_: AccountId) -> Result<(), DispatchError> {
		Ok(())
	}
	fn calculate_reward() -> sp_runtime::DispatchResult {
		Ok(())
	}
}

pub struct ImOnlineSession;
impl pallet_staking::SessionInterface<AccountId> for ImOnlineSession {
	fn disable_validator(_validator_index: u32) -> bool {
		true
	}

	fn validators() -> Vec<AccountId> {
		Validators::get().unwrap()
	}

	fn prune_historical_up_to(_up_to: SessionIndex) {}
}


pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
	type System = Runtime;
	type Solver = SequentialPhragmen<DummyValidatorId, Perbill>;
	type DataProvider = Staking;
	type WeightInfo = ();
	type MaxWinners = ConstU32<100>;
	type Bounds = ElectionsBounds;
}

impl pallet_esg::Config for Runtime {
	type WeightInfo = ();
	type MaxFileSize = ConstU32<102400>;
	type RuntimeEvent = RuntimeEvent;
	type MaxNumOfSudoOracles = ConstU32<5>;
	type MaxNumOfNonSudoOracles = ConstU32<5>;
}

impl pallet_staking::Config for Runtime {
	type Currency = Balances;
	type RewardDistribution = TestReward;
	type CurrencyBalance = <Self as pallet_balances::Config>::Balance;
	type UnixTime = Timestamp;
	type CurrencyToVote = ();
	type RewardRemainder = RewardRemainderMock;
	type RuntimeEvent = RuntimeEvent;
	type Slash = ();
	type Reward = MockReward;
	type SessionsPerEra = SessionsPerEra;
	type SlashDeferDuration = SlashDeferDuration;
	type BondingDuration = BondingDuration;
	type AdminOrigin = frame_system::EnsureRoot<u64>;
	type SessionInterface = ImOnlineSession;
	type MaxExposurePageSize = ConstU32<64>;
	type MaxControllersInDeprecationBatch = ConstU32<100>;
	type EraPayout = ();
	type NextNewSession = Session;
	type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
	type ElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type GenesisElectionProvider = Self::ElectionProvider;
	type VoterList = pallet_staking::UseNominatorsAndValidatorsMap<Self>;
	type TargetList = pallet_staking::UseValidatorsMap<Self>;
	type MaxUnlockingChunks = ConstU32<32>;
	type NominationsQuota = pallet_staking::FixedNominationsQuota<16>;
	type HistoryDepth = ConstU32<84>;
	type EventListeners = ();
	type BenchmarkingConfig = pallet_staking::TestBenchmarkingConfig;
	type WeightInfo = ();
	type ESG = EsgScore;
	type Reliability = ImOnline;
	type Validators = Historical;
	type ValidatorId = pallet_staking::StashOf<Runtime>;
}

impl pallet_session::historical::Config for Runtime {
	type FullIdentification = u64;
	type FullIdentificationOf = ConvertInto;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = ();
	type EventHandler = ImOnline;
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

impl Config for Runtime {
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

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Runtime
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
