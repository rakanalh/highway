use std::pin::Pin;

use futures::{stream::FuturesUnordered, Future, FutureExt};
use highway_core::{
	router::Router,
	traits::{Builder, Service},
	types::{BridgeConfig, ChainType, Event},
};
use highway_ethereum::builder::EthereumBuilder;
use tokio::sync::mpsc::Sender;
use tracing::error;

pub struct Reactor {
	config: BridgeConfig,
}

impl Reactor {
	pub fn new(config: BridgeConfig) -> Self {
		Self { config }
	}

	pub async fn run(self) {
		let mut bridge_services: Vec<Pin<Box<dyn Future<Output = ()> + Send>>> = vec![];
		let (mut router, router_sender) = Router::new();

		for chain in self.config.chains {
			match chain.typ {
				ChainType::Ethereum => {
					let block_confirmations = match chain.extra.get("block_confirmations") {
						Some(bc) => match bc.parse::<u16>() {
							Ok(bc) => bc,
							Err(e) => {
								error!("Block confirmations should be a string: {}", bc);
								return;
							},
						},
						None => 12,
					};
					let bridge_address = match chain.extra.get("bridge") {
						Some(address) => address,
						None => {
							error!("No ethereum bridge address specified in `extra`");
							return;
						},
					};
					let erc20_handler_address = match chain.extra.get("erc20_handler") {
						Some(address) => address,
						None => {
							error!("No ethereum ERC-20 handler address specified in `extra`");
							return;
						},
					};
					let erc721_handler_address = match chain.extra.get("erc721_handler") {
						Some(address) => address,
						None => {
							error!("No ethereum ERC-721 handler address specified in `extra`");
							return;
						},
					};

					let builder_result = EthereumBuilder::new(
						router_sender.clone(),
						chain.identifier,
						chain.name.clone(),
						chain.endpoint,
						chain.from,
						chain.start_block,
						block_confirmations,
					)
					.bridge(bridge_address.clone())
					.erc20_handler(erc20_handler_address.clone())
					.erc721_handler(erc721_handler_address.clone())
					.build()
					.await;

					let sender = match builder_result {
						Ok(services) => {
							println!("OK");
							let reader_service = services.reader;
							let writer_service = services.writer;
							bridge_services.push(Box::pin(reader_service.run()));
							bridge_services.push(Box::pin(writer_service.run()));
							services.writer_sender
						},
						Err(e) => {
							error!("Failed to build ethereum's handler: {:?}", e);
							return;
						},
					};

					router.add_chain(chain.identifier, chain.name, sender);
				},
				ChainType::Substrate => {},
			}
		}

		bridge_services.push(Box::pin(Box::new(router).run()));
		println!("Waiting for {}", bridge_services.len());
		let _ = futures::future::join_all(bridge_services).await;
	}
}
