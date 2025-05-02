use actix_web::{get, HttpResponse, Responder};
use chrono::Local;

#[get("/localtime")]
async fn responder() -> impl Responder {
    let mut l = Local::now().timestamp_millis();

    let offset = Local::now().offset().local_minus_utc() * 1000;
    l = l + offset as i64;

    HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("{{\"t\":{}}}", l))
}