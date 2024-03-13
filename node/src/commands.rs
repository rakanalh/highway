use highway_core::types::BridgeConfig;

use crate::reactor::Reactor;

pub async fn run(config: BridgeConfig) {
	let reactor = Reactor::new(config);

	reactor.run().await;
}
