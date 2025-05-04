#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::non_std_lazy_statics,
    clippy::similar_names,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc
)]

mod controller;
mod router;

use dev_kit::application_init;
use router::init_router;
use salvo::prelude::*;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    let acceptor = application_init().await;
    let router = init_router().await;
    Server::new(acceptor).serve(router).await;
}
