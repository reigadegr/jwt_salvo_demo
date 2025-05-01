use crate::{
    config::redis::redis_write_and_rm,
    exclusive::write_response::{render_error, render_success},
    jwt::{models::Claims, secert_key::generate_token},
};
use salvo::{http::StatusCode, prelude::*};
use serde::Deserialize;

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
        let (token, exp_time) = generate_token(role, login_req.username);
        let Ok(token) = token else {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return render_error(res, "Server has some error.");
        };
        // 把token保存到Redis
        let save_token = redis_write_and_rm(login_req.username, &token, exp_time).await;
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
