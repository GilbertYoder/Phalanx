use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub state: HashMap<String, String>,
}

impl Data {
    pub fn get(&self, key: &str) -> Option<String> {
        self.state.get(key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.state.insert(key, value);
    }
}
