use std::sync::LazyLock;

use anyhow::Result;
use my_entities::{
    prelude::Users,
    users::{Column, Model},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::domain::{
    entity::{Password, Role, User, UserId, Username},
    repository::UserRepository,
};

/// 基于 `SeaORM` 的用户仓储实现
#[must_use]
pub struct UserRepo {
    conn: DatabaseConnection,
}

impl UserRepo {
    #[allow(clippy::missing_const_for_fn)]
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

impl UserRepository for UserRepo {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let model = Users::find()
            .filter(Column::Username.eq(username))
            .one(&self.conn)
            .await
            .map_err(|e| anyhow::anyhow!("数据库查询失败: {e}"))?;

        Ok(model.as_ref().map(model_to_user))
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>> {
        let model = Users::find()
            .filter(Column::UserId.eq(id.as_str()))
            .one(&self.conn)
            .await
            .map_err(|e| anyhow::anyhow!("数据库查询失败: {e}"))?;

        Ok(model.as_ref().map(model_to_user))
    }
}

fn model_to_user(model: &Model) -> User {
    User::new(
        UserId::new(model.user_id.as_str()),
        Username::new(model.username.as_str()),
        Password::new(model.password.as_str()),
        Role::new(model.role.as_str()),
    )
}

/// 默认用户原始数据 — 用于数据库种子插入
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
