#![allow(clippy::type_complexity)]

use crate::inherent_data::CreateInherentDataConfig;
use crate::main_chain_follower::DataSources;
use crate::tests::runtime_api_mock::TestApi;
use chain_params::SidechainParams;
use epoch_derivation::{EpochConfig, MainchainEpochConfig};
use hex_literal::hex;
use main_chain_follower_api::mock_services::TestDataSources;
use sc_consensus_aura::SlotDuration;
use sidechain_domain::*;
use sidechain_slots::{ScSlotConfig, SlotsPerEpoch};
use sp_core::offchain::{Duration, Timestamp};
use std::sync::Arc;

impl From<TestDataSources> for DataSources {
	fn from(value: TestDataSources) -> Self {
		Self { block: value.block, candidate: value.candidate, native_token: value.native_token }
	}
}

pub fn mock_sidechain_params() -> SidechainParams {
	SidechainParams {
		chain_id: 0,
		genesis_committee_utxo: UtxoId {
			tx_hash: McTxHash(hex!(
				"f17e6d3aa72095e04489d13d776bf05a66b5a8c49d89397c28b18a1784b9950e"
			)),
			index: UtxoIndex(0),
		},
		threshold_numerator: 2,
		threshold_denominator: 3,
		governance_authority: MainchainAddressHash(hex!(
			"00112233445566778899001122334455667788990011223344556677"
		)),
	}
}

pub fn test_slot_config() -> ScSlotConfig {
	ScSlotConfig {
		slots_per_epoch: SlotsPerEpoch(10),
		slot_duration: SlotDuration::from_millis(1000),
	}
}

pub fn test_epoch_config() -> EpochConfig {
	let sc_slot_config = test_slot_config();
	EpochConfig {
		mc: MainchainEpochConfig {
			first_epoch_timestamp_millis: Timestamp::from_unix_millis(0),
			first_epoch_number: 0,
			epoch_duration_millis: Duration::from_millis(
				u64::from(sc_slot_config.slots_per_epoch.0)
					* sc_slot_config.slot_duration.as_millis()
					* 10,
			),
			first_slot_number: 0,
		},
	}
}

pub fn test_client() -> Arc<TestApi> {
	Arc::new(TestApi::new(ScEpochNumber(2)))
}

pub fn test_create_inherent_data_config() -> CreateInherentDataConfig {
	CreateInherentDataConfig {
		epoch_config: test_epoch_config(),
		sc_slot_config: test_slot_config(),
		time_source: Arc::new(time_source::MockedTimeSource { current_time_millis: 30000 }),
	}
}
