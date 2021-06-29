use actix_web::{App, HttpServer};

mod endpoint;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(endpoint::publish))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
