#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
use modules::routes;
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod modules {
    pub mod controllers {
        pub mod users {
          pub mod post;
          pub mod getAll;
          pub mod get;
          pub mod put;
          pub mod delete;
        }
        pub mod post;
        pub mod get;
    }
    pub mod routes;
    pub mod services {
        pub mod users {
          pub mod post;
          pub mod getAll;
          pub mod get;
          pub mod put;
          pub mod delete;
        }
        pub mod post;
        pub mod get;
    }
}

mod models;
mod db;
mod error_handler;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .configure(routes::config)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
