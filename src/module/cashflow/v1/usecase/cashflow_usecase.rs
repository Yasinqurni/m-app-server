use async_trait::async_trait;
use crate::module::cashflow::v1::entity::{
    request::{
        create_cashflow_request::CreateCashflowRequest,
        update_cashflow_request::UpdateCashflowRequest,
        get_cashflow_request::GetCashflowRequest,
        list_cashflow_request::GetListCashflowQuery,
    },
    response::{
        cashflow_response::CashflowResponse,
        list_cashflow_response::ListCashflowResponse,
    },
    model::cashflow::ActiveModel,
};
use crate::module::cashflow::v1::repository::cashflow_repository::CashflowRepository;
use crate::pkg::custom_error::AppError;
use sea_orm::{prelude::*, Set};
use std::sync::Arc;
use chrono::Utc;

#[async_trait]
pub trait CashflowUsecase: Send + Sync {
    async fn create_cashflow(&self, request: CreateCashflowRequest) -> Result<CashflowResponse, AppError>;
    async fn get_cashflow(&self, request: GetCashflowRequest) -> Result<CashflowResponse, AppError>;
    async fn list_cashflows(&self, query: GetListCashflowQuery) -> Result<ListCashflowResponse, AppError>;
    async fn update_cashflow(&self, id: i32, request: UpdateCashflowRequest) -> Result<CashflowResponse, AppError>;
    async fn delete_cashflow(&self, id: i32) -> Result<(), AppError>;
}

pub struct CashflowUsecaseImpl {
    cashflow_repository: Arc<dyn CashflowRepository>,
}

impl CashflowUsecaseImpl {
    pub fn new(cashflow_repository: Arc<dyn CashflowRepository>) -> Self {
        Self { cashflow_repository }
    }
}

#[async_trait]
impl CashflowUsecase for CashflowUsecaseImpl {
    async fn create_cashflow(&self, request: CreateCashflowRequest) -> Result<CashflowResponse, AppError> {
        // Create active model
        let cashflow_active_model = ActiveModel {
            note: Set(request.note),
            nominal: Set(request.nominal),
            r#type: Set(request.r#type),
            recap_type: Set(request.recap_type),
            created_at: Set(Some(Utc::now())),
            updated_at: Set(Some(Utc::now())),
            deleted_at: Set(None),
            ..Default::default()
        };

        // Save to repository
        self.cashflow_repository.create(cashflow_active_model).await?;

        // Find the created cashflow to return (we'll get the latest one since we don't have a unique identifier)
        // For now, we'll return a simple success response
        // In a real scenario, you might want to add a unique constraint or return the ID differently
        let created_cashflow = self.cashflow_repository
            .find_with_pagination(GetListCashflowQuery {
                limit: Some("1".to_string()),
                page: Some("1".to_string()),
                search: None,
                order_by: Some("id".to_string()),
                direction: Some("desc".to_string()),
                r#type: None,
                recap_type: None,
            })
            .await?;

        let cashflow = created_cashflow.data.into_iter().next()
            .ok_or_else(|| AppError::InternalError)?;

        Ok(CashflowResponse::from(cashflow))
    }

    async fn get_cashflow(&self, request: GetCashflowRequest) -> Result<CashflowResponse, AppError> {
        let cashflow = self.cashflow_repository
            .find_by_id(request.id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Cashflow with id {} not found", request.id)))?;

        Ok(CashflowResponse::from(cashflow))
    }

    async fn list_cashflows(&self, query: GetListCashflowQuery) -> Result<ListCashflowResponse, AppError> {
        let paginated_result = self.cashflow_repository.find_with_pagination(query).await?;
        Ok(ListCashflowResponse::from(paginated_result))
    }

    async fn update_cashflow(&self, id: i32, request: UpdateCashflowRequest) -> Result<CashflowResponse, AppError> {
        // Check if cashflow exists
        self.cashflow_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Cashflow with id {} not found", id)))?;

        // Create active model with only changed fields
        let mut cashflow_active_model = ActiveModel {
            id: Set(id),
            ..Default::default()
        };

        if let Some(note) = request.note {
            cashflow_active_model.note = Set(note);
        }
        if let Some(nominal) = request.nominal {
            cashflow_active_model.nominal = Set(nominal);
        }
        if let Some(cashflow_type) = request.r#type {
            cashflow_active_model.r#type = Set(cashflow_type);
        }
        if let Some(recap_type) = request.recap_type {
            cashflow_active_model.recap_type = Set(recap_type);
        }

        // Update in repository
        let updated_cashflow = self.cashflow_repository.update(id, cashflow_active_model).await?;

        Ok(CashflowResponse::from(updated_cashflow))
    }

    async fn delete_cashflow(&self, id: i32) -> Result<(), AppError> {
        // Check if cashflow exists
        self.cashflow_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Cashflow with id {} not found", id)))?;

        // Soft delete
        self.cashflow_repository.delete(id).await?;

        Ok(())
    }
}
