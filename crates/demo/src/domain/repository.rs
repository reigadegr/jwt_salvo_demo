use std::future::Future;

use anyhow::Result;

use crate::domain::entity::{User, UserId};

/// 用户仓储接口 — 领域层定义，基础设施层实现
pub trait UserRepository {
    fn find_by_username(&self, username: &str)
    -> impl Future<Output = Result<Option<User>>> + Send;

    fn find_by_id(&self, id: &UserId) -> impl Future<Output = Result<Option<User>>> + Send;
}
