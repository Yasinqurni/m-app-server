use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use chrono::Utc;
use crate::{
    module::transaction::v1::entity::{
        model::transaction::{ActiveModel, Entity as Transaction, Model as TransactionModel, Column},
        request::list_transaction_request::GetListTransactionQuery,
    },
    pkg::{
        custom_error::AppError,
        pagination::PaginatedResult,
    }
};
use tracing::log::error;


#[async_trait]
pub trait TransactionRepository: Send + Sync {
    async fn create(&self, transaction: ActiveModel) -> Result<(), AppError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<TransactionModel>, AppError>;
    async fn find_with_pagination(&self, query: GetListTransactionQuery) -> Result<PaginatedResult<TransactionModel>, AppError>;
    async fn update(&self, id: i32, transaction: ActiveModel) -> Result<TransactionModel, AppError>;
    async fn delete(&self, id: i32) -> Result<(), AppError>;
}

pub struct TransactionRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl TransactionRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TransactionRepository for TransactionRepositoryImpl {
    async fn create(&self, transaction: ActiveModel) -> Result<(), AppError> {
        Transaction::insert(transaction)
            .exec(self.db.as_ref())
            .await
            .map_err(|err| {
                error!("DB error: {:?}", err);
                AppError::InternalError
            })?;

        Ok(())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<TransactionModel>, AppError> {
        Transaction::find_by_id(id)
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
        query: GetListTransactionQuery
    ) -> Result<PaginatedResult<TransactionModel>, AppError> {
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
        
        if let Some(product_id_str) = &query.product_id {
            if let Ok(product_id) = product_id_str.parse::<i32>() {
                search_condition = search_condition.add(Column::ProductId.eq(product_id));
            }
        }
    
        let search_condition_for_count = search_condition.clone();
    
        // Build ordering
        let mut query_builder = Transaction::find().filter(search_condition);
    
        if let Some(order_by) = &query.order_by {
            let direction = match query.direction.as_deref() {
                Some("desc") | Some("DESC") => Order::Desc,
                _ => Order::Asc,
            };
    
            query_builder = match order_by.to_lowercase().as_str() {
                "product_id" => query_builder.order_by(Column::ProductId, direction),
                "hpp_amount" => query_builder.order_by(Column::HppAmount, direction),
                "selling_amount" => query_builder.order_by(Column::SellingAmount, direction),
                "qty" => query_builder.order_by(Column::Qty, direction),
                "created_at" => query_builder.order_by(Column::CreatedAt, direction),
                "updated_at" => query_builder.order_by(Column::UpdatedAt, direction),
                _ => query_builder.order_by(Column::Id, Order::Asc),
            };
        } else {
            query_builder = query_builder.order_by(Column::Id, Order::Asc);
        }
    
        // Get total count
        let total = Transaction::find()
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

    async fn update(&self, id: i32, mut transaction: ActiveModel) -> Result<TransactionModel, AppError> {
        // Set updated_at timestamp
        transaction.updated_at = Set(Some(Utc::now()));

        let result = Transaction::update_many()
            .set(transaction)
            .filter(Column::Id.eq(id))
            .filter(Column::DeletedAt.is_null())
            .exec(self.db.as_ref())
            .await
            .map_err(|err| {
                error!("DB error: {:?}", err);
                AppError::InternalError
            })?;

        if result.rows_affected == 0 {
            return Err(AppError::NotFound(format!("Transaction with id {} not found", id)));
        }

        // Fetch the updated transaction
        Transaction::find_by_id(id)
            .filter(Column::DeletedAt.is_null())
            .one(self.db.as_ref())
            .await
            .map_err(|err| {
                error!("DB error: {:?}", err);
                AppError::InternalError
            })?
            .ok_or_else(|| AppError::NotFound(format!("Transaction with id {} not found", id)))
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        let now = Utc::now();
        let result = Transaction::update_many()
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
            return Err(AppError::NotFound(format!("Transaction with id {} not found", id)));
        }

        Ok(())
    }
}
