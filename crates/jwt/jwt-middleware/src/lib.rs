use std::any::Any;

use my_jwt_core::jwt_utils::models::UserInfo;
use salvo::Depot;

pub fn save_user_info(depot: &mut Depot, claims: &my_jwt_core::jwt_utils::models::Claims) {
    depot.insert("user_info", claims.to_user_info());
}

pub fn get_user_info(depot: &Depot) -> Result<&UserInfo, Option<&Box<dyn Any + Send + Sync>>> {
    depot.get::<UserInfo>("user_info")
}
