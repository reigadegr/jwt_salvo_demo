use dev_kit::nacos::rpc::forward_post;
use dev_kit::{
    graceful_stop::get_handle,
    jwt_utils::{get_claims, secret_key::get_jwt_utils},
    nacos::deregister_instance,
    redisync::redis_set_with_expiry,
    result::{render_error, render_success},
};
use salvo::{http::StatusCode, prelude::*};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct LoginRequest<'a> {
    username: &'a str,
    password: &'a str,
}

#[handler]
pub async fn forward_test(req: &mut Request, res: &mut Response) {
    let rs = forward_post(req, "salvo-4000", "login", None, None).await;
    println!("rs={rs:?}");
    match rs {
        Ok(rs) => render_success(res, rs, "成功转发"),
        Err(e) => render_error(
            res,
            &format!("Cannot forward: {e}"),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) {
    let login_req = match req.parse_json::<LoginRequest>().await {
        Ok(login_req) => login_req,
        Err(e) => {
            let msg = format!("无法解析请求体: {e}");
            return render_error(res, &msg, StatusCode::BAD_REQUEST);
        }
    };
    println!("{login_req:?}");
    // 模拟用户验证
    if login_req.username == "user1" && login_req.password == "password1" {
        let role = "admin";
        let (token, exp_time) = get_jwt_utils().generate_token(role, login_req.username);
        let Ok(token) = token else {
            return render_error(
                res,
                "Server has some error.",
                StatusCode::INTERNAL_SERVER_ERROR,
            );
        };
        // 把token保存到Redis
        let save_token = redis_set_with_expiry(login_req.username, &token, exp_time).await;
        if save_token.is_err() {
            return render_error(
                res,
                "Server has some error.",
                StatusCode::INTERNAL_SERVER_ERROR,
            );
        }
        return render_success(res, &token, "成功生成token");
    }
    render_error(res, "Invalid credentials", StatusCode::UNAUTHORIZED);
}

#[handler]
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

#[handler]
pub async fn hello(res: &mut Response) {
    render_success(res, "Hello World", "OK");
}

#[handler]
pub async fn graceful_stop(req: &Request, res: &mut Response) {
    deregister_instance().await;
    let time = req.param::<u64>("secs").unwrap();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(time)).await;
        get_handle().stop_graceful(None);
    });
    render_success(res, "开始停止接收请求", "OK");
}
