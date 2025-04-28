use salvo::Request;
use serde_json::Value;

pub async fn get_request_body(req: &mut Request) -> Value {
    let payload = req.payload().await.unwrap();
    // 将 Bytes 转换为 &[u8]
    let bytes_slice = payload.as_ref();
    let rs = String::from_utf8(bytes_slice.to_vec()).unwrap_or_default();
    // 解析 JSON 字符串为 serde_json::Value
    let json: Value = serde_json::from_str(&rs).unwrap_or_default();
    json
}

pub fn get_field(json: &Value, field: &str) -> String {
    json.get(field).map_or_else(String::new, |value| {
        value
            .as_str()
            .map_or_else(String::new, std::string::ToString::to_string)
    })
}
