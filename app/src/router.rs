use crate::{
    controller::{
        demo::{hello, login, profile},
        graceful_stop,
    },
    sea_orm::init_db,
};
use my_casbin::rbac::create_casbin_hoop;
use my_jwt::jwt_utils::middleware::jwt_auth;
use obfstr::obfstr;
use salvo::{Router, affix_state, prelude::*};
use sea_orm::DatabaseConnection;
use std::time::Duration;

const MODEL: &str = include_str!("../casbin/rbac_with_pattern_model.conf");
const POLICY: &str = include_str!("../casbin/rbac_with_pattern_policy.csv");

#[derive(Debug, Clone)]
struct AppState {
    pub conn: DatabaseConnection,
}

pub async fn init_router() -> Router {
    let casbin_hoop = create_casbin_hoop(MODEL, POLICY).await;

    let conn = init_db(obfstr!("postgres://user:pass@127.0.0.1:5432/db"))
        .await
        .unwrap();
    let state = AppState { conn };

    let router = Router::new()
        .hoop(Logger::new())
        .hoop(affix_state::inject(state))
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
