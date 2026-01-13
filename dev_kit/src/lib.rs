#![warn(
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::correctness,
    clippy::suspicious
)]
#![allow(
    clippy::similar_names,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]

pub mod config;
pub mod jwt_utils;
pub mod rbac;
pub mod result;
pub mod server_handle;

use config::get_cfg;
use salvo::{conn::tcp::TcpAcceptor, prelude::*};
use server_handle::shutdown_signal;

pub async fn use_http1() -> TcpAcceptor {
    let ip = &get_cfg().client_cfg.service_ip;
    let port = get_cfg().client_cfg.service_port;
    let listen_addr = format!("{ip}:{port}");
    println!(
        "📖 Open API Page: http://{}/scalar",
        listen_addr.replace("0.0.0.0", "127.0.0.1")
    );
    TcpListener::new(listen_addr).bind().await
}
pub fn application_init() {
    tokio::spawn(shutdown_signal());
}
