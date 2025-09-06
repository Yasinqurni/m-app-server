use validator::{Validate};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetListCashflowQuery {
    pub limit: Option<String>,

    pub page: Option<String>,

    pub search: Option<String>,

    pub order_by: Option<String>,

    pub direction: Option<String>,

    pub r#type: Option<String>,

    pub recap_type: Option<String>,
}
