use hex_literal::hex;
use node_primitives::*;
use node_template_runtime::{
	constants::currency::*, opaque::SessionKeys,AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, CouncilConfig,
	DemocracyConfig, ElectionsConfig, GenesisConfig, GrandpaConfig, ImOnlineConfig, MaxNominations,
	SessionConfig, StakerStatus, StakingConfig, SudoConfig, SystemConfig, TechnicalCommitteeConfig,
	NominationPoolsConfig,EVMConfig,EthereumConfig,
	BABE_GENESIS_EPOCH_CONFIG, wasm_binary_unwrap,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public,H160, U256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use serde_json;

use std::str::FromStr;

use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};
use std::{collections::BTreeMap};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

fn session_keys(babe: BabeId, grandpa: GrandpaId, im_online: ImOnlineId,authority_discovery: AuthorityDiscoveryId,) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online,authority_discovery }
}

/// Transfer slice into unchecked evm address
// pub fn transfer_evm_uncheck(raw_account: &[u8]) -> Option<H160> {
//     let data = if raw_account.len() == 20 {
//         raw_account.to_vec()
//     } else if raw_account.len() == 40 {
//         hex::decode(raw_account).ok()?
//     } else if raw_account.len() == 42 {
//         let mut key = [0u8; 40];
//         // remove 0x prefix
//         key.copy_from_slice(&raw_account[2..42]);
//         hex::decode(key).ok()?
//     } else {
//         return None;
//     };

//     let mut key = [0u8; 20];
//     key.copy_from_slice(&data);
//     H160::try_from(key).ok()
// }

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId,AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
		get_from_seed::<AuthorityDiscoveryId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {

	
	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		Some(
			serde_json::from_str(
				"{\"tokenDecimals\": 18, \"tokenSymbol\": \"FIRE\"}",
			)
			.expect("Provided valid json map"),
		),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
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
				],
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}


