use anyhow::{Context, Result};
use log::info;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectionTrait, DatabaseConnection, DbBackend,
    EntityTrait, PaginatorTrait, Schema, Statement,
};
use uuid::Uuid;

use crate::{infrastructure::persistence::DEFAULT_USER_RAW_DATA, sea_orm_entity};

/// 初始化数据库架构
///
/// 检查数据库和表是否存在，如果不存在则创建表并插入默认数据
pub async fn init_database_schema(conn: &DatabaseConnection) -> Result<()> {
    info!("开始初始化数据库架构...");

    ensure_users_table_exists(conn).await?;
    ensure_default_data_exists(conn).await?;

    info!("数据库架构初始化完成");
    Ok(())
}

/// 确保用户表存在，不存在则创建
async fn ensure_users_table_exists(conn: &DatabaseConnection) -> Result<()> {
    let table_exists = check_table_exists(conn, "users").await?;

    if table_exists {
        info!("用户表已存在，跳过创建");
        return Ok(());
    }

    info!("用户表不存在，开始创建...");

    let db_backend = conn.get_database_backend();
    let schema = Schema::new(db_backend);
    let mut create_table = schema.create_table_from_entity(sea_orm_entity::Entity);
    create_table.if_not_exists();

    conn.execute(&create_table)
        .await
        .context("创建用户表失败")?;

    info!("用户表创建成功");
    Ok(())
}

/// 检查表是否存在
async fn check_table_exists(conn: &DatabaseConnection, table_name: &str) -> Result<bool> {
    let db_backend = conn.get_database_backend();

    let check_sql = match db_backend {
        DbBackend::Sqlite => {
            format!("SELECT name FROM sqlite_master WHERE type='table' AND name='{table_name}'")
        }
        DbBackend::MySql => {
            format!("SHOW TABLES LIKE '{table_name}'")
        }
        DbBackend::Postgres => {
            format!(
                "SELECT table_name FROM information_schema.tables WHERE table_name = '{table_name}'"
            )
        }
        _ => return Ok(true),
    };

    let result = conn
        .query_one_raw(Statement::from_string(db_backend, check_sql))
        .await
        .context("检查表是否存在时出错")?;

    Ok(result.is_some())
}

/// 确保默认数据存在，不存在则插入
async fn ensure_default_data_exists(conn: &DatabaseConnection) -> Result<()> {
    info!("检查默认用户数据...");

    let count = sea_orm_entity::Entity::find()
        .count(conn)
        .await
        .context("查询用户数量失败")?;

    if count > 0 {
        info!("用户表已有 {count} 条数据，跳过默认数据插入");
        return Ok(());
    }

    info!("开始插入默认用户数据...");

    for raw in DEFAULT_USER_RAW_DATA.iter() {
        let id = Uuid::now_v7();

        let new_user = sea_orm_entity::ActiveModel {
            id: Set(id),
            user_id: Set(raw.user_id.to_string()),
            username: Set(raw.username.to_string()),
            password: Set(raw.password.to_string()),
            role: Set(raw.role.to_string()),
        };

        new_user
            .insert(conn)
            .await
            .with_context(|| format!("插入用户 {} 失败", raw.username))?;

        info!("已插入默认用户: {} (id: {})", raw.username, id);
    }

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
    use crate::sea_orm_entity;

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
        let create_table = schema.create_table_from_entity(sea_orm_entity::Entity);
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
        let create_table = schema.create_table_from_entity(sea_orm_entity::Entity);
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
        let create_table = schema.create_table_from_entity(sea_orm_entity::Entity);
        let sql = backend.build(&create_table).sql;

        assert!(
            sql.contains("\"id\" uuid") || sql.contains("\"id\" UUID"),
            "PostgreSQL SQL should contain uuid type: {sql}"
        );
        println!("PostgreSQL CREATE TABLE:\n{sql}");
    }
}
