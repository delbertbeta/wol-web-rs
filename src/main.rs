use actix_web::{http::ContentEncoding, middleware, App, HttpServer, web};
use std::sync::Mutex;

mod routes;
mod utils;

struct AppState {
    history: Mutex<utils::history::WakeHistoryManager>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("wol-web-rs is listening at 0.0.0.0:8080");

    let history = utils::history::WakeHistoryManager::new();
    let app_state = web::Data::new(AppState {
        history: Mutex::new(history),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Compress::new(ContentEncoding::Gzip))
            .service(routes::wol::handler)
            .service(routes::index::handler)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
