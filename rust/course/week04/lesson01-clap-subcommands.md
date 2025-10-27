# Lesson 01 — Clap Subcommands and CLI Structure

Why this matters
- A predictable CLI with subcommands and flags is the backbone of developer tooling. `clap` derive keeps definitions near types and generates help/usage.

Concepts covered
- `clap` derive, subcommands, value enums, env var fallbacks.
- Thin `main` delegating to core logic and backends.

Cargo.toml (excerpt)
```toml
[dependencies]
anyhow = "1"
clap = { version = "4", features = ["derive", "env"] }
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }
```

Worked example — `toolbox` skeleton
```rust
use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use tracing::{error};

#[derive(Parser, Debug)]
#[command(name = "toolbox", version, about = "Multi-backend toolbox")]
struct Cli {
    #[arg(long, value_enum, env = "TOOLBOX_BACKEND", default_value_t = BackendKind::File)]
    backend: BackendKind,
    #[arg(long, env = "TOOLBOX_TARGET", help = "Path (file) or base URL (http)")]
    target: String,
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum BackendKind { File, Http }

#[derive(Subcommand, Debug)]
enum Commands {
    List,
    Get { id: String },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    if let Err(e) = real_main().await { error!(error=%e, "failed"); std::process::exit(1); }
}

async fn real_main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List => commands::list(&cli.backend, &cli.target).await?,
        Commands::Get { id } => commands::get(&cli.backend, &cli.target, &id).await?,
    }
    Ok(())
}

mod commands {
    use super::*;
    pub async fn list(kind: &BackendKind, target: &str) -> Result<()> { println!("list on {:?} {}", kind, target); Ok(()) }
    pub async fn get(kind: &BackendKind, target: &str, id: &str) -> Result<()> { println!("get {} on {:?} {}", id, kind, target); Ok(()) }
}
```

Lab
- Replace the stubbed `commands` module by wiring in your Week 3 backends and printing JSON outputs.

Next
- Add config files and structured logging to make the CLI production‑ready.

