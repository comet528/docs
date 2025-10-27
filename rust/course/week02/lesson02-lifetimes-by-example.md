# Lesson 02 — Lifetimes by Example

Why this matters
- Lifetimes make reference relationships explicit to the compiler (and readers). Most simple cases are elided, but knowing the rules helps when signatures get richer.

Concepts covered
- Lifetime elision rules for `&self` methods and function parameters.
- Returning references tied to inputs.
- Storing references in structs vs preferring owned data.

Worked example A — explicit lifetime (demonstration)
```rust
// In practice, you’d write `fn longer(a: &str, b: &str) -> &str` and elision applies.
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str { if a.len() >= b.len() { a } else { b } }

#[test]
fn longer_examples() {
    let a = String::from("abc");
    let b = String::from("abcd");
    assert_eq!(longer(&a, &b), "abcd");
}
```

Worked example B — structs with references vs owned
```rust
struct View<'a> { name: &'a str }        // borrows external data
struct Owned { name: String }            // owns its data

impl Owned {
    fn new(name: &str) -> Self { Self { name: name.to_string() } }
}

#[test]
fn views_and_owned() {
    let s = String::from("rust");
    let view = View { name: &s }; // valid while `s` is alive
    assert_eq!(view.name, "rust");
    let owned = Owned::new(&s);
    drop(s); // `view` can’t be used beyond this point; `owned` survives
    assert_eq!(owned.name, "rust");
}
```

Design guidance
- If a type needs to be returned from a factory or stored broadly, prefer owning fields (`String`).
- Reserve reference-bearing structs (with lifetimes) for views over temporary or external data.

Lab
- Implement `fn first_longer<'a>(a: &'a str, b: &'a str) -> Option<&'a str>` that returns `Some(a)` if `a` is strictly longer.
- Write a struct `LineSplit<'a>` that holds `&'a str` for a line and exposes `first_word()` returning `&'a str`.

Reference solution
```rust
fn first_longer<'a>(a: &'a str, b: &'a str) -> Option<&'a str> {
    (a.len() > b.len()).then_some(a)
}

struct LineSplit<'a> { line: &'a str }
impl<'a> LineSplit<'a> { fn first_word(&self) -> &'a str { super::first_word(self.line) } }
```

Next
- Move to error ergonomics to design friendly libraries and CLIs.

