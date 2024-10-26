
use precompile_utils::precompile_set::*;
use pallet_evm_precompile_batch::BatchPrecompile;
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_registry::PrecompileRegistry;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};


type EthereumPrecompilesChecks = (AcceptDelegateCall, CallableByContract, CallableByPrecompile);


#[precompile_utils::precompile_name_from_address]
type FirePrecompilesAt<R> = (
	// Ethereum precompiles:
	// We allow DELEGATECALL to stay compliant with Ethereum behavior.
	PrecompileAt<AddressU64<1>, ECRecover, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<2>, Sha256, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<3>, Ripemd160, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<4>, Identity, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<5>, Modexp, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<1024>, Sha3FIPS256, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<1025>, ECRecoverPublicKey, EthereumPrecompilesChecks>,

	PrecompileAt<
		AddressU64<4096>,
		BatchPrecompile<R>,
		(
			SubcallWithMaxNesting<2>,
			// Batch is the only precompile allowed to call Batch.
			CallableByPrecompile<OnlyFrom<AddressU64<4096>>>,
		),
	>,
	PrecompileAt<
		AddressU64<4097>,
		PrecompileRegistry<R>,
		(CallableByContract, CallableByPrecompile),
	>,
);

/// The PrecompileSet installed in the Moonriver runtime.
/// We include the nine Istanbul precompiles
/// (https://github.com/ethereum/go-ethereum/blob/3c46f557/core/vm/contracts.go#L69)
/// The following distribution has been decided for the precompiles
/// 0-1023: Ethereum Mainnet Precompiles
/// 1024-2047 Precompiles that are not in Ethereum Mainnet but are neither Moonbeam specific
/// 2048-4095 Moonbeam specific precompiles
pub type FirePrecompiles<R> = PrecompileSetBuilder<
	R,
	(
		// Skip precompiles if out of range.
		PrecompilesInRangeInclusive<(AddressU64<1>, AddressU64<4200>), FirePrecompilesAt<R>>,

	),
>;

