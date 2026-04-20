use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// JWT 令牌中的声明信息
/// role: 用户角色类型
/// username: 用户ID
/// exp: 过期时间
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub role: String,
    pub username: String,
    pub exp: i64,
}

impl Claims {
    #[must_use]
    pub fn new(role: &str, username: &str, exp: DateTime<Utc>) -> Self {
        Self {
            role: role.to_owned(),
            username: username.to_owned(),
            exp: exp.timestamp(),
        }
    }

    /// 转换为用户信息（用于存入 depot）
    #[must_use]
    pub fn to_user_info(&self) -> UserInfo {
        UserInfo {
            role: self.role.clone(),
            username: self.username.clone(),
        }
    }
}

/// 存入 depot 的用户信息
/// 仅包含 role 和 username，不包含 JWT 相关的 exp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub role: String,
    pub username: String,
}
