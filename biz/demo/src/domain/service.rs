use anyhow::Result;

use crate::domain::{entity::User, repository::UserRepository};

/// 认证用户 — 依赖仓储抽象，返回认证成功的用户，失败返回 None
pub async fn authenticate<R: UserRepository + Sync>(
    repo: &R,
    username: &str,
    password: &str,
) -> Result<Option<User>> {
    let Some(user) = repo.find_by_username(username).await? else {
        return Ok(None);
    };
    if user.verify_password(password) {
        Ok(Some(user))
    } else {
        Ok(None)
    }
}
