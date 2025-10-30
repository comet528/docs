# restsvc — Minimal Axum REST Service

Endpoints
- `GET /healthz` → `{ "status": "ok" }`
- `GET /v1/items` → list items (in-memory store)
- `POST /v1/items` → create item `{ id: string }`
- `GET /v1/items/:id` → retrieve by id

Run locally
- From `rust/restsvc`: `RUST_LOG=info cargo run`
- Health: `curl localhost:8080/healthz`
- Create: `curl -XPOST localhost:8080/v1/items -H 'content-type: application/json' -d '{"id":"abc"}'`
- List: `curl localhost:8080/v1/items`
- Get: `curl localhost:8080/v1/items/abc`

Docker
- MUSL scratch: `docker build -f Dockerfile.musl -t restsvc:musl . && docker run --rm -p 8080:8080 -e PORT=8080 restsvc:musl`
- Distroless: `docker build -f Dockerfile.distroless -t restsvc:distroless . && docker run --rm -p 8080:8080 -e PORT=8080 restsvc:distroless`

Link with toolbox CLI
- Start this service, then from `rust/toolbox` run:
  - `cargo run -p toolbox-cli -- --backend http --target http://localhost:8080 -- list`
  - `cargo run -p toolbox-cli -- --backend http --target http://localhost:8080 -- get abc`

CI and Releases
- See `rust/CI.md` for how the GitHub Actions CI and release pipelines build binaries and push container images to GHCR.
