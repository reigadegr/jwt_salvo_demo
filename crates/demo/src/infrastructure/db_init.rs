use anyhow::{Context, Result};
use log::info;
use my_entities::{prelude::Users, users::ActiveModel};
use my_schema::ensure_table_exists;
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait, PaginatorTrait};
use uuid::Uuid;

use crate::infrastructure::persistence::DEFAULT_USER_RAW_DATA;

/// 初始化用户数据库架构
///
/// 检查用户表是否存在，不存在则创建表并插入默认数据
pub async fn init_database_schema(conn: &DatabaseConnection) -> Result<()> {
    info!("开始初始化用户数据库架构...");

    ensure_table_exists::<Users>(conn).await?;
    ensure_default_data_exists(conn).await?;

    info!("用户数据库架构初始化完成");
    Ok(())
}

/// 确保默认数据存在，不存在则插入
async fn ensure_default_data_exists(conn: &DatabaseConnection) -> Result<()> {
    info!("检查默认用户数据...");

    let count = Users::find()
        .count(conn)
        .await
        .context("查询用户数量失败")?;

    if count > 0 {
        info!("用户表已有 {count} 条数据，跳过默认数据插入");
        return Ok(());
    }

    info!("开始插入默认用户数据...");

    let users: Vec<_> = DEFAULT_USER_RAW_DATA
        .iter()
        .map(|raw| ActiveModel {
            id: Set(Uuid::now_v7()),
            user_id: Set(raw.user_id.to_string()),
            username: Set(raw.username.to_string()),
            password: Set(raw.password.to_string()),
            role: Set(raw.role.to_string()),
        })
        .collect();

    Users::insert_many(users)
        .exec(conn)
        .await
        .context("批量插入默认用户数据失败")?;

    info!(
        "默认用户数据插入完成，共 {} 条",
        DEFAULT_USER_RAW_DATA.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use sea_orm::{DbBackend, Schema};

    use super::*;

    #[test]
    fn test_default_users_exist() {
        assert!(!DEFAULT_USER_RAW_DATA.is_empty());
        assert_eq!(DEFAULT_USER_RAW_DATA.len(), 2);
    }

    #[test]
    fn test_uuid_v7_generation() {
        let id = Uuid::now_v7();
        let hex = id.as_simple().to_string();
        assert!(hex.starts_with('0'), "UUID v7 hex: {hex}");
        println!("UUID v7: {id}");
    }

    #[test]
    fn test_schema_sqlite_generates_uuid_text() {
        let backend = DbBackend::Sqlite;
        let schema = Schema::new(backend);
        let create_table = schema.create_table_from_entity(Users);
        let sql = backend.build(&create_table).sql;

        assert!(
            sql.contains("uuid_text") || sql.contains("BLOB"),
            "SQLite SQL should contain uuid_text or BLOB: {sql}"
        );
        println!("SQLite CREATE TABLE:\n{sql}");
    }

    #[test]
    fn test_schema_mysql_generates_binary16() {
        let backend = DbBackend::MySql;
        let schema = Schema::new(backend);
        let create_table = schema.create_table_from_entity(Users);
        let sql = backend.build(&create_table).sql;

        assert!(
            sql.contains("binary(16)") || sql.contains("BINARY(16)"),
            "MySQL SQL should contain binary(16): {sql}"
        );
        println!("MySQL CREATE TABLE:\n{sql}");
    }

    #[test]
    fn test_schema_postgres_generates_uuid() {
        let backend = DbBackend::Postgres;
        let schema = Schema::new(backend);
        let create_table = schema.create_table_from_entity(Users);
        let sql = backend.build(&create_table).sql;

        assert!(
            sql.contains("\"id\" uuid") || sql.contains("\"id\" UUID"),
            "PostgreSQL SQL should contain uuid type: {sql}"
        );
        println!("PostgreSQL CREATE TABLE:\n{sql}");
    }
}
