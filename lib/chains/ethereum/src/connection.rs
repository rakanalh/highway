use async_trait::async_trait;
use ethers_core::types::U256;
use ethers_providers::{Http, JsonRpcClient, Provider, PubsubClient, Ws};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Connection {
	Http(Provider<Http>),
	Ws(Provider<Ws>),
}

impl Connection {
	pub async fn create(endpoint: String) -> Result<Self, String> {
		if endpoint.starts_with("http") || endpoint.starts_with("https") {
			return Ok(Self::Http(
				Provider::<Http>::try_from(endpoint).expect("could not instantiate HTTP Provider"),
			));
		} else if endpoint.starts_with("ws") || endpoint.starts_with("wss") {
			return Ok(Self::Ws(
				Provider::<Ws>::connect(endpoint)
					.await
					.expect("could not instantiate WS Provider"),
			))
		} else {
			return Err(format!("Endpoint {endpoint} is not supported"))
		}
	}
}
