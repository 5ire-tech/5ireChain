use crate::mock::{
	balance, Batch, ExtBuilder, MockPrecompileSet, PCall, PrecompilesValue, Revert, Runtime, RuntimeCall, RuntimeOrigin
};
use crate::{
	log_subcall_failed, log_subcall_succeeded, Mode, LOG_SUBCALL_FAILED, LOG_SUBCALL_SUCCEEDED,
};
use fp_evm::ExitError;
use frame_support::assert_ok;
use pallet_evm::Call as EvmCall;
use precompile_utils::solidity::revert::revert_as_bytes;
use precompile_utils::{evm::costs::call_cost, prelude::*, testing::*};
use sp_core::{H160, H256, U256};
use sp_runtime::DispatchError;
use sp_runtime::{traits::Dispatchable, DispatchErrorWithPostInfo, ModuleError};



fn evm_call(from: impl Into<H160>, input: Vec<u8>) -> EvmCall<Runtime> {
	EvmCall::call {
		source: from.into(),
		target: Batch.into(),
		input,
		value: U256::zero(),
		gas_limit: u64::max_value(),
		max_fee_per_gas: 0.into(),
		max_priority_fee_per_gas: Some(U256::zero()),
		nonce: None,
		access_list: Vec::new(),
	}
}


fn costs() -> (u64, u64) {
	let return_log_cost = log_subcall_failed(Batch, 0).compute_cost().unwrap();
	let call_cost =
		return_log_cost + call_cost(U256::one(), <Runtime as pallet_evm::Config>::config());
	(return_log_cost, call_cost)
}

fn precompiles() -> MockPrecompileSet<Runtime> {
	PrecompilesValue::get()
}

#[test]
fn selectors() {
	assert!(PCall::batch_some_selectors().contains(&0x79df4b9c));
	assert!(PCall::batch_some_until_failure_selectors().contains(&0xcf0491c7));
	assert!(PCall::batch_all_selectors().contains(&0x96e292b8));
	assert_eq!(
		LOG_SUBCALL_FAILED,
		hex_literal::hex!("dbc5d06f4f877f959b1ff12d2161cdd693fa8e442ee53f1790b2804b24881f05")
	);
	assert_eq!(
		LOG_SUBCALL_SUCCEEDED,
		hex_literal::hex!("bf855484633929c3d6688eb3caf8eff910fb4bef030a8d7dbc9390d26759714d")
	);
}


#[test]
fn modifiers() {
	ExtBuilder::default()
		.with_balances(vec![(Alice.into(), 1000)])
		.build()
		.execute_with(|| {
			let mut tester = PrecompilesModifierTester::new(precompiles(), Alice, Batch);

			tester.test_default_modifier(PCall::batch_some_selectors());
			tester.test_default_modifier(PCall::batch_some_until_failure_selectors());
			tester.test_default_modifier(PCall::batch_all_selectors());
		});
}

