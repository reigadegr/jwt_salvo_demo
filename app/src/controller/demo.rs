use my_ext::result::{render_error, render_success};
use my_jwt::jwt_utils::{get_claims, secret_key::get_jwt_utils};
use salvo::{http::StatusCode, prelude::*};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct LoginRequest<'a> {
    username: &'a str,
    password: &'a str,
}

// 模拟用户凭证数据库（实际项目应从DB读取）
const USER_DB: &[(&str, &str, &str)] = &[
    ("user1", "password1", "token-admin"),
    ("user2", "password2", "yz666"),
];

#[endpoint]
pub async fn login(req: &mut Request, res: &mut Response) {
    let login_req = match req.parse_json::<LoginRequest>().await {
        Ok(login_req) => login_req,
        Err(e) => {
            return render_error(
                res,
                &format!("无法解析请求体: {e}"),
                StatusCode::BAD_REQUEST,
            );
        }
    };

    #[cfg(debug_assertions)]
    println!("{login_req:?}");

    // 统一验证逻辑：查找用户 → 生成token
    if let Some(role) = authenticate_user(login_req.username, login_req.password) {
        generate_token_response(res, role, login_req.username);
    } else {
        render_error(res, "Invalid credentials", StatusCode::UNAUTHORIZED);
    }
}

// 抽离：用户认证逻辑（查询匹配）
fn authenticate_user(username: &str, password: &str) -> Option<&'static str> {
    USER_DB
        .iter()
        .find(|(u, p, _)| *u == username && *p == password)
        .map(|(_, _, role)| *role)
}

// 抽离：Token生成与响应处理
fn generate_token_response(res: &mut Response, role: &str, username: &str) {
    match get_jwt_utils().generate_token(role, username) {
        Ok(token) => render_success(res, &token, "成功生成token"),
        Err(_) => render_error(
            res,
            "Server has some error.",
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
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
