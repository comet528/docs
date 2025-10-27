# Lesson 04 — Containers and Release (Scratch/Distroless)

Why this matters
- Minimal images reduce attack surface and pull time. Non‑root execution is a must in many environments.

Concepts covered
- Static MUSL builds for scratch.
- Distroless glibc images.
- Non‑root users, ports via `PORT` env, health checks.

Prereqs
- Ensure your service binary name matches the Dockerfiles (default `restsvc`).

Scratch image (static MUSL)
- Use: `rust/course/templates/Dockerfile.musl-scratch`
```bash
rustup target add x86_64-unknown-linux-musl
docker build -f rust/course/templates/Dockerfile.musl-scratch -t restsvc:musl .
docker run --rm -p 8080:8080 -e PORT=8080 restsvc:musl
```

Distroless (glibc)
- Use: `rust/course/templates/Dockerfile.distroless`
```bash
docker build -f rust/course/templates/Dockerfile.distroless -t restsvc:distroless .
docker run --rm -p 8080:8080 -e PORT=8080 restsvc:distroless
```

Image validation
- Size target: scratch < 15MB, distroless < 40MB (depending on deps).
- Run as non‑root (see `USER` in Dockerfiles). Ensure no shell/extra files in the runtime image.

Release artifacts
```bash
cargo build --release --target x86_64-unknown-linux-musl
mkdir -p dist && cp target/x86_64-unknown-linux-musl/release/restsvc dist/
tar -C dist -czf dist/restsvc-linux-amd64.tar.gz restsvc
sha256sum dist/* > dist/SHA256SUMS
```

Optional: CI
- Create a CI workflow to build release binaries and push images on tag.
- Cache deps with `actions/cache` or `Swatinem/rust-cache`.

Lab
- Build both images and run `curl localhost:8080/healthz` to verify readiness.

Next
- You’ve completed the course. Consider extending the CLI and service with metrics, auth, and deployment manifests.

