use std::sync::Arc;

use async_trait::async_trait;
use crate::repository::MessageRepository;

#[async_trait]
pub trait MessageUsecase: Send + Sync {
    async fn get_message(&self) -> String;
}

pub struct MessageUsecaseImpl {
    repo: Arc<dyn MessageRepository>,
}

impl MessageUsecaseImpl {
    pub fn new(repo: Arc<dyn MessageRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl MessageUsecase for MessageUsecaseImpl {
    async fn get_message(&self) -> String {
        self.repo.fetch_message().await
    }
}

impl Drop for MessageUsecaseImpl {
    fn drop(&mut self) {
        println!("🗑️ MessageUsecaseImpl has been dropped");
    }
}