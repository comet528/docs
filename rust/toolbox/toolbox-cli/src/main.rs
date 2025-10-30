use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;
use std::path::PathBuf;
use toolbox_core::{Backend, FileBackend, HttpBackend, Item};
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(name = "toolbox", version, about = "Multi-backend toolbox CLI")]
struct Cli {
    /// Backend kind
    #[arg(long, value_enum, env = "TOOLBOX_BACKEND", default_value_t = BackendKind::File)]
    backend: BackendKind,

    /// Path (file backend) or Base URL (http backend)
    #[arg(long, env = "TOOLBOX_TARGET")]
    target: String,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum BackendKind { File, Http }

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all items
    List,
    /// Get one item by id
    Get { id: String },
}

#[tokio::main]
async fn main() {
    init_logs();
    if let Err(err) = real_main().await {
        error!(error = %err, "command failed");
        std::process::exit(1);
    }
}

fn init_logs() { let _ = tracing_subscriber::fmt().with_env_filter("info").try_init(); }

async fn real_main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List => {
            let items = list(&cli.backend, &cli.target).await?;
            print_json(&items)?;
        }
        Commands::Get { id } => {
            let item = get(&cli.backend, &cli.target, &id).await?;
            print_json(&item)?;
        }
    }
    Ok(())
}

async fn list(kind: &BackendKind, target: &str) -> Result<Vec<Item>> {
    let b = make_backend(kind, target).await?;
    let items = b.list().await.context("listing items")?;
    info!(count = items.len(), "listed items");
    Ok(items)
}

async fn get(kind: &BackendKind, target: &str, id: &str) -> Result<Item> {
    let b = make_backend(kind, target).await?;
    let item = b.get(id).await.with_context(|| format!("getting {}", id))?;
    info!(id = %item.id, "got item");
    Ok(item)
}

async fn make_backend(kind: &BackendKind, target: &str) -> Result<Box<dyn Backend>> {
    Ok(match kind {
        BackendKind::File => Box::new(FileBackend::new(PathBuf::from(target))) as _ ,
        BackendKind::Http => Box::new(HttpBackend::new(target.to_string())) as _ ,
    })
}

fn print_json<T: Serialize>(t: &T) -> Result<()> { println!("{}", serde_json::to_string_pretty(t)?); Ok(()) }
