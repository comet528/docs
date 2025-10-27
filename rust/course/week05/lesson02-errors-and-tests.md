# Lesson 02 — Errors and Async Tests in Axum

Why this matters
- Clear HTTP error mapping and robust tests keep APIs predictable. Implement `IntoResponse` for your errors and test handlers without a running server.

Concepts covered
- Custom error types implementing `IntoResponse`.
- Testing routers with `tower::ServiceExt` and `axum::http`.

Cargo.toml (extra deps)
```toml
[dependencies]
tower = "0.5"   # for ServiceExt in tests
thiserror = "1"
``` 

Worked example — error type and mapping
```rust
use axum::{http::StatusCode, response::{IntoResponse, Response}};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("not found")] NotFound,
    #[error("bad request: {0}")] BadRequest(String),
    #[error(transparent)] Other(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found").into_response(),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            ApiError::Other(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}
```

Handler using `ApiError`
```rust
use axum::{extract::Path, Json};

async fn get_one(Path(id): Path<String>) -> Result<Json<Item>, ApiError> {
    if id.is_empty() { return Err(ApiError::BadRequest("empty id".into())); }
    // look up from state (omitted)
    Err(ApiError::NotFound)
}
```

Testing the router (no server)
```rust
#[tokio::test]
async fn not_found_returns_404() {
    use axum::{http::{Request, StatusCode}, routing::get, Router};
    use tower::ServiceExt; // for `oneshot`

    let app = Router::new().route("/v1/items/:id", get(get_one));
    let res = app.oneshot(Request::builder().uri("/v1/items/does-not-exist").body(axum::body::Body::empty()).unwrap()).await.unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
```

Lab
- Write tests for `POST /v1/items` happy path and validation error.

Next
- Add observability, request tracing, CORS, and timeouts.

