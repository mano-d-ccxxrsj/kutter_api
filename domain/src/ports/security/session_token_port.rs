pub trait SessionTokenPort: Send + Sync {
    fn generate_session_token(&self, user_id: i32) -> Result<String, String>;
    fn verify_session_token(&self, token: &str) -> Result<i32, String>;
}