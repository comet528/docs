# Toolbox — Multi-backend CLI (Workspace)

Overview
- `toolbox-core` — library with the `Backend` trait and File/HTTP implementations.
- `toolbox-cli` — binary CLI using `clap`, `tokio`, `tracing`.

Prerequisites
- Rust toolchain: `rustup`, `cargo`, `rustc`.
- Optional for HTTP E2E tests: network loopback (localhost), no external network required.

Build and Run
- From workspace root: `rust/toolbox`
- Build: `cargo build`
- List (file backend):
  - `cargo run -p toolbox-cli -- --backend file --target ./ -- list`
- Get (HTTP backend against local REST service):
  - `cargo run -p toolbox-cli -- --backend http --target http://localhost:8080 -- get 123`

Flags and Env
- `--backend` file|http (default: file; can use `TOOLBOX_BACKEND`)
- `--target` path (file) or base URL (http) (can use `TOOLBOX_TARGET`)
- Logs via `RUST_LOG=info` or `RUST_LOG=debug`

JSON Contract
- HTTP backend expects:
  - `GET /v1/items` → `[{ "id": "..." }]`
  - `GET /v1/items/:id` → `{ "id": "..." }`

Tests
- Unit tests (core): `cargo test -p toolbox-core`
- Integration E2E test (CLI + in‑process Axum server): `cargo test -p toolbox-cli -- --nocapture`
  - Spawns a temporary Axum server on a random localhost port, seeds data via HTTP, runs the CLI with `--backend http`, asserts JSON output.

Packaging
- Release build: `cargo build -p toolbox-cli --release`
- Static MUSL (Linux): `rustup target add x86_64-unknown-linux-musl && cargo build -p toolbox-cli --release --target x86_64-unknown-linux-musl`

Link to REST service
- A sample Axum service lives at `rust/restsvc`. Start it (`RUST_LOG=info cargo run`) and point the CLI `--target` to `http://localhost:8080`.

CI and Releases
- See `rust/CI.md` for GitHub Actions setup (CI, releases, and container images).
