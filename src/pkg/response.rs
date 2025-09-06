use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    status: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<serde_json::Value>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(message: &str, data: Option<T>, meta: Option<serde_json::Value>) -> Self {
        ApiResponse {
            status: "success".to_string(),
            message: message.to_string(),
            data,
            errors: None,
            meta,
        }
    }
}
