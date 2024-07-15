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

use super::*;
use crate as pallet_transaction_payment;
use pallet_contracts::{
	Frame,
	Schedule,
	chain_extension::{
		Result as ExtensionResult,
		ChainExtension,
		Environment,
		RetVal,
		InitState,
		Ext,
		RegisteredChainExtension,
		ReturnFlags,
	},
	DefaultAddressGenerator,
};
use sp_core::H256;
use frame_support::traits::Contains;
use sp_runtime::traits::{ BlakeTwo256, IdentityLookup };
use frame_support::{
	dispatch::DispatchClass,
	parameter_types,
	traits::{ ConstU32, ConstU64, Imbalance, OnUnbalanced },
	weights::{ Weight, WeightToFee as WeightToFeeT },
};
use frame_system as system;
use pallet_balances::Call as BalancesCall;

type Block = frame_system::mocking::MockBlock<Runtime>;

frame_support::construct_runtime!(
	pub struct Runtime
	{
		System: system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>},
		Contract:pallet_contracts,
		Timestamp:pallet_timestamp,
		Randomness: pallet_insecure_randomness_collective_flip,
	}
);

pub(crate) const CALL: &<Runtime as frame_system::Config>::RuntimeCall = &RuntimeCall::Balances(
	BalancesCall::transfer_allow_death { dest: 2, value: 69 }
);

parameter_types! {
	pub(crate) static ExtrinsicBaseWeight: Weight = Weight::zero();
}

pub struct BlockWeights;
impl Get<frame_system::limits::BlockWeights> for BlockWeights {
	fn get() -> frame_system::limits::BlockWeights {
		frame_system::limits::BlockWeights
			::builder()
			.base_block(Weight::zero())
			.for_class(DispatchClass::all(), |weights| {
				weights.base_extrinsic = ExtrinsicBaseWeight::get().into();
			})
			.for_class(DispatchClass::non_mandatory(), |weights| {
				weights.max_total = Weight::from_parts(1024, u64::MAX).into();
			})
			.build_or_panic()
	}
}

parameter_types! {
	pub static WeightToFee: u64 = 1;
	pub static TransactionByteFee: u64 = 1;
	pub static OperationalFeeMultiplier: u8 = 5;
}

impl frame_system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = BlockWeights;
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
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Runtime {
	type Balance = u64;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = RuntimeHoldReason;
	type MaxHolds = ConstU32<1>;
}

impl WeightToFeeT for WeightToFee {
	type Balance = u64;

	fn weight_to_fee(weight: &Weight) -> Self::Balance {
		Self::Balance::saturated_from(weight.ref_time()).saturating_mul(
			WEIGHT_TO_FEE.with(|v| *v.borrow())
		)
	}
}

impl WeightToFeeT for TransactionByteFee {
	type Balance = u64;

	fn weight_to_fee(weight: &Weight) -> Self::Balance {
		Self::Balance::saturated_from(weight.ref_time()).saturating_mul(
			TRANSACTION_BYTE_FEE.with(|v| *v.borrow())
		)
	}
}

parameter_types! {
	static TestExtensionTestValue: TestExtension = Default::default();
}

#[derive(Clone)]
pub struct TestExtension {
	enabled: bool,
	last_seen_buffer: Vec<u8>,
	last_seen_inputs: (u32, u32, u32, u32),
}

#[derive(Default)]
pub struct RevertingExtension;

#[derive(Default)]
pub struct DisabledExtension;

#[derive(Default)]
pub struct TempStorageExtension {
	storage: u32,
}

impl Default for TestExtension {
	fn default() -> Self {
		Self { enabled: true, last_seen_buffer: vec![], last_seen_inputs: (0, 0, 0, 0) }
	}
}

impl ChainExtension<Runtime> for TestExtension {
	fn call<E>(&mut self, env: Environment<E, InitState>) -> ExtensionResult<RetVal>
		where E: Ext<T = Runtime>
	{
		let func_id = env.func_id();
		let id = (env.ext_id() as u32) | (func_id as u32);
		match func_id {
			0 => {
				let mut env = env.buf_in_buf_out();
				let input = env.read(8)?;
				env.write(&input, false, None)?;
				TestExtensionTestValue::mutate(|e| {
					e.last_seen_buffer = input;
				});
				Ok(RetVal::Converging(id))
			}
			1 => {
				let env = env.only_in();
				TestExtensionTestValue::mutate(|e| {
					e.last_seen_inputs = (env.val0(), env.val1(), env.val2(), env.val3());
				});
				Ok(RetVal::Converging(id))
			}
			2 => {
				let mut env = env.buf_in_buf_out();
				let weight = Weight::from_parts(0, 0);
				env.charge_weight(weight)?;
				Ok(RetVal::Converging(id))
			}
			3 => Ok(RetVal::Diverging { flags: ReturnFlags::REVERT, data: vec![42, 99] }),
			_ => {
				panic!("Passed unknown id to test chain extension: {}", func_id);
			}
		}
	}

	fn enabled() -> bool {
		TestExtensionTestValue::get().enabled
	}
}

impl ChainExtension<Runtime> for RevertingExtension {
	fn call<E>(&mut self, _env: Environment<E, InitState>) -> ExtensionResult<RetVal>
		where E: Ext<T = Runtime>
	{
		Ok(RetVal::Diverging { flags: ReturnFlags::REVERT, data: vec![0x4b, 0x1d] })
	}

