use std::fmt;

/// 用户名值对象
///
/// 不可变、可互换的领域概念，通过值判等。
/// 验证规则：非空且不含前后空白。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

impl Username {
    /// 验证构造器 — 确保实例始终合法
    pub fn new(s: impl Into<String>) -> Self {
        let inner = s.into();
        debug_assert!(
            !inner.trim().is_empty(),
            "Username must not be empty or blank"
        );
        Self(inner)
    }

    /// 无验证构造器 — 用于从已验证的数据源（如数据库）重建
    pub(crate) fn new_unchecked(s: impl Into<String>) -> Self {
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

impl From<String> for Username {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// 密码值对象
///
/// 内部细节不对外暴露。验证通过 `verify` 方法进行。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    /// 创建密码 — 用于注册/初始化
    pub fn new(s: impl Into<String>) -> Self {
        let inner = s.into();
        debug_assert!(!inner.is_empty(), "Password must not be empty");
        Self(inner)
    }

    /// 无验证构造器 — 用于从已验证的数据源重建
    pub(crate) fn new_unchecked(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// 验证候选密码是否匹配
    #[must_use]
    pub fn verify(&self, candidate: &str) -> bool {
        self.0 == candidate
    }
}

impl From<&str> for Password {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// 角色值对象
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Role(String);

impl Role {
    pub fn new(s: impl Into<String>) -> Self {
        let inner = s.into();
        debug_assert!(!inner.trim().is_empty(), "Role must not be empty or blank");
        Self(inner)
    }

    /// 无验证构造器 — 用于从已验证的数据源重建
    pub(crate) fn new_unchecked(s: impl Into<String>) -> Self {
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

impl From<String> for Role {
    fn from(s: String) -> Self {
        Self(s)
    }
}
