# Lesson 02 — Threads, Channels, and Shared State

Why this matters
- Threads + channels are a foundation for parallel pipelines. Rust’s ownership model adds safety around shared state (`Send`/`Sync`).

Concepts covered
- `std::thread::spawn`, `JoinHandle`, `move` closures.
- Channels: `std::sync::mpsc::{channel, Sender, Receiver}`; closing channels.
- Shared ownership with `Arc<T>`, interior mutability with `Mutex<T>`.

Worked example — fan‑out/fan‑in pipeline
Goal: spawn N workers that compute lengths of strings, then collect results.

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    let (tx_in, rx_in) = mpsc::channel::<String>();
    let (tx_out, rx_out) = mpsc::channel::<(String, usize)>();
    let workers = 4;

    let tx_out = Arc::new(tx_out);
    let mut handles = vec![];
    for _ in 0..workers {
        let rx_in = rx_in.clone(); // Receiver is clonable (multi-consumer)
        let tx_out = Arc::clone(&tx_out);
        handles.push(thread::spawn(move || {
            for s in rx_in.iter() { // ends when input channel closes
                let len = s.len();
                tx_out.send((s, len)).ok();
            }
        }));
    }

    // Feed work, then drop the sender to close the channel
    for s in ["alpha", "beta", "gamma", "delta", "epsilon"] { tx_in.send(s.to_string()).unwrap(); }
    drop(tx_in);

    // Collect until output channel closes (workers exit once input is closed)
    drop(tx_out); // no one else will send new Senders; allows rx_out.iter() to end after workers finish
    let results: Vec<_> = rx_out.iter().collect();
    for (s, n) in results { println!("{} => {}", s, n); }

    for h in handles { h.join().unwrap(); }
}
```

Shared state example — counting occurrences
```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counts = Arc::new(Mutex::new(HashMap::<char, u64>::new()));
    let inputs = vec!["abc", "bcd", "cde", "def"];
    let mut handles = vec![];
    for chunk in inputs.chunks(2) {
        let counts = Arc::clone(&counts);
        let chunk = chunk.to_vec();
        handles.push(thread::spawn(move || {
            let mut map = counts.lock().unwrap();
            for s in chunk { for ch in s.chars() { *map.entry(ch).or_default() += 1; } }
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("counts = {:?}", counts.lock().unwrap());
}
```

Discussion
- Types are `Send` if they can be transferred across threads; `Sync` if `&T` can be shared across threads.
- Use `Arc` to share ownership; wrap interior in `Mutex` if mutability is needed. Keep critical sections short.

Lab
- Modify the fan‑out example to bound the worker count based on CPU cores (`num_cpus` crate) and add graceful shutdown logging when a worker finishes.

Reference hints
- Close channels by dropping all senders; receivers then exit their `for` loops.
- Use `eprintln!(...)` to log worker id start/stop.

Next
- Move to async with Tokio: a single runtime, many lightweight tasks with structured cancellation.

