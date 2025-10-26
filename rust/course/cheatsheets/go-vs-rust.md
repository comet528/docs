# Go ↔ Rust Cheatsheet

Build & Tooling
- Go: `go build`, modules, `go fmt`, `go test`.
- Rust: `cargo build`, `Cargo.toml`, `cargo fmt`, `cargo test`, `cargo clippy`.

Packages vs Crates/Modules
- Go `package` tree vs Rust `crate` (binary/lib) and `mod` files. `pub` controls visibility.

Interfaces vs Traits
- Go interfaces are structural and satisfied implicitly. Dynamic dispatch by default.
- Rust traits are nominal; implemented explicitly. Static dispatch by default (`impl Trait`), dynamic via `dyn Trait`.

Errors
- Go: `error` interface; patterns like `if err != nil`.
- Rust: `Result<T, E>`, `?` operator; libraries expose concrete `E`, binaries often use `anyhow::Result`.

Memory & Concurrency
- Go: GC, goroutines, channels; data races guarded by vet/race detector.
- Rust: Ownership/borrowing at compile time; threads/tasks with `Send`/`Sync`; `Arc<Mutex<T>>` for shared state.

Generics
- Go 1.18+ type params with constraints.
- Rust generics with trait bounds; blanket impls; specialization limited/unstable.

Async
- Go: implicit runtime; goroutines and `select` built-in.
- Rust: explicit runtime (`tokio`); `async/.await`, `select!` macro.

Testing
- Go: `_test.go` packages + `go test`.
- Rust: `#[test]` in the same crate or `tests/` folder; `cargo test`.

Common Equivalents
- Logging: `log` + `env_logger` or `tracing` (Rust) vs `log` packages in Go.
- HTTP: `reqwest`/`hyper` (Rust) vs `net/http` and `http.Client` (Go).
- CLI: `clap` (Rust) vs `urfave/cli`/`cobra` (Go).

