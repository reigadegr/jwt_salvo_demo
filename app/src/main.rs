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
    clippy::missing_panics_doc
)]

mod controller;
mod router;
mod sea_orm;

use crate::router::init_router;
use my_config::config::init_config;
use my_jwt::jwt_utils::secret_key::init_jwt_utils;
use my_server_handle::init_server;
use obfstr::{obfbytes, obfstr};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = init_config(obfstr!(include_str!("../application.toml")));
    let private_key = obfbytes!(include_bytes!("../../keys/private_key.pem"));
    let public_key = obfbytes!(include_bytes!("../../keys/public_key.pem"));

    let _ = init_jwt_utils(private_key, public_key);

    let server = init_server().await;
    let router = init_router().await?;
    server.serve(router).await;
    Ok(())
}
