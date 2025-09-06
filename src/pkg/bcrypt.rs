use bcrypt::{hash, verify, DEFAULT_COST};
use tokio::task;

/// Hash a password using bcrypt
pub async fn hash_password(password: String) -> Result<String, String> {
    task::spawn_blocking(move || {
        hash(password, DEFAULT_COST).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Verify a plain password against a bcrypt hash
pub async fn verify_password(password: String, hashed: String) -> Result<bool, String> {
    task::spawn_blocking(move || {
        verify(password, &hashed).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}