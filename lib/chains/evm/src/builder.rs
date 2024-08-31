use crate::{
	services::{reader::ReaderService, writer::WriterService},
	EthereumConfig,
};
use alloy::{
	primitives::Address,
	providers::{ProviderBuilder, WsConnect},
	transports::http::reqwest::Url,
};
use anyhow::anyhow;
use async_trait::async_trait;
use highway_core::{
	traits::Builder,
	types::{Event, Services},
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
	async fn build(self) -> anyhow::Result<Services> {
		let bridge_address: Address = match self.bridge_address {
			Some(address) => address.parse().map_err(|e| anyhow!("Bridge address: {:?}", e))?,
			None => return Err(anyhow!("Bridge address not specified.")),
		};
		let erc20_handler_address: Address = match self.erc20_handler_address {
			Some(address) => {
				address.parse().map_err(|e| anyhow!("ERC20 handler address: {:?}", e))?
			},
			None => return Err(anyhow!("ERC-20 handler address not specified.")),
		};
		let erc721_handler_address: Address = match self.erc721_handler_address {
			Some(address) => {
				address.parse().map_err(|e| anyhow!("ERC721 handler address: {:?}", e))?
			},
			None => return Err(anyhow!("ERC-721 handler address not specified.")),
		};
		let from_address: Address =
			self.from.parse().map_err(|e| anyhow!("Sender address: {:?}", e))?;

		let config = EthereumConfig {
			from_address,
			bridge_address,
			erc20_handler: erc20_handler_address,
			erc721_handler: erc721_handler_address,
		};

		let services = if self.endpoint.starts_with("http") || self.endpoint.starts_with("https") {
			let provider_url: Url = self
				.endpoint
				.parse()
				.map_err(|e| anyhow!("Failed to parse endpoint: {:?}", e))?;
			let provider = ProviderBuilder::new().on_http(provider_url);

			let (reader_service, reader_sender) = ReaderService::new(
				config.clone(),
				provider.clone(),
				self.router.clone(),
				self.start_block,
				self.block_confirmations.into(),
			);

			let (writer_service, writer_sender) = WriterService::new(config, provider);

			Services {
				reader: Box::new(reader_service),
				reader_sender,
				writer: Box::new(writer_service),
				writer_sender,
			}
		} else if self.endpoint.starts_with("ws") || self.endpoint.starts_with("wss") {
			let ws = WsConnect::new(self.endpoint);
			let provider = ProviderBuilder::new()
				.on_ws(ws)
				.await
				.map_err(|e| anyhow!("Could not connect via WS endpoint: {:?}", e))?;

			let (reader_service, reader_sender) = ReaderService::new(
				config.clone(),
				provider.clone(),
				self.router.clone(),
				self.start_block,
				self.block_confirmations.into(),
			);

			let (writer_service, writer_sender) = WriterService::new(config, provider);

			Services {
				reader: Box::new(reader_service),
				reader_sender,
				writer: Box::new(writer_service),
				writer_sender,
			}
		} else {
			return Err(anyhow!("Endpoint {0} is not supported", self.endpoint));
		};

		Ok(services)
	}
}
