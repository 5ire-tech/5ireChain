use precompile_utils::{prelude::*, testing::PrecompileTesterExt, EvmResult};
use sp_core::H160;

pub struct PrecompileSet;

#[precompile_utils_macro::precompile]
#[precompile::precompile_set]
impl PrecompileSet {
	#[precompile::discriminant]
	fn discriminant(_: H160) -> Option<()> {
		Some(())
	}

	#[precompile::public("default()")]
	fn default(_: (), _: &mut impl PrecompileHandle) -> EvmResult {
		Ok(())
	}

	#[precompile::public("view()")]
	#[precompile::view]
	fn view(_: (), _: &mut impl PrecompileHandle) -> EvmResult {
		Ok(())
	}

	#[precompile::public("payable()")]
	#[precompile::payable]
	fn payable(_: (), _: &mut impl PrecompileHandle) -> EvmResult {
		Ok(())
	}
}

fn main() {
	PrecompileSet
		.prepare_test([0u8; 20], [0u8; 20], PrecompileSetCall::default {})
		.with_value(1)
		.execute_reverts(|output| output == b"Function is not payable");

	PrecompileSet
		.prepare_test([0u8; 20], [0u8; 20], PrecompileSetCall::default {})
		.with_static_call(true)
		.execute_reverts(|output| output == b"Can't call non-static function in static context");

	PrecompileSet
		.prepare_test([0u8; 20], [0u8; 20], PrecompileSetCall::view {})
		.with_value(1)
		.execute_reverts(|output| output == b"Function is not payable");

	PrecompileSet
		.prepare_test([0u8; 20], [0u8; 20], PrecompileSetCall::view {})
		.with_static_call(true)
		.execute_returns(());

	PrecompileSet
		.prepare_test([0u8; 20], [0u8; 20], PrecompileSetCall::payable {})
		.with_value(1)
		.execute_returns(());

	PrecompileSet
		.prepare_test([0u8; 20], [0u8; 20], PrecompileSetCall::payable {})
		.with_static_call(true)
		.execute_reverts(|output| output == b"Can't call non-static function in static context");
}
