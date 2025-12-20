use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use anyhow::{Result, anyhow};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub exp: i64,
}

pub fn create_jwt(user_id: &str, username: &str) -> Result<String> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
    
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .ok_or_else(|| anyhow!("Failed to calculate expiration"))?
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<Claims> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub fn hash_password(password: &str) -> Result<String> {
    let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let valid = bcrypt::verify(password, hash)?;
    Ok(valid)
}
