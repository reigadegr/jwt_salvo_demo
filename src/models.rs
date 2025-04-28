use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// sub: 用户ID
// exp: 过期时间
// iat: 签发时间
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(sub: String, exp: DateTime<Utc>) -> Self {
        Self {
            sub,
            exp: exp.timestamp(),
            iat: Utc::now().timestamp(),
        }
    }
}
