use async_trait::async_trait;
use std::time::Duration;
use tokio::time::sleep;

#[async_trait]
pub trait MessageRepository: Send + Sync {
    async fn fetch_message(&self) -> String;
}

#[derive(Clone)]
pub struct MessageRepoImpl;

#[async_trait]
impl MessageRepository for MessageRepoImpl {
    async fn fetch_message(&self) -> String {
        // Simulasi kerja memori: alokasi 10MB di heap
        let _buffer = vec![0u8; 300 * 1024]; // 100 KB
        sleep(Duration::from_millis(300)).await;
        "Hello from repository!".to_string()
    }
}
