use super::models::Claims;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error,
};
use once_cell::sync::Lazy;

const PRIVATE_KEY: &[u8] = include_bytes!("../../keys/private_key.pem");
const PUBLIC_KEY: &[u8] = include_bytes!("../../keys/public_key.pem");

static ENCODE_KEY: Lazy<EncodingKey> = Lazy::new(|| EncodingKey::from_ed_pem(PRIVATE_KEY).unwrap());
static DECODE_KEY: Lazy<DecodingKey> = Lazy::new(|| DecodingKey::from_ed_pem(PUBLIC_KEY).unwrap());

static JWT_HEADER: Lazy<Header> = Lazy::new(|| Header::new(Algorithm::EdDSA));

static JWT_VATION: Lazy<Validation> = Lazy::new(|| {
    let mut v = Validation::new(Algorithm::EdDSA);
    v.leeway = 0;
    v
});

pub fn generate_token(role: &str, user_id: &str) -> (Result<String, Error>, i64) {
    let exp_time = Utc::now() + Duration::seconds(20);
    let claims = Claims::new(role, user_id, exp_time);
    (
        encode(&JWT_HEADER, &claims, &ENCODE_KEY),
        exp_time.timestamp(),
    )
}

pub fn validate_token(token: &str) -> Result<Claims, Error> {
    decode::<Claims>(token, &DECODE_KEY, &JWT_VATION).map(|data| data.claims)
}
