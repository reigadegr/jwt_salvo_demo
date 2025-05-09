use salvo::{http::StatusCode, prelude::*};
use serde::Serialize;

// 定义响应数据结构体
#[derive(Serialize, Debug)]
pub struct ResData<'a, T> {
    pub code: i8,
    #[serde(borrow)]
    pub msg: &'a str,
    #[serde(borrow)]
    pub data: Option<&'a T>,
}

// 统一响应结构体的实现
impl<'a, T> ResData<'a, T> {
    pub const fn success(data: &'a T, message: &'a str) -> Self {
        ResData {
            code: 0,
            msg: message,
            data: Some(data),
        }
    }

    #[must_use]
    pub const fn error(message: &'a str) -> Self {
        ResData {
            code: -1,
            msg: message,
            data: None,
        }
    }
}

pub fn render_success<T>(res: &mut Response, data: T, msg: &str)
where
    T: serde::Serialize + Sync,
{
    let data = ResData::success(&data, msg);
    res.render(Json(data));
}

pub fn render_error(res: &mut Response, msg: &str, status_code: StatusCode) {
    res.status_code(status_code);
    let data: ResData<()> = ResData::error(msg);
    res.render(Json(data));
}
