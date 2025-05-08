#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::non_std_lazy_statics,
    clippy::similar_names,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]

pub mod config;
pub mod graceful_stop;
pub mod jwt_utils;
pub mod models;
pub mod nacos;
pub mod rbac;
pub mod redisync;
pub mod result;

use config::get_cfg;
use nacos::init_nacos_service;
use redisync::init_redis_pool;
use salvo::{conn::tcp::TcpAcceptor, prelude::*};
pub use tracing_subscriber;

async fn use_http1() -> Server<TcpAcceptor> {
    let ip = &get_cfg().client_cfg.app_ip;
    let port = &get_cfg().client_cfg.app_port;
    let server_args = format!("{ip}:{port}");

    let acceptor = TcpListener::new(server_args).bind().await;
    Server::new(acceptor)
}

pub async fn application_init() -> Server<TcpAcceptor> {
    init_redis_pool().await;
    init_nacos_service().await;
    use_http1().await
}
