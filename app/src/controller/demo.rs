use dev_kit::{
    jwt_utils::{get_claims, secret_key::get_jwt_utils},
    result::{render_error, render_success},
};
use salvo::{http::StatusCode, prelude::*};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct LoginRequest<'a> {
    username: &'a str,
    password: &'a str,
}

#[endpoint]
pub async fn login(req: &mut Request, res: &mut Response) {
    let login_req = match req.parse_json::<LoginRequest>().await {
        Ok(login_req) => login_req,
        Err(e) => {
            let msg = format!("无法解析请求体: {e}");
            return render_error(res, &msg, StatusCode::BAD_REQUEST);
        }
    };
    #[cfg(debug_assertions)]
    println!("{login_req:?}");
    // 模拟用户验证
    if login_req.username == "user1" && login_req.password == "password1" {
        let role = "admin";
        let token = get_jwt_utils().generate_token(role, login_req.username);
        let Ok(token) = token else {
            return render_error(
                res,
                "Server has some error.",
                StatusCode::INTERNAL_SERVER_ERROR,
            );
        };
        return render_success(res, &token, "成功生成token");
    }
    render_error(res, "Invalid credentials", StatusCode::UNAUTHORIZED);
}

#[endpoint]
pub async fn profile(res: &mut Response, depot: &Depot) {
    match get_claims(depot) {
        Ok(user) => render_success(res, user, "成功获取用户信息"),
        Err(_) => render_error(
            res,
            "Can not get now user.",
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}

#[endpoint]
pub async fn hello(res: &mut Response) {
    render_success(res, "Hello World", "OK");
}
