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
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::UncheckedInto, ecdsa, Pair, Public, H160, U256};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};
use std::{collections::BTreeMap, str::FromStr};

pub use firechain_mainnet_runtime::{EVMConfig, RuntimeGenesisConfig};
use firechain_runtime_core_primitives::opaque::{
	AccountId, Balance,  Signature,
};
use hex_literal::hex;

const ALITH: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";
const CHARLETH: &str = "0x1B1Afe03Fc576fD6ca3eA57701Cf7c134Ca0256A";
const DOROTHY: &str = "0xa6beEA3E3D0DD5EB2eE5961ba9BB14bCC1Aa0158";
const ETHAN: &str = "0x10fe10ce77db20f9E23496CD11bCdeC7DE538be2";
const FAITH: &str = "0x5Ad36E29de0706D8CF51d91306e2201bCc701E97";


type AccountPublic = <Signature as Verify>::Signer;


const DEFAULT_PROTOCOL_ID: &str = "mainnet-5ire";
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

fn staging_mainnet_config_genesis() -> RuntimeGenesisConfig {
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
			// 5CcpmJ5LKTy4xGuz5nPUM8xzVGGe3dr5qQrQ82BmXDpt55TQ
			hex!("187f1aebdce344c33e9902ccb2b801a0b0930cdcb444d68b53be3d08aba2ed05").into(),
			// Controller account
			// 5Fjt5SbYfdvixM4aj3WvAjMjxWhf9R1ayroXLjrzodpXckAi
			hex!("a295aced06b13f5adde2833e669eaa57d6581e2e99bb90fd66982c017319ca09").into(),
			// Grandpa account
			// 5Dg1eBtrEqyiunsEij5JFY7qbGhZmDDxzdqoLo4A23Xpe3r9
			array_bytes::hex2array_unchecked("4729b11102606a9faf142f58d95330aa8a93f3850b964253c9aab9ade41009bd")
				.unchecked_into(),
			// Babe Account
			// 5DJ5v9fBdUtUj9UD81AM947py5YB62RpvUMqZJQ2En6n1vF4
			array_bytes::hex2array_unchecked("0x3670af32b5446b631be19e8c841c682956683093f0cd7d82865568dd7e25bc4e")
				.unchecked_into(),
			// imonline Account
			// 5HbGPqUS7Vef2aVGR1vvd4TDgtdETNgWW7JT9Wmdg919J1aH
			array_bytes::hex2array_unchecked("f47c0f59fdced85b0e6e5c750506d807604ea81c3484f1194bcb4fd115a3f82d")
				.unchecked_into(),
			// authority discovery account
			// 5HZ7kiYn6ZtrPDKq22io4vykegmNFsUuM7nTLhChfabMDqGq
			array_bytes::hex2array_unchecked("f2d87bff0c2dc384d100663b68a711729412dd08bc49dd5aebefa457346e5147")
				.unchecked_into(),
		),
		(
			// Stash Account
			// 5HpCFqRwZpyZznqYBK1PYX9QZF1yJBC89oKSrzDm1VjMM7gN
			hex!("fe585216f9755ce330efcdc8de70eaaea87b2591fb10683e66dd88bb30d04626").into(),
			// Controller account
			// 5FR1b1un4161oHWereCYXM1DHWSaDXVXwqjtA4bpWwFDDUNH
			hex!("94314b1c08e0eca215ce16618f71d7d0eb8eaed714e6127a8c4eb36be4762732").into(),
			// Grandpa account
			// 5HcsCXhXJZWiujQAxxqRrRntk18QT4PfF7nhFtZ9866ko5c3
			array_bytes::hex2array_unchecked("f5b4783b0fd027cc2fb7cae250b8e2540e29898d7df90abf8b0f0db56a17068d")
				.unchecked_into(),
			// Babe Account
			// 5C5JrD7bqKZQj6Q11KK5L3wU6VcBc1bokm9qQYqFzYu4HcjG
			array_bytes::hex2array_unchecked("0075cb3a471805fe33c9cf4138bbf48d4e4238dc574715dca99f93f18bca2b28")
				.unchecked_into(),
			// imonline Account
			// 5DXn9nYgWeoYerQ2TDLmcACR2vkQohyaCEYYYMbYGmt9hj6m
			array_bytes::hex2array_unchecked("40e2516c1bcae0b44570d6514fc9c12d4b063166d5116ee657457d4f5cf31546")
				.unchecked_into(),
			// authority discovery account
			// 5C87ywqS2ctv3UjNChuzXjQGKJowkL8D8VNZZnDEfS1ZgKgM
			array_bytes::hex2array_unchecked("029af325faf5b3045a5a170d00f7c7837fb167ff8e04b0deb99a7f5ffac2c251")
				.unchecked_into(),
		),
		(
			// Stash Account
			// 5CqrFASJbK42X8EoP9sYrZph3beG3KboUjBB26H48UC4pdMx
			hex!("226e4778aa989697711bed0e28597e5df391e3cc0e942708bad95d6d25877e41").into(),
			// Controller account
			// 5EewrSaMC4q7VBETeofXAzP3V8xsw5J4tkCVjoam6y9kXqZK
			hex!("7295ecf96ed1b0a3de524565c5fcada5ec9bb677d2f659577ff0d4f1169afb46").into(),
			// Grandpa account
			// 5D3bf2ipzdf1Gzh57Q96mYVjni5VNrr6r48a4kiLCei58EGP
			array_bytes::hex2array_unchecked("2b641d61683b9e965ee641e27abe2621bc5cc6a94635475693e7efa8e5d5846e")
				.unchecked_into(),
			// Babe Account
			// 5DV9YSeqkWpaYdLpdWvTh6p1hBiTjzXrFgshSSmHYPTsmAfB
			array_bytes::hex2array_unchecked("3ee096d278b9036ad225e924129c6db4cd888b81ff4db49c9902db8825a7765d")
				.unchecked_into(),
			// imonline Account
			// 5DUC6GaqAzf8LUiztmPhUe8YJzjAK2YA9zZ56WTuCKVLqro4
			array_bytes::hex2array_unchecked("3e25ec6cf9689368a03024f18af6b60b6876ecdf78c1386dda86029444f18668")
				.unchecked_into(),
			// authority discovery account
			// 5CqgCESdhaaBPqgx4ui7Kmr71524vwHp8rhXjELUSbYBSjEg
			array_bytes::hex2array_unchecked("224c7231e2529f9e3125bc7864eeae448d5b52108cfc259f58d789cf4a573c50")
				.unchecked_into(),
		),
		(
			// Stash Account
			// 5GCU2rjeQUh8bHUEytM27DrzF1EwFmrwAzhPKwev5GZvm5Tc
			hex!("b6dc5c3901edccf7db83e6c2af0bf6d63eceb077b62480ed3f932c7fe90ccf5a").into(),
			// Controller account
			// 5DoV9uMRHcANpVhTP9cUoTky1VdPqBxXotaQQvVERhY7u5p5
			hex!("4cdd06344118f2df0cdbfc646d54d94d77fc46ad86c806deb0dd3cf044776d67").into(),
			// Grandpa account
			// 5Dg1eBtrEqyiunsEij5JFY7qbGhZmDDxzdqoLo4A23Xpe3r9
			array_bytes::hex2array_unchecked("4729b11102606a9faf142f58d95330aa8a93f3850b964253c9aab9ade41009bd")
				.unchecked_into(),
			// Babe Account
			// 5H8UQwVFfWnAPzqBXXz3s5SkBuxgXqZXZp7Nq6XzQLj4Zwr8
			array_bytes::hex2array_unchecked("e00ce4ca0b53d78383e3842ad4dd27a442f3254ec0a489c048c298c3e4f36a3e")
				.unchecked_into(),
			// imonline Account
			// 5HbGPqUS7Vef2aVGR1vvd4TDgtdETNgWW7JT9Wmdg919J1aH
			array_bytes::hex2array_unchecked("f47c0f59fdced85b0e6e5c750506d807604ea81c3484f1194bcb4fd115a3f82d")
				.unchecked_into(),
			// authority discovery account
			// 5GqKJqqjjHXZmekgre3s7yNGCbXXhNeXZuRJRfRLAKqKDBHD
			array_bytes::hex2array_unchecked("d2f7123d1448d942815a2460d39fc3cef0f7c5f53e7dbeb05210af2403369f1a")
				.unchecked_into(),
		),
	];

	let root_key: AccountId = hex!(
		// 5CVFxLsc3xvQJ1LZAThu21xdtEnGRVqpuLv2pWxKdWYnYXE3
		"12b9f0cf6185531102242efed7478dcd53e174bc8764172c371d7cf87d55fa22"
	).into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(initial_authorities, vec![], root_key, Some(endowed_accounts))
}

