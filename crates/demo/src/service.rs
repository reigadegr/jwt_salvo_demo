use anyhow::Result;

use crate::{entity::User, repository::UserRepository};

/// 认证服务 - 纯领域逻辑
///
/// Domain Service: 无状态的领域操作，不自然属于任何实体或值对象
pub struct AuthService<R: UserRepository> {
    user_repo: R,
}

impl<R: UserRepository> AuthService<R> {
    #[must_use]
    pub const fn new(user_repo: R) -> Self {
        Self { user_repo }
    }

    /// 认证用户
    ///
    /// 返回认证成功的用户，失败返回 None
    pub async fn authenticate(&self, username: &str, password: &str) -> Result<Option<User>> {
        let Some(user) = self.user_repo.find_by_username(username).await? else {
            return Ok(None);
        };
        if user.verify_password(password) {
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
}
