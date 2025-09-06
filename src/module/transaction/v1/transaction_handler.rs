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

use crate::module::transaction::v1::entity::{
    request::{
        create_transaction_request::CreateTransactionRequest,
        update_transaction_request::UpdateTransactionRequest,
        get_transaction_request::GetTransactionRequest,
        list_transaction_request::GetListTransactionQuery,
    },
};
use crate::pkg::custom_error::AppError;
use crate::pkg::response::ApiResponse;
use crate::di::AppModule;


pub fn routes() -> Router {
	Router::new()
		.route("/", post(create_transaction))
		.route("/", get(list_transactions))
		.route("/:id", get(get_transaction))
		.route("/:id", put(update_transaction))
		.route("/:id", delete(delete_transaction))
}

async fn create_transaction(
    Extension(app_module): Extension<Arc<AppModule>>,
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate().map_err(|e| {
        AppError::BadRequest(e.to_string().replace("\n", "\n"))
    })?;

    let transaction = app_module.transaction_usecase.create_transaction(payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Transaction created successfully", Some(json!(transaction)), None)),
    ))
}

async fn get_transaction(
    Extension(app_module): Extension<Arc<AppModule>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let request = GetTransactionRequest { id };
    request.validate().map_err(|e| {
        AppError::BadRequest(e.to_string().replace("\n", "\n"))
    })?;

    let transaction = app_module.transaction_usecase.get_transaction(request).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Transaction retrieved successfully", Some(json!(transaction)), None)),
    ))
}

async fn list_transactions(
    Extension(app_module): Extension<Arc<AppModule>>,
    Query(query): Query<GetListTransactionQuery>,
) -> Result<impl IntoResponse, AppError> {
    let transactions = app_module.transaction_usecase.list_transactions(query).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Transactions retrieved successfully", Some(json!(transactions)), None)),
    ))
}

async fn update_transaction(
    Extension(app_module): Extension<Arc<AppModule>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTransactionRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate().map_err(|e| {
        AppError::BadRequest(e.to_string().replace("\n", "\n"))
    })?;

    let transaction = app_module.transaction_usecase.update_transaction(id, payload).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Transaction updated successfully", Some(json!(transaction)), None)),
    ))
}

async fn delete_transaction(
    Extension(app_module): Extension<Arc<AppModule>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    app_module.transaction_usecase.delete_transaction(id).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Transaction deleted successfully", Some(json!({})), None)),
    ))
}
