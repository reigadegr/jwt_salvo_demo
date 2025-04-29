use crate::models::Claims;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error,
};
use once_cell::sync::Lazy;

const PRIVATE_KEY: &[u8] = include_bytes!("../keys/private_key.pem");
const PUBLIC_KEY: &[u8] = include_bytes!("../keys/public_key.pem");

static ENCODE_KEY: Lazy<EncodingKey> = Lazy::new(|| EncodingKey::from_ed_pem(PRIVATE_KEY).unwrap());
static DECODE_KEY: Lazy<DecodingKey> = Lazy::new(|| DecodingKey::from_ed_pem(PUBLIC_KEY).unwrap());

pub fn generate_token(role: &str, user_id: &str) -> Result<String, Error> {
    let reset_timesmap = Utc::now() - Duration::seconds(61);
    let claims = Claims::new(role, user_id, reset_timesmap + Duration::seconds(10));
    encode(&Header::new(Algorithm::EdDSA), &claims, &ENCODE_KEY)
}

pub fn validate_token(token: &str) -> Result<Claims, Error> {
    decode::<Claims>(token, &DECODE_KEY, &Validation::new(Algorithm::EdDSA)).map(|data| data.claims)
}
