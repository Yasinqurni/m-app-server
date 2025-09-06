use sea_orm::DatabaseConnection;

use crate::module::repository::product_repository::{ProductRepositoryImpl};
use crate::module::product::v1::product_usecase::{ProductUsecaseImpl, ProductUsecase};
use crate::module::repository::cashflow_repository::{CashflowRepositoryImpl};
use crate::module::cashflow::v1::cashflow_usecase::{CashflowUsecaseImpl, CashflowUsecase};
use crate::module::repository::transaction_repository::{TransactionRepositoryImpl};
use crate::module::transaction::v1::transaction_usecase::{TransactionUsecaseImpl, TransactionUsecase};

use std::sync::Arc;

#[derive(Clone)]
pub struct AppModule {
	pub product_usecase: Arc<dyn ProductUsecase>,
	pub cashflow_usecase: Arc<dyn CashflowUsecase>,
	pub transaction_usecase: Arc<dyn TransactionUsecase>,
	pub db: Arc<DatabaseConnection>
}

// Dependency injection
pub fn initialize_di(db: Arc<DatabaseConnection>) -> AppModule {
	let product_repository = Arc::new(ProductRepositoryImpl::new(db.clone()));
	let cashflow_repository = Arc::new(CashflowRepositoryImpl::new(db.clone()));
	let transaction_repository = Arc::new(TransactionRepositoryImpl::new(db.clone()));

	let product_usecase = Arc::new(ProductUsecaseImpl::new(product_repository.clone()));
	let cashflow_usecase = Arc::new(CashflowUsecaseImpl::new(cashflow_repository.clone()));
	let transaction_usecase = Arc::new(TransactionUsecaseImpl::new(transaction_repository.clone(), product_repository.clone()));

	AppModule {
        product_usecase,
        cashflow_usecase,
        transaction_usecase,
		db,
	}
}
