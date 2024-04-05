pub mod fake_chain_spec;

#[cfg(feature = "firechain-qa")]
pub mod qa_chain_spec;
#[cfg(feature = "firechain-thunder")]
pub mod thunder_chain_spec;
#[cfg(feature = "firechain-mainnet")]
pub mod mainnet_chain_spec;

#[cfg(not(feature = "firechain-qa"))]
pub mod firechain_qa {
	pub type ChainSpec = crate::chain_spec::fake_chain_spec::FakeSpec;
	pub fn chain_spec_from_json_file(_: std::path::PathBuf) -> Result<ChainSpec, String> {
		panic!("firechain QA runtime not enabled")
	}
	pub fn development_chain_spec(_: Option<String>, _: Option<u32>) -> ChainSpec {
		panic!("firechain QA runtime not enabled")
	}
}
#[cfg(not(feature = "firechain-thunder"))]
pub mod firechain_thunder {
	pub type ChainSpec = crate::chain_spec::fake_chain_spec::FakeSpec;
	pub fn chain_spec_from_json_file(_: std::path::PathBuf) -> Result<ChainSpec, String> {
		panic!("firechain Thunder runtime not enabled")
	}
	pub fn development_chain_spec(_: Option<String>, _: Option<u32>) -> ChainSpec {
		panic!("firechain Thunder runtime not enabled")
	}
}
#[cfg(not(feature = "firechain-mainnet"))]
pub mod firechain_mainnet {
	pub type ChainSpec = crate::chain_spec::fake_chain_spec::FakeSpec;
	pub fn chain_spec_from_json_file(_: std::path::PathBuf) -> Result<ChainSpec, String> {
		panic!("firechain Mainnet runtime not enabled")
	}
	pub fn development_chain_spec(_: Option<String>, _: Option<u32>) -> ChainSpec {
		panic!("firechain Mainnet runtime not enabled")
	}
}
