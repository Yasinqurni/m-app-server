use validator::{Validate};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTransactionRequest {
    #[validate(range(min = 1, message = "Product ID must be positive"))]
    pub product_id: i32,

    #[validate(range(min = 1, message = "Quantity must be positive"))]
    pub qty: i32,
}
