mod phalanx;
use axum::{
    body::Bytes,
    extract::Path,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use phalanx::Phalanx;

#[tokio::main]
async fn main() {
    let phalanx = Arc::new(Phalanx {
        state: Mutex::new(HashMap::new()),
    });

    // build our application with a single route
    let app = Router::new().route(
        "/state/:id",
        get({
            let shared_state = Arc::clone(&phalanx);
            move |path| get_state(path, shared_state)
        })
        .post({
            let shared_state = Arc::clone(&phalanx);
            move |path: Path<String>, payload: Bytes| post_state(path, shared_state, payload)
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8004").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_state(Path(id): Path<String>, phalanx: Arc<Phalanx>) -> impl IntoResponse {
    let state = phalanx.state.lock().unwrap();
    match state.get(&id) {
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

async fn post_state(
    Path(id): Path<String>,
    phalanx: Arc<Phalanx>,
    payload: Bytes,
) -> impl IntoResponse {
    let mut state = phalanx.state.lock().unwrap();
    state.insert(
        id,
        String::from_utf8(payload.to_vec()).expect("Bad boys bad boys.."),
    );
    "ok"
}
