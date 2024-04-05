#![allow(clippy::todo)]

/// Fake specifications, only a workaround to compile with runtime optional features.
/// It's a zero variant enum to ensure at compile time that we never instantiate this type.
pub enum FakeSpec {}

impl FakeSpec {
	/// Parse json file into a `ChainSpec`
	pub fn from_json_file(_path: std::path::PathBuf) -> Result<Self, String> {
		unimplemented!()
	}
}

impl sp_runtime::BuildStorage for FakeSpec {
	fn assimilate_storage(&self, _storage: &mut sp_core::storage::Storage) -> Result<(), String> {
		todo!()
	}
}

impl sc_service::ChainSpec for FakeSpec {
	fn name(&self) -> &str {
		todo!()
	}

	fn id(&self) -> &str {
		todo!()
	}

	fn chain_type(&self) -> sc_chain_spec::ChainType {
		todo!()
	}

	fn boot_nodes(&self) -> &[sc_network::config::MultiaddrWithPeerId] {
		todo!()
	}

	fn telemetry_endpoints(&self) -> &Option<sc_telemetry::TelemetryEndpoints> {
		todo!()
	}

	fn protocol_id(&self) -> Option<&str> {
		todo!()
	}

	fn fork_id(&self) -> Option<&str> {
		todo!()
	}

	fn properties(&self) -> sc_chain_spec::Properties {
		todo!()
	}

	fn extensions(&self) -> &dyn sc_chain_spec::GetExtension {
		todo!()
	}

	fn extensions_mut(&mut self) -> &mut dyn sc_chain_spec::GetExtension {
		todo!()
	}

	fn add_boot_node(&mut self, _addr: sc_network::config::MultiaddrWithPeerId) {
		todo!()
	}

	fn as_json(&self, _raw: bool) -> Result<String, String> {
		todo!()
	}

	fn as_storage_builder(&self) -> &dyn sp_runtime::BuildStorage {
		todo!()
	}

	fn cloned_box(&self) -> Box<dyn sc_service::ChainSpec> {
		todo!()
	}

	fn set_storage(&mut self, _storage: sp_runtime::Storage) {
		todo!()
	}

	fn code_substitutes(&self) -> std::collections::BTreeMap<String, Vec<u8>> {
		todo!()
	}
}
