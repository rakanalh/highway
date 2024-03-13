use std::process::exit;

use async_trait::async_trait;
use derive_more::Deref;
use ethers_contract::abigen;
use ethers_core::types::{Address, U64};
use ethers_providers::{Http, JsonRpcClient, Middleware, Provider, StreamExt, Ws};
use highway_core::{
	traits::Service,
	types::{Event, FungibleTransfer},
};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{
	contracts::{IBridge, IBridgeEvents, IErc20Handler},
	types::DepositEvent,
	EthereumConfig,
};

pub struct BaseBlocksMonitor {
	router: Sender<Event>,
	config: EthereumConfig,
	start_block: u64,
	block_confirmations: u16,
}

impl BaseBlocksMonitor {
	pub async fn fetch_events<C: JsonRpcClient + Clone + 'static>(
		&self,
		provider: Provider<C>,
		block_number: u64,
	) {
		let bridge_contract = IBridge::new(self.config.bridge_address, provider.clone().into());
		let erc20_handler_contract =
			IErc20Handler::new(self.config.erc20_handler, provider.clone().into());

		let events = bridge_contract
			.events()
			.from_block(block_number)
			.to_block(block_number)
			.query()
			.await
			.unwrap();

		println!("Got events: {:?}", events.len());
		for event in &events {
			match event {
				IBridgeEvents::DepositFilter(f) => {
					println!("Deposit event: {f:?}");
					match erc20_handler_contract
						.get_deposit_record(f.deposit_nonce, f.destination_chain_id)
						.await
					{
						Ok(deposit_record) => {
							println!("Deposit record: {:?}", deposit_record);
						},
						Err(e) => {
							println!("Error: {:?}", e);
						},
					};
				},
				IBridgeEvents::ProposalFilter(f) => {
					println!("Proposal event: {f:?}");
				},
				IBridgeEvents::ProposalVoteFilter(f) => {
					println!("Proposal Vote event: {f:?}");
				},
			}
		}

		if events.len() > 0 {
			self.router
				.send(Event::FungibleTransfer(FungibleTransfer {
					source_chain_id: 1,
					destination_chain_id: 2,
					deposit_nonce: 5,
				}))
				.await;
			println!("Got events");
		}
	}
}

#[derive(Deref)]
pub struct HttpBlocksMonitor {
	connection: Provider<Http>,
	receiver: Receiver<Event>,
	#[deref]
	inner: BaseBlocksMonitor,
}

impl HttpBlocksMonitor {
	pub fn new(
		router: Sender<Event>,
		connection: Provider<Http>,
		config: EthereumConfig,
		start_block: u64,
		block_confirmations: u16,
	) -> (Self, Sender<Event>) {
		let (sender, receiver) = mpsc::channel(10);
		(
			Self {
				connection,
				receiver,
				inner: BaseBlocksMonitor { router, config, start_block, block_confirmations },
			},
			sender,
		)
	}
}

#[async_trait]
impl Service for HttpBlocksMonitor {
	async fn run(self: Box<Self>) {
		println!("Hello world 2");
		let _start_block_number = self.start_block;

		loop {}
	}
}

#[derive(Deref)]
pub struct WsBlocksMonitor {
	connection: Provider<Ws>,
	receiver: Receiver<Event>,
	#[deref]
	inner: BaseBlocksMonitor,
}

impl WsBlocksMonitor {
	pub fn new(
		router: Sender<Event>,
		connection: Provider<Ws>,
		config: EthereumConfig,
		start_block: u64,
		block_confirmations: u16,
	) -> (Self, Sender<Event>) {
		let (sender, receiver) = mpsc::channel(10);
		(
			Self {
				connection,
				receiver,
				inner: BaseBlocksMonitor { router, config, start_block, block_confirmations },
			},
			sender,
		)
	}
}

#[async_trait]
impl Service for WsBlocksMonitor {
	async fn run(self: Box<Self>) {
		let mut current_block_number = self.start_block;

		loop {
			let latest_block = self.connection.get_block_number().await.unwrap();
			if U64::from(current_block_number) >= latest_block {
				break;
			}

			// Fetch events
			println!("Fetching block: {:?}", current_block_number);
			self.inner.fetch_events(self.connection.clone(), current_block_number).await;

			current_block_number += 1;
		}

		let mut stream = self.connection.subscribe_blocks().await.unwrap();
		while let Some(block) = stream.next().await {
			println!("{:?}", block.hash);
		}
	}
}
