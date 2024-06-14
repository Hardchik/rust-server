use actix_web::{web, HttpResponse};
use crate::modules::services::post::{handle_post_request, PostData};
use serde_json::json;

pub async fn post_handler(data: web::Json<PostData>) -> HttpResponse {
    match handle_post_request(data.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::BadRequest().json({
            json!({ "status": "FAILURE", "error": err.to_string() })
        }),
    }
}
