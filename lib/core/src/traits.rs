use async_trait::async_trait;

use crate::types::Services;

#[async_trait]
pub trait Service {
	async fn run(self: Box<Self>);
}

#[async_trait]
pub trait Builder {
	async fn build(self) -> anyhow::Result<Services>;
}
