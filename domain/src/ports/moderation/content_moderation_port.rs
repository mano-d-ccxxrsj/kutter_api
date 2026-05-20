use async_trait::async_trait;

#[async_trait]
pub trait ContentModerationPort: Send + Sync {
    async fn validate(
        &self,
        user_id: i32,
        field: &str,
        action: &str,
        target: &str,
        content: &str,
    ) -> Result<(), String>;
}