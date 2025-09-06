use async_trait::async_trait;
use crate::module::transaction::v1::entity::{
    request::{
        create_transaction_request::CreateTransactionRequest,
        update_transaction_request::UpdateTransactionRequest,
        get_transaction_request::GetTransactionRequest,
        list_transaction_request::GetListTransactionQuery,
    },
    response::{
        transaction_response::TransactionResponse,
        list_transaction_response::ListTransactionResponse,
    },
    model::transaction::ActiveModel,
};
use crate::module::repository::transaction_repository::TransactionRepository;
use crate::module::repository::product_repository::ProductRepository;
use crate::pkg::custom_error::AppError;
use sea_orm::{prelude::*, Set};
use std::sync::Arc;
use chrono::Utc;

#[async_trait]
pub trait TransactionUsecase: Send + Sync {
    async fn create_transaction(&self, request: CreateTransactionRequest) -> Result<(), AppError>;
    async fn get_transaction(&self, request: GetTransactionRequest) -> Result<TransactionResponse, AppError>;
    async fn list_transactions(&self, query: GetListTransactionQuery) -> Result<ListTransactionResponse, AppError>;
    async fn update_transaction(&self, id: i32, request: UpdateTransactionRequest) -> Result<TransactionResponse, AppError>;
    async fn delete_transaction(&self, id: i32) -> Result<(), AppError>;
}

pub struct TransactionUsecaseImpl {
    transaction_repository: Arc<dyn TransactionRepository>,
    product_repository: Arc<dyn ProductRepository>,
}

impl TransactionUsecaseImpl {
    pub fn new(
        transaction_repository: Arc<dyn TransactionRepository>,
        product_repository: Arc<dyn ProductRepository>,
    ) -> Self {
        Self { transaction_repository, product_repository }
    }
}

#[async_trait]
impl TransactionUsecase for TransactionUsecaseImpl {
    async fn create_transaction(&self, request: CreateTransactionRequest) -> Result<(), AppError> {
        let product = self.product_repository
            .find_by_id(request.product_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Product with id {} not found", request.product_id)))?;
        // Create active model
        let transaction_active_model = ActiveModel {
            product_id: Set(request.product_id),
            hpp_amount: Set(product.hpp_amount),
            selling_amount: Set(product.selling_amount),
            qty: Set(request.qty),
            created_at: Set(Some(Utc::now())),
            updated_at: Set(Some(Utc::now())),
            deleted_at: Set(None),
            ..Default::default()
        };

        // Save to repository
        self.transaction_repository.create(transaction_active_model).await?;

        Ok(())
    }

    async fn get_transaction(&self, request: GetTransactionRequest) -> Result<TransactionResponse, AppError> {
        let transaction = self.transaction_repository
            .find_by_id(request.id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Transaction with id {} not found", request.id)))?;

        Ok(TransactionResponse::from(transaction))
    }

    async fn list_transactions(&self, query: GetListTransactionQuery) -> Result<ListTransactionResponse, AppError> {
        let paginated_result = self.transaction_repository.find_with_pagination(query).await?;
        Ok(ListTransactionResponse::from(paginated_result))
    }

    async fn update_transaction(&self, id: i32, request: UpdateTransactionRequest) -> Result<TransactionResponse, AppError> {
        // Check if transaction exists
        self.transaction_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Transaction with id {} not found", id)))?;

        // Create active model with only changed fields
        let mut transaction_active_model = ActiveModel {
            id: Set(id),
            ..Default::default()
        };

        if let Some(product_id) = request.product_id {
            transaction_active_model.product_id = Set(product_id);
        }
        if let Some(hpp_amount) = request.hpp_amount {
            transaction_active_model.hpp_amount = Set(hpp_amount);
        }
        if let Some(selling_amount) = request.selling_amount {
            transaction_active_model.selling_amount = Set(selling_amount);
        }
        if let Some(qty) = request.qty {
            transaction_active_model.qty = Set(qty);
        }

        // Update in repository
        let updated_transaction = self.transaction_repository.update(id, transaction_active_model).await?;

        Ok(TransactionResponse::from(updated_transaction))
    }

    async fn delete_transaction(&self, id: i32) -> Result<(), AppError> {
        // Check if transaction exists
        self.transaction_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Transaction with id {} not found", id)))?;

        // Soft delete
        self.transaction_repository.delete(id).await?;

        Ok(())
    }
}
