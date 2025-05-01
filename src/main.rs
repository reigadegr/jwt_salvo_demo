#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::non_std_lazy_statics,
    clippy::similar_names,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc
)]

mod config;
mod controller;
mod exclusive;
mod jwt;
mod rbac;

use config::redis::init_redis_pool;
use controller::{jwt_auth, login, profile};
use rbac::casbin::{init_model, manage_casbin_hoop};
use salvo::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() {
    init_model().await;
    init_redis_pool().await;
    let router = Router::new()
        .hoop(max_concurrency(200))
        .hoop(Timeout::new(Duration::from_secs(5)))
        .push(Router::with_path("login").post(login))
        .push(
            Router::new()
                .hoop(jwt_auth)
                .hoop(manage_casbin_hoop().await)
                .push(Router::with_path("profile").get(profile)),
        );

    let acceptor = TcpListener::new("0.0.0.0:3000").bind().await;
    Server::new(acceptor).serve(router).await;
}
