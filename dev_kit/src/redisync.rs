use crate::config::get_cfg;
use anyhow::{Result, anyhow};
use bb8::{Pool, PooledConnection, RunError};
use bb8_redis::{
    RedisConnectionManager,
    redis::{AsyncCommands, RedisError, ToRedisArgs, ToSingleRedisArg},
};
use std::time::Duration;
use tokio::sync::OnceCell;

pub static REDIS_POOL: OnceCell<Pool<RedisConnectionManager>> = OnceCell::const_new();

pub async fn init_redis_pool() {
    REDIS_POOL
        .get_or_init(|| async {
            let manager = RedisConnectionManager::new(&*get_cfg().redis_cfg.uri).unwrap();
            let max_lifetime = get_cfg().redis_cfg.max_lifetime.map(Duration::from_secs);

            let idle_timeout = get_cfg().redis_cfg.idle_timeout.map(Duration::from_secs);

            let connection_timeout = Duration::from_secs(get_cfg().redis_cfg.connection_timeout);

            Pool::builder()
                .max_size(get_cfg().redis_cfg.max_size)
                .min_idle(get_cfg().redis_cfg.min_idle)
                .max_lifetime(max_lifetime)
                .idle_timeout(idle_timeout)
                .connection_timeout(connection_timeout)
                .test_on_check_out(get_cfg().redis_cfg.test_on_check_out)
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

pub async fn redis_set_with_expiry<T>(key: &str, value: T, time: i64) -> Result<()>
where
    T: ToSingleRedisArg + ToRedisArgs + Send + Sync,
{
    let mut con = get_db_con().await?;
    let _: () = con.set(key, value).await?;
    let _: () = con.expire(key, time).await?;
    Ok(())
}

pub async fn redis_write<T>(key: &str, value: T) -> Result<()>
where
    T: ToSingleRedisArg + ToRedisArgs + Send + Sync,
{
    let mut con = get_db_con().await?;
    let _: () = con.set(key, value).await?;
    Ok(())
}

pub async fn redis_delete(key: &str) -> Result<()> {
    let mut con = get_db_con().await?;
    let _: () = con.del(key).await?;
    Ok(())
}
