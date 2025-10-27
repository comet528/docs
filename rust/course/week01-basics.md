# Week 01 — Rust Basics, Cargo, First CLI

Objectives
- Install and verify Rust toolchain; understand `cargo` workspace basics.
- Learn core syntax: bindings, mutability, types, control-flow, pattern matching.
- Understand `Option`/`Result` and basic error handling.
- Write and test a small CLI with `cargo`.

Day 1 — Environment & Hello World
- Start here: see `rust/course/quickstart-hello.md` for a full, copy-pasteable walkthrough.
- Install: `rustup`, `rustc`, `cargo`. Add rust-analyzer in your editor.
- Create: `cargo new hello-rust && cd hello-rust`.
- Run: `cargo run`, then accept an arg via `std::env::args`, add a unit test, and build a release binary.
- Compare to Go: `cargo` ≈ `go` tool + modules + test in one; debug vs release profiles; `cargo check` for fast type checks.

Day 2 — Types, Control Flow, Match
- Bindings: `let`, `let mut`, shadowing, constants `const`.
- Types: scalar (i32/u32/f64/bool/char), tuples, arrays, slices, `String` vs `&str`.
- Control: `if`, `loop/break`, `while`, `for` over iterators.
- Pattern matching with `match` and `if let`.
- Lab: parse numbers from args, compute min/max/mean.
  Hint: iterate with `std::env::args().skip(1)`, parse with `str::parse::<f64>()`, and aggregate via iterator adaptors.

Day 3 — Ownership & Borrowing (Intro)
- Moves vs copies; `Copy` types; `clone` vs referencing `&T`/`&mut T`.
- Lifetimes (preview): compilers infer most lifetimes; avoid storing references unless needed.
- `Option<T>` and `Result<T, E>`; the `?` operator.
- Lab: write `fn read_to_string(path: &str) -> std::io::Result<String>`; call from `main` with `?` and handle the `Result` at the top.

Day 4 — Modules, Tests, Docs
- Modules: `mod`, `pub`, `use`, file layout.
- Unit tests: `#[cfg(test)] mod tests { ... }`, `cargo test`.
- Doc comments `///` and examples `/// ```rust`.
- Lab: factor logic into `lib.rs` module; add tests for edge cases and one doc-test in a `///` example.

Checkpoint — Tiny CLI
- Build `numtool` CLI: `cargo new numtool`.
- Input: numbers from stdin or space-separated args.
- Output: `--sum`, `--avg`, `--min`, `--max` flags (pick 2–3).
- Parsing tip: start with `std::env::args` or add `clap` for robust flags.
- Deliverable: release binary in `target/release/numtool` with tests covering zero/invalid input.

Go vs Rust Highlights (Week 1)
- No garbage collector; ownership determines validity and drops.
- `Result` replaces errors-by-value patterns; `?` is concise but explicit.
- Pattern matching and exhaustive enums (`enum`) offer clarity vs ad-hoc tagging.

Useful Commands
- Format/lint: `cargo fmt --all`, `cargo clippy -- -D warnings`.
- Show dependency tree: `cargo tree`.
- Run one test: `cargo test <name>`.
