use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// role: 用户角色类型
// username: 用户ID
// exp: 过期时间
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub role: Arc<str>,
    pub username: String,
    pub exp: i64,
}

impl Claims {
    #[must_use]
    pub fn new(role: &str, username: &str, exp: DateTime<Utc>) -> Self {
        Self {
            role: Arc::from(role),
            username: username.to_owned(),
            exp: exp.timestamp(),
        }
    }
}
