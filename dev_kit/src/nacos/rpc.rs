use super::naming_service::get_naming_service;
use anyhow::Result;
use nacos_sdk::api::naming::ServiceInstance;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;
use reqwest::header::HeaderValue;
use salvo::http::Request;
use serde_json::json;
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

pub async fn forward_request(
    req: &mut Request,
    // instance: ServiceInstance,
    path: &str,
) -> Result<()> {
    // let target_url = format!("http://{}/{}", instance.ip_and_port(), path);

    let method = req.method();
    println!("请求方法: {method}");

    // 获取请求头
    let headers = req.headers().clone();
    println!("请求头: {headers:?}");

    // 获取请求体
    let payload = (req.payload().await).map_or(Cow::Borrowed(""), |data| {
        String::from_utf8_lossy(data.as_ref())
    });
    // let payload = String::from_utf8_lossy(&payload);
    let payload: Value = from_str(&payload).unwrap();
    println!("请求体: {payload:?}");
    let request_body: Value =
        from_str("{\"username\":\"user1\",\"password\":\"password1\"}").unwrap();
    println!("请求体2: {request_body:?}");

    let client = reqwest::Client::new();
    println!("开始fas");

    let response = client
        .post("http://127.0.0.1:4000/login")
        // 添加请求头
        .headers(headers)
        // 添加请求体
        .json(&request_body)
        // 发送请求
        .send()
        .await?;

    println!("状态码: {}", response.status());

    // 获取响应体
    let body = response.text().await?;
    println!("响应体: {body}");
    Ok(())
}

pub async fn forward_request2(
    req: &mut Request,
    // instance: ServiceInstance,
    path: &str,
) -> Result<()> {
    // 创建一个reqwest客户端
    let client = reqwest::Client::new();

    // 发送GET请求到http://127.0.0.1:4000/

    let request_body: Value =
        from_str("{\"username\":\"user1\",\"password\":\"password1\"}").unwrap();

    let head: Value = json! ({"content-type": "application/json", "accept": "*/*", "host": "127.0.0.1:4000", "content-length": "43","user-agent": "curl/8.13.0",});
    let mut headers = HeaderMap::new();

    // 确保 head 是一个 JSON 对象
    if let Value::Object(map) = head {
        for (k, v) in map {
            // 转换键为 HeaderName
            if let (Ok(header_name), Some(header_value)) =
                (HeaderName::try_from(k.as_str()), v.as_str())
            {
                // 插入 HeaderMap
                headers.insert(header_name, HeaderValue::try_from(header_value).unwrap());
            }
        }
    }
    println!("请求头映射: {headers:?}");
    let headers = req.headers().clone();
    println!("请求头: {headers:?}");

    // 创建一个请求构建器
    let response = client
        .post("http://127.0.0.1:4000/login")
        // 添加请求头
        .headers(headers)
        // 添加请求体
        .json(&request_body)
        // 发送请求
        .send()
        .await?;

    // 检查响应状态
    println!("状态码: {}", response.status());

    // 获取响应体
    let body = response.text().await?;
    println!("响应体: {body}");
    Ok(())
}
