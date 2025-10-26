# Week 05 — REST with Axum + Distroless Containers

Objectives
- Build a small REST API with `axum` and `tokio`.
- Add observability (logging, metrics), graceful shutdown, and tests.
- Package into minimal/static images (scratch or distroless) and run.

Day 1 — Axum Basics
- New project: `cargo new restsvc && cd restsvc`.
- Deps: `axum`, `tokio`, `serde`, `serde_json`, `tracing`, `tracing-subscriber`.
- Routes: `GET /healthz`, `GET /v1/items`, `POST /v1/items` (in-memory store).
- State: `Arc<Mutex<Store>>` injected via `axum::Extension` or `with_state`.

Day 2 — Errors, Extractors, Tests
- Extractors for path/query/json; custom error type implementing `IntoResponse`.
- Tests: `#[tokio::test]` using `axum::Router` + `hyper`/`reqwest`.
- Graceful shutdown with `tokio::signal`.

Day 3 — Observability & Security
- `tracing` with request IDs; middleware with `tower` (`TraceLayer`).
- CORS (if needed), sensible timeouts; input validation.
- Optional metrics: `prometheus` + `/metrics` endpoint.

Day 4 — Containers (Static/Distroless)
- Build static MUSL binary: `cargo build --release --target x86_64-unknown-linux-musl`.
- Multi-stage Dockerfiles: scratch static and distroless (Debian 12).
- Run as non-root, set `PORT`, expose minimal files.

Checkpoint — Service + Image
- Launch locally: `RUST_LOG=info cargo run` and via Docker image.
- Deliverable: image < 15MB (scratch) or < 40MB (distroless) with health endpoint.

Templates
- See: `rust/course/templates/Dockerfile.musl-scratch` and `rust/course/templates/Dockerfile.distroless`.

Commands
- `docker build -f rust/course/templates/Dockerfile.musl-scratch -t restsvc:musl .`
- `docker run --rm -p 8080:8080 -e PORT=8080 restsvc:musl`

