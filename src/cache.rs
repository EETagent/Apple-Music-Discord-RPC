use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::iTunesInfos;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub version: i32,
    pub cache_file: String,
    pub data: HashMap<String, iTunesInfos>,
}


impl Cache {
    pub fn new() -> Cache {
        Cache {
            version: 3,
            cache_file: "cache.json".to_string(),
            data: HashMap::new(),
        }
    }
    pub fn get(&self, key: String) -> Option<&iTunesInfos> {
        return self.data.get(&key);
    }
    pub fn set(&mut self, key: String, value: iTunesInfos) {
        self.data.insert(key, value);
        self.save_cache();
    }

    pub fn load_cache(&mut self) {
        let text = std::fs::read_to_string(&self.cache_file);
        if text.is_err() {
            return;
        }
        let text = text.unwrap();

        let data: Cache = serde_json::from_str(&text).unwrap();

        if data.version != self.version {
            return;
        }

  
        self.data = data.data;
    }

    pub fn save_cache(&self) {
        let text = serde_json::to_string_pretty(&self).unwrap();

        std::fs::write(&self.cache_file, text).unwrap();
    }
}