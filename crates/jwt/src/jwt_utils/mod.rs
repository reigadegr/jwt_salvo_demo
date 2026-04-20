pub mod models;
pub mod secret_key;

use std::any::Any;

use models::Claims;
use salvo::Depot;

pub fn save_claims(depot: &mut Depot, claims: Claims) {
    depot.insert("claims", claims);
}

pub fn get_claims(depot: &Depot) -> Result<&Claims, Option<&Box<dyn Any + Send + Sync>>> {
    depot.get::<Claims>("claims")
}
