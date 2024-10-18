use crate::error::AppError;
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
}

impl Claims {
    fn new(sub: i32, exp: i64) -> Self {
        Self { sub, exp }
    }
}

pub fn generate_jwt(member_id: i32, secret: &str) -> Result<String, AppError> {
    let time = TimeDelta::new(3600, 0).unwrap();
    let expiration = Utc::now()
        .checked_add_signed(time)
        .expect("Timestamp Invalid")
        .timestamp();
    let claims = Claims::new(member_id, expiration);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(token)
}

pub fn decode_token(token: &str, secret: &str) -> Result<TokenData<Claims>, AppError> {
    let token = decode(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token)
}

pub fn is_valid(claim: &Claims) -> bool {
    claim.exp > Utc::now().timestamp()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn generate_jwt_and_decode_token_should_work() -> Result<()> {
        let secret = "secret";
        let token = generate_jwt(1, secret)?;
        let token_data = decode_token(&token, secret)?;
        assert_eq!(token_data.claims.sub, 1);
        assert!(is_valid(&token_data.claims));
        Ok(())
    }
}
