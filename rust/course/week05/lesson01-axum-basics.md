# Lesson 01 — Axum Basics: Router, State, and JSON

Why this matters
- Axum is a minimal, ergonomic web framework atop `hyper` and `tower`. You can build clear, testable APIs with extractors and typed handlers.

Concepts covered
- `Router`, routes, handlers returning `impl IntoResponse`.
- JSON extractors, shared state via `with_state` and `Arc<Mutex<...>>`.
- Graceful shutdown.

Cargo.toml (excerpt)
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "signal"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }
``` 

Worked example — minimal items API
```rust
use axum::{routing::{get, post}, extract::{Path, State}, Json, Router};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::{Arc, Mutex}};
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
    let addr = SocketAddr::from(([0,0,0,0], port));
    info!(%addr, "listening");
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() { let _ = tokio::signal::ctrl_c().await; }

async fn health() -> Json<serde_json::Value> { Json(serde_json::json!({"status":"ok"})) }

async fn list(State(state): State<AppState>) -> Json<Vec<Item>> { Json(state.store.lock().unwrap().clone()) }

async fn create(State(state): State<AppState>, Json(item): Json<Item>) -> (axum::http::StatusCode, Json<Item>) {
    state.store.lock().unwrap().push(item.clone());
    (axum::http::StatusCode::CREATED, Json(item))
}

async fn get_one(State(state): State<AppState>, Path(id): Path<String>) -> Result<Json<Item>, axum::http::StatusCode> {
    let items = state.store.lock().unwrap();
    items.iter().find(|it| it.id == id).cloned().map(Json).ok_or(axum::http::StatusCode::NOT_FOUND)
}
```

Lab
- Add `DELETE /v1/items/:id` and return 204 on success, 404 otherwise.

Next
- Add error types that implement `IntoResponse` and write async tests for handlers.

