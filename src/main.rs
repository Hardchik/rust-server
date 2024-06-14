use actix_web::{App, HttpServer};
use modules::routes;

mod modules {
    pub mod controllers {
        pub mod post;
        pub mod get;
    }
    pub mod routes;
    pub mod services {
        pub mod post;
        pub mod get;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::config)
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}
