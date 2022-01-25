use actix_web::{App, HttpServer};

mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routes::wol::handler))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
