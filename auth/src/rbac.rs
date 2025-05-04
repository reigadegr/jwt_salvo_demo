use crate::models::Claims;
use anyhow::Result;
use casbin::{CoreApi, DefaultModel, Enforcer, StringAdapter};
use salvo::{
    http::StatusError,
    {Depot, Request},
};
use salvo_casbin::{CasbinHoop, CasbinVals};
use tokio::sync::OnceCell;

const POLICY: &str = include_str!("../casbin/rbac_with_pattern_policy.csv");
const MODEL_CFG: &str = include_str!("../casbin/rbac_with_pattern_model.conf");
static MODEL: OnceCell<DefaultModel> = OnceCell::const_new();

pub async fn init_model() {
    let m = DefaultModel::from_str(MODEL_CFG).await.unwrap();
    MODEL.get_or_init(|| async { m }).await;
}

fn get_model() -> &'static DefaultModel {
    MODEL.get().expect("Model not initialized")
}

pub async fn manage_casbin_hoop()
-> CasbinHoop<Enforcer, fn(&mut Request, &mut Depot) -> Result<Option<CasbinVals>, StatusError>> {
    let m = get_model().clone();

    //定义配置
    let a = StringAdapter::new(POLICY);

    let enforcer = Enforcer::new(m, a).await.unwrap();

    CasbinHoop::new(enforcer, false, |_req, depot| {
        let Ok(auth) = depot.get::<Claims>("user") else {
            return Err(StatusError::bad_request());
        };

        Ok(Some(CasbinVals {
            subject: auth.role.clone(),
            domain: None,
        }))
    })
}
