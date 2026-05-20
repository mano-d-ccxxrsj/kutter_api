pub trait PasswordHashPort: Send + Sync {
    fn hash_password(&self, password: &str) -> Result<String, String>;
    fn verify_password(&self, password: &str, password_hash: &str) -> Result<bool, String>;
}