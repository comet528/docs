# CI and Releases for Toolbox CLI and REST Service

This guide shows how to set up GitHub Actions CI, builds, releases, and container images for the two Rust projects in `rust/`.

What you get
- CI workflow (`.github/workflows/ci.yml`): fmt, clippy, test, and release builds for:
  - Toolbox workspace (`rust/toolbox`)
  - REST service (`rust/restsvc`)
- Release workflow (`.github/workflows/release.yml`): on `v*` tags:
  - Builds Linux x86_64 binaries (glibc and MUSL) for Toolbox and REST service
  - Packages tarballs + SHA256SUMS and attaches to a GitHub Release
  - Builds and pushes container images for REST service to GHCR as:
    - `ghcr.io/<owner>/restsvc:distroless-latest` and `:distroless-<tag>`
    - `ghcr.io/<owner>/restsvc:musl-latest` and `:musl-<tag>`

Prerequisites (one-time)
1) Enable Actions and write permissions for the workflow token
   - Repo → Settings → Actions → General → Workflow permissions → “Read and write permissions” → Save
2) Allow GHCR publishes using GITHUB_TOKEN
   - Repo → Settings → Packages → Ensure packages can be created in this org/user
   - No extra secrets required; the workflow logs in with `secrets.GITHUB_TOKEN`
3) (Optional) Protect main branches as you see fit

File layout
- CI: `.github/workflows/ci.yml`
- Release: `.github/workflows/release.yml`

CI behavior (push/PR)
- Runs rustfmt and clippy on both projects
- Runs tests for `rust/toolbox` and `rust/restsvc`
- Builds release binaries for both (sanity check)

Release behavior (tags `v*`)
- Builds Linux x86_64 binaries for:
  - Toolbox CLI: glibc and MUSL
  - REST service: glibc and MUSL
- Produces tarballs in `dist/` with checksums and attaches to a GitHub Release
- Builds and pushes REST service images to GHCR (distroless and musl/scratch)

How to cut a release
1) Update versions (if needed) in Cargo.toml files
2) Create a tag and push it
   - `git tag v0.1.0`
   - `git push origin v0.1.0`
3) Watch Actions → the “Release” workflow
4) Download artifacts from the Release page and/or pull images from GHCR:
   - `docker pull ghcr.io/<owner>/restsvc:distroless-v0.1.0`
   - `docker pull ghcr.io/<owner>/restsvc:musl-v0.1.0`

Customizing image names
- The workflow names the image `ghcr.io/<owner>/restsvc`. To change it, edit `.github/workflows/release.yml` and update `env: IMAGE_NAME` and any references.

Local validation tips
- Run CI steps locally:
  - Toolbox: `cd rust/toolbox && cargo fmt --all && cargo clippy --all-targets && cargo test --workspace`
  - REST: `cd rust/restsvc && cargo fmt --all && cargo clippy --all-targets && cargo test`
- Build MUSL locally (Linux):
  - `rustup target add x86_64-unknown-linux-musl`
  - `cargo build --release --target x86_64-unknown-linux-musl`
- Build images locally:
  - `cd rust/restsvc`
  - `docker build -f Dockerfile.distroless -t restsvc:distroless .`
  - `docker build -f Dockerfile.musl -t restsvc:musl .`

Notes
- The toolbox CLI isn’t containerized by default; it’s packaged as binaries. You can add a Dockerfile similarly if needed.
- You can extend the matrix to include macOS and Windows builds using additional jobs and `actions-rs/cargo` or `cross`.
- The integration test for the CLI (`rust/toolbox/toolbox-cli/tests/http_e2e.rs`) uses an in‑process Axum server and should pass in CI without external network access.

