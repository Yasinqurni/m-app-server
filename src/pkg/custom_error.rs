use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;
use serde_json::json;
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    ValidationError(validator::ValidationErrors),
    DatabaseError(String),
    ConfigError(String),
    NotFound(String),
    InternalError,
    AuthenticationError(String),
    // AuthorizationError(String),
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_body) = match self {
            AppError::ValidationError(errors) => (
                StatusCode::BAD_REQUEST,
                json!({ "error": "validation", "details": errors }),
            ),
            AppError::DatabaseError(err) => {
                error!("Database error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({ "error": "database" }),
                )
            }
            AppError::ConfigError(msg) => {
                error!("Configuration error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({ "error": "config" }),
                )
            }
            AppError::NotFound(message ) => (
                StatusCode::NOT_FOUND,
                json!({ "error": "not_found", "message": message }),
            ),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "error": "internal server error" }),
            ),
            AppError::AuthenticationError(message) => (
                StatusCode::UNAUTHORIZED,
                json!({ "error": "authentication", "message": message }),
            ),
            // AppError::AuthorizationError(message) => (
            //         StatusCode::FORBIDDEN,
            //         json!({ "error": "authorization", "message": message }),
            // ),
            AppError::BadRequest(message) => {
                (
                    StatusCode::BAD_REQUEST,
                    json!({ "error": "bad_request", "message": message }),
                )
            }
        };

        (status, Json(error_body)).into_response()
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        AppError::ValidationError(errors)
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(_) => AppError::NotFound(err.to_string()),
            _ => {
                error!("Database error: {:?}", err);
                AppError::DatabaseError(err.to_string())
            }
        }
    }
}


impl From<config::ConfigError> for AppError {
	fn from(err: config::ConfigError) -> Self {
		AppError::ConfigError(err.to_string())
	}
}