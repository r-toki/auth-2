mod controller;
mod lib;
mod model;

use lib::config::CONFIG;

use actix_web::{get, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = &CONFIG.host;
    let port = &CONFIG.port;

    HttpServer::new(|| App::new().service(index))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}

#[get("/")]
async fn index() -> impl Responder {
    format!("HELLO WORLD!")
}
