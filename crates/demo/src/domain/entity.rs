use std::sync::Arc;

use crate::domain::value_object::{Password, Role, Username};

/// 用户唯一标识 — 值对象
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

/// 用户实体 — 聚合根
///
/// Entity: 具有唯一标识，通过 Id 判断相等性。
/// Aggregate Root: 对外统一入口，封装内部不变量。
/// 密码仅通过 `verify_password` 暴露验证行为，不暴露密码本身。
#[allow(clippy::missing_fields_in_debug)]
#[derive(Clone, Eq)]
pub struct User {
    id: UserId,
    username: Username,
    password: Password,
    role: Role,
}

impl User {
    /// 创建用户 — 验证所有字段非空
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn new(id: UserId, username: Username, password: Password, role: Role) -> Self {
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

    /// 验证密码是否匹配 — 唯一的密码交互方式
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

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("role", &self.role)
            .finish_non_exhaustive()
    }
}
