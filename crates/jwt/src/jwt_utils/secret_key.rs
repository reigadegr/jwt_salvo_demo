use crate::jwt_utils::models::Claims;
use anyhow::{Context, anyhow};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error,
};
use once_cell::sync::OnceCell;

static JWT_UTILS: OnceCell<SecretKey> = OnceCell::new();

pub fn init_jwt_utils(private_key: &[u8], public_key: &[u8]) -> anyhow::Result<()> {
    let encode_key = EncodingKey::from_ed_pem(private_key)
        .context("Failed to create encoding key from private key")?;
    let decode_key = DecodingKey::from_ed_pem(public_key)
        .context("Failed to create decoding key from public key")?;
    let jwt_header = Header::new(Algorithm::EdDSA);
    let mut jwt_vation = Validation::new(Algorithm::EdDSA);
    jwt_vation.leeway = 0;
    let jwt_utils = SecretKey::new(encode_key, decode_key, jwt_header, jwt_vation);
    JWT_UTILS
        .set(jwt_utils)
        .map_err(|_| anyhow!("Failed to set jwt_utils."))?;
    Ok(())
}

pub fn get_jwt_utils() -> anyhow::Result<&'static SecretKey> {
    JWT_UTILS
        .get()
        .ok_or_else(|| anyhow!("JWT utils not initialized."))
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

    pub fn generate_token(&self, role: &str, user_id: &str) -> Result<String, Error> {
        self.generate_token_with_expiry(role, user_id, Duration::seconds(20))
    }

    pub fn generate_token_with_expiry(
        &self,
        role: &str,
        user_id: &str,
        expiry: Duration,
    ) -> Result<String, Error> {
        let exp_time = Utc::now() + expiry;
        let claims = Claims::new(role, user_id, exp_time);
        encode(&self.jwt_header, &claims, &self.encode_key)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, Error> {
        decode::<Claims>(token, &self.decode_key, &self.jwt_vation).map(|data| data.claims)
    }
}
