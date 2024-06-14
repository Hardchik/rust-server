use actix_web::{web, HttpResponse};
use crate::modules::services::users::post::create_user;
use crate::models::UserDto;
use crate::error_handler::CustomError;

pub async fn create_user_handler(new_user: web::Json<UserDto>) -> Result<HttpResponse, CustomError> {
    match create_user(new_user.into_inner()).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            // Log the error or handle it accordingly
            Ok(HttpResponse::InternalServerError().json(format!("Failed to create user: {}", err)))
        }
    }
}
