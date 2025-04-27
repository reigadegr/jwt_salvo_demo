use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // 用户ID
    pub exp: i64,    // 过期时间
    pub iat: i64,    // 签发时间
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
