use crate::models::cluster::{Cluster, Node, Rumor, RumorMethod};
use axum::{
    body::Bytes,
    extract::{Extension, Json, Path},
    http::{Response, StatusCode},
    response::IntoResponse,
};
use axum_macros::debug_handler;

fn get_rumor(cluster: &Cluster) -> Rumor {
    let myself = cluster.myself.lock().unwrap();
    Rumor::new(
        RumorMethod::SET,
        "Hi".to_string(),
        cluster.clock.lock().unwrap().time.clone(),
        myself.ip.to_string().clone()
            + ":"
            + &myself.port.to_string().clone(),
    )
}

#[debug_handler]
pub async fn post_node(
    Extension(mut cluster): Extension<Cluster>,
    Json(payload): Json<Node>,
) -> impl IntoResponse {
    cluster.add_node(payload);
    let rumor = get_rumor(&cluster);
    cluster.gossip(rumor).await;
    return Response::builder()
        .status(StatusCode::CREATED)
        .body("ok".to_string())
        .unwrap();
}

#[debug_handler]
pub async fn gossip(
    Extension(mut cluster): Extension<Cluster>,
    Json(payload): Json<Rumor>,
) -> impl IntoResponse {
    println!("Recieved rumor.");
    cluster.recieve_rumor(payload);
    Response::builder()
        .status(StatusCode::CREATED)
        .body("ok".to_string())
        .unwrap()
}

pub async fn get_state(
    Path(id): Path<String>,
    Extension(cluster): Extension<Cluster>,
) -> impl IntoResponse {
    match cluster.data.lock().unwrap().get(&id) {
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
    Extension(cluster): Extension<Cluster>,
    payload: Bytes,
) -> impl IntoResponse {
    let value = String::from_utf8(payload.to_vec()).expect("Error w/ payload Bytes.");
    cluster.data.lock().unwrap().set(id, value.clone());
    Response::builder()
        .status(StatusCode::CREATED)
        .body(value)
        .unwrap()
}
