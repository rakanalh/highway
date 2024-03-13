use ethers_contract::EthEvent;
pub use ethers_core::types::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, EthEvent)]
pub struct DepositEvent {
	#[ethevent(indexed)]
	pub destination_chain_id: u8,
	#[ethevent(indexed)]
	pub resource_id: U256,
	pub deposit_nonce: u64,
}
