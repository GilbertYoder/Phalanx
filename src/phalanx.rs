use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Phalanx {
    pub state: Mutex<HashMap<String, String>>,
}
