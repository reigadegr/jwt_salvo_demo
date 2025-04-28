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
mod models;
mod rbac;

use controller::{jwt_auth, login, profile};
use rbac::casbin::manage_casbin_hoop;
use salvo::prelude::*;

#[tokio::main]
async fn main() {
    let router = Router::new()
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
