use alloy::primitives::Address;

pub mod builder;
pub mod reader;
pub mod services;
pub mod types;
pub mod writer;

#[derive(Clone)]
pub struct EthereumConfig {
	bridge_address: Address,
	erc20_handler: Address,
	erc721_handler: Address,
	from_address: Address,
}
