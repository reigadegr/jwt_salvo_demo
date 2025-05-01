use anyhow::{Result, anyhow};
use bb8::{Pool, PooledConnection, RunError};
use bb8_redis::RedisConnectionManager;
use redis::{AsyncCommands, RedisError};
use std::time::Duration;
use tokio::sync::OnceCell;

const REDIS_URI: &str = "redis://127.0.0.1:6379/";

pub static REDIS_POOL: OnceCell<Pool<RedisConnectionManager>> = OnceCell::const_new();

pub async fn init_redis_pool() {
    REDIS_POOL
        .get_or_init(|| async {
            let manager = RedisConnectionManager::new(REDIS_URI).unwrap();
            Pool::builder()
                .max_size(50)
                .min_idle(Some(10))
                .max_lifetime(Some(Duration::from_secs(60)))
                .idle_timeout(Some(Duration::from_secs(60)))
                .connection_timeout(Duration::from_secs(5))
                .test_on_check_out(true)
                .build(manager)
                .await
                .unwrap()
        })
        .await;
}

async fn get_db_con()
-> Result<PooledConnection<'static, RedisConnectionManager>, RunError<RedisError>> {
    REDIS_POOL
        .get()
        .ok_or_else(|| anyhow!("POOL not found"))
        .unwrap()
        .get()
        .await
}

pub async fn redis_read(key: &str) -> Result<String> {
    let mut con = get_db_con().await?;
    let rs = con.get(key).await?;
    Ok(rs)
}

pub async fn redis_write<T: redis::ToRedisArgs + std::marker::Send + std::marker::Sync>(
    key: &str,
    value: T,
) -> Result<()> {
    // Connect to Redis
    let mut con = get_db_con().await?;
    // Throw away the result, just make sure it does not fail
    let _: () = con.set(key, value).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn redis_write_and_rm<T: redis::ToRedisArgs + std::marker::Sync + std::marker::Send>(
    key: &str,
    value: T,
    time: i64,
) -> Result<()> {
    let mut con = get_db_con().await?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set(key, value).await?;
    let _: () = con.expire(key, time).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn redis_delete(key: &str) -> Result<()> {
    let mut con = get_db_con().await?;
    let _: () = con.del(key).await?;
    Ok(())
}
