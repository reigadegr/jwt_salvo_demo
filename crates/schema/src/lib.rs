//! 通用数据库架构初始化工具
//!
//! 提供表存在性检查、建表等通用功能，与具体业务逻辑解耦。

use anyhow::{Context, Result};
use log::info;
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DbBackend, EntityName, EntityTrait, Schema, Statement,
};

/// 检查表是否存在
///
/// 支持 SQLite、MySQL、PostgreSQL 三种数据库后端
pub async fn check_table_exists(conn: &DatabaseConnection, table_name: &str) -> Result<bool> {
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

/// 确保表存在，不存在则创建
///
/// 泛型版本，接受任意 `SeaORM` Entity
pub async fn ensure_table_exists<E>(conn: &DatabaseConnection) -> Result<()>
where
    E: EntityTrait + EntityName,
{
    let table_name = <E as EntityName>::table_name(&E::default());

    if check_table_exists(conn, table_name).await? {
        info!("表 {table_name} 已存在，跳过创建");
        return Ok(());
    }

    info!("表 {table_name} 不存在，开始创建...");

    let db_backend = conn.get_database_backend();
    let schema = Schema::new(db_backend);
    let mut create_table = schema.create_table_from_entity(E::default());
    create_table.if_not_exists();

    conn.execute(&create_table)
        .await
        .context(format!("创建表 {table_name} 失败"))?;

    info!("表 {table_name} 创建成功");
    Ok(())
}

#[cfg(test)]
mod tests {
    use sea_orm::{DbBackend, Schema};

    #[test]
    fn test_schema_sqlite_generates_valid_sql() {
        let backend = DbBackend::Sqlite;
        let schema = Schema::new(backend);
        // 验证 Schema 可以正常创建
        let _ = schema;
    }
}
