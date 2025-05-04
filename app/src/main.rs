#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::non_std_lazy_statics,
    clippy::similar_names,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc
)]

mod controller;
mod router;

use auth::rbac::init_model;
use dev_kit::redisync::init_redis_pool;
use router::init_router;
use salvo::prelude::*;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    salvo_application_start().await;
}

async fn use_http1(router: Router) {
    let acceptor = TcpListener::new("0.0.0.0:3000").bind().await;
    Server::new(acceptor).serve(router).await;
}

pub async fn salvo_application_start() {
    init_model().await;
    init_redis_pool().await;
    use_http1(init_router().await).await;
}
