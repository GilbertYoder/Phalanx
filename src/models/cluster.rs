use serde::Deserialize;
use crate::utils::lamport_clock::LamportClock;

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
    /// Vector clock for node operations
    pub node_ops_clock: LamportClock,
}

impl Cluster {
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
        self.node_ops_clock.increment();
    }
}

impl Cluster {
    pub fn gossip(&self) {

    }
}