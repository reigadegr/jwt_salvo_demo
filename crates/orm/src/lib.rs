use anyhow::{Context, Result, anyhow};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::time::Duration;

fn prepare_sqlite_path(uri: &str) -> Result<()> {
    let path = uri.strip_prefix("sqlite://").unwrap_or(uri);

    if path.starts_with('/') || path.starts_with(':') {
        return Ok(());
    }

    let db_path = std::path::Path::new(path);

    if let Some(parent_dir) = db_path.parent()
        && !parent_dir.exists()
    {
        std::fs::create_dir_all(parent_dir).context("Failed to create database directory")?;
    }

    if !db_path.exists() {
        std::fs::File::create(db_path).context("Failed to create database file")?;
    }

    Ok(())
}

pub async fn init_db(uri: &str) -> Result<DatabaseConnection> {
    prepare_sqlite_path(uri)?;

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