/// 5ireChain mainnet config.
pub fn staging_mainnet_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"5ireChain Mainnet",
		"mainnet_5ireChain_staging",
		ChainType::Live,
		staging_mainnet_config_genesis,
		boot_nodes,
		None,
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
		array_bytes::hex_n_into_unchecked::<_, _, 20>(ALITH),
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
		// array_bytes::hex_n_into_unchecked::<_, _, 20>(BALTATHAR),
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
		system: SystemConfig { code: wasm_binary_unwrap().to_vec(), ..Default::default() },
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
		nomination_pools: NominationPoolsConfig {
			min_create_bond: 10 * DOLLARS,
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
	endowed_accounts: Option<Vec<AccountId>>,
	_chain_id: u64,
) -> RuntimeGenesisConfig {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<ecdsa::Public>("Alice"),
			get_account_id_from_seed::<ecdsa::Public>("Bob"),
			get_account_id_from_seed::<ecdsa::Public>("Charlie"),
			get_account_id_from_seed::<ecdsa::Public>("Dave"),
			get_account_id_from_seed::<ecdsa::Public>("Eve"),
			get_account_id_from_seed::<ecdsa::Public>("Ferdie"),
			get_account_id_from_seed::<ecdsa::Public>("Alice//stash"),
			get_account_id_from_seed::<ecdsa::Public>("Bob//stash"),
			get_account_id_from_seed::<ecdsa::Public>("Charlie//stash"),
			get_account_id_from_seed::<ecdsa::Public>("Dave//stash"),
			get_account_id_from_seed::<ecdsa::Public>("Eve//stash"),
			get_account_id_from_seed::<ecdsa::Public>("Ferdie//stash"),
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
		elections: ElectionsConfig::default(),
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig::default(),
		sudo: SudoConfig { key: Some(root_key) },
		babe: BabeConfig {
			epoch_config: Some(firechain_mainnet_runtime::BABE_GENESIS_EPOCH_CONFIG),
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
		reward: Default::default(),
	}
}

fn development_config_genesis() -> RuntimeGenesisConfig {
	development_genesis(
		vec![authority_keys_from_seed("Alice")],
		vec![],
		array_bytes::hex_n_into_unchecked::<_, _, 20>(ALITH),
		None,
		42, //passing chain_id = 42.  Need to change??
	)
}

/// Development config (single validator Alice)
/// Need to work on it..
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"mainnet_5ireChain_dev",
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

fn local_mainnet_genesis() -> RuntimeGenesisConfig {
	testnet_genesis(
		vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
		vec![],
		get_account_id_from_seed::<ecdsa::Public>("Alice"),
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
