# Lesson 01 — Hello World, Cargo, and Your First Tests

Why this matters
- Before touching ownership or async, you need confidence the toolchain works and you can produce binaries. This lesson walks from a blank folder to a release build with a simple CLI and a unit test, highlighting differences from Go.

Concepts covered
- `rustup` toolchains, `cargo` (build, run, test), crates and crate roots.
- `main` function, macros (`println!`), command-line args, and unit tests.
- Debug vs release builds, how to run binaries, how to structure a tiny CLI.

Setup
- Verify: `rustc --version && cargo --version && rustup --version`
- Update: `rustup update`
- Editor: enable rust-analyzer (VS Code: “Rust Analyzer” extension).

Create the project
- `cargo new hello-rust && cd hello-rust`
- You now have:
  - `Cargo.toml` — package manifest
  - `src/main.rs` — program entrypoint

Worked example A — the canonical hello
```rust
// src/main.rs
fn main() {
    println!("Hello, world!");
}
```
- Run: `cargo run` → “Hello, world!”
- Build an optimized binary: `cargo build --release`
- Run the release binary: `./target/release/hello-rust`

Worked example B — accept an argument
```rust
// src/main.rs
use std::env;

fn main() {
    let who = env::args().nth(1).unwrap_or_else(|| "world".into());
    println!("hello {}", who);
}
```
- Try: `cargo run --` → `hello world`
- Try: `cargo run -- Sam` → `hello Sam`

Worked example C — add a unit test
```rust
// src/main.rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```
- Run: `cargo test`

Common trip-ups (and fixes)
- “cannot find function `println`” → use `println!` macro (notice the `!`).
- “use of moved value” → Rust moves non-`Copy` values by default; borrow with `&` if you only need to read.
- “unresolved import” → ensure the module/file layout matches (`mod x;` looks for `x.rs` or `x/mod.rs`).

Go ↔ Rust mindset
- Go’s `go build; ./prog arg` ≈ Rust’s `cargo run -- arg` during dev; use `cargo build --release` for optimized binaries.
- Rust’s `println!` is a macro. Macros are powerful code generators used throughout the ecosystem.

Lab — personalize hello
- Add an env var prefix: if `HELLO_PREFIX` is set, print `"<prefix> <who>"`.
- Add a `--shout` flag to uppercase the output.
- Exit with nonzero on invalid UTF-8 in env var (simulate by catching `std::env::VarError`).

Reference solution
```rust
use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let mut shout = false;
    let mut who = "world".to_string();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--shout" => shout = true,
            other => who = other.to_string(),
        }
    }

    let prefix = match env::var("HELLO_PREFIX") {
        Ok(p) => p,
        Err(env::VarError::NotPresent) => String::new(),
        Err(e) => {
            eprintln!("env error: {}", e);
            std::process::exit(2);
        }
    };

    let mut msg = if prefix.is_empty() {
        format!("hello {}", who)
    } else {
        format!("{} {}", prefix, who)
    };

    if shout { msg = msg.to_uppercase(); }
    println!("{}", msg);
}

#[cfg(test)]
mod tests {
    #[test]
    fn math_still_works() { assert_eq!(2 + 2, 4); }
}
```

Checkpoint
- Produce the release binary and run it with `--shout`.
- Optional (Linux): build static: `rustup target add x86_64-unknown-linux-musl && cargo build --release --target x86_64-unknown-linux-musl`.

Next
- Proceed to Lesson 02 for Rust’s core types, control flow, and pattern matching.

