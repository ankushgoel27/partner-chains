use crate::chain_spec::*;
use chain_params::SidechainParams;
use sc_service::ChainType;
use sidechain_domain::*;
use sidechain_runtime::{
	AccountId, AuraConfig, BalancesConfig, GrandpaConfig, NativeTokenManagementConfig,
	PolkadotSessionStubForGrandpaConfig, RuntimeGenesisConfig, SessionCommitteeManagementConfig,
	SidechainConfig, SudoConfig, SystemConfig,
};
use sidechain_slots::SlotsPerEpoch;
use sp_core::bytes::from_hex;
use sp_core::{ed25519, sr25519};
use std::str::FromStr;

pub fn authority_keys(
	aura_pub_key: &str,
	grandpa_pub_key: &str,
	sidechain_pub_key: &str,
) -> AuthorityKeys {
	let aura_pk = sr25519::Public::from_raw(from_hex(aura_pub_key).unwrap().try_into().unwrap());
	let granda_pk =
		ed25519::Public::from_raw(from_hex(grandpa_pub_key).unwrap().try_into().unwrap());
	let sidechain_pk = sidechain_domain::SidechainPublicKey(from_hex(sidechain_pub_key).unwrap());

	let session_keys = (aura_pk, granda_pk).into();
	AuthorityKeys { session: session_keys, cross_chain: sidechain_pk.try_into().unwrap() }
}

pub fn development_config() -> Result<ChainSpec, EnvVarReadError> {
	Ok(ChainSpec::builder(runtime_wasm(), None)
		.with_name("Development")
		.with_id("dev")
		.with_chain_type(ChainType::Development)
		.with_genesis_config(testnet_genesis(
			// Initial PoA authorities
			vec![
				//alice public keys
				authority_keys(
					"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
					"0x88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee",
					"0x020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1",
				),
			],
			// Sudo account
			Some(get_account_id_from_seed::<sr25519::Public>(
				"assist draw loud island six improve van gas slam urban penalty lyrics",
			)),
			// Pre-funded accounts
			vec![
				AccountId::from_str(
					"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
				)
				.unwrap(),
				AccountId::from_str(
					"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
				)
				.unwrap(),
				// SDETs test accounts, keys are in https://github.com/input-output-hk/sidechains-tests/tree/master/secrets
				// negative-test
				AccountId::from_str("5F1N52dZx48UpXNLtcCzSMHZEroqQDuYKfidg46Tp37SjPcE").unwrap(),
			],
			true,
		)?)
		.build())
}

pub fn testnet_initial_authorities() -> Vec<AuthorityKeys> {
	vec![
		//alice public keys
		authority_keys(
			"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
			"0x88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee",
			"0x020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1",
		),
		//bob public keys
		authority_keys(
			"0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
			"0xd17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae69",
			"0x0390084fdbf27d2b79d26a4f13f0ccd982cb755a661969143c37cbc49ef5b91f27",
		),
		//charlie public keys
		authority_keys(
			"0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22",
			"0x439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234f",
			"0x0389411795514af1627765eceffcbd002719f031604fadd7d188e2dc585b4e1afb",
		),
	]
}

pub fn testnet_endowed_accounts() -> Vec<AccountId> {
	vec![
		AccountId::from_str("0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d")
			.unwrap(),
		AccountId::from_str("0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48")
			.unwrap(),
		AccountId::from_str("0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22")
			.unwrap(),
		AccountId::from_str("0x306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20")
			.unwrap(),
		AccountId::from_str("0xe659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e")
			.unwrap(),
		AccountId::from_str("0x1cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c")
			.unwrap(),
		AccountId::from_str("0x2c4ed1038f6e4131c21b6b89885ed232c5b81bae09009376e9079cc8aa518a1c")
			.unwrap(),
		AccountId::from_str("0x9cedc9f7b926191f64d68ee77dd90c834f0e73c0f53855d77d3b0517041d5640")
			.unwrap(),
		AccountId::from_str("5CtLD1M83vbXs42XPYyvygkUg6BxxMRBAiNqg1XD5qe7iW8g").unwrap(),
		testnet_sudo_key(),
		// SDETs test accounts, keys are in https://github.com/input-output-hk/sidechains-tests/tree/master/secrets
		// negative-test
		AccountId::from_str("5F1N52dZx48UpXNLtcCzSMHZEroqQDuYKfidg46Tp37SjPcE").unwrap(),
	]
}

pub fn testnet_sudo_key() -> AccountId {
	get_account_id_from_seed::<sr25519::Public>(
		"assist draw loud island six improve van gas slam urban penalty lyrics",
	)
}

pub fn local_testnet_config() -> Result<ChainSpec, EnvVarReadError> {
	Ok(ChainSpec::builder(runtime_wasm(), None)
		.with_name("Local Testnet")
		.with_id("local_testnet")
		.with_chain_type(ChainType::Local)
		.with_genesis_config(testnet_genesis(
			// Initial PoA authorities
			testnet_initial_authorities(),
			// Sudo account
			Some(testnet_sudo_key()),
			// Pre-funded accounts
			testnet_endowed_accounts(),
			true,
		)?)
		.build())
}

/// Configure initial storage state for FRAME modules.
pub fn testnet_genesis(
	initial_authorities: Vec<AuthorityKeys>,
	root_key: Option<AccountId>,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> Result<serde_json::Value, EnvVarReadError> {
	let config = RuntimeGenesisConfig {
		system: SystemConfig { ..Default::default() },
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		aura: AuraConfig { authorities: vec![] },
		grandpa: GrandpaConfig { authorities: vec![], ..Default::default() },
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		transaction_payment: Default::default(),
		polkadot_session_stub_for_grandpa: PolkadotSessionStubForGrandpaConfig {
			keys: initial_authorities
				.iter()
				.map(|authority_keys| {
					(
						authority_keys.cross_chain.clone().into(),
						authority_keys.cross_chain.clone().into(),
						authority_keys.session.clone(),
					)
				})
				.collect(),
		},
		sidechain: SidechainConfig {
			params: SidechainParams {
				chain_id: from_var_or("CHAIN_ID", 1)?,
				threshold_numerator: from_var_or("THRESHOLD_NUMERATOR", 2)?,
				threshold_denominator: from_var_or("THRESHOLD_DENOMINATOR", 3)?,
				genesis_committee_utxo: from_var::<UtxoId>("GENESIS_COMMITTEE_UTXO")?,
				governance_authority: from_var::<MainchainAddressHash>("GOVERNANCE_AUTHORITY")?,
			},
			slots_per_epoch: SlotsPerEpoch(from_var_or("SLOTS_PER_EPOCH", 10)?),
			..Default::default()
		},
		session_committee_management: SessionCommitteeManagementConfig {
			initial_authorities: initial_authorities
				.into_iter()
				.map(|keys| (keys.cross_chain, keys.session))
				.collect(),
			main_chain_scripts: read_mainchain_scripts_from_env()?,
		},
		native_token_management: NativeTokenManagementConfig {
			main_chain_scripts: read_native_token_main_chain_scripts_from_env()?,
			..Default::default()
		},
	};

	Ok(serde_json::to_value(config).expect("Genesis config must be serialized correctly"))
}
