use validator::{Validate};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetListTransactionQuery {
    pub limit: Option<String>,

    pub page: Option<String>,

    pub search: Option<String>,

    pub order_by: Option<String>,

    pub direction: Option<String>,

    pub product_id: Option<String>,
}
