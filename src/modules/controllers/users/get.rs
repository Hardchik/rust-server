use actix_web::{web, HttpResponse};
use crate::modules::services::users::get::get_user_by_id;
use crate::error_handler::CustomError;

pub async fn get_user_by_id_handler(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    match get_user_by_id(id.into_inner()).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            Ok(HttpResponse::InternalServerError().json(format!("Failed to get user: {}", err)))
        }
    }
}
