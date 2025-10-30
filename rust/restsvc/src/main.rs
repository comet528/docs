use std::{net::SocketAddr, sync::{Arc, Mutex}};

use axum::{extract::{Path, State}, http::StatusCode, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Default)]
struct AppState { store: Arc<Mutex<Vec<Item>>> }

#[derive(Serialize, Deserialize, Clone)]
struct Item { id: String }

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let app = Router::new()
        .route("/healthz", get(health))
        .route("/v1/items", get(list).post(create))
        .route("/v1/items/:id", get(get_one))
        .with_state(AppState::default());

    let port: u16 = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!(%addr, "listening");
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() { let _ = tokio::signal::ctrl_c().await; }

async fn health() -> Json<serde_json::Value> { Json(serde_json::json!({"status":"ok"})) }

async fn list(State(state): State<AppState>) -> Json<Vec<Item>> { Json(state.store.lock().unwrap().clone()) }

async fn create(State(state): State<AppState>, Json(item): Json<Item>) -> (StatusCode, Json<Item>) {
    state.store.lock().unwrap().push(item.clone());
    (StatusCode::CREATED, Json(item))
}

async fn get_one(State(state): State<AppState>, Path(id): Path<String>) -> Result<Json<Item>, StatusCode> {
    let items = state.store.lock().unwrap();
    items.iter().find(|it| it.id == id).cloned().map(Json).ok_or(StatusCode::NOT_FOUND)
}

