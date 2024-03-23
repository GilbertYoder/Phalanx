use std::sync::Mutex;
pub struct Node {
    pub ip: String,
    pub port: usize,
    pub name: String,
    pub last_heartbeat: usize,
}
pub struct Cluster {
    pub nodes: Vec<Node>,
}

impl Cluster {
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
}
