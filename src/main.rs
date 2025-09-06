mod pkg;
mod module;
mod di;

use std::net::SocketAddr;
use std::sync::Arc;

use tracing_subscriber::EnvFilter;
use tower_http::trace::TraceLayer;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Init tracing logger with environment filter (info level by default)
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    // Load configuration
    let config = pkg::config::init_config();

    // Connect to database
    let db = pkg::db::DBConnection::new(config.clone())
        .await;

    // Initialize dependency injection (services, repositories, etc.)
    let di_module = Arc::new(di::initialize_di(db.db.clone()));

    // Configure Axum application with routing and dependencies
    let mut app = module::app_module::configure(config.clone(), di_module.clone()).await;

    // Add HTTP logging middleware (TraceLayer)
    app = app.layer(TraceLayer::new_for_http());

    // Define address to bind the server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.app.port));
    println!("🚀 Server is running on: http://{}", addr);

    // Start the Axum server
    let listener = TcpListener::bind(addr)
        .await
        .expect("Could not bind to the address");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}