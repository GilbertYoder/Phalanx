use crate::models::state::Data;
use crate::utils::lamport_clock::LamportClock;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize, Clone, Serialize)]
pub struct Node {
    pub ip: String,
    pub port: usize,
    pub last_heartbeat: usize,
}

#[derive(Deserialize, Serialize)]
pub struct Cluster {
    pub myself: Node,
    pub nodes: Vec<Node>,
    pub clock: LamportClock,
    pub rumors: Vec<Rumor>,
    pub recieved_rumors_ids: HashSet<String>,
    pub data: Data,
}

impl Cluster {
    pub async fn gossip(nodes: Vec<Node>, rumor: Rumor) {
        for node in nodes.iter() {
            println!("Gossiping to {}", node.ip);
            let _ = Cluster::gossip_to_node(node, &rumor).await.expect("Problem gossiping to node.");
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
        self.clock.increment();
    }

    pub async fn recieve_rumor(&mut self, rumor: Rumor) {
        // Don't apply the same rumor twice.
        if self.recieved_rumors_ids.contains(&rumor.id) {
            return;
        }
        // If this is the first rumor heard, go ahead and ask for state.
        if self.clock.time == 0 {
            let _ = self.request_state(rumor.initiator).await;
            return;
        }
        self.clock.recieve(rumor.time);
        self.recieved_rumors_ids.insert(rumor.id.clone());
        Cluster::gossip(self.nodes.clone(), rumor.clone()).await;
        self.rumors.push(rumor);
    }

    async fn request_state(&mut self, from_who: String) -> Result<()> {
        let state = self.make_state_request(from_who).await?;
        let their_cluster: Cluster = serde_json::from_str(&state)?;
        self.data = their_cluster.data;
        self.nodes = their_cluster.nodes;
        self.rumors = their_cluster.rumors;
        self.clock = their_cluster.clock;
        self.recieved_rumors_ids = their_cluster.recieved_rumors_ids;
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

    pub async fn gossip_to_node(node: &Node, rumor: &Rumor) -> Result<()> {
        let client = reqwest::Client::new();
        client
            .post("http://".to_owned() + &node.ip.to_string() + ":" + &node.port.to_string() + "/state")
            .json(&rumor)
            .send()
            .await?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub enum RumorMethod {
    GET,
    SET,
    DELETE,
    APPEND,
}

#[derive(Deserialize, Serialize, Clone)]
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
