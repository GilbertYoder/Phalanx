mod phalanx;
mod state;
use axum::{
    body::Bytes,
    extract::Path,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use clap::Parser;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use phalanx::Node;
use state::State;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// IP Address
    #[arg(short, long, default_value_t = String::from("0.0.0.0"))]
    ip: String,

    /// Port
    #[arg(short, long, default_value_t = 8000)]
    port: usize,

    /// Name
    #[arg(short, long)]
    name: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let node = Node {
        ip: args.ip,
        port: args.port,
        name: args.name,
        last_heartbeat: 0,
    };

    let app_state = Arc::new(State {
        state: Mutex::new(HashMap::new()),
    });

    let app = Router::new().route(
        "/state/:id",
        get({
            let shared_state = Arc::clone(&app_state);
            move |path| get_state(path, shared_state)
        })
        .post({
            let shared_state = Arc::clone(&app_state);
            move |path: Path<String>, payload: Bytes| post_state(path, shared_state, payload)
        }),
    );

    let listener = tokio::net::TcpListener::bind(node.ip + ":" + &node.port.to_string())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_state(Path(id): Path<String>, app_state: Arc<State>) -> impl IntoResponse {
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

async fn post_state(
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
