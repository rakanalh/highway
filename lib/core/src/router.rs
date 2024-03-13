use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{traits::Service, types::Event};

struct ChainProperties {
	name: String,
	sender: Sender<Event>,
}

pub struct Router {
	receiver: Receiver<Event>,
	chains: HashMap<u8, ChainProperties>,
}

impl Router {
	pub fn new() -> (Self, Sender<Event>) {
		let (sender, receiver) = mpsc::channel(1);
		(Self { receiver, chains: HashMap::new() }, sender)
	}

	pub fn add_chain(&mut self, identifier: u8, name: String, sender: Sender<Event>) {
		self.chains.insert(identifier, ChainProperties { name, sender });
	}

	pub async fn send(&self, destination_identifier: u8, event: Event) {
		// Route message
		if let Some(properties) = self.chains.get(&destination_identifier) {
			let _ = properties.sender.send(event).await;
		}
	}
}

#[async_trait]
impl Service for Router {
	async fn run(mut self: Box<Self>) {
		while let Some(event) = self.receiver.recv().await {
			println!("Got event in router: {:?}", event);
		}
	}
}
