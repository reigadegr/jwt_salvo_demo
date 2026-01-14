use crate::router::init_router;
use chrono::Local;
use my_config::config::init_config;
use my_jwt::jwt_utils::secret_key::init_jwt_utils;
use my_server_handle::{
    server_handle::{init_handle, shutdown_signal},
    shutdown_signal_monitor_init, use_http1,
};
use obfstr::{obfbytes, obfstr};
use salvo::{conn::tcp::TcpAcceptor, prelude::*};
use std::fmt;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

struct LoggerFormatter;

impl FormatTime for LoggerFormatter {
    fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

pub async fn init_misc() -> (Server<TcpAcceptor>, Router) {
    tracing_subscriber::fmt().with_timer(LoggerFormatter).init();

    let private_key = obfbytes!(include_bytes!("../keys/private_key.pem"));
    let public_key = obfbytes!(include_bytes!("../keys/public_key.pem"));

    init_config(obfstr!(include_str!("../application.toml")));
    init_jwt_utils(private_key, public_key);

    let router = init_router().await;
    let () = shutdown_signal_monitor_init();
    let acceptor = use_http1().await;
    let server = Server::new(acceptor);
    init_handle(server.handle());
    tokio::spawn(shutdown_signal());
    (server, router)
}
