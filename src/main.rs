mod controllers;
mod models;

use axum::{body::Bytes, extract::Path, routing::get, routing::post, Json, Router};
use clap::Parser;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use controllers::{cluster_routes, state_routes};
use models::cluster::{Cluster, Node};
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
        nodes: vec![],
        myself,
    }));

    let app_state = Arc::new(Mutex::new(State {
        state: HashMap::new(),
    }));

    let app = Router::new()
        .route(
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
        )
        .route(
            "/nodes",
            post({
                let shared_cluster = Arc::clone(&cluster);
                move |payload| cluster_routes::post_node(payload, shared_cluster)
            }),
        );

    let listener = tokio::net::TcpListener::bind(
        ip.to_owned() + ":" + &port.to_string(),
    )
    .await
    .unwrap();
    axum::serve(listener, app).await.unwrap();
}
