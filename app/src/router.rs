use crate::controller::{
    demo::{hello, login, profile},
    graceful_stop,
};
use my_casbin::rbac::create_casbin_hoop;
use my_jwt::jwt_utils::middleware::jwt_auth;
use salvo::{Router, prelude::*};
use std::time::Duration;

const MODEL: &str = include_str!("../casbin/rbac_with_pattern_model.conf");
const POLICY: &str = include_str!("../casbin/rbac_with_pattern_policy.csv");

pub async fn init_router() -> Router {
    let casbin_hoop = create_casbin_hoop(MODEL, POLICY).await;
    let router = Router::new()
        .hoop(Logger::new())
        .goal(hello)
        .hoop(max_concurrency(200))
        .hoop(Timeout::new(Duration::from_secs(5)))
        .push(Router::with_path("login").post(login))
        .push(
            Router::new()
                .hoop(jwt_auth)
                .hoop(casbin_hoop)
                .push(Router::with_path("profile").get(profile))
                .push(Router::with_path("stop/{secs}").get(graceful_stop)),
        );

    let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
    router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(Scalar::new("/api-doc/openapi.json").into_router("scalar"))
}
