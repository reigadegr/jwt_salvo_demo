use crate::{
    config::redis::{redis_read, redis_write},
    exclusive::write_response::{render_error, render_success},
    jwt::{generate_token, validate_token},
    models::Claims,
};
use salvo::{http::StatusCode, prelude::*};
use serde::Deserialize;
use stringzilla::sz;

#[derive(Deserialize, Default)]
struct LoginRequest<'a> {
    username: &'a str,
    password: &'a str,
}

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) {
    let login_req: LoginRequest = req.parse_json().await.unwrap_or_default();
    // 模拟用户验证
    if login_req.username == "user1" && login_req.password == "password1" {
        let role = "admin";
        let token = generate_token(role, login_req.username).unwrap_or_default();

        // 把token保存到Redis
        let save_token = redis_write(login_req.username, &token).await;
        if save_token.is_err() {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return render_error(res, "Server has some error.");
        }
        return render_success(res, &token, "成功生成token");
    }
    res.status_code(StatusCode::UNAUTHORIZED);
    return render_error(res, "Invalid credentials");
}

#[handler]
pub async fn profile(res: &mut Response, depot: &mut Depot) {
    match depot.get::<Claims>("user") {
        Ok(user) => return render_success(res, user, "成功获取用户信息"),
        Err(_) => return render_error(res, "Can not get now user."),
    }
}

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
                return render_error(res, "Token has expired.");
            }
            Err(_) => {
                // Redis操作失败，返回500 InternalServerError
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                return render_error(res, "Server internal error");
            }
        }
    } else {
        res.status_code(StatusCode::FORBIDDEN);
        return render_error(res, "Invalid token");
    }
}
