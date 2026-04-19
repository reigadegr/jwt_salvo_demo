use std::{collections::HashMap, future::Future, sync::LazyLock};

use anyhow::Result;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entity::{User, UserId},
    sea_orm_entity,
    value_object::{Password, Role, Username},
};

/// 用户仓储接口 - 抽象持久化
///
/// Repository: 封装数据访问，提供类似集合的接口
pub trait UserRepository: Send + Sync {
    fn find_by_username(&self, username: &str)
    -> impl Future<Output = Result<Option<User>>> + Send;
    fn find_by_id(&self, id: &UserId) -> impl Future<Output = Result<Option<User>>> + Send;
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
    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        Ok(self.users.get(username).cloned())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>> {
        Ok(self.users.values().find(|u| u.id() == id).cloned())
    }
}

/// 数据库用户仓储实现 - 基于 `SeaORM` 查询
pub struct DatabaseUserRepository<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> DatabaseUserRepository<'a> {
    #[must_use]
    pub const fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    fn model_to_user(model: &sea_orm_entity::Model) -> User {
        User::new(
            UserId::new(model.user_id.as_str()),
            Username::new(model.username.as_str()),
            Password::new(model.password.as_str()),
            Role::new(model.role.as_str()),
        )
    }
}

impl UserRepository for DatabaseUserRepository<'_> {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let model = sea_orm_entity::Entity::find()
            .filter(sea_orm_entity::Column::Username.eq(username))
            .one(self.conn)
            .await
            .map_err(|e| anyhow::anyhow!("数据库查询失败: {e}"))?;

        Ok(model.as_ref().map(Self::model_to_user))
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>> {
        let model = sea_orm_entity::Entity::find()
            .filter(sea_orm_entity::Column::UserId.eq(id.as_str()))
            .one(self.conn)
            .await
            .map_err(|e| anyhow::anyhow!("数据库查询失败: {e}"))?;

        Ok(model.as_ref().map(Self::model_to_user))
    }
}

/// 默认测试用户数据（仅用于数据库种子数据初始化）
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
