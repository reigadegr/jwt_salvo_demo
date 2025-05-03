pub mod redis;
pub mod result;
pub mod router;
pub mod write_response;

use crate::rbac::casbin::init_model;
use redis::init_redis_pool;
use router::init_router;
use salvo::prelude::*;

async fn use_http1(router: Router) {
    let acceptor = TcpListener::new("0.0.0.0:3000").bind().await;
    Server::new(acceptor).serve(router).await;
}

pub async fn salvo_application_start() {
    init_model().await;
    init_redis_pool().await;
    use_http1(init_router().await).await;
}
