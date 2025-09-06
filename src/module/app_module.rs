use std::sync::Arc;

use axum::{
    http::{header, Method},
    Extension, Router,
    routing::get,
};
use tower_cookies::CookieManagerLayer;
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
};

use crate::di::AppModule;
use crate::pkg::config::Config;
use crate::pkg::health;

use super::product::product_module;
use super::cashflow::cashflow_module;

pub async fn configure(config: Arc<Config>, di_module: Arc<AppModule>) -> Router {
    // Setup CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    // Compose the router
    Router::new()
        .route("/health", get(health::health_check))
        .merge(product_module::configure())
        .merge(cashflow_module::configure())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(CookieManagerLayer::new())
        .layer(Extension(config.clone()))
        .layer(Extension(di_module.clone()))
}