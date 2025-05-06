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
    application_init, config::init_config, jwt_utils::secret_key::init_jwt_utils,
    tracing_subscriber,
};
use router::init_router;
use salvo::prelude::*;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    init_config(include_str!("../application.toml"));
    init_jwt_utils(
        include_bytes!("../keys/private_key.pem"),
        include_bytes!("../keys/public_key.pem"),
    );
    let acceptor = application_init().await;
    let router = init_router().await;
    Server::new(acceptor).serve(router).await;
}
