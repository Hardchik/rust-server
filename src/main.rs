use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct MyResponse {
    data: String,
}

#[get("/get")]
async fn get_handler() -> impl Responder {
    let response = MyResponse {
        data: "Hello, World!".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_handler)
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}
