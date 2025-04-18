use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use chrono::{Utc, Local};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/time")]
async fn time() -> impl Responder {
    let t: i64 = Utc::now()
        .timestamp_millis();
    HttpResponse::Ok()
    .content_type("application/json")
    .body(format!("{{\"t\":{}}}", t))
}

#[get("/localtime")]
async fn localtime() -> impl Responder {
    let t = Local::now()
        .timestamp_millis();
    
    HttpResponse::Ok()
    .content_type("application/json")
    .body(format!("{{\"t\":{}}}", t))
}

const MS_PER_HOUR: i64 = 60 * 60 * 1000;
const MS_PER_DAY: i64 = MS_PER_HOUR * 24;
const RECYCLING_PERIOD: i64 = 14;
const GARBAGE_PERIOD: i64 = 7;
const WEEK: i64 = 7;
const THURSDAY: i64 = 4;
const TUESDAY: i64 = 2;
const EPOCH_OFFSET: i64 = 4;
const RECYCLING_OFFSET: i64 = EPOCH_OFFSET + WEEK;

fn get_days_until_garbage(ms_since_epoch: i64) -> i64 {
    let day_of_week = ((ms_since_epoch / MS_PER_DAY) + EPOCH_OFFSET) % GARBAGE_PERIOD;
    (GARBAGE_PERIOD + THURSDAY - day_of_week) % GARBAGE_PERIOD
}

fn get_days_until_recycling(ms_since_epoch: i64) -> i64 {
    let day_of_biweek = ((ms_since_epoch / MS_PER_DAY) + RECYCLING_OFFSET) % RECYCLING_PERIOD;
    (RECYCLING_PERIOD + TUESDAY - day_of_biweek) % RECYCLING_PERIOD
}

#[get("/garbage")]
async fn garbage() -> impl Responder {
    let local = Local::now().timestamp_millis();
    let days_until_garbage = get_days_until_garbage(local);
    let days_until_recycling = get_days_until_recycling(local);
    
    HttpResponse::Ok()
    .content_type("application/json")
    .body(format!("{{\"t\":{},\"g\":{},\"r\":{}}}", local, days_until_garbage, days_until_recycling))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(time)
            .service(localtime)
            .service(garbage)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}