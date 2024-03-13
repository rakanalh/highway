use ethers_contract::abigen;
use ethers_core::types::Address;

// mod connection;
pub mod monitors;
// mod temp;
pub mod builder;
pub mod contracts;
pub mod types;
pub mod writer;

#[derive(Clone)]
pub struct EthereumConfig {
	bridge_address: Address,
	erc20_handler: Address,
	erc721_handler: Address,
	from_address: Address,
}
