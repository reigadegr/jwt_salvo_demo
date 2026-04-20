pub mod models;
pub mod secret_key;

use std::any::Any;

use models::{Claims, UserInfo};
use salvo::Depot;

pub fn save_user_info(depot: &mut Depot, claims: &Claims) {
    depot.insert("user_info", claims.to_user_info());
}

pub fn get_user_info(depot: &Depot) -> Result<&UserInfo, Option<&Box<dyn Any + Send + Sync>>> {
    depot.get::<UserInfo>("user_info")
}
