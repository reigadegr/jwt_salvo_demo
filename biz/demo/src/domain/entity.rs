use std::fmt;

/// 用户唯一标识
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl UserId {
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
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

/// 用户名
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

impl Username {
    #[must_use]
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Username {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// 密码 — 仅暴露验证行为，不暴露内容
#[derive(Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    #[must_use]
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    #[must_use]
    pub fn verify(&self, candidate: &str) -> bool {
        self.0 == candidate
    }
}

/// 角色
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Role(String);

impl Role {
    #[must_use]
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Role {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// 用户实体 — 聚合根
///
/// 通过 Id 判断相等性；密码仅通过 `verify_password` 暴露验证行为。
#[allow(clippy::missing_fields_in_debug)]
#[derive(Clone, Eq)]
pub struct User {
    id: UserId,
    username: Username,
    password: Password,
    role: Role,
}

impl User {
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
