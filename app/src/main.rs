#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::non_std_lazy_statics,
    clippy::similar_names,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc
)]

mod controller;
mod router;

use dev_kit::{
    application_init, config::init_config, graceful_stop::get_handle, graceful_stop::init_handle,
    jwt_utils::secret_key::init_jwt_utils, tracing_subscriber,
};
use router::init_router;
use salvo::{conn::tcp::TcpAcceptor, prelude::*};
use tokio::signal;
use tracing::info;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    let (server, router) = init_misc().await;
    server.serve(router).await;
}

async fn init_misc() -> (Server<TcpAcceptor>, Router) {
    tracing_subscriber::fmt().init();
    init_config(include_str!("../application.toml"));
    init_jwt_utils(
        include_bytes!("../keys/private_key.pem"),
        include_bytes!("../keys/public_key.pem"),
    );

    let router = init_router().await;
    let server = application_init().await;
    init_handle(server.handle());
    tokio::spawn(shutdown_signal());
    (server, router)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => info!("ctrl_c signal received"),
        () = terminate => info!("terminate signal received"),
    }
    get_handle().stop_graceful(std::time::Duration::from_secs(60));
}
