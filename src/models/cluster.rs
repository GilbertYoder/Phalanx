use serde::Deserialize;

#[derive(Deserialize)]
pub struct Node {
    pub ip: String,
    pub port: usize,
    pub last_heartbeat: usize,
}
pub struct Cluster {
    /// Represents the current node.
    pub myself: Node,
    /// Represents the list of *other* nodes.
    pub nodes: Vec<Node>,
}

impl Cluster {
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
}
