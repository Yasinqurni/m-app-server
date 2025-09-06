use sea_orm::DatabaseConnection;

use crate::module::product::v1::repository::product_repository::{ProductRepositoryImpl};
use crate::module::product::v1::usecase::product_usecase::{ProductUsecaseImpl, ProductUsecase};
use crate::module::cashflow::v1::repository::cashflow_repository::{CashflowRepositoryImpl};
use crate::module::cashflow::v1::usecase::cashflow_usecase::{CashflowUsecaseImpl, CashflowUsecase};

use std::sync::Arc;

#[derive(Clone)]
pub struct AppModule {
	pub product_usecase: Arc<dyn ProductUsecase>,
	pub cashflow_usecase: Arc<dyn CashflowUsecase>,
	pub db: Arc<DatabaseConnection>
}

// Dependency injection
pub fn initialize_di(db: Arc<DatabaseConnection>) -> AppModule {
	let product_repository = Arc::new(ProductRepositoryImpl::new(db.clone()));
	let cashflow_repository = Arc::new(CashflowRepositoryImpl::new(db.clone()));

	let product_usecase = Arc::new(ProductUsecaseImpl::new(product_repository.clone()));
	let cashflow_usecase = Arc::new(CashflowUsecaseImpl::new(cashflow_repository.clone()));

	AppModule {
        product_usecase,
        cashflow_usecase,
		db,
	}
}
