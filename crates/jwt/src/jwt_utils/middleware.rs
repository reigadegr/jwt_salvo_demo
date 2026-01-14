use crate::jwt_utils::{save_claims, secret_key::get_jwt_utils};
use my_ext::result::render_error;
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
        save_claims(depot, claims);
    } else {
        // 场景: 传入的token过期或者不合法
        ctrl.skip_rest();
        render_error(res, "Invalid token", StatusCode::FORBIDDEN);
    }
}
