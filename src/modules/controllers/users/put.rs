use actix_web::{web, HttpResponse};
use crate::modules::services::users::put::update_user_by_id;
use crate::models::UserDto;
use crate::error_handler::CustomError;

pub async fn update_user_by_id_handler(id: web::Path<i32>, user_dto: web::Json<UserDto>) -> Result<HttpResponse, CustomError> {
    match update_user_by_id(id.into_inner(), user_dto.into_inner()).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            Ok(HttpResponse::InternalServerError().json(format!("Failed to update user: {}", err)))
        }
    }
}
