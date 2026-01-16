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

pub mod shutdown_handle;

use chrono::Local;
use my_config::config::get_cfg;
use salvo::{conn::tcp::TcpAcceptor, prelude::*};
use shutdown_handle::{init_handle, shutdown_signal};
use std::{fmt, time::Duration};
use tracing::Level;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

struct LoggerFormatter;

impl FormatTime for LoggerFormatter {
    fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

pub async fn use_http1() -> Server<TcpAcceptor> {
    let ip = &get_cfg().client_cfg.service_ip;
    let port = get_cfg().client_cfg.service_port;
    let listen_addr = format!("{ip}:{port}");
    #[cfg(debug_assertions)]
    println!(
        "📖 Open API Page: http://{}/scalar",
        listen_addr.replace("0.0.0.0", "127.0.0.1")
    );

    let acceptor = TcpListener::new(listen_addr).bind().await;

    let mut server = Server::new(acceptor);
    server
        .http1_mut()
        .keep_alive(true)
        .header_read_timeout(Some(Duration::from_secs(5)));

    server
}

pub fn shutdown_signal_monitor_init() {
    tokio::spawn(shutdown_signal());
}

pub async fn init_server() -> Server<TcpAcceptor> {
    let log_level = if cfg!(debug_assertions) {
        Level::DEBUG
    } else {
        Level::INFO
    };

    tracing_subscriber::fmt()
        .with_timer(LoggerFormatter)
        .with_max_level(log_level)
        .init();

    let () = shutdown_signal_monitor_init();
    let server = use_http1().await;
    init_handle(server.handle());
    tokio::spawn(shutdown_signal());
    server
}
