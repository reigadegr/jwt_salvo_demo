use crate::{
    config::{redis::redis_read, write_response::render_error},
    jwt::secert_key::validate_token,
};
use salvo::{http::StatusCode, prelude::*};
use stringzilla::sz;

#[handler]
pub async fn jwt_auth(req: &mut Request, res: &mut Response, depot: &mut Depot) {
    let Some(token) = req.header("Authorization") else {
        res.status_code(StatusCode::UNAUTHORIZED);
        return render_error(res, "No token provided");
    };

    let token: &str = token;
    #[cfg(debug_assertions)]
    println!("{token}");

    let jwt_token: &str = sz::find(token, " ").map_or(token, |p| token[p + 1..].trim_start());

    if let Ok(claims) = validate_token(jwt_token) {
        match redis_read(&claims.username).await {
            Ok(redis_token) if redis_token == jwt_token => {
                depot.insert("user", claims);
            }
            Ok(_) => {
                // Token存在但不匹配，返回401 Unauthorized
                res.status_code(StatusCode::UNAUTHORIZED);
                render_error(res, "Token has expired.");
            }
            Err(_) => {
                // Redis操作失败，返回500 InternalServerError
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                render_error(res, "Server internal error");
            }
        }
    } else {
        res.status_code(StatusCode::FORBIDDEN);
        render_error(res, "Invalid token");
    }
}
