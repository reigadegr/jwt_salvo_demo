#![warn(
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::correctness,
    clippy::suspicious,
    clippy::unwrap_used,
    clippy::expect_used
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
use std::fmt;
use tracing::Level;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

struct LoggerFormatter;

impl FormatTime for LoggerFormatter {
    fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

pub async fn use_http1() -> Server<TcpAcceptor> {
    let cfg = match get_cfg() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to get configuration: {e}");
            std::process::exit(1);
        }
    };
    let ip = &cfg.client_cfg.service_ip;
    let port = cfg.client_cfg.service_port;
    let listen_addr = format!("{ip}:{port}");
    #[cfg(debug_assertions)]
    println!(
        "📖 Open API Page: http://{}/scalar",
        listen_addr.replace("0.0.0.0", "127.0.0.1")
    );

    let acceptor = TcpListener::new(listen_addr).bind().await;

    let mut server = Server::new(acceptor);
    server.http1_mut().keep_alive(false);

    server
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

    let server = use_http1().await;
    if let Err(e) = init_handle(server.handle()) {
        eprintln!("Failed to initialize server handle: {e}");
        std::process::exit(1);
    }
    tokio::spawn(shutdown_signal());
    server
}
