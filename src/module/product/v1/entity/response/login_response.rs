use validator::Validate;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginResponse {
    pub token: String,
}
