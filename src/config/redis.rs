use anyhow::Result;
use deadpool_redis::{
    Config, Connection, Pool, PoolError, Runtime,
    redis::{self, AsyncCommands},
};
use once_cell::sync::Lazy;

const REDIS_URI: &str = "redis://127.0.0.1:6379/";

static REDIS_POOL: Lazy<Pool> = Lazy::new(|| {
    let cfg = Config::from_url(REDIS_URI);
    cfg.create_pool(Some(Runtime::Tokio1))
        .expect("创建Redis连接池失败")
});

async fn get_db_con() -> Result<Connection, PoolError> {
    REDIS_POOL.get().await
}

#[allow(dead_code)]
pub async fn redis_write_and_rm<T: redis::ToRedisArgs + Send + Sync>(
    key: &str,
    value: T,
    time: i64,
) -> Result<()> {
    let mut con = get_db_con().await?;
    let _: () = con.set(key, value).await?;
    let _: () = con.expire(key, time).await?;
    Ok(())
}

pub async fn redis_read(key: &str) -> Result<String> {
    let mut con = get_db_con().await?;
    let rs: String = con.get(key).await?;
    Ok(rs)
}

#[allow(dead_code)]
pub async fn redis_delete(key: &str) -> Result<()> {
    let mut con = get_db_con().await?;
    let _: () = con.del(key).await?;
    Ok(())
}

pub async fn redis_write<T: redis::ToRedisArgs + Send + Sync>(key: &str, value: T) -> Result<()> {
    let mut con = get_db_con().await?;
    let _: () = con.set(key, value).await?;
    Ok(())
}
