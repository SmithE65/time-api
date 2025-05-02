mod api;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::hello::responder)
            .service(api::time::responder)
            .service(api::localtime::responder)
            .service(api::garbage::responder)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
