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
    pub clock: LamportClock,
    pub operations: Vec<Rumor>,
}

impl Cluster {
    pub fn gossip(&self, rumor: &Rumor) {
        for node in self.nodes.iter() {
            println!("Gossiping to {}", node.ip);
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
        self.clock.increment();
        let rumor = Rumor::new(
            GossipMethod::SET,
            "Hi".to_string(),
            self.clock.time,
        );
        self.gossip(&rumor);
    }

    pub fn recieve_node_gossip(&mut self, rumor: Rumor) {
        if self.recieved_gossip.contains(&rumor.id) {
            return;
        }
        self.clock.recieve(rumor.time);
        self.recieved_gossip.insert(rumor.id.clone());
        self.gossip(&rumor);
        self.operations.push(rumor);
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
pub struct Rumor {
    pub id: String,
    pub method: GossipMethod,
    pub message: String,
    pub time: usize,
}

impl Rumor {
    pub fn new(method: GossipMethod, message: String, time: usize) -> Rumor {
        Rumor {
            id: Uuid::new_v4().to_string(),
            method,
            message,
            time,
        }
    }
}
