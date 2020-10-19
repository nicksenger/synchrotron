use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginAttempt {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: i32,
}

struct SecretKey {
    key: &'static str,
}

impl SecretKey {
    fn get_secret_key() -> SecretKey {
        SecretKey { key: "secret" }
    }
}

pub fn encode_jwt(user_id: i32, exp_day: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i32
        + exp_day * 24 * 60 * 60;

    let jwt_secret_key = SecretKey::get_secret_key();
    let my_claims = Claims { user_id, exp };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(jwt_secret_key.key.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_jwt(
    token: String,
) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
    let jwt_secret_key = SecretKey::get_secret_key();
    let validation = Validation::default();

    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret_key.key.as_ref()),
        &validation,
    )
}
