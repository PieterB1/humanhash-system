use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("System service running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health))
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
