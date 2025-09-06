use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductResponse {
    pub id: i32,
    pub name: String,
    pub hpp_amount: i32,
    pub selling_amount: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<crate::module::product::v1::entity::model::product::Model> for ProductResponse {
    fn from(model: crate::module::product::v1::entity::model::product::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            hpp_amount: model.hpp_amount,
            selling_amount: model.selling_amount,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
