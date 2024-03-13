use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

use crate::types::{BuilderError, Event, Services};

#[async_trait]
pub trait Service {
	async fn run(self: Box<Self>);
}

#[async_trait]
pub trait Builder {
	async fn build(self) -> Result<Services, BuilderError>;
}
