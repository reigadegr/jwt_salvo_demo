use std::sync::Arc;

use crate::value_object::{Password, Role, Username};

/// 用户唯一标识
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(Arc<str>);

impl UserId {
    #[must_use]
    pub fn new(id: impl Into<Arc<str>>) -> Self {
        Self(id.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for UserId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// 用户实体 - 领域核心
///
/// Entity: 具有唯一标识，通过 Id 判断相等性
#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    username: Username,
    password: Password,
    role: Role,
}

impl User {
    #[must_use]
    pub const fn new(id: UserId, username: Username, password: Password, role: Role) -> Self {
        Self {
            id,
            username,
            password,
            role,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &UserId {
        &self.id
    }

    #[must_use]
    pub const fn username(&self) -> &Username {
        &self.username
    }

    #[must_use]
    pub const fn role(&self) -> &Role {
        &self.role
    }

    /// 验证密码是否匹配
    #[must_use]
    pub fn verify_password(&self, candidate: &str) -> bool {
        self.password.verify(candidate)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}
