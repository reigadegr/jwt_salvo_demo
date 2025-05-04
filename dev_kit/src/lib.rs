pub mod config;
pub mod jwt;
pub mod middleware;
pub mod models;
pub mod nacos;
pub mod rbac;
pub mod redisync;
pub mod result;

use crate::config::PROFILE;
use nacos::init_nacos_service;
use rbac::init_model;
use redisync::init_redis_pool;
use salvo::{conn::tcp::TcpAcceptor, prelude::*};

async fn use_http1() -> TcpAcceptor {
    let ip = &PROFILE.client_cfg.app_ip;
    let port = &PROFILE.client_cfg.app_port;
    let server_args = format!("{ip}:{port}");

    TcpListener::new(server_args).bind().await
}

pub async fn application_init() -> TcpAcceptor {
    init_model().await;
    init_redis_pool().await;
    init_nacos_service().await;
    use_http1().await
}
