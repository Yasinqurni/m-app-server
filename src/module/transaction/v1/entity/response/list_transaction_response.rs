use serde::{Serialize, Deserialize};
use crate::pkg::pagination::PaginatedResult;
use crate::module::transaction::v1::entity::response::transaction_response::TransactionResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTransactionResponse {
    pub data: Vec<TransactionResponse>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
    pub total_pages: u64,
}

impl From<PaginatedResult<crate::module::transaction::v1::entity::model::transaction::Model>> for ListTransactionResponse {
    fn from(paginated_result: PaginatedResult<crate::module::transaction::v1::entity::model::transaction::Model>) -> Self {
        Self {
            data: paginated_result.data.into_iter().map(TransactionResponse::from).collect(),
            total: paginated_result.total,
            page: paginated_result.page,
            limit: paginated_result.limit,
            total_pages: paginated_result.total_pages,
        }
    }
}
