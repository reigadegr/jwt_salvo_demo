#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::non_std_lazy_statics,
    clippy::similar_names,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc
)]

mod controller;
mod router;

use anyhow::anyhow;
use dev_kit::{
    application_init, config::init_config, jwt_utils::secret_key::init_jwt_utils,
    tracing_subscriber,
};
use once_cell::sync::OnceCell;
use router::init_router;
use salvo::server::ServerHandle;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub static SERVER_HANDLE: OnceCell<ServerHandle> = OnceCell::new();

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    init_config(include_str!("../application.toml"));
    init_jwt_utils(
        include_bytes!("../keys/private_key.pem"),
        include_bytes!("../keys/public_key.pem"),
    );

    let router = init_router().await;
    let server = application_init().await;
    init_handle(server.handle());
    server.serve(router).await;
}

pub fn init_handle(app_config: ServerHandle) {
    SERVER_HANDLE
        .set(app_config)
        .map_err(|_| anyhow!("Failed to set server handle."))
        .unwrap();
}

pub fn get_handle() -> &'static ServerHandle {
    SERVER_HANDLE.get().unwrap()
}
