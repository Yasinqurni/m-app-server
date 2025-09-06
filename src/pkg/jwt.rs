use jsonwebtoken::{encode, Header, EncodingKey, errors::Error};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use crate::pkg::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_token(user_id: &str, config: &Config) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(config.jwt.expiration))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(config.jwt.secret.as_bytes()))
}

// pub fn verify_token(token: &str, config: &Config) -> Result<TokenData<Claims>, Error> {
//     decode::<Claims>(token, &DecodingKey::from_secret(config.jwt.secret.as_bytes()), &Validation::default())
// }