# Rust Projects — Toolbox CLI and REST Service

This folder contains two runnable Rust projects and a course:
- Toolbox CLI workspace: `rust/toolbox`
- Axum REST service: `rust/restsvc`
- Course materials: `rust/course`

Toolbox CLI
- Build: `cd rust/toolbox && cargo build`
- List files: `cargo run -p toolbox-cli -- --backend file --target ./ -- list`
- Get via HTTP (with service running): `cargo run -p toolbox-cli -- --backend http --target http://localhost:8080 -- get abc`
- Tests: `cargo test -p toolbox-cli` (includes an E2E test that starts an in-process Axum server)
- Details: `rust/toolbox/README.md`

REST Service (Axum)
- Run: `cd rust/restsvc && RUST_LOG=info cargo run`
- Health: `curl localhost:8080/healthz`
- Docker (scratch): `docker build -f Dockerfile.musl -t restsvc:musl . && docker run --rm -p 8080:8080 -e PORT=8080 restsvc:musl`
- Details: `rust/restsvc/README.md`

Course
- Start at: `rust/course/README.md`

