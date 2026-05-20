use chrono::{Duration, Utc};
use domain::ports::security::session_token_port::SessionTokenPort;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::jwt::claims_type::SessionClaim;

pub struct JwtSessionTokenService {
    secret: String,
}

impl JwtSessionTokenService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl SessionTokenPort for JwtSessionTokenService {
    fn generate_session_token(&self, user_id: i32) -> Result<String, String> {
        let claims: SessionClaim = SessionClaim {
            sub: user_id,
            exp: (Utc::now() + Duration::days(365 * 20)).timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|error| error.to_string())
    }

    fn verify_session_token(&self, token: &str) -> Result<i32, String> {
        let claims: SessionClaim = decode::<SessionClaim>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|error| error.to_string())?
        .claims;

        Ok(claims.sub)
    }
}