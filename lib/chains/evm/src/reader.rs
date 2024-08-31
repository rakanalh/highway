use alloy::{primitives::U64, providers::RootProvider, transports::Transport};
use tracing::info;

use crate::{
	types::{Bridge, Erc20Handler},
	EthereumConfig,
};

pub struct Reader<T: Transport> {
	config: EthereumConfig,
	provider: RootProvider<T>,
}

impl<T: Transport + Clone> Reader<T> {
	pub fn new(config: EthereumConfig, provider: RootProvider<T>) -> Self {
		Self { config, provider }
	}

	pub async fn fetch_events(
		&self,
		from_block_number: U64,
		to_block_number: U64,
	) -> anyhow::Result<Vec<u64>> {
		info!(
			from_block_number = from_block_number.to::<u64>(),
			to_block_number = to_block_number.to::<u64>(),
			"Query block",
		);

		let bridge_contract = Bridge::new(self.config.bridge_address, self.provider.clone());
		let erc20_handler_contract =
			Erc20Handler::new(self.config.erc20_handler, self.provider.clone());

		// Setup a filter for the Increment and Decrement events.
		let mut deposit_filter = bridge_contract
			.Deposit_filter()
			.from_block(from_block_number)
			.to_block(to_block_number)
			.query()
			.await;

		Ok(vec![])
	}
}
