use actix_web::{web, App, get, HttpRequest, HttpServer, Responder};

#[get("/test/0.1.0/download")]
async fn greet(req: HttpRequest) -> impl Responder {
    println!("{:?}", req.headers());
    format!("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

