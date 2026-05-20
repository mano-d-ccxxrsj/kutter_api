pub trait PublicKeyGeneratorPort: Send + Sync {
    fn generate_public_key(&self) -> [u8; 32];
}