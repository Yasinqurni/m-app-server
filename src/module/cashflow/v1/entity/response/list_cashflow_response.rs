use serde::{Serialize, Deserialize};
use crate::pkg::pagination::PaginatedResult;
use crate::module::cashflow::v1::entity::response::cashflow_response::CashflowResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListCashflowResponse {
    pub data: Vec<CashflowResponse>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
    pub total_pages: u64,
}

impl From<PaginatedResult<crate::module::cashflow::v1::entity::model::cashflow::Model>> for ListCashflowResponse {
    fn from(paginated_result: PaginatedResult<crate::module::cashflow::v1::entity::model::cashflow::Model>) -> Self {
        Self {
            data: paginated_result.data.into_iter().map(CashflowResponse::from).collect(),
            total: paginated_result.total,
            page: paginated_result.page,
            limit: paginated_result.limit,
            total_pages: paginated_result.total_pages,
        }
    }
}
