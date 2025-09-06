use axum::{
	extract::{Extension, Path, Query}, 
    http::StatusCode, 
    response::IntoResponse, 
    routing::{get, post, put, delete}, 
    Json, 
    Router
};
use serde_json::json;
use validator::Validate;
use std::sync::Arc;

use crate::module::product::v1::entity::{
    request::{
        create_product_request::CreateProductRequest,
        update_product_request::UpdateProductRequest,
        get_product_request::GetProductRequest,
        list_product_request::GetListProductQuery,
    },
};
use crate::pkg::custom_error::AppError;
use crate::pkg::response::ApiResponse;
use crate::di::AppModule;


pub fn routes() -> Router {
	Router::new()
		.route("/", post(create_product))
		.route("/", get(list_products))
		.route("/:id", get(get_product))
		.route("/:id", put(update_product))
		.route("/:id", delete(delete_product))
}

async fn create_product(
    Extension(app_module): Extension<Arc<AppModule>>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate().map_err(|e| {
        AppError::BadRequest(e.to_string().replace("\n", "\n"))
    })?;

    let product = app_module.product_usecase.create_product(payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Product created successfully", Some(json!(product)), None)),
    ))
}

async fn get_product(
    Extension(app_module): Extension<Arc<AppModule>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let request = GetProductRequest { id };
    request.validate().map_err(|e| {
        AppError::BadRequest(e.to_string().replace("\n", "\n"))
    })?;

    let product = app_module.product_usecase.get_product(request).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Product retrieved successfully", Some(json!(product)), None)),
    ))
}

async fn list_products(
    Extension(app_module): Extension<Arc<AppModule>>,
    Query(query): Query<GetListProductQuery>,
) -> Result<impl IntoResponse, AppError> {
    let products = app_module.product_usecase.list_products(query).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Products retrieved successfully", Some(json!(products)), None)),
    ))
}

async fn update_product(
    Extension(app_module): Extension<Arc<AppModule>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate().map_err(|e| {
        AppError::BadRequest(e.to_string().replace("\n", "\n"))
    })?;

    let product = app_module.product_usecase.update_product(id, payload).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Product updated successfully", Some(json!(product)), None)),
    ))
}

async fn delete_product(
    Extension(app_module): Extension<Arc<AppModule>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    app_module.product_usecase.delete_product(id).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Product deleted successfully", Some(json!({})), None)),
    ))
}