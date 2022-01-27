use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};
use askama::Template;

use crate::{app_state::AppState, utils::history::WakeHistory};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    history: &'a WakeHistory,
}

#[get("/")]
pub async fn handler(app_state: web::Data<AppState>) -> impl Responder {
    let history_manager = app_state.history.lock().unwrap();

    let index = IndexTemplate {
        history: &history_manager.history,
    };

    HttpResponse::Ok()
        .set(ContentType::html())
        .body(index.render().unwrap())
}
