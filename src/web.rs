use crate::string_to_field;
use crate::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{instrument, trace};
use warp::Filter;

const DEFAULT_ADDR: &'static str = "0.0.0.0";
const DEFAULT_PORT: u16 = 3000;

#[instrument(skip(shared_state))]
pub async fn handler(shared_state: Arc<RwLock<State>>) {
    let addr = std::env::var("KOUNT_ADDR")
        .unwrap_or(DEFAULT_ADDR.to_string())
        .parse()
        .expect("failed to parse address");
    let port = std::env::var("KOUNT_PORT")
        .unwrap_or(DEFAULT_PORT.to_string())
        .parse()
        .expect("failed to parse port");
    let socket_addr = std::net::SocketAddr::new(addr, port);

    let shared_state = warp::any().map(move || Arc::clone(&shared_state));
    let json = warp::path("json").and(shared_state.clone()).and_then(json);
    let field = warp::path("field")
        .and(shared_state.clone())
        .and(warp::path::param())
        .and_then(field);

    let routes = json.or(field);
    warp::serve(routes).run(socket_addr).await;
}

#[instrument(skip(shared_state))]
async fn json(shared_state: Arc<RwLock<State>>) -> Result<impl warp::Reply, warp::Rejection> {
    trace!("serving json");
    let state = shared_state.read().await;
    Ok(warp::reply::json(&state.clone()))
}

#[instrument(skip(shared_state))]
async fn field(
    shared_state: Arc<RwLock<State>>,
    field: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    trace!("serving field");
    let state = shared_state.read().await;

    match string_to_field!(field.as_str(), state) {
        None => Err(warp::reject::not_found()),
        Some(value) => Ok(warp::reply::html(value.to_string())),
    }
}
