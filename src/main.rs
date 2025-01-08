use actix_web::{App, HttpServer};
mod naive_bayes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(naive_bayes::train))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
