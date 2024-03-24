use crate::utils::lamport_clock::LamportClock;
use serde::Deserialize;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Node {
    pub ip: String,
    pub port: usize,
    pub last_heartbeat: usize,
}

pub struct Cluster {
    pub myself: Node,
    pub nodes: Vec<Node>,
    pub recieved_gossip: HashSet<String>,
    pub node_ops_clock: LamportClock,
    pub node_ops: Vec<Gossip>,
}

impl Cluster {
    pub fn gossip(&self, gossip: Gossip) {
        for node in self.nodes.iter() {
            println!("Gossiping to {}", node.ip);
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
        self.node_ops_clock.increment();
        let gossip = Gossip::new(
            GossipMethod::SET,
            "Hi".to_string(),
            self.node_ops_clock.counter,
        );
        self.gossip(gossip);
    }

    pub fn recieve_node_gossip(&mut self, gossip: Gossip) {
        if !self.recieved_gossip.contains(&gossip.id) {
            return;
        }
        self.node_ops_clock.recieve(gossip.time);
        self.node_ops.push(gossip);
        self.gossip(gossip);
        self.recieved_gossip.insert(gossip.id);
    }
}

#[derive(Deserialize)]
pub enum GossipMethod {
    GET,
    SET,
    DELETE,
    APPEND,
}

#[derive(Deserialize)]
pub struct Gossip {
    pub id: String,
    pub method: GossipMethod,
    pub message: String,
    pub time: usize,
}

impl Gossip {
    pub fn new(method: GossipMethod, message: String, time: usize) -> Gossip {
        Gossip {
            id: Uuid::new_v4().to_string(),
            method,
            message,
            time,
        }
    }
}
