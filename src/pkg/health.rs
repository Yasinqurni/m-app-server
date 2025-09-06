use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json, Extension,
};
use serde_json::json;
use std::sync::Arc;

use crate::pkg::response::ApiResponse;
use crate::di::AppModule;

pub async fn health_check(
    Extension(app_module): Extension<Arc<AppModule>>,
) -> Result<impl IntoResponse, StatusCode> {
    // Test database connectivity
    let db_status = match app_module.db.ping().await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    let health_status = if db_status == "healthy" {
        "healthy"
    } else {
        "unhealthy"
    };

    let status_code = if health_status == "healthy" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    let health_data = json!({
        "status": health_status,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "services": {
            "database": db_status
        }
    });

    Ok((
        status_code,
        Json(ApiResponse::success("Health check completed", Some(health_data), None)),
    ))
}
