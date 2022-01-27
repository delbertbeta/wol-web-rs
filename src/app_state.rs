use crate::utils::history::WakeHistoryManager;
use std::sync::Mutex;

pub struct AppState {
    pub history: Mutex<WakeHistoryManager>,
}
