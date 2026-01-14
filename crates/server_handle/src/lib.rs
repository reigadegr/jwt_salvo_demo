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
use my_jwt::jwt_utils::secret_key::init_jwt_utils;
use obfstr::obfbytes;
use salvo::{conn::tcp::TcpAcceptor, prelude::*};
use shutdown_handle::{init_handle, shutdown_signal};
use std::fmt;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

struct LoggerFormatter;

impl FormatTime for LoggerFormatter {
    fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

pub async fn use_http1() -> TcpAcceptor {
    let ip = &get_cfg().client_cfg.service_ip;
    let port = get_cfg().client_cfg.service_port;
    let listen_addr = format!("{ip}:{port}");
    #[cfg(debug_assertions)]
    println!(
        "📖 Open API Page: http://{}/scalar",
        listen_addr.replace("0.0.0.0", "127.0.0.1")
    );
    TcpListener::new(listen_addr).bind().await
}
pub fn shutdown_signal_monitor_init() {
    tokio::spawn(shutdown_signal());
}

pub async fn init_misc() -> Server<TcpAcceptor> {
    tracing_subscriber::fmt().with_timer(LoggerFormatter).init();

    let private_key = obfbytes!(include_bytes!("../../../keys/private_key.pem"));
    let public_key = obfbytes!(include_bytes!("../../../keys/public_key.pem"));

    init_jwt_utils(private_key, public_key);

    let () = shutdown_signal_monitor_init();
    let acceptor = use_http1().await;
    let server = Server::new(acceptor);
    init_handle(server.handle());
    tokio::spawn(shutdown_signal());
    server
}
