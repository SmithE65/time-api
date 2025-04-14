use std::time::SystemTime;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/time")]
async fn time() -> impl Responder {
    let t: u128 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    HttpResponse::Ok()
    .content_type("application/json")
    .body(format!("{{\"t\":{}}}", t))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(time)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}