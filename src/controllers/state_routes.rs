use crate::models::cluster::Cluster;
use axum::{
    body::Bytes,
    extract::Path,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

pub async fn get_entire_state(app_state: Arc<Mutex<Cluster>>) -> Json<Value> {
    let state = app_state.lock().unwrap();
    Json(json!(&*state))
}

pub async fn get_state(
    Path(id): Path<String>,
    app_state: Arc<Mutex<Cluster>>,
) -> impl IntoResponse {
    match app_state.lock().unwrap().data.get(&id) {
        Some(value) => Response::builder()
            .status(StatusCode::OK)
            .body(value.to_string())
            .unwrap(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("null".to_string())
            .unwrap(),
    }
}

pub async fn post_state(
    Path(id): Path<String>,
    app_state: Arc<Mutex<Cluster>>,
    payload: Bytes,
) -> impl IntoResponse {
    let mut state = app_state.lock().unwrap();
    let value = String::from_utf8(payload.to_vec()).expect("Error w/ payload Bytes.");
    state.data.set(id, value.clone());
    Response::builder()
        .status(StatusCode::CREATED)
        .body(value)
        .unwrap()
}
