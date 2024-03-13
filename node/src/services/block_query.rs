use highway_ethereum::types::U64;

pub struct BlockQueryService {
	block_query: HttpBlockQuery,
}

impl BlockQueryService {
	pub fn new(block_query: HttpBlockQuery) -> Self {
		Self { block_query }
	}

	pub async fn run(&self, start_block: Option<U64>, block_confirmations: U64) {
		let mut block_number = if let Some(start_block) = start_block {
			start_block
		} else {
			self.block_query.latest_block().await - block_confirmations
		};

		println!("Block: {:?}", block_number);

		while let events = self.block_query.get(block_number).await {
			println!("Block: {:?}", block_number);

			block_number += 1u64.into();
		}
	}
}
