# Week 03 — Traits, Concurrency, Async

Objectives
- Design with traits and generics; create pluggable backends.
- Use threads, channels, and synchronization primitives.
- Learn async foundations (`Future`, `async/.await`) with `tokio`.

Day 1 — Traits In Practice
- Define trait(s) capturing an abstraction (e.g., `Backend { fetch() }`).
- Static dispatch (`impl Trait`) vs dynamic dispatch (`Box<dyn Trait>`).
- Blanket impls, default methods, trait objects, and lifetimes.
- Lab: two backends: file-based and HTTP-based; a function taking `impl Backend`.

Day 2 — Concurrency Fundamentals
- `std::thread`, `JoinHandle`, channels (`std::sync::mpsc`), `Arc<T>`, `Mutex<T>`.
- `Send` and `Sync` auto-traits; when types are thread-safe.
- Lab: fan-out/fan-in pipeline; bounded channel; graceful shutdown.

Day 3 — Async Foundations with Tokio
- Runtime, tasks `tokio::spawn`, timers, `select!`, cancellation.
- Async IO via `reqwest`/`tokio` and streams.
- Lab: concurrent HTTP fetcher with retry/backoff, rate limiting via `tokio::time`.

Day 4 — Observability & Testing
- Logging with `tracing` + `tracing-subscriber` JSON or compact formats.
- Structured errors and metrics (optional: `metrics` + `prometheus` exporter).
- Tests: `#[tokio::test]`, testing timeouts, and controlling concurrency in tests.

Checkpoint — Pluggable Backends
- Define `trait Backend { async fn list(&self) -> Result<Vec<Item>, Error>; }`
- Implement two backends; choose dispatch style based on needs.
- Run in parallel using async; log timings; add tests with a fake backend.

Go vs Rust Highlights (Week 3)
- Goroutines vs async tasks: explicit runtimes in Rust; structured cancellation using `select!`.
- Channels exist in both; ownership adds compile-time safety around shared state.
- Trait objects vs interfaces: more control over static/dynamic dispatch.

Commands
- Add deps: `tokio = { version = "1", features = ["full"] }`, `tracing`, `tracing-subscriber`.
- Run single-threaded vs multi-threaded runtime: `#[tokio::main(flavor = "current_thread")]` or default.

