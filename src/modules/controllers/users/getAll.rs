use actix_web::{HttpResponse};
use crate::modules::services::users::getAll::get_all_users;
use crate::error_handler::CustomError;

pub async fn get_all_users_handler() -> Result<HttpResponse, CustomError> {
    match get_all_users().await {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(err) => {
            Ok(HttpResponse::InternalServerError().json(format!("Failed to get users: {}", err)))
        }
    }
}
