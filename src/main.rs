mod models;
mod routes;
mod schema;

use actix_web::{App, HttpServer};
use routes::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| App::new().service(routes()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
