pub mod dto;
pub mod endpoint;
pub mod entity;
pub mod facade;
pub mod repository;
pub mod service;
pub mod value_object;

// 重导出常用类型
pub use dto::{LoginRequest, LoginResponse, UserProfile};
pub use endpoint::{hello, login, profile};
pub use entity::{User, UserId};
pub use facade::AuthFacade;
pub use repository::{DEFAULT_USERS, InMemoryUserRepository, UserRepository};
