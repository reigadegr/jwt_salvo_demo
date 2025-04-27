#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::non_std_lazy_statics,
    clippy::similar_names,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc
)]

mod jwt;
mod models;
use crate::jwt::{generate_token, validate_token};
use salvo::http::StatusCode;
use salvo::prelude::*;
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
        println!("token={token}");
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
        .push(Router::with_path("profile").get(profile));

    let acceptor = TcpListener::new("127.0.0.1:3000").bind().await;
    Server::new(acceptor).serve(router).await;
}
