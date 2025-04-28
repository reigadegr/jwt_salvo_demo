use serde::{Deserialize, Serialize};

// 定义响应数据结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct ResData<'a, T> {
    pub code: i8,
    pub msg: &'a str,
    pub data: Option<T>,
}

// 定义统一响应代码
const SUCCESS_CODE: i8 = 0;
const ERROR_CODE: i8 = -1;

// 统一响应结构体的实现
impl<'a, T> ResData<'a, T> {
    pub const fn success(data: T, message: &'a str) -> Self {
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
