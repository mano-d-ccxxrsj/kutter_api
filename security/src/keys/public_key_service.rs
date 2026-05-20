use domain::ports::security::public_key_generator_port::PublicKeyGeneratorPort;
use rand::rngs::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey};

pub struct X25519PublicKeyGenerator;

impl X25519PublicKeyGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl PublicKeyGeneratorPort for X25519PublicKeyGenerator {
    fn generate_public_key(&self) -> [u8; 32] {
        let secret: EphemeralSecret = EphemeralSecret::random_from_rng(OsRng);
        let public_key: PublicKey = PublicKey::from(&secret);

        public_key.to_bytes()
    }
}