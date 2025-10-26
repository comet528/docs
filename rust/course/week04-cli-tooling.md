# Week 04 — Production CLI: Config, Logging, Packaging

Objectives
- Build a robust CLI with subcommands using `clap` derive macros.
- Load configuration from files and env; structure logs with `tracing`.
- Package for distribution and cross-compile.

Day 1 — CLI Structure
- `clap` derive: subcommands, flags, env var fallback.
- Command layout: `cmd mod` per subcommand; dependency inversion into core library.
- Lab: `toolbox-cli` with `list` and `get <id>` subcommands.

Day 2 — Configuration & Logging
- Config search order: CLI args > env > file (`config.toml`/`yaml`).
- Use `serde` for config; merge layers; validate.
- Logging: `tracing` + `tracing-subscriber` (fmt/json), levels via `RUST_LOG`.
- Lab: add `--config` flag; implement `RUST_LOG=info` and structured fields.

Day 3 — HTTP Clients & Retries
- `reqwest` client, timeouts, backoff; `tower` layer stacks (optional).
- Lab: add `--backend http --url <...>` and `--backend file --path <...>`; reuse Week 3 traits.

Day 4 — Packaging & Cross-Compile
- Release builds: `cargo build --release`, `strip` binaries.
- Static builds (musl): `rustup target add x86_64-unknown-linux-musl` and set `--target`.
- Cross-tool: `cross` (optional) for reproducible builds in Docker.
- Create release artifacts: tar/zip with README and licenses; checksums.

Checkpoint — CLI Release
- Produce `toolbox` binary for host OS and Linux-musl.
- Provide `--help` with subcommands; log to stderr; exit codes map to error variants.
- Deliverable: `dist/` folder with artifacts and a short README.

Commands
- `cargo install cargo-binstall` (optional for quick binary installs).
- `strip target/release/toolbox` and verify size; consider `upx` (optional).

