use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetData {
    data: String,
}

#[derive(Debug)]
pub enum ServiceError {
    InvalidData,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub async fn handle_get_request() -> Result<serde_json::Value, ServiceError> {
    let data = GetData {
        data: "Hello, World!".to_string(),
    };

    Ok(json!({
        "data": data,
        "status": "SUCCESS",
        "request": "GET"
    }))
}