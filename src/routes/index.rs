use actix_web::{get, HttpResponse, Responder, http::header::ContentType};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[get("/")]
pub async fn handler() -> impl Responder {
    let index = IndexTemplate{};
    
    HttpResponse::Ok()
        .set(ContentType::html())
        .body(index.render().unwrap())
}
