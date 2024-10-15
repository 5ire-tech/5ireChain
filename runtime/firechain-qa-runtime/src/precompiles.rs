use pallet_evm::{
	IsPrecompileResult, Precompile, PrecompileHandle, PrecompileResult, PrecompileSet,
};
use sp_core::H160;
use sp_std::marker::PhantomData;

use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use pallet_evm_precompile_batch::BatchPrecompile;
use pallet_evm_precompile_registry::RegistryPrecompile;
pub struct FrontierPrecompiles<R>(PhantomData<R>);

#[allow(clippy::new_without_default)]
impl<R> FrontierPrecompiles<R>
where
	R: pallet_evm::Config,
{
	pub fn new() -> Self {
		Self(Default::default())
	}
	pub fn used_addresses() -> [H160; 9] {
		[hash(1), hash(2), hash(3), hash(4), hash(5), hash(1024), hash(1025), hash(4096), hash(4097)]
	}
}
impl<R> PrecompileSet for FrontierPrecompiles<R>
where
	R: pallet_evm::Config,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		match handle.code_address() {
			// Ethereum precompiles :
			a if a == hash(1) => Some(ECRecover::execute(handle)),
			a if a == hash(2) => Some(Sha256::execute(handle)),
			a if a == hash(3) => Some(Ripemd160::execute(handle)),
			a if a == hash(4) => Some(Identity::execute(handle)),
			a if a == hash(5) => Some(Modexp::execute(handle)),
			a if a == hash(4096) => Some(BatchPrecompile::<R>::execute(handle)),
			a if a == hash(4097) => Some(RegistryPrecompile::<R>::execute(handle)),
			// Non-Frontier specific nor Ethereum precompiles :
			a if a == hash(1024) => Some(Sha3FIPS256::execute(handle)),
			a if a == hash(1025) => Some(ECRecoverPublicKey::execute(handle)),
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160, _gas: u64) -> IsPrecompileResult {
		IsPrecompileResult::Answer {
			is_precompile: Self::used_addresses().contains(&address),
			extra_cost: 0,
		}	
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}
