use crate::models::cluster::{Cluster, Node, Rumor, RumorMethod};
use axum::{
    extract::{Extension, Json},
    http::{Response, StatusCode},
    response::IntoResponse,
};
use axum_macros::debug_handler;
use std::sync::{Arc, Mutex};

fn add_node(cluster: Arc<Mutex<Cluster>>, node: Node) {
    let mut shared_cluster = cluster.lock().unwrap();
    shared_cluster.add_node(node);
}

fn get_rumor(cluster: Arc<Mutex<Cluster>>) -> Rumor {
    let shared_cluster = cluster.lock().unwrap();
    Rumor::new(
        RumorMethod::SET,
        "Hi".to_string(),
        shared_cluster.clock.time.clone(),
        shared_cluster.myself.ip.to_string().clone()
            + ":"
            + &shared_cluster.myself.port.to_string().clone(),
    )
}

fn get_nodes(cluster: Arc<Mutex<Cluster>>) -> Vec<Node> {
    let shared_cluster = cluster.lock().unwrap();
    shared_cluster.nodes.clone()
}

#[debug_handler]
pub async fn post_node(
    Extension(shared_state): Extension<Arc<Mutex<Cluster>>>,
    Json(payload): Json<Node>,
) -> impl IntoResponse {
    add_node(shared_state.clone(), payload);
    let rumor = get_rumor(shared_state.clone());
    let nodes = get_nodes(shared_state.clone());
    Cluster::gossip(nodes, rumor).await;
    return Response::builder()
        .status(StatusCode::CREATED)
        .body("ok".to_string())
        .unwrap();
}

#[debug_handler]
pub async fn gossip(
    cluster: Extension<Arc<Mutex<Cluster>>>,
    Json(payload): Json<Rumor>,
) -> impl IntoResponse {
    let mut shared_cluster = cluster.lock().unwrap();
    shared_cluster.recieve_rumor(payload);
    Response::builder()
        .status(StatusCode::CREATED)
        .body("ok".to_string())
        .unwrap()
}
