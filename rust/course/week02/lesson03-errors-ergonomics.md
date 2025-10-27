# Lesson 03 — Errors Done Right (`Result`, `thiserror`, `anyhow`)

Why this matters
- Clear, composable errors make CLIs and services predictable and diagnosable. Libraries should surface precise error types; binaries can use ergonomic context.

Concepts covered
- `Result<T, E>` and the `?` operator; `From` conversions.
- Library error enums via `thiserror`.
- Application‑level errors via `anyhow` with context.

Cargo.toml (add deps)
```toml
[dependencies]
thiserror = "1"
anyhow = "1"
```

Worked example — library + binary
```rust
// src/lib.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse float: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),
}

pub fn parse_numbers(s: &str) -> Result<Vec<f64>, CoreError> {
    s.split_whitespace().map(|t| Ok(t.parse::<f64>()?)).collect()
}

// src/main.rs
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path = std::env::args().nth(1).context("usage: app <path>")?;
    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("reading {}", path))?;
    let nums = mylib::parse_numbers(&contents).context("parsing numbers")?;
    println!("count={}", nums.len());
    Ok(())
}
```

Discussion
- Library exposes a precise error enum; `#[from]` on variants wires automatic conversion and enables chaining `?`.
- Binary uses `anyhow::Result` and `.context(...)` to attach human‑readable details.

Lab
- Add an error variant for empty input; ensure the CLI prints a friendly message and returns nonzero exit code.

Reference hint
```rust
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("empty input")] Empty,
    // ...
}
```

Next
- In Week 3 you will combine traits with concurrency/async to build pluggable backends.

