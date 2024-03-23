use std::collections::HashMap;
use std::sync::Mutex;

pub struct State {
    pub state: Mutex<HashMap<String, String>>,
}

impl State {
    pub fn get(&self, key: &str) -> Option<String> {
        let state = self.state.lock().unwrap();
        state.get(key).cloned()
    }

    pub fn set(&self, key: String, value: String) {
        let mut state = self.state.lock().unwrap();
        state.insert(key, value);
    }
}
