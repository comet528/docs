# Lesson 01 — Traits and Abstractions (Pluggable Backends)

Why this matters
- Traits are Rust’s interfaces. Good trait design lets you swap backends (file, HTTP, memory) and test with fakes. You choose static (monomorphized) or dynamic (trait object) dispatch where it fits.

Concepts covered
- Trait definition, default methods, blanket impls.
- Static dispatch (`impl Trait`/generics) vs dynamic dispatch (`Box<dyn Trait>`).
- Async methods in traits using `async-trait` for ergonomics.

Cargo.toml (example deps)
```toml
[dependencies]
async-trait = "0.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

Worked example — `Backend` trait with file/http impls
```rust
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item { pub id: String }

#[async_trait]
pub trait Backend: Send + Sync {
    async fn list(&self) -> Result<Vec<Item>>;
    async fn get(&self, id: &str) -> Result<Item>;
}

pub struct FileBackend { dir: std::path::PathBuf }
impl FileBackend { pub fn new<P: Into<std::path::PathBuf>>(p: P) -> Self { Self { dir: p.into() } } }

#[async_trait]
impl Backend for FileBackend {
    async fn list(&self) -> Result<Vec<Item>> {
        let mut items = vec![];
        for entry in std::fs::read_dir(&self.dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let name = entry.file_name().to_string_lossy().to_string();
                items.push(Item { id: name });
            }
        }
        Ok(items)
    }
    async fn get(&self, id: &str) -> Result<Item> {
        let path = self.dir.join(id);
        if path.exists() { Ok(Item { id: id.to_string() }) } else { anyhow::bail!("not found") }
    }
}

pub struct HttpBackend { base: String, client: reqwest::Client }
impl HttpBackend { pub fn new(base: impl Into<String>) -> Self { Self { base: base.into(), client: reqwest::Client::new() } } }

#[async_trait]
impl Backend for HttpBackend {
    async fn list(&self) -> Result<Vec<Item>> {
        let url = format!("{}/v1/items", self.base);
        Ok(self.client.get(url).send().await?.error_for_status()?.json().await?)
    }
    async fn get(&self, id: &str) -> Result<Item> {
        let url = format!("{}/v1/items/{}", self.base, id);
        Ok(self.client.get(url).send().await?.error_for_status()?.json().await?)
    }
}

// Static dispatch — generic over any Backend
pub async fn print_all<B: Backend>(b: &B) -> Result<()> {
    for it in b.list().await? { println!("{}", it.id); }
    Ok(())
}

// Dynamic dispatch — trait object for runtime selection
pub async fn print_one(b: &dyn Backend, id: &str) -> Result<()> {
    println!("{}", b.get(id).await?.id);
    Ok(())
}
```

Discussion
- Use generics when the concrete type is known at compile time for best performance; use trait objects (`&dyn Backend`, `Box<dyn Backend>`) when you pick implementations at runtime.
- `async-trait` avoids manual `Future` types; it generates necessary glue behind the scenes.

Lab
- Add a `MemoryBackend` that stores items in a `Vec<Item>` and implements `Backend`.
- Add a function `count_ids<B: Backend>(b: &B, prefix: &str) -> Result<usize>` that counts IDs with a prefix (works with any backend).

Reference sketch
```rust
pub struct MemoryBackend { items: Vec<Item> }
#[async_trait]
impl Backend for MemoryBackend {
    async fn list(&self) -> Result<Vec<Item>> { Ok(self.items.clone()) }
    async fn get(&self, id: &str) -> Result<Item> { self.items.iter().find(|i| i.id==id).cloned().ok_or_else(|| anyhow::anyhow!("not found")) }
}
```

Next
- Concurrency with threads/channels to process work in parallel, then we’ll revisit async with Tokio.

