use crate::controller::{login, profile};
use jwt_utils::middleware::jwt_auth;
use rbac::manage_casbin_hoop;
use salvo::{Router, prelude::*};
use std::time::Duration;

pub async fn init_router() -> Router {
    Router::new()
        .hoop(max_concurrency(200))
        .hoop(Timeout::new(Duration::from_secs(5)))
        .push(Router::with_path("login").post(login))
        .push(
            Router::new()
                .hoop(jwt_auth)
                .hoop(manage_casbin_hoop().await)
                .push(Router::with_path("profile").get(profile)),
        )
}
