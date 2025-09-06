use sea_orm::DatabaseConnection;

use crate::module::product::v1::repository::product_repository::{ProductRepositoryImpl};
use crate::module::product::v1::usecase::product_usecase::{ProductUsecaseImpl, ProductUsecase};

use std::sync::Arc;

#[derive(Clone)]
pub struct AppModule {
	pub product_usecase: Arc<dyn ProductUsecase>,
	pub db: Arc<DatabaseConnection>,
}

// Dependency injection
pub fn initialize_di(db: Arc<DatabaseConnection>) -> AppModule {
	let product_repository = Arc::new(ProductRepositoryImpl::new(db.clone()));

	let product_usecase = Arc::new(ProductUsecaseImpl::new(product_repository.clone()));

	AppModule {
        product_usecase,
        db,
	}
}
