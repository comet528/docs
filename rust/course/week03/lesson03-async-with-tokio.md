# Lesson 03 — Async Foundations with Tokio

Why this matters
- Async IO scales concurrent network work without many OS threads. Tokio provides a runtime, timers, and utilities for structured concurrency.

Concepts covered
- `#[tokio::main]`, tasks with `tokio::spawn`, joining tasks.
- Timeouts, cancellation with `tokio::select!`.
- Concurrency limits with `Semaphore`.

Cargo.toml (deps)
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }
anyhow = "1"
``` 

Worked example — concurrent fetch with limit + retry
```rust
use anyhow::{Context, Result};
use reqwest::Client;
use std::sync::Arc;
use tokio::{sync::Semaphore, time::{sleep, Duration}};

#[tokio::main]
async fn main() -> Result<()> {
    let urls = vec![
        "https://example.com", 
        "https://httpbin.org/status/200",
        "https://httpbin.org/status/503",
    ];
    let client = Arc::new(Client::builder().timeout(Duration::from_secs(5)).build()?);
    let sem = Arc::new(Semaphore::new(4));

    let mut handles = vec![];
    for url in urls {
        let client = Arc::clone(&client);
        let sem = Arc::clone(&sem);
        let url = url.to_string();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            fetch_with_retry(&client, &url, 3).await.map(|code| (url, code))
        }));
    }

    for h in handles { println!("{:?}", h.await??); }
    Ok(())
}

async fn fetch_with_retry(client: &Client, url: &str, tries: u32) -> Result<u16> {
    let mut delay = Duration::from_millis(100);
    for attempt in 1..=tries {
        match client.get(url).send().await {
            Ok(resp) => return Ok(resp.status().as_u16()),
            Err(err) if attempt < tries => {
                eprintln!("attempt {} failed ({}); retrying in {:?}", attempt, err, delay);
                sleep(delay).await; delay *= 2;
            }
            Err(err) => return Err(err).with_context(|| format!("GET {} failed after {} tries", url, tries)),
        }
    }
    unreachable!()
}
```

Cancellation with select
```rust
use tokio::{signal, select, time::{sleep, Duration}};

#[tokio::main]
async fn main() {
    select! {
        _ = long_task() => {}
        _ = signal::ctrl_c() => { eprintln!("cancelled"); }
    }
}

async fn long_task() { sleep(Duration::from_secs(30)).await; }
```

Lab
- Build a URL fetcher that reads URLs from stdin and prints status codes with a concurrency limit and global timeout per request.

Reference hints
- Use `tokio::io::AsyncBufReadExt` to read stdin lines; wrap each fetch in `tokio::time::timeout`.

Next
- Observability and async testing: structured logs and `#[tokio::test]`.

