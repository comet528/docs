# Lesson 04 — Observability and Async Testing

Why this matters
- Production tools need structured logs, sane defaults, and testable async code. `tracing` provides rich context; async tests validate concurrency paths.

Concepts covered
- `tracing` and `tracing-subscriber` with `EnvFilter`.
- `#[tracing::instrument]` for spans, structured fields.
- `#[tokio::test]` and deterministic tests.

Cargo.toml (deps)
```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros", "time"] }
anyhow = "1"
```

Worked example — instrumented backend wrapper
```rust
use anyhow::Result;
use async_trait::async_trait;
use tracing::{info, instrument};

#[async_trait]
trait Backend { async fn do_work(&self, id: &str) -> Result<String>; }

struct Logging<B> { inner: B }
impl<B> Logging<B> { fn new(inner: B) -> Self { Self { inner } } }

#[async_trait]
impl<B: Backend + Send + Sync> Backend for Logging<B> {
    #[instrument(skip(self))]
    async fn do_work(&self, id: &str) -> Result<String> {
        let out = self.inner.do_work(id).await?;
        info!(%id, len = out.len(), "work done");
        Ok(out)
    }
}

fn init_logs() { let _ = tracing_subscriber::fmt().with_env_filter("info").try_init(); }
```

Async test example
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    struct Fake;
    #[async_trait]
    impl Backend for Fake { async fn do_work(&self, id: &str) -> Result<String> { sleep(Duration::from_millis(5)).await; Ok(id.repeat(2)) } }

    #[tokio::test]
    async fn doubles_id() {
        super::init_logs();
        let svc = Logging::new(Fake);
        let out = svc.do_work("ab").await.unwrap();
        assert_eq!(out, "abab");
    }
}
```

Discussion
- Use `EnvFilter` so users can set `RUST_LOG=debug,toolbox=trace`.
- In tests, initialize the subscriber once with `try_init()` to avoid panics on re-init.

Lab
- Wrap one of your Week 3 backends in a logging layer recording durations with `tracing::Span::current().record("ms", &...)` or by measuring `Instant::now()`.

Next
- Week 4: turn these abstractions into a production‑ready CLI with config, logging, and packaging.

