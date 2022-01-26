use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use toml;

#[derive(Deserialize, Serialize, Debug)]
pub struct WakeUpHistoryItem {
    mac_address: String,
    port: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WakeHistory {
    items: Vec<WakeUpHistoryItem>,
}

#[derive(Debug)]
pub struct WakeHistoryManager {
    history: WakeHistory,
    file: File,
}

impl WakeHistoryManager {
    pub fn new() -> WakeHistoryManager {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("history.toml")
            .unwrap();

        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let initial_history: WakeHistory = match toml::from_str::<WakeHistory>(&contents) {
            Ok(wh) => wh,
            Err(e) => {
                eprint!("Parse history file error, try to init it. {:?}", e);

                let empty_history = WakeHistory { items: vec![] };

                file.write_all(toml::to_string(&empty_history).unwrap().as_bytes())
                    .unwrap();

                empty_history
            }
        };

        WakeHistoryManager {
            history: initial_history,
            file,
        }
    }
}
