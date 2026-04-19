//! DDD 三层架构: domain / application / infrastructure

pub mod application;
pub mod domain;
pub mod infrastructure;

// ── 重导出：保持外部 API 兼容 ──

// DTO（app/src/demo/mod.rs 使用 `my_demo::dto::LoginRequest`）
pub mod dto {
    pub use crate::application::dto::{LoginRequest, LoginResponse, UserProfile};
}

// Facade（app/src/demo/mod.rs 使用 `my_demo::facade::AuthFacade`）
pub mod facade {
    pub use crate::application::facade::AuthFacade;
}

// Repository（app/src/demo/mod.rs 使用 `my_demo::repository::DatabaseUserRepository`）
pub mod repository {
    pub use crate::domain::repository::UserRepository;
    pub use crate::infrastructure::persistence::{DatabaseUserRepository, InMemoryUserRepository};
}

// 顶层常用导出
pub use application::dto::{LoginRequest, LoginResponse, UserProfile};
pub use application::facade::AuthFacade;
pub use domain::entity::{User, UserId};
pub use domain::repository::UserRepository;
pub use infrastructure::persistence::{DatabaseUserRepository, InMemoryUserRepository};

// db_init
pub use infrastructure::db_init::init_database_schema;

// SeaORM 实体别名
pub mod sea_orm_entity {
    pub use my_entities::users::*;
}
