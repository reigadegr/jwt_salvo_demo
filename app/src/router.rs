use std::time::Duration;

use anyhow::Context;
use my_casbin::rbac::create_casbin_hoop;
use my_config::config::get_cfg;
use my_demo::init_database_schema;
use my_jwt::jwt_utils::middleware::jwt_auth;
use my_orm::init_db;
use my_server_handle::graceful_stop;
use salvo::{Router, affix_state, prelude::*};
use sea_orm::DatabaseConnection;

use crate::demo::{hello, login, profile};

const MODEL: &str = include_str!("../casbin/rbac_with_pattern_model.conf");
const POLICY: &str = include_str!("../casbin/rbac_with_pattern_policy.csv");

#[derive(Debug, Clone)]
struct AppState {
    #[allow(dead_code)]
    pub conn: DatabaseConnection,
}

pub async fn init_router() -> anyhow::Result<Router> {
    let casbin_hoop = create_casbin_hoop(MODEL, POLICY).await?;

    let cfg = get_cfg()?;
    let conn = init_db(&cfg.database_cfg.db_url).await.context(format!(
        "Failed to initialize database at: {}",
        cfg.database_cfg.db_url
    ))?;

    // 初始化数据库架构（检查库、表是否存在，不存在则创建并插入默认数据）
    init_database_schema(&conn)
        .await
        .context("Failed to initialize database schema")?;

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
    Ok(router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(Scalar::new("/api-doc/openapi.json").into_router("scalar")))
}
