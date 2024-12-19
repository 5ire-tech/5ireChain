// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use firechain_mainnet_runtime::{
	constants::currency::*, AuthorityDiscoveryConfig, BabeConfig,
	BalancesConfig, Block, CouncilConfig, DemocracyConfig, ElectionsConfig, EthereumConfig,
	GrandpaConfig, ImOnlineConfig, IndicesConfig, MaxNominations, NominationPoolsConfig,
	SessionConfig, SessionKeys, StakerStatus, StakingConfig, SudoConfig,
	TechnicalCommitteeConfig,wasm_binary_unwrap
};
use fp_evm::GenesisAccount;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{ecdsa, Pair, Public, H160, U256};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};
use std::{collections::BTreeMap, str::FromStr};

pub use firechain_mainnet_runtime::{EVMConfig, RuntimeGenesisConfig};
use firechain_runtime_core_primitives::opaque::{AccountId, Balance, Signature};

pub fn mainnet_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../../../specs/5ire-mainnet-specRaw.json")[..])
}

const ALITH: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";
const BALTATHAR: &str = "0x3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0";
const CHARLETH: &str = "0x798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc";
const DOROTHY: &str = "0x773539d4Ac0e786233D90A233654ccEE26a613D9";
const ETHAN: &str = "0xFf64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB";
const FAITH: &str = "0xC0F0f4ab324C46e55D02D0033343B4Be8A55532d";

type AccountPublic = <Signature as Verify>::Signer;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig, Extensions>;

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
	authority: &str,
	seed: &str,
) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId) {
	(
		array_bytes::hex_n_into_unchecked::<_, _, 20>(authority),
		get_account_id_from_seed::<ecdsa::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

fn testnet_accounts() -> Vec<AccountId> {
	vec![
		array_bytes::hex_n_into_unchecked::<_, _, 20>(ALITH),
		array_bytes::hex_n_into_unchecked::<_, _, 20>(BALTATHAR),
		array_bytes::hex_n_into_unchecked::<_, _, 20>(CHARLETH),
		array_bytes::hex_n_into_unchecked::<_, _, 20>(DOROTHY),
		array_bytes::hex_n_into_unchecked::<_, _, 20>(ETHAN),
		array_bytes::hex_n_into_unchecked::<_, _, 20>(FAITH),
	]
}
/// Helper function to create RuntimeGenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> RuntimeGenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| vec![]);

	let mut endowed_accounts_validator: Vec<AccountId> = Vec::new();

	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts_validator.contains(x) {
				endowed_accounts_validator.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.choose_multiple(&mut rng, count)
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	// Based on current tokenomics
	// Initial validators Balance
	// Total Balance = Bonding Balance(100_000 5IRE) + Tranferrable Balance(2 5IRE)
	// Bonding Balance: Staking
	// Trafferable Balance: Charge Fee
	const ENDOWMENT_AUTHORITY: Balance = 100_002 * DOLLARS;

	const STASH: Balance = 100_000 * DOLLARS;

	// Pre-minted sudo key for charging transaction fee
	const ENDOWMENT_SUDO: Balance = 20 * DOLLARS;

	let mut endowed_balance: Vec<(AccountId, Balance)> =
		endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT_SUDO)).collect();
	let endowed_validator_balance: Vec<(AccountId, Balance)> = endowed_accounts_validator
		.iter()
		.cloned()
		.map(|x| (x, ENDOWMENT_AUTHORITY))
		.collect();

	endowed_balance.extend(endowed_validator_balance);

	RuntimeGenesisConfig {
		system: Default::default(),
		balances: BalancesConfig { balances: endowed_balance },
		indices: IndicesConfig { indices: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			..Default::default()
		},
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig::default(),
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig::default(),
		sudo: SudoConfig { key: Some(root_key) },
		babe: BabeConfig {
			epoch_config: Some(firechain_mainnet_runtime::BABE_GENESIS_EPOCH_CONFIG),
			..Default::default()
		},
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: Default::default(),
		grandpa: Default::default(),
		technical_membership: Default::default(),
		treasury: Default::default(),
		vesting: Default::default(),
		assets: Default::default(),
		pool_assets: Default::default(),
		transaction_storage: Default::default(),
		transaction_payment: Default::default(),
		alliance: Default::default(),
		alliance_motion: Default::default(),
		nomination_pools: NominationPoolsConfig {
			min_create_bond: 10 * DOLLARS,
			min_join_bond: DOLLARS,
			..Default::default()
		},
		// EVM compatibility
		evm: Default::default(),
		ethereum: Default::default(),
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
		reward: Default::default(),
	}
}

