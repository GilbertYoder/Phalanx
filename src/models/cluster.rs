use crate::utils::lamport_clock::LamportClock;
use crate::models::state::Data;
use serde::Deserialize;
use std::collections::HashSet;
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize, Clone)]
pub struct Node {
    pub ip: String,
    pub port: usize,
    pub last_heartbeat: usize,
}

#[derive(Deserialize)]
pub struct Cluster {
    pub myself: Node,
    pub nodes: Vec<Node>,
    pub clock: LamportClock,
    pub rumors: Vec<Rumor>,
    pub recieved_rumors_ids: HashSet<String>,
    pub data: Data,
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
            self.myself.ip.to_string() + ":" + &self.myself.port.to_string(),
        );
        self.gossip(&rumor);
    }

    pub async fn recieve_rumor(&mut self, rumor: Rumor) {
        // Don't apply the same rumor twice.
        if self.recieved_rumors_ids.contains(&rumor.id) {
            return;
        }
        // If this is the first rumor heard, go ahead and ask for state.
        if self.clock.time == 0 {
            self.request_state(rumor.initiator).await;
            return;
        }
        self.clock.recieve(rumor.time);
        self.recieved_rumors_ids.insert(rumor.id.clone());
        self.gossip(&rumor);
        self.rumors.push(rumor);
    }

    async fn request_state(&mut self, from_who: String) -> Result<()> {
        let state = self.make_state_request(from_who).await?;
        let their_cluster: Cluster = serde_json::from_str(&state)?;
        let my_new_state = Cluster {
            myself: self.myself.clone(),
            ..their_cluster
        };
        Ok(())
    }

    async fn make_state_request(&mut self, from_who: String) -> Result<String> {
        let body = reqwest::get(from_who.to_owned() + "/state")
            .await?
            .text()
            .await?;
        println!("Requesting state from {}", from_who);
        Ok(body)
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
