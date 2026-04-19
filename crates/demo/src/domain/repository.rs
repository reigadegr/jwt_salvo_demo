use std::future::Future;

use anyhow::Result;

use crate::domain::entity::{User, UserId};

/// 用户仓储接口 — 领域层定义抽象，基础设施层实现
///
/// Repository: 封装数据访问，提供类似集合的接口。
/// 仅定义领域所需的最小操作，不暴露持久化细节。
pub trait UserRepository: Send + Sync {
    fn find_by_username(&self, username: &str)
    -> impl Future<Output = Result<Option<User>>> + Send;
    fn find_by_id(&self, id: &UserId) -> impl Future<Output = Result<Option<User>>> + Send;
}
