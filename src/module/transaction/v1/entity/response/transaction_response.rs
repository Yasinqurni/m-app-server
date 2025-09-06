use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionResponse {
    pub id: i32,
    pub product_id: i32,
    pub hpp_amount: i32,
    pub selling_amount: i32,
    pub qty: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<crate::module::transaction::v1::entity::model::transaction::Model> for TransactionResponse {
    fn from(model: crate::module::transaction::v1::entity::model::transaction::Model) -> Self {
        Self {
            id: model.id,
            product_id: model.product_id,
            hpp_amount: model.hpp_amount,
            selling_amount: model.selling_amount,
            qty: model.qty,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
