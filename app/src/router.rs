use crate::controller::{hello, login, profile};
use dev_kit::{jwt_utils::middleware::jwt_auth, rbac::create_casbin_hoop};
use salvo::{Router, prelude::*};
use std::time::Duration;

pub async fn init_router() -> Router {
    let casbin_hoop = create_casbin_hoop().await;
    Router::new()
        .push(Router::with_path("/").get(hello))
        .hoop(max_concurrency(200))
        .hoop(Timeout::new(Duration::from_secs(5)))
        .push(Router::with_path("login").post(login))
        .push(
            Router::new()
                .hoop(jwt_auth)
                .hoop(casbin_hoop)
                .push(Router::with_path("profile").get(profile)),
        )
}
