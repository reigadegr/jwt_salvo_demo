use crate::models::Claims;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error};

const SECRET_KEY: &[u8] = b"your_secret_key";

pub fn generate_token(user_id: &str) -> Result<String, Error> {
    let reset_timesmap = Utc::now() - Duration::seconds(61);
    let claims = Claims::new(user_id.to_string(), reset_timesmap + Duration::seconds(10));
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )
}

pub fn validate_token(token: &str) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
