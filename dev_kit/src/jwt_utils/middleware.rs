use super::{save_claims, secret_key::get_jwt_utils};
use crate::{redisync::redis_read, result::render_error};
use salvo::{http::StatusCode, prelude::*};
use stringzilla::sz;

#[handler]
pub async fn jwt_auth(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
    ctrl: &mut FlowCtrl,
) {
    let Some(token) = req.header("Authorization") else {
        return render_error(res, "No token provided", StatusCode::UNAUTHORIZED);
    };

    let token: &str = token;
    #[cfg(debug_assertions)]
    println!("{token}");

    let jwt_token: &str = sz::find(token, " ").map_or(token, |p| token[p + 1..].trim_start());

    if let Ok(claims) = get_jwt_utils().validate_token(jwt_token) {
        match redis_read(&claims.username).await {
            Ok(redis_token) if redis_token == jwt_token => {
                save_claims(depot, claims);
            }
            Ok(_) => {
                // Token存在但不匹配，返回401 Unauthorized
                // 场景: 修改密码后旧token被拦截
                ctrl.skip_rest();
                render_error(res, "Token has expired.", StatusCode::UNAUTHORIZED);
            }
            Err(_) => {
                // Redis操作失败，返回500 InternalServerError
                ctrl.skip_rest();
                render_error(
                    res,
                    "Server internal error",
                    StatusCode::INTERNAL_SERVER_ERROR,
                );
            }
        }
    } else {
        // 场景: 传入的token过期或者不合法
        ctrl.skip_rest();
        render_error(res, "Invalid token", StatusCode::FORBIDDEN);
    }
}
