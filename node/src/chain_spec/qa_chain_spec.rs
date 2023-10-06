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

use firechain_qa_runtime::{
	constants::currency::*, wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig,
	BalancesConfig, Block, CouncilConfig, DemocracyConfig, ElectionsConfig, EthereumConfig,
	GrandpaConfig, ImOnlineConfig, IndicesConfig, MaxNominations, NominationPoolsConfig,
	SessionConfig, SessionKeys, SocietyConfig, StakerStatus, StakingConfig, SudoConfig,
	SystemConfig, TechnicalCommitteeConfig,
};
use fp_evm::GenesisAccount;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public, H160, U256};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};
use std::{collections::BTreeMap, str::FromStr};

pub use firechain_qa_runtime::{EVMConfig, RuntimeGenesisConfig};
pub use node_primitives::{AccountId, Balance, Signature};

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

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
/// Flaming Fir testnet generator
// pub fn flaming_fir_config() -> Result<ChainSpec, String> {
// 	ChainSpec::from_json_bytes(&include_bytes!("../res/flaming-fir.json")[..])
// }

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

fn staging_testnet_config_genesis() -> RuntimeGenesisConfig {
	#[rustfmt::skip]
		let initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// Stash Account
			// 5Df94LnKM6ptdaLQVpo9rPbTKBQzEpvhcMUKMbdz4Vqoyj7W
			array_bytes::hex_n_into_unchecked("467f6986687ef250f7d17ee591bf143fe53e1b2b6e541c8409c2498e46bd1d17"),
			// Controller account
			// 5Hjn6chCzYt8zM7ej2PGBxuP1YhQ1GVvHxsiM1c5PP55Cc8p
			array_bytes::hex_n_into_unchecked("fafa03430538ddc9f849f74b42065aba2f0330c7be4b9e879eb87816dac58a15"),
			// Grandpa account
			// 5FD7opppAeP2zDqKgxLj3xMHovd9r81Z8SXzRt3RavxWpQq8
			array_bytes::hex2array_unchecked("8b1f4a2e2bc0953f80e9d6f6be8a41bf2c0d7f22648a7a9bb0876e13769a4477")
				.unchecked_into(),
			// Babe Account
			// 5HbD32UJd8JrWDNmBP8zggy8P9HZNypH82LpEYYJ3m2SR5tX
			array_bytes::hex2array_unchecked("f470c0d448a0851086f291ecd432e8e0e924a1d75823626f7b23bf45181a242d")
				.unchecked_into(),
			// imonline Account
			// 5GQviwyEmDqCgrS98yGSKV7JAmcAAFWmsWZH6JnSz45EeJbC
			array_bytes::hex2array_unchecked("c05d27dd7c4a81338d5f7d9bfdd513f4ead5c09abad10d81460f4ba5a754e12f")
				.unchecked_into(),
			// authority discovery account
			// 5HjeF54rtdV9RHLh9BmHfPSfgW268GiqKU12s2cFYWHUYTof
			array_bytes::hex2array_unchecked("fadf92b35dda22345023fcde913a2de2352445def1a8c436873ab96fe2b02468")
				.unchecked_into(),
		),
		(
			// Stash Account
			// 5DrEnS5Htsp8pKBbc24XVwqqR4PJoJxV3hJ575eWpzA2SMqa
			array_bytes::hex_n_into_unchecked("4ef662d89692be4301db3100d2832a060c9bb24c8003c85c4660dabaaa95630f"),
			// Controller account
			// 5EATfvrz79jWrxXKTc3f6TaQTcCJWR864NZPkyu8DruRxdCd
			array_bytes::hex_n_into_unchecked("5cdc30156a362216f071b3bf23526571ecabf138eb5ca7c9e5c91fcd9b84891d"),
			// Grandpa account
			// 5CV4qVPtFRNG1WHHGo27WU6rjK8qgXRJMsxJhJYxoQjWQzMs
			array_bytes::hex2array_unchecked("1294837c326104a861a447816a286e289786f396d2ded9d5374e40dc812ab91a")
				.unchecked_into(),
			// Babe Account
			// 5F9jubJxrByyQHr2NtJoBUbUoz1BnPjhfT5AKV8rby5sG3Pm
			array_bytes::hex2array_unchecked("888bd46502e5a1e10a51b3625b0e6ea76d3353dc1bcac6c9da25d586b6ca7e1c")
				.unchecked_into(),
			// imonline Account
			// 5DfXEnW4veh4XzbhxxaC6YbVWQWUbDvu5q1SpQuEh3zwfk15
			array_bytes::hex2array_unchecked("46ca13b2862bec637e76fbac53e7f73fb04ae8fbb36e9c4a408614c3b4f93525")
				.unchecked_into(),
			// authority discovery account
			// 5Gdv4kU1T7VoX3AxZtNSdMeKAp6vd1JTDpvw7CJvdsy4UGRE
			array_bytes::hex2array_unchecked("ca4521039b528329801bfe8e83f20414329c17859c1372bb149845f71ebee07c")
				.unchecked_into(),
		),
		(
			// Stash Account
			// 5FWGQMkdhvWHp6Zv9MwxwkTJzyv6EHputu5yaqWfF8QWN8Si
			array_bytes::hex_n_into_unchecked("983365767b9938fdb77847dd099bd60da27577c3860deac79361e2302fda931d"),
			// Controller account
			// 5EZnydpGtW3gdBtY39HNBAq3uPmN3itvaymYZMjd5ZacS1y7
			array_bytes::hex_n_into_unchecked("6ea7d1c9b30e32ebe10ac4142066c23138f08fe951ba6f8328992d57c0e04d15"),
			// Grandpa account
			// 5DEsThhp3DH33H44tdbrpjQQvZiTrsd8xjRKwA3EHXjXyVvY
			array_bytes::hex2array_unchecked("33fd047e281b273e0893d5362b5e62bc680b22e170cd377e96e0b3c75c9a3bcd")
				.unchecked_into(),
			// Babe Account
			// 5CJi6NASBmh5469yN84kBPgrK89WKYrrDXQGbVTwPckiLuKi
			array_bytes::hex2array_unchecked("0aae3b0a5957982321825e75ddc34aac99237e3a5b0ec1ebc18db3388d7f8a7e")
				.unchecked_into(),
			// imonline Account
			// 5Eh8fVQRvXAxzXRUVirqS5vp6gQiiUmDeYJe2SNDoGbBaogd
			array_bytes::hex2array_unchecked("7440cf684dc9100f927d089acd6f9b2543658f924abbfe2c02cd6a3ae03d7f1e")
				.unchecked_into(),
			// authority discovery account
			// 5G3w495Ntwms8WYvNh7w5ZG3sAjUjUwCxgg591mPpMrAFQBE
			array_bytes::hex2array_unchecked("b05a1db8a4bfcf011daf21e22e6b681e88a8084f7dcc420d7f569bfe8e903c68")
				.unchecked_into(),
		),
		(
			// Stash Account
			// 5EZhhh24Roth9at1gVPfSmqXDwAXCKttBcRC7wBTWCrmapZU
			array_bytes::hex_n_into_unchecked("6e9610039163f7042f05e706a13a4a12b02b40531fa4e73b4c42e772f45ca43c"),
			// Controller account
			// 5CD7xuZT763zZcCBFd5zDS2hMus2nQ2psEMqj3ZBvkHvHs6z
			array_bytes::hex_n_into_unchecked("066b1dd329aa2fe7ab347534f5bc7cb2af2035680ee1e8169af78a80cd6db815"),
			// Grandpa account
			// 5GnBjhSCabfLSzAXyL77MivXRs7bqN4HcCYxJwGQXY6wzXVR
			array_bytes::hex2array_unchecked("d093d8e2cc4358f1714b246aa164b9dd8607bd897f8dee09944df37376a3ae81")
				.unchecked_into(),
			// Babe Account
			// 5Gje3aohmTiXdXMSX8ghzFmqf5uciK3eAuvdUq1FebSQAYTb
			array_bytes::hex2array_unchecked("cea2ac41d5baeba039de7bb80b41f0a3022b88f8aa005aab7797f192c0ee4303")
				.unchecked_into(),
			// imonline Account
			// 5FW1voGghSqyBkLBjcNZZR5LHwJJwNnwt1yYP2L2yrWesb3N
			array_bytes::hex2array_unchecked("9802ab26f6de56f9f3c24ed37d6f7c9db634537822a028fd23e1001c81ace515")
				.unchecked_into(),
			// authority discovery account
			// 5HGmzrLGGsTPJPLh7v7mFuXV4otd7aN8LFS8iu8CzbCHtgAW
			array_bytes::hex2array_unchecked("e6620d5f33df7b2b953c65dede20b47ecb571269c2b33ecdab2d7d6645912528")
				.unchecked_into(),
		),
	];

	let root_key: AccountId = array_bytes::hex_n_into_unchecked(
		// 5DiKUSBwCfMHrff87PXK7jb5sQHN22PM2QsRgiQfgj1fMngf
		"48ec35b461b474c36b69ea524787bdea45d8cf6cc0e1f38ccc2c2adb5e4ae600",
	);

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(initial_authorities, vec![], root_key, Some(endowed_accounts))
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"Firechain Staging",
		"firechain_staging_network",
		ChainType::Live,
		staging_testnet_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		Some(
			serde_json::from_str("{\"tokenDecimals\": 18, \"tokenSymbol\": \"5IRE\"}")
				.expect("Provided valid json map"),
		),
		Default::default(),
	)
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
	seed: &str,
) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
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
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		]
	});
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
				.choose_multiple(&mut rng, count)
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;

	RuntimeGenesisConfig {
		system: SystemConfig { code: wasm_binary_unwrap().to_vec(), ..Default::default() },
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).collect(),
		},
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
		elections: ElectionsConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		},
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		sudo: SudoConfig { key: Some(root_key) },
		babe: BabeConfig {
			epoch_config: Some(firechain_qa_runtime::BABE_GENESIS_EPOCH_CONFIG),
			..Default::default()
		},
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: Default::default(),
		grandpa: Default::default(),
		technical_membership: Default::default(),
		treasury: Default::default(),
		society: SocietyConfig { pot: 0 },
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
		glutton: Default::default(),
		// EVM compatibility
		evm: Default::default(),
		ethereum: Default::default(),
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
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
	endowed_accounts: Option<Vec<AccountId>>,
	_chain_id: u64,
) -> RuntimeGenesisConfig {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		]
	});
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

	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 5_000_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;

	RuntimeGenesisConfig {
		system: SystemConfig { code: wasm_binary_unwrap().to_vec(), ..Default::default() },
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).collect(),
		},
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
		elections: ElectionsConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		},
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		sudo: SudoConfig { key: Some(root_key) },
		babe: BabeConfig {
			epoch_config: Some(firechain_qa_runtime::BABE_GENESIS_EPOCH_CONFIG),
			..Default::default()
		},
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![], ..Default::default() },
		grandpa: GrandpaConfig { authorities: vec![], ..Default::default() },
		technical_membership: Default::default(),
		treasury: Default::default(),

		society: Default::default(),
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
		glutton: Default::default(),

		// EVM compatibility
		evm: EVMConfig {
			accounts: {
				let mut map = BTreeMap::new();
				map.insert(
					// H160 address of Alice dev account
					// Derived from SS58 (42 prefix) address
					// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
					// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
					// Using the full hex key, truncating to the first 20 bytes (the first 40 hex
					// chars)
					H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
						.expect("internal H160 is valid; qed"),
					GenesisAccount {
						balance: U256::from_str("0xfffffffffffffffffffff")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of Bob dev account
					// Derived from SS58 (42 prefix) address
					// SS58: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
					// hex: 0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48
					// Using the full hex key, truncating to the first 20 bytes (the first 40 hex
					// chars)
					H160::from_str("8eaf04151687736326c9fea17e25fc5287613693")
						.expect("internal H160 is valid; qed"),
					GenesisAccount {
						balance: U256::from_str("0xfffffffffffffffffffff")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of Bob dev account
					// Derived from SS58 (42 prefix) address
					// SS58: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
					// hex: 0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48
					// Using the full hex key, truncating to the first 20 bytes (the first 40 hex
					// chars)
					H160::from_str("05E053aB0f66422d243C1F14Da2091CD56F51F73")
						.expect("internal H160 is valid; qed"),
					GenesisAccount {
						balance: U256::from_str("0xfffffffffffffffffffff")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map
			},
			_marker: Default::default(),
		},
		ethereum: EthereumConfig { _marker: Default::default() },
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
	}
}

fn development_config_genesis() -> RuntimeGenesisConfig {
	development_genesis(
		vec![authority_keys_from_seed("Alice")],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		42, //passing chain_id = 42.  Need to change??
	)
}

/// Development config (single validator Alice)
/// Need to work on it..
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		development_config_genesis,
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

fn local_testnet_genesis() -> RuntimeGenesisConfig {
	testnet_genesis(
		vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		local_testnet_genesis,
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
