use serde::{Serialize, Deserialize};
use crate::pkg::pagination::PaginatedResult;
use crate::module::product::v1::entity::response::product_response::ProductResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListProductResponse {
    pub data: Vec<ProductResponse>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
    pub total_pages: u64,
}

impl From<PaginatedResult<crate::module::product::v1::entity::model::product::Model>> for ListProductResponse {
    fn from(paginated_result: PaginatedResult<crate::module::product::v1::entity::model::product::Model>) -> Self {
        Self {
            data: paginated_result.data.into_iter().map(ProductResponse::from).collect(),
            total: paginated_result.total,
            page: paginated_result.page,
            limit: paginated_result.limit,
            total_pages: paginated_result.total_pages,
        }
    }
}
