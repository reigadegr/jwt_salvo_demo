use casbin::{CoreApi, DefaultModel, Enforcer, StringAdapter};
use my_jwt::jwt_utils::get_claims;
use salvo::{
    http::StatusError,
    {Depot, Request},
};
use salvo_casbin::{CasbinHoop, CasbinVals};

pub async fn create_casbin_hoop(
    model: &str,
    policy: &str,
) -> CasbinHoop<Enforcer, fn(&mut Request, &mut Depot) -> Result<Option<CasbinVals>, StatusError>> {
    //定义配置
    let m = DefaultModel::from_str(model).await.unwrap();
    let a = StringAdapter::new(policy);
    let enforcer = Enforcer::new(m, a).await.unwrap();

    CasbinHoop::new(enforcer, false, |_req, depot| {
        let Ok(auth) = get_claims(depot) else {
            return Err(StatusError::bad_request());
        };

        Ok(Some(CasbinVals {
            subject: auth.role.clone(),
            domain: None,
        }))
    })
}
