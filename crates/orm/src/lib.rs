use anyhow::{Context, Result, anyhow};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::time::Duration;

pub async fn init_db(uri: &str) -> Result<DatabaseConnection> {
    // 如果是 SQLite，确保数据库文件的父目录存在，并创建空文件（如果不存在）
    if uri.starts_with("sqlite://") {
        let path = uri.strip_prefix("sqlite://").unwrap_or(uri);
        // 处理相对路径
        if !path.starts_with('/') && !path.starts_with(':') {
            // 相对路径，创建父目录
            let db_path = std::path::Path::new(path);
            if let Some(parent_dir) = db_path.parent()
                && !parent_dir.exists()
            {
                std::fs::create_dir_all(parent_dir)
                    .context("Failed to create database directory")?;
            }
            // SQLite 不会自动创建文件，需要预先创建空文件
            if !db_path.exists() {
                std::fs::File::create(db_path).context("Failed to create database file")?;
            }
        }
    }

    let mut opt = ConnectOptions::new(uri);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(30))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(30))
        .max_lifetime(Duration::from_secs(30))
        .sqlx_logging(cfg!(debug_assertions))
        .sqlx_logging_level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        });
    Database::connect(opt)
        .await
        .map_err(|e| anyhow!(e))
        .context("Failed to connect to the database")
}
