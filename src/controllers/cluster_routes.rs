use crate::models::cluster::{Cluster, Node};
use axum::{
    extract::Json,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

pub async fn post_node(Json(payload): Json<Node>, cluster: Arc<Mutex<Cluster>>) -> impl IntoResponse {
    let mut shared_cluster = cluster.lock().unwrap();
    shared_cluster.add_node(payload);
    Response::builder()
        .status(StatusCode::CREATED)
        .body("null".to_string())
        .unwrap()
}
