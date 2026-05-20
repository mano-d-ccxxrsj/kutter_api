use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionClaim {
    pub sub: i32,
    pub exp: usize,
}