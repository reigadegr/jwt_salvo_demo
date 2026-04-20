pub mod shutdown_handle;

use std::{fmt, io::IsTerminal};

use chrono::Local;
use my_config::config::get_cfg;
use salvo::{conn::tcp::TcpAcceptor, prelude::*};
pub use shutdown_handle::graceful_stop;
use shutdown_handle::{init_handle, shutdown_signal};
use tracing_subscriber::{
    EnvFilter,
    fmt::{format::Writer, time::FormatTime},
};

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
        "📖 Open API Page: http://{}/docs",
        listen_addr.replace("0.0.0.0", "127.0.0.1")
    );

    let acceptor = TcpListener::new(listen_addr).bind().await;

    let mut server = Server::new(acceptor);
    server.http1_mut().keep_alive(false);

    server
}

pub async fn init_server() -> Server<TcpAcceptor> {
    // 初始化日志
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    let is_terminal = std::io::stdout().is_terminal();

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_timer(LoggerFormatter)
        .with_ansi(is_terminal)
        .init();

    let server = use_http1().await;
    if let Err(e) = init_handle(server.handle()) {
        eprintln!("Failed to initialize server handle: {e}");
        std::process::exit(1);
    }
    tokio::spawn(shutdown_signal());
    server
}
