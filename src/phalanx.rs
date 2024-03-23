use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Node {
    pub ip: String,
    pub port: usize,
    pub name: String,
    pub last_heartbeat: usize,
}

pub struct Phalanx {
    pub state: Mutex<HashMap<String, String>>,
    pub nodes: Mutex<Vec<Node>>,
}
