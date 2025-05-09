use crate::controller::{graceful_stop, hello, login, profile};
use dev_kit::{jwt_utils::middleware::jwt_auth, rbac::create_casbin_hoop};
use salvo::{Router, prelude::*};
use std::time::Duration;

const MODEL: &str = include_str!("../casbin/rbac_with_pattern_model.conf");
const POLICY: &str = include_str!("../casbin/rbac_with_pattern_policy.csv");

pub async fn init_router() -> Router {
    let casbin_hoop = create_casbin_hoop(MODEL, POLICY).await;
    Router::new()
        .get(hello)
        .push(Router::with_path("stop/{secs}").get(graceful_stop))
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
