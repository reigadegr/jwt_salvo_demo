//! DDD 三层架构: domain / application / infrastructure

pub mod application;
pub mod domain;
pub mod infrastructure;

// ── 重导出：保持外部 API 兼容 ──

// DTO
pub mod dto {
    pub use crate::application::dto::{LoginRequest, LoginResponse, UserProfile};
}

// Facade
pub mod facade {
    pub use crate::application::facade::{get_profile, login};
}

// 顶层常用导出
pub use application::{
    dto::{LoginRequest, LoginResponse, UserProfile},
    facade::{get_profile, login},
};
pub use domain::entity::{User, UserId};
// db_init
pub use infrastructure::db_init::init_database_schema;
pub use infrastructure::persistence::{find_user_by_id, find_user_by_username};
