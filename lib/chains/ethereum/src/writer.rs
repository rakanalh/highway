use async_trait::async_trait;
use ethers_providers::{Http, Provider, Ws};
use highway_core::{traits::Service, types::Event};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::EthereumConfig;

pub struct BaseWriter {
	config: EthereumConfig,
}

pub struct HttpWriter {
	inner: BaseWriter,
	connection: Provider<Http>,
	receiver: Receiver<Event>,
}

impl HttpWriter {
	pub fn new(connection: Provider<Http>, config: EthereumConfig) -> (Self, Sender<Event>) {
		let (sender, receiver) = mpsc::channel(10);
		(Self { connection, receiver, inner: BaseWriter { config } }, sender)
	}
}

#[async_trait]
impl Service for HttpWriter {
	async fn run(self: Box<Self>) {}
}

pub struct WsWriter {
	inner: BaseWriter,
	connection: Provider<Ws>,
	receiver: Receiver<Event>,
}

impl WsWriter {
	pub fn new(connection: Provider<Ws>, config: EthereumConfig) -> (Self, Sender<Event>) {
		let (sender, receiver) = mpsc::channel(10);
		(Self { connection, receiver, inner: BaseWriter { config } }, sender)
	}
}

#[async_trait]
impl Service for WsWriter {
	async fn run(self: Box<Self>) {}
}
