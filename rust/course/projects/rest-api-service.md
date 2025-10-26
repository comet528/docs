# Project — REST Service with Axum

Goal
- Build a small JSON REST service with `axum`, instrumented with `tracing`, packaged as a minimal container.

API
- `GET /healthz` → 200 OK JSON `{ status: "ok" }`.
- `GET /v1/items` → list items.
- `POST /v1/items` → create item, validate payload; returns 201 with item.
- `GET /v1/items/:id` → retrieve by id; 404 if missing.

Non-Functional
- Graceful shutdown on SIGTERM.
- Request/response logging with correlation IDs.
- Unit and integration tests for routes.
- Container image runs as non-root and exposes only the binary.

Acceptance Criteria
- `docker run -p 8080:8080 -e PORT=8080` serves API; `GET /healthz` returns `ok`.
- Image size target: scratch < 15MB or distroless < 40MB.
- `cargo clippy` passes with `-D warnings`; `cargo test` passes.

Stretch
- Add `/metrics` with Prometheus exporter; Helm chart or simple `k8s` YAML.

