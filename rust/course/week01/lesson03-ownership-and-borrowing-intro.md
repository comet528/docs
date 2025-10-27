# Lesson 03 — Ownership and Borrowing (Essentials)

Why this matters
- Rust’s memory model (ownership/borrowing) replaces the need for a GC and prevents data races at compile time. Internalizing the rules early saves time later.

Concepts covered
- Moves vs copies; when `Copy` applies (e.g., integers, bools, small tuples of `Copy`).
- Borrowing: `&T` for shared access, `&mut T` for exclusive mutable access.
- String ownership and slicing; returning references vs owned values.
- The `?` operator with `Result`.

Worked example A — “first word” without allocation
```rust
// Returns a borrowed slice of the first word in `s`.
// Lifetimes are elided; Rust infers: fn first_word<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    for (i, b) in s.as_bytes().iter().enumerate() {
        if *b == b' ' { return &s[..i]; }
    }
    s
}

#[test]
fn first_word_examples() {
    assert_eq!(first_word("hello world"), "hello");
    assert_eq!(first_word("rust"), "rust");
}
```

Discussion
- We borrow `&str` and return a subslice `&str`; no allocations.
- If we needed to store the result beyond the input’s lifetime, we’d own it (`String`).

Worked example B — reading a file (return owned data)
```rust
use std::fs;

fn read_to_string(path: &str) -> std::io::Result<String> {
    fs::read_to_string(path)
}

#[test]
fn read_file_ok() {
    // Use a temp file in real tests; simplified here.
    let _ = read_to_string("Cargo.toml"); // just compiles
}
```

Discussion
- We return `String` (owned) because the data must outlive the file handle and local buffers.
- Use `?` to propagate errors from callers:

```rust
fn read_then_first(path: &str) -> std::io::Result<String> {
    let contents = std::fs::read_to_string(path)?;
    Ok(first_word(&contents).to_string())
}
```

Common compiler hints
- “borrow of moved value” → you moved ownership; borrow with `&` or clone with `.clone()` if ownership is required in multiple places.
- “cannot borrow `x` as mutable more than once at a time” → restrict the mutable borrow’s scope, or restructure to avoid aliasing.

Go ↔ Rust mindset
- In Go, you’d return slices of a `string` freely; in Rust, the borrow checker enforces that referenced data outlives its users.
- Prefer owning at API boundaries and borrowing internally to keep lifetimes simple.

Lab
- Implement `read_lines(path: &str) -> std::io::Result<Vec<String>>` that returns all non-empty lines.
- Add a test with a small fixture file.

Reference solution
```rust
use std::io::{BufRead, BufReader};

fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    let f = std::fs::File::open(path)?;
    let reader = BufReader::new(f);
    Ok(reader
        .lines()
        .filter_map(|l| l.ok())
        .filter(|s| !s.trim().is_empty())
        .collect())
}

#[test]
fn lines_basic() {
    // For a proper test, write a temp file; shown inline for brevity.
    assert!(read_lines("Cargo.toml").is_ok());
}
```

Next
- Proceed to Lesson 04 for modules, tests, and docs to start shaping a small library + binary layout.

