#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::non_std_lazy_statics,
    clippy::similar_names,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc
)]

mod jwt;
mod models;
use jwt::{generate_token, validate_token};
use salvo::{http::StatusCode, prelude::*};
use serde::Deserialize;
use serde_json::json;

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
        res.render(Json(json!({ "token": token })));
    } else {
        res.status_code(StatusCode::UNAUTHORIZED);
        res.render(Json(json!({ "message": "Invalid credentials" })));
    }
}

#[handler]
async fn profile(req: &mut Request, res: &mut Response) {
    if let Some(token) = req.header("Authorization") {
        let token: &str = token;
        let token = token.split_whitespace().last().unwrap();
        if let Ok(claims) = validate_token(token) {
            res.render(Json(json!({ "user": claims })));
        } else {
            res.status_code(StatusCode::FORBIDDEN);
            res.render(Json(json!({ "message": "Invalid token" })));
        }
    } else {
        res.status_code(StatusCode::UNAUTHORIZED);
        res.render(Json(json!({ "message": "No token provided" })));
    }
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(Router::with_path("login").post(login))
        .hoop(jwt_auth)
        .push(Router::with_path("profile").get(profile));

    let acceptor = TcpListener::new("127.0.0.1:3000").bind().await;
    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn jwt_auth(req: &mut Request, res: &mut Response) {
    let path = req.uri().path();
    if path == "/login" {
        return;
    }
    if let Some(token) = req.header("Authorization") {
        println!("有token，开始校验");
        let token: &str = token;
        let token = token.split_whitespace().last().unwrap();
        if validate_token(token).is_err() {
            println!("有token，过期了");
            res.status_code(StatusCode::FORBIDDEN);
            return res.render(Json(json!({ "message": "Invalid token" })));
        }
    } else {
        println!("no token拦截");
        return res.render(Json(json!({ "message": "中间件拦截" })));
    }
}
