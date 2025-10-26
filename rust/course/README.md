# Rust 5-Week Intensive (For Go/Systems Engineers)

Audience: Intermediate Go/Java/TypeScript engineers who build CLIs, tools, and REST services.

Outcomes
- Build production-grade CLIs and small REST services in Rust.
- Contrast Rust and Go across syntax, memory, errors, and concurrency.
- Package, cross-compile, and ship static/distroless containers.

Prerequisites
- Linux/macOS or WSL2, container runtime (Docker/Podman), `git`.
- Install toolchain: `curl https://sh.rustup.rs -sSf | sh`
- Verify: `rustc --version`, `cargo --version`.
- Editor: rust-analyzer (VS Code, JetBrains, etc.).
- Targets (for static builds): `rustup target add x86_64-unknown-linux-musl`.

Core Tools & Crates
- Tooling: `rustup`, `cargo`, `rust-analyzer`, `cargo fmt`, `cargo clippy`.
- CLI: `clap` (derive), logging `tracing`, config via `serde` + `toml`/`yaml`.
- HTTP & JSON: `reqwest`, `serde`, `serde_json`.
- Async: `tokio`.
- Web: `axum` (+ `tower` middlewares).
- Errors: `thiserror` (library), `anyhow` (binary), `eyre` optional.

Structure
- Week 01: Rust basics, Cargo, tests, first CLI.
- Week 02: Ownership, borrowing, lifetimes, errors, modules.
- Week 03: Traits/generics, iterators, concurrency, async.
- Week 04: Production CLI (subcommands, config, logging, packaging).
- Week 05: REST with Axum + distroless containers + release.

Navigation
- Week 1: `rust/course/week01-basics.md`
- Week 2: `rust/course/week02-ownership-borrowing.md`
- Week 3: `rust/course/week03-traits-async-concurrency.md`
- Week 4: `rust/course/week04-cli-tooling.md`
- Week 5: `rust/course/week05-web-rest-and-containers.md`
- Projects: `rust/course/projects/`
- Cheatsheets: `rust/course/cheatsheets/`
- Templates: `rust/course/templates/`

Time Commitment
- 6–8 hours/week. Each week includes short readings, guided labs, and a checkpoint.

Verification
- Run formatting/lint: `cargo fmt --all`, `cargo clippy --all-targets -- -D warnings`.
- Tests/examples: `cargo test`, `cargo run --bin <name>`.

Tip (Go → Rust mindset)
- Prefer `Result<T, E>` + `?` over panic; keep functions generic with trait bounds; push shared ownership to edges (`Arc`/`Mutex`), keep core logic single-threaded and explicit.

