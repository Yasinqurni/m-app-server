use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use chrono::Utc;
use crate::{
    module::cashflow::v1::entity::{
        model::cashflow::{ActiveModel, Entity as Cashflow, Model as CashflowModel, Column},
        request::list_cashflow_request::GetListCashflowQuery,
    },
    pkg::{
        custom_error::AppError,
        pagination::PaginatedResult,
    }
};
use tracing::log::error;


#[async_trait]
pub trait CashflowRepository: Send + Sync {
    async fn create(&self, cashflow: ActiveModel) -> Result<(), AppError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<CashflowModel>, AppError>;
    async fn find_with_pagination(&self, query: GetListCashflowQuery) -> Result<PaginatedResult<CashflowModel>, AppError>;
    async fn update(&self, id: i32, cashflow: ActiveModel) -> Result<CashflowModel, AppError>;
    async fn delete(&self, id: i32) -> Result<(), AppError>;
}

pub struct CashflowRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl CashflowRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CashflowRepository for CashflowRepositoryImpl {
    async fn create(&self, cashflow: ActiveModel) -> Result<(), AppError> {
        Cashflow::insert(cashflow)
            .exec(self.db.as_ref())
            .await
            .map_err(|err| {
                error!("DB error: {:?}", err);
                AppError::InternalError
            })?;

        Ok(())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<CashflowModel>, AppError> {
        Cashflow::find_by_id(id)
            .filter(Column::DeletedAt.is_null())
            .one(self.db.as_ref())
            .await
            .map_err(|err| {
                error!("DB error: {:?}", err);
                AppError::InternalError
            })
    }

    async fn find_with_pagination(
        &self,
        query: GetListCashflowQuery
    ) -> Result<PaginatedResult<CashflowModel>, AppError> {
        // Parse pagination parameters
        let page = query.page
            .and_then(|p| p.parse::<u64>().ok())
            .unwrap_or(1)
            .max(1);
    
        let limit = query.limit
            .and_then(|l| l.parse::<u64>().ok())
            .unwrap_or(10)
            .min(100);
    
        let offset = (page - 1) * limit;
    
        // Build search condition
        let mut search_condition = Condition::all().add(Column::DeletedAt.is_null());
        
        if let Some(search) = &query.search {
            if !search.trim().is_empty() {
                search_condition = search_condition.add(Column::Note.contains(search.trim()));
            }
        }

        if let Some(cashflow_type) = &query.r#type {
            if !cashflow_type.trim().is_empty() {
                search_condition = search_condition.add(Column::Type.eq(cashflow_type.trim()));
            }
        }

        if let Some(recap_type) = &query.recap_type {
            if !recap_type.trim().is_empty() {
                search_condition = search_condition.add(Column::RecapType.eq(recap_type.trim()));
            }
        }
    
        let search_condition_for_count = search_condition.clone();
    
        // Build ordering
        let mut query_builder = Cashflow::find().filter(search_condition);
    
        if let Some(order_by) = &query.order_by {
            let direction = match query.direction.as_deref() {
                Some("desc") | Some("DESC") => Order::Desc,
                _ => Order::Asc,
            };
    
            query_builder = match order_by.to_lowercase().as_str() {
                "note" => query_builder.order_by(Column::Note, direction),
                "nominal" => query_builder.order_by(Column::Nominal, direction),
                "type" => query_builder.order_by(Column::Type, direction),
                "recap_type" => query_builder.order_by(Column::RecapType, direction),
                "created_at" => query_builder.order_by(Column::CreatedAt, direction),
                "updated_at" => query_builder.order_by(Column::UpdatedAt, direction),
                _ => query_builder.order_by(Column::Id, Order::Asc),
            };
        } else {
            query_builder = query_builder.order_by(Column::Id, Order::Asc);
        }
    
        // Get total count
        let total = Cashflow::find()
            .filter(search_condition_for_count)
            .count(self.db.as_ref())
            .await
            .map_err(|err| {
                error!("DB error: {:?}", err);
                AppError::InternalError
            })?;
    
        // Get paginated data
        let data = query_builder
            .offset(offset)
            .limit(limit)
            .all(self.db.as_ref())
            .await
            .map_err(|err| {
                error!("DB error: {:?}", err);
                AppError::InternalError
            })?;
    
        let total_pages = (total as u64 as f64 / limit as f64).ceil() as u64;
    
        Ok(PaginatedResult {
            data,
            total,
            page,
            limit,
            total_pages,
        })
    }

    async fn update(&self, id: i32, mut cashflow: ActiveModel) -> Result<CashflowModel, AppError> {
        // Set updated_at timestamp
        cashflow.updated_at = Set(Some(Utc::now()));

        let result = Cashflow::update_many()
            .set(cashflow)
            .filter(Column::Id.eq(id))
            .filter(Column::DeletedAt.is_null())
            .exec(self.db.as_ref())
            .await
            .map_err(|err| {
                error!("DB error: {:?}", err);
                AppError::InternalError
            })?;

        if result.rows_affected == 0 {
            return Err(AppError::NotFound(format!("Cashflow with id {} not found", id)));
        }

        // Fetch the updated cashflow
        Cashflow::find_by_id(id)
            .filter(Column::DeletedAt.is_null())
            .one(self.db.as_ref())
            .await
            .map_err(|err| {
                error!("DB error: {:?}", err);
                AppError::InternalError
            })?
            .ok_or_else(|| AppError::NotFound(format!("Cashflow with id {} not found", id)))
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        let now = Utc::now();
        let result = Cashflow::update_many()
            .col_expr(Column::DeletedAt, now.into())
            .col_expr(Column::UpdatedAt, now.into())
            .filter(Column::Id.eq(id))
            .filter(Column::DeletedAt.is_null())
            .exec(self.db.as_ref())
            .await
            .map_err(|err| {
                error!("DB error: {:?}", err);
                AppError::InternalError
            })?;

        if result.rows_affected == 0 {
            return Err(AppError::NotFound(format!("Cashflow with id {} not found", id)));
        }

        Ok(())
    }
}