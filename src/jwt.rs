use crate::models::Claims;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error,
};

const PRIVATE_KEY: &[u8] = include_bytes!("../keys/private_key.pem");
const PUBLIC_KEY: &[u8] = include_bytes!("../keys/public_key.pem");

pub fn generate_token(user_id: &str) -> Result<String, Error> {
    let reset_timesmap = Utc::now() - Duration::seconds(61);
    let claims = Claims::new(user_id.to_string(), reset_timesmap + Duration::seconds(10));
    encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(PRIVATE_KEY)?,
    )
}

pub fn validate_token(token: &str) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_rsa_pem(PUBLIC_KEY)?,
        &Validation::new(Algorithm::RS256),
    )
    .map(|data| data.claims)
}
