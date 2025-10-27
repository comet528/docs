# Lesson 02 — Configuration and Structured Logging

Why this matters
- Real tools need configuration that’s mergeable (file/env/CLI) and logs that are actionable in terminals and CI logs. This lesson adds both.

Concepts covered
- Config with `serde` and TOML/YAML; layer: CLI > env > file defaults.
- `tracing` with `tracing-subscriber` and `EnvFilter`.
- Mapping errors to exit codes.

Cargo.toml (excerpt)
```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1"
toml = "0.8"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }
anyhow = "1"
```

Worked example — config struct + merge
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub backend: Option<String>,
    pub target: Option<String>,
    pub timeout_secs: Option<u64>,
}

impl Default for Config { fn default() -> Self { Self { backend: Some("file".into()), target: None, timeout_secs: Some(10) } } }

impl Config {
    pub fn load(path: Option<&str>) -> anyhow::Result<Self> {
        let mut cfg = Config::default();
        if let Some(p) = path { if std::path::Path::new(p).exists() { cfg = merge(cfg, from_file(p)?); } }
        if let Ok(b) = std::env::var("TOOLBOX_BACKEND") { cfg.backend = Some(b); }
        if let Ok(t) = std::env::var("TOOLBOX_TARGET") { cfg.target = Some(t); }
        Ok(cfg)
    }
}

fn from_file(path: &str) -> anyhow::Result<Config> {
    let text = std::fs::read_to_string(path)?;
    if path.ends_with(".yaml") || path.ends_with(".yml") { Ok(serde_yaml::from_str(&text)?) }
    else if path.ends_with(".json") { Ok(serde_json::from_str(&text)?) }
    else { Ok(toml::from_str(&text)?) }
}

fn merge(mut a: Config, b: Config) -> Config {
    if b.backend.is_some() { a.backend = b.backend; }
    if b.target.is_some() { a.target = b.target; }
    if b.timeout_secs.is_some() { a.timeout_secs = b.timeout_secs; }
    a
}
```

Initialize logging
```rust
fn init_logs() {
    use tracing_subscriber::{fmt, EnvFilter};
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).with_target(false).init();
}
```

Exit codes
```rust
fn main() {
    init_logs();
    if let Err(e) = real_main() { eprintln!("error: {:#}", e); std::process::exit(2); }
}
```

Lab
- Add a `--config <path>` flag; load and merge config; default to `~/.config/toolbox/config.toml` if present.
- Log each subcommand invocation with structured fields (`backend`, `target`).

Next
- Add real HTTP behavior (timeouts, retries) and wire config/CLI choices into the backend construction.

