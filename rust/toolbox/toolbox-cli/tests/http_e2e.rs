use std::{net::SocketAddr, sync::{Arc, Mutex}};

use assert_cmd::prelude::*;
use axum::{extract::{Path, State}, http::StatusCode, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Clone, Default)]
struct AppState { store: Arc<Mutex<Vec<Item>>> }

#[derive(Serialize, Deserialize, Clone)]
struct Item { id: String }

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_http_backend_list_and_get() {
    // Start axum server on a random localhost port
    let app = Router::new()
        .route("/healthz", get(health))
        .route("/v1/items", get(list).post(create))
        .route("/v1/items/:id", get(get_one))
        .with_state(AppState::default());

    let listener = tokio::net::TcpListener::bind(SocketAddr::from(([127,0,0,1], 0))).await.unwrap();
    let addr = listener.local_addr().unwrap();

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async move { let _ = rx.await; })
            .await
            .unwrap();
    });

    // Seed one item via HTTP
    let client = reqwest::Client::new();
    let url_base = format!("http://{}", addr);
    client.post(format!("{}/v1/items", url_base))
        .json(&Item { id: "abc".to_string() })
        .send().await.unwrap()
        .error_for_status().unwrap();

    // Run CLI: list
    let mut cmd = Command::cargo_bin("toolbox-cli").unwrap();
    let assert = cmd.args(["--backend", "http", "--target", &url_base, "list"]).assert().success();
    let out = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert!(v.as_array().unwrap().iter().any(|x| x.get("id").unwrap()=="abc"));

    // Run CLI: get abc
    let mut cmd = Command::cargo_bin("toolbox-cli").unwrap();
    let assert = cmd.args(["--backend", "http", "--target", &url_base, "get", "abc"]).assert().success();
    let out = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let obj: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(obj.get("id").unwrap(), "abc");

    // Shutdown server
    let _ = tx.send(());
    let _ = server.await;
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

