use actix_web::{HttpResponse};
use crate::modules::services::get::{handle_get_request};
use serde_json::json;

pub async fn get_handler() -> HttpResponse {
    match handle_get_request().await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::BadRequest().json({
            json!({ "status": "FAILURE", "error": err.to_string() })
        }),
    }
}
