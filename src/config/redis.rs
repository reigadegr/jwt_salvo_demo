use anyhow::Result;
use once_cell::sync::Lazy;
use redis::{AsyncCommands, Client, RedisError, aio::MultiplexedConnection};

const REDIS_URI: &str = "redis://127.0.0.1:6379/";

static REDIS_DB: Lazy<Client> = Lazy::new(|| Client::open(REDIS_URI).expect("redis连接失败"));

async fn get_db_con() -> Result<MultiplexedConnection, RedisError> {
    REDIS_DB.get_multiplexed_async_connection().await
}

// #[allow(dead_code)]
// pub async fn redis_write_and_rm<T: redis::ToRedisArgs>(
// key: &str,
// value: T,
// time: i64,
// ) -> Result<()> {
// let mut con = get_db_con().await?;
// // throw away the result, just make sure it does not fail
// let _: () = con.set(key, value).await?;
// let _: () = con.expire(key, time).await?;
// Ok(())
// }

pub async fn redis_read(key: &str) -> Result<String> {
    let mut con = get_db_con().await?;
    let rs = con.get(key).await?;
    Ok(rs)
}

// #[allow(dead_code)]
// pub async fn redis_delete(key: &str) -> Result<()> {
// let mut con = get_db_con().await?;
// let _: () = con.del(key).await?;

// Ok(())
// }

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
