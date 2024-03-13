use std::time::Duration;

use crate::{
	monitors::blocks::{HttpBlocksMonitor, WsBlocksMonitor},
	writer::{HttpWriter, WsWriter},
	EthereumConfig,
};
use async_trait::async_trait;
use ethers_core::types::{Address, U256};
use ethers_providers::{Http, Provider, Ws};
use highway_core::{
	traits::{Builder, Service},
	types::{BuilderError, Event, Services},
};
use tokio::sync::mpsc::Sender;

pub struct EthereumBuilder {
	router: Sender<Event>,
	identifier: u8,
	name: String,
	endpoint: String,
	from: String,
	bridge_address: Option<String>,
	erc20_handler_address: Option<String>,
	erc721_handler_address: Option<String>,
	start_block: u64,
	block_confirmations: u16,
}

impl EthereumBuilder {
	pub fn new(
		router: Sender<Event>,
		identifier: u8,
		name: String,
		endpoint: String,
		from: String,
		start_block: u64,
		block_confirmations: u16,
	) -> Self {
		Self {
			router,
			identifier,
			name,
			endpoint,
			from,
			bridge_address: None,
			erc20_handler_address: None,
			erc721_handler_address: None,
			start_block,
			block_confirmations,
		}
	}

	pub fn bridge(mut self, address: String) -> Self {
		self.bridge_address = Some(address);
		self
	}

	pub fn erc20_handler(mut self, address: String) -> Self {
		self.erc20_handler_address = Some(address);
		self
	}

	pub fn erc721_handler(mut self, address: String) -> Self {
		self.erc721_handler_address = Some(address);
		self
	}
}

#[async_trait]
impl Builder for EthereumBuilder {
	async fn build(self) -> Result<Services, BuilderError> {
		let bridge_address: Address = match self.bridge_address {
			Some(address) => address
				.parse()
				.map_err(|e| BuilderError::InvalidParam(format!("Bridge address: {:?}", e)))?,
			None =>
				return Err(BuilderError::InvalidParam(format!("Bridge address not specified."))),
		};
		let erc20_handler_address: Address = match self.erc20_handler_address {
			Some(address) => address.parse().map_err(|e| {
				BuilderError::InvalidParam(format!("ERC20 handler address: {:?}", e))
			})?,
			None =>
				return Err(BuilderError::InvalidParam(format!(
					"ERC-20 handler address not specified."
				))),
		};
		let erc721_handler_address: Address = match self.erc721_handler_address {
			Some(address) => address.parse().map_err(|e| {
				BuilderError::InvalidParam(format!("ERC721 handler address: {:?}", e))
			})?,
			None =>
				return Err(BuilderError::InvalidParam(format!(
					"ERC-721 handler address not specified."
				))),
		};
		let from_address: Address = self
			.from
			.parse()
			.map_err(|e| BuilderError::InvalidParam(format!("Sender address: {:?}", e)))?;

		let config = EthereumConfig {
			from_address,
			bridge_address,
			erc20_handler: erc20_handler_address,
			erc721_handler: erc721_handler_address,
		};

		let services = if self.endpoint.starts_with("http") || self.endpoint.starts_with("https") {
			let provider = Provider::<Http>::try_from(self.endpoint)
				.expect("could not instantiate HTTP Provider");

			let (blocks_monitor, blocks_monitor_sender) = HttpBlocksMonitor::new(
				self.router.clone(),
				provider.clone(),
				config.clone(),
				self.start_block,
				self.block_confirmations,
			);

			let (writer, writer_sender) = HttpWriter::new(provider.clone(), config.clone());
			Services {
				reader: Box::new(blocks_monitor),
				reader_sender: blocks_monitor_sender,
				writer: Box::new(writer),
				writer_sender,
			}
		} else if self.endpoint.starts_with("ws") || self.endpoint.starts_with("wss") {
			let provider = Provider::<Ws>::connect(self.endpoint)
				.await
				.expect("could not instantiate WS Provider");

			let (blocks_monitor, blocks_monitor_sender) = WsBlocksMonitor::new(
				self.router.clone(),
				provider.clone(),
				config.clone(),
				self.start_block,
				self.block_confirmations,
			);

			let (writer, writer_sender) = WsWriter::new(provider.clone(), config.clone());

			Services {
				reader: Box::new(blocks_monitor),
				reader_sender: blocks_monitor_sender,
				writer: Box::new(writer),
				writer_sender,
			}
		} else {
			return Err(BuilderError::InvalidEndpoint(format!(
				"Endpoint {0} is not supported",
				self.endpoint
			)))
		};

		Ok(services)
	}
}
