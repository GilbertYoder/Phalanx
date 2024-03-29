use crate::models::state::Data;
use crate::utils::lamport_clock::LamportClock;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize, Clone, Serialize)]
pub struct Node {
    pub ip: String,
    pub port: usize,
    pub last_heartbeat: usize,
}

#[derive(Deserialize, Serialize)]
pub struct SerializableCluster {
    pub myself: Node,
    pub nodes: Vec<Node>,
    pub clock: LamportClock,
    pub rumors: Vec<Rumor>,
    pub recieved_rumors_ids: HashSet<String>,
    pub data: Data,
}

#[derive(Clone)]
pub struct Cluster {
    pub myself: Arc<Mutex<Node>>,
    pub nodes: Arc<Mutex<Vec<Node>>>,
    pub clock: Arc<Mutex<LamportClock>>,
    pub rumors: Arc<Mutex<Vec<Rumor>>>,
    pub recieved_rumors_ids: Arc<Mutex<HashSet<String>>>,
    pub data: Arc<Mutex<Data>>,
}

impl Cluster {
    pub fn get_serializable(&self) -> SerializableCluster {
        SerializableCluster {
            myself: self.myself.lock().unwrap().clone(),
            nodes: self.nodes.lock().unwrap().clone(),
            clock: self.clock.lock().unwrap().clone(),
            rumors: self.rumors.lock().unwrap().clone(),
            recieved_rumors_ids: self.recieved_rumors_ids.lock().unwrap().clone(),
            data: self.data.lock().unwrap().clone(),
        }
    }

    fn get_nodes(&self) -> Vec<Node> {
        self.nodes.lock().unwrap().clone()
    }

    pub async fn gossip(&mut self, rumor: Rumor) {
        for node in self.get_nodes() {
            println!("Gossiping to {}", node.ip);
            let _ = Cluster::gossip_to_node(&node, &rumor)
                .await
                .expect("Problem gossiping to node.");
        }
    }

    pub fn add_node(&mut self, node: Node) {
        let mut nodes = self.nodes.lock().unwrap();
        nodes.push(node);
        let mut clock = self.clock.lock().unwrap();
        clock.increment();
    }

    fn update_self(&mut self, new_state: SerializableCluster) {
        let mut data = self.data.lock().unwrap();
        *data = new_state.data;
        let mut nodes = self.nodes.lock().unwrap();
        *nodes = new_state.nodes;
        let mut rumors = self.rumors.lock().unwrap();
        *rumors = new_state.rumors;
        let mut clock = self.clock.lock().unwrap();
        *clock = new_state.clock;
        let mut received_rumors_ids = self.recieved_rumors_ids.lock().unwrap();
        *received_rumors_ids = new_state.recieved_rumors_ids;
    }

    fn is_first_rumor(&self) -> bool {
        let clock = self.clock.lock().unwrap();
        clock.time == 0
    }

    fn has_heard_rumor(&self, rumor_id: &str) -> bool {
        let received_rumors = self.recieved_rumors_ids.lock().unwrap();
        received_rumors.contains(rumor_id)
    }

    pub async fn recieve_rumor(&mut self, rumor: Rumor) {
        // Don't apply the same rumor twice.
        if self.has_heard_rumor(&rumor.id) {
            return;
        }
        // If this is the first rumor heard, go ahead and ask for state.
        if self.is_first_rumor() {
            let state = self.make_state_request(rumor.initiator).await;
            let their_cluster: SerializableCluster = serde_json::from_str(&state).expect("Oops");
            self.update_self(their_cluster);
            return;
        }
        {
            let mut clock = self.clock.lock().unwrap();
            clock.recieve(rumor.time);
            let mut received_rumors = self.recieved_rumors_ids.lock().unwrap();
            received_rumors.insert(rumor.id.clone());
        }
        self.gossip(rumor.clone()).await;
        let mut rumors = self.rumors.lock().unwrap();
        rumors.push(rumor);
    }

    async fn make_state_request(&mut self, from_who: String) -> String {
        let body = reqwest::get(from_who.to_owned() + "/state")
            .await
            .expect("oops")
            .text()
            .await
            .expect("oops again");
        println!("Requesting state from {}", from_who);
        body
    }

    pub async fn gossip_to_node(node: &Node, rumor: &Rumor) -> Result<()> {
        let client = reqwest::Client::new();
        client
            .post(
                "http://".to_owned()
                    + &node.ip.to_string()
                    + ":"
                    + &node.port.to_string()
                    + "/gossip",
            )
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
