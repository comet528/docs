# Week 02 — Ownership, Lifetimes, Errors, Modules

Objectives
- Internalize ownership/borrowing rules and common patterns.
- Work with Strings, slices, Vec, HashMap; iterators and closures.
- Model errors ergonomically with `thiserror` and `anyhow`.
- Organize code into library + binary crates.

Day 1 — Ownership Deep Dive
- Moves, `Copy`, `Clone`, borrowing `&T` vs `&mut T`.
- Borrow checker rules and common compiler messages; reduce mutable aliases.
- Smart pointers: `Box<T>`, `Rc<T>`, `Arc<T>`; interior mutability `RefCell<T>`.
- Lab: implement a small in-memory index with `HashMap<String, Vec<String>>` using `&str` vs `String` intentionally.

Day 2 — Lifetimes By Example
- Function signatures with references; elision rules; method receivers.
- Structs with references; when to switch to owned `String`.
- Traits and lifetimes generics: `impl<'a> Trait for Foo<'a> {}` (light exposure).
- Lab: refactor Day 1 to minimize explicit lifetimes; benchmark mentally via fewer clones.

Day 3 — Iterators, Traits (Intro), Generics
- Iterator adaptors: `map`, `filter`, `collect`, `try_fold`.
- Closures capture and `move` closures.
- Generics + trait bounds: `fn f<T: Read + Send>(t: T)`.
- Lab: parse CSV to structs using `serde` + `serde_json` or `csv` crate; compute aggregates with iterators.

Day 4 — Errors Done Right
- `Result<T, E>` layering: library exposes concrete `E` via `thiserror`; binaries use `anyhow::Result`.
- Conversions: `From`, `Into`, `map_err`, `context` with `anyhow::Context`.
- Lab: wrap IO/parse/HTTP errors in a single enum; write tests asserting error variants via pattern matching.

Checkpoint — Library + Binary
- Create workspace: `cargo new --vcs none --workspace toolbox`.
- Crates: `toolbox-core` (lib) with `thiserror`, `toolbox-cli` (bin) with `anyhow`.
- Implement 2–3 core functions in the lib with tests; CLI calls into lib and propagates errors with `?`.

Go vs Rust Highlights (Week 2)
- Prefer owning data (`String`) at boundaries to avoid lifetime juggling internally.
- Interfaces vs Traits: Rust traits can be blanket-implemented and are resolved at compile time by default.
- Errors are values in both languages; Rust pushes you to define explicit domains and use `?` for flow.

Commands
- Workspaces: root `Cargo.toml` with `members = ["toolbox-core", "toolbox-cli"]`.
- Run specific bin: `cargo run -p toolbox-cli -- <args>`.

