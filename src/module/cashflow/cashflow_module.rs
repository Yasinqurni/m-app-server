use axum::Router;

use crate::module::cashflow::v1::cashflow_handler;

pub fn configure() -> Router {
	Router::new().nest("/api/v1/cashflow", cashflow_handler::routes())
}
