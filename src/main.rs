mod controllers;
mod models;

use axum::{body::Bytes, extract::Path, routing::get, Router};
use clap::Parser;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use controllers::state_routes;
use models::node::Node;
use models::state::State;

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
            move |path| state_routes::get_state(path, shared_state)
        })
        .post({
            let shared_state = Arc::clone(&app_state);
            move |path: Path<String>, payload: Bytes| {
                state_routes::post_state(path, shared_state, payload)
            }
        }),
    );

    let listener = tokio::net::TcpListener::bind(node.ip + ":" + &node.port.to_string())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
