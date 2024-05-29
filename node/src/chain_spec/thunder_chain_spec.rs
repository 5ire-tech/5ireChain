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
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::UncheckedInto, Pair, Public, H160, U256};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};
use std::{collections::BTreeMap, str::FromStr};

pub use firechain_thunder_runtime::{EVMConfig, RuntimeGenesisConfig};
// pub use node_primitives::{AccountId, Balance, Signature};
use firechain_runtime_core_primitives::opaque::{
	AccountId, Balance, Signature,
};
use hex_literal::hex;
use sp_core::ecdsa;

const ALITH: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";
const BALTATHAR: &str = "0x3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0";
const CHARLETH: &str = "0x798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc";
const DOROTHY: &str = "0x773539d4Ac0e786233D90A233654ccEE26a613D9";
const ETHAN: &str = "0xFf64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB";
const FAITH: &str = "0xC0F0f4ab324C46e55D02D0033343B4Be8A55532d";

type AccountPublic = <Signature as Verify>::Signer;

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
			AccountId::from(hex!("bAAd68b1c64D2Cbd55Ff2Ea0c0b6E91564A5d1c3")),
			// Controller account
			AccountId::from(hex!("2400996D897289B494f6A722776eb20005B6fE82")),
			// Grandpa Key
			// 5DXUP46kzDACCrQq5MmuBVK8qJWn8RNWJPECss26YEPDQYeX
			array_bytes::hex2array_unchecked("40a67ea6de9044df14a8d2f59cd92b49d269c98b879c2ee4d37bbafe7a5514cc")
				.unchecked_into(),
			// Babe Key
			// 5ExiCarB3nKmGEqio4vKG4nXG7VdJxLygxGpArUUXc8S67Hk
			array_bytes::hex2array_unchecked("80226242b1135fc43d532400fbc7058150549894fcbb3c65c862effe31ccc93b")
				.unchecked_into(),
			// Imonline Key
			// 5EhL7MR8BnDoqz4gsvkko3wQkaEZ4pQ12ZLqqiK9Qz9BmRpE
			array_bytes::hex2array_unchecked("7467572c8fb7db5e46314e909874ab179d47e81a558b6ea77dced7e400c99623")
				.unchecked_into(),
			// Authority Discovery Key
			// 5FCBSDxxkreTxzzjt1rzKnKrbx4tm5tQYGs1VAEqMCi2giPw
			array_bytes::hex2array_unchecked("8a68417f2d4883e2ec67087fd7ee873da9baf6d5d5fa2d54f51ccb8bc3842c5e")
				.unchecked_into(),
		),
		(
			// Stash Account
			AccountId::from(hex!("6065b716391d339c0C5b3ce63175369Ee821329b")),
			// Controller account
			AccountId::from(hex!("FE33A1b461aD8DAF3BA98b674Ea400F495c0c90E")),
			// Grandpa Key
			// 5ECE279NXB91SiEm9nUV8PRSMUYtH1rAinQ1fRFeBkg8r7eU
			array_bytes::hex2array_unchecked("5e34b8b558953d248289383732c3ebd8910f01442f7dc71380011a5e3b0d5f3a")
				.unchecked_into(),
			// Babe Key
			// 5CPqMXvjXwC8ypXaL6pqMd4odZQos84azGzEKMHEYwZ2midQ
			array_bytes::hex2array_unchecked("0e96e6f0f6713682d35eec15b6ac7ac000988367887c48a7844aaf2e395d0a14")
				.unchecked_into(),
			// Imonline Key
			// 5GC1dpcQxDtvsaa2EH8cz5N4cdLWT519JZXPSj4DBq5xDUc3
			array_bytes::hex2array_unchecked("b6837fd8fc075fde1f9b6d5647ff6c6de8178f39f855bd75477085e596c84658")
				.unchecked_into(),
			// Authority Discovery Key
			// 5CyGGGUJftV5cEctExKYAG4EaejMTVoK9qGDn94XMQTrq6X6
			array_bytes::hex2array_unchecked("2815da300b5a36a5a87a1122b6ff1386cad6f99fa8614e2d99f0ade61e6dc36d")
				.unchecked_into(),
		),
		(
			// Stash Account
			AccountId::from(hex!("083bE71145B70C230Bd2bBa7c2cDE9CaB2075ff6")),
			// Controller account
			AccountId::from(hex!("2e60222AdB99DF69d5d29E4C74bf54fd7F2C8712")),
			// Grandpa Key
			// 5FkXKQLW6wsXWJpwaavhUQnm76tkFGAPwFaacudJzm578U3T
			array_bytes::hex2array_unchecked("a31309f49d6af5074fe80972bb9c9a3bb660b25255573b4be015e977ae0a5586")
				.unchecked_into(),
			// Babe key
			// 5GNUTK7n743hEXxnhmS5APrZ2QTXhjQS9mwHWBJ1rz8gzP7E
			array_bytes::hex2array_unchecked("be7e3bcfd2a82de9ee13e7d8723c1486bc1e256955551ee209f83a64ac159839")
				.unchecked_into(),
			// Imonline Key
			// 5FHe833RVXwM7QF4XJbt8nBUBRuNvJXnecjazXke4ofJ4mqK
			array_bytes::hex2array_unchecked("8e92513c4b95ce2caef7295ea32e27f0cd979a6b32919e3dab1125d1c4cd2b0a")
				.unchecked_into(),
			// Authority Discovery Key
			// 5EF2CGSeaPdsMP5ViGpc9Je6GaC7v8PGKJZuPKY6uSpWwqEu
			array_bytes::hex2array_unchecked("6056a6de5668ff351527d4185cb497e229061adaa37cefd4cec3c9a5a70a1f56")
				.unchecked_into(),
		),
		(
			// Stash Account
			AccountId::from(hex!("1304E9A7229eEB12600E06B0225e6d9bb79907c2")),
			// Controller account
			AccountId::from(hex!("FF4551D31c501714b8787414208397A56149303b")),
			// Grandpa Key
			// 5Dr8NvyQ8ekb97wspJrz6G5oQqRo18M4njwxYKpMFnEQzaQM
			array_bytes::hex2array_unchecked("4ee0d3043b7f828d737343852751bc002f42358ecad36e05777d735f0d849127")
				.unchecked_into(),
			// Babe Key
			// 5H99AwHsiQfU6aa6E6eJmYz8GnDebRRT4mNNfGBwqHhf9zyM
			array_bytes::hex2array_unchecked("e08f5ddd6397b9e0e9299a4a064533cf0a2350278d7972629a608f371466895b")
				.unchecked_into(),
			// Imonline Key
			// 5CyXw9659xLCnrUn5vxuyuDHBtW5pByAp5ECm4QcSibfSoNF
			array_bytes::hex2array_unchecked("284a9a5b41799d57c9fb8841eca4c08dc1c07c6a84432008b984552658bfe44b")
				.unchecked_into(),
			// Authority Discovery Key
			// 5H758x1t5h4Ck6PYZdYGRJwQg35ssnrh5Q455cYusrJcrXij
			array_bytes::hex2array_unchecked("defb4b9881252dc357300763cb4f394594e2fcfdad61d493685c6cef67707860")
				.unchecked_into(),
		),
		(
			// Stash Account
			AccountId::from(hex!("fb55e345e150E39D31334D62B606d287a12da06F")),
			// Controller account
			AccountId::from(hex!("d715fD1aa1998Bf6Ad5E46631B647Ef9AA8340a0")),
			// Grandpa Key
			// 5FnL1s6HnyhG73vNuvtJ7D9xM4c3hpfcbLhNPg8mv5Yu1tvb
			array_bytes::hex2array_unchecked("a4737b823e65d323200d3fff98bef39e4af8a94c35b6ddbe4e6a821de05b96bb")
				.unchecked_into(),
			// Babe Key
			// 5FqWtw2GM1s1hNMhHwQtWEkDz98MoqTfKv415ErZAeqtDNxi
			array_bytes::hex2array_unchecked("a6e1d87aed5e2c4f379d5fe8abcc120e1b1df49c7c7cdc4b71b91b262aaadb0d")
				.unchecked_into(),
			// Imonline Key
			// 5E5GdgP4QCapQveBMrHqKjGh1A6jAuHZGCcCFh2qkR5CgdJq
			array_bytes::hex2array_unchecked("58e6cd09f9348d6200730aaedbf699db4514848569f593db902af969ca16ce4f")
				.unchecked_into(),
			// Authority Discovery Key
			// 5FcQesnuDYL8RDXuDPfJqFxsqSAQkYSJoR9VQGTqjnnksVNE
			array_bytes::hex2array_unchecked("9ce2a44bbe3f90234c7c7e597704c3d3fc6893055a76392337fe86c92a5fe535")
				.unchecked_into(),
		),
		(
			// Stash Account
			AccountId::from(hex!("3A068fFA8F122d4C008Dfc4be8613De28a27a1ca")),
			// Controller account
			AccountId::from(hex!("a64559F997dd529F5A4832d76b3A74D9FE931941")),
			// Grandpa Key
			// 5H9hXNAdU5jxUjLxhTyEUdRdmfrzFWaG3D2WV9kYSKec8TxK
			array_bytes::hex2array_unchecked("e0fc462163f324c6be4631c673a22eb11679f32930eea70911867cfc2caa448a")
				.unchecked_into(),
			// Babe Key
			// 5GcA2zqgnoB1qB7qh9sx8JPRdNaJzFNRpt11Lim6Y53i3svQ
			array_bytes::hex2array_unchecked("c8edaa2705041dd607f6d0706b776ba6fc3e22cd4c05d0aac7bbb44f2b536045")
				.unchecked_into(),
			// Imonline Key
			// 5Dd59syEHxZCsSAK17UWqQDp7WNPwk7NmyS5Ateo6X8R8AmY
			array_bytes::hex2array_unchecked("44ebc7112e0097586bf31b38425984d024d2ca845f40b528060e40fd99b33026")
				.unchecked_into(),
			// Authority Discovery Key
			// 5CV115c4RUChexV3XoW7HyTAgRmEqijKGw3qXmXhRKxbpVHL
			array_bytes::hex2array_unchecked("12879ae186c85a091bda73204785574ec1b9cb65cd86e8a758cbb5c4b22af01d")
				.unchecked_into(),
		),
		(
			// Stash Account
			AccountId::from(hex!("9001456BeB3D2c4e1470b28416D9A065eaBF694D")),
			// Controller account
			AccountId::from(hex!("97C6fE87c56c036fCCDAAf9CFeCEf17CB5F01f7E")),
			// Grandpa Key
			// 5HUZ5h41HWRCBhV1eW9LeYZeCMGR7gKxCkjtfg7do2QdFz78
			array_bytes::hex2array_unchecked("ef5d841eed522be9552a6adad9a63228440839ceffa47803a12cd54769a523e6")
				.unchecked_into(),
			// Babe Key
			// 5E79sfA9mUuXFFzCMpoLueAUw8ei8uEPi5MvpckknDwoGo9v
			array_bytes::hex2array_unchecked("5a5689fa29db07bc3371d455e3ba51830fe5cbc3e29177bbd9c8cdcc08110572")
				.unchecked_into(),
			// Imonline Key
			// 5Et1qaMnRWT82Cf1s5WscfgoLi7jTUBztKP8sWqzP2NcEPN3
			array_bytes::hex2array_unchecked("7c8d87e8372009e884494c002df9fb654c98a7fd9acfc46a6bfb997e1ef8ba1a")
				.unchecked_into(),
			// Authority Discovery Key
			// 5CamJ3Jxh6YMJnnLrPc4tcLr5MhUB3j1rRf9ZWujJaU5Fue6
			array_bytes::hex2array_unchecked("16ecef00d8a379ce55a2550c7d2e0d327f52329c0e2231b2ee37948c1e126b48")
				.unchecked_into(),
		),
		(
			// Stash Account
			AccountId::from(hex!("2a18a6BcFF287F065EAAdA3EED8f21E91cc51599")),
			// Controller account
			AccountId::from(hex!("02edB501f803Fa508FeaB55721B3DbDAAA8177f2")),
			// Grandpa Key
			// 5DzD9fLwBRwhy7ygrbGDjuErrE6v6T7ER12NjzZn3KXLDgaY
			array_bytes::hex2array_unchecked("550ad94f214f3d77b73aa0483d3c9af49adc558d75c31762a4633b2a14aa223f")
				.unchecked_into(),
			// Babe Key
			// 5GpvxhVyf1EwLuNFtRCFyMMux1HwB3YJW7cdMUSJiSAyxgNY
			array_bytes::hex2array_unchecked("d2abd7efd568b2886495959ad6a7265c75f489055d83bd07c1be7fb7ba665d33")
				.unchecked_into(),
			// Imonline Key
			// 5CrWgTST4iTkSqmhpGYXFE4CDyCZBmutMg55jAA72HPEyRfv
			array_bytes::hex2array_unchecked("22efaaa5dd393aff796e5a4755ced6ef1d4f5afcba50e5ff5d3c95febb520d3f")
				.unchecked_into(),
			// Authority Discovery Key
			// 5Fn8U4UhKLsFJ9MzDgTs3A8sUsyNZ5Gw2BhvssfXEJv78QwH
			array_bytes::hex2array_unchecked("a44c9b76b9de9bbd10e16783e558316d7cba3a75c178e00c76bea2e335fa9c3a")
				.unchecked_into(),
		),
		(
			// Stash Account
			AccountId::from(hex!("0B160AA054a708db5920dBDb9A2EF58f90068567")),
			// Controller account
			AccountId::from(hex!("41DF124fcc97e825d15A8C5568eFDACdF66868ca")),
			// Grandpa Key
			// 5Ei4KEpsPP4QQHUcUCkvZqTfaNmTh3crMccwxRXpTumWqy9s
			array_bytes::hex2array_unchecked("74f56a3df0974c8cbf3e55f927093055cb554ebf3c103e355e769ead057e3cd4")
				.unchecked_into(),
			// Babe Key
			// 5DjtR1njvNgq5TBHmGmvsb3DCCPbprvLRmztHMQuxaYWpyoY
			array_bytes::hex2array_unchecked("4a1e5834c0ada51b1384edbb632101c5c07d603ba0b6dfc6521936cb768e3d01")
				.unchecked_into(),
			// Imonline Key
			// 5Ew7rRdk4PbhVdCh3Wm4Fnbp8DH7nZA4KqjgM9HDLneZrYdU
			array_bytes::hex2array_unchecked("7eeb83a33b343c3827ffe91d030fe22b4da3a279b9ef00f86c92d289ad58d766")
				.unchecked_into(),
			// Authority Discovery Key
			// 5E2h3fVUeDGK2PhaGVQ43TwxXeCPo6uTdbLGPLo2byN8TU8x
			array_bytes::hex2array_unchecked("56ef3f7be05d5a14d0903b2b9cfb0305d91e6aa23425b60c7fb2af77e495b72a")
				.unchecked_into(),
		),
	];

	let root_key: AccountId = AccountId::from(hex!("4b30eE3536684671a3f1A66e820E394CdbCd568E"));

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
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![]
	});

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
		sudo: SudoConfig {key: Some(root_key) },
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
		reward:Default::default(),
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
) -> RuntimeGenesisConfig {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		testnet_accounts()
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
		reward:Default::default(),
	}
}

fn development_config_genesis() -> RuntimeGenesisConfig {
	development_genesis(
		vec![authority_keys_from_seed("Alice")],
		vec![],
		array_bytes::hex_n_into_unchecked::<_, _, 20>(ALITH),
		None,
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
		get_account_id_from_seed::<ecdsa::Public>("Alice"),
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