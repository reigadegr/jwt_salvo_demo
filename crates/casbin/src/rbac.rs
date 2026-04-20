use anyhow::{Context, Result};
use casbin::{CoreApi, DefaultModel, Enforcer, StringAdapter};
use my_jwt::jwt_utils::get_claims;
use salvo::{Depot, Request, http::StatusError};
use salvo_casbin::{CasbinHoop, CasbinVals};

pub async fn create_casbin_hoop(
    model: &str,
    policy: &str,
) -> Result<
    CasbinHoop<Enforcer, fn(&mut Request, &mut Depot) -> Result<Option<CasbinVals>, StatusError>>,
> {
    //定义配置
    let m = DefaultModel::from_str(model)
        .await
        .context("Failed to create Casbin model")?;
    let a = StringAdapter::new(policy);
    let enforcer = Enforcer::new(m, a)
        .await
        .context("Failed to create Casbin enforcer")?;

    Ok(CasbinHoop::new(enforcer, false, |req, depot| {
        let Ok(auth) = get_claims(depot) else {
            return Err(StatusError::bad_request());
        };

        #[cfg(debug_assertions)]
        println!(
            "🔒 Casbin Check - role: {}, path: {}, method: {}",
            auth.role,
            req.uri().path(),
            req.method().as_str()
        );

        Ok(Some(CasbinVals {
            subject: auth.role.clone(),
            domain: None,
        }))
    }))
}
