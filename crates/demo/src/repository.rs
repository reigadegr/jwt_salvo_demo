use std::{collections::HashMap, sync::LazyLock};

use crate::{
    entity::{User, UserId},
    value_object::{Password, Role, Username},
};

/// 用户仓储接口 - 抽象持久化
///
/// Repository: 封装数据访问，提供类似集合的接口
pub trait UserRepository: Send + Sync {
    fn find_by_username(&self, username: &str) -> Option<User>;
    fn find_by_id(&self, id: &UserId) -> Option<User>;
}

/// 内存用户仓储实现 - 用于测试/演示
#[derive(Debug, Default)]
pub struct InMemoryUserRepository {
    users: HashMap<String, User>,
}

impl InMemoryUserRepository {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 从静态数据初始化
    #[must_use]
    pub fn with_static_data(users: Vec<User>) -> Self {
        let mut repo = Self::new();
        for user in users {
            repo.users
                .insert(user.username().as_str().to_string(), user);
        }
        repo
    }
}

impl UserRepository for InMemoryUserRepository {
    fn find_by_username(&self, username: &str) -> Option<User> {
        self.users.get(username).cloned()
    }

    fn find_by_id(&self, id: &UserId) -> Option<User> {
        self.users.values().find(|u| u.id() == id).cloned()
    }
}

/// 默认测试用户数据
pub static DEFAULT_USERS: LazyLock<Vec<User>> = LazyLock::new(|| {
    vec![
        User::new(
            UserId::new("1"),
            Username::new("user1"),
            Password::new("password1"),
            Role::new("token-admin"),
        ),
        User::new(
            UserId::new("2"),
            Username::new("user2"),
            Password::new("password2"),
            Role::new("yz666"),
        ),
    ]
});
