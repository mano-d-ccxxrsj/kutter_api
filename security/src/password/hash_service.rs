use bcrypt::{DEFAULT_COST, hash, verify};
use domain::ports::security::password_hash_port::PasswordHashPort;

pub struct BcryptPasswordHash;

impl BcryptPasswordHash {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordHashPort for BcryptPasswordHash {
    fn hash_password(&self, password: &str) -> Result<String, String> {
        hash(password, DEFAULT_COST).map_err(|error| error.to_string())
    }

    fn verify_password(&self, password: &str, password_hash: &str) -> Result<bool, String> {
        verify(password, password_hash).map_err(|error| error.to_string())
    }
}