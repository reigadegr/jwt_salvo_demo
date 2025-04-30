use anyhow::Result;
use once_cell::sync::Lazy;
use redis::{Client, Commands, Connection, RedisError};

const REDIS_URI: &str = "redis://127.0.0.1:6379/";

static REDIS_DB: Lazy<Client> = Lazy::new(|| Client::open(REDIS_URI).expect("redis连接失败"));

fn get_db_con() -> Result<Connection, RedisError> {
    REDIS_DB.get_connection()
}

#[allow(dead_code)]
pub fn redis_write_and_rm<T: redis::ToRedisArgs>(key: &str, value: T, time: i64) -> Result<()> {
    let mut con = get_db_con()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set(key, value)?;
    let _: () = con.expire(key, time)?;
    Ok(())
}

pub fn redis_read(key: &str) -> Result<String> {
    let mut con = get_db_con()?;
    let rs = con.get(key)?;
    Ok(rs)
}

#[allow(dead_code)]
pub fn redis_delete(key: &str) -> Result<()> {
    let mut con = get_db_con()?;
    let _: () = con.del(key)?;

    Ok(())
}

pub fn redis_write<T: redis::ToRedisArgs>(key: &str, value: T) -> Result<()> {
    // Connect to Redis
    let mut con = get_db_con()?;
    // Throw away the result, just make sure it does not fail
    let _: () = con.set(key, value)?;
    Ok(())
}
