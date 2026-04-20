use anyhow::{Context, Result};
use casbin::{CoreApi, DefaultModel, Enforcer, StringAdapter};
use my_jwt::jwt_utils::{save_claims, secret_key::get_jwt_utils};
use salvo::{Depot, Request, http::StatusError};
use salvo_casbin::{CasbinHoop, CasbinVals};

pub async fn create_casbin_hoop(
    model: &str,
    policy: &str,
) -> Result<
    CasbinHoop<Enforcer, fn(&mut Request, &mut Depot) -> Result<Option<CasbinVals>, StatusError>>,
> {
    let m = DefaultModel::from_str(model)
        .await
        .context("Failed to create Casbin model")?;
    let a = StringAdapter::new(policy);
    let enforcer = Enforcer::new(m, a)
        .await
        .context("Failed to create Casbin enforcer")?;

    Ok(CasbinHoop::new(enforcer, false, |req, depot| {
        let Some(token) = req.header::<&str>("Authorization") else {
            return Err(StatusError::unauthorized());
        };

        let jwt_token = token.strip_prefix("Bearer ").unwrap_or(token);

        let jwt_utils = get_jwt_utils().map_err(|_| StatusError::internal_server_error())?;

        let claims = jwt_utils
            .validate_token(jwt_token)
            .map_err(|_| StatusError::forbidden())?;

        #[cfg(debug_assertions)]
        println!(
            "🔒 Casbin Check - role: {}, path: {}, method: {}",
            claims.role,
            req.uri().path(),
            req.method().as_str()
        );

        save_claims(depot, claims.clone());

        Ok(Some(CasbinVals {
            subject: claims.role,
            domain: None,
        }))
    }))
}
