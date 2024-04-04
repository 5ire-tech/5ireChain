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

use firechain_thunder_runtime::{
	constants::currency::*, wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig,
	BalancesConfig, Block, CouncilConfig, DemocracyConfig, ElectionsConfig, EthereumConfig,
	GrandpaConfig, ImOnlineConfig, IndicesConfig, MaxNominations, NominationPoolsConfig,
	SessionConfig, SessionKeys, StakerStatus, StakingConfig, SudoConfig, SystemConfig,
	TechnicalCommitteeConfig,
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

pub use firechain_thunder_runtime::{EVMConfig, RuntimeGenesisConfig};
pub use node_primitives::{AccountId, Balance, Signature};

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "thunder-5ire";
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
			// 5GMrYp9kF7XfHS9VpnuhR3SQSAHzkN5QA78fG3zZaWSgcRAW
			array_bytes::hex_n_into_unchecked("be055db8061cc45db6e9d1081e0557ebc227638093753231cfd1c73c20676269"),
			// Controller Account
			// 5CLceuLf2iRDkBa57AdPXHb8FqCVfFNzdBs9b4SZJ3KVKaos
			array_bytes::hex_n_into_unchecked("0c22697f7bb7a8cb19aed95c8f4c59fb0d9aed8d722e3d705c00d1aa482d144b"),
			// Grandpa Key
			// 5CUQhVgneQU45AAaYRwGxB72xdod6qEczLVEvvo35sdrtgz2
			array_bytes::hex2array_unchecked("12142153d41342889b2d06105ee741a7775cbf57eca364df4f2cb84e57080a07")
				.unchecked_into(),
			// Babe Key
			// 5DbvZ1RMaQ2ALWPE27NKSFcMVrqWgGTJaSAwJjQNnNR5jXbv
			array_bytes::hex2array_unchecked("440b93ef306bc1ce593a600b976d9f56ef60cfda8e9ce8c515474fa940f7602d")
				.unchecked_into(),
			// Imonline Key
			// 5CFwuUwmkD4vgwXPWD4U3smCgGzMwVa5LrpBxmhp4tXUZYiV
			array_bytes::hex2array_unchecked("0892fdafdd340a9ff968587bd71e6d8cb9f6fa77960a05ff95c92ddb3a05d77e")
				.unchecked_into(),
			// Authority Discovery Key
			// 5CaL7qhcdhiM1hYKcoR5VeYizQXen8TeVDxfyJgFbS5WfG7b
			array_bytes::hex2array_unchecked("16982f209446d07ad1b36d6ba3a9c2e90f52db3169c41d608fec08d526ad4744")
				.unchecked_into(),
		),
		(
			//Stash Account
			// 5EJFbw1cBAafHqzx8jbBeBngJgqS5oPwdbwY1qdx9Fz9Nr9x
			array_bytes::hex_n_into_unchecked("62cd8600e09239f8ccb88ac0ab20dcc580306d273f484980f36bc770f300883c"),
			// Controller Account
			// 5EUJs8i23gsEUsn2WDS7vtg9XnTgTLUXndpoUM9epHpLLUqx
			array_bytes::hex_n_into_unchecked("6a78f56cfdc02e3af6e71443eb486b51fd58ea570b29f97817ac4fcdc896c565"),
			// Grandpa Key
			// 5EkNeM8T7QQQgAWEE2JryYpZNwwXYW7fervHmZ28PhTTQrph
			array_bytes::hex2array_unchecked("76b99bb825b4eb9e91d29a5546c1a84aec8142e86096ac6b86cb387cd9cc5d4a")
				.unchecked_into(),
			// Babe Key
			// 5EvXomXtfsWbWHmCpRHTH7mTzKE2AjqrWhKJENe69YYVmGD7
			array_bytes::hex2array_unchecked("7e78e7e28d33590d261815eddbc1a68b3bbfbf9606d7123bc564b7ec9ebb956f")
				.unchecked_into(),
			// Imonline Key
			// 5EPDBBoWkjBQmvobHKN6E75hhzoFPrpNRxhFTQYr8LWeV3Zy
			array_bytes::hex2array_unchecked("669594e928ed093ec5ef5cf75757cf0eb62f1812347f7d69729213a5a6b9ab3b")
				.unchecked_into(),
			// Authority Discovery Key
			// 5CdGkE5ESTTNUdo5aNUmzZxpzLJR1czWMUeRfGEdvm5iibMv
			array_bytes::hex2array_unchecked("18d691219f7cef3e4b5f82d3586ba913a9f00879ea9adc503d111fa2f2ee0b0d")
				.unchecked_into(),
		),
		(
			//Stash Account
			// 5Cr6sFdRTmBDjpciMnYxLWonhHvb19gRVvchwbXB2YkJfaLW
			array_bytes::hex_n_into_unchecked("229f8083a7d8a256bf80fadb21db8ed435b5de311f47db791b706ec450f45111"),
			// Controller Account
			// 5Ccg3RzTzi8awEhoJF3LCE67EqBCjutzfzduq8jhqQ5tpu24
			array_bytes::hex_n_into_unchecked("1861beb96bb42dcd335155305670755799b514ee62616fc5bede4d49e2647525"),
			// Grandpa Key
			// 5HkgqDV2u4kdcygt5ALPVgoc4gMW1KQ9Q1kcvfr2JBGGbYge
			array_bytes::hex2array_unchecked("fbab8867344acd5ed00428a20a2b2d768ad9381216703a8f1984c544c2f1efca")
				.unchecked_into(),
			// Babe key
			// 5Cu56oz5BBusV9z2245jN1VX3tcuMzGD2UfToB1iMHpzYbRh
			array_bytes::hex2array_unchecked("24e34ae24c55945ae248218dc3a4bbe8cd66636eaad4e9ffe4abe10097b7d56f")
				.unchecked_into(),
			// Imonline Key
			// 5D7evaghAUvFZRJvDqpSpdkdJLYNSG2fwrqTr66EVh3jLxkB
			array_bytes::hex2array_unchecked("2e7c190db0670e800ff44e3dd5668b2d8791b5eae826d7c8a481614a05906277")
				.unchecked_into(),
			// Authority Discovery Key
			// 5F7Rj5DJtFemVZG8FsynWVhGdqa9Rz1EjTbEqgWQn37n8Xyc
			array_bytes::hex2array_unchecked("86c8228b885484815ede870f35482b34e303af06568bb6ecc30201919e1b6855")
				.unchecked_into(),
		),
		(
			//Stash Account
			// 5EXKXux2FQXmA3V2p972JzLu1yPKKxskVekEoiDTCGzjcEk8
			array_bytes::hex_n_into_unchecked("6cc4f218867e0ca92a2bf6cec5b916113f1ffee5a541a54d2fd93ba2d388a156"),
			// Controller Account
			// 5HYmh1cQ5mpdV9qV7MG2v5rxdimd17Kr5ueYi1pFmsroJbm3
			array_bytes::hex_n_into_unchecked("f294f17a6ec64bc040c66e1a1da525864100405cd3297af06abcb150c519ee6d"),
			// Grandpa Key
			// 5Gq2LFdGkSJTwGAz47tsrYhZ9xAjoXJFUo2cjdcmcDabhTbS
			array_bytes::hex2array_unchecked("d2bded0d4d41631fe7244999cdc0bf1f42e77fb3abc1c679be2cd0a594c8a98a")
				.unchecked_into(),
			// Babe Key
			// 5Gh28tLMzfsxSVWaxNQe12BjMCwe3WX92WyNZvoq9nQVaC7P
			array_bytes::hex2array_unchecked("cca34d91dded37a9d727c6290ecb16f1ce02226055e2c993bbf4a82f1a7b7c1a")
				.unchecked_into(),
			// Imonline Key
			// 5GQoNDuVaabw2RiY149UN8sL4By247Gt3GiGpkXK9NqGWqxF
			array_bytes::hex2array_unchecked("c04463843a18840651943d898bbaf5a244129ccaa8978bf874292a54a2a28c5a")
				.unchecked_into(),
			// Authority Discovery Key
			// 5Dktowe4tRC9xfBMd9SZ3VqTd5bLEx9nbuDPeKioD3iQs14p
			array_bytes::hex2array_unchecked("4ae2ebb43e0cb7af00e38c38b2ef0c2f2fe3d8bbb11d48a7b7fdf54fb9de7e48")
				.unchecked_into(),
		),
	];

	let root_key: AccountId = array_bytes::hex_n_into_unchecked(
		// 5DiKUSBwCfMHrff87PXK7jb5sQHN22PM2QsRgiQfgj1fMngf
		"08d9167acc03a89754b85bb9d3f5c82a3964c222a7811689a6c852c9f3afed77",
	);

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(initial_authorities, vec![], root_key, Some(endowed_accounts))
}

