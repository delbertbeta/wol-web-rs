use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::structs::wol_response::WolResponse;
use crate::utils::error_response::get_error_res;
use crate::utils::wol::MagicPacket;

#[derive(Deserialize, Debug)]
pub struct WolParams {
    nickname: String,
    mac_address: String,
    port: u16,
}

#[post("/wol")]
pub async fn handler(
    wol_params: web::Json<WolParams>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let magic_packet = MagicPacket::new(&wol_params.mac_address, wol_params.port);

    match magic_packet {
        Ok(mp) => match mp.send() {
            Ok(_) => {
                let mut history_manager = app_state.history.lock().unwrap();

                history_manager
                    .insert(
                        &wol_params.nickname,
                        &wol_params.mac_address,
                        wol_params.port.into(),
                    )
                    .expect("Error: Can't save to history file");

                HttpResponse::Ok().json(WolResponse {
                    code: 0,
                    msg: "Magic packet sent".to_string(),
                })
            }
            Err(e) => get_error_res(e),
        },
        Err(e) => get_error_res(e),
    }
}
