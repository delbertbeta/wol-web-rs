use crate::structs::wol_response::WolResponse;
use actix_web::HttpResponse;

pub fn get_error_res<A: std::fmt::Debug>(e: A) -> HttpResponse {
    HttpResponse::BadRequest().json(WolResponse {
        code: 1,
        msg: format!("Error: {:?}", e),
    })
}
