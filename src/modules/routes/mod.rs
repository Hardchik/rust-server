use actix_web::{web};
use crate::modules::controllers::post::post_handler;
use crate::modules::controllers::get::get_handler;
use crate::modules::controllers::users::post::create_user_handler;
use crate::modules::controllers::users::getAll::get_all_users_handler;
use crate::modules::controllers::users::get::get_user_by_id_handler;
use crate::modules::controllers::users::put::update_user_by_id_handler;
use crate::modules::controllers::users::delete::delete_user_by_id_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/post")
            .route(web::post().to(post_handler))
    )
    .service(
        web::resource("/get")
            .route(web::get().to(get_handler))
    )
    .service(
        web::resource("/request")
            .route(web::post().to(create_user_handler))
            .route(web::get().to(get_all_users_handler))
    )
    .service(
        web::resource("/request/{id}")
            .route(web::get().to(get_user_by_id_handler))
            .route(web::put().to(update_user_by_id_handler))
            .route(web::delete().to(delete_user_by_id_handler))
    );
}