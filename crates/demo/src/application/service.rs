use anyhow::Result;

use crate::{
    application::dto::{LoginRequest, LoginResponse, UserProfile},
    domain::{entity::User, repository::UserRepository, service::authenticate},
};

/// 登录用例
pub async fn login<R: UserRepository + Sync>(
    repo: &R,
    req: &LoginRequest,
) -> Result<Option<LoginResponse>> {
    let Some(user) = authenticate(repo, &req.username, &req.password).await? else {
        return Ok(None);
    };

    let token = generate_token(&user)?;
    Ok(Some(LoginResponse { token }))
}

/// 获取用户信息 — 领域对象转 DTO
#[must_use]
pub fn get_profile(user: &User) -> UserProfile {
    UserProfile {
        username: user.username().to_string(),
        role: user.role().to_string(),
    }
}

fn generate_token(user: &User) -> Result<String> {
    use my_jwt_core::jwt_utils::secret_key::get_jwt_utils;

    let jwt_utils = get_jwt_utils()?;
    jwt_utils
        .generate_token(user.role().as_str(), user.username().as_str())
        .map_err(Into::into)
}
