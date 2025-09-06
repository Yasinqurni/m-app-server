use axum::Router;

use crate::module::product::v1::product_handler;

pub fn  configure() -> Router {
	Router::new().nest("/api/v1/product", product_handler::routes())
}