///  Thunder config.
pub fn staging_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"5ireChain Thunder",
		"thunder_firechain_staging",
		ChainType::Live,
		staging_testnet_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
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
			epoch_config: Some(firechain_thunder_runtime::BABE_GENESIS_EPOCH_CONFIG),
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
			epoch_config: Some(firechain_thunder_runtime::BABE_GENESIS_EPOCH_CONFIG),
			..Default::default()
		},
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![], ..Default::default() },
		grandpa: GrandpaConfig { authorities: vec![], ..Default::default() },
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
		evm: EVMConfig {
			accounts: {
				let mut map = BTreeMap::new();
				map.insert(
					// SS58: 5Ge11vgR8YoB7xJPEj4j1eRmvx6vKBLk3uq1nv37eeEbYmR9
					// hex: 0xca55cbeb97bf4ad51541ec60a784381b5df71bab3c605ee98f48c9cd8e790d70
					H160::from_str("48Df7B35247786418a7e279e508325952B9Fc92F")
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
					// SS58: 5CrF9cwmf3SvradcJP7jU5BfXqGGniAt5jCMi6XeCNo2LBDB
					// hex: 0x22bb61e352da49e18ca6d292cb7ed667678fa88870860efb4c8bdf91e8a44a01
					H160::from_str("74E4214c9C3D9726E1A0B57357C4dd117641c536")
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
					// SS58: 5FFVZ9cfrRTHf4eaXLZfGkDwot1ULv46ddgT3fLcs4fxe6CS
					// hex: 0x8ceefcc55493e13574f43c75a59142c0de950bdc431ffc1b12add8c786e7cc6c
					H160::from_str("FE31f14425993A3d9aeDEd195C56999eBE097d92")
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
		"thunder_5ireChain_dev",
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
		"5irechain Local Testnet",
		"thunder_5ireChain_local_testnet",
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
