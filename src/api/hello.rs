use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn responder() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}