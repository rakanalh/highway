use std::sync::Arc;

use ethers_contract::{abigen, EthEvent};
use ethers_core::{
	// abi::Bytes,
	// types::{Address, Block, Filter, H160, H256, U256, U64},
	types::{Address, U256, U64},
};
use ethers_providers::{Http, Middleware, Provider};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, EthEvent)]
pub struct Transfer {
	#[ethevent(indexed)]
	pub from: Address,
	#[ethevent(indexed)]
	pub to: Address,
	pub tokens: U256,
}

const BRIDGE_ADDRESS: &str = "0xCaF65AB2eC9B39403966991eb34B1e8B9E44C041";
abigen!(
	IBridge,
	r#"[
        event Deposit(uint8 indexed destinationChainID, bytes32 indexed resourceID, uint64 indexed depositNonce)
	    event Proposal(uint8 indexed originChainID,uint64 indexed depositNonce,uint8 indexed status,bytes32 resourceID,bytes32 dataHash)
        event ProposalVote(uint8 indexed originChainID,uint64 indexed depositNonce,uint8 indexed status,bytes32 resourceID)
    ]"#,
);

pub struct HttpBlockQuery {
	provider: Arc<Provider<Http>>,
}

impl HttpBlockQuery {
	pub fn new(provider: Provider<Http>) -> Self {
		Self { provider: Arc::new(provider) }
	}

	pub async fn latest_block(&self) -> U64 {
		self.provider.get_block_number().await.unwrap()
	}

	pub async fn get(&self, block_number: U64) -> Vec<Transfer> {
		// let event = Transfer::new::<_, Provider<Http>>(Filter::new(), Arc::new(&self.provider));
		// let mut transfers = event.subscribe().await?.take(5);
		// while let Some(log) = transfers.next().await {
		// 	println!("Transfer: {:?}", log);
		// }
		let address: Address = BRIDGE_ADDRESS.parse().unwrap();
		let contract = IBridge::new(address, self.provider.clone());
		let events = contract
			.events()
			.from_block(block_number)
			.to_block(block_number)
			.query()
			.await
			.unwrap();
		for event in events {
			match event {
				IBridgeEvents::DepositFilter(f) => {
					println!("Deposit event: {f:?}");
				},
				IBridgeEvents::ProposalFilter(f) => {
					println!("Proposal event: {f:?}");
				},
				IBridgeEvents::ProposalVoteFilter(f) => {
					println!("Proposal Vote event: {f:?}");
				},
			}
		}
		// for event in events.stream().await {
		// 	println!("Event {:?}", event);
		// }
		// println!("Fetching logs for {:?}", block_number);
		// let filter = Filter::new()
		// 	.address(BRIDGE_ADDRESS.parse::<Address>().unwrap())
		// 	.event("Deposit(uint8,bytes32,uint64)")
		// 	.from_block(block_number)
		// 	.to_block(block_number + 1);
		// let logs = self.provider.get_logs(&filter).await.unwrap();
		// println!("Fetched logs: {:?}", logs);
		// for log in logs {
		// 	println!("Log: {:?}", log);
		// 	let destination_chain = U256::from_big_endian(log.topics[1].as_bytes()).as_u64();
		// 	let resource_id = Bytes::from(log.topics[2].as_bytes());
		// 	let deposit_nonce = U256::from_big_endian(&log.topics[3].as_bytes()).as_u64();
		// 	println!("Destination: {:?}", destination_chain);
		// 	println!("Resource ID: {:?}", resource_id);
		// 	println!("Deposit Nonce: {:?}", deposit_nonce);
		// }
		vec![]
	}
}
