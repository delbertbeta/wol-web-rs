use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::utils::wol::MagicPacket;

#[derive(Deserialize, Debug)]
pub struct WolParams {
    mac_address: String,
    port: u16,
}

#[derive(Serialize)]
struct WolResponse {
    code: usize,
    msg: String,
}

fn get_error_res<A: std::fmt::Debug>(e: A) -> HttpResponse {
    HttpResponse::BadRequest().json(WolResponse {
        code: 1,
        msg: format!("Error: {:?}", e),
    })
}

#[post("/wol")]
pub async fn handler(wol_params: web::Json<WolParams>) -> impl Responder {
    let magic_packet = MagicPacket::new(&wol_params.mac_address, wol_params.port);

    match magic_packet {
        Ok(mp) => match mp.send() {
            Ok(_) => HttpResponse::Ok().json(WolResponse {
                code: 0,
                msg: "Magic packet sent".to_string(),
            }),
            Err(e) => get_error_res(e),
        },
        Err(e) => get_error_res(e),
    }
}
