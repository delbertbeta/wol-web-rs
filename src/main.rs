use actix_web::{http::ContentEncoding, middleware, web, App, HttpServer};
use std::sync::Mutex;

mod app_state;
mod routes;
mod structs;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("wol-web-rs is listening at 0.0.0.0:8080");

    let history = utils::history::WakeHistoryManager::new();
    let app_state = web::Data::new(app_state::AppState {
        history: Mutex::new(history),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Compress::new(ContentEncoding::Gzip))
            .service(routes::wol::handler)
            .service(routes::index::handler)
            .service(routes::remove_history::handler)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
