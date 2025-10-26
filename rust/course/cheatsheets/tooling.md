# Rust Tooling Cheatsheet

Install/Update
- Install: `curl https://sh.rustup.rs -sSf | sh`
- Update: `rustup update`
- Toolchain pin: `rustup override set stable` (per repo)

Cargo
- New bin/lib: `cargo new <name>`, `cargo new --lib <name>`
- Build/run/test: `cargo build`, `cargo run -- <args>`, `cargo test`
- Release: `cargo build --release`
- Add dep: edit `Cargo.toml` or `cargo add <crate>` (with `cargo-edit`)
- Workspaces: root `Cargo.toml` with `members = [ ... ]`

Quality
- Format: `cargo fmt --all`
- Lint: `cargo clippy --all-targets -- -D warnings`
- Audit (optional): `cargo install cargo-audit && cargo audit`

Cross-Compile
- Add target: `rustup target add x86_64-unknown-linux-musl`
- Build: `cargo build --release --target x86_64-unknown-linux-musl`

Async/Web
- Tokio: `tokio = { version = "1", features = ["full"] }`
- Axum: `axum = "0.7"`, `tower = "0.5"`

