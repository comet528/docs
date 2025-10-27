# Lesson 03 — HTTP Client, Retries, and Backend Selection

Why this matters
- Production CLIs must handle flaky networks with timeouts and backoff. This lesson wires your CLI to real backends and adds resilience.

Concepts covered
- Building a `reqwest::Client` with timeouts and TLS.
- Backoff and retry strategy.
- Selecting a backend from CLI/config and executing subcommands.

Client initialization
```rust
use reqwest::Client;
use std::time::Duration;

fn http_client(timeout_secs: u64) -> anyhow::Result<Client> {
    Ok(Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .pool_idle_timeout(Duration::from_secs(30))
        .build()?)
}
```

Retry helper
```rust
use tokio::time::{sleep, Duration};

async fn with_retry<F, Fut, T, E>(mut op: F, tries: u32) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut delay = Duration::from_millis(100);
    for attempt in 1..=tries {
        match op().await {
            Ok(v) => return Ok(v),
            Err(e) if attempt < tries => { sleep(delay).await; delay *= 2; }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}
```

Backend selection
```rust
enum BackendKind { File, Http }

async fn make_backend(kind: BackendKind, target: &str, timeout: u64) -> anyhow::Result<Box<dyn Backend>> {
    Ok(match kind {
        BackendKind::File => Box::new(FileBackend::new(target)) as _ ,
        BackendKind::Http => {
            let client = http_client(timeout)?;
            Box::new(HttpBackend::new_with_client(target.to_string(), client)) as _
        }
    })
}
```

Wire into commands
```rust
pub async fn list(kind: BackendKind, target: String, timeout: u64) -> anyhow::Result<()> {
    let b = make_backend(kind, &target, timeout).await?;
    let items = with_retry(|| b.list(), 3).await?; // retryable operation
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}
```

Lab
- Add a `--retries <n>` flag to control attempts; log each attempt with `tracing` at `debug` level.
- Add concurrency limit to `list` for HTTP backend if it performs multiple requests.

Next
- Package binaries for distribution, including static musl builds and small container images.