	fn enabled() -> bool {
		TestExtensionTestValue::get().enabled
	}
}

impl RegisteredChainExtension<Runtime> for RevertingExtension {
	const ID: u16 = 1;
}

parameter_types! {
	pub(crate) static TipUnbalancedAmount: u64 = 0;
	pub(crate) static FeeUnbalancedAmount: u64 = 0;
}

pub struct DealWithFees;
impl OnUnbalanced<pallet_balances::NegativeImbalance<Runtime>> for DealWithFees {
	fn on_unbalanceds<B>(
		mut fees_then_tips: impl Iterator<Item = pallet_balances::NegativeImbalance<Runtime>>
	) {
		if let Some(fees) = fees_then_tips.next() {
			FeeUnbalancedAmount::mutate(|a| {
				*a += fees.peek();
			});
			if let Some(tips) = fees_then_tips.next() {
				TipUnbalancedAmount::mutate(|a| {
					*a += tips.peek();
				});
			}
		}
	}
}

parameter_types! {
	pub MySchedule: Schedule<Runtime> = {
		let schedule = <Schedule<Runtime>>::default();
		schedule
	};
	pub static DepositPerByte: BalanceOf<Runtime> = 1;
	pub const DepositPerItem: BalanceOf<Runtime> = 2;
	pub static MaxDelegateDependencies: u32 = 32;
	pub static CodeHashLockupDepositPercent: Perbill = Perbill::from_percent(0);
	// We need this one set high enough for running benchmarks.
	pub static DefaultDepositLimit: BalanceOf<Runtime> = 10_000_000;
}

/// A filter whose filter function can be swapped at runtime.
pub struct TestFilter;

#[derive(Clone)]
pub struct Filters {
	filter: fn(&RuntimeCall) -> bool,
}

impl Default for Filters {
	fn default() -> Self {
		Filters { filter: |_| true }
	}
}

parameter_types! {
	static CallFilter: Filters = Default::default();
}

impl TestFilter {
	pub fn set_filter(filter: fn(&RuntimeCall) -> bool) {
		CallFilter::mutate(|fltr| {
			fltr.filter = filter;
		});
	}
}

impl Contains<RuntimeCall> for TestFilter {
	fn contains(call: &RuntimeCall) -> bool {
		(CallFilter::get().filter)(call)
	}
}

impl RegisteredChainExtension<Runtime> for TempStorageExtension {
	const ID: u16 = 3;
}

impl ChainExtension<Runtime> for TempStorageExtension {
	fn call<E>(&mut self, env: Environment<E, InitState>) -> ExtensionResult<RetVal>
		where E: Ext<T = Runtime>
	{
		let func_id = env.func_id();
		match func_id {
			0 => {
				self.storage = 42;
			}
			1 => assert_eq!(self.storage, 42, "Storage is preserved inside the same call."),
			2 => {
				assert_eq!(self.storage, 0, "Storage is different for different calls.");
				self.storage = 99;
			}
			3 => assert_eq!(self.storage, 99, "Storage is preserved inside the same call."),
			_ => {
				panic!("Passed unknown id to test chain extension: {}", func_id);
			}
		}
		Ok(RetVal::Converging(0))
	}

	fn enabled() -> bool {
		TestExtensionTestValue::get().enabled
	}
}

impl RegisteredChainExtension<Runtime> for TestExtension {
	const ID: u16 = 0;
}

impl RegisteredChainExtension<Runtime> for DisabledExtension {
	const ID: u16 = 2;
}

impl ChainExtension<Runtime> for DisabledExtension {
	fn call<E>(&mut self, _env: Environment<E, InitState>) -> ExtensionResult<RetVal>
		where E: Ext<T = Runtime>
	{
		panic!("Disabled chain extensions are never called")
	}

	fn enabled() -> bool {
		false
	}
}

impl Convert<Weight, BalanceOf<Self>> for Runtime {
	fn convert(w: Weight) -> BalanceOf<Self> {
		w.ref_time()
	}
}
impl pallet_insecure_randomness_collective_flip::Config for Runtime {}

impl pallet_timestamp::Config for Runtime {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<1>;
	type WeightInfo = ();
}

impl pallet_contracts::Config for Runtime {
	type Time = Timestamp;
	type Randomness = Randomness;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type ContractRuntimeCall = RuntimeCall;
	type CallFilter = TestFilter;
	type CallStack = [Frame<Self>; 5];
	type WeightPrice = Self;
	type WeightInfo = ();
	type ChainExtension = (TestExtension,DisabledExtension,RevertingExtension,TempStorageExtension);
	type Schedule = MySchedule;
	type DepositPerByte = DepositPerByte;
	type DepositPerItem = DepositPerItem;
	type DefaultDepositLimit = DefaultDepositLimit;
	type AddressGenerator = DefaultAddressGenerator;
	type MaxCodeLen = ConstU32<{ 123 * 1024 }>;
	type MaxStorageKeyLen = ConstU32<128>;
	type UnsafeUnstableInterface = ();
	type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
	type RuntimeHoldReason = RuntimeHoldReason;
	type Migrations = ();
	type CodeHashLockupDepositPercent = CodeHashLockupDepositPercent;
	type MaxDelegateDependencies = MaxDelegateDependencies;
	type Debug = ();
	type Environment = ();
}

impl Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = CurrencyAdapter<Balances, DealWithFees>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type WeightToFee = WeightToFee;
	type LengthToFee = TransactionByteFee;
	type FeeMultiplierUpdate = ();
}
