pub mod dto;
pub mod entity;
pub mod facade;
pub mod repository;
pub mod service;
pub mod value_object;

// 重导出常用类型
pub use dto::{LoginRequest, LoginResponse, UserProfile};
pub use entity::{User, UserId};
pub use facade::AuthFacade;
pub use repository::{DEFAULT_USERS, InMemoryUserRepository, UserRepository};

pub mod db_init;
pub use db_init::init_database_schema;

// SeaORM 实体别名
pub mod sea_orm_entity {
    pub use my_entities::users::*;
}
