mod controllers;
mod models;
mod utils;

use axum::{
    body::Bytes,
    extract::Path,
    middleware::AddExtension,
    routing::{get, post},
    Extension, Json, Router,
};
use clap::Parser;
use serde_json::{json, Value};
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};
use utils::lamport_clock::LamportClock;

use controllers::{cluster_routes, state_routes};
use models::cluster::{Cluster, Node};
use models::state::Data;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// IP Address
    #[arg(short, long, default_value_t = String::from("0.0.0.0"))]
    ip: String,

    /// Port
    #[arg(short, long, default_value_t = 8000)]
    port: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let ip = args.ip;
    let port = args.port;

    let myself = Node {
        ip: ip.clone(),
        port,
        last_heartbeat: 0,
    };

    let cluster = Arc::new(Mutex::new(Cluster {
        nodes: vec![myself.clone()],
        myself,
        clock: LamportClock::new(),
        data: Data {
            state: HashMap::new(),
        },
        rumors: vec![],
        recieved_rumors_ids: HashSet::new(),
    }));

    let shared_state = Arc::clone(&cluster);

    let app = Router::new()
        .route("/state", get(state_routes::get_entire_state))
        .route(
            "/state/:id",
            get(state_routes::get_state).post(state_routes::post_state),
        )
        .route("/nodes", post(cluster_routes::post_node))
        .route("/gossip", post(cluster_routes::gossip))
        .layer(Extension(shared_state));

    let listener = tokio::net::TcpListener::bind(ip.to_owned() + ":" + &port.to_string())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
