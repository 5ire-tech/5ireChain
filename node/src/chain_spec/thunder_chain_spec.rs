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
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public, H160, U256};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};
use std::{collections::BTreeMap, str::FromStr};

pub use firechain_thunder_runtime::{EVMConfig, RuntimeGenesisConfig};
pub use node_primitives::{AccountId, Balance, Signature};

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
			// 5CrQp2Mv9tK6SeGhMDmJo63AV3ejdSNtiaEpW5XTsY5k3R3B
			array_bytes::hex_n_into_unchecked("22dbe873a9de6183e47416db21be8980377f487c9bf312a92fd3c63a89f0693c"),
			// Controller Account
			// 5FA47XaUdjCNVuejzcRfaESScqRfpdA1QPyLk73Pijev5Lvn
			array_bytes::hex_n_into_unchecked("88c91da59f9f0738039984fea3be6cba36907fe9f6ec051beaa15544f212a90f"),
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
			//Stash Account
			// 5CJdffZVQw4jzw2ubBfKbM26VnSiYvjdkE3LNRbcH4VY9Gqs
			array_bytes::hex_n_into_unchecked("0a9f54f3c073bc6e138cfe4c1df07f62cb668de695fe64ab6abff0c968065600"),
			// Controller Account
			// 5FsMGf43hazfhpz56yKDRv9RWQPs21mC3F69gZKuMyV5ZrcM
			array_bytes::hex_n_into_unchecked("a847ef2ffc18eab119bdb65eeb7afd1119cd4e30c2924e69c4d85877a07b7225"),
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
			//Stash Account
			// 5CVaBwm5ErpKB5vZAjkfyXLCDF3m2wePUvbLvr5M3iKg6MjZ
			array_bytes::hex_n_into_unchecked("12f752d067cf98c4b2f6fc0e3e3a34d0929c3714640a27f6df052b2e9e6cc93d"),
			// Controller Account
			// 5EhWbwLVarsEJTkJFjSuPKLHnaeq37j5cwXtPh8n6C1ZagfA
			array_bytes::hex_n_into_unchecked("748aa99eb9487c4f652ffb17ea08404d3ca29f6e32baee18e97de174b81c3847"),
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
			//Stash Account
			// 5EFk47KZXsyWZojkXyoo385pg9eqsWPhimDWWSeouD1kk51F
			array_bytes::hex_n_into_unchecked("60e3901ff4464d0bc18d42888fe14f4e42d6733b81be018ee95c4edff929c548"),
			// Controller Account
			// 5CAg4h4nRRvoxbN1S2KLMcZKCtVXbg4k2iP3pGMrMqY2DjyN
			array_bytes::hex_n_into_unchecked("048d700860157c2d38caae251306df0c09ffd5f4648c07e2cc106eee8a025701"),
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
			//Stash Account
			// 5ECgK4iYj8v9kSxQBTFw8Vj3So6kyKo4nD8z4L5BkmvZhcxV
			array_bytes::hex_n_into_unchecked("5e8d3ac4adb096517fd6750e0e595b97f13f7f5c1a74106a31af934f34910c2f"),
			// Controller Account
			// 5DkZycYgNyoqs6Z9CezU3soShc5J7TWNAAvMvuPYAvF1fLaL
			array_bytes::hex_n_into_unchecked("4aa385b5fdbad6b67e8aa2fbabcc3411938b110576e31a133a6c1e31a9d38a08"),
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
			//Stash Account
			// 5CJRXKUKwjZpvCcjgze5Aw7wLK8CJToHm1PPPLk3iDhAdrNS
			array_bytes::hex_n_into_unchecked("0a7673aab3899cb3416ff5abf32fe5c4b4cf6c707bfeff22dc16ecaf0eee6e7e"),
			// Controller Account
			// 5Gn7JPF7UZeGRW5yPWNhUFHBMQjGb79YzMGB3btpqv4TbGrf
			array_bytes::hex_n_into_unchecked("d084e9ae441a29c52238fac586085d859835b26bb3c6689f5ad9c9aced9f997a"),
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
			//Stash Account
			// 5DDSa44Z12WnTbPGaBTiSxPwcjfJNVnYxVCucL5XtqR3Ct2A
			array_bytes::hex_n_into_unchecked("32e5fbb67b8923f226485b8aabd9e8e3084291a6d851a9ff160b37301b42d85d"),
			// Controller Account
			// 5FZqqs6U57hTheTwsEqSWGxVjPQMVxxrZzWfaSqrh8SKQxvW
			array_bytes::hex_n_into_unchecked("9aedb36829d8a3bb144e872c8efe705cb605cafd5a00c90640be65521ffa2153"),
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
			//Stash Account
			// 5GRGkzgyTv3wrPThbHGh4aguKasKq1pwqzDsVhVJjGsKrsWJ
			array_bytes::hex_n_into_unchecked("c0a099bc0bff2d8eb7e6fb32b0b939464b516d5a8ddd50018f8e8ed227f3db0a"),
			// Controller Account
			// 5GZabnXPPR2a448pnjXoC66cQJAMuAuat3peEj8XcLogaJMs
			array_bytes::hex_n_into_unchecked("c6f69f64c46aa13695c1a581aa2893e9ccff4b82b87fa74276b15cac77c60868"),
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
			//Stash Account
			// 5DXDekNJA1bn7YA5LnFJdytqbuHR2didC8sUrxEMjxubynYn
			array_bytes::hex_n_into_unchecked("4074e93f5910a8b4a1292d148d46a5762eb463fb0b138db99f94abe8f17b212b"),
			// Controller Account
			// 5HgcVTCmd4yVwKtMFdHtwDuCy5JFT7wd4SKpYkMCmNMo5Enq
			array_bytes::hex_n_into_unchecked("f88ff05e8f628ba9495668b20c88902c0fdfac2996b6274386b7db1117f73254"),
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

	let root_key: AccountId = array_bytes::hex_n_into_unchecked(
		// 5DqjKokJMesbvPzZMDxAdufXk27bEJGU2K6eoBK73gFB3hzD
		"4e9337d0ef398b146c9d89da14ea4f71d0d2701199088d728f9ef67c8e64c51c",
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
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
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
