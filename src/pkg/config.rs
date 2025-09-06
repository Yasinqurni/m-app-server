use dotenvy::dotenv;
use std::env;
use std::sync::{Arc, OnceLock};

#[derive(Debug, Clone)]
pub struct Config {
    pub db: DbConfig,
    pub app: AppConfig,
    pub jwt: Jwt,
}

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub host: String,
    pub name: String,
    pub user: String,
    pub password: String,
    pub port: u16,
    pub driver: String,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct Jwt {
    pub secret: String,
    pub expiration: i64,
}

static CONFIG: OnceLock<Arc<Config>> = OnceLock::new();

pub fn init_config() -> Arc<Config> {
    CONFIG
        .get_or_init(|| {
            dotenv().ok();

            Arc::new(Config {
                app: AppConfig {
                    port: env::var("APP_PORT")
                        .unwrap_or_else(|_| "3001".to_string()) 
                        .parse() 
                        .unwrap_or(3001),
                },
                db: DbConfig {
                    host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
                    name: env::var("DB_NAME").unwrap_or_else(|_| "test_db".to_string()),
                    user: env::var("DB_USER").unwrap_or_else(|_| "root".to_string()),
                    password: env::var("DB_PASSWORD").unwrap_or_else(|_| "".to_string()),
                    port: env::var("DB_PORT")
                        .unwrap_or_else(|_| "5432".to_string())
                        .parse()
                        .unwrap_or(5432),
                    driver: env::var("DB_DRIVER").unwrap_or_else(|_| "postgres".to_string()),
                },
                jwt: Jwt {
                    secret: env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string()),
                    expiration: env::var("JWT_EXPIRATION")
                        .unwrap_or_else(|_| "3600".to_string())
                        .parse()
                        .unwrap_or(3600),
                },
            })
        })
        .clone()
}
