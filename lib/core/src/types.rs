use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

use crate::traits::Service;

#[derive(Debug, Serialize, Deserialize)]
pub enum ChainType {
	#[serde(rename = "ethereum")]
	Ethereum,
	#[serde(rename = "substrate")]
	Substrate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainConfig {
	pub identifier: u8,
	pub name: String,
	pub from: String,
	#[serde(rename = "type")]
	pub typ: ChainType,
	pub endpoint: String,
	pub start_block: u64,
	pub extra: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BridgeConfig {
	pub chains: Vec<ChainConfig>,
}

#[derive(Debug, Clone)]
pub struct FungibleTransfer {
	pub source_chain_id: u8,
	pub destination_chain_id: u8,
	pub deposit_nonce: u64,
}

#[derive(Debug, Clone)]
pub enum Event {
	FungibleTransfer(FungibleTransfer),
	NonFungibleTransfer,
	Stop,
}

pub struct Services {
	pub reader: Box<dyn Service>,
	pub reader_sender: Sender<Event>,
	pub writer: Box<dyn Service>,
	pub writer_sender: Sender<Event>,
}
