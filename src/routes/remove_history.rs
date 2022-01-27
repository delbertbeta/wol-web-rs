use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{app_state::AppState, structs::wol_response::WolResponse};

#[derive(Deserialize, Debug)]
pub struct RemoveHistoryParams {
    mac_address: String,
}

#[post("/remove_history")]
pub async fn handler(
    remove_history_params: web::Json<RemoveHistoryParams>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let mut history_manager = app_state.history.lock().unwrap();

    history_manager
        .remove(&remove_history_params.mac_address)
        .expect("Error: Can't save to history file");

    HttpResponse::Ok().json(WolResponse {
        code: 0,
        msg: "History removed".to_string(),
    })
}
