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
pub mod nacos;
pub mod rbac;
pub mod redisync;
pub mod result;

use config::get_cfg;
use nacos::init_nacos_service;
use redisync::init_redis_pool;
use salvo::{
    conn::tcp::TcpAcceptor,
    prelude::*,
};

pub async fn use_http1() -> TcpAcceptor {
    let ip = &get_cfg().client_cfg.service_ip;
    let port = &get_cfg().client_cfg.service_port;
    let listen_addr = format!("{ip}:{port}");
    println!(
        "📖 Open API Page: http://{}/scalar",
        listen_addr.replace("0.0.0.0", "127.0.0.1")
    );
    TcpListener::new(listen_addr).bind().await
}

pub async fn application_init() {
    init_redis_pool().await;
    init_nacos_service().await;
}
