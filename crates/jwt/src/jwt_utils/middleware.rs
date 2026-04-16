use my_ext::result::render_error;
use salvo::{http::StatusCode, prelude::*};
use stringzilla::sz;

use crate::jwt_utils::{save_claims, secret_key::get_jwt_utils};

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

    let Ok(jwt_utils) = get_jwt_utils() else {
        ctrl.skip_rest();
        return render_error(
            res,
            "JWT not initialized",
            StatusCode::INTERNAL_SERVER_ERROR,
        );
    };

    if let Ok(claims) = jwt_utils.validate_token(jwt_token) {
        // 调试：打印当前用户信息
        println!(
            "🔐 JWT Auth - username: {}, role: {}",
            claims.username, claims.role
        );
        save_claims(depot, claims);
    } else {
        // 场景: 传入的token过期或者不合法
        ctrl.skip_rest();
        render_error(res, "Invalid token", StatusCode::FORBIDDEN);
    }
}
