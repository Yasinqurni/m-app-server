use validator::{Validate};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateProductRequest {
    #[validate(length(min = 1, max = 255, message = "Name must be between 1 and 255 characters"))]
    pub name: Option<String>,

    #[validate(range(min = 0, message = "HPP amount must be non-negative"))]
    pub hpp_amount: Option<i32>,

    #[validate(range(min = 0, message = "Selling amount must be non-negative"))]
    pub selling_amount: Option<i32>,
}
