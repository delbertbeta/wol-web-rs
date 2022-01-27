use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, SeekFrom};
use toml;

#[derive(Deserialize, Serialize, Debug)]
pub struct WakeUpHistoryItem {
    pub nickname: String,
    pub mac_address: String,
    pub port: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WakeHistory {
    pub items: Vec<WakeUpHistoryItem>,
}

#[derive(Debug)]
pub struct WakeHistoryManager {
    pub history: WakeHistory,
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

                file.seek(SeekFrom::Start(0)).unwrap();
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

    pub fn find_index_of_address(&self, mac_address: &str) -> Option<usize> {
        self.history
            .items
            .iter()
            .position(|h| h.mac_address == mac_address)
    }

    pub fn insert(
        &mut self,
        nickname: &str,
        mac_address: &str,
        port: usize,
    ) -> std::io::Result<()> {
        match self.find_index_of_address(&mac_address) {
            Some(_) => Ok(()),
            None => {
                self.history.items.push(WakeUpHistoryItem {
                    nickname: String::from(nickname),
                    mac_address: String::from(mac_address),
                    port,
                });
                self.save_to_file()
            }
        }
    }

    pub fn remove(&mut self, mac_address: &str) -> std::io::Result<()> {
        match self.find_index_of_address(&mac_address) {
            Some(index) => {
                self.history.items.remove(index);
                self.save_to_file()
            }
            None => Ok(()),
        }
    }

    fn save_to_file(&mut self) -> std::io::Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        self.file
            .write_all(toml::to_string(&self.history).unwrap().as_bytes())
    }
}
