use alloy::{providers::RootProvider, transports::Transport};
use highway_core::{traits::Service, types::Event};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{reader::Reader, writer::Writer, EthereumConfig};

pub struct WriterService<T: Transport> {
	inner: Writer<T>,
	receiver: Receiver<Event>,
}

impl<T: Transport + Clone> WriterService<T> {
	pub fn new(config: EthereumConfig, provider: RootProvider<T>) -> (Self, Sender<Event>) {
		let (sender, receiver) = mpsc::channel(10);
		(Self { inner: Writer::new(config, provider), receiver }, sender)
	}
}

#[async_trait::async_trait]
impl<T: Transport + Clone> Service for WriterService<T> {
	async fn run(self: Box<Self>) {
		let block_number = self.inner;
	}
}
