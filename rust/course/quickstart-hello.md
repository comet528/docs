# Quickstart — Hello World (Prove Your Toolchain)

Goal
- Verify your Rust toolchain works by building and running a small program, then create a release binary and (optionally) a static MUSL build.

1) Verify Installation
- Versions: `rustc --version && cargo --version && rustup --version`
- Update: `rustup update`
- Optional: pin repo to stable: `rustup override set stable`
- Editor: ensure rust-analyzer is enabled (VS Code: Rust Analyzer extension).

2) Create Project
- New binary crate: `cargo new hello-rust && cd hello-rust`
- Layout: `Cargo.toml` (manifest) and `src/main.rs` (entrypoint)

3) First Run
- Run: `cargo run`
- You should see: `Hello, world!`

4) Make It Yours
- Edit `src/main.rs`:
  
  ```rust
  fn main() {
      println!("Hello, Rust!");
  }
  ```

- Run again: `cargo run`

5) Accept Args (tiny CLI)
- Replace `src/main.rs` with:
  
  ```rust
  use std::env;

  fn main() {
      let who = env::args().nth(1).unwrap_or_else(|| "world".into());
      println!("hello {}", who);
  }
  ```

- Examples:
  - `cargo run --` → `hello world`
  - `cargo run -- Sam` → `hello Sam`

6) Add a Test
- Append to `src/main.rs`:
  
  ```rust
  #[cfg(test)]
  mod tests {
      #[test]
      fn it_works() {
          assert_eq!(2 + 2, 4);
      }
  }
  ```

- Run tests: `cargo test`

7) Build Release Binary
- Optimized build: `cargo build --release`
- Run it: `./target/release/hello-rust Sam`
- Compare sizes: `ls -lh target/debug/hello-rust target/release/hello-rust`

8) Optional — Static MUSL Build (Linux)
- Add target: `rustup target add x86_64-unknown-linux-musl`
- Build: `cargo build --release --target x86_64-unknown-linux-musl`
- Binary: `target/x86_64-unknown-linux-musl/release/hello-rust`
- Check linkage (Linux): `ldd target/x86_64-unknown-linux-musl/release/hello-rust` → "not a dynamic executable" (static)

Notes
- macOS/Windows cannot run Linux MUSL binaries locally; use Docker or WSL2. For cross-building in containers, consider `cross`.
- Handy commands: `cargo check` (fast compile check), `cargo fmt`, `cargo clippy -- -D warnings`.

Next
- Move to Week 1 labs for variables, control flow, and a tiny numeric CLI.

