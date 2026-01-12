use crate::router::init_router;
use chrono::Local;
use dev_kit::{
    application_init,
    config::init_config,
    jwt_utils::secret_key::init_jwt_utils,
    server_handle::{init_handle, shutdown_signal},
    use_http1,
};
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

    init_config(include_str!("../application.toml"));
    init_jwt_utils(
        include_bytes!("../keys/private_key.pem"),
        include_bytes!("../keys/public_key.pem"),
    );

    let router = init_router().await;
    let () = application_init().await;
    let acceptor = use_http1().await;
    let server = Server::new(acceptor);
    init_handle(server.handle());
    tokio::spawn(shutdown_signal());
    (server, router)
}
