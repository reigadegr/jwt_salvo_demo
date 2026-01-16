use anyhow::{Context, Result, anyhow};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::time::Duration;

pub async fn init_db(uri: &str) -> Result<DatabaseConnection> {
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
