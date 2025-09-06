use async_trait::async_trait;
use crate::module::product::v1::entity::{
    request::{
        create_product_request::CreateProductRequest,
        update_product_request::UpdateProductRequest,
        get_product_request::GetProductRequest,
        list_product_request::GetListProductQuery,
    },
    response::{
        product_response::ProductResponse,
        list_product_response::ListProductResponse,
    },
    model::product::ActiveModel,
};
use crate::module::repository::product_repository::ProductRepository;
use crate::pkg::custom_error::AppError;
use sea_orm::{prelude::*, Set};
use std::sync::Arc;
use chrono::Utc;

#[async_trait]
pub trait ProductUsecase: Send + Sync {
    async fn create_product(&self, request: CreateProductRequest) -> Result<ProductResponse, AppError>;
    async fn get_product(&self, request: GetProductRequest) -> Result<ProductResponse, AppError>;
    async fn list_products(&self, query: GetListProductQuery) -> Result<ListProductResponse, AppError>;
    async fn update_product(&self, id: i32, request: UpdateProductRequest) -> Result<ProductResponse, AppError>;
    async fn delete_product(&self, id: i32) -> Result<(), AppError>;
}

pub struct ProductUsecaseImpl {
    product_repository: Arc<dyn ProductRepository>,
}

impl ProductUsecaseImpl {
    pub fn new(product_repository: Arc<dyn ProductRepository>) -> Self {
        Self { product_repository }
    }
}

#[async_trait]
impl ProductUsecase for ProductUsecaseImpl {
    async fn create_product(&self, request: CreateProductRequest) -> Result<ProductResponse, AppError> {

        // Check if product with same name already exists
        if let Some(_) = self.product_repository.find_by_name(&request.name).await? {
            return Err(AppError::BadRequest("Product with this name already exists".to_string()));
        }

        // Create active model
        let product_active_model = ActiveModel {
            name: Set(request.name.clone()),
            hpp_amount: Set(request.hpp_amount),
            selling_amount: Set(request.selling_amount),
            created_at: Set(Some(Utc::now())),
            updated_at: Set(Some(Utc::now())),
            deleted_at: Set(None),
            ..Default::default()
        };

        // Save to repository
        self.product_repository.create(product_active_model).await?;

        // Find the created product to return
        let created_product = self.product_repository
            .find_by_name(&request.name)
            .await?
            .ok_or_else(|| AppError::InternalError)?;

        Ok(ProductResponse::from(created_product))
    }

    async fn get_product(&self, request: GetProductRequest) -> Result<ProductResponse, AppError> {
        let product = self.product_repository
            .find_by_id(request.id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Product with id {} not found", request.id)))?;

        Ok(ProductResponse::from(product))
    }

    async fn list_products(&self, query: GetListProductQuery) -> Result<ListProductResponse, AppError> {
        let paginated_result = self.product_repository.find_with_pagination(query).await?;
        Ok(ListProductResponse::from(paginated_result))
    }

    async fn update_product(&self, id: i32, request: UpdateProductRequest) -> Result<ProductResponse, AppError> {
        // Check if product exists
        let existing_product = self.product_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Product with id {} not found", id)))?;

        // Check if new name conflicts with existing product (if name is being updated)
        if let Some(new_name) = &request.name {
            if new_name != &existing_product.name {
                if let Some(_) = self.product_repository.find_by_name(new_name).await? {
                    return Err(AppError::BadRequest("Product with this name already exists".to_string()));
                }
            }
        }

        // Create active model with only changed fields
        let mut product_active_model = ActiveModel {
            id: Set(id),
            ..Default::default()
        };

        if let Some(name) = request.name {
            product_active_model.name = Set(name);
        }
        if let Some(hpp_amount) = request.hpp_amount {
            product_active_model.hpp_amount = Set(hpp_amount);
        }
        if let Some(selling_amount) = request.selling_amount {
            product_active_model.selling_amount = Set(selling_amount);
        }

        // Update in repository
        let updated_product = self.product_repository.update(id, product_active_model).await?;

        Ok(ProductResponse::from(updated_product))
    }

    async fn delete_product(&self, id: i32) -> Result<(), AppError> {
        // Check if product exists
        self.product_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Product with id {} not found", id)))?;

        // Soft delete
        self.product_repository.delete(id).await?;

        Ok(())
    }
}