pub fn my_testnet_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::from_genesis(
		// Name
		"Fireconfig",
		// ID
		"fire_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("vehicle twist pear mutual shuffle dream mind twist deputy negative cover stone"), authority_keys_from_seed("merit decrease drift aunt bottom pear lunch fetch board ready solve need")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("vehicle twist pear mutual shuffle dream mind twist deputy negative cover stone"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("vehicle twist pear mutual shuffle dream mind twist deputy negative cover stone"),
					get_account_id_from_seed::<sr25519::Public>("merit decrease drift aunt bottom pear lunch fetch board ready solve need"),
					get_account_id_from_seed::<sr25519::Public>("office acid pave always cliff meat carpet wool robust upgrade lava recycle"),
					get_account_id_from_seed::<sr25519::Public>("govern admit judge actor fine during business flip lumber impact arena tiger"),
					// get_account_id_from_seed::<sr25519::Public>("Eve"),
					// get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					// get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					// get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					// get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					// get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					// get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					// get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

pub fn staging_network_config() -> ChainSpec {
	let boot_nodes = vec![];

	ChainSpec::from_genesis(
		"Substrate Stencil",
		"stencil_network",
		ChainType::Live,
		staging_network_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		None,
		Default::default(),
	)
}

fn staging_network_config_genesis() -> GenesisConfig {
	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId,AuthorityDiscoveryId,)> = vec![
		(
			//5FNCTJVDxfFnmUYKHqbJHjUi7UFbZ6pzC39sL6E5RVpB4vc9
			hex!["920c238572e2b31c2efd19dad1a5674c8188388d9a30d0d01847759a5dc64069"].into(),
			//5GgaLpTUcgbCTGnwVkCjSSzZ5jTaEPuxtWGRDhi8M1BP1hTs
			hex!["cc4c78c7f22298f17e0e2dcefb7cff85b30e19dc1699cb9d1de00e5ea65a433d"].into(),
			//5Fm7Lc3XDxxbH4LBKxn1tf44P1R5M5cm2vmuLZbUnPFLfu5p
			hex!["a3859016b0b17b7ed6a5b2efcb4ce0e2b6b56ec8594d416c0ea3685929f0a15c"].unchecked_into(),
			//5CyLUbfTe941tZDvQQ5AYPXZ6zzqwS987DTwFGnZ3yPFX5wB
			hex!["2824087e4d670acc6f2ac4251736b7fb581b5bff414437b6abc88dc118ea8d5c"].unchecked_into(),
			//5CahSqUXepwzCkbC7KNUSghUcuJxPDPKiQ4ow144Gb9qBPsX
			hex!["16dffa9a82c7bb62f0f9929407223bf156458a4e7970ec4007ab2da7fb389f7d"].unchecked_into(),
			//5CahSqUXepwzCkbC7KNUSghUcuJxPDPKiQ4ow144Gb9qBPsX
			hex!["16dffa9a82c7bb62f0f9929407223bf156458a4e7970ec4007ab2da7fb389f7d"].unchecked_into(),
			),
			(
			//5DP3mCevjzqrYhJgPpQFkpoERKg55K422u5KiRGPQaoJEgRH
			hex!["3a39a8d0654e0f52b2ee8202ed3488e7a82650dde0daadaddbc8ea825e408d13"].into(),
			//5HeTTicL5u17JCkDhAwcAHUXMGEzXbDLjPYmNC5ahKhwaLgt
			hex!["f6eb0cff5244d7437ed659ac34e6ea66daa857f3d1c580f452b8512ae7fdba0f"].into(),
			//5FKFid7kAaVFkfbpShH8dzw3wJipiuGPruTzc6WB2WKMviUX
			hex!["8fcd640390db86812092a0b2b244aac9d8375be2c0a3434eb9062b58643c60fb"].unchecked_into(),
			//5G4AdD8rQ6MHp2K1L7vF1E43eX69JMZDQ1vknonsALwGQMwW
			hex!["b087cc20818f98e543c55989afccd3ec28c57e425dae970d9dd63cad806c1f6d"].unchecked_into(),
			//5DknzWSQVCpo7bNf2NnBsjb529K2WVpvGv6Q3kn9RgcFgoeQ
			hex!["4acf560d0aa80158ee06971c0ebbf4e6a1a407e6de2df16a003a765b73e63d7b"].unchecked_into(),
			//5CahSqUXepwzCkbC7KNUSghUcuJxPDPKiQ4ow144Gb9qBPsX
			hex!["16dffa9a82c7bb62f0f9929407223bf156458a4e7970ec4007ab2da7fb389f7d"].unchecked_into(),
			),
			(
			//5DJQ1NXeThmu2N5yQHZUsY64Lmgm95nnchpRWi1nSBU2rgod
			hex!["36ad94b252606800bc80869baf453663ac2e9276e83f0401107384c053552f3e"].into(),
			//5EWQq4ns7miu8B8ArsspZ9KBHX6gwjJXptJq5dbLgQucZvdc
			hex!["6c1386fd76e4eec0365a439db0decae0d5d715e33db934bc44be28f73df50674"].into(),
			//5EUsrdaXAAJ87Y7yCRdrYKeyHdTYbSr9tJFCYEy12CNap2v2
			hex!["6ae80477725a1e4f3194fac59286662ea491c9461cb54909432228351be3474a"].unchecked_into(),
			//5FHCHVMPD9VfpzMcGVyL7gqkq2Rd9NomkHFHP8BzP8isUBnh
			hex!["8e3b579b007999dce44a28bb266f73b54e6f7ec219c495ae23fe0dc3c101e158"].unchecked_into(),
			//5GRarw8oivnRh5ViPC9kH6ztbPNiyrfb61BitYz2YzhoqS4L
			hex!["c0dd89e234665e119ac8396af69c37d1956ffbf4a0173c21ee5872fea2366026"].unchecked_into(),
			//5CahSqUXepwzCkbC7KNUSghUcuJxPDPKiQ4ow144Gb9qBPsX
			hex!["16dffa9a82c7bb62f0f9929407223bf156458a4e7970ec4007ab2da7fb389f7d"].unchecked_into(),
			),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5FemZuvaJ7wVy4S49X7Y9mj7FyTR4caQD5mZo2rL7MXQoXMi
		"a2bf32e50edd79c181888da41c80c67c191e9e6b29d3f2efb102ca0e2b53c558"
	]
	.into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(
		initial_authorities,
		vec![],
		root_key,
		endowed_accounts
	)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId,AuthorityDiscoveryId)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	mut endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
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
	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;
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
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary_unwrap().to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(x.0.clone(), x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone(),x.5.clone()))
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			// TODO: ForceEra::ForceNone
			..Default::default()
		},
		babe: BabeConfig { authorities: vec![], epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG) },
		grandpa: GrandpaConfig { authorities: vec![] },
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
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
		technical_membership: Default::default(),
		treasury: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		nomination_pools: NominationPoolsConfig {
			min_create_bond: 10 * DOLLARS,
			min_join_bond: 1 * DOLLARS,
			..Default::default()
		},

		evm: EVMConfig {
			accounts: {
				let mut map = BTreeMap::new();
				map.insert(
					// H160 address of Alice dev account
					// Derived from SS58 (42 prefix) address
					// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
					// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
					// Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
					H160::from_str("43e318e18f4C61460B5fb9E1c0bf26A1502044BD")
						.expect("internal H160 is valid; qed"),
					fp_evm::GenesisAccount {
						balance: U256::from_str("0xfffffffffffffffffffff")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("482E579993cA638c96Dd4D20df60836a2Fd7DcBe")
						.expect("internal H160 is valid; qed"),
					fp_evm::GenesisAccount {
						balance: U256::from_str("0xfffff")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map
			},
		},
		ethereum: EthereumConfig {},
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
	}
}
