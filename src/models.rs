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
    pub fn new(role: &str, sub: &str, exp: DateTime<Utc>) -> Self {
        Self {
            role: role.to_owned(),
            sub: sub.to_owned(),
            exp: exp.timestamp(),
            iat: Utc::now().timestamp(),
        }
    }
}
