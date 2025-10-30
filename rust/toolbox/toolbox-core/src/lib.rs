use std::path::{Path, PathBuf};

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Item {
    pub id: String,
}

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("not found")] 
    NotFound,
}

pub type CoreResult<T> = std::result::Result<T, CoreError>;

#[async_trait]
pub trait Backend: Send + Sync {
    async fn list(&self) -> CoreResult<Vec<Item>>;
    async fn get(&self, id: &str) -> CoreResult<Item>;
}

pub struct FileBackend {
    dir: PathBuf,
}

impl FileBackend {
    pub fn new<P: Into<PathBuf>>(p: P) -> Self { Self { dir: p.into() } }
}

#[async_trait]
impl Backend for FileBackend {
    async fn list(&self) -> CoreResult<Vec<Item>> {
        let mut items = vec![];
        for entry in std::fs::read_dir(&self.dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                items.push(Item { id: entry.file_name().to_string_lossy().to_string() });
            }
        }
        Ok(items)
    }

    async fn get(&self, id: &str) -> CoreResult<Item> {
        let p = self.dir.join(id);
        if p.exists() && is_file(&p)? { Ok(Item { id: id.to_string() }) } else { Err(CoreError::NotFound) }
    }
}

fn is_file(p: &Path) -> std::io::Result<bool> { Ok(std::fs::metadata(p)?.is_file()) }

pub struct HttpBackend {
    base: String,
    client: Client,
}

impl HttpBackend {
    pub fn new(base: impl Into<String>) -> Self {
        Self { base: base.into(), client: Client::new() }
    }
    pub fn new_with_client(base: impl Into<String>, client: Client) -> Self {
        Self { base: base.into(), client }
    }
}

#[async_trait]
impl Backend for HttpBackend {
    async fn list(&self) -> CoreResult<Vec<Item>> {
        let url = format!("{}/v1/items", self.base);
        let v = self.client.get(url).send().await?.error_for_status()?.json::<Vec<Item>>().await?;
        Ok(v)
    }
    async fn get(&self, id: &str) -> CoreResult<Item> {
        let url = format!("{}/v1/items/{}", self.base, id);
        let v = self.client.get(url).send().await?.error_for_status()?.json::<Item>().await?;
        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[tokio::test]
    async fn file_backend_lists_files() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("a.txt")).unwrap();
        File::create(dir.path().join("b.txt")).unwrap();
        let fb = FileBackend::new(dir.path());
        let mut items = fb.list().await.unwrap();
        items.sort_by(|a,b| a.id.cmp(&b.id));
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].id, "a.txt");
        assert_eq!(items[1].id, "b.txt");
    }

    #[tokio::test]
    async fn file_backend_get_ok_and_not_found() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("x")) .unwrap();
        let fb = FileBackend::new(dir.path());
        assert!(matches!(fb.get("x").await, Ok(_)));
        assert!(matches!(fb.get("y").await, Err(CoreError::NotFound)));
    }
}

