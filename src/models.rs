use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// role: 用户角色类型
// sub: 用户ID
// exp: 过期时间
// iat: 签发时间
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub role: String,
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(role: String, sub: String, exp: DateTime<Utc>) -> Self {
        Self {
            role,
            sub,
            exp: exp.timestamp(),
            iat: Utc::now().timestamp(),
        }
    }
}
