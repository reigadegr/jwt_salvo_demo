use anyhow::Result;

use crate::{
    dto::{LoginRequest, LoginResponse, UserProfile},
    entity::User,
    repository::UserRepository,
    service::AuthService,
};

/// 认证门面 - 对外统一接口
///
/// Facade: 协调领域对象，提供简化的应用层接口
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

    /// 登录 - 返回 token
    pub fn login(&self, req: &LoginRequest) -> Result<Option<LoginResponse>> {
        let Some(user) = self.auth_service.authenticate(&req.username, &req.password) else {
            return Ok(None);
        };

        let token = generate_token(&user)?;
        Ok(Some(LoginResponse { token }))
    }

    /// 获取用户信息
    #[must_use]
    pub fn get_profile(user: &User) -> UserProfile {
        UserProfile {
            username: user.username().to_string(),
            role: user.role().to_string(),
        }
    }
}

/// Token 生成 - 基础设施层关注点
fn generate_token(user: &User) -> Result<String> {
    use my_jwt::jwt_utils::secret_key::get_jwt_utils;

    let jwt_utils = get_jwt_utils()?;
    jwt_utils
        .generate_token(user.role().as_str(), user.username().as_str())
        .map_err(Into::into)
}
