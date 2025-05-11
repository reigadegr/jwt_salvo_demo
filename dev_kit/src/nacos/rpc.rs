use super::naming_service::get_naming_service;
use anyhow::Result;
use nacos_sdk::api::naming::ServiceInstance;
use salvo::http::Request;
use serde_json::{Value, from_str};
use std::borrow::Cow;

async fn get_healthy_instance(
    service_name: &str,
    group: &str,
    cluster: Vec<String>,
) -> Result<ServiceInstance> {
    let instance = get_naming_service()
        .select_one_healthy_instance(
            String::from(service_name),
            Some(group.to_string()),
            cluster,
            true,
        )
        .await?;
    Ok(instance)
}

pub async fn forward_request_post(
    req: &mut Request,
    instance: ServiceInstance,
    path: &str,
) -> Result<String> {
    // 创建一个reqwest客户端
    let client = reqwest::Client::new();

    let mut headers = req.headers().clone();

    // 移除可能干扰转发的头字段
    headers.remove("content-length");
    println!("请求头: {headers:?}");

    // 获取请求体
    let payload = (req.payload().await).map_or(Cow::Borrowed(""), |data| {
        String::from_utf8_lossy(data.as_ref())
    });
    let payload: Value = from_str(&payload).unwrap();
    println!("请求体: {payload:?}");

    // 创建一个请求构建器
    let response = client
        .post(format!("http://{}/{}", instance.ip_and_port(), path))
        // 添加请求头
        .headers(headers)
        // 添加请求体
        .json(&payload)
        // 发送请求
        .send()
        .await?;

    // 检查响应状态
    println!("状态码: {}", response.status());

    // 获取响应体
    let body = response.text().await?;
    println!("响应体: {body}");
    Ok(body)
}

pub async fn forward_post(
    req: &mut Request,
    service_name: &str,
    path: &str,
    group: &str,
    cluster: Vec<String>,
) -> Result<String> {
    let instance = get_healthy_instance(service_name, group, cluster).await?;
    let res_body = forward_request_post(req, instance, path).await?;
    Ok(res_body)
}
