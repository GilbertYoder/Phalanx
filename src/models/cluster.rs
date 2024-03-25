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
    pub clock: LamportClock,
    pub rumors: Vec<Rumor>,
    pub recieved_rumors_ids: HashSet<String>,
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
            RumorMethod::SET,
            "Hi".to_string(),
            self.clock.time,
            self.myself.ip.to_string(),
        );
        self.gossip(&rumor);
    }

    pub fn recieve_rumor(&mut self, rumor: Rumor) {
        // Don't apply the same rumor twice.
        if self.recieved_rumors_ids.contains(&rumor.id) {
            return;
        }
        // If this is the first rumor heard, go ahead and ask for state.
        if self.clock.time == 0 {
            self.request_state(rumor.initiator);
            return;
        }
        self.clock.recieve(rumor.time);
        self.recieved_rumors_ids.insert(rumor.id.clone());
        self.gossip(&rumor);
        self.rumors.push(rumor);
    }

    pub fn request_state(&mut self, from_who: String) {
        println!("Requesting state from {}", from_who);
    }
}

#[derive(Deserialize)]
pub enum RumorMethod {
    GET,
    SET,
    DELETE,
    APPEND,
}

#[derive(Deserialize)]
pub struct Rumor {
    pub id: String,
    pub method: RumorMethod,
    pub message: String,
    pub time: usize,
    pub initiator: String,
}

impl Rumor {
    pub fn new(method: RumorMethod, message: String, time: usize, initiator: String) -> Rumor {
        Rumor {
            id: Uuid::new_v4().to_string(),
            method,
            message,
            time,
            initiator,
        }
    }
}
