use validator::{Validate};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCashflowRequest {
    #[validate(length(min = 1, max = 500, message = "Note must be between 1 and 500 characters"))]
    pub note: String,

    #[validate(range(min = 1, message = "Nominal must be positive"))]
    pub nominal: i32,

    #[validate(length(min = 1, max = 50, message = "Type must be between 1 and 50 characters"))]
    pub r#type: String,

    #[validate(length(min = 1, max = 50, message = "Recap type must be between 1 and 50 characters"))]
    pub recap_type: String,
}
