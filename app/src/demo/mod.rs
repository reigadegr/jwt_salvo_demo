use my_demo::{dto::LoginRequest, facade::AuthFacade, repository::DatabaseUserRepository};
use my_ext::result::{render_error, render_success};
use my_jwt::jwt_utils::get_claims;
use salvo::{http::StatusCode, prelude::*};
use sea_orm::DatabaseConnection;

/// 登录端点
#[endpoint]
pub async fn login(req: &mut Request, res: &mut Response, depot: &Depot) {
    let Ok(conn) = depot.obtain::<DatabaseConnection>() else {
        return render_error(res, "数据库连接不可用", StatusCode::INTERNAL_SERVER_ERROR);
    };

    let login_req = match req.parse_json::<LoginRequest>().await {
        Ok(r) => r,
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

    let repo = DatabaseUserRepository::new(conn);
    let facade = AuthFacade::new(repo);

    match facade.login(&login_req).await {
        Ok(Some(resp)) => render_success(res, &resp.token, "成功生成token"),
        Ok(None) => render_error(res, "Invalid credentials", StatusCode::UNAUTHORIZED),
        Err(_) => render_error(
            res,
            "Server has some error.",
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}

/// 获取当前用户信息端点
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

/// 健康检查端点
#[endpoint]
pub async fn hello(res: &mut Response) {
    render_success(res, "Hello World", "OK");
}
