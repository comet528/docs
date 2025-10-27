# Lesson 04 — Modules, Tests, and Docs (Library + Binary)

Why this matters
- Production Rust crates separate reusable logic (library) from the CLI or service (binary). This lesson guides you to factor code into `lib.rs`, write unit and doc tests, and keep `main.rs` thin.

Concepts covered
- Module layout: `lib.rs`, `main.rs`, `mod`, `pub`, `use`.
- Unit tests vs integration tests, doc tests.
- Keeping the binary small; surfacing errors via `Result`.

Worked example — factor `numtool` into a library

1) Create project
- `cargo new numtool && cd numtool`

2) lib.rs with core logic
```rust
// src/lib.rs
/// Computes basic stats: (min, max, mean) for non-empty input.
///
/// # Examples
/// ```
/// let xs = vec![1.0, 2.0, 3.0];
/// let (min, max, mean) = numtool::stats(&xs).unwrap();
/// assert_eq!((min, max, mean), (1.0, 3.0, 2.0));
/// ```
pub fn stats(xs: &[f64]) -> Result<(f64, f64, f64), &'static str> {
    if xs.is_empty() { return Err("empty input"); }
    let min = xs.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let sum: f64 = xs.iter().sum();
    Ok((min, max, sum / xs.len() as f64))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn stats_ok() {
        let (min, max, mean) = stats(&[1.0, 2.0, 3.0]).unwrap();
        assert_eq!((min, max, mean), (1.0, 3.0, 2.0));
    }

    #[test]
    fn stats_empty() { assert!(stats(&[]).is_err()); }
}
```

3) main.rs forwards to the library
```rust
// src/main.rs
fn main() {
    let xs: Vec<f64> = std::env::args().skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();
    match numtool::stats(&xs) {
        Ok((min, max, mean)) => println!("min={} max={} mean={}", min, max, mean),
        Err(e) => { eprintln!("error: {}", e); std::process::exit(1); }
    }
}
```

4) Run and test
- `cargo test` to run unit and doc tests (the example in the doc comment).
- `cargo run -- 1 2 3`

Integration tests (optional)
- Create `tests/cli.rs` with end-to-end tests that run the binary via `assert_cmd`.
- This requires adding dev‑dependencies; you’ll do more of this in Week 4.

Go ↔ Rust mindset
- Think “core logic in lib; thin bin” — similar to Go packages with a small `main`.
- Use doc comments and examples; Rust will compile and run them as tests.

Lab
- Add a `median` function to `lib.rs` with tests and a doc test.
- Update `main.rs` to support `--median` printing.

Reference solution (median function)
```rust
pub fn median(mut xs: Vec<f64>) -> Option<f64> {
    if xs.is_empty() { return None; }
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = xs.len() / 2;
    if xs.len() % 2 == 0 {
        Some((xs[mid - 1] + xs[mid]) / 2.0)
    } else {
        Some(xs[mid])
    }
}
```

Next
- You’ve completed Week 1’s fundamentals with real code and tests. In Week 2 you’ll deepen ownership, lifetimes, and error modeling.

