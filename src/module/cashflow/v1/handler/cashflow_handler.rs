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

use crate::module::cashflow::v1::entity::{
    request::{
        create_cashflow_request::CreateCashflowRequest,
        update_cashflow_request::UpdateCashflowRequest,
        get_cashflow_request::GetCashflowRequest,
        list_cashflow_request::GetListCashflowQuery,
    },
};
use crate::pkg::custom_error::AppError;
use crate::pkg::response::ApiResponse;
use crate::di::AppModule;


pub fn routes() -> Router {
	Router::new()
		.route("/", post(create_cashflow))
		.route("/", get(list_cashflows))
		.route("/:id", get(get_cashflow))
		.route("/:id", put(update_cashflow))
		.route("/:id", delete(delete_cashflow))
}

async fn create_cashflow(
    Extension(app_module): Extension<Arc<AppModule>>,
    Json(payload): Json<CreateCashflowRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate().map_err(|e| {
        AppError::BadRequest(e.to_string().replace("\n", "\n"))
    })?;

    let cashflow = app_module.cashflow_usecase.create_cashflow(payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success("Cashflow created successfully", Some(json!(cashflow)), None)),
    ))
}

async fn get_cashflow(
    Extension(app_module): Extension<Arc<AppModule>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let request = GetCashflowRequest { id };
    request.validate().map_err(|e| {
        AppError::BadRequest(e.to_string().replace("\n", "\n"))
    })?;

    let cashflow = app_module.cashflow_usecase.get_cashflow(request).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Cashflow retrieved successfully", Some(json!(cashflow)), None)),
    ))
}

async fn list_cashflows(
    Extension(app_module): Extension<Arc<AppModule>>,
    Query(query): Query<GetListCashflowQuery>,
) -> Result<impl IntoResponse, AppError> {
    let cashflows = app_module.cashflow_usecase.list_cashflows(query).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Cashflows retrieved successfully", Some(json!(cashflows)), None)),
    ))
}

async fn update_cashflow(
    Extension(app_module): Extension<Arc<AppModule>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCashflowRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate().map_err(|e| {
        AppError::BadRequest(e.to_string().replace("\n", "\n"))
    })?;

    let cashflow = app_module.cashflow_usecase.update_cashflow(id, payload).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Cashflow updated successfully", Some(json!(cashflow)), None)),
    ))
}

async fn delete_cashflow(
    Extension(app_module): Extension<Arc<AppModule>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    app_module.cashflow_usecase.delete_cashflow(id).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success("Cashflow deleted successfully", Some(json!({})), None)),
    ))
}
