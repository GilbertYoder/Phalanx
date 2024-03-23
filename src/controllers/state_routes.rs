use crate::models::state::State;
use axum::{
    body::Bytes,
    extract::Path,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use std::sync::Arc;

pub async fn get_state(Path(id): Path<String>, app_state: Arc<State>) -> impl IntoResponse {
    match app_state.get(&id) {
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
    app_state: Arc<State>,
    payload: Bytes,
) -> impl IntoResponse {
    let value = String::from_utf8(payload.to_vec()).expect("Error w/ payload Bytes.");
    app_state.set(id, value.clone());
    Response::builder()
        .status(StatusCode::CREATED)
        .body(value)
        .unwrap()
}
