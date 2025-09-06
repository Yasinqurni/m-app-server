use validator::{Validate};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetTransactionRequest {
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,
}
