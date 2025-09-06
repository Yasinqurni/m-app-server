use axum::Router;

use crate::module::transaction::v1::transaction_handler;

pub fn configure() -> Router {
	Router::new().nest("/api/v1/transaction", transaction_handler::routes())
}
