mod error;
mod models;
mod prelude;
mod routes;
mod schema;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use actix_web::{App, HttpServer};
use routes::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    // Main router manager
    HttpServer::new(|| App::new().service(routes()))
        .bind_openssl(("127.0.0.1", 8080), builder)?
        .run()
        .await
}
