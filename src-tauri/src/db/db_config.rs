use std::fs;

use serde::{Serialize, Deserialize};

const CONFIG_FILE: &str = "db_path_config.json";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DbConfig {
    pub db_path: Option<String>,
}

impl DbConfig {
    pub fn load() -> Self {
        if let Ok(content) = fs::read_to_string(CONFIG_FILE) {
            if let Ok(cfg) = serde_json::from_str::<DbConfig>(&content) {
                return cfg;
            }
        }
        DbConfig::default()
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(CONFIG_FILE, json);
        }
    }

    pub fn set_db_path(&mut self, path: String) {
        self.db_path = Some(path);
        self.save();
    }

    pub fn get_db_path(&self) -> Option<String> {
        self.db_path.clone()
    }
}