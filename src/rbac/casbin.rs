use super::create_policy_csv::create_policy_file;
use crate::models::Claims;
use casbin::{CoreApi, DefaultModel, Enforcer, FileAdapter};
use salvo::{
    http::StatusError,
    {Depot, Request},
};
use salvo_casbin::{CasbinHoop, CasbinVals};

pub async fn manage_casbin_hoop()
-> CasbinHoop<Enforcer, fn(&mut Request, &mut Depot) -> Result<Option<CasbinVals>, StatusError>> {
    //定义规则
    let m = include_str!("../../casbin/rbac_with_pattern_model.conf");
    let m = DefaultModel::from_str(m).await.unwrap();
    //定义配置
    let _ = create_policy_file();
    let a = FileAdapter::new("casbin/rbac_with_pattern_policy.csv");
    CasbinHoop::new(Enforcer::new(m, a).await.unwrap(), false, |_req, depot| {
        let Ok(auth) = depot.get::<Claims>("user") else {
            return Err(StatusError::bad_request());
        };

        Ok(Some(CasbinVals {
            subject: auth.role.clone(),
            domain: None,
        }))
    })
}
