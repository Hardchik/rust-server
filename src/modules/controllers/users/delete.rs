use actix_web::{web, HttpResponse};
use crate::modules::services::users::delete::delete_user_by_id;
use crate::error_handler::CustomError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeleteMessage {
    pub message: String,
}

pub async fn delete_user_by_id_handler(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let data = DeleteMessage {
        message: "User deleted successfully!".to_string(),
    };
    match delete_user_by_id(id.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(data)),
        Err(err) => {
            Ok(HttpResponse::InternalServerError().json(format!("Failed to delete user: {}", err)))
        }
    }
}
