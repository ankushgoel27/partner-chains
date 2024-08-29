pub mod types;

use derive_new::new;
use epoch_derivation::{EpochConfig, MainchainEpochDerivation};
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::{error::ErrorCode, ErrorObject, ErrorObjectOwned},
};
use sidechain_domain::McEpochNumber;
use sidechain_slots::SlotApi;
use sp_api::ProvideRuntimeApi;
use sp_core::offchain::Timestamp;
use sp_runtime::traits::Block as BlockT;
use sp_sidechain::{GetSidechainParams, GetSidechainStatus, SidechainDataSource};
use std::sync::Arc;
use time_source::*;
use types::*;

#[cfg(test)]
mod tests;

#[rpc(client, server, namespace = "sidechain")]
pub trait SidechainRpcApi<SidechainParams> {
	#[method(name = "getParams")]
	fn get_params(&self) -> RpcResult<SidechainParams>;

	/// Returns data related to the status of both the main chain and the sidechain, like their epochs or the timestamp associated to the next epoch.
	#[method(name = "getStatus")]
	async fn get_status(&self) -> RpcResult<GetStatusResponse>;
}

#[derive(new)]
pub struct SidechainRpc<C, Block, E> {
	client: Arc<C>,
	epoch_config: EpochConfig,
	block_data_source: Arc<dyn SidechainDataSource<Error = E> + Send + Sync>,
	time_source: Arc<dyn TimeSource + Send + Sync>,
	_marker: std::marker::PhantomData<Block>,
}

impl<C, B, E> SidechainRpc<C, B, E> {
	fn get_current_timestamp(&self) -> Timestamp {
		Timestamp::from_unix_millis(self.time_source.get_current_time_millis())
	}
}

#[async_trait]
impl<C, Block, SidechainParams, E> SidechainRpcApiServer<SidechainParams>
	for SidechainRpc<C, Block, E>
where
	Block: BlockT,
	SidechainParams: parity_scale_codec::Decode,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: GetBestHash<Block>,
	C::Api: SlotApi<Block> + GetSidechainParams<Block, SidechainParams> + GetSidechainStatus<Block>,
	E: std::error::Error + Send + Sync + 'static,
{
	fn get_params(&self) -> RpcResult<SidechainParams> {
		let api = self.client.runtime_api();
		let best_block = self.client.best_hash();

		let sidechain_params = api.sidechain_params(best_block).map_err(error_object_from)?;

		Ok(sidechain_params)
	}

	async fn get_status(&self) -> RpcResult<GetStatusResponse> {
		let api = self.client.runtime_api();
		let best_block = self.client.best_hash();

		let slot_config = api.slot_config(best_block).map_err(error_object_from)?;

		let current_timestamp = self.get_current_timestamp();
		let current_sidechain_slot =
			slot_config.slot_from_timestamp(current_timestamp.unix_millis());
		let current_sidechain_epoch = slot_config.epoch_number(current_sidechain_slot);
		let next_sidechain_epoch_timestamp = slot_config
			.epoch_start_time(current_sidechain_epoch.next())
			.ok_or(GetStatusRpcError::CannotConvertSidechainSlotToTimestamp)?;

		let latest_mainchain_block =
			self.block_data_source.get_latest_block_info().await.map_err(|err| {
				ErrorObject::owned(
					ErrorCode::InternalError.code(),
					format!("Internal error: GetLatestBlockResponse error '{:?}", err),
					None::<u8>,
				)
			})?;
		let next_mainchain_epoch_timestamp = self
			.epoch_config
			.mc
			.mainchain_epoch_to_timestamp(McEpochNumber(latest_mainchain_block.epoch.0 + 1));

		Ok(GetStatusResponse {
			sidechain: SidechainData {
				epoch: current_sidechain_epoch.0,
				slot: current_sidechain_slot.into(),
				next_epoch_timestamp: next_sidechain_epoch_timestamp,
			},
			mainchain: MainchainData {
				epoch: latest_mainchain_block.epoch.0,
				slot: latest_mainchain_block.slot.0,
				next_epoch_timestamp: next_mainchain_epoch_timestamp,
			},
		})
	}
}

pub fn error_object_from<T: std::fmt::Debug>(err: T) -> ErrorObjectOwned {
	ErrorObject::owned::<u8>(-1, format!("{err:?}"), None)
}
