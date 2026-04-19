use std::{collections::HashMap, sync::LazyLock};

use anyhow::Result;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::domain::entity::{User, UserId};
use crate::domain::repository::UserRepository;
use crate::domain::value_object::{Password, Role, Username};
use crate::sea_orm_entity;

/// 内存用户仓储实现 — 用于测试/演示
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

/// 数据库用户仓储实现 — 基于 `SeaORM` 查询
pub struct DatabaseUserRepository<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> DatabaseUserRepository<'a> {
    #[must_use]
    pub const fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    /// 数据映射器 — `SeaORM` Model → 领域 User
    fn model_to_user(model: &sea_orm_entity::Model) -> User {
        User::new(
            UserId::new(model.user_id.as_str()),
            Username::new_unchecked(model.username.as_str()),
            Password::new_unchecked(model.password.as_str()),
            Role::new_unchecked(model.role.as_str()),
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

/// 默认用户原始数据 — 用于数据库种子插入
///
/// 聚合根不暴露密码，但基础设施层需要完整字段写入数据库。
/// 通过此结构体将领域对象与持久化细节解耦。
pub static DEFAULT_USER_RAW_DATA: LazyLock<Vec<RawUserData>> = LazyLock::new(|| {
    vec![
        RawUserData {
            user_id: "1",
            username: "user1",
            password: "password1",
            role: "token-admin",
        },
        RawUserData {
            user_id: "2",
            username: "user2",
            password: "password2",
            role: "yz666",
        },
    ]
});

/// 原始用户数据 — 基础设施层专用
pub struct RawUserData {
    pub user_id: &'static str,
    pub username: &'static str,
    pub password: &'static str,
    pub role: &'static str,
}
