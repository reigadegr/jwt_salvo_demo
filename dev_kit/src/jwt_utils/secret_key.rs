use crate::models::Claims;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error,
};
use once_cell::sync::Lazy;

const PRIVATE_KEY: &[u8] = include_bytes!("../../keys/private_key.pem");
const PUBLIC_KEY: &[u8] = include_bytes!("../../keys/public_key.pem");

static JWT_UTILS: Lazy<SecretKey> = Lazy::new(|| {
    let encode_key = EncodingKey::from_ed_pem(PRIVATE_KEY).unwrap();
    let decode_key = DecodingKey::from_ed_pem(PUBLIC_KEY).unwrap();
    let jwt_header = Header::new(Algorithm::EdDSA);
    let mut jwt_vation = Validation::new(Algorithm::EdDSA);
    jwt_vation.leeway = 0;

    SecretKey::new(encode_key, decode_key, jwt_header, jwt_vation)
});

#[must_use]
pub fn get_jwt_utils() -> &'static SecretKey {
    &JWT_UTILS
}

pub struct SecretKey {
    encode_key: EncodingKey,
    decode_key: DecodingKey,
    jwt_header: Header,
    jwt_vation: Validation,
}

impl SecretKey {
    const fn new(
        encode_key: EncodingKey,
        decode_key: DecodingKey,
        jwt_header: Header,
        jwt_vation: Validation,
    ) -> Self {
        Self {
            encode_key,
            decode_key,
            jwt_header,
            jwt_vation,
        }
    }

    pub fn generate_token(&self, role: &str, user_id: &str) -> (Result<String, Error>, i64) {
        let exp_time = Utc::now() + Duration::seconds(20);
        let claims = Claims::new(role, user_id, exp_time);
        (
            encode(&self.jwt_header, &claims, &self.encode_key),
            exp_time.timestamp(),
        )
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, Error> {
        decode::<Claims>(token, &self.decode_key, &self.jwt_vation).map(|data| data.claims)
    }
}
