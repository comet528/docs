# Project — CLI: Multi-Backend Toolbox

Goal
- Build a CLI that lists and fetches items from multiple backends (file, HTTP). No UI; focus on reliability, logging, and tests.

Requirements
- Subcommands: `list`, `get <id>`.
- Backends: `--backend file --path <p>` and `--backend http --url <u>`.
- Config file (TOML/YAML) with defaults; env var overrides.
- Structured logs with `tracing` and proper exit codes on errors.

Architecture
- Core trait: `trait Backend { async fn list(&self) -> Result<Vec<Item>, Error>; async fn get(&self, id: Id) -> Result<Item, Error>; }`
- Implementations: FileBackend and HttpBackend.
- CLI layer depends on trait; backend chosen via config/flag.

Acceptance Criteria
- `toolbox list --backend file --path data/` prints JSON list.
- `toolbox get 123 --backend http --url https://example/api` prints item as JSON.
- Errors are human-friendly (binaries use `anyhow` with context) and logged.
- 80%+ code paths covered by unit tests; one integration test runs CLI against temp data.

Stretch
- Concurrency limit for HTTP; exponential backoff; cache to local file.

