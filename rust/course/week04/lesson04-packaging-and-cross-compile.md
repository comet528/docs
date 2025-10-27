# Lesson 04 — Packaging, Cross‑Compile, and Release Artifacts

Why this matters
- Shipping reliable binaries across platforms and in minimal containers is essential for ops‑friendly tools.

Concepts covered
- Release profiles, stripping, size.
- Static MUSL builds; cross‑compilation.
- Minimal container images (scratch, distroless) and non‑root execution.

Release build and strip
```bash
cargo build --release
strip target/release/toolbox || true
ls -lh target/release/toolbox
```

Optimize release profile (Cargo.toml)
```toml
[profile.release]
codegen-units = 1
lto = true
opt-level = "z"
strip = true
```

Static MUSL build (Linux target)
```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
file target/x86_64-unknown-linux-musl/release/toolbox
```

Containers
- Scratch (static): see `rust/course/templates/Dockerfile.musl-scratch`
- Distroless (glibc): see `rust/course/templates/Dockerfile.distroless`

Example (scratch)
```bash
docker build -f rust/course/templates/Dockerfile.musl-scratch -t toolbox:musl .
docker run --rm -e RUST_LOG=info toolbox:musl --help
```

Example (distroless)
```bash
docker build -f rust/course/templates/Dockerfile.distroless -t toolbox:distroless .
docker run --rm toolbox:distroless --help
```

Artifacts and checksums
```bash
mkdir -p dist
tar -C target/release -czf dist/toolbox-linux-amd64.tar.gz toolbox
sha256sum dist/* > dist/SHA256SUMS
```

Optional: `cross`
- Use `cross` for reproducible cross‑compiles via Docker.
- Install: `cargo install cross`
- Build: `cross build --release --target x86_64-unknown-linux-musl`

Lab
- Produce `toolbox` builds for your host and for Linux MUSL, package them with checksums, and run the scratch and distroless containers.

Next
- Week 5: REST with Axum, observability, graceful shutdown, and distroless containers.

