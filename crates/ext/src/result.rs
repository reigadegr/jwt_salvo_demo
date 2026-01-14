use salvo::{http::StatusCode, prelude::*};
use serde::Serialize;
use serde_json::to_vec;

// 定义响应数据结构体
#[derive(Serialize, Debug)]
pub struct ResData<'a, T: ?Sized> {
    pub code: i8,
    #[serde(borrow)]
    pub msg: &'a str,
    #[serde(borrow)]
    pub data: Option<&'a T>,
}

// 统一响应结构体的实现
impl<'a, T: ?Sized> ResData<'a, T> {
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

// 实现Scribe trait
impl<T: Serialize> Scribe for ResData<'_, T> {
    fn render(self, res: &mut Response) {
        if let Ok(json_bytes) = to_vec(&self) {
            let _ = res.write_body(json_bytes);
        }
    }
}

pub fn render_success<'a, T>(res: &mut Response, data: &'a T, msg: &'a str)
where
    T: Serialize + Sync + ?Sized,
{
    let data = ResData::success(data, msg);
    res.render(Json(data));
}

pub fn render_error(res: &mut Response, msg: &str, status_code: StatusCode) {
    res.status_code(status_code);
    let data: ResData<()> = ResData::error(msg);
    res.render(Json(data));
}
