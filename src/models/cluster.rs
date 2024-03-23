use crate::utils::lamport_clock::LamportClock;
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
    /// Vector clock for node operations
    pub node_ops_clock: LamportClock,
}

impl Cluster {
    pub fn gossip(&self) {
        for node in self.nodes.iter() {
            println!("Gossiping to {}", node.ip);
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
        self.node_ops_clock.increment();
    }
}
