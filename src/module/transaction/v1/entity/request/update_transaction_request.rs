use validator::{Validate};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateTransactionRequest {
    #[validate(range(min = 1, message = "Product ID must be positive"))]
    pub product_id: Option<i32>,

    #[validate(range(min = 0, message = "HPP amount must be non-negative"))]
    pub hpp_amount: Option<i32>,

    #[validate(range(min = 0, message = "Selling amount must be non-negative"))]
    pub selling_amount: Option<i32>,

    #[validate(range(min = 1, message = "Quantity must be positive"))]
    pub qty: Option<i32>,
}
