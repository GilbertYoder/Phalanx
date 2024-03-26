use crate::models::cluster::{Cluster, Node, Rumor, RumorMethod};
use axum::{
    extract::{Extension, Json},
    http::{Response, StatusCode},
    response::IntoResponse,
};
use axum_macros::debug_handler;
use std::sync::{Arc, Mutex};

#[debug_handler]
pub async fn post_node(
    Extension(shared_state): Extension<Arc<Mutex<Cluster>>>,
    Json(payload): Json<Node>,
) -> impl IntoResponse {
    {
        let mut shared_cluster = shared_state.lock().unwrap();
        shared_cluster.add_node(payload);
        let rumor = Rumor::new(
            RumorMethod::SET,
            "Hi".to_string(),
            shared_cluster.clock.time,
            shared_cluster.myself.ip.to_string() + ":" + &shared_cluster.myself.port.to_string(),
        );
    }
    match shared_cluster.gossip(rumor).await {
        Ok(_) => {
            return Response::builder()
                .status(StatusCode::CREATED)
                .body("ok".to_string())
                .unwrap()
        }
        Err(e) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Error processing request: {}", e))
                .unwrap()
        }
    }
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
