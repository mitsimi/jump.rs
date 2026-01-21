mod api;
mod arp;
mod error;
mod models;
mod storage;
mod wol;

use crate::storage::{SharedStorage, load_storage};
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let storage: SharedStorage = load_storage();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .fallback_service(ServeDir::new("static/dist"))
        .route("/api/devices", get(api::get_devices))
        .route("/api/devices", post(api::create_device))
        .route("/api/devices/{id}", put(api::update_device))
        .route("/api/devices/{id}", delete(api::delete_device))
        .route("/api/devices/{id}/wake", post(api::wake_device))
        .route("/api/arp-lookup", post(api::arp_lookup))
        .with_state(storage)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.expect("Server failed");
}
