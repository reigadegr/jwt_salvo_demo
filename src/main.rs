#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::non_std_lazy_statics,
    clippy::similar_names,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc
)]

mod config;
mod exclusive;
mod jwt;
mod models;
use exclusive::write_response::{render_error, render_success};
use jwt::{generate_token, validate_token};
use salvo::{http::StatusCode, prelude::*};
use serde::Deserialize;
use simd_json::json;
use stringzilla::sz;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[handler]
async fn login(req: &mut Request, res: &mut Response) {
    let login_req: LoginRequest = req.parse_json().await.unwrap();
    // 模拟用户验证
    if login_req.username == "user1" && login_req.password == "password1" {
        let token = generate_token(&login_req.username).unwrap();
        return render_success(res, json!({ "token": token }), "成功生成token");
    }
    res.status_code(StatusCode::UNAUTHORIZED);
    return render_error(res, "Invalid credentials");
}

#[handler]
async fn profile(req: &mut Request, res: &mut Response) {
    if let Some(token) = req.header("Authorization") {
        let token: &str = token;
        let token = token.split_whitespace().last().unwrap();
        if let Ok(claims) = validate_token(token) {
            return render_success(res, json!({ "user": claims  }), "成功获取用户信息");
        }
        res.status_code(StatusCode::FORBIDDEN);
        return render_error(res, "Invalid token");
    }
    res.status_code(StatusCode::UNAUTHORIZED);
    return render_error(res, "No token provided");
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(Router::with_path("login").post(login))
        .push(
            Router::new()
                .hoop(jwt_auth)
                .push(Router::with_path("profile").get(profile)),
        );

    let acceptor = TcpListener::new("127.0.0.1:3000").bind().await;
    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn jwt_auth(req: &mut Request, res: &mut Response) {
    let Some(token) = req.header("Authorization") else {
        res.status_code(StatusCode::UNAUTHORIZED);
        return render_error(res, "No token provided");
    };

    let token: &str = token;
    #[cfg(debug_assertions)]
    println!("{token}");

    let Some(pos) = sz::find(token, " ") else {
        res.status_code(StatusCode::UNAUTHORIZED);
        return render_error(res, "Invalid token format");
    };
    let jwt_token = &token[pos + 1..];
    if validate_token(jwt_token).is_err() {
        res.status_code(StatusCode::FORBIDDEN);
        return render_error(res, "Invalid token");
    }
}
