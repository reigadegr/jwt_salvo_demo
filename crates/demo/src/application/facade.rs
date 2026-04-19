use anyhow::Result;

use crate::application::dto::{LoginRequest, LoginResponse, UserProfile};
use crate::domain::{entity::User, repository::UserRepository, service::AuthService};

/// 认证门面 — 应用层统一入口
///
/// Facade: 协调领域对象，提供简化的用例接口。
/// 基础设施关注点（如 JWT 生成）通过私有函数隔离。
pub struct AuthFacade<R: UserRepository> {
    auth_service: AuthService<R>,
}

impl<R: UserRepository> AuthFacade<R> {
    #[must_use]
    pub const fn new(user_repo: R) -> Self {
        Self {
            auth_service: AuthService::new(user_repo),
        }
    }

    /// 登录用例 — 返回 token
    pub async fn login(&self, req: &LoginRequest) -> Result<Option<LoginResponse>> {
        let Some(user) = self
            .auth_service
            .authenticate(&req.username, &req.password)
            .await?
        else {
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
}

/// Token 生成 — 基础设施关注点，通过函数隔离
fn generate_token(user: &User) -> Result<String> {
    use my_jwt::jwt_utils::secret_key::get_jwt_utils;

    let jwt_utils = get_jwt_utils()?;
    jwt_utils
        .generate_token(user.role().as_str(), user.username().as_str())
        .map_err(Into::into)
}
