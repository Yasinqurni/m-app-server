use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CashflowResponse {
    pub id: i32,
    pub note: String,
    pub nominal: i32,
    pub r#type: String,
    pub recap_type: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<crate::module::cashflow::v1::entity::model::cashflow::Model> for CashflowResponse {
    fn from(model: crate::module::cashflow::v1::entity::model::cashflow::Model) -> Self {
        Self {
            id: model.id,
            note: model.note,
            nominal: model.nominal,
            r#type: model.r#type,
            recap_type: model.recap_type,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
