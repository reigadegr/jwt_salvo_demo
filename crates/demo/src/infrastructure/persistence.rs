use std::sync::LazyLock;

use anyhow::Result;
use my_entities::{
    prelude::Users,
    users::{Column, Model},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::domain::{
    entity::{User, UserId},
    value_object::{Password, Role, Username},
};

/// 根据用户名查找用户 — 数据库查询
pub async fn find_user_by_username(
    conn: &DatabaseConnection,
    username: &str,
) -> Result<Option<User>> {
    let model = Users::find()
        .filter(Column::Username.eq(username))
        .one(conn)
        .await
        .map_err(|e| anyhow::anyhow!("数据库查询失败: {e}"))?;

    Ok(model.as_ref().map(model_to_user))
}

/// 根据 ID 查找用户 — 数据库查询
pub async fn find_user_by_id(conn: &DatabaseConnection, id: &UserId) -> Result<Option<User>> {
    let model = Users::find()
        .filter(Column::UserId.eq(id.as_str()))
        .one(conn)
        .await
        .map_err(|e| anyhow::anyhow!("数据库查询失败: {e}"))?;

    Ok(model.as_ref().map(model_to_user))
}

/// 数据映射器 — `SeaORM` Model → 领域 User
fn model_to_user(model: &Model) -> User {
    User::new(
        UserId::new(model.user_id.as_str()),
        Username::new_unchecked(model.username.as_str()),
        Password::new_unchecked(model.password.as_str()),
        Role::new_unchecked(model.role.as_str()),
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
