use anyhow::{Context, Result};
use log::{info, warn};
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use uuid::Uuid;

use crate::repository::DEFAULT_USERS;

/// 初始化数据库架构
///
/// 检查数据库和表是否存在，如果不存在则创建表并插入默认数据
pub async fn init_database_schema(conn: &DatabaseConnection) -> Result<()> {
    info!("开始初始化数据库架构...");

    // 获取数据库类型
    let db_backend = conn.get_database_backend();

    // 检查并创建用户表
    ensure_users_table_exists(conn, db_backend).await?;

    // 检查并插入默认数据
    ensure_default_data_exists(conn).await?;

    info!("数据库架构初始化完成");
    Ok(())
}

/// 确保用户表存在，不存在则创建
async fn ensure_users_table_exists(conn: &DatabaseConnection, db_backend: DbBackend) -> Result<()> {
    // 检查表是否存在
    let table_exists = check_table_exists(conn, "users").await?;

    if table_exists {
        info!("用户表已存在，跳过创建");
        return Ok(());
    }

    info!("用户表不存在，开始创建...");

    // 创建用户表的 SQL
    let create_table_sql = match db_backend {
        DbBackend::Sqlite => {
            // SQLite 使用 UUID v7 作为主键
            r"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL UNIQUE,
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                role TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
            "
        }
        DbBackend::MySql => {
            // MySQL 版本 - 使用 UUID v7 作为主键
            r"
            CREATE TABLE IF NOT EXISTS users (
                id CHAR(36) PRIMARY KEY,
                user_id VARCHAR(64) NOT NULL UNIQUE,
                username VARCHAR(64) NOT NULL UNIQUE,
                password VARCHAR(128) NOT NULL,
                role VARCHAR(64) NOT NULL,
                INDEX idx_users_username (username)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
            "
        }
        DbBackend::Postgres => {
            // PostgreSQL 版本 - 使用 UUID v7 作为主键
            r"
            CREATE TABLE IF NOT EXISTS users (
                id UUID PRIMARY KEY,
                user_id VARCHAR(64) NOT NULL UNIQUE,
                username VARCHAR(64) NOT NULL UNIQUE,
                password VARCHAR(128) NOT NULL,
                role VARCHAR(64) NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
            "
        }
        _ => {
            warn!("未知的数据库类型，使用通用 SQL");
            r"
            CREATE TABLE IF NOT EXISTS users (
                id CHAR(36) PRIMARY KEY,
                user_id VARCHAR(64) NOT NULL UNIQUE,
                username VARCHAR(64) NOT NULL UNIQUE,
                password VARCHAR(128) NOT NULL,
                role VARCHAR(64) NOT NULL
            )
            "
        }
    };

    // 执行创建表语句 - 使用 execute_raw 方法
    conn.execute_raw(Statement::from_string(
        db_backend,
        create_table_sql.to_string(),
    ))
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
            // MySQL 需要数据库名，这里用简化的方式
            format!("SHOW TABLES LIKE '{table_name}'")
        }
        DbBackend::Postgres => {
            format!(
                "SELECT table_name FROM information_schema.tables WHERE table_name = '{table_name}'"
            )
        }
        _ => {
            // 默认假设表存在
            return Ok(true);
        }
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

    let db_backend = conn.get_database_backend();

    // 检查是否已有数据
    let count_sql = "SELECT COUNT(*) as count FROM users";
    let result = conn
        .query_one_raw(Statement::from_string(db_backend, count_sql))
        .await
        .context("查询用户数量失败")?;

    let count: i64 = result.map_or(0, |r| r.try_get::<i64>("", "count").unwrap_or(0));

    if count > 0 {
        info!("用户表已有 {count} 条数据，跳过默认数据插入");
        return Ok(());
    }

    info!("开始插入默认用户数据...");

    // 插入默认用户
    for user in DEFAULT_USERS.iter() {
        // 使用 UUID v7 作为主键
        let id = Uuid::now_v7();

        let insert_sql = format!(
            "INSERT INTO users (id, user_id, username, password, role) VALUES ('{}', '{}', '{}', '{}', '{}')",
            id,
            user.id().as_str(),
            user.username().as_str(),
            user.password().as_str(),
            user.role().as_str()
        );

        conn.execute_raw(Statement::from_string(db_backend, insert_sql.clone()))
            .await
            .with_context(|| format!("插入用户 {} 失败", user.username().as_str()))?;

        info!("已插入默认用户: {} (id: {})", user.username().as_str(), id);
    }

    info!("默认用户数据插入完成，共 {} 条", DEFAULT_USERS.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_users_exist() {
        assert!(!DEFAULT_USERS.is_empty());
        assert_eq!(DEFAULT_USERS.len(), 2);
    }
}
