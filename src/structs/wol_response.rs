use serde::Serialize;

#[derive(Serialize)]
pub struct WolResponse {
    pub code: usize,
    pub msg: String,
}
