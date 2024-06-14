use actix_web::{web};
use crate::modules::controllers::post::post_handler;
use crate::modules::controllers::get::get_handler;

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
            .route(web::get().to(get_user))
            .route(web::get().to(create_user))
            .route(web::delete().to(delete_user))
            .route(web::put().to(update_user))
    );
}