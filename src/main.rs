mod handlers;
// mod models;
// mod db;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(handlers::config)
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
