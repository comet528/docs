# Lesson 02 — Types, Mutability, Pattern Matching, and a Numeric CLI

Why this matters
- Rust’s type system and control flow encourage explicit, predictable code. Getting comfortable with `let` vs `let mut`, `String` vs `&str`, and `match` unlocks most day-to-day work.

Concepts covered
- Bindings: `let`, `let mut`, shadowing, `const`.
- Scalars (integers, floats, bool, char), compound types (tuple, array, slice), `String` vs `&str`.
- Control flow: `if`, `loop/break/continue`, `while`, `for` over iterators.
- Pattern matching with `match` and `if let`.

Worked example — a minimal numeric CLI
Goal: read numbers from CLI args and compute `min`, `max`, `mean`.

```rust
// src/main.rs (new project: `cargo new numtool && cd numtool`)
use std::env;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Stats { min: f64, max: f64, mean: f64 }

fn main() {
    let nums = parse_numbers(env::args().skip(1));
    match nums.as_slice() {
        [] => {
            eprintln!("provide at least one number");
            std::process::exit(1);
        }
        slice => {
            let stats = compute_stats(slice);
            println!("min={} max={} mean={}", stats.min, stats.max, stats.mean);
        }
    }
}

fn parse_numbers<I, S>(iter: I) -> Vec<f64>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    iter
        .into_iter()
        .filter_map(|s| s.as_ref().parse::<f64>().ok())
        .collect()
}

fn compute_stats(xs: &[f64]) -> Stats {
    let (mut min, mut max, mut sum) = (f64::INFINITY, f64::NEG_INFINITY, 0.0);
    for &x in xs {
        if x < min { min = x; }
        if x > max { max = x; }
        sum += x;
    }
    Stats { min, max, mean: sum / xs.len() as f64 }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn stats_basic() {
        let xs = vec![1.0, 2.0, 3.0];
        let s = compute_stats(&xs);
        assert_eq!(s, Stats { min: 1.0, max: 3.0, mean: 2.0 });
    }
}
```

Discussion
- `let` creates an immutable binding by default; use `let mut` to allow reassignment.
- We accept any iterator of items convertible to `&str` via `AsRef<str>` — a gentle intro to generics.
- `match nums.as_slice()` ensures we handle the empty case explicitly (exhaustive matching).

Alternative using iterators
```rust
fn compute_stats(xs: &[f64]) -> Stats {
    let min = xs.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let sum: f64 = xs.iter().sum();
    Stats { min, max, mean: sum / xs.len() as f64 }
}
```

Go ↔ Rust mindset
- Rust’s `match` is exhaustive and works with powerful patterns; prefer it over chains of `if/else` when matching discrete cases.
- Strings: `&str` is a borrowed view into a UTF‑8 string; `String` owns the allocation. Own at boundaries; borrow within.

Lab
- Extend CLI with flags: `--sum`, `--avg`, `--min`, `--max`; print only what’s requested.
- If no args are passed, read newline‑separated numbers from stdin.

Reference solution (flags only)
```rust
use std::env;

fn main() {
    let mut flags = vec![];
    let mut vals = vec![];
    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--min" | "--max" | "--avg" | "--sum" => flags.push(arg),
            other => match other.parse::<f64>() {
                Ok(x) => vals.push(x),
                Err(_) => eprintln!("skipping non-number: {}", other),
            },
        }
    }

    if vals.is_empty() { eprintln!("no numbers provided"); std::process::exit(1); }
    let s = compute(&vals);
    for f in &flags {
        match f.as_str() {
            "--min" => println!("{}", s.0),
            "--max" => println!("{}", s.1),
            "--avg" => println!("{}", s.2),
            "--sum" => println!("{}", s.3),
            _ => {}
        }
    }
}

fn compute(xs: &[f64]) -> (f64, f64, f64, f64) {
    let min = xs.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let sum: f64 = xs.iter().sum();
    let avg = sum / xs.len() as f64;
    (min, max, avg, sum)
}
```

Next
- Proceed to Lesson 03 for ownership and borrowing essentials.

