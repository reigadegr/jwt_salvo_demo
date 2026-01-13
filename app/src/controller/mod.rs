pub mod demo;

use dev_kit::{nacos::deregister_instance, result::render_success, server_handle::get_handle};
use salvo::prelude::*;

#[endpoint]
pub async fn graceful_stop(req: &Request, res: &mut Response) {
    deregister_instance().await;
    let time = req.param::<u64>("secs").unwrap_or(1);
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(time)).await;
        get_handle().stop_graceful(std::time::Duration::from_secs(60));
    });
    render_success(res, "开始停止接收请求", "OK");
}
