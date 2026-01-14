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
    clippy::missing_panics_doc
)]

mod controller;
mod router;

use crate::router::init_router;
use my_config::config::init_config;
use my_server_handle::init_misc;
use obfstr::obfstr;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    init_config(obfstr!(include_str!("../application.toml")));
    let server = init_misc().await;
    let router = init_router().await;
    server.serve(router).await;
}
