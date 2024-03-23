use std::collections::HashMap;

pub struct State {
    pub state: HashMap<String, String>,
}

impl State {
    pub fn get(&self, key: &str) -> Option<String> {
        self.state.get(key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.state.insert(key, value);
    }
}