/// Helper function to create RuntimeGenesisConfig for development
pub fn development_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
) -> serde_json::Value {
	let mut endowed_accounts: Vec<AccountId> = testnet_accounts();
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	const ENDOWMENT: Balance = 5_000_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;
	let evm_accounts = {
		let mut map = BTreeMap::new();
		map.insert(
			// H160 address of Alice dev account
			// Derived from SS58 (42 prefix) address
			// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
			// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
			// Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
			H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
				.expect("internal H160 is valid; qed"),
			fp_evm::GenesisAccount {
				balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
					.expect("internal U256 is valid; qed"),
				code: Default::default(),
				nonce: Default::default(),
				storage: Default::default(),
			},
		);
		map.insert(
			// H160 address of CI test runner account
			H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b")
				.expect("internal H160 is valid; qed"),
			fp_evm::GenesisAccount {
				balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
					.expect("internal U256 is valid; qed"),
				code: Default::default(),
				nonce: Default::default(),
				storage: Default::default(),
			},
		);
		map.insert(
			// H160 address for benchmark usage
			H160::from_str("1000000000000000000000000000000000000001")
				.expect("internal H160 is valid; qed"),
			fp_evm::GenesisAccount {
				nonce: U256::from(1),
				balance: U256::from(1_000_000_000_000_000_000_000_000u128),
				storage: Default::default(),
				code: vec![0x00],
			},
		);
		map
	};

	serde_json::json!({
		"balances": {
			"balances": endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)) .collect::<Vec<_>>(),
		},
		"indices": IndicesConfig { indices: vec![] },
		"session": {
			"keys": initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		"staking": {
			"validator_count": initial_authorities.len() as u32,
			"minimum_validator_count": initial_authorities.len() as u32,
			"invulnerables": initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
			"slash_reward_fraction": Perbill::from_percent(10),
			"stakers": stakers.clone(),
		},
		"sudo": { "key": Some(root_key) },
		"babe": {
			"epoch_config": Some(firechain_mainnet_runtime::BABE_GENESIS_EPOCH_CONFIG),
		
		},
		"nomination_pools": {
			"min_create_bond": 10 * DOLLARS,
			"min_join_bond": DOLLARS,
		},
		"evm": { "accounts": evm_accounts },
	})
}

fn development_config_genesis() -> serde_json::Value  {
	development_genesis(
		vec![authority_keys_from_seed(ALITH, "Alice")],
		vec![],
		array_bytes::hex_n_into_unchecked::<_, _, 20>(ALITH),
	)
}

pub fn fire_chain_spec_properties() -> serde_json::map::Map<String, serde_json::Value> {
	serde_json::json!({
		"tokenDecimals": 18,
		"tokenSymbol": "5IRE",
	})
	.as_object()
	.expect("Map given; 5IRE")
	.clone()
}

/// Development config (single validator Alice)
/// Need to work on it..
pub fn development_config() -> ChainSpec {
	ChainSpec::builder(wasm_binary_unwrap(), Default::default())
		.with_name("Development")
		.with_id("mainnet_5ireChain_dev")
		.with_chain_type(ChainType::Development)
		.with_genesis_config_patch(development_config_genesis())
		.with_properties(fire_chain_spec_properties())
		.build()
}

fn local_mainnet_genesis() -> RuntimeGenesisConfig {
	testnet_genesis(
		vec![authority_keys_from_seed(ALITH, "Alice"), authority_keys_from_seed(BALTATHAR, "Bob")],
		vec![],
		array_bytes::hex_n_into_unchecked::<_, _, 20>(ALITH),
		None,
	)
}

/// Local mainnet config (multivalidator Alice + Bob)
pub fn local_mainnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"5ireChain Local Testnet",
		"mainnet_5ireChain_local",
		ChainType::Local,
		local_mainnet_genesis,
		vec![],
		None,
		None,
		None,
		Some(
			serde_json::from_str("{\"tokenDecimals\": 18, \"tokenSymbol\": \"5IRE\"}")
				.expect("Provided valid json map"),
		),
		Default::default(),
	)
}
