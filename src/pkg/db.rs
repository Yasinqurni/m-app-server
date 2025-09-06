use sea_orm::{ConnectOptions as SeaOrmConnectOptions, Database, DatabaseConnection};
use crate::pkg::config::Config;
use std::sync::Arc;
use std::time::Duration;
use log::LevelFilter;

pub struct DBConnection {
    pub db: Arc<DatabaseConnection>,
}

impl DBConnection {
    pub async fn new(config: Arc<Config>) -> Self {
        assert_eq!(
            config.db.driver, "postgres",
            "Only PostgreSQL is supported in this DBConnection"
        );

        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}?sslmode={}",
            config.db.user,
            config.db.password,
            config.db.host,
            config.db.port,
            config.db.name,
            config.db.ssl_mode
        );

        println!("{:?}", db_url.clone());

        let mut sea_opts = SeaOrmConnectOptions::new(&db_url);
        sea_opts
            .max_connections(5)
            .connect_timeout(Duration::from_secs(30))
            .sqlx_logging(true)
            .sqlx_logging_level(LevelFilter::Debug);

        let db = Database::connect(sea_opts)
            .await
            .expect("Failed to connect to PostgreSQL database");

        Self {
            db: Arc::new(db),
        }
    }
}