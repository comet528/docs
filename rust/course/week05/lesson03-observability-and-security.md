# Lesson 03 — Observability and Security (Trace, CORS, Timeouts)

Why this matters
- Good logs make on-call life easier. Sensible cross‑origin and timeout policies protect your service. `tower-http` offers ready‑made layers.

Concepts covered
- Request/response tracing with `TraceLayer` and request IDs.
- CORS, timeouts, and limits using `tower`/`tower-http`.

Cargo.toml (extra deps)
```toml
[dependencies]
tower = "0.5"
tower-http = { version = "0.5", features = ["trace", "cors", "request-id"] }
uuid = { version = "1", features = ["v4"] }
```

Trace and request IDs
```rust
use tower_http::trace::TraceLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};

let app = Router::new()
    .route("/healthz", get(health))
    // ... routes ...
    .layer(TraceLayer::new_for_http())
    .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
    .layer(PropagateRequestIdLayer::x_request_id());
```

CORS
```rust
use tower_http::cors::{Any, CorsLayer};

let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
    .allow_headers(Any);
let app = app.layer(cors);
```

Timeouts
```rust
use tower::timeout::TimeoutLayer;
use std::time::Duration;
let app = app.layer(TimeoutLayer::new(Duration::from_secs(10)));
```

Lab
- Add the layers above to your Axum service; ensure logs include `x-request-id` and measure request duration.

Next
- Containerize with scratch/distroless and produce a release.

