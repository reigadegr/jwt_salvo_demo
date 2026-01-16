use anyhow::anyhow;
use once_cell::sync::OnceCell;
use salvo::server::ServerHandle;
use tokio::signal;
use tracing::info;

static SERVER_HANDLE: OnceCell<ServerHandle> = OnceCell::new();

pub fn init_handle(handle: ServerHandle) {
    SERVER_HANDLE
        .set(handle)
        .map_err(|_| anyhow!("Failed to set server handle."))
        .unwrap();
}

pub fn get_handle() -> &'static ServerHandle {
    SERVER_HANDLE.get().unwrap()
}

pub async fn shutdown_signal() {
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

    #[cfg(debug_assertions)]
    let timeout = std::time::Duration::from_secs(1);

    #[cfg(not(debug_assertions))]
    let timeout = std::time::Duration::from_secs(60);

    get_handle().stop_graceful(timeout);
}
