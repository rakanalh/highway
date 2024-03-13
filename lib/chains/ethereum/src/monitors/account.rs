use async_trait::async_trait;
use derive_more::Deref;
use ethers_core::types::{Address, U256};
use ethers_providers::{Http, Middleware, Provider, Ws};
use highway_core::{traits::Service, types::Event};
use std::time::Duration;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::{error, warn};

pub struct BaseAccountMonitor {
	account: Address,
	threshold: U256,
	timeout: Duration,
}

#[derive(Deref)]
pub struct HttpAccountMonitor {
	connection: Provider<Http>,
	receiver: Receiver<Event>,
	#[deref]
	inner: BaseAccountMonitor,
}

impl HttpAccountMonitor {
	pub fn new(
		connection: Provider<Http>,
		account: Address,
		threshold: U256,
		timeout: Duration,
	) -> (Self, Sender<Event>) {
		let (sender, receiver) = mpsc::channel(10);
		(
			Self {
				connection,
				receiver,
				inner: BaseAccountMonitor { account, threshold, timeout },
			},
			sender,
		)
	}
}

#[async_trait]
impl Service for HttpAccountMonitor {
	async fn run(self) {
		loop {
			match self.connection.get_balance(self.account, None).await {
				Ok(balance) => {
					if balance < self.threshold {
						warn!("Account is running out of balance");
					}
					let _ = tokio::time::sleep(self.timeout);
				},
				Err(e) => {
					error!("Could not fetch balance because: {:?}", e)
				},
			}
		}
	}
}

#[derive(Deref)]
pub struct WsAccountMonitor {
	connection: Provider<Ws>,
	receiver: Receiver<Event>,
	#[deref]
	inner: BaseAccountMonitor,
}

impl WsAccountMonitor {
	pub fn new(
		connection: Provider<Ws>,
		account: Address,
		threshold: U256,
		timeout: Duration,
	) -> (Self, Sender<Event>) {
		let (sender, receiver) = mpsc::channel(10);
		(
			Self {
				connection,
				receiver,
				inner: BaseAccountMonitor { account, threshold, timeout },
			},
			sender,
		)
	}
}

#[async_trait]
impl Service for WsAccountMonitor {
	async fn run(self) {
		loop {
			match self.connection.get_balance(self.account, None).await {
				Ok(balance) => {
					if balance < self.threshold {
						warn!("Account is running out of balance");
					}
					let _ = tokio::time::sleep(self.timeout);
				},
				Err(e) => {
					error!("Could not fetch balance because: {:?}", e)
				},
			}
		}
	}
}
