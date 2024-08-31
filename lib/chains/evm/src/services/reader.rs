use alloy::{
	primitives::U64,
	providers::{Provider, RootProvider},
	transports::Transport,
};
use futures::StreamExt;
use highway_core::{
	traits::Service,
	types::{Event, FungibleTransfer},
};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{reader::Reader, EthereumConfig};

pub struct ReaderService<T: Transport> {
	inner: Reader<T>,
	provider: RootProvider<T>,
	receiver: Receiver<Event>,
	router: Sender<Event>,
	start_block: u64,
	block_confirmations: u64,
}

impl<T: Transport + Clone> ReaderService<T> {
	pub fn new(
		config: EthereumConfig,
		provider: RootProvider<T>,
		router: Sender<Event>,
		start_block: u64,
		block_confirmations: u64,
	) -> (Self, Sender<Event>) {
		let (sender, receiver) = mpsc::channel(10);
		(
			Self {
				inner: Reader::new(config, provider.clone()),
				provider,
				receiver,
				router,
				start_block,
				block_confirmations,
			},
			sender,
		)
	}
}

#[async_trait::async_trait]
impl<T: Transport + Clone> Service for ReaderService<T> {
	async fn run(mut self: Box<Self>) {
		let mut last_block_number = U64::from(self.start_block);
		let mut block_monitor = self
			.provider
			.subscribe_blocks()
			.await
			.expect("Should be able to subscribe to blocks")
			.into_stream()
			.fuse();

		loop {
			tokio::select! {
				_signal = self.receiver.recv() => {
					return;
				},
				current_block = block_monitor.select_next_some() => {
					let current_block_number =
						U64::from(current_block.header.number.expect("Block should have a number"));
					let events = self.inner.fetch_events(last_block_number, current_block_number).await;
					println!("Events: {:?}", events);

					for event in events {
						self.router
							.send(Event::FungibleTransfer(FungibleTransfer {
								source_chain_id: 1,
								destination_chain_id: 2,
								deposit_nonce: 5,
							}))
							.await;
						println!("Got events");
					}

					last_block_number = current_block_number;
				}
			}
		}
	}
}
