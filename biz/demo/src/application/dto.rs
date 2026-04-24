use serde::{Deserialize, Serialize};

/// 登录请求 DTO
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 登录响应 DTO
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

/// 用户信息响应 DTO
#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub username: String,
    pub role: String,
}
