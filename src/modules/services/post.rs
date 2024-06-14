use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct PostData {
    key: String,
    value: String,
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

pub async fn handle_post_request(data: PostData) -> Result<serde_json::Value, ServiceError> {
    // Here you can add any business logic or validation
    println!("Received data: {:?}", data);

    Ok(json!({
        "data": data,
        "status": "SUCCESS",
        "request": "POST"
    }))
}