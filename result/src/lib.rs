use salvo::prelude::{Json, Response};
use serde::Serialize;
use simd_json::json;

// 定义响应数据结构体
#[derive(Serialize, Debug)]
pub struct ResData<'a, T> {
    pub code: i8,
    #[serde(borrow)]
    pub msg: &'a str,
    #[serde(borrow)]
    pub data: Option<&'a T>,
}

// 定义统一响应代码
const SUCCESS_CODE: i8 = 0;
const ERROR_CODE: i8 = -1;

// 统一响应结构体的实现
impl<'a, T> ResData<'a, T> {
    pub const fn success(data: &'a T, message: &'a str) -> Self {
        ResData {
            code: SUCCESS_CODE,
            msg: message,
            data: Some(data),
        }
    }

    pub const fn error(message: &'a str) -> Self {
        ResData {
            code: ERROR_CODE,
            msg: message,
            data: None,
        }
    }
}

pub fn render_success<T>(res: &mut Response, data: T, msg: &str)
where
    T: serde::Serialize,
{
    let data = ResData::success(&data, msg);
    res.render(Json(json!(data)));
}

pub fn render_error(res: &mut Response, msg: &str) {
    let data: ResData<()> = ResData::error(msg);
    res.render(Json(json!(data)));
}
