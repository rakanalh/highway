use alloy::{providers::RootProvider, transports::Transport};

use crate::EthereumConfig;

pub struct Writer<T: Transport> {
	config: EthereumConfig,
	provider: RootProvider<T>,
}

impl<T: Transport + Clone> Writer<T> {
	pub fn new(config: EthereumConfig, provider: RootProvider<T>) -> Self {
		Self { config, provider }
	}

	pub fn transact(&self) -> anyhow::Result<()> {
		Ok(())
	}
}
