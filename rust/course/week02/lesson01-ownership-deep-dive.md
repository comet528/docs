# Lesson 01 — Ownership Deep Dive

Why this matters
- Ownership and borrowing rules are the bedrock of Rust’s safety guarantees. Mastering them enables fear‑free refactoring and efficient code without a GC.

Concepts covered
- Move semantics, `Copy` and `Clone`.
- Borrowing (`&T`) vs mutable borrowing (`&mut T`); reference scopes.
- Smart pointers overview: `Box<T>`, `Rc<T>`, `Arc<T>`; interior mutability with `RefCell<T>`.

Worked example A — move vs borrow
```rust
fn takes_ownership(s: String) -> usize { s.len() }
fn borrows_str(s: &str) -> usize { s.len() }

#[test]
fn ownership_and_borrowing() {
    let s = String::from("hello");
    let len = borrows_str(&s);   // borrow, s still usable
    assert_eq!(len, 5);

    let s2 = String::from("world");
    let len2 = takes_ownership(s2); // move; s2 is no longer valid here
    assert_eq!(len2, 5);
    // s2; // <- would be a compile error
}
```

Worked example B — mutable borrow rules
```rust
#[test]
fn mutable_borrowing() {
    let mut v = vec![1, 2, 3];
    // Only one mutable borrow at a time
    let r = &mut v;
    r.push(4);
    // Borrow ends here; v is usable again
    assert_eq!(v.len(), 4);
}
```

When to clone
- Cloning copies heap data and can be expensive. Prefer borrowing (`&T`) when possible.
- Clone at the edges when API design or ownership requirements demand separate ownership.

Shared ownership
- Single-threaded: `Rc<T>` (non-atomic ref counting). Multi-threaded: `Arc<T>`.
- For shared mutable state, combine with interior mutability (`RefCell<T>` single-thread) or synchronization primitives (`Mutex<T>` multi-thread).

Lab
- Implement an in-memory index: `HashMap<String, Vec<String>>` with functions:
  - `insert(tag: &str, item: String)`
  - `items(tag: &str) -> &[String]` (discuss why returning `&[String]` ties lifetime to the map)
  - Provide an alternative returning `Vec<String>` to own results.

Reference sketch
```rust
use std::collections::HashMap;

#[derive(Default)]
struct Index { map: HashMap<String, Vec<String>> }

impl Index {
    fn insert(&mut self, tag: &str, item: String) {
        self.map.entry(tag.to_string()).or_default().push(item);
    }

    fn items_owned(&self, tag: &str) -> Vec<String> {
        self.map.get(tag).cloned().unwrap_or_default()
    }
}
```

Next
- Proceed to lifetimes to understand how references flow through function signatures and structs.

