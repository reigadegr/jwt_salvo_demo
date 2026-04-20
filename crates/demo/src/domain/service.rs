use anyhow::Result;
use sea_orm::DatabaseConnection;

use crate::domain::entity::User;
use crate::infrastructure::persistence::find_user_by_username;

/// 认证用户 — 返回认证成功的用户，失败返回 None
pub async fn authenticate(
    conn: &DatabaseConnection,
    username: &str,
    password: &str,
) -> Result<Option<User>> {
    let Some(user) = find_user_by_username(conn, username).await? else {
        return Ok(None);
    };
    if user.verify_password(password) {
        Ok(Some(user))
    } else {
        Ok(None)
    }
}
