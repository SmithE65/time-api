use actix_web::{get, HttpResponse, Responder};
use chrono::Utc;

#[get("/time")]
async fn responder() -> impl Responder {
    let t: i64 = Utc::now().timestamp_millis();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("{{\"t\":{}}}", t))
}